use egui::{CollapsingHeader, Grid, RichText, ScrollArea, Ui};
use crate::config::AppConfig;
use super::palette::*;

pub fn show(ui: &mut Ui, cfg: &mut AppConfig, dirty: &mut bool) {
    ScrollArea::vertical().id_salt("settings_scroll").show(ui, |ui| {
        ui.heading(RichText::new("Settings — all parameters").color(ACCENT));
        ui.label(RichText::new("Changes take effect on next Analyze. Save persists to msv70_config.json next to the exe.").color(FG_DIM).small());
        ui.add_space(8.0);

        // ── Fuel Trims ────────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Fuel Trim Thresholds (%)").color(FG).strong())
            .default_open(true)
            .show(ui, |ui| {
                Grid::new("stft_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "STFT notable (%)", &mut |c: &mut AppConfig| &mut c.stft_note, 1.0..=30.0);
                    slider(ui, cfg, dirty, "STFT warning (%)", &mut |c: &mut AppConfig| &mut c.stft_warn, 1.0..=40.0);
                    slider(ui, cfg, dirty, "STFT alarm (%)", &mut |c: &mut AppConfig| &mut c.stft_alarm, 5.0..=50.0);
                    slider(ui, cfg, dirty, "STFT imbalance (%)", &mut |c: &mut AppConfig| &mut c.stft_imbalance, 1.0..=20.0);
                    slider(ui, cfg, dirty, "LTFT warning (%)", &mut |c: &mut AppConfig| &mut c.ltft_warn, 1.0..=30.0);
                    slider(ui, cfg, dirty, "LTFT alarm (%)", &mut |c: &mut AppConfig| &mut c.ltft_alarm, 5.0..=40.0);
                    slider(ui, cfg, dirty, "LTFT imbalance (%)", &mut |c: &mut AppConfig| &mut c.ltft_imbalance, 1.0..=25.0);
                });
            });

        ui.add_space(4.0);

        // ── Lambda ────────────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Lambda Thresholds (λ)").color(FG).strong())
            .default_open(true)
            .show(ui, |ui| {
                Grid::new("lmb_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "Lean under load (λ >)", &mut |c: &mut AppConfig| &mut c.lmb_lean_load, 1.00..=1.15);
                    slider(ui, cfg, dirty, "Rich alarm (λ <)", &mut |c: &mut AppConfig| &mut c.lmb_rich, 0.80..=1.00);
                    slider(ui, cfg, dirty, "WOT target λ", &mut |c: &mut AppConfig| &mut c.lmb_wot_target, 0.80..=1.00);
                    slider(ui, cfg, dirty, "WOT lean warn (λ >)", &mut |c: &mut AppConfig| &mut c.lmb_wot_warn, 0.85..=1.05);
                });
            });

        ui.add_space(4.0);

        // ── Temperature ───────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Temperature (°C)").color(FG).strong())
            .default_open(true)
            .show(ui, |ui| {
                Grid::new("temp_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "Coolant overtemp alarm", &mut |c: &mut AppConfig| &mut c.coolant_warn, 90.0..=130.0);
                    slider(ui, cfg, dirty, "Coolant CL minimum", &mut |c: &mut AppConfig| &mut c.coolant_cl_min, 30.0..=80.0);
                });
            });

        ui.add_space(4.0);

        // ── VANOS ─────────────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("VANOS (°CRK)").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("vanos_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "Steady-state warning", &mut |c: &mut AppConfig| &mut c.vanos_steady_warn, 1.0..=20.0);
                    slider(ui, cfg, dirty, "Steady-state alarm", &mut |c: &mut AppConfig| &mut c.vanos_steady_alarm, 2.0..=30.0);
                });
            });

        ui.add_space(4.0);

        // ── Injector ─────────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Injector Duty Cycle (%)").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("inj_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "DC warning (%)", &mut |c: &mut AppConfig| &mut c.inj_duty_warn, 50.0..=95.0);
                    slider(ui, cfg, dirty, "DC alarm (%)", &mut |c: &mut AppConfig| &mut c.inj_duty_alarm, 60.0..=100.0);
                });
            });

        ui.add_space(4.0);

        // ── Knock proxy ───────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Knock / Ignition Retard Proxy").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("knock_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "Min RPM for knock zone", &mut |c: &mut AppConfig| &mut c.knock_rpm_min, 500.0..=4000.0);
                    slider(ui, cfg, dirty, "Min load for knock zone (%)", &mut |c: &mut AppConfig| &mut c.knock_load_min, 10.0..=70.0);
                    slider(ui, cfg, dirty, "Retard threshold (°)", &mut |c: &mut AppConfig| &mut c.knock_delta, 1.0..=15.0);
                    slider(ui, cfg, dirty, "Hot engine baseline coolant (°C)", &mut |c: &mut AppConfig| &mut c.knock_hot_coolant, 60.0..=105.0);
                });
            });

        ui.add_space(4.0);

        // ── Phase Detection ───────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Phase Detection").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                ui.label(RichText::new(
                    "EngineOff: RPM < 50  |  Cranking: RPM < 400\n\
                     FuelCut: pedal < fuel_cut_pedal AND load < fuel_cut_load AND rpm > fuel_cut_rpm AND speed > fuel_cut_speed\n\
                     → load guard prevents misclassifying cruise control (pedal=0 but engine loaded)\n\
                     WOT: pedal > wot_pedal OR load > wot_load\n\
                     Accel: pedal > accel_pedal  |  HotIdle/Warmup: speed < idle_speed AND rpm < idle_rpm\n\
                     Cruise: everything else above hot_coolant threshold"
                ).color(FG_DIM).small());
                ui.add_space(6.0);
                Grid::new("phase_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider(ui, cfg, dirty, "Min phase duration (s) — merge shorter spans", &mut |c: &mut AppConfig| &mut c.phase_min_duration_s, 0.0..=5.0);
                    slider(ui, cfg, dirty, "CL entry min RPM", &mut |c: &mut AppConfig| &mut c.phase_cl_rpm_min, 200.0..=1500.0);
                    slider(ui, cfg, dirty, "CL entry min load (%)", &mut |c: &mut AppConfig| &mut c.phase_cl_load_min, 0.5..=10.0);
                    slider(ui, cfg, dirty, "WOT pedal threshold (%)", &mut |c: &mut AppConfig| &mut c.phase_wot_pedal, 50.0..=100.0);
                    slider(ui, cfg, dirty, "WOT load threshold (%)", &mut |c: &mut AppConfig| &mut c.phase_wot_load, 40.0..=100.0);
                    slider(ui, cfg, dirty, "Accel pedal threshold (%)", &mut |c: &mut AppConfig| &mut c.phase_accel_pedal, 10.0..=80.0);
                    slider(ui, cfg, dirty, "Fuel cut pedal (%)", &mut |c: &mut AppConfig| &mut c.phase_fuel_cut_pedal, 1.0..=15.0);
                    slider(ui, cfg, dirty, "Fuel cut max load% (guard vs cruise ctrl)", &mut |c: &mut AppConfig| &mut c.phase_fuel_cut_load, 1.0..=30.0);
                    slider(ui, cfg, dirty, "Fuel cut min RPM", &mut |c: &mut AppConfig| &mut c.phase_fuel_cut_rpm, 500.0..=3000.0);
                    slider(ui, cfg, dirty, "Fuel cut min speed (km/h)", &mut |c: &mut AppConfig| &mut c.phase_fuel_cut_speed, 0.0..=30.0);
                    slider(ui, cfg, dirty, "Hot idle coolant boundary (°C)", &mut |c: &mut AppConfig| &mut c.phase_hot_coolant, 50.0..=100.0);
                    slider(ui, cfg, dirty, "Idle max RPM", &mut |c: &mut AppConfig| &mut c.phase_idle_rpm, 500.0..=2000.0);
                    slider(ui, cfg, dirty, "Idle max speed (km/h)", &mut |c: &mut AppConfig| &mut c.phase_idle_speed, 0.0..=15.0);
                });
            });

        ui.add_space(4.0);

        // ── Analysis Parameters ───────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Analysis Engine").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("analysis_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider_usize(ui, cfg, dirty, "Engine cylinders (for mg/stk)", &mut |c: &mut AppConfig| &mut c.engine_cyl, 4..=12);
                    slider(ui, cfg, dirty, "Load% → mg/stk scale (fallback, no MAF)", &mut |c: &mut AppConfig| &mut c.load_pct_to_mg_stk_scale, 1.0..=15.0);
                    slider_usize(ui, cfg, dirty, "Min cell samples (STFT map)", &mut |c: &mut AppConfig| &mut c.cell_min_samples, 2..=20);
                    slider_usize(ui, cfg, dirty, "Min ign cell samples (knock)", &mut |c: &mut AppConfig| &mut c.ign_cell_min_samples, 3..=30);
                    slider_usize(ui, cfg, dirty, "LEAN debounce (consecutive)", &mut |c: &mut AppConfig| &mut c.lean_debounce, 1..=10);
                    slider_usize(ui, cfg, dirty, "STFT alarm streak (consecutive)", &mut |c: &mut AppConfig| &mut c.stft_alarm_streak, 2..=20);
                    slider(ui, cfg, dirty, "STFT bucket width (s)", &mut |c: &mut AppConfig| &mut c.stft_bucket_seconds, 10.0..=300.0);
                    slider(ui, cfg, dirty, "Steady-state max dt (s)", &mut |c: &mut AppConfig| &mut c.steady_state_max_dt, 0.1..=5.0);
                    slider(ui, cfg, dirty, "Steady-state max ΔRPM/s", &mut |c: &mut AppConfig| &mut c.steady_state_drpm, 50.0..=1000.0);
                    slider(ui, cfg, dirty, "Steady-state max ΔLoad%/s", &mut |c: &mut AppConfig| &mut c.steady_state_dload, 2.0..=100.0);
                    slider_usize(ui, cfg, dirty, "Max display points", &mut |c: &mut AppConfig| &mut c.display_max_points, 500..=10000);
                });
            });

        ui.add_space(4.0);

        // ── Bin Edges ─────────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("RPM & Load Bin Edges (STFT map)").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                ui.label(RichText::new("RPM in rpm, Load in mg/stk (derived from MAF). Defaults match ip_mff_cor_opm_1_1 XDF axes.").color(FG_DIM).small());
                ui.label(RichText::new("Space-separated values, strictly increasing. Re-analyze after change.").color(FG_DIM).small());
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("RPM bins:").color(FG).strong());
                    let mut rpm_str: String = cfg.rpm_bins.iter().map(|v| format!("{:.0}", v)).collect::<Vec<_>>().join(" ");
                    if ui.add(egui::TextEdit::singleline(&mut rpm_str).desired_width(400.0)).changed() {
                        if let Some(parsed) = parse_bins(&rpm_str) {
                            if parsed.len() >= 3 {
                                cfg.rpm_bins = parsed;
                                *dirty = true;
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Load bins:").color(FG).strong());
                    let mut load_str: String = cfg.load_bins.iter().map(|v| format!("{:.0}", v)).collect::<Vec<_>>().join(" ");
                    if ui.add(egui::TextEdit::singleline(&mut load_str).desired_width(400.0)).changed() {
                        if let Some(parsed) = parse_bins(&load_str) {
                            if parsed.len() >= 3 {
                                cfg.load_bins = parsed;
                                *dirty = true;
                            }
                        }
                    }
                });

                ui.add_space(4.0);
                ui.label(RichText::new(format!("Current RPM labels: {}", cfg.rpm_labels().join("  "))).color(FG_DIM).small());
                ui.label(RichText::new(format!("Current Load labels: {}", cfg.load_labels().join("  "))).color(FG_DIM).small());
            });

        ui.add_space(4.0);

        // ── Column Aliases ────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("Column Aliases (CSV Header Mapping)").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                ui.label(RichText::new("Internal name → CSV header alternatives (first match wins, case-insensitive).").color(FG_DIM).small());
                ui.add_space(4.0);
                Grid::new("aliases_grid").num_columns(2).spacing([8.0, 2.0]).show(ui, |ui| {
                    ui.label(RichText::new("Internal name").color(FG_DIM).strong().small());
                    ui.label(RichText::new("CSV aliases (comma-separated)").color(FG_DIM).strong().small());
                    ui.end_row();

                    let keys: Vec<String> = cfg.col_aliases.keys().cloned().collect();
                    for key in &keys {
                        ui.label(RichText::new(key).color(ACCENT).small().monospace());
                        let aliases = cfg.col_aliases.get(key).cloned().unwrap_or_default();
                        let mut alias_str = aliases.join(", ");
                        if ui.add(egui::TextEdit::singleline(&mut alias_str).desired_width(300.0)).changed() {
                            let new_aliases: Vec<String> = alias_str.split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            cfg.col_aliases.insert(key.clone(), new_aliases);
                            *dirty = true;
                        }
                        ui.end_row();
                    }
                });
            });

        ui.add_space(12.0);

        // ── UI Layout ────────────────────────────────────────────────────────
        CollapsingHeader::new(RichText::new("UI Layout").color(FG).strong())
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("ui_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                    slider_f32(ui, cfg, dirty, "Sidebar width (px)", &mut |c: &mut AppConfig| &mut c.sidebar_width, 150.0..=400.0);
                    slider_f32(ui, cfg, dirty, "Report width (px)", &mut |c: &mut AppConfig| &mut c.report_width, 250.0..=700.0);
                });
            });
    });
}

