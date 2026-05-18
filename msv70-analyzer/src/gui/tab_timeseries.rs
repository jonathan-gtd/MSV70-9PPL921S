use std::time::Instant;
use egui::{Color32, FontId, RichText, Ui};
use egui_plot::{HLine, Line, Plot, PlotBounds, PlotPoints, Points, VLine};
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

const CURSOR_COL: Color32 = Color32::from_rgb(0, 220, 220);

// ── Channel key ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChannelKey {
    Rpm, LoadPct, StftB1, StftB2, LmbB1, LmbB2, IgnDeg, CoolantC,
}

pub const ALL_CHANNELS: &[ChannelKey] = &[
    ChannelKey::Rpm,    ChannelKey::LoadPct,
    ChannelKey::StftB1, ChannelKey::StftB2,
    ChannelKey::LmbB1,  ChannelKey::LmbB2,
    ChannelKey::IgnDeg, ChannelKey::CoolantC,
];

impl ChannelKey {
    pub fn label(&self) -> &'static str {
        match self {
            ChannelKey::Rpm      => "RPM",
            ChannelKey::LoadPct  => "Load %",
            ChannelKey::StftB1   => "STFT B1",
            ChannelKey::StftB2   => "STFT B2",
            ChannelKey::LmbB1    => "λ Bank 1",
            ChannelKey::LmbB2    => "λ Bank 2",
            ChannelKey::IgnDeg   => "Ignition",
            ChannelKey::CoolantC => "Coolant",
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            ChannelKey::Rpm      => "rpm",
            ChannelKey::LoadPct  => "%",
            ChannelKey::StftB1 | ChannelKey::StftB2 => "%",
            ChannelKey::LmbB1 | ChannelKey::LmbB2   => "λ",
            ChannelKey::IgnDeg   => "°",
            ChannelKey::CoolantC => "°C",
        }
    }

    pub fn color(&self) -> Color32 {
        match self {
            ChannelKey::Rpm      => CHART[0],
            ChannelKey::LoadPct  => CHART[4],
            ChannelKey::StftB1   => CHART[1],
            ChannelKey::StftB2   => CHART[7],
            ChannelKey::LmbB1    => CHART[6],
            ChannelKey::LmbB2    => CHART[8],
            ChannelKey::IgnDeg   => CHART[9],
            ChannelKey::CoolantC => CHART[3],
        }
    }

    fn value(&self, row: &Row, cfg: &AppConfig) -> Option<f64> {
        match self {
            ChannelKey::Rpm      => Some(row.rpm),
            ChannelKey::LoadPct  => Some(row.load_pct),
            ChannelKey::StftB1   => if row.cl_b1 { Some(row.stft_b1) } else { None },
            ChannelKey::StftB2   => if row.cl_b2 { Some(row.stft_b2) } else { None },
            ChannelKey::LmbB1    => if (0.5..2.5).contains(&row.lmb_b1) { Some(row.lmb_b1) } else { None },
            ChannelKey::LmbB2    => if (0.5..2.5).contains(&row.lmb_b2) { Some(row.lmb_b2) } else { None },
            ChannelKey::IgnDeg   => if row.ign_deg.abs() > cfg.near_zero { Some(row.ign_deg) } else { None },
            ChannelKey::CoolantC => Some(row.coolant_c),
        }
    }
}

// ── Panel types ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YAxis { Left, Right }

#[derive(Clone, Debug)]
pub struct ChartChannel { pub key: ChannelKey, pub y_axis: YAxis }

#[derive(Clone, Debug)]
pub struct ChartPanel {
    pub id:       usize,
    pub channels: Vec<ChartChannel>,
    pub height:   f32,
}

pub fn default_panels() -> Vec<ChartPanel> {
    vec![
        ChartPanel { id: 0, height: 200.0, channels: vec![
            ChartChannel { key: ChannelKey::Rpm,     y_axis: YAxis::Left  },
            ChartChannel { key: ChannelKey::LoadPct,  y_axis: YAxis::Right },
        ]},
        ChartPanel { id: 1, height: 200.0, channels: vec![
            ChartChannel { key: ChannelKey::StftB1,  y_axis: YAxis::Left },
            ChartChannel { key: ChannelKey::StftB2,  y_axis: YAxis::Left },
        ]},
        ChartPanel { id: 2, height: 200.0, channels: vec![
            ChartChannel { key: ChannelKey::LmbB1,   y_axis: YAxis::Left  },
            ChartChannel { key: ChannelKey::LmbB2,   y_axis: YAxis::Right },
        ]},
    ]
}

