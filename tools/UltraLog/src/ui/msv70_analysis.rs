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
        egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 38)
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

    // Thresholds
    pub wot_tps_thresh:    f64,
    pub idle_rpm_thresh:   f64,
    pub idle_load_thresh:  f64,
    pub accel_dtps_thresh: f64,
    pub decel_drpm_thresh: f64,
    pub overrun_tps_max:   f64,
    pub ltft_warn:         f64,
    pub ltft_crit:         f64,
    pub bank_alert:        f64,
    pub min_coolant:       f64,
    pub duty_warn:         f64,
    pub duty_crit:         f64,
}

impl Msv70AnalysisState {
    pub fn new() -> Self {
        Self {
            wot_tps_thresh:    85.0,
            idle_rpm_thresh:   900.0,
            idle_load_thresh:  12.0,
            accel_dtps_thresh: 5.0,
            decel_drpm_thresh: -20.0,
            overrun_tps_max:   5.0,
            ltft_warn:         5.0,
            ltft_crit:         10.0,
            bank_alert:        5.0,
            min_coolant:       70.0,
            duty_warn:         80.0,
            duty_crit:         85.0,
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

    for i in 1..n {
        let r  = rpm_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(0.0);
        let t  = tps_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(0.0);
        let l  = load_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(0.0);
        let c  = cool_d.as_deref().and_then(|v| v.get(i)).copied().unwrap_or(100.0);
        let dt = t - tps_d.as_deref().and_then(|v| v.get(i - 1)).copied().unwrap_or(t);
        let dr = r - rpm_d.as_deref().and_then(|v| v.get(i - 1)).copied().unwrap_or(r);

        out[i] = if c < cfg.min_coolant {
            Phase::WarmUp
        } else if t > cfg.wot_tps_thresh {
            Phase::Wot
        } else if t < cfg.overrun_tps_max && r > cfg.idle_rpm_thresh + 200.0 {
            if dr < cfg.decel_drpm_thresh { Phase::Deceleration } else { Phase::Overrun }
        } else if r < cfg.idle_rpm_thresh || l < cfg.idle_load_thresh {
            Phase::Idle
        } else if dt > cfg.accel_dtps_thresh {
            Phase::Acceleration
        } else {
            Phase::Cruise
        };
    }
    out
}

fn to_segments(phases: &[Phase], times: &[f64]) -> Vec<PhaseSegment> {
    if phases.is_empty() || times.is_empty() { return Vec::new(); }
    let mut segs = Vec::new();
    let mut cur = phases[0];
    let mut start = times[0];
    for i in 1..phases.len().min(times.len()) {
        if phases[i] != cur {
            let end = times[i];
            if end - start > 0.04 {
                segs.push(PhaseSegment { start, end, phase: cur });
            }
            cur = phases[i]; start = times[i];
        }
    }
    if let Some(&last) = times.last() {
        if last - start > 0.04 { segs.push(PhaseSegment { start, end: last, phase: cur }); }
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
                status: s, detail: "Doit être riche (> 1.05) ou coupure carbu".into() });
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

    // Knock
    if let Some(knock) = get(log, KNOCK) {
        let events: Vec<f64> = knock.iter().zip(warm.iter())
            .filter_map(|(&k, &w)| if w && k.abs() > 0.5 { Some(k.abs()) } else { None }).collect();
        let ne = events.len(); let ar = if ne > 0 { mean(&events) } else { 0.0 };
        let s = if ne == 0 { DiagStatus::Ok } else if ar < 2.0 { DiagStatus::Warn } else { DiagStatus::Critical };
        out.push(DiagCheck { label: "Événements cliquetis".into(), value: format!("{}", ne), status: s,
            detail: if ne > 0 { format!("Retard moy: {:.1}°", ar) } else { "Aucun".into() } });
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

    if let Some(oil) = get(log, OIL) {
        let mx = vmax(&oil);
        let s = if mx > 80.0 { DiagStatus::Ok } else { DiagStatus::Info };
        out.push(DiagCheck { label: "Temp. huile (max)".into(), value: format!("{:.1}°C", mx),
            status: s, detail: "> 80°C attendu en conduite".into() });
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

fn run_analysis(log: &Log, st: &mut Msv70AnalysisState) {
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

/// Compact 2-line card for use in the narrow 280px side panel
fn check_row_compact(ui: &mut egui::Ui, c: &DiagCheck) {
    egui::Frame::NONE
        .fill(egui::Color32::from_rgb(28, 30, 34))
        .corner_radius(4)
        .inner_margin(egui::vec2(7.0, 5.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(&c.label)
                        .size(11.0)
                        .color(egui::Color32::LIGHT_GRAY),
                ).wrap());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    badge(ui, c.status);
                });
            });
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&c.value).monospace().size(11.0).strong());
                if !c.detail.is_empty() {
                    ui.label(
                        egui::RichText::new(format!("· {}", &c.detail))
                            .size(10.0)
                            .color(egui::Color32::from_rgb(120, 120, 120))
                            .italics(),
                    );
                }
            });
        });
    ui.add_space(2.0);
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
            5 => self.tab_settings(ui),
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
        let sections: &[(&str, usize, usize)] = &[
            ("Court terme (STFT)", 0, 2),
            ("Long terme add.", 2, 4),
            ("Long terme mul.", 4, 6),
            ("Balance & Lambda", 6, checks.len()),
        ];
        for &(title, from, to) in sections {
            let slice = &checks[from.min(checks.len())..to.min(checks.len())];
            if slice.is_empty() { continue; }
            egui::CollapsingHeader::new(egui::RichText::new(title).size(11.5).strong())
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(2.0);
                    for c in slice { check_row_compact(ui, c); }
                });
            ui.add_space(4.0);
        }
    }

    // ── Engine (compact) ──────────────────────────────────────────────────────
    fn tab_engine_compact(&mut self, ui: &mut egui::Ui) {
        let eng   = self.msv70_analysis.checks_engine.clone();
        let elec  = self.msv70_analysis.checks_elec.clone();
        let temps = self.msv70_analysis.checks_temp.clone();

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
            for c in &checks { check_row_compact(ui, c); }
        }
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
            row!("Seuil accélération (Δ%/spl)",   accel_dtps_thresh, 1.0,  20.0,  0.5);
            row!("Seuil décélération (ΔRPM/spl)", decel_drpm_thresh,-200.0,-1.0,  1.0);
            row!("TPS max roue libre (%)",         overrun_tps_max,   0.0,  15.0,  0.5);
            row!("LTFT avertissement (%)",         ltft_warn,         1.0,  15.0,  0.5);
            row!("LTFT critique (%)",              ltft_crit,         5.0,  25.0,  0.5);
            row!("Alerte balance banques (%)",     bank_alert,        1.0,  15.0,  0.5);
            row!("Coolant moteur chaud min (°C)",  min_coolant,      40.0, 100.0,  1.0);
            row!("Duty cycle avert. (%)",          duty_warn,        60.0,  95.0,  1.0);
            row!("Duty cycle critique (%)",        duty_crit,        70.0, 100.0,  1.0);
        });
        ui.add_space(12.0);
        ui.colored_label(egui::Color32::GRAY, "Les seuils prennent effet à la prochaine analyse (▶ Analyser).");
    }
}
