use std::collections::HashMap;

/// One row of parsed CSV data
#[derive(Debug, Clone)]
pub struct Row {
    pub time_ms:     f64,
    /// Values for CSV columns not matched by col_aliases (keyed by original column header)
    pub extra: HashMap<String, f64>,
    pub rpm:         f64,
    pub load_pct:    f64,
    pub pedal_pct:   f64,
    pub speed_kmh:   f64,
    pub coolant_c:   f64,
    pub stft_b1:     f64,
    pub stft_b2:     f64,
    pub ltft_add_b1: f64,
    pub ltft_add_b2: f64,
    pub ltft_mul_b1: f64,
    pub ltft_mul_b2: f64,
    pub lmb_b1:      f64,
    pub lmb_b2:      f64,
    pub lmb_tgt_b1:  f64,
    pub maf_kgh:     f64,
    pub inj_ms:      f64,
    pub vanos_act:   f64,
    pub vanos_tgt:   f64,
    pub ign_deg:     f64,
    pub iat_c:       f64,
    pub oil_c:       f64,
    // Knock & post-cat trim
    pub knock_status:  f64,   // 0x5A36  0=no knock, 1=knock
    pub lmb_trim_b1:   f64,   // 0x5878  Lambdaverschiebung B1 (±0.05 max)
    pub lmb_trim_b2:   f64,   // 0x5879  Lambdaverschiebung B2
    pub hinterkat_b1:  f64,   // 0x5A13  post-cat O2 voltage B1 (V)
    pub hinterkat_b2:  f64,   // 0x5A14  post-cat O2 voltage B2 (V)
    // Derived
    pub phase:        Phase,
    pub rpm_bin:      usize,
    pub load_bin:     usize,
    pub load_mg_stk:  f64,
    pub lmb_rpm_bin:  usize,
    pub lmb_load_bin: usize,
    pub vanos_err:    f64,
    pub cl_b1:        bool,
    pub cl_b2:        bool,
}

impl Default for Row {
    fn default() -> Self {
        Row {
            time_ms: 0.0, rpm: 0.0, load_pct: 0.0, pedal_pct: 0.0,
            speed_kmh: 0.0, coolant_c: 20.0,
            stft_b1: 0.0, stft_b2: 0.0,
            ltft_add_b1: 0.0, ltft_add_b2: 0.0, ltft_mul_b1: 0.0, ltft_mul_b2: 0.0,
            lmb_b1: 1.0, lmb_b2: 1.0, lmb_tgt_b1: 1.0,
            maf_kgh: 0.0, inj_ms: 0.0, vanos_act: 0.0, vanos_tgt: 0.0,
            ign_deg: 0.0, iat_c: 20.0, oil_c: 0.0,
            knock_status: 0.0, lmb_trim_b1: 0.0, lmb_trim_b2: 0.0,
            hinterkat_b1: 0.0, hinterkat_b2: 0.0,
            phase: Phase::EngineOff, rpm_bin: 0, load_bin: 0,
            load_mg_stk: 0.0, lmb_rpm_bin: 0, lmb_load_bin: 0,
            vanos_err: 0.0, cl_b1: false, cl_b2: false,
            extra: HashMap::new(),
        }
    }
}

impl Row {
    pub fn ts(&self) -> f64 { self.time_ms / 1000.0 }

    pub fn inj_duty(&self) -> Option<f64> {
        if self.inj_ms > 0.5 && self.rpm > 0.0 {
            Some(self.inj_ms / (120000.0 / self.rpm) * 100.0)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    EngineOff, Cranking, Warmup, HotIdle, Cruise, Accel, Wot, FuelCut,
}

impl Phase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::EngineOff => "ENGINE_OFF", Phase::Cranking  => "CRANKING",
            Phase::Warmup    => "WARMUP",     Phase::HotIdle   => "HOT_IDLE",
            Phase::Cruise    => "CRUISE",     Phase::Accel     => "ACCEL",
            Phase::Wot       => "WOT",        Phase::FuelCut   => "FUEL_CUT",
        }
    }
    pub fn all() -> &'static [Phase] {
        &[Phase::EngineOff, Phase::Cranking, Phase::Warmup, Phase::HotIdle,
          Phase::Cruise, Phase::Accel, Phase::Wot, Phase::FuelCut]
    }
    pub fn cl_phases() -> &'static [Phase] {
        &[Phase::Warmup, Phase::HotIdle, Phase::Cruise, Phase::Accel, Phase::Wot]
    }
}

/// Tuning correction for one (rpm_bin, load_bin) cell
#[derive(Debug, Clone)]
pub struct Correction {
    pub rpm_idx:   usize,
    pub load_idx:  usize,
    pub med_b1:    Option<f64>,
    pub med_b2:    Option<f64>,
    pub avg_delta: f64,
    pub factor:    f64,
}

/// Anomaly event
#[derive(Debug, Clone)]
pub struct AnomalyEvent {
    pub kind:   String,
    pub time_s: f64,
    pub msg:    String,
}

/// 60-second STFT bucket
#[derive(Debug, Clone)]
pub struct StftBucket {
    pub t_s:    f64,
    pub p50_b1: Option<f64>,
    pub p50_b2: Option<f64>,
    pub n_b1:   usize,
    pub n_b2:   usize,
    pub rpm:    f64,
    pub load:   f64,
    pub cool:   f64,
}

/// Full analysis result for one session
#[derive(Debug, Clone)]
pub struct SessionData {
    pub file_name:   String,
    pub n_rows:      usize,
    pub dur_s:       f64,
    pub cool_start:  f64,
    pub cool_end:    f64,
    pub cool_max:    f64,

    pub rows:        Vec<Row>,
    pub cl_b1_idx:   Vec<usize>,
    pub cl_b2_idx:   Vec<usize>,
    pub cl_both_idx: Vec<usize>,
    pub run_idx:     Vec<usize>,
    pub hot_idx:     Vec<usize>,
    pub steady_idx:  Vec<usize>,

    pub phase_dur:   HashMap<Phase, f64>,
    pub cell_b1:     HashMap<(usize, usize), Vec<f64>>,
    pub cell_b2:     HashMap<(usize, usize), Vec<f64>>,
    pub cell_ign_p50: HashMap<(usize, usize), f64>,

    pub corrections:  Vec<Correction>,
    pub stft_buckets: Vec<StftBucket>,
    pub events:       Vec<AnomalyEvent>,
    pub retard_idx:   Vec<usize>,

    pub has_ltft:     bool,
    pub has_vanos:    bool,
    pub has_ign:      bool,
    pub has_inj:      bool,
    pub has_lmb:      bool,
    pub has_iat:      bool,
    pub has_maf:      bool,
    pub has_knock:    bool,
    pub has_lmb_trim: bool,

    // Knock retard per (rpm_bin, load_bin): (retard_n, hot_candidate_n)
    pub knock_retard_cell: HashMap<(usize, usize), (usize, usize)>,

    // STFT map bin labels
    pub rpm_labels:  Vec<String>,
    pub load_labels: Vec<String>,

    // Lambda actual map (binned by ip_lamb_bas_1 axes)
    pub cell_lmb_b1:    HashMap<(usize, usize), Vec<f64>>,
    pub cell_lmb_b2:    HashMap<(usize, usize), Vec<f64>>,
    pub lmb_rpm_labels: Vec<String>,
    pub lmb_load_labels: Vec<String>,

    /// CSV columns not matched by any col_alias — available as extra graph channels
    pub extra_cols: Vec<String>,
}
