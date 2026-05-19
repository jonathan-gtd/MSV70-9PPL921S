use std::time::Instant;
use egui::{Color32, FontId, RichText, Ui, vec2};
use egui_plot::{HLine, Line, Plot, PlotBounds, PlotPoints, Polygon, VLine};
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

const CURSOR_COL: Color32 = Color32::from_rgb(0, 220, 220);

// ── Channel key ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChannelKey {
    // Raw channels
    Rpm, LoadPct, StftB1, StftB2, LmbB1, LmbB2, IgnDeg, CoolantC,
    // Computed channels
    GearRatio, StftAvg, InjDuty, PedalPct,
}

pub const ALL_CHANNELS: &[ChannelKey] = &[
    ChannelKey::Rpm,      ChannelKey::LoadPct,
    ChannelKey::StftB1,   ChannelKey::StftB2,
    ChannelKey::LmbB1,    ChannelKey::LmbB2,
    ChannelKey::IgnDeg,   ChannelKey::CoolantC,
    ChannelKey::GearRatio, ChannelKey::StftAvg,
    ChannelKey::InjDuty,  ChannelKey::PedalPct,
];

impl ChannelKey {
    pub fn label(&self) -> &'static str {
        match self {
            ChannelKey::Rpm       => "RPM",
            ChannelKey::LoadPct   => "Load %",
            ChannelKey::StftB1    => "STFT B1",
            ChannelKey::StftB2    => "STFT B2",
            ChannelKey::LmbB1     => "λ Bank 1",
            ChannelKey::LmbB2     => "λ Bank 2",
            ChannelKey::IgnDeg    => "Ignition",
            ChannelKey::CoolantC  => "Coolant",
            ChannelKey::GearRatio => "Rapport BV",
            ChannelKey::StftAvg   => "STFT Avg",
            ChannelKey::InjDuty   => "Inj Duty",
            ChannelKey::PedalPct  => "Pédale",
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            ChannelKey::Rpm       => "rpm",
            ChannelKey::LoadPct   => "%",
            ChannelKey::StftB1 | ChannelKey::StftB2 | ChannelKey::StftAvg => "%",
            ChannelKey::LmbB1 | ChannelKey::LmbB2   => "λ",
            ChannelKey::IgnDeg    => "°",
            ChannelKey::CoolantC  => "°C",
            ChannelKey::GearRatio => "rpm/v",
            ChannelKey::InjDuty   => "%",
            ChannelKey::PedalPct  => "%",
        }
    }

    pub fn color(&self) -> Color32 {
        match self {
            ChannelKey::Rpm       => CHART[0],
            ChannelKey::LoadPct   => CHART[4],
            ChannelKey::StftB1    => CHART[1],
            ChannelKey::StftB2    => CHART[7],
            ChannelKey::LmbB1     => CHART[6],
            ChannelKey::LmbB2     => CHART[8],
            ChannelKey::IgnDeg    => CHART[9],
            ChannelKey::CoolantC  => CHART[3],
            ChannelKey::GearRatio => CHART[2],
            ChannelKey::StftAvg   => CHART[5],
            ChannelKey::InjDuty   => SAGE,
            ChannelKey::PedalPct  => CORN,
        }
    }

    pub fn is_computed(&self) -> bool {
        matches!(self, ChannelKey::GearRatio | ChannelKey::StftAvg | ChannelKey::InjDuty | ChannelKey::PedalPct)
    }

    pub fn value(&self, row: &Row, cfg: &AppConfig) -> Option<f64> {
        match self {
            ChannelKey::Rpm      => Some(row.rpm),
            ChannelKey::LoadPct  => Some(row.load_pct),
            ChannelKey::StftB1   => if row.cl_b1 { Some(row.stft_b1) } else { None },
            ChannelKey::StftB2   => if row.cl_b2 { Some(row.stft_b2) } else { None },
            ChannelKey::LmbB1    => if (0.5..2.5).contains(&row.lmb_b1) { Some(row.lmb_b1) } else { None },
            ChannelKey::LmbB2    => if (0.5..2.5).contains(&row.lmb_b2) { Some(row.lmb_b2) } else { None },
            ChannelKey::IgnDeg   => if row.ign_deg.abs() > cfg.near_zero { Some(row.ign_deg) } else { None },
            ChannelKey::CoolantC => Some(row.coolant_c),
            // ── Computed ─────────────────────────────────────────────────────
            ChannelKey::GearRatio => {
                if row.speed_kmh > 3.0 && row.rpm > 500.0 {
                    Some(row.rpm / row.speed_kmh)
                } else { None }
            }
            ChannelKey::StftAvg => {
                match (row.cl_b1, row.cl_b2) {
                    (true, true)  => Some((row.stft_b1 + row.stft_b2) / 2.0),
                    (true, false) => Some(row.stft_b1),
                    (false, true) => Some(row.stft_b2),
                    (false, false) => None,
                }
            }
            ChannelKey::InjDuty  => row.inj_duty(),
            ChannelKey::PedalPct => Some(row.pedal_pct),
        }
    }
}

