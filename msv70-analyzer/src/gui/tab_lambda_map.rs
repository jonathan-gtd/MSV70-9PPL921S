use egui::{Color32, FontId, RichText, ScrollArea, Ui};
use crate::analysis::percentile;
use crate::config::AppConfig;
use crate::data::types::*;
use super::palette::*;

// XDF ip_lamb_bas_1 target values (8 RPM rows × 8 load cols)
// RPM axis: [704, 1248, 1760, 2016, 2496, 3008, 4000, 5024]
// Load axis: [75, 125, 175, 225, 275, 375, 423, 500] mg/stk
const LMB_TARGET: [[f64; 8]; 8] = [
    [0.9924, 0.9924, 0.9924, 0.9924, 0.9924, 0.9974, 0.9974, 0.9974], // 704 rpm
    [0.9963, 0.9954, 0.9954, 0.9974, 0.9963, 0.9969, 0.9969, 0.9974], // 1248
    [0.9944, 0.9942, 0.9954, 0.9974, 0.9964, 0.9944, 0.9924, 0.9974], // 1760
    [0.9952, 0.9952, 0.9957, 0.9979, 0.9984, 0.9939, 0.9944, 0.9974], // 2016
    [0.9949, 0.9954, 0.9949, 0.9974, 0.9984, 0.9979, 0.9964, 0.9974], // 2496
    [0.9944, 0.9944, 0.9954, 0.9959, 0.9964, 0.9964, 0.9944, 0.9974], // 3008
    [0.9964, 0.9964, 0.9964, 0.9974, 0.9974, 0.9974, 0.9974, 0.9974], // 4000
    [0.9964, 0.9964, 0.9964, 0.9964, 0.9964, 0.9964, 0.9964, 0.9964], // 5024
];

pub fn show(ui: &mut Ui, d: &SessionData, cfg: &AppConfig) {
    ScrollArea::both().id_salt("lmb_map_scroll").auto_shrink([false, false]).show(ui, |ui| {
        let n_rb = d.lmb_rpm_labels.len();
        let n_lb = d.lmb_load_labels.len();
        if n_rb == 0 || n_lb == 0 {
            ui.label(RichText::new("No data — check lmb_rpm_bins and lmb_load_bins in Settings.").color(FG_DIM));
            return;
        }
        if !d.has_lmb {
            ui.label(RichText::new("Lambda sensor data not available in this log.").color(ORANGE));
            return;
        }

        ui.heading(RichText::new("Lambda Actual vs Target — RPM × Load (mg/stk)").color(ACCENT));
        ui.label(RichText::new(
            "Target λ from XDF ip_lamb_bas_1  |  Actual = median wideband  |  Δ = actual − target"
        ).color(FG_DIM));
        ui.add_space(8.0);

        let label_w: f32 = 60.0;
        let cell_h:  f32 = 44.0;
        let gap:     f32 = 2.0;
        let avail_w = ui.available_width().max(300.0);
        let cell_w  = ((avail_w - label_w - gap * n_lb as f32) / n_lb as f32).max(38.0).min(95.0);
        let font    = FontId::proportional(10.5);
        let sfont   = FontId::proportional(8.5);

        for (cell_dict, lbl) in [
            (&d.cell_lmb_b1, "Bank 1 — λ actual"),
            (&d.cell_lmb_b2, "Bank 2 — λ actual"),
        ] {
            ui.label(RichText::new(lbl).color(ACCENT).strong());
            ui.add_space(4.0);

            draw_lmb_grid(ui, d, cfg, cell_w, cell_h, label_w, gap, &font, &sfont, n_rb, n_lb, |i, j| {
                let tgt = if i < 8 && j < 8 { LMB_TARGET[i][j] } else { 1.0 };
                cell_dict.get(&(i, j))
                    .filter(|v| v.len() >= cfg.cell_min_samples)
                    .map(|v| {
                        let med = percentile_of(v, 50.0);
                        (med, tgt, v.len())
                    })
            });
            ui.add_space(12.0);
        }

        // Combined deviation note
        ui.separator();
        ui.add_space(4.0);
        ui.label(RichText::new("Color scale: blue = rich (λ < target) · red = lean (λ > target)").color(FG_DIM).small());
        ui.label(RichText::new(format!(
            "Target source: ip_lamb_bas_1 (XDF)  |  Min cell samples: {}",
            cfg.cell_min_samples
        )).color(FG_DIM).small());
    });
}

