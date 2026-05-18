use egui::Ui;
use egui_plot::{HLine, Line, Plot, PlotPoints, Points};
use crate::analysis::Thresholds as THR;
use crate::data::types::*;
use super::palette::*;

pub fn show(ui: &mut Ui, d: &SessionData) {
    let rows = &d.rows;
    let t0 = rows[0].ts();

    let step = (rows.len() / 4000).max(1);
    let ts: Vec<f64>    = rows.iter().step_by(step).map(|r| r.ts() - t0).collect();
    let rpms: Vec<f64>  = rows.iter().step_by(step).map(|r| r.rpm).collect();
    let loads: Vec<f64> = rows.iter().step_by(step).map(|r| r.load_pct).collect();
    let stft1: Vec<f64> = rows.iter().step_by(step).map(|r| if r.cl_b1 { r.stft_b1 } else { f64::NAN }).collect();
    let stft2: Vec<f64> = rows.iter().step_by(step).map(|r| if r.cl_b2 { r.stft_b2 } else { f64::NAN }).collect();
    let lmb1: Vec<f64>  = rows.iter().step_by(step).map(|r| if (0.5..2.5).contains(&r.lmb_b1) { r.lmb_b1 } else { f64::NAN }).collect();
    let lmb2: Vec<f64>  = rows.iter().step_by(step).map(|r| if (0.5..2.5).contains(&r.lmb_b2) { r.lmb_b2 } else { f64::NAN }).collect();
    let ign: Vec<f64>   = rows.iter().step_by(step).map(|r| if r.ign_deg.abs() > 0.001 { r.ign_deg } else { f64::NAN }).collect();
    let cool: Vec<f64>  = rows.iter().step_by(step).map(|r| r.coolant_c).collect();

    let to_points = |xs: &[f64], ys: &[f64]| -> PlotPoints {
        PlotPoints::new(xs.iter().zip(ys.iter()).filter(|(_, y)| !y.is_nan()).map(|(&x, &y)| [x, y]).collect())
    };

    let avail_h  = ui.available_height();
    let panel_h  = (avail_h / 4.0 - 4.0).max(80.0);
    let dashed   = egui_plot::LineStyle::Dashed { length: 10.0 };
    let dotted   = egui_plot::LineStyle::Dotted { spacing: 6.0 };

    // ── Panel 1: RPM + Load ─────────────────────────────────────────────────
    Plot::new("ts_rpm")
        .height(panel_h)
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new(to_points(&ts, &rpms)).color(ACCENT).width(1.0).name("RPM"));
            plot_ui.line(Line::new(to_points(&ts, &loads)).color(YELLOW).width(0.8).name("Load %"));
        });

    // ── Panel 2: STFT ──────────────────────────────────────────────────────
    Plot::new("ts_stft")
        .height(panel_h)
        .include_y(-35.0)
        .include_y(35.0)
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new(to_points(&ts, &stft1)).color(RED).width(0.9).name("STFT B1"));
            plot_ui.line(Line::new(to_points(&ts, &stft2)).color(ACCENT).width(0.9).name("STFT B2"));

            // 60s bucket medians
            if !d.stft_buckets.is_empty() {
                let b1pts: Vec<[f64; 2]> = d.stft_buckets.iter()
                    .filter_map(|b| b.p50_b1.map(|v| [b.t_s - t0, v])).collect();
                let b2pts: Vec<[f64; 2]> = d.stft_buckets.iter()
                    .filter_map(|b| b.p50_b2.map(|v| [b.t_s - t0, v])).collect();
                if !b1pts.is_empty() {
                    plot_ui.points(Points::new(PlotPoints::new(b1pts)).color(RED).radius(4.0).name("B1 60s med"));
                }
                if !b2pts.is_empty() {
                    plot_ui.points(Points::new(PlotPoints::new(b2pts)).color(ACCENT).radius(4.0).name("B2 60s med"));
                }
            }

            for &thr_v in &[-THR::STFT_ALARM, THR::STFT_ALARM] {
                plot_ui.hline(HLine::new(thr_v).color(RED).style(dotted));
            }
            for &thr_v in &[-THR::STFT_WARN, THR::STFT_WARN] {
                plot_ui.hline(HLine::new(thr_v).color(ORANGE).style(dotted));
            }
            plot_ui.hline(HLine::new(0.0).color(FG_DIM).style(dashed));

            // Anomaly markers
            let ev_y = |kind: &str| -> f64 {
                match kind {
                    "LEAN_B1" => 28.0, "LEAN_B2" => 26.0,
                    "RICH_B1" => -28.0, "RICH_B2" => -26.0,
                    "STFT_B1" => 32.0, "STFT_B2" => 30.0,
                    _ => 0.0,
                }
            };
            let ev_col = |kind: &str| -> egui::Color32 {
                match kind {
                    "LEAN_B1" | "LEAN_B2" | "STFT_B1" | "STFT_B2" | "OVERTEMP" => RED,
                    "RICH_B1" | "RICH_B2" => PURPLE,
                    _ => YELLOW,
                }
            };
            for ev in d.events.iter().take(40) {
                let et = ev.time_s - t0;
                let y = ev_y(&ev.kind);
                plot_ui.points(Points::new(PlotPoints::new(vec![[et, y]]))
                    .color(ev_col(&ev.kind)).radius(4.0)
                    .shape(egui_plot::MarkerShape::Up)
                    .name(&ev.kind));
            }
        });

    // ── Panel 3: Lambda ────────────────────────────────────────────────────
    if d.has_lmb {
        Plot::new("ts_lmb")
            .height(panel_h)
            .include_y(0.78)
            .include_y(1.18)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(to_points(&ts, &lmb1)).color(PURPLE).width(0.9).name("λ B1"));
                plot_ui.line(Line::new(to_points(&ts, &lmb2)).color(ORANGE).width(0.9).name("λ B2"));
                plot_ui.hline(HLine::new(1.00).color(GREEN).style(dashed));
                plot_ui.hline(HLine::new(THR::LMB_LEAN_LOAD).color(RED).style(dotted));
                plot_ui.hline(HLine::new(THR::LMB_RICH).color(ORANGE).style(dotted));
            });
    }

    // ── Panel 4: Ignition + Coolant ────────────────────────────────────────
    if d.has_ign {
        Plot::new("ts_ign")
            .height(panel_h)
            .x_axis_label("Time (s)")
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(to_points(&ts, &ign)).color(GREEN).width(0.9).name("Ign °BTDC"));
                plot_ui.line(Line::new(to_points(&ts, &cool)).color(ORANGE).width(0.7).name("Coolant °C"));
                plot_ui.hline(HLine::new(THR::COOLANT_WARN).color(RED).style(dotted));
            });
    }
}
