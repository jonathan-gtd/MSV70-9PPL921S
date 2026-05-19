use egui::{RichText, Ui};
use egui_plot::{Bar, BarChart, HLine, Line, Plot, PlotPoints, Points};
use crate::analysis::percentile;
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

pub fn show(ui: &mut Ui, sessions: &[SessionData], cfg: &AppConfig) {
    if sessions.is_empty() {
        ui.label(RichText::new("No logs loaded — click '+ All Logs'").color(FG_DIM));
        return;
    }

    let names: Vec<String> = sessions.iter()
        .map(|d| d.file_name.trim_end_matches(".csv").chars().rev().take(12).collect::<String>().chars().rev().collect())
        .collect();

    let b1_meds: Vec<f64> = sessions.iter().map(|d| {
        if d.cl_b1_idx.is_empty() { f64::NAN }
        else {
            let mut vals: Vec<f64> = d.cl_b1_idx.iter().map(|&i| d.rows[i].stft_b1).collect();
            percentile(&mut vals, 50.0)
        }
    }).collect();

    let b2_meds: Vec<f64> = sessions.iter().map(|d| {
        if d.cl_b2_idx.is_empty() { f64::NAN }
        else {
            let mut vals: Vec<f64> = d.cl_b2_idx.iter().map(|&i| d.rows[i].stft_b2).collect();
            percentile(&mut vals, 50.0)
        }
    }).collect();

    let wot_lmbs: Vec<f64> = sessions.iter().map(|d| {
        let wot: Vec<f64> = d.cl_b1_idx.iter()
            .filter(|&&i| d.rows[i].phase == Phase::Wot && (0.3..2.0).contains(&d.rows[i].lmb_b1)
                && d.rows[i].rpm > 2500.0 && d.rows[i].coolant_c > 75.0)
            .map(|&i| d.rows[i].lmb_b1).collect();
        if wot.is_empty() { f64::NAN } else { wot.iter().sum::<f64>() / wot.len() as f64 }
    }).collect();

    let dc_maxes: Vec<f64> = sessions.iter().map(|d| {
        if !d.has_inj { return f64::NAN; }
        let m = d.run_idx.iter().filter_map(|&i| d.rows[i].inj_duty()).fold(f64::NEG_INFINITY, f64::max);
        if m == f64::NEG_INFINITY { f64::NAN } else { m }
    }).collect();

    let n_events: Vec<usize> = sessions.iter().map(|d| d.events.len()).collect();
    let durations: Vec<f64>  = sessions.iter().map(|d| d.dur_s / 60.0).collect();

    let avail_h = ui.available_height();
    let panel_h = (avail_h / 3.0 - 4.0).max(80.0);
    let dashed  = egui_plot::LineStyle::Dashed { length: 10.0 };
    let dotted  = egui_plot::LineStyle::Dotted { spacing: 6.0 };

    ui.label(RichText::new(format!("STFT Trend — target ±{:.0}% band", cfg.stft_note)).color(ACCENT).strong());
    Plot::new("trend_stft")
        .height(panel_h)
        .show(ui, |plot_ui| {
            let b1pts: Vec<[f64; 2]> = b1_meds.iter().enumerate().filter(|(_, v)| !v.is_nan()).map(|(i, &v)| [i as f64, v]).collect();
            let b2pts: Vec<[f64; 2]> = b2_meds.iter().enumerate().filter(|(_, v)| !v.is_nan()).map(|(i, &v)| [i as f64, v]).collect();
            if !b1pts.is_empty() {
                plot_ui.line(Line::new(PlotPoints::new(b1pts.clone())).color(RED).width(2.0).name("B1 STFT p50"));
                plot_ui.points(Points::new(PlotPoints::new(b1pts)).color(RED).radius(5.0).name("B1 pts"));
            }
            if !b2pts.is_empty() {
                plot_ui.line(Line::new(PlotPoints::new(b2pts.clone())).color(ACCENT).width(2.0).name("B2 STFT p50"));
                plot_ui.points(Points::new(PlotPoints::new(b2pts)).color(ACCENT).radius(5.0).name("B2 pts"));
            }
            plot_ui.hline(HLine::new(0.0).color(FG_DIM).style(dashed));
            for &tv in &[-cfg.stft_warn, cfg.stft_warn] {
                plot_ui.hline(HLine::new(tv).color(RED).style(dotted));
            }
        });

    ui.horizontal(|ui| {
        let half = (ui.available_width() / 2.0 - 4.0).max(100.0);
        let ph2  = panel_h * 0.8;

        ui.allocate_ui([half, ph2 + 20.0].into(), |ui| {
            ui.label(RichText::new("WOT Lambda B1").color(ACCENT));
            Plot::new("trend_wot").height(ph2).show(ui, |plot_ui| {
                let pts: Vec<[f64; 2]> = wot_lmbs.iter().enumerate().filter(|(_, v)| !v.is_nan()).map(|(i, &v)| [i as f64, v]).collect();
                if !pts.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(pts.clone())).color(YELLOW).width(1.5).name("WOT λ"));
                    plot_ui.points(Points::new(PlotPoints::new(pts)).color(YELLOW).radius(4.0).name("WOT pts"));
                }
                plot_ui.hline(HLine::new(cfg.lmb_wot_target).color(GREEN).style(dashed));
                plot_ui.hline(HLine::new(cfg.lmb_wot_warn).color(RED).style(dotted));
            });
        });

        ui.allocate_ui([half, ph2 + 20.0].into(), |ui| {
            ui.label(RichText::new("Peak Injector DC").color(ACCENT));
            Plot::new("trend_dc").height(ph2).show(ui, |plot_ui| {
                let bars: Vec<Bar> = dc_maxes.iter().enumerate().filter(|(_, v)| !v.is_nan()).map(|(i, &v)| {
                    let col = if v > cfg.inj_duty_warn { RED } else if v > cfg.inj_duty_warn * 0.75 { ORANGE } else { GREEN };
                    Bar::new(i as f64, v).fill(col)
                }).collect();
                if !bars.is_empty() {
                    plot_ui.bar_chart(BarChart::new(bars));
                }
                plot_ui.hline(HLine::new(cfg.inj_duty_warn).color(RED).style(dashed));
            });
        });
    });

    ui.horizontal(|ui| {
        let half = (ui.available_width() / 2.0 - 4.0).max(100.0);
        let ph2  = panel_h * 0.8;

        ui.allocate_ui([half, ph2 + 20.0].into(), |ui| {
            ui.label(RichText::new("Anomaly Events").color(ACCENT));
            Plot::new("trend_ev").height(ph2).show(ui, |plot_ui| {
                let bars: Vec<Bar> = n_events.iter().enumerate().map(|(i, &n)| {
                    let col = if n > 10 { RED } else if n > 4 { ORANGE } else { GREEN };
                    Bar::new(i as f64, n as f64).fill(col)
                }).collect();
                plot_ui.bar_chart(BarChart::new(bars));
            });
        });

        ui.allocate_ui([half, ph2 + 20.0].into(), |ui| {
            ui.label(RichText::new("Session Duration (min)").color(ACCENT));
            Plot::new("trend_dur").height(ph2).show(ui, |plot_ui| {
                let bars: Vec<Bar> = durations.iter().enumerate().map(|(i, &v)| {
                    Bar::new(i as f64, v).fill(ACCENT)
                }).collect();
                plot_ui.bar_chart(BarChart::new(bars));
            });
        });
    });

    ui.horizontal_wrapped(|ui| {
        for (i, name) in names.iter().enumerate() {
            ui.label(RichText::new(format!("[{i}] {name}")).color(FG_DIM).small());
        }
    });
}
