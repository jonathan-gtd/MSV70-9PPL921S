//! MSV70 Fuel Trim & Phase Analysis — complete rewrite.
//!
//! Tabbed floating window with:
//!  - Dashboard  : coloured health cards + global score
//!  - Phases     : 7-phase distribution bars + chart overlay toggle
//!  - Fuel       : STFT/LTFT/bank-balance/lambda gauges
//!  - Engine     : MAF, load, knock, injectors, electrical, temperatures
//!  - Settings   : editable thresholds

use eframe::egui;
use crate::app::UltraLogApp;
use crate::parsers::Log;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

const NEAR_ZERO: f64 = 1e-10;

const RPM:       &[&str] = &["RPM"];
const LOAD:      &[&str] = &["CalcLoad", "Load %", "Calc Load"];
const TPS:       &[&str] = &["Pedal %", "Pedal", "TPS"];
const COOLANT:   &[&str] = &["Coolant", "Coolant Temp"];
const STFT1:     &[&str] = &["STFT B1", "STFT Bank1"];
const STFT2:     &[&str] = &["STFT B2", "STFT Bank2"];
const LTFA1:     &[&str] = &["LTFT add. B1", "LTFT add B1"];
const LTFA2:     &[&str] = &["LTFT add. B2", "LTFT add B2"];
const LTFM1:     &[&str] = &["LTFT mul. B1", "LTFT mul B1"];
const LTFM2:     &[&str] = &["LTFT mul. B2", "LTFT mul B2"];
const LAMBDA1:   &[&str] = &["Lambda B1", "Lambda 1", "Lambda"];
const MAF:       &[&str] = &["AirMass", "MAF kg/h", "MAF"];
const INJ:       &[&str] = &["InjTmCyl1", "Inj C1 ms", "Inj Time"];
const KNOCK:     &[&str] = &["Knock retard", "Knock Retard", "Knock"];
const VANOS_ACT: &[&str] = &["VANOS act.", "VANOS actual", "VANOS pos"];
const VANOS_SET: &[&str] = &["VANOS set.", "VANOS setpoint", "VANOS tgt"];
const BATTERY:   &[&str] = &["Battery V", "Battery", "Voltage", "Ubatt"];
const OIL:       &[&str] = &["Oil Temp", "Oil temp", "OilTemp"];

// ─────────────────────────────────────────────────────────────────────────────
// Phase engine
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Phase {
    WarmUp,
    Wot,
    Acceleration,
    Cruise,
    Deceleration,
    Overrun,
    Idle,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::WarmUp       => "Warm-Up",
            Phase::Wot          => "WOT",
            Phase::Acceleration => "Accélération",
            Phase::Cruise       => "Croisière",
            Phase::Deceleration => "Décélération/DFCO",
            Phase::Overrun      => "Roue libre",
            Phase::Idle         => "Ralenti",
        }
    }

    pub fn bar_color(self) -> egui::Color32 {
        match self {
            Phase::WarmUp       => egui::Color32::from_rgb(100, 160, 220),
            Phase::Wot          => egui::Color32::from_rgb(210, 60,  50),
            Phase::Acceleration => egui::Color32::from_rgb(230, 140, 30),
            Phase::Cruise       => egui::Color32::from_rgb(60,  180, 90),
            Phase::Deceleration => egui::Color32::from_rgb(160, 80,  200),
            Phase::Overrun      => egui::Color32::from_rgb(80,  130, 200),
            Phase::Idle         => egui::Color32::from_rgb(130, 130, 130),
        }
    }

    /// Low-alpha colour for chart background overlay
    pub fn overlay_color(self) -> egui::Color32 {
        let c = self.bar_color();
        egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 65)
    }

    pub fn all() -> &'static [Phase] {
        &[Phase::WarmUp, Phase::Wot, Phase::Acceleration, Phase::Cruise,
          Phase::Deceleration, Phase::Overrun, Phase::Idle]
    }
}

/// A contiguous time segment with a single phase label
#[derive(Clone, Debug)]
pub struct PhaseSegment {
    pub start: f64,
    pub end:   f64,
    pub phase: Phase,
}

// ─────────────────────────────────────────────────────────────────────────────
// Diagnostic result
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum DiagStatus { Ok, Warn, Critical, Info, Missing }

impl DiagStatus {
    pub fn color(self) -> egui::Color32 {
        match self {
            DiagStatus::Ok       => egui::Color32::from_rgb(50,  190, 100),
            DiagStatus::Warn     => egui::Color32::from_rgb(230, 160,  30),
            DiagStatus::Critical => egui::Color32::from_rgb(210,  55,  55),
            DiagStatus::Info     => egui::Color32::from_rgb(100, 150, 220),
            DiagStatus::Missing  => egui::Color32::from_rgb(90,   90,  90),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            DiagStatus::Ok       => "OK",
            DiagStatus::Warn     => "ATTENTION",
            DiagStatus::Critical => "CRITIQUE",
            DiagStatus::Info     => "INFO",
            DiagStatus::Missing  => "N/A",
        }
    }
}

#[derive(Clone)]
pub struct DiagCheck {
    pub label:  String,
    pub value:  String,
    pub status: DiagStatus,
    pub detail: String,
}

impl DiagCheck {
    #[allow(dead_code)]
    fn ok(label: &str, value: impl Into<String>, detail: &str) -> Self {
        Self { label: label.into(), value: value.into(), status: DiagStatus::Ok,       detail: detail.into() }
    }
    #[allow(dead_code)]
    fn warn(label: &str, value: impl Into<String>, detail: &str) -> Self {
        Self { label: label.into(), value: value.into(), status: DiagStatus::Warn,     detail: detail.into() }
    }
    #[allow(dead_code)]
    fn crit(label: &str, value: impl Into<String>, detail: &str) -> Self {
        Self { label: label.into(), value: value.into(), status: DiagStatus::Critical, detail: detail.into() }
    }
    fn info(label: &str, value: impl Into<String>, detail: &str) -> Self {
        Self { label: label.into(), value: value.into(), status: DiagStatus::Info,     detail: detail.into() }
    }
    fn missing(label: &str) -> Self {
        Self { label: label.into(), value: "—".into(), status: DiagStatus::Missing,
               detail: "Canal absent du log".into() }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Panel state
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct Msv70AnalysisState {
    pub active_tab:        usize,
    /// Selected phase in the "Par Phase" tab (index into Phase::all())
    pub selected_phase_tab: usize,
    pub show_phase_overlay: bool,
    pub phase_segments:    Vec<PhaseSegment>,
    pub phase_counts:      [usize; 7],
    pub total_samples:     usize,

    pub checks_fuel:   Vec<DiagCheck>,
    pub checks_engine: Vec<DiagCheck>,
    pub checks_elec:   Vec<DiagCheck>,
    pub checks_temp:   Vec<DiagCheck>,
    /// Per-phase deep diagnostics (indexed by Phase as usize)
    pub checks_by_phase: Vec<Vec<DiagCheck>>,
    /// Per-phase means: [stft_b1, stft_b2, ltft_add_b1, ltft_add_b2, ltft_mul_b1, ltft_mul_b2]
    /// Indexed by Phase as usize. NaN if channel absent or fewer than 5 samples in that phase.
    pub bank_compare_by_phase: Vec<[f64; 6]>,

    // Thresholds
    pub wot_tps_thresh:    f64,
    pub idle_rpm_thresh:   f64,
    pub idle_load_thresh:  f64,
    pub accel_dtps_thresh: f64,
    pub decel_drpm_thresh: f64,
    pub overrun_tps_max:   f64,
    pub phase_min_dur_s:   f64,
    pub ltft_warn:         f64,
    pub ltft_crit:         f64,
    pub bank_alert:        f64,
    pub min_coolant:       f64,
    pub duty_warn:         f64,
    pub duty_crit:         f64,

    // E85-specific thresholds
    pub wot_lambda_min_e85: f64,
    pub wot_lambda_max_e85: f64,
    pub e85_duty_warn:      f64,
    pub e85_duty_crit:      f64,

    pub checks_e85: Vec<DiagCheck>,
}

impl Msv70AnalysisState {
    pub fn new() -> Self {
        Self {
            show_phase_overlay: true,
            wot_tps_thresh:    85.0,
            idle_rpm_thresh:   900.0,
            idle_load_thresh:  12.0,
            accel_dtps_thresh: 20.0,
            decel_drpm_thresh: -200.0,
            overrun_tps_max:   5.0,
            phase_min_dur_s:   1.0,
            ltft_warn:         5.0,
            ltft_crit:         10.0,
            bank_alert:        5.0,
            min_coolant:       70.0,
            duty_warn:         80.0,
            duty_crit:         85.0,
            wot_lambda_min_e85: 0.82,
            wot_lambda_max_e85: 0.90,
            e85_duty_warn:      80.0,
            e85_duty_crit:      92.0,
            ..Default::default()
        }
    }

    pub fn health_score(&self) -> (usize, usize) {
        let all: Vec<&DiagCheck> = self.checks_fuel.iter()
            .chain(&self.checks_engine).chain(&self.checks_elec).chain(&self.checks_temp)
            .collect();
        let scored: Vec<&DiagCheck> = all.iter().copied()
            .filter(|c| c.status != DiagStatus::Missing).collect();
        if scored.is_empty() { return (0, 0); }
        let pts: usize = scored.iter().map(|c| match c.status {
            DiagStatus::Ok | DiagStatus::Info => 2,
            DiagStatus::Warn     => 1,
            DiagStatus::Critical => 0,
            DiagStatus::Missing  => 0,
        }).sum();
        (pts * 100 / (scored.len() * 2), scored.len())
    }

    pub fn e85_score(&self) -> (usize, usize) {
        let scored: Vec<&DiagCheck> = self.checks_e85.iter()
            .filter(|c| c.status != DiagStatus::Missing && c.status != DiagStatus::Info)
            .collect();
        if scored.is_empty() { return (0, 0); }
        let pts: usize = scored.iter().map(|c| match c.status {
            DiagStatus::Ok | DiagStatus::Info => 2,
            DiagStatus::Warn     => 1,
            DiagStatus::Critical | DiagStatus::Missing => 0,
        }).sum();
        (pts * 100 / (scored.len() * 2), scored.len())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Data helpers
// ─────────────────────────────────────────────────────────────────────────────

fn get(log: &Log, names: &[&str]) -> Option<Vec<f64>> {
    for name in names {
        if let Some(idx) = log.find_channel_index(name) {
            let v: Vec<f64> = log.get_channel_data(idx).into_iter()
                .map(|x| if x.abs() < NEAR_ZERO { 0.0 } else { x }).collect();
            return Some(v);
        }
    }
    None
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { return 0.0; }
    v.iter().sum::<f64>() / v.len() as f64
}

fn stddev(v: &[f64]) -> f64 {
    if v.len() < 2 { return 0.0; }
    let m = mean(v);
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / v.len() as f64).sqrt()
}

fn maxabs(v: &[f64]) -> f64 { v.iter().map(|x| x.abs()).fold(0.0_f64, f64::max) }
fn vmax(v: &[f64]) -> f64   { v.iter().cloned().fold(f64::NEG_INFINITY, f64::max) }
fn vmin(v: &[f64]) -> f64   { v.iter().cloned().fold(f64::INFINITY, f64::min) }

fn warm_vals(v: &[f64], mask: &[bool]) -> Vec<f64> {
    v.iter().zip(mask.iter()).filter_map(|(&x, &w)| if w { Some(x) } else { None }).collect()
}

fn pearson(a: &[f64], b: &[f64]) -> Option<f64> {
    let n = a.len().min(b.len());
    if n < 10 { return None; }
    let (ma, mb) = (mean(&a[..n]), mean(&b[..n]));
    let num: f64 = a[..n].iter().zip(b[..n].iter()).map(|(x, y)| (x - ma) * (y - mb)).sum();
    let da = a[..n].iter().map(|x| (x - ma).powi(2)).sum::<f64>().sqrt();
    let db = b[..n].iter().map(|y| (y - mb).powi(2)).sum::<f64>().sqrt();
    if da < 1e-12 || db < 1e-12 { return None; }
    Some(num / (da * db))
}

fn linear_slope(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len().min(ys.len());
    if n < 2 { return 0.0; }
    let mx = mean(&xs[..n]);
    let my = mean(&ys[..n]);
    let num: f64 = xs[..n].iter().zip(ys[..n].iter()).map(|(x, y)| (x - mx) * (y - my)).sum();
    let den: f64 = xs[..n].iter().map(|x| (x - mx).powi(2)).sum();
    if den.abs() < 1e-12 { 0.0 } else { num / den }
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase classification
// ─────────────────────────────────────────────────────────────────────────────

fn classify_phases(log: &Log, cfg: &Msv70AnalysisState) -> Vec<Phase> {
    let n = log.times.len();
    let rpm_d  = get(log, RPM);
    let tps_d  = get(log, TPS);
    let load_d = get(log, LOAD);
    let cool_d = get(log, COOLANT);
    let mut out = vec![Phase::Idle; n];

    // Rate smoothing window: look back up to 0.5 s to compute %/s and RPM/s.
    // This normalises for sample rate and filters single-sample noise.
    const SMOOTH_S: f64 = 0.5;

    for i in 1..n {
        let r = rpm_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(0.0);
        let t = tps_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(0.0);
        let l = load_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(0.0);
        let c = cool_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(100.0);

        // Reference index: earliest sample within the smoothing window
        let t_ref_target = log.times[i] - SMOOTH_S;
        let ref_idx = log.times[..i].partition_point(|&ts| ts < t_ref_target).saturating_sub(1);
        let dt_s = (log.times[i] - log.times[ref_idx]).max(1e-6);

        let t_ref = tps_d.as_deref().and_then(|v| v.get(ref_idx)).copied().unwrap_or(t);
        let r_ref = rpm_d.as_deref().and_then(|v| v.get(ref_idx)).copied().unwrap_or(r);

        let dtps_per_s = (t - t_ref) / dt_s;  // %/s
        let drpm_per_s = (r - r_ref) / dt_s;  // RPM/s

        out[i] = if c < cfg.min_coolant {
            Phase::WarmUp
        } else if t > cfg.wot_tps_thresh {
            Phase::Wot
        } else if t < cfg.overrun_tps_max && r > cfg.idle_rpm_thresh + 200.0 {
            if drpm_per_s < cfg.decel_drpm_thresh { Phase::Deceleration } else { Phase::Overrun }
        } else if r < cfg.idle_rpm_thresh || l < cfg.idle_load_thresh {
            Phase::Idle
        } else if dtps_per_s > cfg.accel_dtps_thresh {
            Phase::Acceleration
        } else {
            Phase::Cruise
        };
    }

    debounce_phases(&mut out, &log.times, cfg.phase_min_dur_s);
    out
}

/// Merge phase segments shorter than `min_dur_s` into the preceding phase.
/// Iterates until stable (handles chains of short segments).
fn debounce_phases(phases: &mut [Phase], times: &[f64], min_dur_s: f64) {
    let n = phases.len().min(times.len());
    if n == 0 || min_dur_s <= 0.0 { return; }

    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i < n {
            let cur = phases[i];
            let mut j = i + 1;
            while j < n && phases[j] == cur { j += 1; }

            let seg_end = times[j.min(n - 1)];
            let seg_dur = seg_end - times[i];
            if seg_dur < min_dur_s && i > 0 {
                let prev = phases[i - 1];
                for k in i..j { phases[k] = prev; }
                changed = true;
            }

            i = j;
        }
    }
}

fn to_segments(phases: &[Phase], times: &[f64]) -> Vec<PhaseSegment> {
    if phases.is_empty() || times.is_empty() { return Vec::new(); }
    let mut segs: Vec<PhaseSegment> = Vec::new();
    let mut cur = phases[0];
    let mut start = times[0];
    let n = phases.len().min(times.len());

    for i in 1..n {
        if phases[i] != cur {
            // Emit or extend: guarantees zero gaps between adjacent segments
            match segs.last_mut() {
                Some(last) if last.phase == cur => last.end = times[i],
                _ => segs.push(PhaseSegment { start, end: times[i], phase: cur }),
            }
            cur = phases[i];
            start = times[i];
        }
    }
    // Final segment
    if let Some(&t_end) = times.last() {
        match segs.last_mut() {
            Some(last) if last.phase == cur => last.end = t_end,
            _ => segs.push(PhaseSegment { start, end: t_end, phase: cur }),
        }
    }
    segs
}

// ─────────────────────────────────────────────────────────────────────────────
// Diagnostic builders
// ─────────────────────────────────────────────────────────────────────────────

fn fuel_checks(log: &Log, cfg: &Msv70AnalysisState, warm: &[bool], phases: &[Phase]) -> Vec<DiagCheck> {
    let mut out = Vec::new();
    let (lw, lc) = (cfg.ltft_warn, cfg.ltft_crit);
    let cruise_m: Vec<bool> = warm.iter().zip(phases.iter())
        .map(|(&w, &p)| w && p == Phase::Cruise).collect();
    let wot_m: Vec<bool> = phases.iter().map(|&p| p == Phase::Wot).collect();

    // STFT
    for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
        if let Some(d) = get(log, names) {
            let wv = warm_vals(&d, warm);
            let m = mean(&wv); let mx = maxabs(&wv);
            let s = if m.abs() < lw { DiagStatus::Ok } else if m.abs() < lc { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: format!("STFT B{} — moyenne", b), value: format!("{:+.2}%", m),
                status: s, detail: format!("Max |val|: {:.2}%", mx) });
        } else { out.push(DiagCheck::missing(&format!("STFT B{}", b))); }
    }

    // LTFT add
    for (b, names) in [(1usize, LTFA1), (2usize, LTFA2)] {
        if let Some(d) = get(log, names) {
            let m = mean(&warm_vals(&d, warm));
            let s = if m.abs() < lw { DiagStatus::Ok } else { DiagStatus::Warn };
            out.push(DiagCheck { label: format!("LTFT add B{}", b), value: format!("{:+.2}%", m), status: s, detail: String::new() });
        } else { out.push(DiagCheck::missing(&format!("LTFT add B{}", b))); }
    }

    // LTFT mul
    for (b, names) in [(1usize, LTFM1), (2usize, LTFM2)] {
        if let Some(d) = get(log, names) {
            let m = mean(&warm_vals(&d, warm));
            let s = if m.abs() < lw { DiagStatus::Ok } else if m.abs() < lc { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: format!("LTFT mul B{}", b), value: format!("{:+.2}%", m), status: s, detail: String::new() });
        } else { out.push(DiagCheck::missing(&format!("LTFT mul B{}", b))); }
    }

