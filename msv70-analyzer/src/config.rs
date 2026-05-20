use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// All user-configurable parameters. Nothing is hardcoded in analysis/GUI code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // ── Fuel trim thresholds (%) ─────────────────────────────────────────────
    pub stft_note:       f64,   // notable (·)
    pub stft_warn:       f64,   // warning (⚠)
    pub stft_alarm:      f64,   // alarm   (⛔)
    pub stft_imbalance:  f64,   // B1-B2 persistent gap
    pub ltft_warn:       f64,
    pub ltft_alarm:      f64,
    pub ltft_imbalance:  f64,

    // ── Lambda thresholds ────────────────────────────────────────────────────
    pub lmb_lean_load:   f64,   // lean alarm under load (λ>this)
    pub lmb_rich:        f64,   // rich alarm (λ<this)
    pub lmb_wot_target:  f64,   // E85 WOT target lambda
    pub lmb_wot_warn:    f64,   // lean warning at WOT

    // ── VANOS (°CRK) ─────────────────────────────────────────────────────────
    pub vanos_steady_warn:  f64,
    pub vanos_steady_alarm: f64,

    // ── Injector duty (%) ────────────────────────────────────────────────────
    pub inj_duty_warn:   f64,
    pub inj_duty_alarm:  f64,

    // ── Coolant (°C) ─────────────────────────────────────────────────────────
    pub coolant_warn:    f64,   // overtemp alarm
    pub coolant_cl_min:  f64,   // below → not in closed-loop

    // ── Knock proxy ──────────────────────────────────────────────────────────
    pub knock_rpm_min:      f64,
    pub knock_load_min:     f64,
    pub knock_delta:        f64,   // °BTDC below per-cell p50 → retard event
    pub knock_hot_coolant:  f64,   // baseline built only above this

    // ── Phase detection conditions ───────────────────────────────────────────
    pub phase_cl_rpm_min:       f64,   // CL entry: rpm >
    pub phase_cl_load_min:      f64,   // CL entry: load >
    pub phase_wot_pedal:        f64,   // WOT threshold pedal %
    pub phase_wot_load:         f64,   // WOT threshold load %
    pub phase_accel_pedal:      f64,   // ACCEL threshold
    pub phase_fuel_cut_pedal:   f64,   // fuel-cut: pedal <
    pub phase_fuel_cut_load:    f64,   // fuel-cut: load% < (guards against cruise control at pedal=0)
    pub phase_fuel_cut_rpm:     f64,   // fuel-cut: rpm >
    pub phase_fuel_cut_speed:   f64,   // fuel-cut: speed > km/h
    pub phase_hot_coolant:      f64,   // HOT_IDLE vs WARMUP boundary
    pub phase_idle_rpm:         f64,   // standing idle: rpm <
    pub phase_idle_speed:       f64,   // standing idle: speed < km/h

    // ── Engine ───────────────────────────────────────────────────────────────
    pub engine_cyl:               usize,  // number of cylinders (for mg/stk computation)
    pub load_pct_to_mg_stk_scale: f64,    // fallback when no MAF: load% × scale = mg/stk

    // ── Phase smoothing ──────────────────────────────────────────────────────
    pub phase_min_duration_s: f64,  // merge phase spans shorter than this

    // ── STFT map bins (RPM in rpm, Load in mg/stk) ───────────────────────────
    pub rpm_bins:  Vec<f64>,
    pub load_bins: Vec<f64>,

    // ── Lambda map bins (matches ip_lamb_bas_1 XDF axes) ────────────────────
    pub lmb_rpm_bins:  Vec<f64>,
    pub lmb_load_bins: Vec<f64>,

    // ── Analysis engine parameters ───────────────────────────────────────────
    pub near_zero:             f64,    // TestO float noise floor
    pub lean_debounce:         usize,  // consecutive samples before LEAN event
    pub cell_min_samples:      usize,  // min samples for a valid map cell
    pub ign_cell_min_samples:  usize,  // min samples for knock baseline cell
    pub stft_bucket_seconds:   f64,    // STFT drift bucket width
    pub stft_alarm_streak:     usize,  // consecutive CL samples above alarm
    pub display_max_points:    usize,  // max points in time-series plots
    pub steady_state_max_dt:   f64,    // max dt (s) for "steady-state" row
    pub steady_state_drpm:     f64,    // max ΔRPM/s for steady-state
    pub steady_state_dload:    f64,    // max ΔLoad%/s for steady-state

    // ── Column aliases (internal_name → list of CSV header strings) ──────────
    pub col_aliases: BTreeMap<String, Vec<String>>,

    // ── App ──────────────────────────────────────────────────────────────────
    pub sidebar_width:  f32,
    pub report_width:   f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut col_aliases: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let aliases: &[(&str, &[&str])] = &[
            ("time_ms",     &["time"]),
            ("rpm",         &["RPM"]),
            ("load_pct",    &["Load %"]),
            ("pedal_pct",   &["Pedal %"]),
            ("speed_kmh",   &["Speed km/h"]),
            ("coolant_c",   &["Cool degC"]),
            ("stft_b1",     &["STFTGrp1", "STFT B1"]),
            ("stft_b2",     &["STFTGrp2", "STFT B2"]),
            ("ltft_add_b1", &["LTFT add. B1"]),
            ("ltft_add_b2", &["LTFT add. B2"]),
            ("ltft_mul_b1", &["LTFT Mul B1"]),
            ("ltft_mul_b2", &["LTFT Mul B2"]),
            ("lmb_b1",      &["Lmb Act B1"]),
            ("lmb_b2",      &["Lmb Act B2"]),
            ("lmb_tgt_b1",  &["Lmb Tgt B1"]),
            ("maf_kgh",     &["AirMass", "MAF kg/h"]),
            ("inj_ms",      &["Inj C1 ms"]),
            ("vanos_act",   &["Vanos InAct"]),
            ("vanos_tgt",   &["Vanos InTgt"]),
            ("ign_deg",     &["Ign Cyl1 deg"]),
            ("iat_c",       &["IAT degC"]),
            ("oil_c",       &["Oil degC"]),
            // IGN_KNOCK job channels
            ("knock_status", &["ISKLO", "Status Knock", "Knock Stat", "Klopf Stat", "LV KNK"]),
            // LAMB_TRIM job channels
            ("lmb_trim_b1",  &["ILRR1", "Lamb Shift B1", "Lambda shift 1", "Lmb Shift 1", "Lamb Shift Ctrl B1"]),
            ("lmb_trim_b2",  &["ILRR2", "Lamb Shift B2", "Lambda shift 2", "Lmb Shift 2", "Lamb Shift Ctrl B2"]),
            ("hinterkat_b1", &["IUSN1", "Post Cat B1", "HK V B1", "Post Cat V B1", "HinterKat B1"]),
            ("hinterkat_b2", &["IUSN2", "Post Cat B2", "HK V B2", "Post Cat V B2", "HinterKat B2"]),
        ];
        for &(k, v) in aliases {
            col_aliases.insert(k.to_string(), v.iter().map(|s| s.to_string()).collect());
        }

        AppConfig {
            stft_note:      8.0,
            stft_warn:      15.0,
            stft_alarm:     22.0,
            stft_imbalance: 8.0,
            ltft_warn:      12.0,
            ltft_alarm:     18.0,
            ltft_imbalance: 12.0,

            lmb_lean_load:  1.04,
            lmb_rich:       0.93,
            lmb_wot_target: 0.92,
            lmb_wot_warn:   0.95,

            vanos_steady_warn:  5.0,
            vanos_steady_alarm: 10.0,

            inj_duty_warn:  80.0,
            inj_duty_alarm: 90.0,

            coolant_warn:   114.0,
            coolant_cl_min: 55.0,

            knock_rpm_min:     2000.0,
            knock_load_min:    35.0,
            knock_delta:       6.0,
            knock_hot_coolant: 85.0,

            phase_cl_rpm_min:     600.0,
            phase_cl_load_min:    2.0,
            phase_wot_pedal:      85.0,
            phase_wot_load:       72.0,
            phase_accel_pedal:    40.0,
            phase_fuel_cut_pedal: 5.0,
            phase_fuel_cut_load:  10.0,  // below this load% = real fuel cut (not cruise control)
            phase_fuel_cut_rpm:   1200.0,
            phase_fuel_cut_speed: 5.0,
            phase_hot_coolant:    80.0,
            phase_idle_rpm:       1200.0,
            phase_idle_speed:     5.0,

            engine_cyl:               6,
            load_pct_to_mg_stk_scale: 7.0,
            phase_min_duration_s:     0.5,

            // Midpoints of ip_mff_cor_opm_1_1 XDF axes as bin edges
            rpm_bins: vec![
                0.0, 752.0, 1056.0, 1328.0, 1520.0, 1808.0, 2256.0, 2752.0,
                3248.0, 3696.0, 4000.0, 4304.0, 4656.0, 4944.0, 5296.0, 5760.0, 9999.0,
            ],
            load_bins: vec![
                0.0, 53.8, 100.0, 150.0, 212.5, 287.5, 362.5,
                425.0, 475.0, 525.0, 575.0, 650.0, 9999.0,
            ],

            // Midpoints of ip_lamb_bas_1 XDF axes as bin edges
            lmb_rpm_bins:  vec![0.0, 976.0, 1504.0, 1888.0, 2256.0, 2752.0, 3504.0, 4512.0, 9999.0],
            lmb_load_bins: vec![0.0, 100.0, 150.0, 200.0, 250.0, 325.0, 399.0, 461.5, 9999.0],

            near_zero:            1e-5,
            lean_debounce:        3,
            cell_min_samples:     5,
            ign_cell_min_samples: 10,
            stft_bucket_seconds:  60.0,
            stft_alarm_streak:    6,
            display_max_points:   4000,
            steady_state_max_dt:  1.5,
            steady_state_drpm:    300.0,
            steady_state_dload:   20.0,

            col_aliases,
            sidebar_width: 220.0,
            report_width:  420.0,
        }
    }
}

