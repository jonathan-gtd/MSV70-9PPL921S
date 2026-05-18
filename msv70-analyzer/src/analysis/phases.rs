use crate::config::AppConfig;
use crate::data::types::{Phase, Row};

pub fn detect_phase(row: &Row, cfg: &AppConfig) -> Phase {
    let rpm   = row.rpm;
    let speed = row.speed_kmh;
    let pedal = row.pedal_pct;
    let load  = row.load_pct;
    let cool  = row.coolant_c;

    if rpm < 50.0   { return Phase::EngineOff; }
    if rpm < 400.0  { return Phase::Cranking; }
    if pedal < cfg.phase_fuel_cut_pedal && rpm > cfg.phase_fuel_cut_rpm && speed > cfg.phase_fuel_cut_speed {
        return Phase::FuelCut;
    }
    if pedal > cfg.phase_wot_pedal || load > cfg.phase_wot_load { return Phase::Wot; }
    if pedal > cfg.phase_accel_pedal { return Phase::Accel; }
    if speed < cfg.phase_idle_speed && rpm < cfg.phase_idle_rpm {
        return if cool >= cfg.phase_hot_coolant { Phase::HotIdle } else { Phase::Warmup };
    }
    if cool < cfg.phase_hot_coolant { return Phase::Warmup; }
    Phase::Cruise
}

pub fn is_steady(curr: &Row, prev: &Row, dt_s: f64, cfg: &AppConfig) -> bool {
    if dt_s <= 0.0 || dt_s > cfg.steady_state_max_dt { return false; }
    (curr.rpm - prev.rpm).abs() / dt_s < cfg.steady_state_drpm
        && (curr.load_pct - prev.load_pct).abs() / dt_s < cfg.steady_state_dload
}