// ── Main entry point ──────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    d: &SessionData,
    cfg: &AppConfig,
    panels:          &mut Vec<ChartPanel>,
    next_id:         &mut usize,
    cursor_s:        &mut Option<f64>,
    is_playing:      &mut bool,
    playback_speed:  &mut f64,
    _last_frame_time: &mut Option<Instant>,
) {
    let rows = &d.rows;
    if rows.is_empty() { return; }
    let t0    = rows[0].ts();
    let t_max = rows.last().unwrap().ts() - t0;
    let step  = (rows.len() / cfg.display_max_points).max(1);
    let ts: Vec<f64> = rows.iter().step_by(step).map(|r| r.ts() - t0).collect();

    // ── Timeline (bottom) ─────────────────────────────────────────────────────
    egui::TopBottomPanel::bottom("ts_timeline")
        .frame(egui::Frame::NONE.fill(BG2).inner_margin(egui::Margin::symmetric(8, 6)))
        .exact_height(72.0)
        .show_inside(ui, |ui| {
            show_timeline(ui, t_max, cursor_s, is_playing, playback_speed);
        });

    // ── Channel cards (top) ───────────────────────────────────────────────────
    egui::TopBottomPanel::top("ts_cards")
        .frame(egui::Frame::NONE.fill(BG2).inner_margin(egui::Margin::symmetric(6, 4)))
        .exact_height(82.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::horizontal().id_salt("ts_cards_scroll").show(ui, |ui| {
                ui.horizontal(|ui| {
                    show_channel_cards(ui, rows, t0, *cursor_s, panels, cfg);
                });
            });
        });

    // ── Chart area ────────────────────────────────────────────────────────────
    egui::CentralPanel::default().show_inside(ui, |ui| {
        // Palette + controls bar
        egui::TopBottomPanel::top("ts_palette")
            .frame(egui::Frame::NONE.fill(BG2).inner_margin(egui::Margin::symmetric(6, 4)))
            .exact_height(36.0)
            .show_inside(ui, |ui| {
                show_palette_bar(ui, panels, next_id);
            });

        // Chart panels (scrollable)
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().id_salt("ts_panels_scroll").show(ui, |ui| {
                let is_dragging = egui::DragAndDrop::payload::<ChannelKey>(ui.ctx()).is_some();
                let cs = *cursor_s;
                let mut to_delete: Option<usize> = None;
                let n = panels.len();

                for panel_idx in 0..n {
                    let panel_id = panels[panel_idx].id;

                    // Drop zone frame
                    let drop_frame_fill = if is_dragging { BG3 } else { BG };
                    let drop_frame = egui::Frame::NONE
                        .fill(drop_frame_fill)
                        .stroke(egui::Stroke::new(
                            if is_dragging { 1.5 } else { 0.0 },
                            ACCENT.gamma_multiply(0.4),
                        ))
                        .corner_radius(4.0)
                        .inner_margin(egui::Margin::symmetric(0, 2));

                    // Available rect center before dnd_drop_zone (for L/R detection)
                    let avail_center_x = ui.available_rect_before_wrap().center().x;

                    let (_, dropped) = ui.dnd_drop_zone::<ChannelKey, _>(drop_frame, |ui| {
                        // Panel header
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("Chart {}", panel_idx + 1))
                                .color(FG_DIM).size(11.0).strong());
                            ui.add_space(6.0);

                            // Channel chips
                            let n_ch = panels[panel_idx].channels.len();
                            let mut toggle_idx: Option<usize> = None;
                            let mut remove_idx: Option<usize> = None;
                            for ci in 0..n_ch {
                                let ch = &panels[panel_idx].channels[ci];
                                let col = ch.key.color();
                                egui::Frame::NONE
                                    .fill(BG3)
                                    .stroke(egui::Stroke::new(1.2, col))
                                    .corner_radius(3.0)
                                    .inner_margin(egui::Margin { left: 6, right: 4, top: 2, bottom: 2 })
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            let (r, _) = ui.allocate_exact_size(
                                                egui::vec2(7.0, 7.0), egui::Sense::hover(),
                                            );
                                            ui.painter().rect_filled(r, 1.0, col);
                                            ui.label(RichText::new(ch.key.label()).color(FG).size(11.0));
                                            // L/R toggle
                                            let axis_lbl = match ch.y_axis {
                                                YAxis::Left  => "L",
                                                YAxis::Right => "R",
                                            };
                                            if ui.add(egui::Button::new(
                                                RichText::new(axis_lbl).size(10.0).color(AMBER)
                                            ).small().frame(false)).clicked() {
                                                toggle_idx = Some(ci);
                                            }
                                            // Remove
                                            if ui.add(egui::Button::new(
                                                RichText::new("×").size(11.0).color(FG_DIM)
                                            ).small().frame(false)).clicked() {
                                                remove_idx = Some(ci);
                                            }
                                        });
                                    });
                                ui.add_space(3.0);
                            }

                            if let Some(ci) = toggle_idx {
                                let ch = &mut panels[panel_idx].channels[ci];
                                ch.y_axis = match ch.y_axis {
                                    YAxis::Left  => YAxis::Right,
                                    YAxis::Right => YAxis::Left,
                                };
                            }
                            if let Some(ci) = remove_idx {
                                panels[panel_idx].channels.remove(ci);
                            }

                            // Drop hint
                            if panels[panel_idx].channels.is_empty() {
                                ui.label(RichText::new("← glisser un canal ici").color(FG_DIM).size(10.0).italics());
                            }

                            // Delete panel
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add(egui::Button::new(
                                    RichText::new("✕").size(11.0).color(RUST)
                                ).small().frame(false)).on_hover_text("Supprimer ce graphique").clicked() {
                                    to_delete = Some(panel_idx);
                                }
                            });
                        });

                        // ── Plot ──────────────────────────────────────────────
                        let panel = &panels[panel_idx];
                        if !panel.channels.is_empty() {
                            render_chart(ui, panel, panel_id, rows, &ts, step, t0, cs, cfg);
                        } else {
                            let (rect, _) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), panels[panel_idx].height),
                                egui::Sense::hover(),
                            );
                            ui.painter().rect_filled(rect, 4.0, BG3);
                            ui.painter().text(
                                rect.center(), egui::Align2::CENTER_CENTER,
                                "Glisser un canal depuis la barre du haut",
                                FontId::proportional(12.0), FG_DIM,
                            );
                        }
                    });

                    // Handle drop
                    if let Some(key_arc) = dropped {
                        let key = (*key_arc).clone();
                        if !panels[panel_idx].channels.iter().any(|c| c.key == key) {
                            let drop_x = ui.ctx().input(|i| {
                                i.pointer.interact_pos().map(|p| p.x)
                            }).unwrap_or(avail_center_x);
                            let y_axis = if drop_x < avail_center_x { YAxis::Left } else { YAxis::Right };
                            panels[panel_idx].channels.push(ChartChannel { key, y_axis });
                        }
                    }

                    ui.add_space(4.0);
                }

                if let Some(idx) = to_delete {
                    panels.remove(idx);
                }
            });
        });
    });
}

