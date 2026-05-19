use egui::{Color32, FontId, RichText, ScrollArea, Ui};
use crate::analysis::percentile;
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;
use egui::Layout;

pub fn show(ui: &mut Ui, d: &SessionData, cfg: &AppConfig) {
    ScrollArea::both().id_salt("stft_map_scroll").auto_shrink([false, false]).show(ui, |ui| {
        let n_rb = d.rpm_labels.len();
        let n_lb = d.load_labels.len();
        if n_rb == 0 || n_lb == 0 { return; }

        ui.horizontal(|ui| {
            ui.heading(RichText::new("STFT 2D Map — RPM × Load").color(ACCENT));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(egui::Button::new(RichText::new("Export CSV").size(11.0).color(OLIVE)).fill(BG3)).clicked() {
                    export_stft_csv(d, cfg);
                }
            });
        });
        ui.label(RichText::new("Negative = rich (–) → reduce ip_mff_cor  |  Positive = lean (+) → add fuel").color(FG_DIM));
        ui.add_space(8.0);

        let label_w: f32 = 60.0;
        let cell_h:  f32 = 36.0;
        let gap:     f32 = 2.0;
        let avail_w = ui.available_width().max(300.0);
        let cell_w  = ((avail_w - label_w - gap * n_lb as f32) / n_lb as f32).max(36.0).min(80.0);
        let font    = FontId::proportional(10.5);
        let sfont   = FontId::proportional(8.5);

        // ── Bank 1 and Bank 2 individual maps ────────────────────────────────
        for (cell_dict, label) in [
            (&d.cell_b1, "Bank 1"),
            (&d.cell_b2, "Bank 2"),
        ] {
            ui.label(RichText::new(format!("STFT {label} — closed-loop rows only")).color(ACCENT).strong());
            ui.add_space(4.0);
            draw_grid(ui, d, cfg, cell_w, cell_h, label_w, gap, &font, &sfont, n_rb, n_lb, |i, j| {
                cell_dict.get(&(i, j))
                    .filter(|v| v.len() >= cfg.cell_min_samples)
                    .map(|v| (percentile_of(v, 50.0), v.len(), None))
            });
            ui.add_space(12.0);
        }

        // ── Average B1+B2 map ────────────────────────────────────────────────
        ui.label(RichText::new("STFT Moyenne B1+B2").color(ACCENT).strong());
        ui.label(RichText::new("(B1 med + B2 med) / 2  —  si une seule banque disponible, sa valeur seule")
            .color(FG_DIM).size(10.0).italics());
        ui.add_space(4.0);
        draw_grid(ui, d, cfg, cell_w, cell_h, label_w, gap, &font, &sfont, n_rb, n_lb, |i, j| {
            let v1 = d.cell_b1.get(&(i, j)).filter(|v| v.len() >= cfg.cell_min_samples);
            let v2 = d.cell_b2.get(&(i, j)).filter(|v| v.len() >= cfg.cell_min_samples);
            match (v1, v2) {
                (Some(a), Some(b)) => {
                    let med = (percentile_of(a, 50.0) + percentile_of(b, 50.0)) / 2.0;
                    Some((med, a.len() + b.len(), Some("B1+B2")))
                }
                (Some(a), None) => Some((percentile_of(a, 50.0), a.len(), Some("B1"))),
                (None, Some(b)) => Some((percentile_of(b, 50.0), b.len(), Some("B2"))),
                (None, None)    => None,
            }
        });
        ui.add_space(12.0);

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

// ── Grid renderer ─────────────────────────────────────────────────────────────
// `cell_fn(rpm_idx, load_idx)` → Some((median, n_samples, Option<badge_label>)) or None

fn draw_grid(
    ui: &mut Ui,
    d: &SessionData,
    cfg: &AppConfig,
    cell_w: f32, cell_h: f32, label_w: f32, gap: f32,
    font: &FontId, sfont: &FontId,
    n_rb: usize, n_lb: usize,
    cell_fn: impl Fn(usize, usize) -> Option<(f64, usize, Option<&'static str>)>,
) {
    // Header row
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(label_w, cell_h * 0.7), egui::Sense::hover());
        ui.painter().text(
            egui::pos2(rect.right() - 2.0, rect.bottom() - 2.0),
            egui::Align2::RIGHT_BOTTOM, "mg/stk →", sfont.clone(), FG_DIM,
        );
        for lbl in &d.load_labels {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h * 0.7), egui::Sense::hover());
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, lbl, sfont.clone(), FG_DIM);
            ui.add_space(gap);
        }
    });
    ui.add_space(gap);

    // Data rows
    for i in 0..n_rb {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(label_w, cell_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(rect.right() - 4.0, rect.center().y),
                egui::Align2::RIGHT_CENTER, &d.rpm_labels[i], sfont.clone(), FG_DIM,
            );

            for j in 0..n_lb {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h), egui::Sense::hover());

                if let Some((med, n, badge)) = cell_fn(i, j) {
                    let (bg, fg) = cell_colors(med, cfg);
                    ui.painter().rect_filled(rect.shrink(1.0), 3.0, bg);
                    let sign = if med >= 0.0 { "+" } else { "" };

                    let value_y = rect.center().y - 3.0;
                    ui.painter().text(
                        egui::pos2(rect.center().x, value_y),
                        egui::Align2::CENTER_CENTER,
                        format!("{sign}{med:.1}%"),
                        font.clone(), fg,
                    );

                    // Sample count
                    ui.painter().text(
                        egui::pos2(rect.center().x, rect.bottom() - 3.0),
                        egui::Align2::CENTER_BOTTOM,
                        format!("n={n}"),
                        sfont.clone(),
                        fg.gamma_multiply(0.55),
                    );

                    // (badge text intentionally omitted)
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
    let _ = cfg; // used via cell_fn closure
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

/// Build the STFT CSV content for a session (used by single export and batch).
pub fn build_stft_csv(d: &SessionData, cfg: &AppConfig) -> String {
    let n_rb = d.rpm_labels.len();
    let n_lb = d.load_labels.len();
    let mut out = String::new();
    out.push_str(&format!("# Log: {}\n", d.file_name));
    for (label, cell_dict) in [("B1", &d.cell_b1), ("B2", &d.cell_b2)] {
        out.push_str(&format!("# STFT {label} — median (%)\n"));
        out.push_str("RPM \\ Load");
        for lbl in &d.load_labels { out.push(','); out.push_str(lbl); }
        out.push('\n');
        for i in 0..n_rb {
            out.push_str(&d.rpm_labels[i]);
            for j in 0..n_lb {
                out.push(',');
                if let Some(v) = cell_dict.get(&(i, j)).filter(|v| v.len() >= cfg.cell_min_samples) {
                    out.push_str(&format!("{:.2}", percentile_of(v, 50.0)));
                }
            }
            out.push('\n');
        }
        out.push('\n');
        out.push_str(&format!("# STFT {label} — n samples\n"));
        out.push_str("RPM \\ Load");
        for lbl in &d.load_labels { out.push(','); out.push_str(lbl); }
        out.push('\n');
        for i in 0..n_rb {
            out.push_str(&d.rpm_labels[i]);
            for j in 0..n_lb {
                out.push(',');
                if let Some(v) = cell_dict.get(&(i, j)) {
                    out.push_str(&v.len().to_string());
                }
            }
            out.push('\n');
        }
        out.push('\n');
    }
    out
}

pub fn log_stem_pub(file_name: &str) -> &str {
    file_name.strip_suffix(".csv").or_else(|| file_name.strip_suffix(".CSV")).unwrap_or(file_name)
}

fn log_stem(file_name: &str) -> &str { log_stem_pub(file_name) }

fn export_stft_csv(d: &SessionData, cfg: &AppConfig) {
    let stem = log_stem(&d.file_name);
    let default_name = format!("{stem}_stft.csv");
    let path = if let Some(p) = rfd::FileDialog::new()
        .set_file_name(&default_name)
        .add_filter("CSV", &["csv"])
        .save_file()
    { p } else { return; };

    let _ = std::fs::write(&path, build_stft_csv(d, cfg));
}
