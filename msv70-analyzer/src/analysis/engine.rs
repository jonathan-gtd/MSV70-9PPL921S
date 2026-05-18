use std::collections::HashMap;
use std::path::Path;

use crate::data::{load_csv, LoadResult};
use crate::data::types::*;
use super::phases::{detect_phase, is_steady};
use super::thresholds::Thresholds as THR;

pub fn percentile(data: &mut Vec<f64>, p: f64) -> f64 {
    if data.is_empty() { return 0.0; }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = (data.len() - 1) as f64 * p / 100.0;
    let lo = idx as usize;
    let hi = (lo + 1).min(data.len() - 1);
    data[lo] + (idx - lo as f64) * (data[hi] - data[lo])
}

pub fn percentile_sorted(data: &[f64], p: f64) -> f64 {
    if data.is_empty() { return 0.0; }
    let idx = (data.len() - 1) as f64 * p / 100.0;
    let lo = idx as usize;
    let hi = (lo + 1).min(data.len() - 1);
    data[lo] + (idx - lo as f64) * (data[hi] - data[lo])
}

fn percentile_of(values: &[f64], p: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    let mut v = values.to_vec();
    percentile(&mut v, p)
}

fn pct_outside(values: &[f64], threshold: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    100.0 * values.iter().filter(|&&v| v.abs() > threshold).count() as f64 / values.len() as f64
}

/// Phase duration accumulator
fn phase_dur(rows: &[Row]) -> HashMap<Phase, f64> {
    let mut d: HashMap<Phase, f64> = HashMap::new();
    for i in 1..rows.len() {
        let dt = (rows[i].time_ms - rows[i - 1].time_ms) / 1000.0;
        if dt > 0.0 && dt < 2.0 {
            *d.entry(rows[i].phase).or_insert(0.0) += dt;
        }
    }
    d
}

