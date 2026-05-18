use crate::data::types::{Phase, Row};

pub fn detect_phase(row: &Row) -> Phase {
    let rpm   = row.rpm;
    let speed = row.speed_kmh;
    let pedal = row.pedal_pct;
    let load  = row.load_pct;
    let cool  = row.coolant_c;

    if rpm < 50.0   { return Phase::EngineOff; }
    if rpm < 400.0  { return Phase::Cranking; }
    if pedal < 5.0 && rpm > 1200.0 && speed > 5.0 { return Phase::FuelCut; }
    if pedal > 85.0 || load > 72.0 { return Phase::Wot; }
    if pedal > 40.0 { return Phase::Accel; }
    if speed < 5.0 && rpm < 1200.0 {
        return if cool >= 80.0 { Phase::HotIdle } else { Phase::Warmup };
    }
    if cool < 80.0  { return Phase::Warmup; }
    Phase::Cruise
}

pub fn is_steady(curr: &Row, prev: &Row, dt_s: f64) -> bool {
    if dt_s <= 0.0 || dt_s > 1.5 { return false; }
    (curr.rpm - prev.rpm).abs() / dt_s < 300.0
        && (curr.load_pct - prev.load_pct).abs() / dt_s < 20.0
}
