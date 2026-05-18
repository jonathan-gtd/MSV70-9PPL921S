use std::time::Instant;
use egui::{Color32, RichText, Ui};
use egui_plot::{HLine, Line, Plot, PlotPoints, Points, VLine};
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

const CURSOR_COL: Color32 = Color32::from_rgb(0, 230, 230);

pub fn show(
    ui: &mut Ui,
    d: &SessionData,
    cfg: &AppConfig,
    show_rpm:     bool, show_load:    bool,
    show_stft_b1: bool, show_stft_b2: bool,
    show_lmb_b1:  bool, show_lmb_b2:  bool,
    show_ign:     bool, show_coolant:  bool,
    cursor_s:         &mut Option<f64>,
    is_playing:       &mut bool,
    playback_speed:   &mut f64,
    _last_frame_time: &mut Option<Instant>,
) {
    let rows = &d.rows;
    if rows.is_empty() { return; }
    let t0    = rows[0].ts();
    let t_max = rows.last().unwrap().ts() - t0;

    let step = (rows.len() / cfg.display_max_points).max(1);
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
                    show_channel_cards(
                        ui, rows, t0, *cursor_s,
                        show_rpm, show_load,
                        show_stft_b1, show_stft_b2,
                        show_lmb_b1, show_lmb_b2,
                        show_ign, show_coolant,
                        d.has_lmb, d.has_ign, cfg,
                    );
                });
            });
        });

    // ── Stacked chart panels ──────────────────────────────────────────────────
    egui::CentralPanel::default().show_inside(ui, |ui| {
        let cs     = *cursor_s;
        let dashed = egui_plot::LineStyle::Dashed { length: 10.0 };
        let dotted = egui_plot::LineStyle::Dotted { spacing: 6.0 };

        let n_panels = 2
            + if d.has_lmb { 1 } else { 0 }
            + if d.has_ign  { 1 } else { 0 };
        let avail_h  = ui.available_height();
        let panel_h  = ((avail_h / n_panels as f32) - 4.0).max(80.0);

        // ── RPM / Load ────────────────────────────────────────────────────────
        Plot::new("ts_rpm")
            .height(panel_h)
            .show(ui, |plot_ui| {
                if show_rpm {
                    let rpms: Vec<f64> = rows.iter().step_by(step).map(|r| r.rpm).collect();
                    plot_ui.line(
                        Line::new(PlotPoints::new(to_pts(&ts, &rpms)))
                            .color(CHART[0]).width(1.5).name("RPM"),
                    );
                }
                if show_load {
                    let loads: Vec<f64> = rows.iter().step_by(step).map(|r| r.load_pct).collect();
                    plot_ui.line(
                        Line::new(PlotPoints::new(to_pts(&ts, &loads)))
                            .color(CHART[4]).width(1.2).name("Load %"),
                    );
                }
                vline(plot_ui, cs);
            });

        // ── STFT ──────────────────────────────────────────────────────────────
        Plot::new("ts_stft")
            .height(panel_h)
            .include_y(-35.0)
            .include_y(35.0)
            .show(ui, |plot_ui| {
                if show_stft_b1 {
                    let s1: Vec<f64> = rows.iter().step_by(step)
                        .map(|r| if r.cl_b1 { r.stft_b1 } else { f64::NAN }).collect();
                    plot_ui.line(
                        Line::new(PlotPoints::new(to_pts(&ts, &s1)))
                            .color(CHART[1]).width(1.2).name("STFT B1"),
                    );
                }
                if show_stft_b2 {
                    let s2: Vec<f64> = rows.iter().step_by(step)
                        .map(|r| if r.cl_b2 { r.stft_b2 } else { f64::NAN }).collect();
                    plot_ui.line(
                        Line::new(PlotPoints::new(to_pts(&ts, &s2)))
                            .color(CHART[0]).width(1.2).name("STFT B2"),
                    );
                }

                if !d.stft_buckets.is_empty() {
                    let b1pts: Vec<[f64; 2]> = d.stft_buckets.iter()
                        .filter_map(|b| b.p50_b1.map(|v| [b.t_s - t0, v])).collect();
                    let b2pts: Vec<[f64; 2]> = d.stft_buckets.iter()
                        .filter_map(|b| b.p50_b2.map(|v| [b.t_s - t0, v])).collect();
                    if !b1pts.is_empty() {
                        plot_ui.points(
                            Points::new(PlotPoints::new(b1pts))
                                .color(CHART[1]).radius(4.0).name("B1 60s med"),
                        );
                    }
                    if !b2pts.is_empty() {
                        plot_ui.points(
                            Points::new(PlotPoints::new(b2pts))
                                .color(CHART[0]).radius(4.0).name("B2 60s med"),
                        );
                    }
                }

                plot_ui.hline(HLine::new(0.0).color(FG_DIM).style(dashed));
                for &v in &[-cfg.stft_alarm, cfg.stft_alarm] {
                    plot_ui.hline(HLine::new(v).color(RED).style(dotted));
                }
                for &v in &[-cfg.stft_warn, cfg.stft_warn] {
                    plot_ui.hline(HLine::new(v).color(ORANGE).style(dotted));
                }

                let ev_y = |kind: &str| -> f64 {
                    match kind {
                        "LEAN_B1" => 28.0,  "LEAN_B2" => 26.0,
                        "RICH_B1" => -28.0, "RICH_B2" => -26.0,
                        "STFT_B1" => 32.0,  "STFT_B2" => 30.0,
                        _ => 0.0,
                    }
                };
                let ev_col = |kind: &str| -> Color32 {
                    match kind {
                        "LEAN_B1" | "LEAN_B2" | "STFT_B1" | "STFT_B2" | "OVERTEMP" => RED,
                        "RICH_B1" | "RICH_B2" => PURPLE,
                        _ => YELLOW,
                    }
                };
                for ev in d.events.iter().take(40) {
                    let et = ev.time_s - t0;
                    let y  = ev_y(&ev.kind);
                    plot_ui.points(
                        Points::new(PlotPoints::new(vec![[et, y]]))
                            .color(ev_col(&ev.kind)).radius(4.0)
                            .shape(egui_plot::MarkerShape::Up)
                            .name(&ev.kind),
                    );
                }
                vline(plot_ui, cs);
            });

        // ── Lambda ────────────────────────────────────────────────────────────
        if d.has_lmb {
            Plot::new("ts_lmb")
                .height(panel_h)
                .include_y(0.78)
                .include_y(1.18)
                .show(ui, |plot_ui| {
                    if show_lmb_b1 {
                        let l1: Vec<f64> = rows.iter().step_by(step)
                            .map(|r| if (0.5..2.5).contains(&r.lmb_b1) { r.lmb_b1 } else { f64::NAN }).collect();
                        plot_ui.line(
                            Line::new(PlotPoints::new(to_pts(&ts, &l1)))
                                .color(CHART[6]).width(1.2).name("λ B1"),
                        );
                    }
                    if show_lmb_b2 {
                        let l2: Vec<f64> = rows.iter().step_by(step)
                            .map(|r| if (0.5..2.5).contains(&r.lmb_b2) { r.lmb_b2 } else { f64::NAN }).collect();
                        plot_ui.line(
                            Line::new(PlotPoints::new(to_pts(&ts, &l2)))
                                .color(CHART[8]).width(1.2).name("λ B2"),
                        );
                    }
                    plot_ui.hline(HLine::new(1.00).color(GREEN).style(dashed));
                    plot_ui.hline(HLine::new(cfg.lmb_lean_load).color(RED).style(dotted));
                    plot_ui.hline(HLine::new(cfg.lmb_rich).color(ORANGE).style(dotted));
                    vline(plot_ui, cs);
                });
        }

        // ── Ignition / Coolant ────────────────────────────────────────────────
        if d.has_ign {
            Plot::new("ts_ign")
                .height(panel_h)
                .x_axis_label("Time (s)")
                .show(ui, |plot_ui| {
                    if show_ign {
                        let ign: Vec<f64> = rows.iter().step_by(step)
                            .map(|r| if r.ign_deg.abs() > cfg.near_zero { r.ign_deg } else { f64::NAN }).collect();
                        plot_ui.line(
                            Line::new(PlotPoints::new(to_pts(&ts, &ign)))
                                .color(CHART[9]).width(1.2).name("Ign °BTDC"),
                        );
                    }
                    if show_coolant {
                        let cool: Vec<f64> = rows.iter().step_by(step).map(|r| r.coolant_c).collect();
                        plot_ui.line(
                            Line::new(PlotPoints::new(to_pts(&ts, &cool)))
                                .color(CHART[3]).width(0.9).name("Coolant °C"),
                        );
                    }
                    plot_ui.hline(HLine::new(cfg.coolant_warn).color(RED).style(dotted));
                    vline(plot_ui, cs);
                });
        }
    });
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

    // Time labels
    ui.horizontal(|ui| {
        ui.label(RichText::new(fmt_time(0.0)).color(FG_DIM).size(11.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(fmt_time(t_max)).color(FG_DIM).size(11.0));
        });
    });

    // Full-width slider
    let old_sw = ui.spacing().slider_width;
    ui.spacing_mut().slider_width = (ui.available_width() - 12.0).max(100.0);
    let slider_r = ui.add(
        egui::Slider::new(cur, 0.0..=t_max)
            .show_value(false)
            .clamping(egui::SliderClamping::Always),
    );
    ui.spacing_mut().slider_width = old_sw;
    if slider_r.changed() { *is_playing = false; }

    // Controls row
    let btn = egui::vec2(28.0, 28.0);
    ui.horizontal(|ui| {
        // Play / Pause
        let (play_text, play_col) = if *is_playing { ("⏸", AMBER) } else { ("▶", LGREEN) };
        if ui.add(egui::Button::new(
            RichText::new(play_text).size(16.0).color(play_col)
        ).min_size(btn)).clicked() {
            *is_playing = !*is_playing;
        }

        // Stop
        if ui.add(egui::Button::new(
            RichText::new("⏹").size(16.0).color(RUST)
        ).min_size(btn)).clicked() {
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
        ui.label(
            RichText::new(format!("⏱ {}", fmt_time(*cur)))
                .color(CURSOR_COL).size(13.0).strong(),
        );
    });
}

