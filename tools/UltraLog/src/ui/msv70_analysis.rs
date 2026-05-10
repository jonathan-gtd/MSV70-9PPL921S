//! MSV70 Fuel Trim & Phase Analysis Panel.
//!
//! Displays as a floating egui window.  Opens from the Tools panel.
//!
//! Features:
//!  - Fuel trim health (STFT / LTFT add / LTFT mul, both banks)
//!  - Bank balance check (Δ > 5 % → alert)
//!  - MAF vs Calculated-Load Pearson correlation
//!  - Engine state distribution (WOT / Cruise / Transient / Idle)
//!  - Injector duty cycle estimate (Cyl 1, warm engine)

use eframe::egui;

use crate::app::UltraLogApp;

/// Near-zero threshold: Testo emits 2.22045e-14 for zeroed channels.
const NEAR_ZERO: f64 = 1e-10;

// ─────────────────────────────────────────────────────────────────────────────
// Known MSV70 channel header names
// ─────────────────────────────────────────────────────────────────────────────

const RPM_NAMES:        &[&str] = &["RPM"];
const LOAD_NAMES:       &[&str] = &["CalcLoad", "Load %"];
const TPS_NAMES:        &[&str] = &["Pedal %", "Pedal"];
const COOLANT_NAMES:    &[&str] = &["Coolant"];
const STFT1_NAMES:      &[&str] = &["STFT B1"];
const STFT2_NAMES:      &[&str] = &["STFT B2"];
const LTFT_ADD1_NAMES:  &[&str] = &["LTFT add. B1"];
const LTFT_ADD2_NAMES:  &[&str] = &["LTFT add. B2"];
const LTFT_MUL1_NAMES:  &[&str] = &["LTFT mul. B1"];
const LTFT_MUL2_NAMES:  &[&str] = &["LTFT mul. B2"];
const MAF_NAMES:        &[&str] = &["AirMass", "MAF kg/h"];
const INJ_NAMES:        &[&str] = &["InjTmCyl1", "Inj C1 ms"];

// ─────────────────────────────────────────────────────────────────────────────
// Small helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Look up a channel by a list of candidate names and return its data.
fn get_data(log: &crate::parsers::Log, candidates: &[&str]) -> Option<Vec<f64>> {
    for name in candidates {
        if let Some(idx) = log.find_channel_index(name) {
            let v: Vec<f64> = log
                .get_channel_data(idx)
                .into_iter()
                .map(|x| if x.abs() < NEAR_ZERO { 0.0 } else { x })
                .collect();
            return Some(v);
        }
    }
    None
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { return 0.0; }
    v.iter().sum::<f64>() / v.len() as f64
}

fn max_abs(v: &[f64]) -> f64 {
    v.iter().map(|x| x.abs()).fold(0.0_f64, f64::max)
}

fn pearson(a: &[f64], b: &[f64]) -> Option<f64> {
    let n = a.len().min(b.len());
    if n < 10 { return None; }
    let ma = mean(&a[..n]);
    let mb = mean(&b[..n]);
    let num: f64 = a[..n].iter().zip(b[..n].iter()).map(|(x, y)| (x - ma) * (y - mb)).sum();
    let da: f64 = a[..n].iter().map(|x| (x - ma).powi(2)).sum::<f64>().sqrt();
    let db: f64 = b[..n].iter().map(|y| (y - mb).powi(2)).sum::<f64>().sqrt();
    if da < 1e-12 || db < 1e-12 { return None; }
    Some(num / (da * db))
}

// ─────────────────────────────────────────────────────────────────────────────
// State stored on UltraLogApp (added by wiring code)
// ─────────────────────────────────────────────────────────────────────────────

