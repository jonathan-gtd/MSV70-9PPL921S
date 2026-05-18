pub struct Thresholds;

impl Thresholds {
    pub const STFT_NOTE:          f64 = 8.0;
    pub const STFT_WARN:          f64 = 15.0;
    pub const STFT_ALARM:         f64 = 22.0;
    pub const STFT_IMBALANCE:     f64 = 8.0;
    pub const LTFT_WARN:          f64 = 12.0;
    pub const LTFT_ALARM:         f64 = 18.0;
    pub const LTFT_IMBALANCE:     f64 = 12.0;
    pub const LMB_LEAN_LOAD:      f64 = 1.04;
    pub const LMB_RICH:           f64 = 0.93;
    pub const VANOS_STEADY_WARN:  f64 = 5.0;
    pub const VANOS_STEADY_ALARM: f64 = 10.0;
    pub const INJ_DUTY_WARN:      f64 = 80.0;
    pub const COOLANT_WARN:       f64 = 114.0;
    pub const KNOCK_RPM_MIN:      f64 = 2000.0;
    pub const KNOCK_LOAD_MIN:     f64 = 35.0;
    pub const KNOCK_DELTA:        f64 = 6.0;
    pub const KNOCK_HOT_COOLANT:  f64 = 85.0;
}
