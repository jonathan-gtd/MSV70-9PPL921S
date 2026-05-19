use crate::config::AppConfig;
use crate::data::types::*;
use super::engine::{percentile, percentile_sorted};

fn fmt_pct(v: f64) -> String {
    format!("{:+.1}%", v)
}

fn flag(v: f64, warn: f64, alarm: f64) -> &'static str {
    if v.abs() >= alarm { " ⛔" }
    else if v.abs() >= warn { " ⚠" }
    else { "" }
}

pub fn generate_report(d: &SessionData, cfg: &AppConfig) -> String {
    let rows    = &d.rows;
    let n       = d.n_rows;
    let run     = &d.run_idx;
    let cl_b1   = &d.cl_b1_idx;
    let cl_b2   = &d.cl_b2_idx;
    let cl_both = &d.cl_both_idx;
    let hot     = &d.hot_idx;

    const W: usize = 72;
    let sep = "═".repeat(W);
    let thn = "─".repeat(W);

    let mut lines: Vec<String> = Vec::new();

    lines.push(sep.clone());
    lines.push(format!("  MSV70 DIAGNOSTIC — E85_v5  N52B30  9PPL921S"));
    lines.push(format!("  File     : {}", d.file_name));
    lines.push(format!("  Duration : {:.0}s  ({:.1} min)  |  {} rows",
        d.dur_s, d.dur_s / 60.0, n));
    lines.push(format!("  Coolant  : {:.0}°C → {:.0}°C  (peak {:.0}°C)",
        d.cool_start, d.cool_end, d.cool_max));
    lines.push(sep.clone());

    lines.push(format!("\n[1]  SESSION OVERVIEW\n{}", thn));

    let rpm_v: Vec<f64>  = run.iter().map(|&i| rows[i].rpm).collect();
    let load_v: Vec<f64> = run.iter().map(|&i| rows[i].load_pct).collect();
    let spd_v: Vec<f64>  = rows.iter().filter(|r| r.speed_kmh > 2.0).map(|r| r.speed_kmh).collect();

    if !rpm_v.is_empty() {
        let mut rv = rpm_v.clone();
        lines.push(format!("  RPM        avg {:.0}  p50 {:.0}  p95 {:.0}  max {:.0}",
            rv.iter().sum::<f64>() / rv.len() as f64,
            percentile(&mut rv, 50.0),
            percentile(&mut rv, 95.0),
            rv.iter().cloned().fold(f64::NEG_INFINITY, f64::max)));
    }
    if !load_v.is_empty() {
        let mut lv = load_v.clone();
        lines.push(format!("  Load       avg {:.1}%  p50 {:.1}%  p95 {:.1}%  max {:.1}%",
            lv.iter().sum::<f64>() / lv.len() as f64,
            percentile(&mut lv, 50.0),
            percentile(&mut lv, 95.0),
            lv.iter().cloned().fold(f64::NEG_INFINITY, f64::max)));
    }
    if !spd_v.is_empty() {
        lines.push(format!("  Speed      avg {:.0}  max {:.0} km/h",
            spd_v.iter().sum::<f64>() / spd_v.len() as f64,
            spd_v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)));
    }

    let b1_cl_pct = 100.0 * cl_b1.len() as f64 / n as f64;
    let b2_cl_pct = 100.0 * cl_b2.len() as f64 / n as f64;
    let cl_note = if (b1_cl_pct - b2_cl_pct).abs() > 15.0 {
        let lesser = if b2_cl_pct < b1_cl_pct { "B2" } else { "B1" };
        format!("  [{lesser} enters CL less often — monitor STFT asymmetry]")
    } else { String::new() };
    lines.push(format!("  CL B1      {:.0}% of rows ({})", b1_cl_pct, cl_b1.len()));
    lines.push(format!("  CL B2      {:.0}% of rows ({}){}", b2_cl_pct, cl_b2.len(), cl_note));

    lines.push(format!("\n[2]  PHASE DISTRIBUTION\n{}", thn));
    lines.push(format!("  {:<14}  {:>6}  {:>5}  {:>6}  {:>8}  {:>9}  {:>9}",
        "Phase", "Dur", "%", "Rows", "RPM avg", "Load avg", "Coolant"));

    for ph in Phase::all() {
        let ph_rows: Vec<usize> = (0..n).filter(|&i| rows[i].phase == *ph).collect();
        if ph_rows.is_empty() { continue; }
        let dur = d.phase_dur.get(ph).copied().unwrap_or(0.0);
        let pct = if d.dur_s > 0.0 { 100.0 * dur / d.dur_s } else { 0.0 };
        let avg_rpm  = ph_rows.iter().map(|&i| rows[i].rpm).sum::<f64>() / ph_rows.len() as f64;
        let avg_load = ph_rows.iter().map(|&i| rows[i].load_pct).sum::<f64>() / ph_rows.len() as f64;
        let avg_cool = ph_rows.iter().map(|&i| rows[i].coolant_c).sum::<f64>() / ph_rows.len() as f64;
        lines.push(format!("  {:<14}  {:>5.0}s  {:>4.0}%  {:>6}  {:>8.0}  {:>8.1}%  {:>8.1}°C",
            ph.as_str(), dur, pct, ph_rows.len(), avg_rpm, avg_load, avg_cool));
    }

    lines.push(format!("\n[3]  STFT — PER PHASE  (each bank filtered to its own CL rows)\n{}", thn));
    lines.push(format!("  {:<14}  {:>7}  {:>7}  {:>7}  {:>5}  {:>7}  {:>7}  {:>7}  {:>5}  {:>6}",
        "Phase", "B1 p5", "B1 p50", "B1 p95", "nB1", "B2 p5", "B2 p50", "B2 p95", "nB2", "Δmed"));

    for ph in Phase::cl_phases() {
        let r1: Vec<f64> = cl_b1.iter().filter(|&&i| rows[i].phase == *ph).map(|&i| rows[i].stft_b1).collect();
        let r2: Vec<f64> = cl_b2.iter().filter(|&&i| rows[i].phase == *ph).map(|&i| rows[i].stft_b2).collect();
        if r1.is_empty() && r2.is_empty() { continue; }

        let s1 = if r1.is_empty() { vec![0.0] } else { r1.clone() };
        let s2 = if r2.is_empty() { vec![0.0] } else { r2.clone() };

        let (p5_1, p50_1, p95_1) = (percentile_of(&s1, 5.0), percentile_of(&s1, 50.0), percentile_of(&s1, 95.0));
        let (p5_2, p50_2, p95_2) = (percentile_of(&s2, 5.0), percentile_of(&s2, 50.0), percentile_of(&s2, 95.0));
        let delta = p50_1 - p50_2;

        let f1 = flag(p50_1, cfg.stft_note, cfg.stft_warn);
        let f2 = flag(p50_2, cfg.stft_note, cfg.stft_warn);
        let fi = flag(delta, cfg.stft_imbalance, cfg.stft_imbalance * 1.5);

        lines.push(format!("  {:<14}  {:>7}  {:>7}{}  {:>7}  {:>5}  {:>7}  {:>7}{}  {:>7}  {:>5}  {:>6}{}",
            ph.as_str(), fmt_pct(p5_1), fmt_pct(p50_1), f1, fmt_pct(p95_1), r1.len(),
            fmt_pct(p5_2), fmt_pct(p50_2), f2, fmt_pct(p95_2), r2.len(), fmt_pct(delta), fi));
    }

    if !cl_b1.is_empty() || !cl_b2.is_empty() {
        let s1_all: Vec<f64> = cl_b1.iter().map(|&i| rows[i].stft_b1).collect();
        let s2_all: Vec<f64> = cl_b2.iter().map(|&i| rows[i].stft_b2).collect();
        let p50_1 = if s1_all.is_empty() { 0.0 } else { percentile_of(&s1_all, 50.0) };
        let p50_2 = if s2_all.is_empty() { 0.0 } else { percentile_of(&s2_all, 50.0) };
        lines.push(thn.clone());
        lines.push(format!("  {:<14}  {:>7}  {:>7}  {:>7}  {:>5}  {:>7}  {:>7}  {:>7}  {:>5}  {:>6}",
            "OVERALL",
            if s1_all.is_empty() { "    ---".into() } else { fmt_pct(percentile_of(&s1_all, 5.0)) },
            fmt_pct(p50_1),
            if s1_all.is_empty() { "    ---".into() } else { fmt_pct(percentile_of(&s1_all, 95.0)) },
            cl_b1.len(),
            if s2_all.is_empty() { "    ---".into() } else { fmt_pct(percentile_of(&s2_all, 5.0)) },
            fmt_pct(p50_2),
            if s2_all.is_empty() { "    ---".into() } else { fmt_pct(percentile_of(&s2_all, 95.0)) },
            cl_b2.len(), fmt_pct(p50_1 - p50_2)));

        let sat_warn_1  = pct_outside_v(&s1_all, cfg.stft_warn);
        let sat_warn_2  = pct_outside_v(&s2_all, cfg.stft_warn);
        let sat_alarm_1 = pct_outside_v(&s1_all, cfg.stft_alarm);
        let sat_alarm_2 = pct_outside_v(&s2_all, cfg.stft_alarm);
        lines.push(format!("\n  Saturation >|{:.0}%| :  B1 {:.1}%  B2 {:.1}%  of their CL rows",
            cfg.stft_warn, sat_warn_1, sat_warn_2));
        lines.push(format!("  Saturation >|{:.0}%| :  B1 {:.1}%  B2 {:.1}%  of their CL rows",
            cfg.stft_alarm, sat_alarm_1, sat_alarm_2));
    }

    lines.push(format!("\n[4]  STFT 2D MAP — RPM × Load  (median, per-bank CL rows only)\n{}", thn));
    lines.push(format!("  <{} rows = ---  |  · = |{:.0}%|  ⚠ = |{:.0}%|  ⛔ = |{:.0}%|",
        cfg.cell_min_samples, cfg.stft_note, cfg.stft_warn, cfg.stft_alarm));
    lines.push("  Negative = rich (reduce ip_mff_cor)  |  Positive = lean (add fuel)".into());

    lines.extend(render_map("STFT B1 — median per cell", &d.cell_b1, &d.rpm_labels, &d.load_labels, cfg));
    lines.extend(render_map("STFT B2 — median per cell", &d.cell_b2, &d.rpm_labels, &d.load_labels, cfg));

    lines.push(format!("\n  TUNING CORRECTIONS — cells where |avg STFT| > {:.0}%", cfg.stft_note));
    lines.push("  Action on ip_mff_cor cells : multiply by correction factor".into());
    lines.push(format!("  {:<14} {:>7} {:>8} {:>8} {:>8}  {:>7}  {}",
        "RPM range", "Load", "B1 med", "B2 med", "avg Δ", "Factor", "Severity"));
    lines.push(format!("  {}", "─".repeat(66)));

    for c in &d.corrections {
        let severity = if c.avg_delta.abs() > cfg.stft_alarm { "⛔ CRITICAL" }
            else if c.avg_delta.abs() > cfg.stft_warn { "⚠  HIGH   " }
            else { "·  notable" };
        let s1 = c.med_b1.map(|v| fmt_pct(v)).unwrap_or_else(|| "    ---".into());
        let s2 = c.med_b2.map(|v| fmt_pct(v)).unwrap_or_else(|| "    ---".into());
        let rpm_lbl  = d.rpm_labels.get(c.rpm_idx).map(|s| s.as_str()).unwrap_or("?");
        let load_lbl = d.load_labels.get(c.load_idx).map(|s| s.as_str()).unwrap_or("?");
        lines.push(format!("  {:<14} {:>7} {:>8}  {:>8}  {:>8}  ×{:.3}   {}",
            rpm_lbl, load_lbl, s1, s2, fmt_pct(c.avg_delta), c.factor, severity));
    }
    if d.corrections.is_empty() {
        lines.push(format!("  No cells with |STFT| > {:.0}% — map well calibrated in covered range", cfg.stft_note));
    }

    lines.push(format!("\n[5]  LTFT — LONG-TERM FUEL TRIM\n{}", thn));
    if d.has_ltft && !run.is_empty() {
        let mul1 = percentile_of(&run.iter().map(|&i| rows[i].ltft_mul_b1).collect::<Vec<_>>(), 50.0);
        let mul2 = percentile_of(&run.iter().map(|&i| rows[i].ltft_mul_b2).collect::<Vec<_>>(), 50.0);
        let add1 = percentile_of(&run.iter().map(|&i| rows[i].ltft_add_b1).collect::<Vec<_>>(), 50.0);
        let add2 = percentile_of(&run.iter().map(|&i| rows[i].ltft_add_b2).collect::<Vec<_>>(), 50.0);
        let imbal = mul1 - mul2;
        let f_m1 = flag(mul1, cfg.ltft_warn, cfg.ltft_alarm);
        let f_m2 = flag(mul2, cfg.ltft_warn, cfg.ltft_alarm);
        let f_im = flag(imbal, cfg.ltft_imbalance, cfg.ltft_imbalance * 1.5);
        lines.push(format!("  Multiplicative:  B1 {}{}   B2 {}{}   Δ = {}{}",
            fmt_pct(mul1), f_m1, fmt_pct(mul2), f_m2, fmt_pct(imbal), f_im));
        lines.push(format!("  Additive:        B1 {:+.3} mg/stk   B2 {:+.3} mg/stk", add1, add2));
        let ceiling = 20.0;
        lines.push(format!("  Ceiling margin:  B1 {:.1}% left   B2 {:.1}% left  (ceiling ±{:.0}%)",
            ceiling - mul1.abs(), ceiling - mul2.abs(), ceiling));
    } else {
        lines.push("  LTFT = 0  — no adaptation yet (reset or session too short)".into());
        lines.push("  → Need 30–40+ min of CL operation to accumulate meaningful LTFT".into());
    }

    if d.has_inj {
        lines.push(format!("\n[8]  INJECTION — DUTY CYCLE\n{}", thn));
        lines.push("  DC formula: inj_ms / (120000/RPM) × 100  [4-stroke, 1 fire per 720°]".into());
        let duties: Vec<f64> = run.iter()
            .filter_map(|&i| rows[i].inj_duty())
            .collect();
        if !duties.is_empty() {
            let d_avg = duties.iter().sum::<f64>() / duties.len() as f64;
            let d_p95 = percentile_of(&duties, 95.0);
            let d_max = duties.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let f_dc  = flag(d_max, cfg.inj_duty_warn, cfg.inj_duty_alarm);
            lines.push(format!("  Overall :  avg {:.1}%  p95 {:.1}%  max {:.1}%{}", d_avg, d_p95, d_max, f_dc));
        }
    }

    lines.push(format!("\n[9]  VANOS — INLET CAMSHAFT TRACKING\n{}", thn));
    if d.has_vanos {
        let v_act: Vec<f64> = rows.iter()
            .filter(|r| r.vanos_tgt.abs() > 1.0 && r.rpm > 800.0)
            .map(|r| r.vanos_err)
            .collect();
        if !v_act.is_empty() {
            let mean_e = v_act.iter().sum::<f64>() / v_act.len() as f64;
            let p50_e  = percentile_of(&v_act, 50.0);
            let abs_e: Vec<f64> = v_act.iter().map(|v| v.abs()).collect();
            let p95_e  = percentile_of(&abs_e, 95.0);
            let max_e  = abs_e.iter().cloned().fold(0.0_f64, f64::max);
            let f_ss   = flag(mean_e, cfg.vanos_steady_warn, cfg.vanos_steady_alarm);
            lines.push(format!("  All active (n={}):", v_act.len()));
            lines.push(format!("    mean {:+.2}°{}  p50 {:+.2}°  p95 |{:.1}°|  max |{:.1}°|",
                mean_e, f_ss, p50_e, p95_e, max_e));
        } else {
            lines.push("  No VANOS active samples".into());
        }
    } else {
        lines.push("  VANOS columns not present in this log".into());
    }

    lines.push(format!("\n[10]  IGNITION — ADVANCE & KNOCK PROXY\n{}", thn));
    lines.push(format!("  Knock detection: hot engine only (>{:.0}°C), RPM>{:.0}, Load>{:.0}%, >{:.0}° below local p50",
        cfg.knock_hot_coolant, cfg.knock_rpm_min, cfg.knock_load_min, cfg.knock_delta));
    if d.has_ign {
        let ign_run: Vec<f64> = run.iter()
            .filter(|&&i| rows[i].ign_deg.abs() > cfg.near_zero)
            .map(|&i| rows[i].ign_deg)
            .collect();
        if !ign_run.is_empty() {
            let mean_i = ign_run.iter().sum::<f64>() / ign_run.len() as f64;
            let p50_i  = percentile_of(&ign_run, 50.0);
            let p95_i  = percentile_of(&ign_run, 95.0);
            let max_i  = ign_run.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            lines.push(format!("\n  Overall:  avg {:+.1}°  p50 {:+.1}°  p95 {:+.1}°  max {:+.1}°",
                mean_i, p50_i, p95_i, max_i));
        }
        let knock_cands: Vec<usize> = hot.iter().copied()
            .filter(|&i| rows[i].rpm > cfg.knock_rpm_min && rows[i].load_pct > cfg.knock_load_min
                && d.cell_ign_p50.contains_key(&(rows[i].rpm_bin, rows[i].load_bin)))
            .collect();
        let ret_pct = if knock_cands.is_empty() { 0.0 }
            else { 100.0 * d.retard_idx.len() as f64 / knock_cands.len() as f64 };
        lines.push(format!("  Retard events: {}/{}  = {:.1}%  (hot, RPM>{:.0}, Load>{:.0}%)",
            d.retard_idx.len(), knock_cands.len(), ret_pct, cfg.knock_rpm_min, cfg.knock_load_min));
        if ret_pct > 5.0 {
            lines.push("  ⚠ Significant retard activity — check fuel quality".into());
        } else if ret_pct > 1.0 {
            lines.push("  · Some retard events — transients, monitor".into());
        } else {
            lines.push("  Clean — no sustained knock retard detected ✓".into());
        }
    } else {
        lines.push("  Ignition column not present".into());
    }

    lines.push(format!("\n[11]  ANOMALY EVENT TIMELINE\n{}", thn));
    if d.events.is_empty() {
        lines.push("  No anomaly events detected.".into());
    } else {
        for ev in d.events.iter().take(60) {
            lines.push(format!("  t={:>7.1}s  [{:<10}]  {}", ev.time_s, ev.kind, ev.msg));
        }
        if d.events.len() > 60 {
            lines.push(format!("  … {} more events (same types)", d.events.len() - 60));
        }
    }

    lines.push(format!("\n{}", sep));
    lines.push("  VERDICT".into());
    lines.push(thn.clone());

    let (mut issues, mut warnings, mut oks) = (vec![], vec![], vec![]);

    if !cl_b1.is_empty() {
        let vals: Vec<f64> = cl_b1.iter().map(|&i| rows[i].stft_b1).collect();
        let med1 = percentile_of(&vals, 50.0);
        let sat1 = pct_outside_v(&vals, cfg.stft_alarm);
        if sat1 > 5.0 { issues.push(format!("STFT B1 {:.1}% rows >|{:.0}%| — map richness off", sat1, cfg.stft_alarm)); }
        else if med1.abs() > cfg.stft_warn { warnings.push(format!("STFT B1 median {:+.1}% — elevated, adjust ip_mff_cor", med1)); }
        else { oks.push(format!("STFT B1 median {:+.1}% — OK", med1)); }
    }
    if !cl_b2.is_empty() {
        let vals: Vec<f64> = cl_b2.iter().map(|&i| rows[i].stft_b2).collect();
        let med2 = percentile_of(&vals, 50.0);
        let sat2 = pct_outside_v(&vals, cfg.stft_alarm);
        if sat2 > 5.0 { issues.push(format!("STFT B2 {:.1}% rows >|{:.0}%| — map richness off", sat2, cfg.stft_alarm)); }
        else if med2.abs() > cfg.stft_warn { warnings.push(format!("STFT B2 median {:+.1}% — elevated, adjust ip_mff_cor", med2)); }
        else { oks.push(format!("STFT B2 median {:+.1}% — OK", med2)); }
    }
    if !cl_both.is_empty() {
        let delta_med = percentile_of(
            &cl_both.iter().map(|&i| rows[i].stft_b1 - rows[i].stft_b2).collect::<Vec<_>>(), 50.0);
        if delta_med.abs() > cfg.stft_imbalance { issues.push(format!("B1/B2 persistent gap {:+.1}% — sensor or mechanical", delta_med)); }
        else if delta_med.abs() > 4.0 { warnings.push(format!("B1/B2 slight asymmetry {:+.1}%", delta_med)); }
    }

    if d.has_ltft && !run.is_empty() {
        let mul1 = percentile_of(&run.iter().map(|&i| rows[i].ltft_mul_b1).collect::<Vec<_>>(), 50.0);
        let mul2 = percentile_of(&run.iter().map(|&i| rows[i].ltft_mul_b2).collect::<Vec<_>>(), 50.0);
        if mul1.abs() > cfg.ltft_alarm || mul2.abs() > cfg.ltft_alarm {
            issues.push(format!("LTFT near ceiling: B1={:+.1}% B2={:+.1}% — map wrong", mul1, mul2));
        } else if mul1.abs() > cfg.ltft_warn || mul2.abs() > cfg.ltft_warn {
            warnings.push(format!("LTFT elevated: B1={:+.1}% B2={:+.1}%", mul1, mul2));
        } else {
            oks.push(format!("LTFT: B1={:+.1}% B2={:+.1}% — OK", mul1, mul2));
        }
    } else {
        oks.push("LTFT = 0 (not yet adapted)".into());
    }

    if d.cool_max > cfg.coolant_warn { issues.push(format!("Coolant {:.0}°C peak — check cooling system", d.cool_max)); }
    else { oks.push(format!("Coolant OK (max {:.0}°C)", d.cool_max)); }

    let status = if !issues.is_empty() { "⛔ CRITICAL" }
        else if !warnings.is_empty() { "⚠  WARNINGS" }
        else { "✓  OK      " };
    lines.push(format!("  STATUS : {}", status));

    if !issues.is_empty() {
        lines.push(String::new());
        for x in &issues   { lines.push(format!("  ✗ [ISSUE]   {}", x)); }
    }
    if !warnings.is_empty() {
        lines.push(String::new());
        for x in &warnings { lines.push(format!("  ⚠ [WARN]    {}", x)); }
    }
    if !oks.is_empty() {
        lines.push(String::new());
        for x in &oks      { lines.push(format!("  ✓ [OK]      {}", x)); }
    }

    lines.push(sep);
    lines.push(String::new());
    lines.join("\n")
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn percentile_of(values: &[f64], p: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    percentile_sorted(&v, p)
}