/// Analysis state kept across frames (cached on last run).
#[derive(Default)]
pub struct Msv70AnalysisState {
    pub report: String,
    pub wot_pct: f64,
    pub cruise_pct: f64,
    pub idle_pct: f64,
    pub transient_pct: f64,
    /// Thresholds (editable in the panel)
    pub wot_tps_thresh: f64,
    pub idle_rpm_thresh: f64,
    pub idle_load_thresh: f64,
    pub ltft_warn: f64,
    pub bank_alert: f64,
    pub min_coolant: f64,
}

impl Msv70AnalysisState {
    pub fn new() -> Self {
        Self {
            wot_tps_thresh: 85.0,
            idle_rpm_thresh: 900.0,
            idle_load_thresh: 12.0,
            ltft_warn: 5.0,
            bank_alert: 5.0,
            min_coolant: 70.0,
            ..Default::default()
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Panel rendering
// ─────────────────────────────────────────────────────────────────────────────

impl UltraLogApp {
    /// Render the MSV70 Fuel Trim & Phase Analysis floating window.
    pub fn render_msv70_analysis(&mut self, ctx: &egui::Context) {
        if !self.show_msv70_analysis {
            return;
        }

        let mut open = true;
        egui::Window::new("MSV70 — Fuel Trim & Phase Analysis")
            .open(&mut open)
            .resizable(true)
            .default_width(640.0)
            .default_height(580.0)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                self.render_msv70_analysis_content(ui);
            });

        if !open {
            self.show_msv70_analysis = false;
        }
    }

    fn render_msv70_analysis_content(&mut self, ui: &mut egui::Ui) {
        let has_file = self.selected_file.is_some() && !self.files.is_empty();

        // ── Threshold controls ────────────────────────────────────────────────
        egui::CollapsingHeader::new("⚙  Thresholds")
            .default_open(false)
            .show(ui, |ui| {
                egui::Grid::new("msv70_thresholds")
                    .num_columns(2)
                    .spacing([16.0, 4.0])
                    .show(ui, |ui| {
                        let s = &mut self.msv70_analysis;
                        ui.label("WOT pedal (%):");
                        ui.add(egui::DragValue::new(&mut s.wot_tps_thresh).range(50.0..=100.0).speed(0.5));
                        ui.end_row();
                        ui.label("Idle RPM max:");
                        ui.add(egui::DragValue::new(&mut s.idle_rpm_thresh).range(400.0..=1500.0).speed(10.0));
                        ui.end_row();
                        ui.label("Idle load max (%):");
                        ui.add(egui::DragValue::new(&mut s.idle_load_thresh).range(2.0..=30.0).speed(0.5));
                        ui.end_row();
                        ui.label("LTFT warning (%):");
                        ui.add(egui::DragValue::new(&mut s.ltft_warn).range(1.0..=20.0).speed(0.5));
                        ui.end_row();
                        ui.label("Bank alert (Δ%):");
                        ui.add(egui::DragValue::new(&mut s.bank_alert).range(1.0..=15.0).speed(0.5));
                        ui.end_row();
                        ui.label("Min coolant (°C):");
                        ui.add(egui::DragValue::new(&mut s.min_coolant).range(40.0..=100.0).speed(1.0));
                        ui.end_row();
                    });
            });

        ui.add_space(6.0);

        // ── Run button ────────────────────────────────────────────────────────
        let run_resp = ui.add_enabled(
            has_file,
            egui::Button::new("▶  Run Analysis"),
        );

        if run_resp.clicked() {
            if let Some(file_idx) = self.selected_file {
                if let Some(file) = self.files.get(file_idx) {
                    let report = Self::compute_msv70_report(&file.log, &self.msv70_analysis);
                    self.msv70_analysis.report = report;
                }
            }
        }

        if !has_file {
            ui.colored_label(egui::Color32::GRAY, "Load a Testo CSV log first.");
            return;
        }

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(4.0);

        // ── Phase bar ─────────────────────────────────────────────────────────
        if self.msv70_analysis.wot_pct + self.msv70_analysis.cruise_pct > 0.0 {
            ui.label(egui::RichText::new("Engine State Distribution").strong());
            ui.add_space(2.0);

            let s = &self.msv70_analysis;
            for (label, pct, color) in [
                ("WOT",        s.wot_pct,        egui::Color32::from_rgb(192, 57, 43)),
                ("Cruise",     s.cruise_pct,     egui::Color32::from_rgb(39, 174, 96)),
                ("Transient",  s.transient_pct,  egui::Color32::from_rgb(243, 156, 18)),
                ("Idle",       s.idle_pct,        egui::Color32::from_rgb(127, 140, 141)),
            ] {
                ui.horizontal(|ui| {
                    ui.label(format!("{:<12}", label));
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(200.0 * pct as f32 / 100.0, 14.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(rect, 2.0, color);
                    ui.label(format!("{:.1}%", pct));
                });
            }
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);
        }

        // ── Report text ───────────────────────────────────────────────────────
        egui::ScrollArea::vertical()
            .id_salt("msv70_report_scroll")
            .show(ui, |ui| {
                let report = self.msv70_analysis.report.clone();
                if report.is_empty() {
                    ui.colored_label(egui::Color32::GRAY, "Press '▶ Run Analysis' to see results.");
                } else {
                    ui.add(
                        egui::TextEdit::multiline(&mut report.as_str())
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .interactive(false),
                    );
                }
            });
    }

    fn compute_msv70_report(log: &crate::parsers::Log, cfg: &Msv70AnalysisState) -> String {
        let mut out = String::new();
        macro_rules! ln {
            ($($t:tt)*) => { out.push_str(&format!($($t)*)); out.push('\n'); }
        }

        ln!("╔══════════════════════════════════════════════╗");
        ln!("║   MSV70 — Fuel Trim & Phase Diagnostic      ║");
        ln!("╚══════════════════════════════════════════════╝");
        ln!("");

        // ── State machine ─────────────────────────────────────────────────────
        let rpm_data  = get_data(log, RPM_NAMES);
        let tps_data  = get_data(log, TPS_NAMES);
        let load_data = get_data(log, LOAD_NAMES);

        let (wot_n, idle_n, trans_n, cruise_n, total) = if let (Some(tps), Some(rpm), Some(load)) =
            (&tps_data, &rpm_data, &load_data)
        {
            let n = tps.len().min(rpm.len()).min(load.len());
            let mut wot = 0usize; let mut idle = 0; let mut trans = 0; let mut cruise = 0;
            for i in 1..n {
                let dtps = (tps[i] - tps[i - 1]).abs();
                if tps[i] > cfg.wot_tps_thresh              { wot   += 1; }
                else if rpm[i] < cfg.idle_rpm_thresh
                     || load[i] < cfg.idle_load_thresh       { idle  += 1; }
                else if dtps > 5.0                            { trans += 1; }
                else                                          { cruise += 1; }
            }
            (wot, idle, trans, cruise, n.saturating_sub(1))
        } else { (0, 0, 0, 0, 0) };

        let pct = |n: usize| if total > 0 { n as f64 / total as f64 * 100.0 } else { 0.0 };
        ln!("[ ENGINE STATE DISTRIBUTION ]");
        ln!("  WOT        {:5.1}%", pct(wot_n));
        ln!("  Cruise     {:5.1}%", pct(cruise_n));
        ln!("  Transient  {:5.1}%", pct(trans_n));
        ln!("  Idle       {:5.1}%", pct(idle_n));
        ln!("");

        // Warm mask
        let warm_mask: Vec<bool> = if let Some(cool) = get_data(log, COOLANT_NAMES) {
            cool.iter().map(|&c| c >= cfg.min_coolant).collect()
        } else {
            vec![true; log.times.len()]
        };
        let warm_count = warm_mask.iter().filter(|&&b| b).count();
        let cold_pct = if !warm_mask.is_empty() {
            (warm_mask.len() - warm_count) as f64 / warm_mask.len() as f64 * 100.0
        } else { 0.0 };

        ln!("[ TEMPERATURE ]");
        if cold_pct > 20.0 {
            ln!("  ⚠  Cold samples (< {:.0}°C): {:.1}% — trims may be skewed",
                cfg.min_coolant, cold_pct);
        } else {
            ln!("  ✓  Engine warm (cold fraction: {:.1}%)", cold_pct);
        }
        ln!("");

        // Helper: warm mean
        let warm_mean = |v: &Vec<f64>| -> f64 {
            let warm: Vec<f64> = v.iter().zip(warm_mask.iter())
                .filter_map(|(&val, &w)| if w { Some(val) } else { None })
                .collect();
            mean(&warm)
        };
        let warm_max_abs = |v: &Vec<f64>| -> f64 {
            let warm: Vec<f64> = v.iter().zip(warm_mask.iter())
                .filter_map(|(&val, &w)| if w { Some(val) } else { None })
                .collect();
            max_abs(&warm)
        };

        // ── Fuel trims ────────────────────────────────────────────────────────
        ln!("[ FUEL TRIM ANALYSIS ]");

        let stft1 = get_data(log, STFT1_NAMES);
        let stft2 = get_data(log, STFT2_NAMES);
        let ltft_add1 = get_data(log, LTFT_ADD1_NAMES);
        let ltft_add2 = get_data(log, LTFT_ADD2_NAMES);
        let ltft_mul1 = get_data(log, LTFT_MUL1_NAMES);
        let ltft_mul2 = get_data(log, LTFT_MUL2_NAMES);

        for (bank, stft, ltft_a, ltft_m) in [
            (1usize, &stft1, &ltft_add1, &ltft_mul1),
            (2usize, &stft2, &ltft_add2, &ltft_mul2),
        ] {
            let stft_mean = stft.as_ref().map(|v| warm_mean(v));
            let stft_max  = stft.as_ref().map(|v| warm_max_abs(v));
            let la_mean   = ltft_a.as_ref().map(|v| warm_mean(v));
            let lm_mean   = ltft_m.as_ref().map(|v| warm_mean(v));

            if stft_mean.is_none() && lm_mean.is_none() {
                ln!("  Bank {} — no trim data found.", bank);
                continue;
            }

            ln!("  ┌─ Bank {} ─────────────────────────────────", bank);
            if let (Some(sm), Some(sx)) = (stft_mean, stft_max) {
                let flag = if sm.abs() < cfg.ltft_warn { "✓" }
                           else if sm.abs() < cfg.ltft_warn * 2.0 { "⚠" }
                           else { "✗ CRITICAL" };
                ln!("  │  STFT  mean={:+.2}%  max={:.2}%  {}", sm, sx, flag);
            }
            if let Some(la) = la_mean {
                let flag = if la.abs() < cfg.ltft_warn { "✓" } else { "⚠" };
                ln!("  │  LTFT add  mean={:+.2}%  {}", la, flag);
            }
            if let Some(lm) = lm_mean {
                let flag = if lm.abs() < cfg.ltft_warn { "✓" }
                           else if lm.abs() < cfg.ltft_warn * 2.0 { "⚠" }
                           else { "✗ CRITICAL" };
                ln!("  │  LTFT mul  mean={:+.2}%  {}", lm, flag);
            }
            let total_drift = stft_mean.unwrap_or(0.0).abs() + lm_mean.unwrap_or(0.0).abs();
            let verdict = if total_drift < 3.5 { "✓ HEALTHY" }
                          else if total_drift < 7.0 { "~ MARGINAL" }
                          else { "✗ ACTION REQUIRED" };
            ln!("  └─ Verdict: {}", verdict);
            ln!("");
        }

        // ── Bank balance ──────────────────────────────────────────────────────
        ln!("[ BANK BALANCE ]");
        let b1_trim = stft1.as_ref().map(|s| {
            s.iter().zip(warm_mask.iter())
             .filter_map(|(&v, &w)| if w { Some(v) } else { None })
             .zip(ltft_mul1.as_ref().map(|v| v.iter().copied().collect::<Vec<_>>()).unwrap_or_default().into_iter()
                  .chain(std::iter::repeat(0.0)))
             .map(|(s, m)| s + m)
             .collect::<Vec<f64>>()
        });
        let b2_trim = stft2.as_ref().map(|s| {
            s.iter().zip(warm_mask.iter())
             .filter_map(|(&v, &w)| if w { Some(v) } else { None })
             .zip(ltft_mul2.as_ref().map(|v| v.iter().copied().collect::<Vec<_>>()).unwrap_or_default().into_iter()
                  .chain(std::iter::repeat(0.0)))
             .map(|(s, m)| s + m)
             .collect::<Vec<f64>>()
        });

        if let (Some(b1), Some(b2)) = (b1_trim, b2_trim) {
            let n = b1.len().min(b2.len());
            if n > 0 {
                let delta_mean: f64 = b1[..n].iter().zip(b2[..n].iter()).map(|(a,b)| (a-b).abs()).sum::<f64>() / n as f64;
                let delta_max: f64  = b1[..n].iter().zip(b2[..n].iter()).map(|(a,b)| (a-b).abs()).fold(0.0_f64, f64::max);
                let flag = if delta_mean > cfg.bank_alert { "✗ ALERT — check injectors/O2 sensors" } else { "✓ OK" };
                ln!("  Mean Δ = {:.2}%   Max Δ = {:.2}%   {}", delta_mean, delta_max, flag);
            } else {
                ln!("  — insufficient warm samples");
            }
        } else {
            ln!("  — trim data missing for one or both banks");
        }
        ln!("");

        // ── MAF / Load correlation ─────────────────────────────────────────────
        ln!("[ AIRFLOW CHECK — MAF vs Calculated Load ]");
        if let (Some(maf), Some(load)) = (get_data(log, MAF_NAMES), &load_data) {
            let pairs: Vec<(f64,f64)> = maf.iter().zip(load.iter()).zip(warm_mask.iter())
                .filter_map(|((&m, &l), &w)| if w && m > 0.0 && l > 0.0 { Some((m, l)) } else { None })
                .collect();
            let ma: Vec<f64> = pairs.iter().map(|p| p.0).collect();
            let la: Vec<f64> = pairs.iter().map(|p| p.1).collect();
            if let Some(r) = pearson(&ma, &la) {
                let verdict = if r > 0.88 { "✓ Excellent — no significant air leak" }
                              else if r > 0.70 { "~ Marginal — check DISA flaps or intake seals" }
                              else { "✗ Poor — check MAF sensor, DISA, intake gaskets" };
                ln!("  Pearson r = {:.4}   {}", r, verdict);
            } else {
                ln!("  — insufficient data pairs");
            }
        } else {
            ln!("  — MAF or Load column missing from this log");
        }
        ln!("");

        // ── Injector duty ─────────────────────────────────────────────────────
        if let (Some(inj), Some(rpm)) = (get_data(log, INJ_NAMES), &rpm_data) {
            let duties: Vec<f64> = inj.iter().zip(rpm.iter()).zip(warm_mask.iter())
                .filter_map(|((&t, &r), &w)| {
                    if w && r > 100.0 {
                        Some((t / (120_000.0 / r)) * 100.0)
                    } else { None }
                })
                .collect();
            if !duties.is_empty() {
                let d_mean = mean(&duties);
                let d_max  = duties.iter().cloned().fold(0.0_f64, f64::max);
                ln!("[ INJECTOR DUTY CYCLE (Cyl 1, warm) ]");
                ln!("  Mean = {:.1}%   Max = {:.1}%{}", d_mean, d_max,
                    if d_max > 85.0 { "  ⚠  Near saturation" } else { "" });
                ln!("");
            }
        }

        ln!("─────────────────────────────────────────────────");
        ln!("Analysis complete.");

        out
    }
}