fn draw_lmb_grid(
    ui: &mut Ui,
    d: &SessionData,
    _cfg: &AppConfig,
    cell_w: f32, cell_h: f32, label_w: f32, gap: f32,
    font: &FontId, sfont: &FontId,
    n_rb: usize, n_lb: usize,
    cell_fn: impl Fn(usize, usize) -> Option<(f64, f64, usize)>, // (actual, target, n)
) {
    // Header
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(label_w, cell_h * 0.6), egui::Sense::hover());
        ui.painter().text(
            egui::pos2(rect.right() - 2.0, rect.bottom() - 2.0),
            egui::Align2::RIGHT_BOTTOM, "mg/stk →", sfont.clone(), FG_DIM,
        );
        for lbl in &d.lmb_load_labels {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h * 0.6), egui::Sense::hover());
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, lbl, sfont.clone(), FG_DIM);
            ui.add_space(gap);
        }
    });
    ui.add_space(gap);

    // Rows
    for i in 0..n_rb {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(label_w, cell_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(rect.right() - 4.0, rect.center().y),
                egui::Align2::RIGHT_CENTER, &d.lmb_rpm_labels[i], sfont.clone(), FG_DIM,
            );

            for j in 0..n_lb {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h), egui::Sense::hover());

                if let Some((actual, tgt, n)) = cell_fn(i, j) {
                    let delta = actual - tgt;
                    let (bg, fg) = lmb_cell_colors(delta);
                    ui.painter().rect_filled(rect.shrink(1.0), 3.0, bg);

                    // tgt (top)
                    ui.painter().text(
                        egui::pos2(rect.center().x, rect.top() + 3.0),
                        egui::Align2::CENTER_TOP,
                        format!("t{tgt:.3}"),
                        sfont.clone(), fg.gamma_multiply(0.6),
                    );

                    // Actual λ (upper-center)
                    ui.painter().text(
                        egui::pos2(rect.center().x, rect.center().y - 5.0),
                        egui::Align2::CENTER_CENTER,
                        format!("{actual:.3}"),
                        font.clone(), fg,
                    );

                    // Delta (lower-center)
                    let dsign = if delta >= 0.0 { "+" } else { "" };
                    ui.painter().text(
                        egui::pos2(rect.center().x, rect.center().y + 7.0),
                        egui::Align2::CENTER_CENTER,
                        format!("Δ{dsign}{delta:.3}"),
                        sfont.clone(), fg.gamma_multiply(0.85),
                    );

                    // n (bottom)
                    ui.painter().text(
                        egui::pos2(rect.center().x, rect.bottom() - 2.0),
                        egui::Align2::CENTER_BOTTOM,
                        format!("n={n}"),
                        sfont.clone(), fg.gamma_multiply(0.5),
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
}

// delta = actual - target; negative = rich (blue), positive = lean (red)
fn lmb_cell_colors(delta: f64) -> (Color32, Color32) {
    let intensity = (delta.abs() / 0.02).min(1.0); // 0.02 = 2% = full saturation
    let bg = if delta < 0.0 {
        // Rich: blue
        Color32::from_rgb(
            (37.0 * (1.0 - intensity * 0.5)) as u8,
            (37.0 * (1.0 - intensity * 0.3)) as u8,
            (38.0 + 176.0 * intensity) as u8,
        )
    } else if delta > 0.001 {
        // Lean: red
        Color32::from_rgb(
            (37.0 + 207.0 * intensity) as u8,
            (37.0 * (1.0 - intensity * 0.7)) as u8,
            (38.0 * (1.0 - intensity * 0.7)) as u8,
        )
    } else {
        BG3
    };
    let fg = if intensity > 0.5 { Color32::WHITE } else { FG };
    (bg, fg)
}

fn percentile_of(values: &[f64], p: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    let mut v = values.to_vec();
    percentile(&mut v, p)
}
