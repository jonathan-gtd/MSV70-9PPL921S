#!/usr/bin/env python3
"""
MSV70 Diagnostic Log Analyzer
Analyzes CSV logs from the custom MSV70_DIAG TestO job.
Usage: python analyze_diag.py [file.csv]
       If no file given, uses the most recent CSV in ../LOGS/
"""

import sys
import os
import csv
import math
import statistics
from pathlib import Path
from collections import defaultdict

# Force UTF-8 output on Windows
if sys.platform == "win32":
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

# ─── column aliases (map various TestO label variants to internal names) ────────

COL_ALIASES = {
    "time_ms":      ["time"],
    "rpm":          ["RPM", "rpm"],
    "load_pct":     ["Load %", "load %"],
    "pedal_pct":    ["Pedal %", "pedal %"],
    "speed_kmh":    ["Speed km/h", "speed km/h"],
    "coolant_c":    ["Cool degC", "cool degc"],
    "stft_b1":      ["STFTGrp1", "STFT B1", "stft b1"],
    "stft_b2":      ["STFTGrp2", "STFT B2", "stft b2"],
    "ltft_add_b1":  ["LTFT add. B1", "ltft add. b1"],
    "ltft_add_b2":  ["LTFT add. B2", "ltft add. b2"],
    "ltft_mul_b1":  ["LTFT Mul B1", "ltft mul b1"],
    "ltft_mul_b2":  ["LTFT Mul B2", "ltft mul b2"],
    "lmb_b1":       ["Lmb Act B1", "lmb act b1"],
    "lmb_b2":       ["Lmb Act B2", "lmb act b2"],
    "lmb_tgt_b1":   ["Lmb Tgt B1", "lmb tgt b1"],
    "maf_kgh":      ["AirMass", "MAF kg/h", "maf kg/h"],
    "inj_ms":       ["Inj C1 ms", "inj c1 ms"],
    "vanos_act":    ["Vanos InAct", "vanos inact"],
    "vanos_tgt":    ["Vanos InTgt", "vanos intgt"],
    "ign_deg":      ["Ign Cyl1 deg", "ign cyl1 deg"],
    "iat_c":        ["IAT degC", "iat degc"],
    "oil_c":        ["Oil degC", "oil degc"],
}

NEAR_ZERO = 1e-5  # float noise threshold

# ─── thresholds ─────────────────────────────────────────────────────────────────

THR = {
    "stft_warn":        15.0,   # % — sustained deviation worth noting
    "stft_alarm":       22.0,   # % — approaching ±40% saturation ceiling
    "stft_imbalance":   10.0,   # % — |B1-B2| alerting inter-bank gap
    "ltft_warn":        12.0,   # % multiplicative LTFT
    "ltft_alarm":       18.0,   # % — close to ±20% ceiling
    "ltft_imbalance":   15.0,   # % |B1-B2|
    "lmb_lean_load":    1.04,   # lambda — lean alarm under load
    "lmb_rich":         0.93,   # lambda — rich alarm
    "vanos_err_warn":   8.0,    # °CRK tracking error
    "vanos_err_alarm":  15.0,
    "ign_retard_warn":  -12.0,  # °CRK — knock-proxy (ignition retard events)
    "coolant_warn":     105.0,  # °C
}

# ─── phases ─────────────────────────────────────────────────────────────────────

PHASES = ["ENGINE_OFF", "CRANKING", "WARMUP", "HOT_IDLE", "IDLE",
          "CRUISE", "ACCEL", "WOT", "FUEL_CUT"]


def detect_phase(row):
    rpm    = row["rpm"]
    speed  = row["speed_kmh"]
    pedal  = row["pedal_pct"]
    load   = row["load_pct"]
    cool   = row["coolant_c"]

    if rpm < 50:
        return "ENGINE_OFF"
    if rpm < 400:
        return "CRANKING"
    if cool < 80 and speed < 5 and rpm < 1500:
        return "WARMUP"
    if pedal < 5 and rpm > 1200 and speed > 5:
        return "FUEL_CUT"
    if pedal > 85 or load > 72:
        return "WOT"
    if pedal > 40:
        return "ACCEL"
    if speed < 5 and rpm < 1200:
        return "HOT_IDLE" if cool >= 80 else "WARMUP"
    return "CRUISE"


def is_cl_active(row):
    """Closed-loop fuel control active heuristic."""
    return (row["rpm"] > 600
            and row["coolant_c"] > 55
            and row["load_pct"] > 2
            and abs(row["stft_b1"]) > NEAR_ZERO)