// ── Slider helpers ─────────────────────────────────────────────────────────────

fn slider(
    ui: &mut Ui,
    cfg: &mut AppConfig,
    dirty: &mut bool,
    label: &str,
    field: &mut dyn FnMut(&mut AppConfig) -> &mut f64,
    range: std::ops::RangeInclusive<f64>,
) {
    ui.label(RichText::new(label).color(FG).small());
    let v = field(cfg);
    if ui.add(egui::Slider::new(v, range).smart_aim(true)).changed() {
        *dirty = true;
    }
    ui.end_row();
}

fn slider_usize(
    ui: &mut Ui,
    cfg: &mut AppConfig,
    dirty: &mut bool,
    label: &str,
    field: &mut dyn FnMut(&mut AppConfig) -> &mut usize,
    range: std::ops::RangeInclusive<usize>,
) {
    ui.label(RichText::new(label).color(FG).small());
    let v = field(cfg);
    if ui.add(egui::Slider::new(v, range)).changed() {
        *dirty = true;
    }
    ui.end_row();
}

fn slider_f32(
    ui: &mut Ui,
    cfg: &mut AppConfig,
    dirty: &mut bool,
    label: &str,
    field: &mut dyn FnMut(&mut AppConfig) -> &mut f32,
    range: std::ops::RangeInclusive<f32>,
) {
    ui.label(RichText::new(label).color(FG).small());
    let v = field(cfg);
    if ui.add(egui::Slider::new(v, range)).changed() {
        *dirty = true;
    }
    ui.end_row();
}

fn parse_bins(s: &str) -> Option<Vec<f64>> {
    let vals: Vec<f64> = s.split_whitespace()
        .filter_map(|t| t.parse::<f64>().ok())
        .collect();
    if vals.len() < 2 { return None; }
    for i in 1..vals.len() {
        if vals[i] <= vals[i - 1] { return None; }
    }
    Some(vals)
}