    // Bank balance
    let b1 = get(log, STFT1).zip(get(log, LTFM1));
    let b2 = get(log, STFT2).zip(get(log, LTFM2));
    match (b1, b2) {
        (Some((s1, m1)), Some((s2, m2))) => {
            let n = s1.len().min(s2.len()).min(m1.len()).min(m2.len());
            let deltas: Vec<f64> = (0..n).filter(|&i| warm.get(i).copied().unwrap_or(false))
                .map(|i| ((s1[i] + m1[i]) - (s2[i] + m2[i])).abs()).collect();
            let dm = mean(&deltas); let dmax = vmax(&deltas.iter().map(|&x| x).collect::<Vec<_>>());
            let s = if dm < cfg.bank_alert { DiagStatus::Ok } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Balance banques (Δ moyen)".into(),
                value: format!("{:.2}%", dm), status: s, detail: format!("Max Δ: {:.2}%", dmax) });
        }
        _ => out.push(DiagCheck::missing("Balance banques")),
    }

    // Lambda WOT
    if let Some(lam) = get(log, LAMBDA1) {
        let wv: Vec<f64> = lam.iter().zip(wot_m.iter()).filter_map(|(&v, &w)| if w && v > 0.5 { Some(v) } else { None }).collect();
        if wv.is_empty() {
            out.push(DiagCheck::info("Lambda WOT", "—", "Aucun échantillon WOT"));
        } else {
            let lm = mean(&wv);
            let s = if (0.85..=0.96).contains(&lm) { DiagStatus::Ok }
                    else if (0.80..=1.02).contains(&lm) { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Lambda WOT (moyenne)".into(), value: format!("{:.3} λ", lm),
                status: s, detail: "Cible 0.85–0.96 (atmo)".into() });
        }
    } else { out.push(DiagCheck::missing("Lambda WOT")); }

    // Lambda closed-loop stability
    if let Some(lam) = get(log, LAMBDA1) {
        let cv: Vec<f64> = lam.iter().zip(cruise_m.iter()).filter_map(|(&v, &w)| if w && v > 0.5 { Some(v) } else { None }).collect();
        if cv.len() > 10 {
            let sd = stddev(&cv);
            let s = if sd < 0.025 { DiagStatus::Ok } else if sd < 0.05 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Lambda stabilité croisière (σ)".into(), value: format!("{:.4}", sd),
                status: s, detail: "< 0.025 = BF stable  |  > 0.05 = chasse".into() });
        }
    }

    // Lambda at decel/DFCO (should go lean or cut)
    if let Some(lam) = get(log, LAMBDA1) {
        let dec_m: Vec<bool> = phases.iter().map(|&p| p == Phase::Deceleration).collect();
        let dv: Vec<f64> = lam.iter().zip(dec_m.iter()).filter_map(|(&v, &w)| if w && v > 0.5 { Some(v) } else { None }).collect();
        if dv.len() > 5 {
            let lm = mean(&dv);
            let s = if lm > 1.05 { DiagStatus::Ok } else { DiagStatus::Warn };
            out.push(DiagCheck { label: "Lambda décélération/DFCO".into(), value: format!("{:.3} λ", lm),
                status: s, detail: "Doit être maigre (> 1.05) ou coupure carbu".into() });
        }
    }

    // Total correction factor B1 / B2  (STFT + LTFT_mul combined — net error the ECU compensates)
    for (b, snames, mnames) in [(1usize, STFT1, LTFM1), (2usize, STFT2, LTFM2)] {
        if let (Some(stft), Some(ltfm)) = (get(log, snames), get(log, mnames)) {
            let n = stft.len().min(ltfm.len());
            let combined: Vec<f64> = (0..n)
                .filter(|&i| warm.get(i).copied().unwrap_or(false))
                .map(|i| stft[i] + ltfm[i])
                .collect();
            if !combined.is_empty() {
                let m = mean(&combined);
                let s = if m.abs() < cfg.ltft_warn { DiagStatus::Ok }
                        else if m.abs() < cfg.ltft_crit { DiagStatus::Warn }
                        else { DiagStatus::Critical };
                let interp = if m > cfg.ltft_crit { "Moteur maigre — revoir cartographie ou fuites" }
                             else if m < -cfg.ltft_crit { "Moteur riche — injecteurs/pression carburant" }
                             else if m.abs() < cfg.ltft_warn { "Mélange correct" }
                             else { "Dérive légère — surveiller" };
                out.push(DiagCheck {
                    label: format!("Correction totale B{} (STFT+LTFT_mul)", b),
                    value: format!("{:+.2}%", m),
                    status: s,
                    detail: interp.into(),
                });
            }
        }
    }

    // STFT oscillation σ in cruise (hunting = bad O2 sensor / exhaust leak)
    let cruise_stft_samples: Vec<f64> = {
        let mut v = Vec::new();
        if let Some(d) = get(log, STFT1) {
            v.extend(d.iter().zip(phases.iter()).zip(warm.iter())
                .filter_map(|((&x, &p), &w)| if w && p == Phase::Cruise { Some(x) } else { None }));
        }
        v
    };
    if cruise_stft_samples.len() > 20 {
        let sd = stddev(&cruise_stft_samples);
        let s = if sd < 2.0 { DiagStatus::Ok } else if sd < 4.0 { DiagStatus::Warn } else { DiagStatus::Critical };
        out.push(DiagCheck {
            label: "STFT oscillation croisière (σ)".into(),
            value: format!("{:.2}%", sd),
            status: s,
            detail: if sd > 4.0 { "Sonde O2 chasse — vérifier sonde/fuite échappement".into() }
                    else if sd > 2.0 { "Légère instabilité boucle fermée".into() }
                    else { "Boucle fermée stable".into() },
        });
    }

    // LTFT convergence: compare first half vs second half of warm log
    if let Some(ltfm) = get(log, LTFM1) {
        let warm_vals_ltfm: Vec<(usize, f64)> = ltfm.iter().enumerate()
            .filter(|&(i, _)| warm.get(i).copied().unwrap_or(false))
            .map(|(i, &v)| (i, v))
            .collect();
        if warm_vals_ltfm.len() > 40 {
            let mid = warm_vals_ltfm.len() / 2;
            let first_half: Vec<f64> = warm_vals_ltfm[..mid].iter().map(|&(_, v)| v).collect();
            let second_half: Vec<f64> = warm_vals_ltfm[mid..].iter().map(|&(_, v)| v).collect();
            let delta = (mean(&second_half) - mean(&first_half)).abs();
            let s = if delta < 0.5 { DiagStatus::Ok } else if delta < 1.5 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Convergence LTFT (1ère vs 2ème moitié)".into(),
                value: format!("Δ {:.2}%", delta),
                status: s,
                detail: if delta > 1.5 { "LTFT encore en adaptation — log insuffisant".into() }
                        else if delta > 0.5 { "Légère dérive — refaire un log plus long".into() }
                        else { "LTFT convergé".into() },
            });
        }
    }

    out
}