# ─── helpers ────────────────────────────────────────────────────────────────────

def safe(v):
    """Return 0 for near-zero float noise."""
    return 0.0 if abs(v) < NEAR_ZERO else v


def fmt(v, decimals=2, unit=""):
    sign = "+" if v > 0 else ""
    return f"{sign}{v:.{decimals}f}{unit}"


def bar(value, lo, hi, width=20):
    """ASCII bar chart between lo and hi."""
    frac = max(0.0, min(1.0, (value - lo) / (hi - lo)))
    filled = int(round(frac * width))
    return "[" + "█" * filled + "░" * (width - filled) + "]"


def pstats(values):
    """Return (mean, std, min, max) or None."""
    if not values:
        return None
    n = len(values)
    mu = sum(values) / n
    if n > 1:
        std = statistics.stdev(values)
    else:
        std = 0.0
    return mu, std, min(values), max(values)


def load_csv(path):
    with open(path, encoding="utf-8") as f:
        reader = csv.DictReader(f, delimiter=";")
        raw = list(reader)
    if not raw:
        raise ValueError("Empty CSV")

    # Build column map
    headers_lower = {h.lower(): h for h in raw[0].keys()}
    col_map = {}
    for internal, aliases in COL_ALIASES.items():
        for alias in aliases:
            if alias.lower() in headers_lower:
                col_map[internal] = headers_lower[alias.lower()]
                break

    required = ["time_ms", "rpm"]
    for r in required:
        if r not in col_map:
            raise ValueError(f"Missing required column '{r}' in {path.name}")

    rows = []
    for raw_row in raw:
        row = {}
        for internal, header in col_map.items():
            try:
                row[internal] = float(raw_row[header])
            except (ValueError, KeyError):
                row[internal] = 0.0
        # Apply safe() to all fuel-trim and lambda columns
        for key in ["stft_b1", "stft_b2", "ltft_add_b1", "ltft_add_b2",
                    "ltft_mul_b1", "ltft_mul_b2"]:
            if key in row:
                row[key] = safe(row[key])
        rows.append(row)

    # Provide defaults for optional columns
    optional_defaults = {
        "stft_b1": 0.0, "stft_b2": 0.0,
        "ltft_add_b1": 0.0, "ltft_add_b2": 0.0,
        "ltft_mul_b1": 0.0, "ltft_mul_b2": 0.0,
        "lmb_b1": 1.0, "lmb_b2": 1.0, "lmb_tgt_b1": 1.0,
        "maf_kgh": 0.0, "inj_ms": 0.0,
        "vanos_act": 0.0, "vanos_tgt": 0.0,
        "ign_deg": 0.0, "load_pct": 0.0,
        "pedal_pct": 0.0, "speed_kmh": 0.0,
        "coolant_c": 20.0, "iat_c": 20.0, "oil_c": 0.0,
    }
    for row in rows:
        for key, default in optional_defaults.items():
            if key not in row:
                row[key] = default

    return rows, col_map


def time_s(row):
    return row["time_ms"] / 1000.0


# ─── analysis ───────────────────────────────────────────────────────────────────