// ── Palette bar ───────────────────────────────────────────────────────────────

fn show_palette_bar(ui: &mut Ui, panels: &mut Vec<ChartPanel>, next_id: &mut usize) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Canaux :").color(FG_DIM).size(11.0));
        ui.add_space(4.0);

        for key in ALL_CHANNELS {
            let col = key.color();
            let _resp = ui.dnd_drag_source(
                egui::Id::new(format!("palette_{}", key.label())),
                key.clone(),
                |ui| {
                    egui::Frame::NONE
                        .fill(BG3)
                        .stroke(egui::Stroke::new(1.2, col))
                        .corner_radius(4.0)
                        .inner_margin(egui::Margin { left: 7, right: 7, top: 3, bottom: 3 })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let (r, _) = ui.allocate_exact_size(
                                    egui::vec2(8.0, 8.0), egui::Sense::hover(),
                                );
                                ui.painter().circle_filled(r.center(), 4.0, col);
                                ui.label(RichText::new(key.label()).color(FG).size(11.5));
                            });
                        });
                },
            );
            ui.add_space(3.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(
                egui::Button::new(RichText::new("+ Graphique").color(OLIVE).size(12.0))
                    .fill(BG3)
            ).clicked() {
                panels.push(ChartPanel { id: *next_id, height: 200.0, channels: vec![] });
                *next_id += 1;
            }
            ui.add_space(8.0);
            ui.label(RichText::new("← glisser L | glisser R →").color(FG_DIM).size(9.5).italics());
        });
    });
}