fn engine_checks(log: &Log, cfg: &Msv70AnalysisState, warm: &[bool], phases: &[Phase]) -> Vec<DiagCheck> {
    let mut out = Vec::new();
    let load_d = get(log, LOAD);
    let rpm_d  = get(log, RPM);
    let wot_m:  Vec<bool> = phases.iter().map(|&p| p == Phase::Wot).collect();
    let idle_m: Vec<bool> = phases.iter().map(|&p| p == Phase::Idle).collect();

    // MAF vs Load Pearson
    if let (Some(maf), Some(load)) = (get(log, MAF), &load_d) {
        let pairs: Vec<(f64,f64)> = maf.iter().zip(load.iter()).zip(warm.iter())
            .filter_map(|((&m, &l), &w)| if w && m > 0.1 && l > 0.1 { Some((m, l)) } else { None }).collect();
        let ma: Vec<f64> = pairs.iter().map(|p| p.0).collect();
        let la: Vec<f64> = pairs.iter().map(|p| p.1).collect();
        if let Some(r) = pearson(&ma, &la) {
            let s = if r > 0.88 { DiagStatus::Ok } else if r > 0.70 { DiagStatus::Warn } else { DiagStatus::Critical };
            let note = if r > 0.88 { "Débit air cohérent" }
                       else if r > 0.70 { "Vérifier clapets DISA / joints admission" }
                       else { "Vérifier débitmètre, DISA, joints culasse" };
            out.push(DiagCheck { label: "MAF ↔ Charge (Pearson r)".into(), value: format!("{:.4}", r), status: s, detail: note.into() });
        } else { out.push(DiagCheck::info("MAF ↔ Charge", "—", "Insuffisant")); }
    } else { out.push(DiagCheck::missing("Corrélation MAF/Charge")); }

    // WOT load
    if let Some(load) = &load_d {
        let wv: Vec<f64> = load.iter().zip(wot_m.iter()).filter_map(|(&l, &w)| if w { Some(l) } else { None }).collect();
        if !wv.is_empty() {
            let avg = mean(&wv); let mx = vmax(&wv);
            let s = if avg > 90.0 { DiagStatus::Ok } else if avg > 80.0 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Charge WOT (moyenne)".into(), value: format!("{:.1}%", avg), status: s, detail: format!("Max: {:.1}%", mx) });
        }
    }

    // Idle RPM stability
    if let Some(rpm) = &rpm_d {
        let iv: Vec<f64> = rpm.iter().zip(idle_m.iter()).filter_map(|(&r, &i)| if i && r > 200.0 { Some(r) } else { None }).collect();
        if iv.len() > 20 {
            let sd = stddev(&iv);
            let s = if sd < 50.0 { DiagStatus::Ok } else if sd < 120.0 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Stabilité ralenti (σ RPM)".into(), value: format!("{:.0} tr/min", sd),
                status: s, detail: format!("Moy. ralenti: {:.0} tr/min", mean(&iv)) });
        }
    }

    // Injector duty
    if let (Some(inj), Some(rpm)) = (get(log, INJ), &rpm_d) {
        let duties: Vec<f64> = inj.iter().zip(rpm.iter()).zip(warm.iter())
            .filter_map(|((&t, &r), &w)| if w && r > 100.0 { Some((t / (120_000.0 / r)) * 100.0) } else { None })
            .collect();
        if !duties.is_empty() {
            let dm = mean(&duties); let dmax = vmax(&duties);
            let s = if dmax < cfg.duty_warn { DiagStatus::Ok } else if dmax < cfg.duty_crit { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Rapport cyclique injecteur (max)".into(), value: format!("{:.1}%", dmax),
                status: s, detail: format!("Moy: {:.1}% — critique > {:.0}%", dm, cfg.duty_crit) });
        }
    } else { out.push(DiagCheck::missing("Rapport cyclique injecteur")); }

    // Idle RPM absolute value (not just stability — is it in spec?)
    if let Some(rpm) = &rpm_d {
        let iv: Vec<f64> = rpm.iter().zip(idle_m.iter()).zip(warm.iter())
            .filter_map(|((&r, &i), &w)| if i && w && r > 200.0 { Some(r) } else { None }).collect();
        if iv.len() > 20 {
            let m = mean(&iv);
            // MSV70 M54 target idle: 680–800 RPM (varies with A/C load)
            let s = if (650.0..=820.0).contains(&m) { DiagStatus::Ok }
                    else if (580.0..=900.0).contains(&m) { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "RPM ralenti (valeur)".into(),
                value: format!("{:.0} tr/min", m),
                status: s,
                detail: if m < 650.0 { "Trop bas — vérifier IACV / idle trim".into() }
                        else if m > 820.0 { "Trop haut — fuite admission ou IACV".into() }
                        else { "Dans la plage MSV70 (680–800)".into() },
            });
        }
    }

    // Knock — absolute count
    if let Some(knock) = get(log, KNOCK) {
        let events: Vec<f64> = knock.iter().zip(warm.iter())
            .filter_map(|(&k, &w)| if w && k.abs() > 0.5 { Some(k.abs()) } else { None }).collect();
        let ne = events.len(); let ar = if ne > 0 { mean(&events) } else { 0.0 };
        let s = if ne == 0 { DiagStatus::Ok } else if ar < 2.0 { DiagStatus::Warn } else { DiagStatus::Critical };
        out.push(DiagCheck { label: "Événements cliquetis".into(), value: format!("{}", ne), status: s,
            detail: if ne > 0 { format!("Retard moy: {:.1}°", ar) } else { "Aucun".into() } });

        // Knock density per 100 seconds of warm log
        let warm_dur: f64 = log.times.windows(2)
            .zip(warm.iter())
            .filter(|(_, &w)| w)
            .map(|(w, _)| w[1] - w[0])
            .sum();
        if warm_dur > 10.0 {
            let density = ne as f64 / warm_dur * 100.0;
            let s2 = if density < 0.5 { DiagStatus::Ok } else if density < 2.0 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Densité cliquetis (/100 s)".into(),
                value: format!("{:.2} evt/100s", density),
                status: s2,
                detail: format!("Sur {:.0}s de log chaud — < 0.5 = acceptable", warm_dur),
            });
        }
    } else { out.push(DiagCheck::missing("Retard cliquetis")); }

    // VANOS deviation
    if let (Some(act), Some(set)) = (get(log, VANOS_ACT), get(log, VANOS_SET)) {
        let n = act.len().min(set.len());
        let errs: Vec<f64> = act[..n].iter().zip(set[..n].iter()).zip(warm.iter())
            .filter_map(|((&a, &s), &w)| if w { Some((a - s).abs()) } else { None }).collect();
        if !errs.is_empty() {
            let em = mean(&errs); let emax = vmax(&errs);
            let s = if em < 2.0 { DiagStatus::Ok } else if em < 5.0 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "VANOS — erreur position".into(), value: format!("{:.2}°", em),
                status: s, detail: format!("Max: {:.2}° — usure si > 5°", emax) });
        }
    }

    // Overrun lambda (should drop or cut when DFCO)
    if let Some(lam) = get(log, LAMBDA1) {
        let ov_m: Vec<bool> = phases.iter().map(|&p| p == Phase::Overrun).collect();
        let ov: Vec<f64> = lam.iter().zip(ov_m.iter()).filter_map(|(&v, &w)| if w && v > 0.5 { Some(v) } else { None }).collect();
        if ov.len() > 5 {
            let lm = mean(&ov);
            let s = if lm > 1.0 { DiagStatus::Ok } else { DiagStatus::Warn };
            out.push(DiagCheck { label: "Lambda roue libre".into(), value: format!("{:.3} λ", lm),
                status: s, detail: "Doit être > 1.0 (appauvrissement roue libre)".into() });
        }
    }

    out
}

fn elec_checks(log: &Log, warm: &[bool]) -> Vec<DiagCheck> {
    let mut out = Vec::new();
    if let Some(batt) = get(log, BATTERY) {
        let wv = warm_vals(&batt, warm);
        if !wv.is_empty() {
            let bmin = vmin(&wv); let bmax = vmax(&wv); let bavg = mean(&wv);
            let low_n = wv.iter().filter(|&&v| v < 12.5).count();
            let s = if bmin > 13.0 { DiagStatus::Ok } else if bmin > 12.0 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck { label: "Tension batterie (min)".into(), value: format!("{:.2} V", bmin),
                status: s, detail: format!("Moy: {:.2} V  Max: {:.2} V", bavg, bmax) });
            let s2 = if low_n == 0 { DiagStatus::Ok } else { DiagStatus::Warn };
            out.push(DiagCheck { label: "Chutes < 12.5 V".into(), value: format!("{}", low_n),
                status: s2, detail: "Vérifier alternateur et cosses".into() });
        }
    } else { out.push(DiagCheck::missing("Tension batterie")); }
    out
}