// ── Panel types ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YAxis { Left, Right }

#[derive(Clone, Debug)]
pub struct ChartChannel {
    pub key:         ChannelKey,
    pub y_axis:      YAxis,
    pub filter_min:  Option<f64>,
    pub filter_max:  Option<f64>,
    pub show_filter: bool,
}

impl ChartChannel {
    pub fn new(key: ChannelKey, y_axis: YAxis) -> Self {
        Self { key, y_axis, filter_min: None, filter_max: None, show_filter: false }
    }
}

#[derive(Clone, Debug)]
pub struct ChartPanel {
    pub id:       usize,
    pub channels: Vec<ChartChannel>,
    pub height:   f32,
    pub y2_zoom:  f64,  // independent Y2 amplitude zoom; 1.0 = full range, <1 = zoomed in
}

pub fn default_panels() -> Vec<ChartPanel> {
    vec![
        ChartPanel { id: 0, height: 200.0, y2_zoom: 1.0, channels: vec![] },
    ]
}

// ── Main entry point ──────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    d: &SessionData,
    cfg: &AppConfig,
    panels:           &mut Vec<ChartPanel>,
    next_id:          &mut usize,
    cursor_s:         &mut Option<f64>,
    is_playing:       &mut bool,
    playback_speed:   &mut f64,
    _last_frame_time: &mut Option<Instant>,
    show_phases:      &mut bool,
) {
    let rows = &d.rows;
    if rows.is_empty() { return; }
    let t0    = rows[0].ts();
    let t_max = rows.last().unwrap().ts() - t0;
    let step  = (rows.len() / cfg.display_max_points).max(1);
    let ts: Vec<f64> = rows.iter().step_by(step).map(|r| r.ts() - t0).collect();

    // Precompute phase spans (used by all charts)
    let spans  = phase_spans(rows, t0);
    let t_max2 = t_max; // shadowed for borrow capture below

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
        .exact_height(94.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::horizontal().id_salt("ts_cards_scroll").show(ui, |ui| {
                ui.horizontal(|ui| {
                    show_channel_cards(ui, rows, t0, *cursor_s, panels, cfg);
                });
            });
        });

    // ── Chart area ────────────────────────────────────────────────────────────
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::TopBottomPanel::top("ts_palette")
            .frame(egui::Frame::NONE.fill(BG2).inner_margin(egui::Margin::symmetric(6, 4)))
            .exact_height(36.0)
            .show_inside(ui, |ui| {
                show_palette_bar(ui, panels, next_id, show_phases);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let is_dragging = egui::DragAndDrop::payload::<ChannelKey>(ui.ctx()).is_some();
            let cs = *cursor_s;
            let mut to_delete: Option<usize> = None;
            let n = panels.len();

            let avail_h = ui.available_height();
            let panel_h = if n > 0 { (avail_h / n as f32).max(80.0) } else { avail_h };

            for panel_idx in 0..n {
                let panel_id = panels[panel_idx].id;
                let filter_count = panels[panel_idx].channels.iter()
                    .filter(|ch| ch.show_filter).count();
                let chart_h = (panel_h - 40.0 - 26.0 * filter_count as f32).max(60.0);

                let drop_frame = egui::Frame::NONE
                    .fill(if is_dragging { BG3 } else { BG })
                    .stroke(egui::Stroke::new(if is_dragging { 1.5 } else { 0.0 }, ACCENT.gamma_multiply(0.3)))
                    .corner_radius(4.0)
                    .inner_margin(egui::Margin::symmetric(0, 2));

                let avail_center_x = ui.available_rect_before_wrap().center().x;

                let (dnd_resp, dropped) = ui.dnd_drop_zone::<ChannelKey, _>(drop_frame, |ui| {
                    // ── Panel header ─────────────────────────────────────────
                    let n_ch = panels[panel_idx].channels.len();
                    let mut toggle_idx:        Option<usize> = None;
                    let mut remove_idx:        Option<usize> = None;
                    let mut toggle_filter_idx: Option<usize> = None;

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Chart {}", panel_idx + 1))
                            .color(FG_DIM).size(11.0).strong());
                        ui.add_space(6.0);

                        for ci in 0..n_ch {
                            let (col, label, axis_lbl, has_filter, is_computed) = {
                                let ch = &panels[panel_idx].channels[ci];
                                (
                                    ch.key.color(),
                                    ch.key.label(),
                                    match ch.y_axis { YAxis::Left => "L", YAxis::Right => "R" },
                                    ch.filter_min.is_some() || ch.filter_max.is_some(),
                                    ch.key.is_computed(),
                                )
                            };

                            egui::Frame::NONE
                                .fill(BG3)
                                .stroke(egui::Stroke::new(1.2, col))
                                .corner_radius(3.0)
                                .inner_margin(egui::Margin { left: 6, right: 4, top: 2, bottom: 2 })
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        let (r, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                                        if is_computed {
                                            ui.painter().circle_stroke(r.center(), 3.5, egui::Stroke::new(1.2, col));
                                        } else {
                                            ui.painter().rect_filled(r, 1.0, col);
                                        }
                                        ui.label(RichText::new(label).color(FG).size(11.0));
                                        if ui.add(egui::Button::new(
                                            RichText::new(axis_lbl).size(10.0).color(AMBER)
                                        ).small().frame(false)).clicked() {
                                            toggle_idx = Some(ci);
                                        }
                                        let filter_col = if has_filter { AMBER } else { FG_DIM };
                                        if ui.add(egui::Button::new(
                                            RichText::new("f").size(10.0).color(filter_col)
                                        ).small().frame(false)).on_hover_text("Filtre valeurs").clicked() {
                                            toggle_filter_idx = Some(ci);
                                        }
                                        if ui.add(egui::Button::new(
                                            RichText::new("×").size(11.0).color(FG_DIM)
                                        ).small().frame(false)).clicked() {
                                            remove_idx = Some(ci);
                                        }
                                    });
                                });
                            ui.add_space(3.0);
                        }

                        if panels[panel_idx].channels.is_empty() {
                            ui.label(RichText::new("← glisser un canal ici").color(FG_DIM).size(10.0).italics());
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(egui::Button::new(
                                RichText::new("✕").size(11.0).color(RUST)
                            ).small().frame(false)).on_hover_text("Supprimer").clicked() {
                                to_delete = Some(panel_idx);
                            }
                            ui.add_space(4.0);
                            let y2z = panels[panel_idx].y2_zoom;
                            if (y2z - 1.0).abs() > 0.05 {
                                if ui.add(
                                    egui::Button::new(RichText::new(format!("Y2 ×{:.1}", 1.0 / y2z))
                                        .size(9.5).color(AMBER)).small().frame(false)
                                ).on_hover_text("Clic = reset Y2").clicked() {
                                    panels[panel_idx].y2_zoom = 1.0;
                                }
                            }
                            ui.label(RichText::new("Z").size(10.0).color(FG_DIM))
                                .on_hover_text("Glisser sur zone axe X (bas) = zoom horizontal · zone Y1 (gauche) = zoom vertical · zone Y2 (droite) = zoom axe droit · double-clic = reset");
                        });
                    });

                    // Apply deferred mutations
                    if let Some(ci) = toggle_idx {
                        let ch = &mut panels[panel_idx].channels[ci];
                        ch.y_axis = match ch.y_axis { YAxis::Left => YAxis::Right, YAxis::Right => YAxis::Left };
                    }
                    if let Some(ci) = remove_idx {
                        panels[panel_idx].channels.remove(ci);
                    }
                    if let Some(ci) = toggle_filter_idx {
                        let v = &mut panels[panel_idx].channels[ci].show_filter;
                        *v = !*v;
                    }

                    // ── Filter controls ──────────────────────────────────────
                    let filter_cis: Vec<usize> = (0..panels[panel_idx].channels.len())
                        .filter(|&ci| panels[panel_idx].channels[ci].show_filter)
                        .collect();

                    for &ci in &filter_cis {
                        let (col, label) = {
                            let ch = &panels[panel_idx].channels[ci];
                            (ch.key.color(), ch.key.label())
                        };
                        let mut has_min = panels[panel_idx].channels[ci].filter_min.is_some();
                        let mut min_val = panels[panel_idx].channels[ci].filter_min.unwrap_or(0.0);
                        let mut has_max = panels[panel_idx].channels[ci].filter_max.is_some();
                        let mut max_val = panels[panel_idx].channels[ci].filter_max.unwrap_or(100.0);

                        ui.horizontal(|ui| {
                            ui.add_space(6.0);
                            let (r, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                            ui.painter().circle_filled(r.center(), 3.5, col);
                            ui.label(RichText::new(label).color(col).size(10.0));
                            ui.add_space(8.0);
                            ui.checkbox(&mut has_min, "");
                            ui.label(RichText::new("Min").color(FG_DIM).size(10.0));
                            if has_min { ui.add(egui::DragValue::new(&mut min_val).speed(0.1)); }
                            ui.add_space(6.0);
                            ui.checkbox(&mut has_max, "");
                            ui.label(RichText::new("Max").color(FG_DIM).size(10.0));
                            if has_max { ui.add(egui::DragValue::new(&mut max_val).speed(0.1)); }
                            ui.add_space(8.0);
                            if ui.small_button("✕").clicked() { has_min = false; has_max = false; }
                        });

                        panels[panel_idx].channels[ci].filter_min = if has_min { Some(min_val) } else { None };
                        panels[panel_idx].channels[ci].filter_max = if has_max { Some(max_val) } else { None };
                    }

                    // ── Plot ─────────────────────────────────────────────────
                    if !panels[panel_idx].channels.is_empty() {
                        let mut y2_zoom = panels[panel_idx].y2_zoom;
                        let hover_t = {
                            let panel = &panels[panel_idx];
                            render_chart(ui, panel, panel_id, rows, &ts, step, t0, cs, cfg, chart_h, &spans, &mut y2_zoom, *show_phases)
                        };
                        panels[panel_idx].y2_zoom = y2_zoom;
                        if !*is_playing {
                            if let Some(ht) = hover_t {
                                *cursor_s = Some(ht.clamp(0.0, t_max2));
                            }
                        }
                    } else {
                        let (rect, _) = ui.allocate_exact_size(
                            vec2(ui.available_width(), chart_h), egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(rect, 4.0, BG3);
                        ui.painter().text(
                            rect.center(), egui::Align2::CENTER_CENTER,
                            "Glisser un canal depuis la barre du haut",
                            FontId::proportional(12.0), FG_DIM,
                        );
                    }
                });

                // ── L/R drop overlay ─────────────────────────────────────────
                if is_dragging {
                    let zone_rect = dnd_resp.response.rect;
                    let ptr_in_zone = ui.ctx().input(|i| {
                        i.pointer.hover_pos().map(|p| zone_rect.contains(p)).unwrap_or(false)
                    });
                    if ptr_in_zone {
                        let px = ui.ctx().input(|i| {
                            i.pointer.hover_pos().map(|p| p.x).unwrap_or(avail_center_x)
                        });
                        let is_left = px < avail_center_x;
                        let half = if is_left {
                            egui::Rect::from_min_max(zone_rect.min,
                                egui::pos2(avail_center_x, zone_rect.max.y))
                        } else {
                            egui::Rect::from_min_max(
                                egui::pos2(avail_center_x, zone_rect.min.y), zone_rect.max)
                        };
                        ui.painter().rect_filled(half, 4.0, LGREEN.gamma_multiply(0.18));
                        ui.painter().rect_stroke(half, 4.0,
                            egui::Stroke::new(1.5, LGREEN.gamma_multiply(0.7)),
                            egui::StrokeKind::Middle);
                        ui.painter().text(
                            half.center(), egui::Align2::CENTER_CENTER,
                            if is_left { "← Axe Gauche" } else { "Axe Droit →" },
                            FontId::proportional(12.0), LGREEN,
                        );
                    }
                }

                // Handle drop
                if let Some(key_arc) = dropped {
                    let key = (*key_arc).clone();
                    if !panels[panel_idx].channels.iter().any(|c| c.key == key) {
                        let drop_x = ui.ctx().input(|i| {
                            i.pointer.interact_pos().map(|p| p.x)
                        }).unwrap_or(avail_center_x);
                        let y_axis = if drop_x < avail_center_x { YAxis::Left } else { YAxis::Right };
                        panels[panel_idx].channels.push(ChartChannel::new(key, y_axis));
                    }
                }
            }

            if let Some(idx) = to_delete {
                panels.remove(idx);
            }
        });
    });
}