def analyze(path):
    rows, col_map = load_csv(path)
    available = set(col_map.keys())

    for row in rows:
        row["phase"] = detect_phase(row)
        row["cl_active"] = is_cl_active(row)
        row["vanos_err"] = row.get("vanos_act", 0) - row.get("vanos_tgt", 0)

    duration_s = time_s(rows[-1]) - time_s(rows[0])
    n_rows = len(rows)
    coolant_start = rows[0]["coolant_c"]
    coolant_end   = rows[-1]["coolant_c"]
    coolant_max   = max(r["coolant_c"] for r in rows)

    # ── Phase buckets ────────────────────────────────────────────────────────
    phase_rows = defaultdict(list)
    for row in rows:
        phase_rows[row["phase"]].append(row)

    # Estimate phase durations using timestamp deltas
    phase_dur = defaultdict(float)
    for i in range(1, n_rows):
        dt = (rows[i]["time_ms"] - rows[i-1]["time_ms"]) / 1000.0
        if dt > 2.0:
            dt = 0.1  # skip big gaps (logging paused)
        phase_dur[rows[i]["phase"]] += dt

    # ── CL-active rows only for fuel trim ───────────────────────────────────
    cl_rows = [r for r in rows if r["cl_active"]]
    cl_phase = defaultdict(list)
    for row in cl_rows:
        cl_phase[row["phase"]].append(row)

    # ── LTFT check ──────────────────────────────────────────────────────────
    ltft_rows = [r for r in rows if r["rpm"] > 600]
    ltft_mul_b1_vals = [r["ltft_mul_b1"] for r in ltft_rows]
    ltft_mul_b2_vals = [r["ltft_mul_b2"] for r in ltft_rows]
    ltft_add_b1_vals = [r["ltft_add_b1"] for r in ltft_rows]
    ltft_add_b2_vals = [r["ltft_add_b2"] for r in ltft_rows]

    has_ltft = any(abs(v) > NEAR_ZERO for v in ltft_mul_b1_vals + ltft_mul_b2_vals)

    # ── VANOS ────────────────────────────────────────────────────────────────
    has_vanos = "vanos_act" in available and "vanos_tgt" in available
    vanos_rows = [r for r in rows if r["rpm"] > 800 and abs(r["vanos_tgt"]) > 1]

    # ── Ignition ─────────────────────────────────────────────────────────────
    has_ign = "ign_deg" in available
    ign_rows = [r for r in rows if r["rpm"] > 600 and abs(r["ign_deg"]) > NEAR_ZERO]

    # ── Injection ────────────────────────────────────────────────────────────
    has_inj = "inj_ms" in available

    # ── Lambda ───────────────────────────────────────────────────────────────
    has_lmb = "lmb_b1" in available and "lmb_b2" in available
    # Under-load lambda (load > 35%, CL active)
    load_lmb_rows = [r for r in rows if r["cl_active"] and r["load_pct"] > 35
                     and r["lmb_b1"] > 0.5]

    # ── Anomaly events ───────────────────────────────────────────────────────
    events = []

    prev_stft_b1_hi = False
    prev_stft_b2_hi = False
    stft_alarm_b1_streak = 0
    stft_alarm_b2_streak = 0

    for i, row in enumerate(rows):
        t = time_s(row)
        if not row["cl_active"]:
            stft_alarm_b1_streak = 0
            stft_alarm_b2_streak = 0
            continue

        s1, s2 = row["stft_b1"], row["stft_b2"]

        # STFT sustained alarm (>5 consecutive CL samples above threshold)
        if abs(s1) > THR["stft_alarm"]:
            stft_alarm_b1_streak += 1
            if stft_alarm_b1_streak == 5:
                events.append(("STFT_B1", t, f"STFT B1 sustained {s1:+.1f}%  [ALARM >{THR['stft_alarm']:.0f}%]"))
        else:
            stft_alarm_b1_streak = 0

        if abs(s2) > THR["stft_alarm"]:
            stft_alarm_b2_streak += 1
            if stft_alarm_b2_streak == 5:
                events.append(("STFT_B2", t, f"STFT B2 sustained {s2:+.1f}%  [ALARM >{THR['stft_alarm']:.0f}%]"))
        else:
            stft_alarm_b2_streak = 0

        # Instantaneous imbalance > threshold
        delta = s1 - s2
        if abs(delta) > THR["stft_imbalance"] + 5 and row["load_pct"] > 20:
            events.append(("IMBALANCE", t, f"B1-B2 gap {delta:+.1f}%  (load {row['load_pct']:.0f}%)"))

        # Lambda under load
        if has_lmb and row["load_pct"] > 35:
            if row["lmb_b1"] > THR["lmb_lean_load"]:
                events.append(("LEAN_B1", t, f"Lambda B1 {row['lmb_b1']:.3f} LEAN under load {row['load_pct']:.0f}%"))
            if row["lmb_b2"] > THR["lmb_lean_load"]:
                events.append(("LEAN_B2", t, f"Lambda B2 {row['lmb_b2']:.3f} LEAN under load {row['load_pct']:.0f}%"))
            if row["lmb_b1"] < THR["lmb_rich"] and row["stft_b1"] < -THR["stft_warn"]:
                events.append(("RICH_B1", t, f"Lambda B1 {row['lmb_b1']:.3f} RICH, STFT {row['stft_b1']:+.1f}%"))

        # VANOS
        if has_vanos and abs(row["vanos_err"]) > THR["vanos_err_alarm"] and row["rpm"] > 1000:
            events.append(("VANOS", t, f"Vanos err {row['vanos_err']:+.1f}° CRK  (act {row['vanos_act']:.0f} tgt {row['vanos_tgt']:.0f})"))

        # Ignition retard (knock proxy)
        if has_ign and row["ign_deg"] < THR["ign_retard_warn"] and row["rpm"] > 1500:
            events.append(("KNOCK_PROXY", t, f"Ign {row['ign_deg']:+.1f}° CRK retard  RPM={row['rpm']:.0f} Load={row['load_pct']:.0f}%"))

    # Coolant overtemp
    for row in rows:
        if row["coolant_c"] > THR["coolant_warn"]:
            events.append(("OVERTEMP", time_s(row),
                           f"Coolant {row['coolant_c']:.1f}°C  RPM={row['rpm']:.0f}"))
            break

    # Deduplicate events: keep first per type per 10s window
    deduped = []
    last_t = {}
    for ev_type, t, msg in sorted(events, key=lambda e: e[1]):
        key = (ev_type, int(t / 10))
        if key not in last_t:
            last_t[key] = t
            deduped.append((ev_type, t, msg))
    events = deduped

    # ── Build report ─────────────────────────────────────────────────────────
    W = 66
    sep = "═" * W
    thin = "─" * W

    def hdr(n, title):
        return f"\n[{n}]  {title}\n{thin}"

    lines = []
    lines.append(sep)
    lines.append(f"  MSV70 DIAGNOSTIC REPORT — E85_v5 / N52B30 9PPL921S")
    lines.append(f"  File     : {path.name}")
    lines.append(f"  Duration : {duration_s:.0f}s  ({duration_s/60:.1f} min)  |  {n_rows} rows")
    lines.append(f"  Coolant  : {coolant_start:.0f}°C → {coolant_end:.0f}°C  (peak {coolant_max:.0f}°C)")
    lines.append(sep)

    # 1 ── Session overview
    lines.append(hdr(1, "SESSION OVERVIEW"))
    rpm_vals = [r["rpm"] for r in rows if r["rpm"] > 200]
    load_vals = [r["load_pct"] for r in rows if r["rpm"] > 600]
    speed_vals = [r["speed_kmh"] for r in rows if r["speed_kmh"] > 2]
    if rpm_vals:
        lines.append(f"  RPM       : avg {statistics.mean(rpm_vals):.0f}  max {max(rpm_vals):.0f}  "
                     f"  {bar(statistics.mean(rpm_vals), 600, 5500)} (600–5500)")
    if load_vals:
        lines.append(f"  Load      : avg {statistics.mean(load_vals):.1f}%  max {max(load_vals):.1f}%"
                     f"  {bar(statistics.mean(load_vals), 0, 100)}")
    if speed_vals:
        lines.append(f"  Speed     : avg {statistics.mean(speed_vals):.0f}  max {max(speed_vals):.0f} km/h"
                     f"  {bar(statistics.mean(speed_vals), 0, 130)}")
    if has_inj:
        inj_run = [r["inj_ms"] for r in rows if r["rpm"] > 600 and r["inj_ms"] > 0.5]
        if inj_run:
            lines.append(f"  Inj C1    : avg {statistics.mean(inj_run):.2f} ms  max {max(inj_run):.2f} ms")
    if "iat_c" in available:
        iat_vals = [r["iat_c"] for r in rows if r["rpm"] > 200]
        if iat_vals:
            lines.append(f"  IAT       : {min(iat_vals):.0f}°C → {max(iat_vals):.0f}°C")
    cl_pct = 100 * len(cl_rows) / n_rows if n_rows else 0
    lines.append(f"  CL-loop   : {cl_pct:.0f}% of session ({len(cl_rows)} rows)")

    # 2 ── Phase distribution
    lines.append(hdr(2, "PHASE DISTRIBUTION"))
    lines.append(f"  {'Phase':<14}  {'Dur':>6}  {'%':>5}  {'Rows':>6}  {'Avg RPM':>8}  {'Avg Load':>9}")
    active_phases = [p for p in PHASES if phase_rows[p]]
    for ph in active_phases:
        pr = phase_rows[ph]
        dur = phase_dur[ph]
        pct = 100 * dur / duration_s if duration_s else 0
        avg_rpm  = statistics.mean(r["rpm"] for r in pr) if pr else 0
        avg_load = statistics.mean(r["load_pct"] for r in pr) if pr else 0
        lines.append(f"  {ph:<14}  {dur:>5.0f}s  {pct:>4.0f}%  {len(pr):>6}  {avg_rpm:>8.0f}  {avg_load:>8.1f}%")

    # 3 ── Fuel trims per phase
    lines.append(hdr(3, "FUEL TRIMS — STFT per phase  (closed-loop rows only)"))
    lines.append(f"  {'Phase':<14}  {'STFT B1 µ±σ':>14}  {'STFT B2 µ±σ':>14}  {'Δ(B1-B2)':>9}  {'Sat%B1':>7}  {'Sat%B2':>7}")
    alarms_ft = []
    for ph in ["WARMUP", "HOT_IDLE", "IDLE", "CRUISE", "ACCEL", "WOT", "FUEL_CUT"]:
        pr = cl_phase[ph]
        if not pr:
            continue
        s1v = [r["stft_b1"] for r in pr]
        s2v = [r["stft_b2"] for r in pr]
        mu1 = statistics.mean(s1v); sd1 = statistics.stdev(s1v) if len(s1v) > 1 else 0
        mu2 = statistics.mean(s2v); sd2 = statistics.stdev(s2v) if len(s2v) > 1 else 0
        delta = mu1 - mu2
        sat1 = 100 * sum(1 for v in s1v if abs(v) > THR["stft_alarm"]) / len(s1v)
        sat2 = 100 * sum(1 for v in s2v if abs(v) > THR["stft_alarm"]) / len(s2v)

        flags = ""
        if abs(mu1) > THR["stft_warn"] or sat1 > 5:
            flags += " ⚠B1"
        if abs(mu2) > THR["stft_warn"] or sat2 > 5:
            flags += " ⚠B2"
        if abs(delta) > THR["stft_imbalance"]:
            flags += " ⚠IMBAL"
        if abs(mu1) > THR["stft_alarm"] or abs(mu2) > THR["stft_alarm"]:
            flags += " ⛔ALARM"

        lines.append(f"  {ph:<14}  {fmt(mu1,2,'%'):>6}±{sd1:4.1f}%  "
                     f"{fmt(mu2,2,'%'):>6}±{sd2:4.1f}%  "
                     f"{fmt(delta,1,'%'):>9}  {sat1:>6.1f}%  {sat2:>6.1f}%{flags}")
        if flags:
            alarms_ft.append((ph, mu1, mu2, delta))

    # Overall CL stats
    if cl_rows:
        s1_all = [r["stft_b1"] for r in cl_rows]
        s2_all = [r["stft_b2"] for r in cl_rows]
        mu1 = statistics.mean(s1_all); sd1 = statistics.stdev(s1_all) if len(s1_all) > 1 else 0
        mu2 = statistics.mean(s2_all); sd2 = statistics.stdev(s2_all) if len(s2_all) > 1 else 0
        lines.append(thin)
        lines.append(f"  {'OVERALL (CL)':<14}  {fmt(mu1,2,'%'):>6}±{sd1:4.1f}%  "
                     f"{fmt(mu2,2,'%'):>6}±{sd2:4.1f}%  {fmt(mu1-mu2,1,'%'):>9}")

    # LTFT
    lines.append("")
    lines.append(f"  LTFT Multiplicative:")
    if has_ltft:
        mul1 = statistics.mean(ltft_mul_b1_vals) if ltft_mul_b1_vals else 0
        mul2 = statistics.mean(ltft_mul_b2_vals) if ltft_mul_b2_vals else 0
        add1 = statistics.mean(ltft_add_b1_vals) if ltft_add_b1_vals else 0
        add2 = statistics.mean(ltft_add_b2_vals) if ltft_add_b2_vals else 0
        ltft_imbal = abs(mul1 - mul2)
        flag_ltft = ""
        if ltft_imbal > THR["ltft_imbalance"]:
            flag_ltft = "  ⚠⚠ ASYMMETRY"
        elif ltft_imbal > 8:
            flag_ltft = "  ⚠ imbalance"
        if abs(mul1) > THR["ltft_alarm"] or abs(mul2) > THR["ltft_alarm"]:
            flag_ltft += "  ⛔ AT LIMIT"
        elif abs(mul1) > THR["ltft_warn"] or abs(mul2) > THR["ltft_warn"]:
            flag_ltft += "  ⚠ HIGH"
        lines.append(f"    B1 = {fmt(mul1,2,'%')}   B2 = {fmt(mul2,2,'%')}   Δ = {fmt(mul1-mul2,1,'%')}{flag_ltft}")
        lines.append(f"  LTFT Additive:  B1 = {add1:+.3f} mg/stk   B2 = {add2:+.3f} mg/stk")
    else:
        lines.append("    [no LTFT data — values are 0 / session too short for adaptation]")

    # 4 ── Lambda
    lines.append(hdr(4, "LAMBDA ANALYSIS"))
    if has_lmb:
        # Overall CL lambda
        lmb1_cl = [r["lmb_b1"] for r in cl_rows if 0.5 < r["lmb_b1"] < 2.0]
        lmb2_cl = [r["lmb_b2"] for r in cl_rows if 0.5 < r["lmb_b2"] < 2.0]
        if lmb1_cl:
            mu1 = statistics.mean(lmb1_cl); sd1 = statistics.stdev(lmb1_cl) if len(lmb1_cl)>1 else 0
            lines.append(f"  CL Lambda B1  : {mu1:.4f} ± {sd1:.4f}  "
                         f"  range [{min(lmb1_cl):.3f} – {max(lmb1_cl):.3f}]")
        if lmb2_cl:
            mu2 = statistics.mean(lmb2_cl); sd2 = statistics.stdev(lmb2_cl) if len(lmb2_cl)>1 else 0
            lines.append(f"  CL Lambda B2  : {mu2:.4f} ± {sd2:.4f}  "
                         f"  range [{min(lmb2_cl):.3f} – {max(lmb2_cl):.3f}]")

        # Under-load lambda
        if load_lmb_rows:
            ll1 = [r["lmb_b1"] for r in load_lmb_rows if 0.5 < r["lmb_b1"] < 2.0]
            ll2 = [r["lmb_b2"] for r in load_lmb_rows if 0.5 < r["lmb_b2"] < 2.0]
            if ll1:
                mu = statistics.mean(ll1)
                flag = "  ⚠ LEAN" if mu > THR["lmb_lean_load"] else ("  ⚠ RICH" if mu < THR["lmb_rich"] else "  OK")
                lines.append(f"  Load>35% λ B1 : {mu:.4f} (n={len(ll1)}){flag}")
            if ll2:
                mu = statistics.mean(ll2)
                flag = "  ⚠ LEAN" if mu > THR["lmb_lean_load"] else ("  ⚠ RICH" if mu < THR["lmb_rich"] else "  OK")
                lines.append(f"  Load>35% λ B2 : {mu:.4f} (n={len(ll2)}){flag}")

        # Decel lambda (fuel cut → open loop)
        decel_rows = [r for r in rows if r["phase"] == "FUEL_CUT"]
        if decel_rows:
            l1 = [r["lmb_b1"] for r in decel_rows if r["lmb_b1"] > 0.5]
            l2 = [r["lmb_b2"] for r in decel_rows if r["lmb_b2"] > 0.5]
            if l1:
                lines.append(f"  Fuel-cut λ B1 : avg {statistics.mean(l1):.3f}  "
                             f"range [{min(l1):.3f}–{max(l1):.3f}]  (open loop)")
            if l2:
                lines.append(f"  Fuel-cut λ B2 : avg {statistics.mean(l2):.3f}  "
                             f"range [{min(l2):.3f}–{max(l2):.3f}]")

        # Lambda target tracking
        if "lmb_tgt_b1" in available:
            lmb_tgt_rows = [r for r in cl_rows if abs(r["lmb_tgt_b1"]) > NEAR_ZERO]
            if lmb_tgt_rows:
                errs = [r["lmb_b1"] - r["lmb_tgt_b1"] for r in lmb_tgt_rows]
                lines.append(f"  λ B1 tracking error: {statistics.mean(errs):+.4f} avg, "
                             f"±{statistics.stdev(errs) if len(errs)>1 else 0:.4f} std")
    else:
        lines.append("  [no lambda columns found]")

    # 5 ── VANOS & Ignition
    lines.append(hdr(5, "VANOS & IGNITION"))
    if has_vanos:
        if vanos_rows:
            errs = [r["vanos_err"] for r in vanos_rows]
            abs_errs = [abs(e) for e in errs]
            mu_err = statistics.mean(errs)
            max_err = max(abs_errs)
            flag_v = ""
            if max_err > THR["vanos_err_alarm"]:
                flag_v = "  ⚠ TRACKING ALARM"
            elif max_err > THR["vanos_err_warn"]:
                flag_v = "  ⚠ tracking warn"
            lines.append(f"  VANOS error   : avg {mu_err:+.2f}°  max |{max_err:.1f}°|{flag_v}")
            act_vals = [r["vanos_act"] for r in vanos_rows]
            tgt_vals = [r["vanos_tgt"] for r in vanos_rows]
            lines.append(f"  Actual range  : {min(act_vals):.0f}° – {max(act_vals):.0f}°")
            lines.append(f"  Target range  : {min(tgt_vals):.0f}° – {max(tgt_vals):.0f}°")

            # Correlation check
            tracking_ok = sum(1 for e in abs_errs if e < THR["vanos_err_warn"]) / len(abs_errs)
            lines.append(f"  Tracking OK   : {tracking_ok*100:.1f}% of active samples within ±{THR['vanos_err_warn']:.0f}°")
        else:
            lines.append("  VANOS: no active samples (target=0 for all rows?)")
    else:
        lines.append("  VANOS: columns not present in this log")

    if has_ign:
        if ign_rows:
            ign_vals = [r["ign_deg"] for r in ign_rows]
            mu_ign = statistics.mean(ign_vals)
            # Retard events vs mean (knock proxy: >8° below mean)
            retard_threshold = mu_ign - 8.0
            retard_events = [r for r in ign_rows if r["ign_deg"] < retard_threshold and r["rpm"] > 1500]
            retard_pct = 100 * len(retard_events) / len(ign_rows)
            lines.append(f"\n  Ignition C1   : avg {mu_ign:+.1f}°  range [{min(ign_vals):+.1f}° – {max(ign_vals):+.1f}°]")
            if retard_pct > 0.5:
                lines.append(f"  Knock retard  : {len(retard_events)} events ({retard_pct:.1f}%)  threshold < {retard_threshold:+.1f}°  ⚠")
            else:
                lines.append(f"  Knock retard  : {len(retard_events)} events ({retard_pct:.1f}%)  — clean")

            # By phase
            lines.append(f"  {'Phase':<14}  {'Ign avg':>8}  {'Ign min':>8}  {'n':>6}")
            for ph in ["IDLE", "HOT_IDLE", "CRUISE", "ACCEL", "WOT"]:
                ph_ign = [r["ign_deg"] for r in ign_rows if r["phase"] == ph]
                if ph_ign:
                    lines.append(f"  {ph:<14}  {statistics.mean(ph_ign):>+7.1f}°  {min(ph_ign):>+7.1f}°  {len(ph_ign):>6}")
    else:
        lines.append("\n  Ignition: column not present in this log")

    # 6 ── STFT time evolution (buckets of 60s)
    lines.append(hdr(6, "STFT EVOLUTION OVER TIME  (60s buckets, CL rows only)"))
    if cl_rows:
        t0 = time_s(cl_rows[0])
        t_end = time_s(cl_rows[-1])
        bucket_size = 60.0
        n_buckets = max(1, int((t_end - t0) / bucket_size) + 1)
        buckets = defaultdict(list)
        for row in cl_rows:
            b = int((time_s(row) - t0) / bucket_size)
            buckets[b].append(row)

        lines.append(f"  {'t(s)':>6}  {'STFT B1':>8}  {'STFT B2':>8}  {'Δ':>7}  {'RPM':>6}  {'Load':>6}  {'Coolant':>8}")
        for b in sorted(buckets.keys()):
            br = buckets[b]
            t_mid = t0 + b * bucket_size
            s1_b = [r["stft_b1"] for r in br]
            s2_b = [r["stft_b2"] for r in br]
            mu1 = statistics.mean(s1_b)
            mu2 = statistics.mean(s2_b)
            delta = mu1 - mu2
            avg_rpm  = statistics.mean(r["rpm"] for r in br)
            avg_load = statistics.mean(r["load_pct"] for r in br)
            avg_cool = statistics.mean(r["coolant_c"] for r in br)
            flag = ""
            if abs(mu1) > THR["stft_warn"] or abs(mu2) > THR["stft_warn"]:
                flag = " ⚠"
            if abs(mu1) > THR["stft_alarm"] or abs(mu2) > THR["stft_alarm"]:
                flag = " ⛔"
            lines.append(f"  {t_mid:>6.0f}s  {fmt(mu1,1,'%'):>8}  {fmt(mu2,1,'%'):>8}  "
                         f"{fmt(delta,1,'%'):>7}  {avg_rpm:>6.0f}  {avg_load:>5.1f}%  {avg_cool:>7.1f}°C{flag}")
    else:
        lines.append("  [no closed-loop rows in this log]")

    # 7 ── Anomaly events log
    lines.append(hdr(7, "ANOMALY EVENT LOG"))
    if events:
        for ev_type, t, msg in events[:50]:
            lines.append(f"  t={t:>7.1f}s  [{ev_type:<14}]  {msg}")
        if len(events) > 50:
            lines.append(f"  ... ({len(events) - 50} more events truncated)")
    else:
        lines.append("  No anomaly events detected.")

    # 8 ── Verdict
    lines.append(f"\n{sep}")
    lines.append("  VERDICT")
    lines.append(thin)

    issues = []
    oks = []

    # STFT
    if cl_rows:
        s1_all = [r["stft_b1"] for r in cl_rows]
        s2_all = [r["stft_b2"] for r in cl_rows]
        mu1 = statistics.mean(s1_all); mu2 = statistics.mean(s2_all)
        if abs(mu1) > THR["stft_alarm"]:
            issues.append(f"STFT B1 avg {mu1:+.1f}% — approaching saturation, check fueling")
        elif abs(mu1) > THR["stft_warn"]:
            issues.append(f"STFT B1 avg {mu1:+.1f}% — elevated, tune may need adjustment")
        else:
            oks.append(f"STFT B1 avg {mu1:+.1f}% — within normal range")
        if abs(mu2) > THR["stft_alarm"]:
            issues.append(f"STFT B2 avg {mu2:+.1f}% — approaching saturation, check fueling")
        elif abs(mu2) > THR["stft_warn"]:
            issues.append(f"STFT B2 avg {mu2:+.1f}% — elevated")
        else:
            oks.append(f"STFT B2 avg {mu2:+.1f}% — within normal range")
        delta = mu1 - mu2
        if abs(delta) > THR["stft_imbalance"]:
            issues.append(f"B1/B2 imbalance {delta:+.1f}% — suspect O2 sensor or mechanical issue")

    # LTFT
    if has_ltft:
        mul1 = statistics.mean(ltft_mul_b1_vals)
        mul2 = statistics.mean(ltft_mul_b2_vals)
        if abs(mul1) > THR["ltft_alarm"] or abs(mul2) > THR["ltft_alarm"]:
            issues.append(f"LTFT Mul at ceiling: B1={mul1:+.1f}% B2={mul2:+.1f}% — map enrichment OFF")
        elif abs(mul1) > THR["ltft_warn"] or abs(mul2) > THR["ltft_warn"]:
            issues.append(f"LTFT Mul elevated: B1={mul1:+.1f}% B2={mul2:+.1f}% — consider map revision")
        else:
            oks.append(f"LTFT Mul: B1={mul1:+.1f}% B2={mul2:+.1f}% — OK")
        if abs(mul1 - mul2) > THR["ltft_imbalance"]:
            issues.append(f"LTFT asymmetry {mul1-mul2:+.1f}% — sensor or mechanical fault suspected")
    else:
        oks.append("LTFT: 0 (no adaptation yet or just reset)")

    # Coolant
    if coolant_max > THR["coolant_warn"]:
        issues.append(f"Coolant peaked at {coolant_max:.0f}°C — check cooling system")
    else:
        oks.append(f"Coolant OK (max {coolant_max:.0f}°C)")

    # Knock
    if has_ign and ign_rows:
        ign_vals = [r["ign_deg"] for r in ign_rows]
        mu_ign = statistics.mean(ign_vals)
        retard_events_count = sum(1 for r in ign_rows
                                  if r["ign_deg"] < mu_ign - 8.0 and r["rpm"] > 1500)
        if retard_events_count > 5:
            issues.append(f"Ignition retard events: {retard_events_count} — possible knock activity")
        else:
            oks.append(f"Ignition: no significant retard events ({retard_events_count})")

    if issues:
        lines.append(f"  STATUS: ⚠  {len(issues)} issue(s) found\n")
        for issue in issues:
            lines.append(f"  ✗ {issue}")
    else:
        lines.append(f"  STATUS: OK  No critical issues found\n")

    if oks:
        lines.append("")
        for ok in oks:
            lines.append(f"  ✓ {ok}")

    lines.append(sep)
    lines.append("")

    return "\n".join(lines)


# ─── entry point ────────────────────────────────────────────────────────────────

def find_latest_csv():
    logs_dir = Path(__file__).parent.parent / "LOGS"
    csvs = sorted(logs_dir.glob("*.csv"), key=lambda p: p.stat().st_mtime, reverse=True)
    if not csvs:
        # Fallback: check old/
        csvs = sorted((logs_dir / "old").glob("*.csv"),
                      key=lambda p: p.stat().st_mtime, reverse=True)
    if not csvs:
        raise FileNotFoundError(f"No CSV files found in {logs_dir}")
    return csvs[0]


def main():
    if len(sys.argv) > 1:
        path = Path(sys.argv[1])
        if not path.is_absolute():
            path = Path.cwd() / path
    else:
        path = find_latest_csv()
        print(f"Auto-selected: {path.name}")

    if not path.exists():
        print(f"Error: file not found: {path}", file=sys.stderr)
        sys.exit(1)

    try:
        report = analyze(path)
    except Exception as e:
        print(f"Error analyzing {path}: {e}", file=sys.stderr)
        import traceback; traceback.print_exc()
        sys.exit(1)

    print(report)


if __name__ == "__main__":
    main()
