use std::collections::HashMap;

pub const NEAR_ZERO: f64 = 1e-5;

// RPM / Load bin edges and labels
pub const RPM_BINS: &[f64] = &[0.0, 800.0, 1200.0, 1600.0, 2000.0, 2500.0, 3000.0, 3800.0, 5000.0, 9999.0];
pub const LOAD_BINS: &[f64] = &[0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 75.0, 100.0];
pub const RPM_LABELS: &[&str] = &[
    "<800", "800-1200", "1200-1600", "1600-2000",
    "2000-2500", "2500-3000", "3000-3800", "3800-5000", "5000+",
];
pub const LOAD_LABELS: &[&str] = &[
    "0-10", "10-20", "20-30", "30-40", "40-50", "50-60", "60-75", "75-100",
];

pub fn rpm_bin(rpm: f64) -> usize {
    for i in 0..RPM_BINS.len() - 1 {
        if rpm >= RPM_BINS[i] && rpm < RPM_BINS[i + 1] {
            return i;
        }
    }
    RPM_BINS.len() - 2
}

pub fn load_bin(load: f64) -> usize {
    for i in 0..LOAD_BINS.len() - 1 {
        if load >= LOAD_BINS[i] && load < LOAD_BINS[i + 1] {
            return i;
        }
    }
    LOAD_BINS.len() - 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    EngineOff,
    Cranking,
    Warmup,
    HotIdle,
    Cruise,
    Accel,
    Wot,
    FuelCut,
}

impl Phase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::EngineOff => "ENGINE_OFF",
            Phase::Cranking  => "CRANKING",
            Phase::Warmup    => "WARMUP",
            Phase::HotIdle   => "HOT_IDLE",
            Phase::Cruise    => "CRUISE",
            Phase::Accel     => "ACCEL",
            Phase::Wot       => "WOT",
            Phase::FuelCut   => "FUEL_CUT",
        }
    }

    pub fn all() -> &'static [Phase] {
        &[
            Phase::EngineOff, Phase::Cranking, Phase::Warmup, Phase::HotIdle,
            Phase::Cruise, Phase::Accel, Phase::Wot, Phase::FuelCut,
        ]
    }

    pub fn cl_phases() -> &'static [Phase] {
        &[Phase::Warmup, Phase::HotIdle, Phase::Cruise, Phase::Accel, Phase::Wot]
    }
}

/// One row of parsed CSV data — all fields are plain f64 with sensible defaults.
#[derive(Debug, Clone)]
pub struct Row {
    pub time_ms:     f64,
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
    // Derived
    pub phase:       Phase,
    pub rpm_bin:     usize,
    pub load_bin:    usize,
    pub vanos_err:   f64,
    pub cl_b1:       bool,
    pub cl_b2:       bool,
}

impl Default for Row {
    fn default() -> Self {
        Row {
            time_ms: 0.0, rpm: 0.0, load_pct: 0.0, pedal_pct: 0.0,
            speed_kmh: 0.0, coolant_c: 20.0, stft_b1: 0.0, stft_b2: 0.0,
            ltft_add_b1: 0.0, ltft_add_b2: 0.0, ltft_mul_b1: 0.0, ltft_mul_b2: 0.0,
            lmb_b1: 1.0, lmb_b2: 1.0, lmb_tgt_b1: 1.0,
            maf_kgh: 0.0, inj_ms: 0.0, vanos_act: 0.0, vanos_tgt: 0.0,
            ign_deg: 0.0, iat_c: 20.0, oil_c: 0.0,
            phase: Phase::EngineOff, rpm_bin: 0, load_bin: 0, vanos_err: 0.0,
            cl_b1: false, cl_b2: false,
        }
    }
}

impl Row {
    pub fn ts(&self) -> f64 {
        self.time_ms / 1000.0
    }

    pub fn inj_duty(&self) -> Option<f64> {
        if self.inj_ms > 0.5 && self.rpm > 0.0 {
            Some(self.inj_ms / (120000.0 / self.rpm) * 100.0)
        } else {
            None
        }
    }
}

/// Tuning correction suggestion for a (rpm_bin, load_bin) cell
#[derive(Debug, Clone)]
pub struct Correction {
    pub rpm_idx:  usize,
    pub load_idx: usize,
    pub med_b1:   Option<f64>,
    pub med_b2:   Option<f64>,
    pub avg_delta: f64,
    pub factor:   f64,
}

/// An anomaly event from the event timeline
#[derive(Debug, Clone)]
pub struct AnomalyEvent {
    pub kind: String,
    pub time_s: f64,
    pub msg:  String,
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

/// Full result of analyzing one session
#[derive(Debug, Clone)]
pub struct SessionData {
    pub file_name:    String,
    pub n_rows:       usize,
    pub dur_s:        f64,
    pub cool_start:   f64,
    pub cool_end:     f64,
    pub cool_max:     f64,

    pub rows:         Vec<Row>,

    // Pre-filtered subsets (index into rows)
    pub cl_b1_idx:    Vec<usize>,
    pub cl_b2_idx:    Vec<usize>,
    pub cl_both_idx:  Vec<usize>,
    pub run_idx:      Vec<usize>,
    pub hot_idx:      Vec<usize>,
    pub steady_idx:   Vec<usize>,

    pub phase_dur:    HashMap<Phase, f64>,

    // STFT 2D map: key = (rpm_bin, load_bin), value = Vec<stft>
    pub cell_b1:      HashMap<(usize, usize), Vec<f64>>,
    pub cell_b2:      HashMap<(usize, usize), Vec<f64>>,

    // Ignition baseline per cell (hot engine p50)
    pub cell_ign_p50: HashMap<(usize, usize), f64>,

    pub corrections:  Vec<Correction>,
    pub stft_buckets: Vec<StftBucket>,
    pub events:       Vec<AnomalyEvent>,

    // Knock event indices
    pub retard_idx:   Vec<usize>,

    // Feature availability flags
    pub has_ltft:  bool,
    pub has_vanos: bool,
    pub has_ign:   bool,
    pub has_inj:   bool,
    pub has_lmb:   bool,
    pub has_iat:   bool,
}