// ── Chart rendering with dual Y axis ─────────────────────────────────────────

fn render_chart(
    ui: &mut Ui,
    panel: &ChartPanel,
    panel_id: usize,
    rows: &[Row],
    ts: &[f64],
    step: usize,
    t0: f64,
    cursor_s: Option<f64>,
    cfg: &AppConfig,
) {
    let l_channels: Vec<&ChartChannel> = panel.channels.iter().filter(|c| c.y_axis == YAxis::Left).collect();
    let r_channels: Vec<&ChartChannel> = panel.channels.iter().filter(|c| c.y_axis == YAxis::Right).collect();

    // Build series data
    let l_series: Vec<(Color32, &str, Vec<[f64; 2]>)> = l_channels.iter().map(|ch| {
        let pts = build_pts(ts, rows, step, &ch.key, cfg);
        (ch.key.color(), ch.key.label(), pts)
    }).collect();

    let r_series: Vec<(Color32, &str, Vec<[f64; 2]>)> = r_channels.iter().map(|ch| {
        let pts = build_pts(ts, rows, step, &ch.key, cfg);
        (ch.key.color(), ch.key.label(), pts)
    }).collect();

    // Compute ranges
    let l_range = value_range(l_series.iter().flat_map(|(_, _, pts)| pts.iter().map(|p| p[1])));
    let r_range = value_range(r_series.iter().flat_map(|(_, _, pts)| pts.iter().map(|p| p[1])));

    // For normalized rendering of right channels
    let (l_min, l_max) = l_range.unwrap_or((-1.0, 1.0));
    let (r_min, r_max) = r_range.unwrap_or((l_min, l_max));

    let has_right = !r_series.is_empty();
    let plot_right_margin = if has_right { 52.0 } else { 0.0 };

    let mut captured_bounds: Option<PlotBounds> = None;

    let plot_resp = Plot::new(format!("chart_{}", panel_id))
        .height(panel.height)
        .legend(egui_plot::Legend::default().position(egui_plot::Corner::LeftTop))
        .show(ui, |plot_ui| {
            // Left-axis channels (normal scale)
            for (col, name, pts) in &l_series {
                if !pts.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(pts.clone()))
                        .color(*col).width(1.4).name(*name));
                }
            }

            // Right-axis channels (normalized to L range)
            for (col, name, pts) in &r_series {
                if !pts.is_empty() {
                    let norm: Vec<[f64; 2]> = pts.iter()
                        .map(|p| [p[0], normalize(p[1], r_min, r_max, l_min, l_max)])
                        .collect();
                    plot_ui.line(Line::new(PlotPoints::new(norm))
                        .color(*col).width(1.4).style(egui_plot::LineStyle::Dashed { length: 8.0 })
                        .name(format!("{name} (R)")));
                }
            }

            // STFT-specific overlays
            for ch in &panel.channels {
                match &ch.key {
                    ChannelKey::StftB1 | ChannelKey::StftB2 => {
                        let dashed = egui_plot::LineStyle::Dashed { length: 10.0 };
                        let dotted = egui_plot::LineStyle::Dotted { spacing: 6.0 };
                        let target_axis = &ch.y_axis;
                        let hlines: &[(f64, Color32, egui_plot::LineStyle)] = &[
                            (0.0, FG_DIM, dashed),
                            ( cfg.stft_warn, ORANGE, dotted),
                            (-cfg.stft_warn, ORANGE, dotted),
                            ( cfg.stft_alarm, RED, dotted),
                            (-cfg.stft_alarm, RED, dotted),
                        ];
                        for &(v, col, style) in hlines {
                            let y = if *target_axis == YAxis::Right && r_range.is_some() {
                                normalize(v, r_min, r_max, l_min, l_max)
                            } else { v };
                            plot_ui.hline(HLine::new(y).color(col).style(style));
                        }
                        break;
                    }
                    ChannelKey::LmbB1 | ChannelKey::LmbB2 => {
                        let dashed = egui_plot::LineStyle::Dashed { length: 10.0 };
                        let dotted = egui_plot::LineStyle::Dotted { spacing: 6.0 };
                        let target_axis = &ch.y_axis;
                        let hlines: &[(f64, Color32, egui_plot::LineStyle)] = &[
                            (1.00, GREEN, dashed),
                            (cfg.lmb_lean_load, RED, dotted),
                            (cfg.lmb_rich, ORANGE, dotted),
                        ];
                        for &(v, col, style) in hlines {
                            let y = if *target_axis == YAxis::Right && r_range.is_some() {
                                normalize(v, r_min, r_max, l_min, l_max)
                            } else { v };
                            plot_ui.hline(HLine::new(y).color(col).style(style));
                        }
                        break;
                    }
                    _ => {}
                }
            }

            // Cursor line
            if let Some(t) = cursor_s {
                plot_ui.vline(VLine::new(t).color(CURSOR_COL).width(1.0));
            }

            // Event markers on STFT panels
            if panel.channels.iter().any(|c| matches!(c.key, ChannelKey::StftB1 | ChannelKey::StftB2)) {
                // placeholder — could add event markers here
            }

            captured_bounds = Some(plot_ui.plot_bounds());
        });

    // ── Draw right Y axis manually ─────────────────────────────────────────
    if has_right {
        if let Some(bounds) = captured_bounds {
            let rect = plot_resp.response.rect;
            let ax_l = 44.0_f32;
            let ax_b = 22.0_f32;
            let data_top    = rect.top()    + 6.0;
            let data_bottom = rect.bottom() - ax_b;
            let data_right  = rect.right()  - 6.0;

            let painter = ui.painter();
            painter.line_segment(
                [egui::pos2(data_right, data_top), egui::pos2(data_right, data_bottom)],
                egui::Stroke::new(1.0, FG_DIM),
            );

            let n_ticks = 5usize;
            let font = FontId::proportional(9.0);
            for i in 0..=n_ticks {
                let t = i as f32 / n_ticks as f32;
                let sy = data_top + t * (data_bottom - data_top);
                let v  = r_max - t as f64 * (r_max - r_min);

                painter.line_segment(
                    [egui::pos2(data_right, sy), egui::pos2(data_right + 4.0, sy)],
                    egui::Stroke::new(1.0, FG_DIM),
                );
                let label = if (r_max - r_min).abs() > 10.0 {
                    format!("{v:.0}")
                } else {
                    format!("{v:.2}")
                };
                painter.text(
                    egui::pos2(data_right + 5.0, sy),
                    egui::Align2::LEFT_CENTER,
                    label,
                    font.clone(),
                    FG_DIM,
                );
            }
            // Right axis label: show channel names
            let r_label: String = r_channels.iter()
                .map(|c| format!("{} ({})", c.key.label(), c.key.unit()))
                .collect::<Vec<_>>().join(", ");
            painter.text(
                egui::pos2(data_right + plot_right_margin * 0.5, data_top - 2.0),
                egui::Align2::CENTER_BOTTOM,
                r_label,
                FontId::proportional(8.5),
                FG_DIM,
            );
            let _ = (ax_l, bounds);
        }
    }
}