// ── Channel cards ─────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn show_channel_cards(
    ui: &mut Ui,
    rows: &[Row],
    t0: f64,
    cursor_s: Option<f64>,
    show_rpm: bool, show_load: bool,
    show_stft_b1: bool, show_stft_b2: bool,
    show_lmb_b1: bool, show_lmb_b2: bool,
    show_ign: bool, show_coolant: bool,
    has_lmb: bool, has_ign: bool,
    cfg: &AppConfig,
) {
    let cur_i = cursor_s.map(|cs| {
        let target = cs + t0;
        rows.partition_point(|r| r.ts() < target).min(rows.len().saturating_sub(1))
    });

    if show_rpm {
        let (mn, mx) = minmax(rows, |r| r.rpm);
        channel_card(ui, "RPM", CHART[0], mn, mx, cur_i.map(|i| rows[i].rpm), "rpm");
    }
    if show_load {
        let (mn, mx) = minmax(rows, |r| r.load_pct);
        channel_card(ui, "Load %", CHART[4], mn, mx, cur_i.map(|i| rows[i].load_pct), "%");
    }
    if show_stft_b1 {
        let vals: Vec<f64> = rows.iter().filter(|r| r.cl_b1).map(|r| r.stft_b1).collect();
        let (mn, mx) = minmax_slice(&vals);
        let cur = cur_i.map(|i| if rows[i].cl_b1 { rows[i].stft_b1 } else { f64::NAN });
        channel_card(ui, "STFT B1", CHART[1], mn, mx, cur, "%");
    }
    if show_stft_b2 {
        let vals: Vec<f64> = rows.iter().filter(|r| r.cl_b2).map(|r| r.stft_b2).collect();
        let (mn, mx) = minmax_slice(&vals);
        let cur = cur_i.map(|i| if rows[i].cl_b2 { rows[i].stft_b2 } else { f64::NAN });
        channel_card(ui, "STFT B2", CHART[0], mn, mx, cur, "%");
    }
    if has_lmb {
        if show_lmb_b1 {
            let vals: Vec<f64> = rows.iter().map(|r| r.lmb_b1).filter(|&v| (0.5..2.5).contains(&v)).collect();
            let (mn, mx) = minmax_slice(&vals);
            channel_card(ui, "λ Bank 1", CHART[6], mn, mx, cur_i.map(|i| rows[i].lmb_b1), "λ");
        }
        if show_lmb_b2 {
            let vals: Vec<f64> = rows.iter().map(|r| r.lmb_b2).filter(|&v| (0.5..2.5).contains(&v)).collect();
            let (mn, mx) = minmax_slice(&vals);
            channel_card(ui, "λ Bank 2", CHART[8], mn, mx, cur_i.map(|i| rows[i].lmb_b2), "λ");
        }
    }
    if has_ign {
        if show_ign {
            let vals: Vec<f64> = rows.iter().map(|r| r.ign_deg).filter(|&v| v.abs() > cfg.near_zero).collect();
            let (mn, mx) = minmax_slice(&vals);
            channel_card(ui, "Ignition", CHART[9], mn, mx, cur_i.map(|i| rows[i].ign_deg), "°");
        }
        if show_coolant {
            let (mn, mx) = minmax(rows, |r| r.coolant_c);
            channel_card(ui, "Coolant", CHART[3], mn, mx, cur_i.map(|i| rows[i].coolant_c), "°C");
        }
    }
}

