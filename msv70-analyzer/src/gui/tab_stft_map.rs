use egui::{Color32, FontId, RichText, ScrollArea, Ui};
use crate::analysis::percentile;
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

pub fn show(ui: &mut Ui, d: &SessionData, cfg: &AppConfig) {
    ScrollArea::vertical().id_salt("stft_map_scroll").show(ui, |ui| {
        let n_rb = d.rpm_labels.len();
        let n_lb = d.load_labels.len();
        if n_rb == 0 || n_lb == 0 { return; }

        ui.heading(RichText::new("STFT 2D Map — RPM × Load").color(ACCENT));
        ui.label(RichText::new("Negative = rich (–) → reduce ip_mff_cor  |  Positive = lean (+) → add fuel").color(FG_DIM));
        ui.add_space(8.0);

        let label_w: f32 = 72.0;
        let cell_h:  f32 = 46.0;
        let gap:     f32 = 2.0;
        let avail_w = ui.available_width();
        let cell_w  = ((avail_w - label_w - gap * n_lb as f32) / n_lb as f32).max(38.0).min(100.0);
        let font    = FontId::proportional(10.5);
        let sfont   = FontId::proportional(8.5);

        for (bank, cell_dict, label) in [
            (1usize, &d.cell_b1, "Bank 1"),
            (2usize, &d.cell_b2, "Bank 2"),
        ] {
            let _ = bank;
            ui.label(RichText::new(format!("STFT {label} — closed-loop rows only")).color(ACCENT).strong());
            ui.add_space(4.0);

            // ── Header row ────────────────────────────────────────────────────
            ui.horizontal(|ui| {
                // corner cell
                let (rect, _) = ui.allocate_exact_size(egui::vec2(label_w, cell_h * 0.7), egui::Sense::hover());
                ui.painter().text(
                    egui::pos2(rect.right() - 2.0, rect.bottom() - 2.0),
                    egui::Align2::RIGHT_BOTTOM,
                    "Load →",
                    sfont.clone(),
                    FG_DIM,
                );
                for lbl in &d.load_labels {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h * 0.7), egui::Sense::hover());
                    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER,
                        lbl, sfont.clone(), FG_DIM);
                    ui.add_space(gap);
                }
            });
            ui.add_space(gap);

            // ── Data rows ─────────────────────────────────────────────────────
            for i in 0..n_rb {
                ui.horizontal(|ui| {
                    // RPM label
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(label_w, cell_h), egui::Sense::hover());
                    ui.painter().text(
                        egui::pos2(rect.right() - 4.0, rect.center().y),
                        egui::Align2::RIGHT_CENTER,
                        &d.rpm_labels[i],
                        sfont.clone(),
                        FG_DIM,
                    );

                    // Cells
                    for j in 0..n_lb {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h), egui::Sense::hover());
                        let vals = cell_dict.get(&(i, j));
                        if let Some(v) = vals.filter(|v| v.len() >= cfg.cell_min_samples) {
                            let med = percentile_of(v, 50.0);
                            let (bg, fg) = cell_colors(med, cfg);
                            ui.painter().rect_filled(rect.shrink(1.0), 3.0, bg);
                            let sign = if med >= 0.0 { "+" } else { "" };
                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.center().y - 3.0),
                                egui::Align2::CENTER_CENTER,
                                format!("{sign}{med:.1}%"),
                                font.clone(),
                                fg,
                            );
                            ui.painter().text(
                                egui::pos2(rect.center().x, rect.bottom() - 3.0),
                                egui::Align2::CENTER_BOTTOM,
                                format!("n={}", v.len()),
                                sfont.clone(),
                                fg.gamma_multiply(0.55),
                            );
                        } else {
                            ui.painter().rect_filled(rect.shrink(1.0), 3.0, BG3);
                            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER,
                                "—", font.clone(), FG_DIM);
                        }
                        ui.add_space(gap);
                    }
                });
                ui.add_space(gap);
            }
            ui.add_space(12.0);
        }

        // ── Corrections table ─────────────────────────────────────────────────
        if !d.corrections.is_empty() {
            ui.separator();
            ui.label(RichText::new(
                format!("Corrections — cellules |STFT| > {:.0}%", cfg.stft_note)
            ).color(ACCENT).strong());
            ui.add_space(4.0);

            egui::Grid::new("corrections").striped(true).spacing([8.0, 2.0]).show(ui, |ui| {
                for h in ["RPM", "Load", "B1 med", "B2 med", "avg Δ", "Facteur", "Sévérité"] {
                    ui.label(RichText::new(h).color(FG_DIM).strong().small());
                }
                ui.end_row();

                for c in &d.corrections {
                    let (sev_text, sev_col) = if c.avg_delta.abs() > cfg.stft_alarm {
                        ("⛔ CRITICAL", RED)
                    } else if c.avg_delta.abs() > cfg.stft_warn {
                        ("⚠ HIGH", ORANGE)
                    } else {
                        ("· notable", YELLOW)
                    };
                    let s1 = c.med_b1.map(|v| format!("{v:+.1}%")).unwrap_or_else(|| "—".into());
                    let s2 = c.med_b2.map(|v| format!("{v:+.1}%")).unwrap_or_else(|| "—".into());
                    let rpm_lbl  = d.rpm_labels.get(c.rpm_idx).map(|s| s.as_str()).unwrap_or("?");
                    let load_lbl = d.load_labels.get(c.load_idx).map(|s| s.as_str()).unwrap_or("?");
                    ui.label(RichText::new(rpm_lbl).color(FG).small());
                    ui.label(RichText::new(load_lbl).color(FG).small());
                    ui.label(RichText::new(s1).color(stft_color(c.med_b1.unwrap_or(0.0), cfg.stft_note, cfg.stft_warn, cfg.stft_alarm)).small());
                    ui.label(RichText::new(s2).color(stft_color(c.med_b2.unwrap_or(0.0), cfg.stft_note, cfg.stft_warn, cfg.stft_alarm)).small());
                    ui.label(RichText::new(format!("{:+.1}%", c.avg_delta)).color(stft_color(c.avg_delta, cfg.stft_note, cfg.stft_warn, cfg.stft_alarm)).small());
                    ui.label(RichText::new(format!("×{:.3}", c.factor)).color(FG).small());
                    ui.label(RichText::new(sev_text).color(sev_col).small());
                    ui.end_row();
                }
            });
        }
    });
}

fn cell_colors(v: f64, cfg: &AppConfig) -> (Color32, Color32) {
    let intensity = (v.abs() / cfg.stft_alarm / 1.15).min(1.0);
    let bg = if v < 0.0 {
        Color32::from_rgb(
            (37.0 + 207.0 * intensity) as u8,
            (37.0 * (1.0 - intensity * 0.7)) as u8,
            (38.0 * (1.0 - intensity * 0.7)) as u8,
        )
    } else {
        Color32::from_rgb(
            (37.0 * (1.0 - intensity * 0.5)) as u8,
            (37.0 * (1.0 - intensity * 0.3)) as u8,
            (38.0 + 176.0 * intensity) as u8,
        )
    };
    let fg = if intensity > 0.5 { Color32::WHITE } else { FG };
    (bg, fg)
}

fn percentile_of(values: &[f64], p: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    let mut v = values.to_vec();
    percentile(&mut v, p)
}