pub fn compute_session(path: &Path) -> Result<SessionData, String> {
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?")
        .to_string();

    let LoadResult { mut rows, available } = load_csv(path)?;

    let n = rows.len();
    if n == 0 { return Err("No rows".to_string()); }

    let has_ltft  = rows.iter().any(|r| r.ltft_mul_b1.abs() > NEAR_ZERO || r.ltft_mul_b2.abs() > NEAR_ZERO);
    let has_vanos = *available.get("vanos_act").unwrap_or(&false) && *available.get("vanos_tgt").unwrap_or(&false);
    let has_ign   = *available.get("ign_deg").unwrap_or(&false);
    let has_inj   = *available.get("inj_ms").unwrap_or(&false);
    let has_lmb   = *available.get("lmb_b1").unwrap_or(&false) && *available.get("lmb_b2").unwrap_or(&false);
    let has_iat   = *available.get("iat_c").unwrap_or(&false);

    // Derive phase, bins, CL flags on each row
    for i in 0..n {
        let phase = detect_phase(&rows[i]);
        rows[i].phase    = phase;
        rows[i].rpm_bin  = rpm_bin(rows[i].rpm);
        rows[i].load_bin = load_bin(rows[i].load_pct);
        rows[i].vanos_err = rows[i].vanos_act - rows[i].vanos_tgt;
        rows[i].cl_b1 = rows[i].rpm > 600.0 && rows[i].coolant_c > 55.0
            && rows[i].load_pct > 2.0 && rows[i].stft_b1.abs() > NEAR_ZERO;
        rows[i].cl_b2 = rows[i].rpm > 600.0 && rows[i].coolant_c > 55.0
            && rows[i].load_pct > 2.0 && rows[i].stft_b2.abs() > NEAR_ZERO;
    }

    let dur_s     = rows[n - 1].ts() - rows[0].ts();
    let cool_start = rows[0].coolant_c;
    let cool_end   = rows[n - 1].coolant_c;
    let cool_max   = rows.iter().map(|r| r.coolant_c).fold(f64::NEG_INFINITY, f64::max);

    // Build index subsets
    let cl_b1_idx:   Vec<usize> = (0..n).filter(|&i| rows[i].cl_b1).collect();
    let cl_b2_idx:   Vec<usize> = (0..n).filter(|&i| rows[i].cl_b2).collect();
    let cl_both_idx: Vec<usize> = (0..n).filter(|&i| rows[i].cl_b1 && rows[i].cl_b2).collect();
    let run_idx:     Vec<usize> = (0..n).filter(|&i| rows[i].rpm > 600.0).collect();
    let hot_idx:     Vec<usize> = (0..n).filter(|&i| rows[i].coolant_c > THR::KNOCK_HOT_COOLANT).collect();

    let mut steady_idx: Vec<usize> = Vec::new();
    for i in 1..n {
        let dt = (rows[i].time_ms - rows[i - 1].time_ms) / 1000.0;
        if is_steady(&rows[i], &rows[i - 1], dt) && rows[i].rpm > 800.0 {
            steady_idx.push(i);
        }
    }

    let phase_dur = phase_dur(&rows);

    // STFT 2D map
    let mut cell_b1: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
    let mut cell_b2: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
    for &i in &cl_b1_idx {
        cell_b1.entry((rows[i].rpm_bin, rows[i].load_bin)).or_default().push(rows[i].stft_b1);
    }
    for &i in &cl_b2_idx {
        cell_b2.entry((rows[i].rpm_bin, rows[i].load_bin)).or_default().push(rows[i].stft_b2);
    }

    // Ignition baseline (hot engine only, per cell, min 10 samples)
    let mut ign_cell: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
    if has_ign {
        for &i in &hot_idx {
            if rows[i].ign_deg.abs() > NEAR_ZERO && rows[i].rpm > 600.0 {
                ign_cell.entry((rows[i].rpm_bin, rows[i].load_bin)).or_default().push(rows[i].ign_deg);
            }
        }
    }
    let cell_ign_p50: HashMap<(usize, usize), f64> = ign_cell.into_iter()
        .filter(|(_, v)| v.len() >= 10)
        .map(|(k, mut v)| { let p = percentile(&mut v, 50.0); (k, p) })
        .collect();

    // Tuning corrections
    let n_rb = RPM_BINS.len() - 1;
    let n_lb = LOAD_BINS.len() - 1;
    let mut corrections: Vec<Correction> = Vec::new();
    for i in 0..n_rb {
        for j in 0..n_lb {
            let v1 = cell_b1.get(&(i, j));
            let v2 = cell_b2.get(&(i, j));
            let med1 = v1.filter(|v| v.len() >= 5).map(|v| percentile_of(v, 50.0));
            let med2 = v2.filter(|v| v.len() >= 5).map(|v| percentile_of(v, 50.0));
            if med1.is_none() && med2.is_none() { continue; }
            let vals: Vec<f64> = [med1, med2].iter().flatten().copied().collect();
            let avg_delta = vals.iter().sum::<f64>() / vals.len() as f64;
            if avg_delta.abs() < THR::STFT_NOTE { continue; }
            corrections.push(Correction {
                rpm_idx: i, load_idx: j, med_b1: med1, med_b2: med2,
                avg_delta, factor: 1.0 + avg_delta / 100.0,
            });
        }
    }
    corrections.sort_by(|a, b| b.avg_delta.abs().partial_cmp(&a.avg_delta.abs()).unwrap());

    // STFT 60s buckets
    let stft_buckets = build_stft_buckets(&rows, &cl_b1_idx, &cl_b2_idx);

    // Retard / knock events (indices into rows)
    let retard_idx: Vec<usize> = if has_ign {
        hot_idx.iter().copied()
            .filter(|&i| {
                rows[i].rpm > THR::KNOCK_RPM_MIN
                    && rows[i].load_pct > THR::KNOCK_LOAD_MIN
                    && cell_ign_p50.contains_key(&(rows[i].rpm_bin, rows[i].load_bin))
                    && rows[i].ign_deg < cell_ign_p50[&(rows[i].rpm_bin, rows[i].load_bin)] - THR::KNOCK_DELTA
            })
            .collect()
    } else {
        vec![]
    };

    // Anomaly events
    let events = build_events(&rows, has_lmb, has_vanos, has_ign);

    Ok(SessionData {
        file_name, n_rows: n, dur_s, cool_start, cool_end, cool_max,
        rows, cl_b1_idx, cl_b2_idx, cl_both_idx, run_idx, hot_idx, steady_idx,
        phase_dur, cell_b1, cell_b2, cell_ign_p50,
        corrections, stft_buckets, events, retard_idx,
        has_ltft, has_vanos, has_ign, has_inj, has_lmb, has_iat,
    })
}