// ── Timeline ──────────────────────────────────────────────────────────────────

fn show_timeline(
    ui: &mut Ui,
    t_max: f64,
    cursor_s:       &mut Option<f64>,
    is_playing:     &mut bool,
    playback_speed: &mut f64,
) {
    let cur = cursor_s.get_or_insert(0.0);

    ui.horizontal(|ui| {
        ui.label(RichText::new(fmt_time(0.0)).color(FG_DIM).size(11.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(fmt_time(t_max)).color(FG_DIM).size(11.0));
        });
    });

    let old_sw = ui.spacing().slider_width;
    ui.spacing_mut().slider_width = (ui.available_width() - 12.0).max(100.0);
    let slider_r = ui.add(
        egui::Slider::new(cur, 0.0..=t_max)
            .show_value(false)
            .clamping(egui::SliderClamping::Always),
    );
    ui.spacing_mut().slider_width = old_sw;
    if slider_r.changed() { *is_playing = false; }

    let btn = egui::vec2(28.0, 28.0);
    ui.horizontal(|ui| {
        let (play_text, play_col) = if *is_playing { ("⏸", AMBER) } else { ("▶", LGREEN) };
        if ui.add(egui::Button::new(RichText::new(play_text).size(16.0).color(play_col)).min_size(btn)).clicked() {
            *is_playing = !*is_playing;
        }
        if ui.add(egui::Button::new(RichText::new("⏹").size(16.0).color(RUST)).min_size(btn)).clicked() {
            *is_playing = false;
            *cur = 0.0;
        }
        ui.separator();
        ui.label(RichText::new("Speed").color(FG_DIM).size(12.0));
        egui::ComboBox::from_id_salt("ts_speed")
            .selected_text(format!("{}×", playback_speed))
            .width(62.0)
            .show_ui(ui, |ui| {
                for &s in &[0.25_f64, 0.5, 1.0, 2.0, 4.0, 8.0] {
                    ui.selectable_value(playback_speed, s, format!("{}×", s));
                }
            });
        ui.separator();
        ui.label(RichText::new(format!("⏱ {}", fmt_time(*cur))).color(CURSOR_COL).size(13.0).strong());
    });
}

