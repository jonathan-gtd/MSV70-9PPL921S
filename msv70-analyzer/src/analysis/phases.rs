use crate::config::AppConfig;
use crate::data::types::{Phase, Row};

pub fn smooth_phases(rows: &mut [Row], min_dur_s: f64) {
    if rows.len() < 2 || min_dur_s <= 0.0 { return; }

    // Build (start_idx, end_idx_exclusive, phase) spans
    let mut spans: Vec<(usize, usize, Phase)> = vec![];
    let mut cur_start = 0usize;
    let mut cur_phase = rows[0].phase;
    for i in 1..rows.len() {
        if rows[i].phase != cur_phase {
            spans.push((cur_start, i, cur_phase));
            cur_start = i;
            cur_phase = rows[i].phase;
        }
    }
    spans.push((cur_start, rows.len(), cur_phase));

    // Forward pass: merge spans shorter than min_dur_s into previous phase
    if spans.len() > 1 {
        let mut prev_phase = spans[0].2;
        for s in spans.iter_mut().skip(1) {
            let dur = (rows[s.1 - 1].time_ms - rows[s.0].time_ms) / 1000.0;
            if dur < min_dur_s {
                s.2 = prev_phase;
            } else {
                prev_phase = s.2;
            }
        }
    }

    // Re-apply
    for (start, end, phase) in &spans {
        for r in &mut rows[*start..*end] { r.phase = *phase; }
    }
}

pub fn detect_phase(row: &Row, cfg: &AppConfig) -> Phase {
    let rpm   = row.rpm;
    let speed = row.speed_kmh;
    let pedal = row.pedal_pct;
    let load  = row.load_pct;
    let cool  = row.coolant_c;

    if rpm < 50.0   { return Phase::EngineOff; }
    if rpm < 400.0  { return Phase::Cranking; }
    if pedal < cfg.phase_fuel_cut_pedal && load < cfg.phase_fuel_cut_load
        && rpm > cfg.phase_fuel_cut_rpm && speed > cfg.phase_fuel_cut_speed
    {
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