fn build_stft_buckets(rows: &[Row], cl_b1: &[usize], cl_b2: &[usize]) -> Vec<StftBucket> {
    if cl_b1.is_empty() && cl_b2.is_empty() { return vec![]; }

    let t0 = {
        let first_b1 = cl_b1.first().map(|&i| rows[i].ts());
        let first_b2 = cl_b2.first().map(|&i| rows[i].ts());
        match (first_b1, first_b2) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None)    => a,
            (None,    Some(b)) => b,
            (None,    None)    => return vec![],
        }
    };

    const BUCKET: f64 = 60.0;
    let mut bkt_b1: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut bkt_b2: HashMap<usize, Vec<usize>> = HashMap::new();
    for &i in cl_b1 {
        bkt_b1.entry(((rows[i].ts() - t0) / BUCKET) as usize).or_default().push(i);
    }
    for &i in cl_b2 {
        bkt_b2.entry(((rows[i].ts() - t0) / BUCKET) as usize).or_default().push(i);
    }

    let mut all_keys: Vec<usize> = bkt_b1.keys().chain(bkt_b2.keys()).copied().collect();
    all_keys.sort_unstable();
    all_keys.dedup();

    all_keys.into_iter().map(|b| {
        let br1 = bkt_b1.get(&b).map(|v| v.as_slice()).unwrap_or(&[]);
        let br2 = bkt_b2.get(&b).map(|v| v.as_slice()).unwrap_or(&[]);
        let all: Vec<usize> = br1.iter().chain(br2.iter()).copied().collect();

        let p50_b1 = if br1.is_empty() { None } else {
            let mut vals: Vec<f64> = br1.iter().map(|&i| rows[i].stft_b1).collect();
            Some(percentile(&mut vals, 50.0))
        };
        let p50_b2 = if br2.is_empty() { None } else {
            let mut vals: Vec<f64> = br2.iter().map(|&i| rows[i].stft_b2).collect();
            Some(percentile(&mut vals, 50.0))
        };

        let rpm  = all.iter().map(|&i| rows[i].rpm).sum::<f64>() / all.len() as f64;
        let load = all.iter().map(|&i| rows[i].load_pct).sum::<f64>() / all.len() as f64;
        let cool = all.iter().map(|&i| rows[i].coolant_c).sum::<f64>() / all.len() as f64;

        StftBucket {
            t_s: t0 + b as f64 * BUCKET,
            p50_b1, p50_b2, n_b1: br1.len(), n_b2: br2.len(),
            rpm, load, cool,
        }
    }).collect()
}