// ── Palette bar ───────────────────────────────────────────────────────────────

fn show_palette_bar(ui: &mut Ui, panels: &mut Vec<ChartPanel>, next_id: &mut usize, show_phases: &mut bool) {
    ui.horizontal(|ui| {
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
                        .inner_margin(egui::Margin { left: 6, right: 6, top: 2, bottom: 2 })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let (r, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                                if key.is_computed() {
                                    ui.painter().circle_stroke(r.center(), 3.5, egui::Stroke::new(1.2, col));
                                } else {
                                    ui.painter().circle_filled(r.center(), 3.5, col);
                                }
                                ui.label(RichText::new(key.label()).color(FG).size(11.0));
                            });
                        });
                },
            );
            ui.add_space(2.0);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(
                egui::Button::new(RichText::new("+ Chart").color(OLIVE).size(12.0)).fill(BG3)
            ).clicked() {
                panels.push(ChartPanel { id: *next_id, height: 200.0, y2_zoom: 1.0, channels: vec![] });
                *next_id += 1;
            }
            ui.add_space(4.0);
            ui.label(RichText::new("← L | R →").color(FG_DIM).size(9.5).italics());
            ui.add_space(4.0);
            // Phase toggle button + legend
            let phase_col = if *show_phases { ACCENT } else { FG_DIM };
            if ui.add(
                egui::Button::new(RichText::new("Phases").size(9.5).color(phase_col)).fill(BG3).small()
            ).on_hover_text(if *show_phases { "Cacher phases" } else { "Afficher phases" }).clicked() {
                *show_phases = !*show_phases;
            }
            if *show_phases {
                ui.add_space(2.0);
                for phase in Phase::all() {
                    if matches!(phase, Phase::EngineOff | Phase::Cranking) { continue; }
                    let col = phase_color(phase);
                    let resp = ui.add(egui::Label::new(
                        RichText::new(format!(" {} ", phase.as_str())).color(col).size(9.0)
                    ).sense(egui::Sense::hover()));
                    let (r, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                    ui.painter().circle_filled(r.center(), 3.5, col);
                    drop(resp);
                }
            }
        });
    });
}