fn temp_checks(log: &Log, cfg: &Msv70AnalysisState) -> Vec<DiagCheck> {
    let mut out = Vec::new();
    if let Some(cool) = get(log, COOLANT) {
        let cold_n = cool.iter().filter(|&&c| c < cfg.min_coolant).count();
        let cold_pct = cold_n as f64 / cool.len().max(1) as f64 * 100.0;
        let s = if cold_pct < 20.0 { DiagStatus::Ok } else if cold_pct < 40.0 { DiagStatus::Warn } else { DiagStatus::Info };
        out.push(DiagCheck { label: "Fraction à froid".into(), value: format!("{:.1}%", cold_pct),
            status: s, detail: format!("< {:.0}°C = {} pts", cfg.min_coolant, cold_n) });
        let mx = vmax(&cool);
        let s2 = if (85.0..=105.0).contains(&mx) { DiagStatus::Ok } else if mx > 110.0 { DiagStatus::Critical } else { DiagStatus::Warn };
        out.push(DiagCheck { label: "Temp. max liquide refroid.".into(), value: format!("{:.1}°C", mx),
            status: s2, detail: "Normal 85–105°C | > 110°C = surchauffe".into() });
    } else { out.push(DiagCheck::missing("Liquide de refroidissement")); }

    // Thermostat check: once warm, does coolant plateau in expected range?
    if let Some(cool) = get(log, COOLANT) {
        let warm_temps: Vec<f64> = cool.iter().filter(|&&c| c >= cfg.min_coolant).copied().collect();
        if warm_temps.len() > 20 {
            let operating_avg = mean(&warm_temps);
            let operating_max = vmax(&warm_temps);
            let s = if (83.0..=98.0).contains(&operating_avg) { DiagStatus::Ok }
                    else if operating_avg < 83.0 { DiagStatus::Warn }  // thermostat bloqué ouvert
                    else { DiagStatus::Critical };  // surchauffe
            out.push(DiagCheck {
                label: "Thermostat (temp. fonctionnement)".into(),
                value: format!("{:.1}°C moy / {:.1}°C max", operating_avg, operating_max),
                status: s,
                detail: if operating_avg < 83.0 {
                    "Thermostat peut-être bloqué ouvert (< 83°C)".into()
                } else if operating_avg > 98.0 {
                    "Surchauffe — vérifier pompe à eau / thermostat".into()
                } else {
                    "Plage normale MSV70 (83–98°C)".into()
                },
            });
        }
    }

    if let Some(oil) = get(log, OIL) {
        let mx = vmax(&oil); let mn = vmin(&oil);
        let s = if mx > 80.0 { DiagStatus::Ok } else if mx > 60.0 { DiagStatus::Warn } else { DiagStatus::Info };
        out.push(DiagCheck { label: "Temp. huile (max)".into(), value: format!("{:.1}°C", mx),
            status: s, detail: format!("Min: {:.1}°C — > 80°C attendu en conduite", mn) });
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// E85 compatibility diagnostic
// ─────────────────────────────────────────────────────────────────────────────

fn e85_checks(log: &Log, cfg: &Msv70AnalysisState, warm: &[bool], phases: &[Phase], times: &[f64]) -> Vec<DiagCheck> {
    let mut out = Vec::new();
    let rpm_d   = get(log, RPM);
    let inj_d   = get(log, INJ);
    let lam_d   = get(log, LAMBDA1);
    let knock_d = get(log, KNOCK);

    // ─── Lambda par phase (cibles E85) ───────────────────────────────────────

    if let Some(lam) = &lam_d {
        // WOT: cible configurable, défaut 0.82–0.90 pour E85
        let wot_v: Vec<f64> = lam.iter().zip(phases.iter())
            .filter_map(|(&x, &p)| if p == Phase::Wot && x > 0.5 { Some(x) } else { None })
            .collect();
        if wot_v.is_empty() {
            out.push(DiagCheck::info("Lambda WOT (E85)", "—", "Aucun échantillon WOT dans le log"));
        } else {
            let m = mean(&wot_v); let sd = stddev(&wot_v);
            let mn = vmin(&wot_v); let mx = vmax(&wot_v);
            let (lo, hi) = (cfg.wot_lambda_min_e85, cfg.wot_lambda_max_e85);
            let s = if (lo..=hi).contains(&m) { DiagStatus::Ok }
                    else if ((lo - 0.04).max(0.74)..=(hi + 0.06)).contains(&m) { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Lambda WOT (E85)".into(),
                value: format!("{:.3} λ", m),
                status: s,
                detail: format!("Cible E85 {:.2}–{:.2} | σ={:.4} | plage [{:.3}–{:.3}]", lo, hi, sd, mn, mx),
            });
        }

        // Ralenti: 1.00 ±0.02 (identique essence, mais on valide boucle fermée E85)
        let idle_v: Vec<f64> = lam.iter().zip(phases.iter()).zip(warm.iter())
            .filter_map(|((&x, &p), &w)| if p == Phase::Idle && w && x > 0.5 { Some(x) } else { None })
            .collect();
        if idle_v.len() > 5 {
            let m = mean(&idle_v); let sd = stddev(&idle_v);
            let s = if (0.98..=1.02).contains(&m) && sd < 0.02 { DiagStatus::Ok }
                    else if (0.95..=1.05).contains(&m) { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Lambda ralenti (E85)".into(),
                value: format!("{:.3} λ", m),
                status: s,
                detail: format!("σ={:.4} — boucle fermée stable E85? Cible 1.00 ±0.02", sd),
            });
        }

        // Croisière: stabilité σ (boucle fermée doit être stable avec E85)
        let cru_v: Vec<f64> = lam.iter().zip(phases.iter()).zip(warm.iter())
            .filter_map(|((&x, &p), &w)| if p == Phase::Cruise && w && x > 0.5 { Some(x) } else { None })
            .collect();
        if cru_v.len() > 10 {
            let m = mean(&cru_v); let sd = stddev(&cru_v);
            let s = if sd < 0.025 { DiagStatus::Ok } else if sd < 0.05 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Lambda croisière (E85, σ)".into(),
                value: format!("σ={:.4} / {:.3} λ", sd, m),
                status: s,
                detail: if sd > 0.05 { "Instabilité boucle fermée E85 — sonde O2 / pression carbu?".into() }
                        else if sd > 0.025 { "Légère oscillation — surveiller tendance".into() }
                        else { "Boucle fermée E85 stable en croisière".into() },
            });
        }

        // Accélération: doit être riche (< 1.0) — enrichissement transitoire E85
        let acc_v: Vec<f64> = lam.iter().zip(phases.iter())
            .filter_map(|(&x, &p)| if p == Phase::Acceleration && x > 0.5 { Some(x) } else { None })
            .collect();
        if acc_v.len() > 3 {
            let m = mean(&acc_v); let mn_a = vmin(&acc_v);
            let s = if m < 1.0 { DiagStatus::Ok } else if m < 1.05 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Lambda accélération (E85)".into(),
                value: format!("{:.3} λ", m),
                status: s,
                detail: format!("Min: {:.3} — enrichissement transitoire E85 doit être < 1.0", mn_a),
            });
        }

        // Décélération/DFCO: doit monter > 1.05 ou coupure
        let dec_v: Vec<f64> = lam.iter().zip(phases.iter())
            .filter_map(|(&x, &p)| if p == Phase::Deceleration && x > 0.5 { Some(x) } else { None })
            .collect();
        if dec_v.len() > 3 {
            let m = mean(&dec_v);
            let s = if m > 1.05 { DiagStatus::Ok } else { DiagStatus::Warn };
            out.push(DiagCheck {
                label: "Lambda DFCO (E85)".into(),
                value: format!("{:.3} λ", m),
                status: s,
                detail: "DFCO actif si λ > 1.05 — coupure carbu normale avec E85".into(),
            });
        }

        // Roue libre: doit également être maigre
        let ov_v: Vec<f64> = lam.iter().zip(phases.iter())
            .filter_map(|(&x, &p)| if p == Phase::Overrun && x > 0.5 { Some(x) } else { None })
            .collect();
        if ov_v.len() > 3 {
            let m = mean(&ov_v);
            let s = if m > 1.0 { DiagStatus::Ok } else { DiagStatus::Warn };
            out.push(DiagCheck {
                label: "Lambda roue libre (E85)".into(),
                value: format!("{:.3} λ", m),
                status: s,
                detail: "Appauvrissement roue libre attendu avec E85 — doit être > 1.0".into(),
            });
        }
    } else {
        out.push(DiagCheck::missing("Lambda (E85)"));
    }

    // ─── Cliquetis — tolérance zéro avec E85 ─────────────────────────────────
    // E85 a un indice d'octane ~105 — tout cliquetis est très suspect

    if let Some(knock) = &knock_d {
        let wot_knock: Vec<f64> = knock.iter().zip(phases.iter())
            .filter_map(|(&k, &p)| if p == Phase::Wot && k.abs() > 0.5 { Some(k.abs()) } else { None })
            .collect();
        let ne_wot = wot_knock.len();
        let s_wot = if ne_wot == 0 { DiagStatus::Ok }
                    else if ne_wot < 3 { DiagStatus::Warn }
                    else { DiagStatus::Critical };
        out.push(DiagCheck {
            label: "Cliquetis WOT (E85, tolérance 0)".into(),
            value: format!("{} événements", ne_wot),
            status: s_wot,
            detail: if ne_wot == 0 {
                "Aucun cliquetis WOT — E85 correct".into()
            } else {
                format!("E85 résiste normalement au cliquetis — avance trop agressive? | retard moy: {:.1}°",
                    if ne_wot > 0 { mean(&wot_knock) } else { 0.0 })
            },
        });

        let all_warm_knock: Vec<f64> = knock.iter().zip(warm.iter())
            .filter_map(|(&k, &w)| if w && k.abs() > 0.5 { Some(k.abs()) } else { None }).collect();
        let ne_all = all_warm_knock.len();
        let s_all = if ne_all == 0 { DiagStatus::Ok }
                    else if ne_all < 5 { DiagStatus::Warn }
                    else { DiagStatus::Critical };
        out.push(DiagCheck {
            label: "Cliquetis global (E85)".into(),
            value: format!("{} événements", ne_all),
            status: s_all,
            detail: if ne_all == 0 {
                "Zéro cliquetis — comportement E85 attendu avec octane élevé".into()
            } else {
                format!("Cliquetis avec E85 = suspect — capteur knock? avance? mélange? | moy retard: {:.1}°",
                    mean(&all_warm_knock))
            },
        });
    } else {
        out.push(DiagCheck::missing("Cliquetis (E85)"));
    }

    // ─── Rapport cyclique injecteur (E85 consomme ~30% plus de carburant) ────

    if let (Some(inj), Some(rpm)) = (&inj_d, &rpm_d) {
        let wot_duties: Vec<f64> = inj.iter().zip(rpm.iter()).zip(phases.iter())
            .filter_map(|((&t, &r), &p)| if p == Phase::Wot && r > 100.0 {
                Some((t / (120_000.0 / r)) * 100.0)
            } else { None })
            .collect();
        if !wot_duties.is_empty() {
            let dmax = vmax(&wot_duties); let dm = mean(&wot_duties);
            let s = if dmax < cfg.e85_duty_warn { DiagStatus::Ok }
                    else if dmax < cfg.e85_duty_crit { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Duty injecteur WOT (E85)".into(),
                value: format!("{:.1}% max / {:.1}% moy", dmax, dm),
                status: s,
                detail: if dmax > cfg.e85_duty_crit {
                    "Injecteurs proches saturation — risque mélange pauvre sous charge E85".into()
                } else if dmax > cfg.e85_duty_warn {
                    "Marge injecteurs réduite pour E85 — surveiller".into()
                } else {
                    "Injecteurs ont assez de marge pour enrichissement E85".into()
                },
            });
        }

        // Rapport duty WOT / Ralenti — E85 correctement enrichi = ratio élevé
        let idle_duties: Vec<f64> = inj.iter().zip(rpm.iter()).zip(phases.iter())
            .filter_map(|((&t, &r), &p)| if p == Phase::Idle && r > 200.0 {
                Some((t / (120_000.0 / r)) * 100.0)
            } else { None })
            .collect();
        if !idle_duties.is_empty() && !wot_duties.is_empty() {
            let idle_d = mean(&idle_duties); let wot_d = mean(&wot_duties);
            let ratio = if idle_d > 0.5 { wot_d / idle_d } else { 0.0 };
            let s = if ratio > 3.0 { DiagStatus::Ok } else if ratio > 1.5 { DiagStatus::Warn } else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Rapport duty WOT/Ralenti (E85)".into(),
                value: format!("{:.1}×", ratio),
                status: s,
                detail: format!("Ralenti {:.1}% | WOT {:.1}% — enrichissement E85 doit être visible au WOT", idle_d, wot_d),
            });
        }
    } else {
        out.push(DiagCheck::missing("Injecteur/RPM (E85)"));
    }

    // ─── STFT — dérive temporelle (régression linéaire) ──────────────────────
    // Si STFT dérive dans le temps → LTFT pas encore convergé → carto E85 à ajuster

    for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
        if let Some(stft) = get(log, names) {
            let pairs: Vec<(f64, f64)> = stft.iter().zip(times.iter()).zip(warm.iter())
                .filter_map(|((&v, &t), &w)| if w { Some((t, v)) } else { None }).collect();
            if pairs.len() > 30 {
                let xs: Vec<f64> = pairs.iter().map(|p| p.0).collect();
                let ys: Vec<f64> = pairs.iter().map(|p| p.1).collect();
                let slope_per_min = linear_slope(&xs, &ys) * 60.0;
                let s = if slope_per_min.abs() < 0.5 { DiagStatus::Ok }
                        else if slope_per_min.abs() < 2.0 { DiagStatus::Warn }
                        else { DiagStatus::Critical };
                out.push(DiagCheck {
                    label: format!("Dérive STFT B{} (tendance)", b),
                    value: format!("{:+.3}%/min", slope_per_min),
                    status: s,
                    detail: if slope_per_min.abs() < 0.5 {
                        format!("STFT B{} stable — adaptation E85 convergée", b)
                    } else if slope_per_min > 0.0 {
                        format!("STFT B{} monte — ECU cherche encore son trim (trop maigre?)", b)
                    } else {
                        format!("STFT B{} descend — excès carburant ou sonde O2 lente", b)
                    },
                });
            }
        }
    }

    // ─── Écart STFT B1 / B2 par phase ────────────────────────────────────────

    if let (Some(s1d), Some(s2d)) = (get(log, STFT1), get(log, STFT2)) {
        let mut max_delta = 0.0_f64;
        let mut worst_phase = Phase::Idle;
        let mut any_phase = false;
        for &phase in Phase::all() {
            let v1: Vec<f64> = s1d.iter().zip(phases.iter()).filter_map(|(&x, &p)| if p == phase { Some(x) } else { None }).collect();
            let v2: Vec<f64> = s2d.iter().zip(phases.iter()).filter_map(|(&x, &p)| if p == phase { Some(x) } else { None }).collect();
            if v1.len() >= 5 && v2.len() >= 5 {
                let delta = (mean(&v1) - mean(&v2)).abs();
                if delta > max_delta { max_delta = delta; worst_phase = phase; }
                any_phase = true;
            }
        }
        if any_phase {
            let s = if max_delta < 2.0 { DiagStatus::Ok }
                    else if max_delta < 4.0 { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Écart STFT B1/B2 (pire phase, E85)".into(),
                value: format!("{:.2}%", max_delta),
                status: s,
                detail: format!("Phase la plus déséquilibrée: {} — < 2% = correct | sonde/inj B1 vs B2?", worst_phase.label()),
            });
        }
    }

    // ─── LTFT add par phase — résumé E85 ─────────────────────────────────────

    for (b, names) in [(1usize, LTFA1), (2usize, LTFA2)] {
        if let Some(ltfa) = get(log, names) {
            let global: Vec<f64> = ltfa.iter().zip(warm.iter()).filter_map(|(&x, &w)| if w { Some(x) } else { None }).collect();
            if !global.is_empty() {
                let gm = mean(&global);
                // Trouver la phase avec le LTFT add le plus élevé
                let (worst_p, worst_v) = Phase::all().iter().map(|&p| {
                    let v: Vec<f64> = ltfa.iter().zip(phases.iter()).zip(warm.iter())
                        .filter_map(|((&x, &ph), &w)| if ph == p && w { Some(x) } else { None }).collect();
                    (p, if v.len() >= 5 { mean(&v) } else { 0.0 })
                }).max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((Phase::Idle, 0.0));
                let s = if gm.abs() < 3.0 { DiagStatus::Ok }
                        else if gm.abs() < 7.0 { DiagStatus::Warn }
                        else { DiagStatus::Critical };
                out.push(DiagCheck {
                    label: format!("LTFT add B{} (E85)", b),
                    value: format!("{:+.2}%", gm),
                    status: s,
                    detail: format!("Phase critique: {} = {:+.2}% | Cible < 3% — carto E85 à affiner si > 7%", worst_p.label(), worst_v),
                });
            }
        } else {
            out.push(DiagCheck::missing(&format!("LTFT add B{} (E85)", b)));
        }
    }

    // ─── LTFT mul par phase — correction globale E85 ─────────────────────────

    for (b, names) in [(1usize, LTFM1), (2usize, LTFM2)] {
        if let Some(ltfm) = get(log, names) {
            let global: Vec<f64> = ltfm.iter().zip(warm.iter()).filter_map(|(&x, &w)| if w { Some(x) } else { None }).collect();
            if !global.is_empty() {
                let gm = mean(&global);
                let s = if gm.abs() < 5.0 { DiagStatus::Ok }
                        else if gm.abs() < 10.0 { DiagStatus::Warn }
                        else { DiagStatus::Critical };
                let interp = if gm < -10.0 {
                    "LTFT mul très négatif — carto E85 trop riche ou sonde O2 HS"
                } else if gm > 10.0 {
                    "LTFT mul positif élevé — carto E85 trop maigre / manque carburant"
                } else if gm.abs() < 5.0 {
                    "LTFT mul convergé — mélange E85 bien cartographié"
                } else {
                    "Dérive modérée — carto E85 à ajuster légèrement"
                };
                out.push(DiagCheck {
                    label: format!("LTFT mul B{} (E85)", b),
                    value: format!("{:+.2}%", gm),
                    status: s,
                    detail: interp.into(),
                });
            }
        } else {
            out.push(DiagCheck::missing(&format!("LTFT mul B{} (E85)", b)));
        }
    }

    // ─── Écart LTFT mul B1 / B2 ──────────────────────────────────────────────

    if let (Some(m1d), Some(m2d)) = (get(log, LTFM1), get(log, LTFM2)) {
        let v1: Vec<f64> = m1d.iter().zip(warm.iter()).filter_map(|(&x, &w)| if w { Some(x) } else { None }).collect();
        let v2: Vec<f64> = m2d.iter().zip(warm.iter()).filter_map(|(&x, &w)| if w { Some(x) } else { None }).collect();
        if v1.len() >= 5 && v2.len() >= 5 {
            let delta = (mean(&v1) - mean(&v2)).abs();
            let s = if delta < 2.0 { DiagStatus::Ok }
                    else if delta < 5.0 { DiagStatus::Warn }
                    else { DiagStatus::Critical };
            out.push(DiagCheck {
                label: "Écart LTFT mul B1/B2 (E85)".into(),
                value: format!("{:.2}%", delta),
                status: s,
                detail: "Les banques doivent corriger pareillement — > 2% = déséquilibre injecteurs/sondes".into(),
            });
        }
    }

    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-phase deep diagnostic checks
// ─────────────────────────────────────────────────────────────────────────────