fn pct_outside_v(values: &[f64], threshold: f64) -> f64 {
    if values.is_empty() { return 0.0; }
    100.0 * values.iter().filter(|&&v| v.abs() > threshold).count() as f64 / values.len() as f64
}

fn render_map(
    label: &str,
    cell: &std::collections::HashMap<(usize, usize), Vec<f64>>,
    rpm_labels: &[String],
    load_labels: &[String],
    cfg: &AppConfig,
) -> Vec<String> {
    let n_rb = rpm_labels.len();
    let n_lb = load_labels.len();
    const COL_W: usize = 13;
    let mut out = vec![format!("\n  {}", label)];
    let header = format!("  RPM \\ Load  {}",
        load_labels.iter().map(|l| format!("{:>COL_W$}", l)).collect::<String>());
    out.push(header);
    out.push(format!("  {}", "─".repeat(13 + COL_W * n_lb)));
    for i in 0..n_rb {
        let mut row_line = format!("  {:<12} ", rpm_labels[i]);
        let mut any_data = false;
        for j in 0..n_lb {
            if let Some(vals) = cell.get(&(i, j)) {
                if vals.len() >= cfg.cell_min_samples {
                    let med = percentile_of(vals, 50.0);
                    any_data = true;
                    let tag = if med.abs() > cfg.stft_alarm { "⛔" }
                        else if med.abs() > cfg.stft_warn { "⚠" }
                        else if med.abs() > cfg.stft_note { "·" }
                        else { "" };
                    let s = format!("{:+.1}%({}){}",  med, vals.len(), tag);
                    row_line.push_str(&format!("{:>COL_W$}", s));
                } else {
                    row_line.push_str(&format!("{:>COL_W$}", "---"));
                }
            } else {
                row_line.push_str(&format!("{:>COL_W$}", "---"));
            }
        }
        if any_data { out.push(row_line); }
    }
    out
}