// ── Chart rendering ───────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
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
    chart_h: f32,
    phase_spans: &[(f64, f64, Phase)],
    y2_zoom: &mut f64,
    show_phases: bool,
) -> Option<f64> {
    let l_channels: Vec<&ChartChannel> = panel.channels.iter().filter(|c| c.y_axis == YAxis::Left).collect();
    let r_channels: Vec<&ChartChannel> = panel.channels.iter().filter(|c| c.y_axis == YAxis::Right).collect();

    let l_series: Vec<(Color32, &str, Vec<[f64; 2]>)> = l_channels.iter().map(|ch| {
        (ch.key.color(), ch.key.label(), build_pts(ts, rows, step, &ch.key, cfg, ch.filter_min, ch.filter_max))
    }).collect();

    let r_series: Vec<(Color32, &str, Vec<[f64; 2]>)> = r_channels.iter().map(|ch| {
        (ch.key.color(), ch.key.label(), build_pts(ts, rows, step, &ch.key, cfg, ch.filter_min, ch.filter_max))
    }).collect();

    let l_range = value_range(l_series.iter().flat_map(|(_, _, pts)| pts.iter().map(|p| p[1])));
    let r_range = value_range(r_series.iter().flat_map(|(_, _, pts)| pts.iter().map(|p| p[1])));
    let (l_min, l_max) = l_range.unwrap_or((-1.0, 1.0));
    let (r_min, r_max) = r_range.unwrap_or((l_min, l_max));
    let has_right = !r_series.is_empty();

    // ── Axis-drag zones (derived from last frame's plot rect) ─────────────────
    let rect_id   = egui::Id::new(format!("chart_rect_{}", panel_id));
    let bounds_id = egui::Id::new(format!("chart_bnds_{}", panel_id));
    let last_rect: Option<egui::Rect> = ui.ctx().data(|d| d.get_temp(rect_id));
    let prev_bounds: Option<[f64; 4]>  = ui.ctx().data(|d| d.get_temp(bounds_id));

    let x_zone  = last_rect.map(|r| egui::Rect::from_min_max(
        egui::pos2(r.left() + 50.0, r.bottom() - 24.0),
        egui::pos2(r.right(), r.bottom()),
    ));
    let y1_zone = last_rect.map(|r| egui::Rect::from_min_max(
        egui::pos2(r.left(), r.top()),
        egui::pos2(r.left() + 48.0, r.bottom() - 24.0),
    ));
    let y2_zone: Option<egui::Rect> = if has_right {
        last_rect.map(|r| egui::Rect::from_min_max(
            egui::pos2(r.right() - 60.0, r.top()),
            egui::pos2(r.right(), r.bottom() - 24.0),
        ))
    } else { None };

    let (ptr_pos, ptr_down, press_origin, drag_delta) = ui.ctx().input(|i| (
        i.pointer.hover_pos(),
        i.pointer.primary_down(),
        i.pointer.press_origin(),
        i.pointer.delta(),
    ));

    let in_zone = |z: Option<egui::Rect>, p: egui::Pos2| z.map(|r| r.contains(p)).unwrap_or(false);
    let drag_on_x  = ptr_down && press_origin.map(|p| in_zone(x_zone,  p)).unwrap_or(false);
    let drag_on_y1 = ptr_down && press_origin.map(|p| in_zone(y1_zone, p)).unwrap_or(false);
    let drag_on_y2 = ptr_down && press_origin.map(|p| in_zone(y2_zone, p)).unwrap_or(false);
    let axis_drag  = drag_on_x || drag_on_y1 || drag_on_y2;

    // Cursor feedback when hovering axis zones
    if let Some(pos) = ptr_pos {
        if in_zone(x_zone, pos)        { ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal); }
        else if in_zone(y1_zone, pos)  { ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical); }
        else if in_zone(y2_zone, pos)  { ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical); }
    }

    // Y2 drag → adjust y2_zoom
    if drag_on_y2 && drag_delta.y.abs() > 0.1 {
        let factor = (-drag_delta.y as f64 * 0.009).exp();
        *y2_zoom = (*y2_zoom * factor).clamp(0.05, 20.0);
    }

    // Compute PlotBounds override for X / Y1 drag
    let bounds_override: Option<PlotBounds> = if (drag_on_x || drag_on_y1) && drag_delta.length() > 0.2 {
        prev_bounds.map(|[xmin, ymin, xmax, ymax]| {
            let (mut nxmin, mut nxmax, mut nymin, mut nymax) = (xmin, xmax, ymin, ymax);
            if drag_on_x {
                let xctr  = (xmin + xmax) / 2.0;
                let xhalf = (xmax - xmin) / 2.0;
                // drag right = zoom out, drag left = zoom in
                let f = (drag_delta.x as f64 * 0.009).exp();
                let h = (xhalf * f).max(0.01);
                nxmin = xctr - h; nxmax = xctr + h;
            }
            if drag_on_y1 {
                let yctr  = (ymin + ymax) / 2.0;
                let yhalf = (ymax - ymin) / 2.0;
                // drag up = zoom in, drag down = zoom out
                let f = (-drag_delta.y as f64 * 0.009).exp();
                let h = (yhalf * f).max(0.0001);
                nymin = yctr - h; nymax = yctr + h;
            }
            PlotBounds::from_min_max([nxmin, nymin], [nxmax, nymax])
        })
    } else { None };

    // Adjusted right-axis range with Y2 zoom
    let y2_ctr  = (r_min + r_max) / 2.0;
    let y2_half = (r_max - r_min) / 2.0 * *y2_zoom;
    let (adj_r_min, adj_r_max) = (y2_ctr - y2_half, y2_ctr + y2_half);

    // Use large Y bounds for phase polygons (plot clips them)
    let y_lo = (l_min - (l_max - l_min).abs()) * 5.0 - 1e3;
    let y_hi = (l_max + (l_max - l_min).abs()) * 5.0 + 1e3;

    let mut captured_bounds: Option<PlotBounds> = None;

    let mut plot = Plot::new(format!("chart_{}", panel_id))
        .height(chart_h)
        .allow_zoom(egui::Vec2b::TRUE)
        .allow_drag(!axis_drag)
        .legend(egui_plot::Legend::default().position(egui_plot::Corner::LeftTop));
    if bounds_override.is_some() {
        plot = plot.auto_bounds(egui::Vec2b::FALSE);
    }
    let plot_resp = plot.show(ui, |plot_ui| {
        if let Some(b) = bounds_override { plot_ui.set_plot_bounds(b); }
            // ── Phase overlays ────────────────────────────────────────────────
            if show_phases {
                for (t_start, t_end, phase) in phase_spans {
                    if matches!(phase, Phase::EngineOff | Phase::Cranking) { continue; }
                    let col = phase_color(phase);
                    let fill = Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), 22);
                    let pts = PlotPoints::new(vec![
                        [*t_start, y_lo], [*t_end, y_lo], [*t_end, y_hi], [*t_start, y_hi],
                    ]);
                    plot_ui.polygon(Polygon::new(pts)
                        .fill_color(fill)
                        .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT)));
                }
            }

            // ── Left-axis channels ────────────────────────────────────────────
            for (col, name, pts) in &l_series {
                if !pts.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(pts.clone()))
                        .color(*col).width(1.4).name(*name));
                }
            }

            // ── Right-axis channels (normalized to L range, Y2-zoom applied) ──
            for (col, name, pts) in &r_series {
                if !pts.is_empty() {
                    let norm: Vec<[f64; 2]> = pts.iter()
                        .map(|p| [p[0], normalize(p[1], adj_r_min, adj_r_max, l_min, l_max)])
                        .collect();
                    plot_ui.line(Line::new(PlotPoints::new(norm))
                        .color(*col).width(1.4)
                        .style(egui_plot::LineStyle::Dashed { length: 8.0 })
                        .name(format!("{name} (R)")));
                }
            }

            // ── Semantic overlays ─────────────────────────────────────────────
            for ch in &panel.channels {
                match &ch.key {
                    ChannelKey::StftB1 | ChannelKey::StftB2 | ChannelKey::StftAvg => {
                        let dashed = egui_plot::LineStyle::Dashed { length: 10.0 };
                        let dotted = egui_plot::LineStyle::Dotted { spacing: 6.0 };
                        let ax = &ch.y_axis;
                        for &(v, col, style) in &[
                            (0.0,             FG_DIM, dashed),
                            ( cfg.stft_warn,  ORANGE, dotted),
                            (-cfg.stft_warn,  ORANGE, dotted),
                            ( cfg.stft_alarm, RED,    dotted),
                            (-cfg.stft_alarm, RED,    dotted),
                        ] {
                            let y = if *ax == YAxis::Right && r_range.is_some() {
                                normalize(v, adj_r_min, adj_r_max, l_min, l_max)
                            } else { v };
                            plot_ui.hline(HLine::new(y).color(col).style(style));
                        }
                        break;
                    }
                    ChannelKey::LmbB1 | ChannelKey::LmbB2 => {
                        let dashed = egui_plot::LineStyle::Dashed { length: 10.0 };
                        let dotted = egui_plot::LineStyle::Dotted { spacing: 6.0 };
                        let ax = &ch.y_axis;
                        for &(v, col, style) in &[
                            (1.00,              GREEN,  dashed),
                            (cfg.lmb_lean_load, RED,    dotted),
                            (cfg.lmb_rich,      ORANGE, dotted),
                        ] {
                            let y = if *ax == YAxis::Right && r_range.is_some() {
                                normalize(v, adj_r_min, adj_r_max, l_min, l_max)
                            } else { v };
                            plot_ui.hline(HLine::new(y).color(col).style(style));
                        }
                        break;
                    }
                    _ => {}
                }
            }

            if let Some(t) = cursor_s {
                plot_ui.vline(VLine::new(t).color(CURSOR_COL).width(1.0));
            }

            captured_bounds = Some(plot_ui.plot_bounds());
        });

    // ── Post-plot: store rect + bounds, reset, Y2 indicator ──────────────────
    ui.ctx().data_mut(|d| d.insert_temp(rect_id, plot_resp.response.rect));
    if let Some(b) = captured_bounds {
        let arr = [b.min()[0], b.min()[1], b.max()[0], b.max()[1]];
        ui.ctx().data_mut(|d| d.insert_temp(bounds_id, arr));
    }
    if plot_resp.response.double_clicked() {
        *y2_zoom = 1.0;
    }

    // Highlight axis zones on hover (painted on top of plot)
    if let Some(pos) = ptr_pos {
        let painter = ui.painter();
        if in_zone(x_zone, pos) {
            painter.rect_filled(x_zone.unwrap(), 0.0, ACCENT.gamma_multiply(0.10));
        } else if in_zone(y1_zone, pos) {
            painter.rect_filled(y1_zone.unwrap(), 0.0, ACCENT.gamma_multiply(0.10));
        } else if in_zone(y2_zone, pos) {
            painter.rect_filled(y2_zone.unwrap(), 0.0, AMBER.gamma_multiply(0.10));
        }
    }
    if (*y2_zoom - 1.0).abs() > 0.05 {
        let r = plot_resp.response.rect;
        ui.painter().text(
            egui::pos2(r.right() - 6.0, r.top() + 4.0),
            egui::Align2::RIGHT_TOP,
            format!("Y2 ×{:.1}", 1.0 / *y2_zoom),
            FontId::proportional(9.5), AMBER,
        );
    }

    // ── Hover tooltip (phase + channel values) ────────────────────────────────
    let hover_t = ui.ctx().input(|i| i.pointer.hover_pos())
        .filter(|_| plot_resp.response.hovered())
        .map(|pos| plot_resp.transform.value_from_position(pos).x);
    if let Some(ht) = hover_t {
        let rect = plot_resp.response.rect;
        let cur_phase = phase_spans.iter()
            .find(|(ts, te, _)| ht >= *ts && ht < *te)
            .map(|(_, _, p)| *p);

        let t_abs = ht + t0;
        let row_i = rows.partition_point(|r| r.ts() < t_abs).min(rows.len().saturating_sub(1));
        let row   = &rows[row_i];

        // Collect channel values
        let value_lines: Vec<(Color32, String)> = panel.channels.iter()
            .filter_map(|ch| {
                ch.key.value(row, cfg).map(|v| {
                    (ch.key.color(), format!("{}: {:.2}{}", ch.key.label(), v, ch.key.unit()))
                })
            })
            .collect();

        let n_lines = value_lines.len() + cur_phase.is_some() as usize;
        if n_lines > 0 {
            let line_h = 15.0_f32;
            let box_w  = 155.0_f32;
            let box_h  = n_lines as f32 * line_h + 10.0;
            let pos    = egui::pos2(rect.left() + 6.0, rect.top() + 4.0);
            let bx     = egui::Rect::from_min_size(pos, egui::vec2(box_w, box_h));
            let p      = ui.painter();
            p.rect_filled(bx, 4.0, super::palette::BG2.gamma_multiply(0.93));
            let mut y = pos.y + 5.0;
            if let Some(phase) = cur_phase {
                p.text(egui::pos2(pos.x + 6.0, y), egui::Align2::LEFT_TOP,
                    format!("● {}", phase.as_str()),
                    FontId::proportional(10.5), phase_color(&phase));
                y += line_h;
            }
            for (col, text) in &value_lines {
                p.text(egui::pos2(pos.x + 6.0, y), egui::Align2::LEFT_TOP,
                    text, FontId::proportional(10.0), *col);
                y += line_h;
            }
        }
    }

    // ── Right Y axis labels ───────────────────────────────────────────────────
    if has_right {
        if let Some(_bounds) = captured_bounds {
            let rect       = plot_resp.response.rect;
            let data_top   = rect.top()    + 6.0;
            let data_bot   = rect.bottom() - 22.0;
            let data_right = rect.right()  - 6.0;
            let painter    = ui.painter();

            painter.line_segment(
                [egui::pos2(data_right, data_top), egui::pos2(data_right, data_bot)],
                egui::Stroke::new(1.0, FG_DIM),
            );
            let font = FontId::proportional(9.0);
            for i in 0..=5usize {
                let t  = i as f32 / 5.0;
                let sy = data_top + t * (data_bot - data_top);
                let v  = adj_r_max - t as f64 * (adj_r_max - adj_r_min);
                painter.line_segment(
                    [egui::pos2(data_right, sy), egui::pos2(data_right + 4.0, sy)],
                    egui::Stroke::new(1.0, FG_DIM),
                );
                let label = if (adj_r_max - adj_r_min).abs() > 10.0 { format!("{v:.0}") } else { format!("{v:.2}") };
                painter.text(egui::pos2(data_right + 5.0, sy),
                    egui::Align2::LEFT_CENTER, label, font.clone(), FG_DIM);
            }
            let r_label: String = r_channels.iter()
                .map(|c| format!("{} ({})", c.key.label(), c.key.unit()))
                .collect::<Vec<_>>().join(", ");
            painter.text(egui::pos2(data_right + 28.0, data_top - 2.0),
                egui::Align2::CENTER_BOTTOM, r_label,
                FontId::proportional(8.5), FG_DIM);
        }
    }

    hover_t
}

