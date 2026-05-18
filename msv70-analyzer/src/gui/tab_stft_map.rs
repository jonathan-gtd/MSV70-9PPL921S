use egui::{Color32, Grid, RichText, ScrollArea, Ui};
use crate::analysis::percentile;
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

pub fn show(ui: &mut Ui, d: &SessionData, cfg: &AppConfig) {
    ScrollArea::vertical().id_salt("stft_map_scroll").show(ui, |ui| {
        let n_rb = d.rpm_labels.len();
        let n_lb = d.load_labels.len();

        ui.heading(RichText::new("STFT 2D Map — RPM × Load").color(ACCENT));
        ui.label(RichText::new("Negative = rich (red) → reduce ip_mff_cor  |  Positive = lean (blue) → add fuel").color(FG_DIM));
        ui.add_space(8.0);

        for (bank, cell_dict, label) in [
            (1usize, &d.cell_b1, "Bank 1"),
            (2usize, &d.cell_b2, "Bank 2"),
        ] {
            ui.label(RichText::new(format!("STFT {label} — CL rows only")).color(ACCENT).strong());
            Grid::new(format!("map_{bank}"))
                .striped(false)
                .spacing([2.0, 2.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("RPM \\ Load").color(FG_DIM).small());
                    for lbl in &d.load_labels {
                        ui.label(RichText::new(lbl).color(FG_DIM).small());
                    }
                    ui.end_row();

                    for i in 0..n_rb {
                        ui.label(RichText::new(&d.rpm_labels[i]).color(FG_DIM).small());
                        for j in 0..n_lb {
                            let vals = cell_dict.get(&(i, j));
                            if let Some(v) = vals.filter(|v| v.len() >= cfg.cell_min_samples) {
                                let med = percentile_of(v, 50.0);
                                let (bg, fg) = cell_colors(med, cfg);
                                let sign = if med >= 0.0 { "+" } else { "" };
                                let text = format!("{sign}{med:.1}\n({n})", n = v.len());
                                egui::Frame::new()
                                    .fill(bg)
                                    .corner_radius(3.0)
                                    .inner_margin(egui::Margin::symmetric(4, 2))
                                    .show(ui, |ui| {
                                        ui.label(RichText::new(text).color(fg).small());
                                    });
                            } else {
                                egui::Frame::new()
                                    .fill(BG3)
                                    .corner_radius(3.0)
                                    .inner_margin(egui::Margin::symmetric(4, 2))
                                    .show(ui, |ui| {
                                        ui.label(RichText::new("---").color(FG_DIM).small());
                                    });
                            }
                        }
                        ui.end_row();
                    }
                });
            ui.add_space(12.0);
        }

        if !d.corrections.is_empty() {
            ui.separator();
            ui.label(RichText::new(format!("Tuning Corrections — cells |STFT| > {:.0}%", cfg.stft_note)).color(ACCENT).strong());
            ui.add_space(4.0);

            Grid::new("corrections").striped(true).spacing([8.0, 2.0]).show(ui, |ui| {
                for header in ["RPM range", "Load", "B1 med", "B2 med", "avg Δ", "Factor", "Severity"] {
                    ui.label(RichText::new(header).color(FG_DIM).strong().small());
                }
                ui.end_row();

                for c in &d.corrections {
                    let (sev_text, sev_color) = if c.avg_delta.abs() > cfg.stft_alarm {
                        ("⛔ CRITICAL", RED)
                    } else if c.avg_delta.abs() > cfg.stft_warn {
                        ("⚠ HIGH", ORANGE)
                    } else {
                        ("· notable", YELLOW)
                    };
                    let s1 = c.med_b1.map(|v| format!("{v:+.1}%")).unwrap_or_else(|| "---".into());
                    let s2 = c.med_b2.map(|v| format!("{v:+.1}%")).unwrap_or_else(|| "---".into());
                    let rpm_lbl  = d.rpm_labels.get(c.rpm_idx).map(|s| s.as_str()).unwrap_or("?");
                    let load_lbl = d.load_labels.get(c.load_idx).map(|s| s.as_str()).unwrap_or("?");
                    ui.label(RichText::new(rpm_lbl).color(FG).small());
                    ui.label(RichText::new(load_lbl).color(FG).small());
                    ui.label(RichText::new(s1).color(stft_color(c.med_b1.unwrap_or(0.0), cfg.stft_note, cfg.stft_warn, cfg.stft_alarm)).small());
                    ui.label(RichText::new(s2).color(stft_color(c.med_b2.unwrap_or(0.0), cfg.stft_note, cfg.stft_warn, cfg.stft_alarm)).small());
                    ui.label(RichText::new(format!("{:+.1}%", c.avg_delta)).color(stft_color(c.avg_delta, cfg.stft_note, cfg.stft_warn, cfg.stft_alarm)).small());
                    ui.label(RichText::new(format!("×{:.3}", c.factor)).color(FG).small());
                    ui.label(RichText::new(sev_text).color(sev_color).small());
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
            (0x25 as f64 + (0xf4 - 0x25) as f64 * intensity) as u8,
            (0x25 as f64 * (1.0 - intensity * 0.7)) as u8,
            (0x26 as f64 * (1.0 - intensity * 0.7)) as u8,
        )
    } else {
        Color32::from_rgb(
            (0x25 as f64 * (1.0 - intensity * 0.5)) as u8,
            (0x25 as f64 * (1.0 - intensity * 0.3)) as u8,
            (0x26 as f64 + (0xd6 - 0x26) as f64 * intensity) as u8,
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