impl AppConfig {
    /// Load from JSON, fall back to default on error
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn config_path() -> PathBuf {
        let exe = std::env::current_exe().unwrap_or_default();
        exe.parent().unwrap_or(Path::new(".")).join("msv70_config.json")
    }

    /// Compute bin labels from bin edges
    pub fn rpm_labels(&self) -> Vec<String> {
        bin_labels(&self.rpm_bins)
    }

    pub fn load_labels(&self) -> Vec<String> {
        bin_labels(&self.load_bins)
    }

    pub fn rpm_bin(&self, v: f64) -> usize { bin_idx(&self.rpm_bins, v) }
    pub fn load_bin(&self, v: f64) -> usize { bin_idx(&self.load_bins, v) }
    pub fn lmb_rpm_bin(&self, v: f64) -> usize { bin_idx(&self.lmb_rpm_bins, v) }
    pub fn lmb_load_bin(&self, v: f64) -> usize { bin_idx(&self.lmb_load_bins, v) }
    pub fn lmb_rpm_labels(&self)  -> Vec<String> { bin_labels(&self.lmb_rpm_bins) }
    pub fn lmb_load_labels(&self) -> Vec<String> { bin_labels(&self.lmb_load_bins) }
}

fn bin_idx(bins: &[f64], v: f64) -> usize {
    for i in 0..bins.len().saturating_sub(1) {
        if v >= bins[i] && v < bins[i + 1] { return i; }
    }
    bins.len().saturating_sub(2)
}

fn bin_labels(bins: &[f64]) -> Vec<String> {
    if bins.len() < 2 { return vec![]; }
    let mut labels = Vec::new();
    for i in 0..bins.len() - 1 {
        let lo = bins[i];
        let hi = bins[i + 1];
        if i == bins.len() - 2 {
            labels.push(format!("{:.0}+", lo));
        } else if i == 0 {
            labels.push(format!("<{:.0}", hi));
        } else {
            labels.push(format!("{:.0}-{:.0}", lo, hi));
        }
    }
    labels
}
