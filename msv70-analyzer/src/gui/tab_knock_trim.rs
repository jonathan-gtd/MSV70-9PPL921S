use egui::{RichText, ScrollArea, Ui};
use egui_plot::{HLine, Line, Plot, PlotPoints, Points};
use crate::data::types::SessionData;
use super::palette::*;

const SAT_LIMIT: f64 = 0.048; // warn near ±0.05 P2096/P2098 saturation

pub fn show(ui: &mut Ui, d: &SessionData) {
    ScrollArea::vertical().id_salt("knock_trim_scroll").show(ui, |ui| {
        show_knock_section(ui, d);
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        show_lamb_trim_section(ui, d);
    });
}

// ── IGN_KNOCK ─────────────────────────────────────────────────────────────────

fn show_knock_section(ui: &mut Ui, d: &SessionData) {
    ui.label(RichText::new("IGN_KNOCK — Ignition & Knock Detection").color(ACCENT).strong().size(13.0));
    ui.add_space(2.0);

    let knock_v_cols = find_knock_v_cols(d);
    if !d.has_ign && !d.has_knock && knock_v_cols.is_empty() {
        show_missing(ui, "IGN_KNOCK", &[
            ("ign_deg",      "\"Ign Cyl1 deg\""),
            ("knock_status", "\"ISKLO\" / \"Status Knock\" / \"Knock Stat\""),
            ("Knock V Cyl1-6", "\"Knock voltage Cylinder N\" / \"IUKZN\""),
        ]);
        return;
    }

    let rows = &d.rows;
    let step = (rows.len() / 4000).max(1);
    let h = 160.0_f32;

    // Stats row
    if d.has_knock {
        let n_knock = rows.iter().filter(|r| r.knock_status > 0.5).count();
        let pct = 100.0 * n_knock as f64 / rows.len() as f64;
        let col = if pct > 2.0 { RED } else if pct > 0.5 { AMBER } else { SAGE };
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Knock events: {} rows ({:.1}%)", n_knock, pct))
                .color(col).small().strong());
            if !d.retard_idx.is_empty() {
                ui.add_space(12.0);
                ui.label(RichText::new(format!("Retard proxy (Δign>{}°): {} rows",
                    0, d.retard_idx.len())).color(AMBER).small());
            }
        });
        ui.add_space(3.0);
    }

    // Ignition angle + knock event overlay
    if d.has_ign || d.has_knock {
        ui.label(RichText::new("Ignition Angle Cyl.1  (°BTDC)").color(FG_DIM).small().strong());
        Plot::new("knock_ign_timeline")
            .height(h)
            .show(ui, |plot_ui| {
                if d.has_ign {
                    let pts: Vec<[f64; 2]> = rows.iter().step_by(step)
                        .map(|r| [r.ts(), r.ign_deg])
                        .collect();
                    plot_ui.line(Line::new(PlotPoints::new(pts)).color(AMBER).width(1.5).name("Ign °BTDC"));
                }
                // Real knock events as red dots on the ign angle line
                if d.has_knock {
                    let kpts: Vec<[f64; 2]> = rows.iter()
                        .filter(|r| r.knock_status > 0.5)
                        .map(|r| [r.ts(), r.ign_deg])
                        .collect();
                    if !kpts.is_empty() {
                        plot_ui.points(Points::new(PlotPoints::new(kpts))
                            .color(RED).radius(4.5).name("KNOCK"));
                    }
                }
                // Retard proxy events (ign_deg < per-cell p50 – delta)
                if d.has_ign && !d.retard_idx.is_empty() {
                    let rpts: Vec<[f64; 2]> = d.retard_idx.iter()
                        .map(|&i| [rows[i].ts(), rows[i].ign_deg])
                        .collect();
                    plot_ui.points(Points::new(PlotPoints::new(rpts))
                        .color(RUST).radius(2.5).name("Retard proxy"));
                }
            });
        ui.add_space(4.0);
    }

    // Per-cylinder knock voltages
    if !knock_v_cols.is_empty() {
        ui.label(RichText::new("Knock Voltages per Cylinder  (V)").color(FG_DIM).small().strong());
        Plot::new("knock_v_timeline")
            .height(h)
            .show(ui, |plot_ui| {
                for (idx, col) in knock_v_cols.iter().enumerate() {
                    let pts: Vec<[f64; 2]> = rows.iter().step_by(step)
                        .map(|r| [r.ts(), r.extra.get(col.as_str()).copied().unwrap_or(0.0)])
                        .collect();
                    plot_ui.line(Line::new(PlotPoints::new(pts))
                        .color(CHART[idx % CHART.len()])
                        .width(1.0)
                        .name(format!("Cyl {}", idx + 1)));
                }
            });
    } else if d.has_ign || d.has_knock {
        ui.label(RichText::new(
            "Knock voltages per cylinder not found — add aliases for IUKZ1…IUKZ6 in msv70_config.json"
        ).color(FG_DIM).small());
    }
}

// ── LAMB_TRIM ─────────────────────────────────────────────────────────────────