fn phase_checks_for(
    log: &Log,
    cfg: &Msv70AnalysisState,
    phase: Phase,
    warm: &[bool],
    phases: &[Phase],
    times: &[f64],
) -> Vec<DiagCheck> {
    let mut out = Vec::new();
    let mask: Vec<bool> = phases.iter().map(|&p| p == phase).collect();
    let n_spl = mask.iter().filter(|&&b| b).count();

    if n_spl < 5 {
        out.push(DiagCheck::info(
            "Données insuffisantes",
            &format!("{} points", n_spl),
            "Phase trop courte ou absente du log",
        ));
        return out;
    }

    let rpm_d  = get(log, RPM);
    let load_d = get(log, LOAD);
    let tps_d  = get(log, TPS);
    let lam_d  = get(log, LAMBDA1);
    let inj_d  = get(log, INJ);

    // Helper: filter a Vec<f64> by a boolean mask
    let filtered = |v: &[f64], m: &[bool]| -> Vec<f64> {
        v.iter().zip(m.iter()).filter_map(|(&x, &b)| if b { Some(x) } else { None }).collect()
    };

    match phase {
        Phase::Wot => {
            // Lambda WOT: cible 0.85-0.96 (enrichissement pleine charge)
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter())
                    .filter_map(|(&x, &b)| if b && x > 0.5 { Some(x) } else { None }).collect();
                if !v.is_empty() {
                    let m = mean(&v); let sd = stddev(&v); let mx = vmax(&v);
                    let s = if (0.85..=0.96).contains(&m) { DiagStatus::Ok }
                            else if (0.80..=1.0).contains(&m) { DiagStatus::Warn }
                            else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Lambda WOT".into(), value: format!("{:.3} λ", m),
                        status: s, detail: format!("Max: {:.3}  σ: {:.4}", mx, sd) });
                }
            } else { out.push(DiagCheck::missing("Lambda WOT")); }

            // Charge WOT
            if let Some(load) = &load_d {
                let v = filtered(load, &mask);
                if !v.is_empty() {
                    let m = mean(&v); let mn = vmin(&v);
                    let s = if m > 90.0 { DiagStatus::Ok } else if m > 80.0 { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Charge WOT".into(), value: format!("{:.1}%", m),
                        status: s, detail: format!("Min WOT: {:.1}% — aspiration OK si > 90%", mn) });
                }
            }

            // Cliquetis sous WOT (le plus critique)
            if let Some(knock) = get(log, KNOCK) {
                let v: Vec<f64> = knock.iter().zip(mask.iter())
                    .filter_map(|(&k, &b)| if b && k.abs() > 0.5 { Some(k.abs()) } else { None }).collect();
                let ne = v.len(); let ar = if ne > 0 { mean(&v) } else { 0.0 };
                let s = if ne == 0 { DiagStatus::Ok } else if ar < 2.0 { DiagStatus::Warn } else { DiagStatus::Critical };
                out.push(DiagCheck { label: "Cliquetis WOT".into(), value: format!("{} événements", ne),
                    status: s, detail: if ne > 0 { format!("Retard moy: {:.1}° — danger moteur!", ar) } else { "Aucun cliquetis".into() } });
            }

            // Rapport cyclique WOT
            if let (Some(inj), Some(rpm)) = (&inj_d, &rpm_d) {
                let duties: Vec<f64> = inj.iter().zip(rpm.iter()).zip(mask.iter())
                    .filter_map(|((&t, &r), &b)| if b && r > 100.0 { Some((t / (120_000.0 / r)) * 100.0) } else { None })
                    .collect();
                if !duties.is_empty() {
                    let dmax = vmax(&duties); let dm = mean(&duties);
                    let s = if dmax < cfg.duty_warn { DiagStatus::Ok } else if dmax < cfg.duty_crit { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Duty injecteur WOT (max)".into(), value: format!("{:.1}%", dmax),
                        status: s, detail: format!("Moy: {:.1}% — risque si > {:.0}%", dm, cfg.duty_crit) });
                }
            }

            // STFT WOT (doit être proche de 0 — correcteur stable)
            for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
                if let Some(d) = get(log, names) {
                    let v: Vec<f64> = d.iter().zip(mask.iter())
                        .filter_map(|(&x, &msk)| if msk { Some(x) } else { None }).collect();
                    if v.len() > 3 {
                        let m = mean(&v); let mx = maxabs(&v);
                        let s = if m.abs() < 3.0 { DiagStatus::Ok } else if m.abs() < 7.0 { DiagStatus::Warn } else { DiagStatus::Critical };
                        out.push(DiagCheck { label: format!("STFT B{} sous WOT", b), value: format!("{:+.2}%", m),
                            status: s, detail: format!("Max |val|: {:.2}%", mx) });
                    }
                }
            }
        }

        Phase::Idle => {
            // Stabilité RPM ralenti
            if let Some(rpm) = &rpm_d {
                let v: Vec<f64> = rpm.iter().zip(mask.iter())
                    .filter_map(|(&r, &b)| if b && r > 200.0 { Some(r) } else { None }).collect();
                if v.len() > 10 {
                    let sd = stddev(&v); let m = mean(&v);
                    let s = if sd < 40.0 { DiagStatus::Ok } else if sd < 100.0 { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Stabilité RPM ralenti (σ)".into(), value: format!("{:.0} tr/min", sd),
                        status: s, detail: format!("Moy: {:.0} tr/min — < 40 σ = excellent", m) });
                }
            }

            // Lambda au ralenti (doit être ~1.00 ±0.02)
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter())
                    .filter_map(|(&x, &b)| if b && x > 0.5 { Some(x) } else { None }).collect();
                if v.len() > 5 {
                    let m = mean(&v); let sd = stddev(&v);
                    let s = if (0.98..=1.02).contains(&m) && sd < 0.02 { DiagStatus::Ok }
                            else if (0.96..=1.04).contains(&m) { DiagStatus::Warn }
                            else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Lambda ralenti".into(), value: format!("{:.3} λ", m),
                        status: s, detail: format!("σ = {:.4} — cible 1.00 ±0.02", sd) });
                }
            }

            // Variabilité STFT ralenti (stabilité boucle fermée)
            for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
                if let Some(d) = get(log, names) {
                    let v: Vec<f64> = d.iter().zip(mask.iter())
                        .filter_map(|(&x, &msk)| if msk { Some(x) } else { None }).collect();
                    if v.len() > 10 {
                        let m = mean(&v); let sd = stddev(&v);
                        let s = if m.abs() < 3.0 && sd < 2.0 { DiagStatus::Ok }
                                else if m.abs() < 7.0 { DiagStatus::Warn }
                                else { DiagStatus::Critical };
                        out.push(DiagCheck { label: format!("STFT B{} ralenti", b), value: format!("{:+.2}% σ={:.2}", m, sd),
                            status: s, detail: "Chasse si σ > 2 — vérif. sonde/préchauffe".into() });
                    }
                }
            }

            // Charge ralenti (doit être faible)
            if let Some(load) = &load_d {
                let v = filtered(load, &mask);
                if !v.is_empty() {
                    let m = mean(&v);
                    let s = if m < cfg.idle_load_thresh { DiagStatus::Ok }
                            else if m < cfg.idle_load_thresh * 1.5 { DiagStatus::Warn }
                            else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Charge au ralenti".into(), value: format!("{:.1}%", m),
                        status: s, detail: format!("Seuil: {:.0}% — fuites admission si élevé", cfg.idle_load_thresh) });
                }
            }
        }

        Phase::Cruise => {
            // Lambda croisière — stabilité σ
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter()).zip(warm.iter())
                    .filter_map(|((&x, &b), &w)| if b && w && x > 0.5 { Some(x) } else { None }).collect();
                if v.len() > 10 {
                    let m = mean(&v); let sd = stddev(&v);
                    let s = if sd < 0.02 { DiagStatus::Ok } else if sd < 0.05 { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Lambda croisière (σ)".into(), value: format!("{:.4}", sd),
                        status: s, detail: format!("Moy: {:.3} — chasse si σ > 0.05", m) });
                }
            }

            // Corrélation MAF-Charge en croisière
            if let (Some(maf), Some(load)) = (get(log, MAF), &load_d) {
                let pairs: Vec<(f64,f64)> = maf.iter().zip(load.iter()).zip(mask.iter()).zip(warm.iter())
                    .filter_map(|(((&m, &l), &b), &w)| if b && w && m > 0.1 { Some((m, l)) } else { None }).collect();
                let ma: Vec<f64> = pairs.iter().map(|p| p.0).collect();
                let la: Vec<f64> = pairs.iter().map(|p| p.1).collect();
                if let Some(r) = pearson(&ma, &la) {
                    let s = if r > 0.88 { DiagStatus::Ok } else if r > 0.70 { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "MAF ↔ Charge (croisière r)".into(), value: format!("{:.4}", r),
                        status: s, detail: if r < 0.70 { "Fuite air / débitmètre suspect".into() } else { "Cohérence MAF-Charge OK".into() } });
                }
            }

            // STFT croisière
            for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
                if let Some(d) = get(log, names) {
                    let v: Vec<f64> = d.iter().zip(mask.iter()).zip(warm.iter())
                        .filter_map(|((&x, &msk), &w)| if msk && w { Some(x) } else { None }).collect();
                    if v.len() > 5 {
                        let m = mean(&v); let mx = maxabs(&v);
                        let s = if m.abs() < cfg.ltft_warn { DiagStatus::Ok }
                                else if m.abs() < cfg.ltft_crit { DiagStatus::Warn }
                                else { DiagStatus::Critical };
                        out.push(DiagCheck { label: format!("STFT B{} croisière", b), value: format!("{:+.2}%", m),
                            status: s, detail: format!("Max |val|: {:.2}%", mx) });
                    }
                }
            }

            // RPM croisière stabilité
            if let Some(rpm) = &rpm_d {
                let v = filtered(rpm, &mask);
                if !v.is_empty() {
                    let m = mean(&v); let sd = stddev(&v);
                    let s = if sd < 100.0 { DiagStatus::Ok } else { DiagStatus::Info };
                    out.push(DiagCheck { label: "Régime croisière (σ)".into(), value: format!("{:.0} tr/min", sd),
                        status: s, detail: format!("Moy: {:.0} tr/min", m) });
                }
            }
        }

        Phase::Deceleration => {
            // Lambda DFCO — doit monter (> 1.05) ou coupure carbu
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter())
                    .filter_map(|(&x, &b)| if b && x > 0.5 { Some(x) } else { None }).collect();
                if v.len() > 3 {
                    let m = mean(&v); let mn = vmin(&v);
                    let s = if m > 1.05 { DiagStatus::Ok } else { DiagStatus::Warn };
                    out.push(DiagCheck { label: "Lambda DFCO".into(), value: format!("{:.3} λ", m),
                        status: s, detail: format!("Min: {:.3} — appauvrissement attendu", mn) });
                }
            }

            // Durée DFCO — nombre de segments et durée totale
            let dfco_segs: Vec<&PhaseSegment> = {
                // Recompute from times
                let mut segs = Vec::new();
                let mut in_phase = false;
                let mut seg_start = 0.0_f64;
                for (i, (&b, &t)) in mask.iter().zip(times.iter()).enumerate() {
                    if b && !in_phase { in_phase = true; seg_start = t; }
                    else if !b && in_phase {
                        segs.push((seg_start, times.get(i.saturating_sub(1)).copied().unwrap_or(t)));
                        in_phase = false;
                    }
                }
                // Return durations
                let durs: Vec<f64> = segs.iter().map(|(s, e)| e - s).collect();
                let n_seg = durs.len();
                let total_dur: f64 = durs.iter().sum();
                let mean_dur = if n_seg > 0 { total_dur / n_seg as f64 } else { 0.0 };
                let s = if n_seg > 0 { DiagStatus::Ok } else { DiagStatus::Info };
                out.push(DiagCheck { label: "Durée DFCO".into(),
                    value: format!("{:.1} s total", total_dur),
                    status: s,
                    detail: format!("{} événements, moy: {:.1} s", n_seg, mean_dur) });
                Vec::new()
            };
            let _ = dfco_segs;

            // RPM en DFCO (doit tomber progressivement)
            if let Some(rpm) = &rpm_d {
                let v = filtered(rpm, &mask);
                if v.len() > 3 {
                    let m = mean(&v); let mn = vmin(&v); let mx = vmax(&v);
                    out.push(DiagCheck { label: "RPM décélération".into(), value: format!("{:.0} tr/min moy", m),
                        status: DiagStatus::Info, detail: format!("Range: {:.0}–{:.0} tr/min", mn, mx) });
                }
            }

            // TPS en DFCO (doit être < 3%)
            if let Some(tps) = &tps_d {
                let v = filtered(tps, &mask);
                if !v.is_empty() {
                    let m = mean(&v); let mx = vmax(&v);
                    let s = if mx < 3.0 { DiagStatus::Ok } else { DiagStatus::Warn };
                    out.push(DiagCheck { label: "TPS décélération (max)".into(), value: format!("{:.1}%", mx),
                        status: s, detail: format!("Moy: {:.1}% — DFCO actif si TPS < 3%", m) });
                }
            }
        }

        Phase::Acceleration => {
            // Lambda enrichissement (doit être < 1.0 = riche)
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter())
                    .filter_map(|(&x, &b)| if b && x > 0.5 { Some(x) } else { None }).collect();
                if !v.is_empty() {
                    let m = mean(&v); let mn = vmin(&v);
                    let s = if m < 1.0 { DiagStatus::Ok } else if m < 1.05 { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Lambda accélération".into(), value: format!("{:.3} λ", m),
                        status: s, detail: format!("Min: {:.3} — enrichissement transitoire attendu", mn) });
                }
            }

            // STFT en accélération
            for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
                if let Some(d) = get(log, names) {
                    let v: Vec<f64> = d.iter().zip(mask.iter())
                        .filter_map(|(&x, &msk)| if msk { Some(x) } else { None }).collect();
                    if v.len() > 3 {
                        let m = mean(&v); let mx = maxabs(&v);
                        let s = if mx < 10.0 { DiagStatus::Ok } else { DiagStatus::Warn };
                        out.push(DiagCheck { label: format!("STFT B{} accélération", b), value: format!("{:+.2}%", m),
                            status: s, detail: format!("Max |val|: {:.2}%", mx) });
                    }
                }
            }

            // Montée en charge (variation load/TPS)
            if let (Some(load), Some(tps)) = (&load_d, &tps_d) {
                let lv = filtered(load, &mask);
                let tv = filtered(tps, &mask);
                if !lv.is_empty() && !tv.is_empty() {
                    let l_rise = vmax(&lv) - vmin(&lv);
                    let t_rise = vmax(&tv) - vmin(&tv);
                    let ratio = if t_rise > 1.0 { l_rise / t_rise } else { 0.0 };
                    let s = if ratio > 0.8 { DiagStatus::Ok } else if ratio > 0.5 { DiagStatus::Warn } else { DiagStatus::Critical };
                    out.push(DiagCheck { label: "Réponse charge/TPS".into(), value: format!("{:.2}", ratio),
                        status: s, detail: format!("ΔCharge: {:.1}%  ΔTPS: {:.1}% — papillon = charge?", l_rise, t_rise) });
                }
            }
        }

        Phase::Overrun => {
            // Lambda roue libre (doit être maigre > 1.0)
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter())
                    .filter_map(|(&x, &b)| if b && x > 0.5 { Some(x) } else { None }).collect();
                if v.len() > 3 {
                    let m = mean(&v);
                    let s = if m > 1.0 { DiagStatus::Ok } else { DiagStatus::Warn };
                    out.push(DiagCheck { label: "Lambda roue libre".into(), value: format!("{:.3} λ", m),
                        status: s, detail: "Doit être > 1.0 (appauvrissement)".into() });
                }
            }

            // Stabilité RPM roue libre
            if let Some(rpm) = &rpm_d {
                let v = filtered(rpm, &mask);
                if v.len() > 3 {
                    let m = mean(&v); let sd = stddev(&v);
                    out.push(DiagCheck { label: "RPM roue libre (σ)".into(), value: format!("{:.0} tr/min", sd),
                        status: DiagStatus::Info, detail: format!("Moy: {:.0} tr/min", m) });
                }
            }
        }

        Phase::WarmUp => {
            // Enrichissement démarrage à froid (lambda < 1.0 attendu)
            if let Some(lam) = &lam_d {
                let v: Vec<f64> = lam.iter().zip(mask.iter())
                    .filter_map(|(&x, &b)| if b && x > 0.3 { Some(x) } else { None }).collect();
                if !v.is_empty() {
                    let m = mean(&v); let mn = vmin(&v);
                    let s = if m < 0.98 { DiagStatus::Ok } else if m < 1.05 { DiagStatus::Info } else { DiagStatus::Warn };
                    out.push(DiagCheck { label: "Lambda chauffe (moy)".into(), value: format!("{:.3} λ", m),
                        status: s, detail: format!("Min: {:.3} — enrichissement à froid normal si < 1.0", mn) });
                }
            }

            // Montée en température (taux °C/s)
            if let Some(cool) = get(log, COOLANT) {
                let pairs: Vec<(f64,f64)> = cool.iter().zip(mask.iter()).zip(times.iter())
                    .filter_map(|((&c, &b), &t)| if b { Some((t, c)) } else { None }).collect();
                if pairs.len() > 2 {
                    let dt = pairs.last().unwrap().0 - pairs.first().unwrap().0;
                    let dc = pairs.last().unwrap().1 - pairs.first().unwrap().1;
                    let rate = if dt > 0.0 { dc / dt } else { 0.0 };
                    let s = if rate > 0.0 { DiagStatus::Ok } else { DiagStatus::Warn };
                    out.push(DiagCheck { label: "Taux chauffe (°C/s)".into(), value: format!("{:.3}", rate),
                        status: s, detail: format!("De {:.0}°C à {:.0}°C en {:.0}s",
                            pairs.first().unwrap().1, pairs.last().unwrap().1, dt) });
                }
            }

            // STFT à froid (peut être élevé par correction)
            for (b, names) in [(1usize, STFT1), (2usize, STFT2)] {
                if let Some(d) = get(log, names) {
                    let v: Vec<f64> = d.iter().zip(mask.iter())
                        .filter_map(|(&x, &msk)| if msk { Some(x) } else { None }).collect();
                    if v.len() > 3 {
                        let m = mean(&v);
                        out.push(DiagCheck { label: format!("STFT B{} chauffe", b), value: format!("{:+.2}%", m),
                            status: DiagStatus::Info, detail: "Normal si négatif à froid (enrichissement ECU)".into() });
                    }
                }
            }
        }
    }

    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Run analysis entry point