fn build_events(rows: &[Row], has_lmb: bool, has_vanos: bool, _has_ign: bool) -> Vec<AnomalyEvent> {
    let mut events: Vec<AnomalyEvent> = Vec::new();
    let mut stft_b1_streak = 0usize;
    let mut stft_b2_streak = 0usize;
    let mut lean_b1_streak = 0usize;
    let mut lean_b2_streak = 0usize;
    const LEAN_DEBOUNCE: usize = 3;

    for row in rows {
        let t = row.ts();

        if row.cl_b1 {
            let s1 = row.stft_b1;
            if s1.abs() > THR::STFT_ALARM {
                stft_b1_streak += 1;
                if stft_b1_streak == 6 {
                    events.push(AnomalyEvent {
                        kind: "STFT_B1".into(), time_s: t,
                        msg: format!("B1 sustained {:+.1}% (>{:.0}%)  RPM={:.0} Load={:.0}%",
                            s1, THR::STFT_ALARM, row.rpm, row.load_pct),
                    });
                }
            } else { stft_b1_streak = 0; }
        }

        if row.cl_b2 {
            let s2 = row.stft_b2;
            if s2.abs() > THR::STFT_ALARM {
                stft_b2_streak += 1;
                if stft_b2_streak == 6 {
                    events.push(AnomalyEvent {
                        kind: "STFT_B2".into(), time_s: t,
                        msg: format!("B2 sustained {:+.1}% (>{:.0}%)  RPM={:.0} Load={:.0}%",
                            s2, THR::STFT_ALARM, row.rpm, row.load_pct),
                    });
                }
            } else { stft_b2_streak = 0; }
        }

        if has_lmb && row.load_pct > 35.0 && row.coolant_c > 60.0 {
            if row.lmb_b1 > THR::LMB_LEAN_LOAD {
                lean_b1_streak += 1;
                if lean_b1_streak == LEAN_DEBOUNCE {
                    events.push(AnomalyEvent {
                        kind: "LEAN_B1".into(), time_s: t,
                        msg: format!("λB1={:.3} LEAN @ Load={:.0}% RPM={:.0}",
                            row.lmb_b1, row.load_pct, row.rpm),
                    });
                }
            } else { lean_b1_streak = 0; }

            if row.lmb_b2 > THR::LMB_LEAN_LOAD {
                lean_b2_streak += 1;
                if lean_b2_streak == LEAN_DEBOUNCE {
                    events.push(AnomalyEvent {
                        kind: "LEAN_B2".into(), time_s: t,
                        msg: format!("λB2={:.3} LEAN @ Load={:.0}% RPM={:.0}",
                            row.lmb_b2, row.load_pct, row.rpm),
                    });
                }
            } else { lean_b2_streak = 0; }
        }

        if has_lmb && row.cl_b1 && row.lmb_b1 < THR::LMB_RICH && row.stft_b1 < -THR::STFT_WARN {
            events.push(AnomalyEvent {
                kind: "RICH_B1".into(), time_s: t,
                msg: format!("λB1={:.3}  STFT {:+.1}%", row.lmb_b1, row.stft_b1),
            });
        }
        if has_lmb && row.cl_b2 && row.lmb_b2 < THR::LMB_RICH && row.stft_b2 < -THR::STFT_WARN {
            events.push(AnomalyEvent {
                kind: "RICH_B2".into(), time_s: t,
                msg: format!("λB2={:.3}  STFT {:+.1}%", row.lmb_b2, row.stft_b2),
            });
        }

        if has_vanos && row.rpm > 1000.0 && row.vanos_err.abs() > THR::VANOS_STEADY_ALARM * 2.5 {
            events.push(AnomalyEvent {
                kind: "VANOS".into(), time_s: t,
                msg: format!("err={:+.1}°  act={:.0}° tgt={:.0}°  RPM={:.0}",
                    row.vanos_err, row.vanos_act, row.vanos_tgt, row.rpm),
            });
        }

        if row.coolant_c > THR::COOLANT_WARN {
            events.push(AnomalyEvent {
                kind: "OVERTEMP".into(), time_s: t,
                msg: format!("Coolant {:.1}°C", row.coolant_c),
            });
        }
    }

    // Dedup within time windows per type
    let dedup_window: HashMap<&str, f64> = [
        ("STFT_B1", 15.0), ("STFT_B2", 15.0), ("LEAN_B1", 20.0), ("LEAN_B2", 20.0),
        ("RICH_B1", 15.0), ("RICH_B2", 15.0), ("VANOS", 30.0),    ("OVERTEMP", 60.0),
    ].iter().cloned().collect();

    events.sort_by(|a, b| a.time_s.partial_cmp(&b.time_s).unwrap());

    let mut seen: HashMap<(String, i64), bool> = HashMap::new();
    let mut deduped: Vec<AnomalyEvent> = Vec::new();
    for ev in events {
        let w = dedup_window.get(ev.kind.as_str()).copied().unwrap_or(15.0);
        let key = (ev.kind.clone(), (ev.time_s / w) as i64);
        if !seen.contains_key(&key) {
            seen.insert(key, true);
            deduped.push(ev);
        }
    }
    deduped
}