fn channel_card(
    ui: &mut Ui,
    name: &str,
    color: Color32,
    mn: f64, mx: f64,
    cur: Option<f64>,
    unit: &str,
) {
    egui::Frame::NONE
        .fill(BG3)
        .stroke(egui::Stroke::new(1.5, color))
        .corner_radius(4.0)
        .inner_margin(egui::Margin { left: 8, right: 8, top: 4, bottom: 4 })
        .show(ui, |ui| {
            ui.set_width(115.0);
            ui.label(RichText::new(name).color(color).size(10.0).strong());
            let val_str = match cur {
                Some(v) if !v.is_nan() => format!("{v:.1} {unit}"),
                _ => "—".into(),
            };
            ui.label(RichText::new(val_str).color(FG).size(14.0).strong());
            ui.label(RichText::new(format!("{mn:.1} — {mx:.1}")).color(FG_DIM).size(9.0));
        });
    ui.add_space(4.0);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn to_pts(xs: &[f64], ys: &[f64]) -> Vec<[f64; 2]> {
    xs.iter()
        .zip(ys.iter())
        .filter(|(_, y)| !y.is_nan())
        .map(|(&x, &y)| [x, y])
        .collect()
}

fn vline(plot_ui: &mut egui_plot::PlotUi, cs: Option<f64>) {
    if let Some(t) = cs {
        plot_ui.vline(VLine::new(t).color(CURSOR_COL).width(1.0));
    }
}

fn minmax(rows: &[Row], f: impl Fn(&Row) -> f64) -> (f64, f64) {
    rows.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), r| {
        let v = f(r);
        (mn.min(v), mx.max(v))
    })
}

fn minmax_slice(vals: &[f64]) -> (f64, f64) {
    vals.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(mn, mx), &v| (mn.min(v), mx.max(v)),
    )
}

fn fmt_time(s: f64) -> String {
    let m   = (s as u64) / 60;
    let sec = s % 60.0;
    format!("{m}:{sec:05.2}")
}
