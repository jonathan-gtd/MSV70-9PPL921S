use egui::Ui;
use egui_plot::{Bar, BarChart, HLine, Line, Plot, PlotPoints, Points};
use crate::analysis::percentile;
use crate::analysis::Thresholds as THR;
use crate::data::types::*;
use super::palette::*;

pub fn show(ui: &mut Ui, d: &SessionData) {
    let rows  = &d.rows;
    let cl_b1 = &d.cl_b1_idx;
    let cl_b2 = &d.cl_b2_idx;
    let run   = &d.run_idx;

    let width = ui.available_width();
    let half  = width / 2.0 - 6.0;
    let height = ui.available_height() / 2.0 - 6.0;

    ui.horizontal(|ui| {
        // ── Phase distribution ──────────────────────────────────────────────
        ui.allocate_ui([half, height].into(), |ui| {
            ui.label(egui::RichText::new("Phase Distribution").color(ACCENT).strong());
            let ph_order = [Phase::HotIdle, Phase::Warmup, Phase::Cruise, Phase::Accel, Phase::Wot, Phase::FuelCut];
            let bars: Vec<Bar> = ph_order.iter().enumerate().filter_map(|(idx, ph)| {
                let dur = d.phase_dur.get(ph).copied().unwrap_or(0.0);
                if dur < 1.0 { return None; }
                Some(Bar::new(idx as f64, dur)
                    .fill(phase_color(ph))
                    .name(ph.as_str()))
            }).collect();
            if !bars.is_empty() {
                Plot::new("phase_dist")
                    .height(height - 24.0)
                    .show_axes(true)
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(BarChart::new(bars));
                    });
            }
        });

        // ── STFT per phase ──────────────────────────────────────────────────
        ui.allocate_ui([half, height].into(), |ui| {
            ui.label(egui::RichText::new("STFT Median by Phase").color(ACCENT).strong());
            let cl_phases = [Phase::HotIdle, Phase::Warmup, Phase::Cruise, Phase::Accel, Phase::Wot];
            let b1_bars: Vec<Bar> = cl_phases.iter().enumerate().filter_map(|(idx, ph)| {
                let vals: Vec<f64> = cl_b1.iter().filter(|&&i| rows[i].phase == *ph).map(|&i| rows[i].stft_b1).collect();
                if vals.is_empty() { return None; }
                let med = percentile_of(&vals, 50.0);
                Some(Bar::new(idx as f64 - 0.2, med).fill(stft_color(med)).width(0.35).name("B1"))
            }).collect();
            let b2_bars: Vec<Bar> = cl_phases.iter().enumerate().filter_map(|(idx, ph)| {
                let vals: Vec<f64> = cl_b2.iter().filter(|&&i| rows[i].phase == *ph).map(|&i| rows[i].stft_b2).collect();
                if vals.is_empty() { return None; }
                let med = percentile_of(&vals, 50.0);
                Some(Bar::new(idx as f64 + 0.2, med).fill(stft_color(med).gamma_multiply(0.7)).width(0.35).name("B2"))
            }).collect();
            Plot::new("stft_phase")
                .height(height - 24.0)
                .show(ui, |plot_ui| {
                    if !b1_bars.is_empty() { plot_ui.bar_chart(BarChart::new(b1_bars)); }
                    if !b2_bars.is_empty() { plot_ui.bar_chart(BarChart::new(b2_bars)); }
                    plot_ui.hline(HLine::new(0.0).color(FG_DIM).style(egui_plot::LineStyle::Dashed { length: 10.0 }));
                    for &thr_v in &[-THR::STFT_WARN, THR::STFT_WARN] {
                        plot_ui.hline(HLine::new(thr_v).color(RED).style(egui_plot::LineStyle::Dotted { spacing: 6.0 }));
                    }
                });
        });
    });

    ui.horizontal(|ui| {
        // ── Lambda vs Load ──────────────────────────────────────────────────
        ui.allocate_ui([half, height].into(), |ui| {
            ui.label(egui::RichText::new("Lambda vs Load").color(ACCENT).strong());
            if d.has_lmb && !cl_b1.is_empty() {
                let mut pts_b1: Vec<[f64; 2]> = Vec::new();
                let mut pts_b2: Vec<[f64; 2]> = Vec::new();
                for lo in (0..95).step_by(5) {
                    let hi = lo + 5;
                    let r1: Vec<f64> = cl_b1.iter()
                        .filter(|&&i| (lo as f64) <= rows[i].load_pct && rows[i].load_pct < (hi as f64) && (0.5..2.0).contains(&rows[i].lmb_b1))
                        .map(|&i| rows[i].lmb_b1).collect();
                    let r2: Vec<f64> = cl_b2.iter()
                        .filter(|&&i| (lo as f64) <= rows[i].load_pct && rows[i].load_pct < (hi as f64) && (0.5..2.0).contains(&rows[i].lmb_b2))
                        .map(|&i| rows[i].lmb_b2).collect();
                    if r1.len() >= 5 { pts_b1.push([lo as f64 + 2.5, percentile_of(&r1, 50.0)]); }
                    if r2.len() >= 5 { pts_b2.push([lo as f64 + 2.5, percentile_of(&r2, 50.0)]); }
                }
                Plot::new("lmb_load")
                    .height(height - 24.0)
                    .include_y(0.85)
                    .include_y(1.08)
                    .show(ui, |plot_ui| {
                        if !pts_b1.is_empty() {
                            plot_ui.line(Line::new(PlotPoints::new(pts_b1)).color(ACCENT).width(2.0).name("λ B1"));
                        }
                        if !pts_b2.is_empty() {
                            plot_ui.line(Line::new(PlotPoints::new(pts_b2)).color(ORANGE).width(2.0).name("λ B2"));
                        }
                        plot_ui.hline(HLine::new(1.00).color(GREEN).style(egui_plot::LineStyle::Dashed { length: 10.0 }));
                        plot_ui.hline(HLine::new(0.92).color(YELLOW).style(egui_plot::LineStyle::Dotted { spacing: 6.0 }));
                        plot_ui.hline(HLine::new(THR::LMB_LEAN_LOAD).color(RED).style(egui_plot::LineStyle::Dotted { spacing: 6.0 }));
                    });
            }
        });

        // ── Injector DC vs RPM ──────────────────────────────────────────────
        ui.allocate_ui([half, height].into(), |ui| {
            ui.label(egui::RichText::new("Injector Duty Cycle vs RPM").color(ACCENT).strong());
            if d.has_inj && !run.is_empty() {
                let step = (run.len() / 2000).max(1);
                let pts: Vec<[f64; 2]> = run.iter().step_by(step)
                    .filter_map(|&i| rows[i].inj_duty().map(|dc| [rows[i].rpm, dc]))
                    .collect();
                let (red_pts, orange_pts, normal_pts): (Vec<_>, Vec<_>, Vec<_>) = pts.iter().fold(
                    (vec![], vec![], vec![]),
                    |(mut r, mut o, mut n), &pt| {
                        if pt[1] > 80.0 { r.push(pt); } else if pt[1] > 60.0 { o.push(pt); } else { n.push(pt); }
                        (r, o, n)
                    }
                );
                Plot::new("inj_dc")
                    .height(height - 24.0)
                    .show(ui, |plot_ui| {
                        if !normal_pts.is_empty() {
                            plot_ui.points(Points::new(PlotPoints::new(normal_pts)).color(ACCENT).radius(1.0).name("DC normal"));
                        }
                        if !orange_pts.is_empty() {
                            plot_ui.points(Points::new(PlotPoints::new(orange_pts)).color(ORANGE).radius(1.5).name("DC warn"));
                        }
                        if !red_pts.is_empty() {
                            plot_ui.points(Points::new(PlotPoints::new(red_pts)).color(RED).radius(2.0).name("DC alarm"));
                        }
                        plot_ui.hline(HLine::new(80.0).color(RED).style(egui_plot::LineStyle::Dashed { length: 10.0 }));
                        plot_ui.hline(HLine::new(60.0).color(ORANGE).style(egui_plot::LineStyle::Dotted { spacing: 6.0 }));
                    });
            }
        });
    });
}

fn percentile_of(values: &[f64], p: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    let mut v = values.to_vec();
    percentile(&mut v, p)
}