// ── Channel cards ─────────────────────────────────────────────────────────────

fn show_channel_cards(
    ui: &mut Ui,
    rows: &[Row],
    t0: f64,
    cursor_s: Option<f64>,
    panels: &[ChartPanel],
    cfg: &AppConfig,
) {
    // Collect which channels are currently in any panel
    let active_keys: Vec<&ChannelKey> = panels.iter()
        .flat_map(|p| p.channels.iter().map(|c| &c.key))
        .collect();

    let cur_i = cursor_s.map(|cs| {
        let target = cs + t0;
        rows.partition_point(|r| r.ts() < target).min(rows.len().saturating_sub(1))
    });

    for key in ALL_CHANNELS {
        if !active_keys.contains(&key) { continue; }
        let (mn, mx) = ch_minmax(rows, key, cfg);
        let cur = cur_i.and_then(|i| key.value(&rows[i], cfg));
        channel_card(ui, key, mn, mx, cur);
    }
}

fn channel_card(ui: &mut Ui, key: &ChannelKey, mn: f64, mx: f64, cur: Option<f64>) {
    egui::Frame::NONE
        .fill(BG3)
        .stroke(egui::Stroke::new(1.5, key.color()))
        .corner_radius(4.0)
        .inner_margin(egui::Margin { left: 8, right: 8, top: 4, bottom: 4 })
        .show(ui, |ui| {
            ui.set_width(115.0);
            ui.label(RichText::new(key.label()).color(key.color()).size(10.0).strong());
            let val_str = match cur {
                Some(v) if !v.is_nan() => format!("{v:.1} {}", key.unit()),
                _ => "—".into(),
            };
            ui.label(RichText::new(val_str).color(FG).size(14.0).strong());
            ui.label(RichText::new(format!("{mn:.1} — {mx:.1}")).color(FG_DIM).size(9.0));
        });
    ui.add_space(4.0);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_pts(ts: &[f64], rows: &[Row], step: usize, key: &ChannelKey, cfg: &AppConfig) -> Vec<[f64; 2]> {
    ts.iter()
        .zip(rows.iter().step_by(step))
        .filter_map(|(&t, row)| key.value(row, cfg).map(|y| [t, y]))
        .collect()
}

fn value_range(iter: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let (mn, mx) = iter.filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), v| (mn.min(v), mx.max(v)));
    if mn.is_finite() && mx.is_finite() && mx > mn { Some((mn, mx)) } else { None }
}

fn normalize(v: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    if (from_max - from_min).abs() < 1e-10 { return (to_min + to_max) / 2.0; }
    let t = (v - from_min) / (from_max - from_min);
    to_min + t * (to_max - to_min)
}

fn ch_minmax(rows: &[Row], key: &ChannelKey, cfg: &AppConfig) -> (f64, f64) {
    rows.iter()
        .filter_map(|r| key.value(r, cfg))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), v| (mn.min(v), mx.max(v)))
}

fn fmt_time(s: f64) -> String {
    let m   = (s as u64) / 60;
    let sec = s % 60.0;
    format!("{m}:{sec:05.2}")
}