// ─────────────────────────────────────────────────────────────────────────────

pub fn run_analysis(log: &Log, st: &mut Msv70AnalysisState) {
    let n = log.times.len();
    let phase_vec = classify_phases(log, st);
    let mut counts = [0usize; 7];
    for &p in &phase_vec { counts[p as usize] += 1; }
    st.phase_counts  = counts;
    st.total_samples = n.saturating_sub(1);
    st.phase_segments = to_segments(&phase_vec, &log.times);

    let warm: Vec<bool> = if let Some(c) = get(log, COOLANT) {
        c.iter().map(|&v| v >= st.min_coolant).collect()
    } else { vec![true; n] };

    st.checks_fuel   = fuel_checks(log, st, &warm, &phase_vec);
    st.checks_engine = engine_checks(log, st, &warm, &phase_vec);
    st.checks_elec   = elec_checks(log, &warm);
    st.checks_temp   = temp_checks(log, st);

    // Per-phase deep diagnostics
    st.checks_by_phase = Phase::all()
        .iter()
        .map(|&p| phase_checks_for(log, st, p, &warm, &phase_vec, &log.times))
        .collect();

    // Per-phase bank comparison: mean of STFT/LTFT add/mul for B1 and B2
    let mut bank_cmp = vec![[f64::NAN; 6]; 7];
    if let (Some(s1), Some(s2), Some(a1), Some(a2), Some(m1), Some(m2)) = (
        get(log, STFT1), get(log, STFT2),
        get(log, LTFA1), get(log, LTFA2),
        get(log, LTFM1), get(log, LTFM2),
    ) {
        let len = s1.len().min(s2.len()).min(a1.len()).min(a2.len()).min(m1.len()).min(m2.len());
        for (p_idx, _) in Phase::all().iter().enumerate() {
            let mut sums = [0f64; 6];
            let mut count = 0usize;
            for i in 0..len {
                if phase_vec.get(i).copied().unwrap_or(Phase::Idle) as usize == p_idx {
                    sums[0] += s1[i]; sums[1] += s2[i];
                    sums[2] += a1[i]; sums[3] += a2[i];
                    sums[4] += m1[i]; sums[5] += m2[i];
                    count += 1;
                }
            }
            if count >= 5 {
                for v in &mut sums { *v /= count as f64; }
                bank_cmp[p_idx] = sums;
            }
        }
    }
    st.bank_compare_by_phase = bank_cmp;

    st.checks_e85 = e85_checks(log, st, &warm, &phase_vec, &log.times);
}

// ─────────────────────────────────────────────────────────────────────────────
// Rendering helpers
// ─────────────────────────────────────────────────────────────────────────────

fn badge(ui: &mut egui::Ui, status: DiagStatus) {
    egui::Frame::NONE.fill(status.color()).corner_radius(4)
        .inner_margin(egui::vec2(6.0, 2.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(status.label()).color(egui::Color32::WHITE).size(11.0).strong());
        });
}

#[allow(dead_code)]
fn check_row(ui: &mut egui::Ui, c: &DiagCheck) {
    ui.horizontal(|ui| {
        ui.add_sized([230.0, 18.0], egui::Label::new(
            egui::RichText::new(&c.label).size(12.0).color(egui::Color32::LIGHT_GRAY)));
        ui.add_sized([90.0, 18.0], egui::Label::new(
            egui::RichText::new(&c.value).monospace().size(12.0)));
        badge(ui, c.status);
        if !c.detail.is_empty() {
            ui.label(egui::RichText::new(&c.detail).size(11.0).color(egui::Color32::GRAY).italics());
        }
    });
}

/// Compact 2-line card with coloured left-border status indicator
fn check_row_compact(ui: &mut egui::Ui, c: &DiagCheck) {
    if c.status == DiagStatus::Missing { return; } // missing cards handled separately

    let border_col = c.status.color();
    let bg = match c.status {
        DiagStatus::Critical => egui::Color32::from_rgb(38, 24, 24),
        DiagStatus::Warn     => egui::Color32::from_rgb(36, 32, 20),
        _                    => egui::Color32::from_rgb(26, 28, 32),
    };

    ui.horizontal(|ui| {
        // Left accent bar
        let (r, _) = ui.allocate_exact_size(egui::vec2(3.0, 40.0), egui::Sense::hover());
        ui.painter().rect_filled(r, 1.5, border_col);
        ui.add_space(2.0);

        egui::Frame::NONE
            .fill(bg)
            .corner_radius(egui::CornerRadius { ne: 4, se: 4, nw: 0, sw: 0 })
            .inner_margin(egui::vec2(6.0, 4.0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(&c.label).size(11.0).color(egui::Color32::LIGHT_GRAY),
                    ).wrap());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        badge(ui, c.status);
                    });
                });
                ui.horizontal(|ui| {
                    let val_col = match c.status {
                        DiagStatus::Critical => egui::Color32::from_rgb(240, 110, 90),
                        DiagStatus::Warn     => egui::Color32::from_rgb(230, 185, 70),
                        _                    => egui::Color32::WHITE,
                    };
                    ui.label(egui::RichText::new(&c.value).monospace().size(11.5).strong().color(val_col));
                    if !c.detail.is_empty() {
                        ui.label(
                            egui::RichText::new(format!("· {}", &c.detail))
                                .size(9.5)
                                .color(egui::Color32::from_rgb(110, 110, 110))
                                .italics(),
                        );
                    }
                });
            });
    });
    ui.add_space(3.0);
}

/// Draw OK/WARN/CRIT counters as small pills — call at top of each tab
fn status_counters(ui: &mut egui::Ui, checks: &[DiagCheck]) {
    let ok   = checks.iter().filter(|c| c.status == DiagStatus::Ok).count();
    let warn = checks.iter().filter(|c| c.status == DiagStatus::Warn).count();
    let crit = checks.iter().filter(|c| c.status == DiagStatus::Critical).count();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for (n, col, lbl) in [
            (ok,   egui::Color32::from_rgb(50, 170, 90),  "✓"),
            (warn, egui::Color32::from_rgb(210, 160, 40), "⚠"),
            (crit, egui::Color32::from_rgb(210, 60, 50),  "✗"),
        ] {
            if n == 0 { continue; }
            egui::Frame::NONE.fill(col.gamma_multiply(0.25)).corner_radius(10)
                .inner_margin(egui::vec2(6.0, 2.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(format!("{lbl} {n}")).size(10.0).color(col).strong());
                });
        }
    });
    ui.add_space(4.0);
}

/// Render a summary line for missing channels instead of one card per channel
fn missing_channels_summary(ui: &mut egui::Ui, checks: &[DiagCheck]) {
    let missing: Vec<&str> = checks.iter()
        .filter(|c| c.status == DiagStatus::Missing)
        .map(|c| c.label.as_str())
        .collect();
    if missing.is_empty() { return; }
    egui::Frame::NONE
        .fill(egui::Color32::from_rgb(30, 28, 22))
        .corner_radius(4)
        .inner_margin(egui::vec2(7.0, 5.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new(
                format!("⚠ {} canal(aux) absent(s) du log", missing.len())
            ).size(10.5).color(egui::Color32::from_rgb(180, 150, 60)).strong());
            ui.label(egui::RichText::new(missing.join("  ·  "))
                .size(9.5).color(egui::Color32::GRAY).italics());
        });
    ui.add_space(4.0);
}