fn show_lamb_trim_section(ui: &mut Ui, d: &SessionData) {
    ui.label(RichText::new("LAMB_TRIM — Post-Cat Lambda Controller").color(ACCENT).strong().size(13.0));
    ui.add_space(2.0);

    let has_hk = d.rows.iter().any(|r| r.hinterkat_b1 != 0.0 || r.hinterkat_b2 != 0.0);

    if !d.has_lmb_trim && !has_hk {
        show_missing(ui, "LAMB_TRIM", &[
            ("lmb_trim_b1",  "\"ILRR1\" / \"Lamb Shift B1\" / \"Lambda shift 1\""),
            ("lmb_trim_b2",  "\"ILRR2\" / \"Lamb Shift B2\" / \"Lambda shift 2\""),
            ("hinterkat_b1", "\"IUSN1\" / \"Post Cat B1\" / \"HK V B1\""),
            ("hinterkat_b2", "\"IUSN2\" / \"Post Cat B2\" / \"HK V B2\""),
        ]);
        return;
    }

    let rows = &d.rows;
    let step = (rows.len() / 4000).max(1);
    let h = 160.0_f32;

    // Saturation stats
    if d.has_lmb_trim {
        let (mut b1_hi, mut b1_lo, mut b2_hi, mut b2_lo) = (0usize, 0usize, 0usize, 0usize);
        let (mut b1_max, mut b1_min) = (f64::NEG_INFINITY, f64::INFINITY);
        let (mut b2_max, mut b2_min) = (f64::NEG_INFINITY, f64::INFINITY);
        for r in rows {
            if r.lmb_trim_b1 != 0.0 {
                if r.lmb_trim_b1 >  SAT_LIMIT { b1_hi += 1; }
                if r.lmb_trim_b1 < -SAT_LIMIT { b1_lo += 1; }
                b1_max = b1_max.max(r.lmb_trim_b1);
                b1_min = b1_min.min(r.lmb_trim_b1);
            }
            if r.lmb_trim_b2 != 0.0 {
                if r.lmb_trim_b2 >  SAT_LIMIT { b2_hi += 1; }
                if r.lmb_trim_b2 < -SAT_LIMIT { b2_lo += 1; }
                b2_max = b2_max.max(r.lmb_trim_b2);
                b2_min = b2_min.min(r.lmb_trim_b2);
            }
        }
        let n = rows.len().max(1) as f64;
        let b1_sat = 100.0 * (b1_hi + b1_lo) as f64 / n;
        let b2_sat = 100.0 * (b2_hi + b2_lo) as f64 / n;
        let sat_col = |pct: f64| if pct > 15.0 { RED } else if pct > 5.0 { AMBER } else { SAGE };

        egui::Grid::new("lmb_trim_stats").num_columns(6).spacing([8.0, 2.0]).show(ui, |ui| {
            ui.label(RichText::new("B1").color(ACCENT).small().strong());
            ui.label(RichText::new(format!("sat {:.1}%", b1_sat)).color(sat_col(b1_sat)).small());
            ui.label(RichText::new(format!("max {:+.3}", b1_max)).color(if b1_max > 0.049 { RED } else { FG_DIM }).small());
            ui.label(RichText::new(format!("min {:+.3}", b1_min)).color(if b1_min < -0.049 { RED } else { FG_DIM }).small());
            ui.label(RichText::new(format!("hi {} rows", b1_hi)).color(FG_DIM).small());
            ui.label(RichText::new(format!("lo {} rows", b1_lo)).color(FG_DIM).small());
            ui.end_row();
            ui.label(RichText::new("B2").color(ORANGE).small().strong());
            ui.label(RichText::new(format!("sat {:.1}%", b2_sat)).color(sat_col(b2_sat)).small());
            ui.label(RichText::new(format!("max {:+.3}", b2_max)).color(if b2_max > 0.049 { RED } else { FG_DIM }).small());
            ui.label(RichText::new(format!("min {:+.3}", b2_min)).color(if b2_min < -0.049 { RED } else { FG_DIM }).small());
            ui.label(RichText::new(format!("hi {} rows", b2_hi)).color(FG_DIM).small());
            ui.label(RichText::new(format!("lo {} rows", b2_lo)).color(FG_DIM).small());
            ui.end_row();
        });

        if b1_sat > 5.0 || b2_sat > 5.0 {
            ui.label(RichText::new(
                "⚠  Controller saturating — apply c_lamb_delta_i_*_lam_adj fix (±0.050 → ±0.080) in TunerPro"
            ).color(AMBER).small());
        } else {
            ui.label(RichText::new("✓  No significant saturation detected").color(SAGE).small());
        }
        ui.add_space(4.0);

        // Lambdaverschiebung timeline
        ui.label(RichText::new("Lambdaverschiebung (post-cat lambda integral shift)").color(FG_DIM).small().strong());
        Plot::new("lmb_trim_timeline")
            .height(h)
            .include_y(-0.06)
            .include_y(0.06)
            .show(ui, |plot_ui| {
                plot_ui.hline(HLine::new(0.05)
                    .color(RED)
                    .style(egui_plot::LineStyle::Dashed { length: 8.0 }));
                plot_ui.hline(HLine::new(-0.05)
                    .color(RED)
                    .style(egui_plot::LineStyle::Dashed { length: 8.0 }));
                plot_ui.hline(HLine::new(0.0)
                    .color(FG_DIM)
                    .style(egui_plot::LineStyle::Dotted { spacing: 4.0 }));

                let b1: Vec<[f64; 2]> = rows.iter().step_by(step)
                    .filter(|r| r.lmb_trim_b1 != 0.0)
                    .map(|r| [r.ts(), r.lmb_trim_b1])
                    .collect();
                let b2: Vec<[f64; 2]> = rows.iter().step_by(step)
                    .filter(|r| r.lmb_trim_b2 != 0.0)
                    .map(|r| [r.ts(), r.lmb_trim_b2])
                    .collect();

                if !b1.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(b1)).color(ACCENT).width(1.5).name("Shift B1"));
                }
                if !b2.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(b2)).color(ORANGE).width(1.5).name("Shift B2"));
                }
            });
        ui.add_space(4.0);
    }

    // HinterKat voltage
    if has_hk {
        ui.label(RichText::new("HinterKat O2 Voltage  (V, post-cat narrowband sensor)").color(FG_DIM).small().strong());
        Plot::new("hinterkat_timeline")
            .height(h)
            .include_y(0.0)
            .include_y(1.0)
            .show(ui, |plot_ui| {
                // NB target rich: 0.64-0.68 V (cat-warm rich bias)
                plot_ui.hline(HLine::new(0.64)
                    .color(SAGE)
                    .style(egui_plot::LineStyle::Dashed { length: 8.0 }));
                plot_ui.hline(HLine::new(0.30)
                    .color(AMBER)
                    .style(egui_plot::LineStyle::Dotted { spacing: 4.0 }));

                let b1: Vec<[f64; 2]> = rows.iter().step_by(step)
                    .filter(|r| r.hinterkat_b1 != 0.0)
                    .map(|r| [r.ts(), r.hinterkat_b1])
                    .collect();
                let b2: Vec<[f64; 2]> = rows.iter().step_by(step)
                    .filter(|r| r.hinterkat_b2 != 0.0)
                    .map(|r| [r.ts(), r.hinterkat_b2])
                    .collect();

                if !b1.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(b1)).color(CORN).width(1.5).name("HK B1 V"));
                }
                if !b2.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(b2)).color(PURPLE).width(1.5).name("HK B2 V"));
                }
            });
        ui.add_space(4.0);
    }

    // STFT context if closed-loop data present
    if !d.cl_b1_idx.is_empty() || !d.cl_b2_idx.is_empty() {
        ui.label(RichText::new("STFT Context  (%)").color(FG_DIM).small().strong());
        Plot::new("lamb_trim_stft")
            .height(h * 0.65)
            .show(ui, |plot_ui| {
                plot_ui.hline(HLine::new(0.0)
                    .color(FG_DIM)
                    .style(egui_plot::LineStyle::Dotted { spacing: 4.0 }));
                let s1: Vec<[f64; 2]> = rows.iter().step_by(step)
                    .filter(|r| r.cl_b1)
                    .map(|r| [r.ts(), r.stft_b1])
                    .collect();
                let s2: Vec<[f64; 2]> = rows.iter().step_by(step)
                    .filter(|r| r.cl_b2)
                    .map(|r| [r.ts(), r.stft_b2])
                    .collect();
                if !s1.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(s1)).color(ACCENT).width(1.0).name("STFT B1%"));
                }
                if !s2.is_empty() {
                    plot_ui.line(Line::new(PlotPoints::new(s2)).color(ORANGE).width(1.0).name("STFT B2%"));
                }
            });
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn find_knock_v_cols(d: &SessionData) -> Vec<String> {
    let mut found = Vec::new();
    for cyl in 1u8..=6 {
        let s = cyl.to_string();
        let col = d.extra_cols.iter().find(|c| {
            let lc = c.to_lowercase();
            lc == format!("knock voltage cylinder {s}")
                || lc == format!("iukz{s}")
                || lc == format!("knock v cyl {s}")
                || lc == format!("knk v c{s}")
                || ((lc.contains("knock") || lc.contains("klopf"))
                    && lc.contains(&s)
                    && (lc.contains("volt") || lc.contains("span") || lc.contains(" v ")))
        });
        if let Some(c) = col {
            found.push(c.clone());
        }
    }
    found
}

fn show_missing(ui: &mut Ui, job: &str, channels: &[(&str, &str)]) {
    ui.add_space(4.0);
    ui.label(RichText::new(format!("No {} channels detected in this CSV.", job)).color(FG_DIM));
    ui.label(RichText::new(format!(
        "Load a CSV recorded with the {} custom job (TestO Datalogger).", job
    )).color(FG_DIM).small());
    ui.add_space(4.0);
    ui.label(RichText::new("Expected column names (add to msv70_config.json aliases if needed):").color(FG_DIM).small());
    for (name, aliases) in channels {
        ui.label(RichText::new(format!("  •  {name:20}  {aliases}")).color(FG_DIM).small().monospace());
    }
}