// ── Timeline ──────────────────────────────────────────────────────────────────

fn show_timeline(
    ui: &mut Ui,
    t_max: f64,
    cursor_s: &mut Option<f64>,
    is_playing: &mut bool,
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

    let btn = vec2(28.0, 28.0);
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
        let (mean, sigma) = compute_stats(rows, key, cfg);
        let cur = cur_i.and_then(|i| key.value(&rows[i], cfg));
        channel_card(ui, key, mn, mx, mean, sigma, cur);
    }
}

fn channel_card(ui: &mut Ui, key: &ChannelKey, mn: f64, mx: f64, mean: f64, sigma: f64, cur: Option<f64>) {
    egui::Frame::NONE
        .fill(BG3)
        .stroke(egui::Stroke::new(1.5, key.color()))
        .corner_radius(4.0)
        .inner_margin(egui::Margin { left: 8, right: 8, top: 4, bottom: 4 })
        .show(ui, |ui| {
            ui.set_width(125.0);
            // Label row
            ui.horizontal(|ui| {
                ui.label(RichText::new(key.label()).color(key.color()).size(10.0).strong());
                if key.is_computed() {
                    ui.label(RichText::new("~").color(AMBER).size(9.0));
                }
            });
            // Current value
            let val_str = match cur {
                Some(v) if !v.is_nan() => format!("{v:.1} {}", key.unit()),
                _ => "—".into(),
            };
            ui.label(RichText::new(val_str).color(FG).size(14.0).strong());
            // μ ± σ
            ui.label(RichText::new(format!("μ={mean:.1}  σ={sigma:.1}"))
                .color(FG_DIM).size(9.5));
            // min — max
            ui.label(RichText::new(format!("{mn:.1} — {mx:.1}"))
                .color(FG_DIM.gamma_multiply(0.7)).size(8.5));
        });
    ui.add_space(4.0);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn phase_spans(rows: &[Row], t0: f64) -> Vec<(f64, f64, Phase)> {
    if rows.is_empty() { return vec![]; }
    let mut spans = vec![];
    let mut cur   = rows[0].phase;
    let mut start = rows[0].ts() - t0;
    for r in rows.iter().skip(1) {
        if r.phase != cur {
            spans.push((start, r.ts() - t0, cur));
            cur   = r.phase;
            start = r.ts() - t0;
        }
    }
    spans.push((start, rows.last().unwrap().ts() - t0, cur));
    spans
}

fn compute_stats(rows: &[Row], key: &ChannelKey, cfg: &AppConfig) -> (f64, f64) {
    let mut sum  = 0.0_f64;
    let mut sum2 = 0.0_f64;
    let mut n    = 0usize;
    for row in rows {
        if let Some(v) = key.value(row, cfg) {
            sum  += v;
            sum2 += v * v;
            n    += 1;
        }
    }
    if n == 0 { return (0.0, 0.0); }
    let mean     = sum / n as f64;
    let variance = (sum2 / n as f64) - mean * mean;
    (mean, variance.max(0.0).sqrt())
}

fn build_pts(
    ts: &[f64],
    rows: &[Row],
    step: usize,
    key: &ChannelKey,
    cfg: &AppConfig,
    filter_min: Option<f64>,
    filter_max: Option<f64>,
) -> Vec<[f64; 2]> {
    ts.iter()
        .zip(rows.iter().step_by(step))
        .filter_map(|(&t, row)| {
            let y = key.value(row, cfg)?;
            if filter_min.map(|m| y < m).unwrap_or(false) { return None; }
            if filter_max.map(|m| y > m).unwrap_or(false) { return None; }
            Some([t, y])
        })
        .collect()
}

fn value_range(iter: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let (mn, mx) = iter.filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), v| (mn.min(v), mx.max(v)));
    if mn.is_finite() && mx.is_finite() && mx > mn { Some((mn, mx)) } else { None }
}

fn normalize(v: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    if (from_max - from_min).abs() < 1e-10 { return (to_min + to_max) / 2.0; }
    to_min + (v - from_min) / (from_max - from_min) * (to_max - to_min)
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