fn phase_bar(ui: &mut egui::Ui, phase: Phase, count: usize, total: usize) {
    let pct = if total > 0 { count as f64 / total as f64 * 100.0 } else { 0.0 };
    ui.horizontal(|ui| {
        ui.add_sized([150.0, 16.0], egui::Label::new(egui::RichText::new(phase.label()).size(12.0)));
        let full = 220.0_f32;
        let fill = (full * pct as f32 / 100.0).max(if count > 0 { 3.0 } else { 0.0 });
        let (r, _) = ui.allocate_exact_size(egui::vec2(fill, 14.0), egui::Sense::hover());
        ui.painter().rect_filled(r, 3.0, phase.bar_color());
        let (r2, _) = ui.allocate_exact_size(egui::vec2(full - fill, 14.0), egui::Sense::hover());
        ui.painter().rect_filled(r2, 0.0, egui::Color32::from_rgb(42, 42, 42));
        ui.label(egui::RichText::new(format!("{:.1}%  ({} pts)", pct, count)).size(12.0).color(egui::Color32::GRAY));
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// Inline side panel (5th activity bar icon)
// ─────────────────────────────────────────────────────────────────────────────

impl UltraLogApp {
    /// Render MSV70 diagnostic as inline side panel content (called from side_panel.rs routing)
    pub fn render_msv70_panel_content(&mut self, ui: &mut egui::Ui) {
        let has_file = self.selected_file.is_some() && !self.files.is_empty();

        // ── Action bar ────────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            let run = ui.add_enabled(
                has_file,
                egui::Button::new(egui::RichText::new("▶  Analyser").strong().size(12.0))
                    .fill(egui::Color32::from_rgb(70, 105, 50)),
            );
            if run.clicked() {
                if let Some(idx) = self.selected_file {
                    if let Some(file) = self.files.get(idx) {
                        run_analysis(&file.log.clone(), &mut self.msv70_analysis);
                    }
                }
            }

            let job_btn = egui::Frame::NONE
                .fill(egui::Color32::from_rgb(45, 65, 95))
                .corner_radius(4)
                .inner_margin(egui::vec2(8.0, 4.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("⬇ Job Testo").color(egui::Color32::WHITE).size(11.0));
                });
            if job_btn.response.interact(egui::Sense::click()).clicked() {
                self.generate_testo_diagnostic_job();
            }
            if job_btn.response.hovered() {
                job_btn.response.on_hover_text(
                    "Générer customjobs.xml avec tous les canaux requis pour l'analyse MSV70 complète"
                );
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            let ov = self.msv70_analysis.show_phase_overlay;
            let ov_fill = if ov {
                egui::Color32::from_rgb(35, 75, 50)
            } else {
                egui::Color32::from_rgb(45, 48, 54)
            };
            let ov_btn = egui::Frame::NONE
                .fill(ov_fill)
                .corner_radius(4)
                .inner_margin(egui::vec2(8.0, 4.0))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(if ov { "🎨 ON" } else { "🎨" })
                            .color(egui::Color32::WHITE)
                            .size(11.0),
                    );
                });
            if ov_btn.response.interact(egui::Sense::click()).clicked() {
                self.msv70_analysis.show_phase_overlay = !self.msv70_analysis.show_phase_overlay;
            }
            if ov_btn.response.hovered() {
                ov_btn.response.on_hover_text("Activer l'overlay de phases sur le graphique");
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        });

        if !has_file {
            ui.add_space(8.0);
            ui.colored_label(egui::Color32::GRAY, "Charger un log Testo CSV d'abord.");
            return;
        }

        let analyzed = !self.msv70_analysis.checks_fuel.is_empty();

        // ── Score banner ──────────────────────────────────────────────────────
        if analyzed {
            let (score, n_checks) = self.msv70_analysis.health_score();
            let sc = if score >= 80 {
                egui::Color32::from_rgb(50, 190, 100)
            } else if score >= 55 {
                egui::Color32::from_rgb(230, 160, 30)
            } else {
                egui::Color32::from_rgb(210, 55, 55)
            };
            let verd = if score >= 80 { "Carto validée" }
                       else if score >= 55 { "Anomalies mineures" }
                       else { "Problèmes critiques" };
            ui.add_space(6.0);
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(26, 28, 32))
                .corner_radius(6)
                .inner_margin(egui::vec2(10.0, 7.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{}%", score)).size(22.0).strong().color(sc));
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(verd).size(11.5).color(sc));
                            ui.label(egui::RichText::new(format!("{} contrôles", n_checks)).size(10.0).color(egui::Color32::GRAY));
                        });
                    });
                });
        }

        ui.add_space(6.0);
        ui.separator();

        // ── Tab bar — icon-only to fit 280px ──────────────────────────────────
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            for (i, (icon, tip)) in [
                ("📊", "Dashboard"),
                ("🔄", "Phases"),
                ("⛽", "Carburant"),
                ("⚙", "Moteur"),
                ("🎯", "Par Phase"),
                ("🌽", "E85"),
                ("🔧", "Seuils"),
            ]
            .iter()
            .enumerate()
            {
                let sel = self.msv70_analysis.active_tab == i;
                let fill = if sel {
                    egui::Color32::from_rgb(50, 55, 62)
                } else {
                    egui::Color32::TRANSPARENT
                };
                let tcol = if sel {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::from_rgb(135, 135, 135)
                };
                let fr = egui::Frame::NONE
                    .fill(fill)
                    .corner_radius(4)
                    .inner_margin(egui::vec2(5.0, 3.0))
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(*icon).size(14.0).color(tcol));
                    });
                let resp = fr.response.interact(egui::Sense::click());
                if resp.clicked() { self.msv70_analysis.active_tab = i; }
                if resp.hovered() {
                    resp.clone().on_hover_text(*tip);
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            }
        });
        ui.separator();
        ui.add_space(4.0);

        if !analyzed {
            ui.colored_label(egui::Color32::GRAY, "Appuyer sur ▶ Analyser.");
            return;
        }

        // No outer ScrollArea here — side_panel.rs already wraps in one
        ui.set_width(ui.available_width());
        match self.msv70_analysis.active_tab {
            0 => self.tab_dashboard_compact(ui),
            1 => self.tab_phases(ui),
            2 => self.tab_fuel_compact(ui),
            3 => self.tab_engine_compact(ui),
            4 => self.tab_per_phase(ui),
            5 => self.tab_e85(ui),
            6 => self.tab_settings(ui),
            _ => {}
        }
    }

    // ── Dashboard (compact 280px version) ────────────────────────────────────
    fn tab_dashboard_compact(&mut self, ui: &mut egui::Ui) {
        let all: Vec<DiagCheck> = self.msv70_analysis.checks_fuel.iter()
            .chain(&self.msv70_analysis.checks_engine)
            .chain(&self.msv70_analysis.checks_elec)
            .chain(&self.msv70_analysis.checks_temp)
            .cloned().collect();

        let n_ok   = all.iter().filter(|c| c.status == DiagStatus::Ok).count();
        let n_warn = all.iter().filter(|c| c.status == DiagStatus::Warn).count();
        let n_crit = all.iter().filter(|c| c.status == DiagStatus::Critical).count();

        // Summary tiles — horizontal strip
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            for (label, n, color) in [
                ("✔", n_ok,   egui::Color32::from_rgb(50, 190, 100)),
                ("⚠", n_warn, egui::Color32::from_rgb(230, 160, 30)),
                ("✖", n_crit, egui::Color32::from_rgb(210, 55, 55)),
            ] {
                egui::Frame::NONE.fill(egui::Color32::from_rgb(32, 34, 38))
                    .corner_radius(6).inner_margin(egui::vec2(10.0, 6.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("{}", n)).size(20.0).strong().color(color));
                            ui.add_space(3.0);
                            ui.label(egui::RichText::new(label).size(13.0).color(color));
                        });
                    });
            }
        });
        ui.add_space(8.0);

        // Critical items
        let critical: Vec<DiagCheck> = all.iter().filter(|c| c.status == DiagStatus::Critical).cloned().collect();
        let warn: Vec<DiagCheck>     = all.iter().filter(|c| c.status == DiagStatus::Warn).cloned().collect();

        if !critical.is_empty() {
            ui.label(egui::RichText::new("🚨 Critiques").size(11.5).strong().color(egui::Color32::from_rgb(210, 55, 55)));
            ui.add_space(3.0);
            for c in &critical { check_row_compact(ui, c); }
            ui.add_space(6.0);
        }
        if !warn.is_empty() {
            ui.label(egui::RichText::new("⚠ À surveiller").size(11.5).strong().color(egui::Color32::from_rgb(230, 160, 30)));
            ui.add_space(3.0);
            for c in &warn { check_row_compact(ui, c); }
            ui.add_space(6.0);
        }
        if critical.is_empty() && warn.is_empty() {
            ui.add_space(4.0);
            ui.label(egui::RichText::new("✅ Aucun problème détecté").size(12.0).color(egui::Color32::from_rgb(50, 190, 100)));
            ui.add_space(8.0);
        }

        ui.separator();
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Tous les contrôles").size(11.0).strong().color(egui::Color32::GRAY));
        ui.add_space(4.0);
        for c in &all { check_row_compact(ui, c); }
    }

    // ── Phases ────────────────────────────────────────────────────────────────
    fn tab_phases(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Distribution des phases moteur").size(13.0).strong());
        ui.add_space(8.0);
        let total = self.msv70_analysis.total_samples;
        for p in Phase::all() {
            phase_bar(ui, *p, self.msv70_analysis.phase_counts[*p as usize], total);
            ui.add_space(2.0);
        }
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Légende").size(12.0).strong().color(egui::Color32::GRAY));
        ui.add_space(4.0);
        for p in Phase::all() {
            let desc = match p {
                Phase::WarmUp       => "Moteur froid — coolant sous le seuil",
                Phase::Wot          => "Pleine charge (pédale > seuil WOT)",
                Phase::Acceleration => "Accélération transitoire (ouverture rapide pédale)",
                Phase::Cruise       => "Régime stabilisé, charge constante",
                Phase::Deceleration => "Décélération / DFCO — coupure carburant",
                Phase::Overrun      => "Roue libre — pédale levée, régime élevé",
                Phase::Idle         => "Ralenti",
            };
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                ui.painter().rect_filled(r, 3.0, p.bar_color());
                ui.add_space(4.0);
                ui.label(egui::RichText::new(format!("{} — {}", p.label(), desc)).size(11.0));
            });
            ui.add_space(2.0);
        }

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);

        let ov = self.msv70_analysis.show_phase_overlay;
        let fill = if ov { egui::Color32::from_rgb(40, 85, 55) } else { egui::Color32::from_rgb(48, 50, 56) };
        let btn = egui::Frame::NONE.fill(fill).corner_radius(6)
            .inner_margin(egui::vec2(14.0, 8.0))
            .show(ui, |ui| {
                ui.label(egui::RichText::new(if ov { "🎨  Overlay activé sur le graphique" } else { "🎨  Activer l'overlay sur le graphique" })
                    .color(egui::Color32::WHITE).size(13.0));
            });
        if btn.response.interact(egui::Sense::click()).clicked() {
            self.msv70_analysis.show_phase_overlay = !self.msv70_analysis.show_phase_overlay;
        }
        if btn.response.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
        if ov {
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Chaque phase est visible en fond coloré semi-transparent sur le graphique principal.")
                .size(11.0).color(egui::Color32::GRAY).italics());
        }
    }

    // ── Fuel (compact) ────────────────────────────────────────────────────────
    fn tab_fuel_compact(&mut self, ui: &mut egui::Ui) {
        let checks = self.msv70_analysis.checks_fuel.clone();
        status_counters(ui, &checks);
        missing_channels_summary(ui, &checks);

        // Group checks by label prefix for flexible sectioning
        let is_stft   = |c: &DiagCheck| c.label.contains("STFT") && !c.label.contains("oscillation") && !c.label.contains("Avancé");
        let is_ltft   = |c: &DiagCheck| c.label.contains("LTFT") && !c.label.contains("Convergence") && !c.label.contains("oscillation");
        let is_lambda = |c: &DiagCheck| c.label.contains("Lambda") || c.label.contains("Balance");
        let is_adv    = |c: &DiagCheck| c.label.contains("Correction totale") || c.label.contains("oscillation") || c.label.contains("Convergence");

        for (title, predicate) in [
            ("Court terme (STFT)",    &is_stft   as &dyn Fn(&DiagCheck) -> bool),
            ("Long terme (LTFT)",     &is_ltft),
            ("Balance & Lambda",      &is_lambda),
            ("Analyse avancée",       &is_adv),
        ] {
            let subset: Vec<&DiagCheck> = checks.iter().filter(|c| predicate(c)).collect();
            if subset.is_empty() { continue; }
            egui::CollapsingHeader::new(egui::RichText::new(title).size(11.5).strong())
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(2.0);
                    for c in &subset { check_row_compact(ui, c); }
                });
            ui.add_space(4.0);
        }

        // Bank 1 vs Bank 2 per-phase comparison table
        let bank_cmp = self.msv70_analysis.bank_compare_by_phase.clone();
        let has_data = bank_cmp.iter().any(|row| row.iter().any(|v| !v.is_nan()));
        if has_data {
            egui::CollapsingHeader::new(
                egui::RichText::new("Banque 1 vs Banque 2 par phase").size(11.5).strong()
            )
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(4.0);
                let col_w = [72.0f32, 40.0, 40.0, 40.0, 40.0, 40.0, 40.0];
                // Header
                ui.horizontal(|ui| {
                    ui.add_sized([col_w[0], 14.0], egui::Label::new(
                        egui::RichText::new("Phase").size(10.0).color(egui::Color32::GRAY).strong()));
                    for lbl in ["STFT B1", "STFT B2", "LTFA B1", "LTFA B2", "LTFM B1", "LTFM B2"] {
                        ui.add_sized([col_w[1], 14.0], egui::Label::new(
                            egui::RichText::new(lbl).size(9.5).color(egui::Color32::GRAY).strong()));
                    }
                });
                ui.separator();
                ui.add_space(2.0);

                for (p_idx, phase) in Phase::all().iter().enumerate() {
                    let row = &bank_cmp[p_idx];
                    if row.iter().any(|v| v.is_nan()) { continue; }
                    let [s1, s2, a1, a2, m1, m2] = *row;
                    let delta_stft = (s1 - s2).abs();
                    let delta_ltfm = (m1 - m2).abs();
                    let row_color = if delta_stft > 3.0 || delta_ltfm > 3.0 {
                        egui::Color32::from_rgb(220, 100, 60)
                    } else if delta_stft > 1.5 || delta_ltfm > 1.5 {
                        egui::Color32::from_rgb(200, 170, 60)
                    } else {
                        egui::Color32::LIGHT_GRAY
                    };
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 14.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 1.5, phase.bar_color());
                        ui.add_sized([col_w[0] - 5.0, 14.0], egui::Label::new(
                            egui::RichText::new(phase.label()).size(10.0).color(row_color)));
                        for val in [s1, s2, a1, a2, m1, m2] {
                            let col = if val.abs() > 5.0 {
                                egui::Color32::from_rgb(220, 100, 60)
                            } else if val.abs() > 2.0 {
                                egui::Color32::from_rgb(200, 170, 60)
                            } else {
                                egui::Color32::from_rgb(120, 200, 120)
                            };
                            ui.add_sized([col_w[1], 14.0], egui::Label::new(
                                egui::RichText::new(format!("{:+.1}%", val)).size(10.0)
                                    .monospace().color(col)));
                        }
                    });
                }
                ui.add_space(4.0);
                ui.label(egui::RichText::new(
                    "Rouge = écart B1/B2 > 3% ou valeur > 5% | Orange = > 1.5% ou > 2%"
                ).size(9.5).color(egui::Color32::GRAY).italics());
            });
            ui.add_space(4.0);
        }
    }

    // ── Engine (compact) ──────────────────────────────────────────────────────
    fn tab_engine_compact(&mut self, ui: &mut egui::Ui) {
        let eng   = self.msv70_analysis.checks_engine.clone();
        let elec  = self.msv70_analysis.checks_elec.clone();
        let temps = self.msv70_analysis.checks_temp.clone();

        let all: Vec<DiagCheck> = eng.iter().chain(&elec).chain(&temps).cloned().collect();
        status_counters(ui, &all);
        missing_channels_summary(ui, &all);

        for (title, checks) in [
            ("Mécanique & charge", &eng),
            ("Électricité",        &elec),
            ("Températures",       &temps),
        ] {
            egui::CollapsingHeader::new(egui::RichText::new(title).size(11.5).strong())
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(2.0);
                    for c in checks { check_row_compact(ui, c); }
                });
            ui.add_space(4.0);
        }
    }

    // ── Par Phase ─────────────────────────────────────────────────────────────
    fn tab_per_phase(&mut self, ui: &mut egui::Ui) {
        // Phase selector pills
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(3.0, 3.0);
            for (i, p) in Phase::all().iter().enumerate() {
                let sel = self.msv70_analysis.selected_phase_tab == i;
                let cnt = self.msv70_analysis.phase_counts[i];
                let fill = if sel { p.bar_color() } else { egui::Color32::from_rgb(38, 40, 44) };
                let tcol = if sel { egui::Color32::WHITE } else { egui::Color32::from_rgb(160, 160, 160) };
                let fr = egui::Frame::NONE
                    .fill(fill)
                    .corner_radius(12)
                    .inner_margin(egui::vec2(7.0, 3.0))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!("{} ({})", p.label(), cnt))
                                .size(10.5)
                                .color(tcol),
                        );
                    });
                if fr.response.interact(egui::Sense::click()).clicked() {
                    self.msv70_analysis.selected_phase_tab = i;
                }
                if fr.response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            }
        });
        ui.add_space(6.0);
        ui.separator();
        ui.add_space(4.0);

        let phase_idx = self.msv70_analysis.selected_phase_tab;
        let phase = Phase::all()[phase_idx];
        let cnt = self.msv70_analysis.phase_counts[phase_idx];

        // Phase header with colour bar
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(egui::vec2(4.0, 16.0), egui::Sense::hover());
            ui.painter().rect_filled(r, 2.0, phase.bar_color());
            ui.add_space(4.0);
            ui.label(egui::RichText::new(phase.label()).size(12.5).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("{} pts", cnt)).size(10.5).color(egui::Color32::GRAY));
            });
        });
        ui.add_space(4.0);

        // Phase description
        let desc = match phase {
            Phase::WarmUp       => "Moteur froid — correcteur d'enrichissement actif",
            Phase::Wot          => "Pleine charge — lambda riche, knock=0 impératif",
            Phase::Acceleration => "Transitoire — enrichissement injecteurs attendu",
            Phase::Cruise       => "Régime stabilisé — boucle fermée stable",
            Phase::Deceleration => "DFCO — coupure carburant / lambda maigre",
            Phase::Overrun      => "Roue libre — pédale levée, régime élevé",
            Phase::Idle         => "Ralenti — boucle fermée, stabilité RPM",
        };
        ui.label(egui::RichText::new(desc).size(10.5).color(egui::Color32::GRAY).italics());
        ui.add_space(6.0);

        if cnt < 5 {
            ui.colored_label(egui::Color32::from_rgb(150, 150, 80),
                "Phase insuffisamment représentée dans ce log.");
            return;
        }

        // Show per-phase checks
        if let Some(checks) = self.msv70_analysis.checks_by_phase.get(phase_idx) {
            let checks = checks.clone();
            status_counters(ui, &checks);
            missing_channels_summary(ui, &checks);
            for c in &checks { check_row_compact(ui, c); }
        }
    }

    // ── E85 Compatibility ─────────────────────────────────────────────────────
    fn tab_e85(&mut self, ui: &mut egui::Ui) {
        let checks = self.msv70_analysis.checks_e85.clone();

        // E85 score banner
        let (score, n_checks) = self.msv70_analysis.e85_score();
        if n_checks > 0 {
            let sc = if score >= 75 { egui::Color32::from_rgb(50, 190, 100) }
                     else if score >= 50 { egui::Color32::from_rgb(230, 160, 30) }
                     else { egui::Color32::from_rgb(210, 55, 55) };
            let verdict = if score >= 75 { "E85 compatible avec la carto" }
                          else if score >= 50 { "E85 partiellement adapté" }
                          else { "E85 incompatible / carto insuffisante" };
            let n_crit = checks.iter().filter(|c| c.status == DiagStatus::Critical).count();
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(26, 28, 32))
                .corner_radius(6)
                .inner_margin(egui::vec2(10.0, 7.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{}%", score)).size(22.0).strong().color(sc));
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(verdict).size(11.5).color(sc));
                            ui.label(egui::RichText::new(
                                format!("{} contrôles E85{}", n_checks,
                                    if n_crit > 0 { format!(" · {} critique(s)", n_crit) } else { String::new() })
                            ).size(10.0).color(egui::Color32::GRAY));
                        });
                    });
                });
            ui.add_space(6.0);
        } else if checks.is_empty() {
            ui.colored_label(egui::Color32::GRAY, "Lancer l'analyse (▶) pour diagnostiquer la compatibilité E85.");
            return;
        }

        status_counters(ui, &checks);
        missing_channels_summary(ui, &checks);

        // Predicats mutuellement exclusifs par nom de label
        let is_lambda = |c: &DiagCheck| c.label.starts_with("Lambda");
        let is_knock  = |c: &DiagCheck| c.label.starts_with("Cliquetis");
        let is_duty   = |c: &DiagCheck| c.label.starts_with("Duty") || c.label.starts_with("Rapport duty");
        let is_stft   = |c: &DiagCheck| c.label.starts_with("Dérive") || c.label.starts_with("Écart STFT");
        let is_ltft   = |c: &DiagCheck| c.label.starts_with("LTFT") || c.label.starts_with("Écart LTFT");

        for (title, predicate) in [
            ("Lambda par phase (E85)",      &is_lambda as &dyn Fn(&DiagCheck) -> bool),
            ("Cliquetis — tolérance E85",   &is_knock),
            ("Injecteurs (E85)",             &is_duty),
            ("STFT — dérive & équilibre",   &is_stft),
            ("LTFT & déséquilibre banques", &is_ltft),
        ] {
            let subset: Vec<&DiagCheck> = checks.iter().filter(|c| predicate(c)).collect();
            if subset.is_empty() { continue; }
            egui::CollapsingHeader::new(egui::RichText::new(title).size(11.5).strong())
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(2.0);
                    for c in &subset { check_row_compact(ui, c); }
                });
            ui.add_space(4.0);
        }

        // Info footer
        ui.add_space(2.0);
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(22, 30, 42))
            .corner_radius(4)
            .inner_margin(egui::vec2(8.0, 6.0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new(
                    "ℹ Seuils lambda WOT et duty cycle E85 configurables dans l'onglet 🔧"
                ).size(10.0).color(egui::Color32::from_rgb(100, 150, 200)).italics());
            });
    }

    // ── Settings ──────────────────────────────────────────────────────────────
    fn tab_settings(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Seuils de classification des phases").size(13.0).strong());
        ui.add_space(8.0);
        egui::Grid::new("msv70_cfg").num_columns(2).spacing([20.0, 6.0]).show(ui, |ui| {
            macro_rules! row {
                ($lbl:expr, $fld:ident, $min:expr, $max:expr, $spd:expr) => {
                    ui.label(egui::RichText::new($lbl).size(12.0));
                    ui.add(egui::DragValue::new(&mut self.msv70_analysis.$fld).range($min..=$max).speed($spd));
                    ui.end_row();
                }
            }
            row!("WOT pédale (%)",               wot_tps_thresh,    50.0, 100.0,  0.5);
            row!("RPM ralenti max",               idle_rpm_thresh,  400.0,1500.0, 10.0);
            row!("Charge ralenti max (%)",         idle_load_thresh,  2.0,  30.0,  0.5);
            row!("Seuil accélération (Δ%/s)",      accel_dtps_thresh,  5.0, 200.0,  1.0);
            row!("Seuil décélération (ΔRPM/s)",   decel_drpm_thresh,-800.0, -10.0, 5.0);
            row!("Durée min. phase (s)",           phase_min_dur_s,    0.0,  10.0,  0.1);
            row!("TPS max roue libre (%)",         overrun_tps_max,   0.0,  15.0,  0.5);
            row!("LTFT avertissement (%)",         ltft_warn,         1.0,  15.0,  0.5);
            row!("LTFT critique (%)",              ltft_crit,         5.0,  25.0,  0.5);
            row!("Alerte balance banques (%)",     bank_alert,        1.0,  15.0,  0.5);
            row!("Coolant moteur chaud min (°C)",  min_coolant,      40.0, 100.0,  1.0);
            row!("Duty cycle avert. (%)",          duty_warn,        60.0,  95.0,  1.0);
            row!("Duty cycle critique (%)",        duty_crit,        70.0, 100.0,  1.0);
        });
        ui.add_space(16.0);
        ui.label(egui::RichText::new("🌽 Seuils E85 spécifiques").size(13.0).strong()
            .color(egui::Color32::from_rgb(100, 200, 130)));
        ui.add_space(8.0);
        egui::Grid::new("msv70_e85_cfg").num_columns(2).spacing([20.0, 6.0]).show(ui, |ui| {
            macro_rules! row_e85 {
                ($lbl:expr, $fld:ident, $min:expr, $max:expr, $spd:expr) => {
                    ui.label(egui::RichText::new($lbl).size(12.0));
                    ui.add(egui::DragValue::new(&mut self.msv70_analysis.$fld).range($min..=$max).speed($spd));
                    ui.end_row();
                }
            }
            row_e85!("Lambda WOT min E85",          wot_lambda_min_e85, 0.70, 0.95, 0.005);
            row_e85!("Lambda WOT max E85",          wot_lambda_max_e85, 0.80, 1.00, 0.005);
            row_e85!("Duty injecteur avert. E85 (%)", e85_duty_warn,    60.0, 95.0, 1.0);
            row_e85!("Duty injecteur crit. E85 (%)", e85_duty_crit,     70.0, 100.0, 1.0);
        });
        ui.add_space(12.0);
        ui.colored_label(egui::Color32::GRAY, "Les seuils prennent effet à la prochaine analyse (▶ Analyser).");
    }

    // ── Generate Testo diagnostic job ─────────────────────────────────────────
    fn generate_testo_diagnostic_job(&mut self) {
        // Hard-coded address lists for the two required MSV70 diagnostic jobs.
        // Job 1: fuel trims + engine basics
        let fuel_args = "3;0x4807;0x5804;0x480B;0x4300;0x4402;0x460A;0x5806;0x5808;0x5A83;0x5A84;0x5A85;0x5A86";
        // Job 2: MAF, injection, VANOS, ignition angle
        let engine_args = "3;0x4807;0x5804;0x4203;0x5A42;0x4508;0x4505;0x5A49";

        let xml = format!(
            "<?xml version='1.0' encoding='UTF-8'?>\n\
             <customjobs>\n\
             \x20 <customjob ecu=\"MSV70\">\n\
             \x20   <jobname>STATUS_MESSWERTBLOCK_LESEN</jobname>\n\
             \x20   <virtualname>MSV70_DIAG_FUEL</virtualname>\n\
             \x20   <comment>RPM, Load, Pedal, Coolant, Oil, Battery, STFT B1/B2, LTFT add B1/B2, LTFT mul B1/B2</comment>\n\
             \x20   <argument>{fuel_args}</argument>\n\
             \x20 </customjob>\n\
             \x20 <customjob ecu=\"MSV70\">\n\
             \x20   <jobname>STATUS_MESSWERTBLOCK_LESEN</jobname>\n\
             \x20   <virtualname>MSV70_DIAG_ENGINE</virtualname>\n\
             \x20   <comment>RPM, Load, MAF kg/h, Inj Cyl1, VANOS actual, VANOS setpoint, Ignition angle Cyl1</comment>\n\
             \x20   <argument>{engine_args}</argument>\n\
             \x20 </customjob>\n\
             </customjobs>\n",
        );

        // Default save path: same directory as the user's existing customjobs.xml if found
        let default_name = "customjobs_msv70_diag.xml";
        let dialog = rfd::FileDialog::new()
            .set_file_name(default_name)
            .add_filter("XML", &["xml"])
            .set_title("Sauvegarder le job Testo MSV70 Diagnostic");

        if let Some(path) = dialog.save_file() {
            match std::fs::write(&path, &xml) {
                Ok(_) => self.show_toast_success(
                    &format!("Job Testo sauvegardé : {}", path.display())
                ),
                Err(e) => self.show_toast_error(
                    &format!("Erreur sauvegarde : {e}")
                ),
            }
        }
    }
}
