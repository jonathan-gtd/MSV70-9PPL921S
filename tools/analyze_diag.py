#!/usr/bin/env python3
"""
MSV70 Diagnostic Log Analyzer — deep precision mode
Analyzes CSV logs from the custom MSV70_DIAG TestO job.

Usage:
    python analyze_diag.py [file.csv]
    If no file given, uses the most recent CSV in ../LOGS/
"""

import sys
import os
import csv
import math
import statistics
from pathlib import Path
from collections import defaultdict

if sys.platform == "win32":
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

# ─── Column aliases ─────────────────────────────────────────────────────────────
COL_ALIASES = {
    "time_ms":      ["time"],
    "rpm":          ["RPM"],
    "load_pct":     ["Load %"],
    "pedal_pct":    ["Pedal %"],
    "speed_kmh":    ["Speed km/h"],
    "coolant_c":    ["Cool degC"],
    "stft_b1":      ["STFTGrp1", "STFT B1"],
    "stft_b2":      ["STFTGrp2", "STFT B2"],
    "ltft_add_b1":  ["LTFT add. B1"],
    "ltft_add_b2":  ["LTFT add. B2"],
    "ltft_mul_b1":  ["LTFT Mul B1"],
    "ltft_mul_b2":  ["LTFT Mul B2"],
    "lmb_b1":       ["Lmb Act B1"],
    "lmb_b2":       ["Lmb Act B2"],
    "lmb_tgt_b1":   ["Lmb Tgt B1"],
    "maf_kgh":      ["AirMass", "MAF kg/h"],
    "inj_ms":       ["Inj C1 ms"],
    "vanos_act":    ["Vanos InAct"],
    "vanos_tgt":    ["Vanos InTgt"],
    "ign_deg":      ["Ign Cyl1 deg"],
    "iat_c":        ["IAT degC"],
    "oil_c":        ["Oil degC"],
}

NEAR_ZERO = 1e-5

# ─── Thresholds ─────────────────────────────────────────────────────────────────
THR = {
    "stft_note":         8.0,   # % — worth noting
    "stft_warn":        15.0,   # % — sustained deviation
    "stft_alarm":       22.0,   # % — approaching ±40% saturation ceiling
    "stft_imbalance":    8.0,   # % — |B1-B2| persistent gap
    "ltft_warn":        12.0,   # % — multiplicative LTFT
    "ltft_alarm":       18.0,   # % — close to ±20% ceiling
    "ltft_imbalance":   12.0,   # % — |B1-B2|
    "lmb_lean_load":    1.04,   # λ — lean alarm under load
    "lmb_rich":         0.93,   # λ — rich alarm
    "vanos_steady_warn": 5.0,   # °CRK — steady-state tracking
    "vanos_steady_alarm":10.0,
    "inj_duty_warn":    80.0,   # % duty cycle
    "coolant_warn":    114.0,   # °C — N52 thermostat 112°C normal
}

# ─── RPM / Load bins for 2D map ─────────────────────────────────────────────────
RPM_BINS  = [0, 800, 1200, 1600, 2000, 2500, 3000, 3800, 5000, 9999]
LOAD_BINS = [0, 10,  20,   30,   40,   50,   60,   75,   100]

RPM_LABELS  = ["<800", "800-1200", "1200-1600", "1600-2000",
               "2000-2500", "2500-3000", "3000-3800", "3800-5000", "5000+"]
LOAD_LABELS = ["0-10", "10-20", "20-30", "30-40",
               "40-50", "50-60", "60-75", "75-100"]

# ─── Phases ─────────────────────────────────────────────────────────────────────
PHASES = ["ENGINE_OFF", "CRANKING", "WARMUP", "HOT_IDLE", "IDLE",
          "CRUISE", "ACCEL", "WOT", "FUEL_CUT"]

CL_PHASES = ["HOT_IDLE", "IDLE", "CRUISE", "ACCEL", "WOT"]


def rpm_bin(rpm):
    for i in range(len(RPM_BINS) - 1):
        if RPM_BINS[i] <= rpm < RPM_BINS[i + 1]:
            return i
    return len(RPM_BINS) - 2


def load_bin(load):
    for i in range(len(LOAD_BINS) - 1):
        if LOAD_BINS[i] <= load < LOAD_BINS[i + 1]:
            return i
    return len(LOAD_BINS) - 2


def detect_phase(row):
    rpm   = row["rpm"]
    speed = row["speed_kmh"]
    pedal = row["pedal_pct"]
    load  = row["load_pct"]
    cool  = row["coolant_c"]

    if rpm < 50:
        return "ENGINE_OFF"
    if rpm < 400:
        return "CRANKING"
    if pedal < 5 and rpm > 1200 and speed > 5:
        return "FUEL_CUT"
    if pedal > 85 or load > 72:
        return "WOT"
    if pedal > 40:
        return "ACCEL"
    if speed < 5 and rpm < 1200:
        return "HOT_IDLE" if cool >= 80 else "WARMUP"
    if cool < 80:
        return "WARMUP"
    return "CRUISE"


def is_cl_active(row):
    return (row["rpm"] > 600
            and row["coolant_c"] > 55
            and row["load_pct"] > 2
            and abs(row["stft_b1"]) > NEAR_ZERO)


def is_steady(row, prev_row, dt_s):
    """True if engine not in rapid transient (for VANOS steady-state analysis)."""
    if dt_s <= 0 or dt_s > 1.5:
        return False
    drpm  = abs(row["rpm"]   - prev_row["rpm"])   / dt_s
    dload = abs(row["load_pct"] - prev_row["load_pct"]) / dt_s
    return drpm < 300 and dload < 20


# ─── Data loading ────────────────────────────────────────────────────────────────
def safe(v):
    return 0.0 if abs(v) < NEAR_ZERO else v


def load_csv(path):
    with open(path, encoding="utf-8") as f:
        reader = csv.DictReader(f, delimiter=";")
        raw = list(reader)
    if not raw:
        raise ValueError("Empty CSV")

    headers_lower = {h.lower(): h for h in raw[0].keys()}
    col_map = {}
    for internal, aliases in COL_ALIASES.items():
        for alias in aliases:
            if alias.lower() in headers_lower:
                col_map[internal] = headers_lower[alias.lower()]
                break

    for req in ["time_ms", "rpm"]:
        if req not in col_map:
            raise ValueError(f"Missing required column '{req}' — is this an MSV70_DIAG log?")

    optional_defaults = {
        "stft_b1": 0.0, "stft_b2": 0.0,
        "ltft_add_b1": 0.0, "ltft_add_b2": 0.0,
        "ltft_mul_b1": 0.0, "ltft_mul_b2": 0.0,
        "lmb_b1": 1.0, "lmb_b2": 1.0, "lmb_tgt_b1": 1.0,
        "maf_kgh": 0.0, "inj_ms": 0.0,
        "vanos_act": 0.0, "vanos_tgt": 0.0, "ign_deg": 0.0,
        "load_pct": 0.0, "pedal_pct": 0.0, "speed_kmh": 0.0,
        "coolant_c": 20.0, "iat_c": 20.0, "oil_c": 0.0,
    }

    rows = []
    for raw_row in raw:
        row = {}
        for internal, header in col_map.items():
            try:
                row[internal] = float(raw_row[header])
            except (ValueError, KeyError):
                row[internal] = 0.0
        for key in ["stft_b1", "stft_b2", "ltft_add_b1", "ltft_add_b2",
                    "ltft_mul_b1", "ltft_mul_b2"]:
            if key in row:
                row[key] = safe(row[key])
        for key, default in optional_defaults.items():
            row.setdefault(key, default)
        rows.append(row)

    return rows, col_map


def ts(row):
    return row["time_ms"] / 1000.0


# ─── Statistical helpers ─────────────────────────────────────────────────────────
def percentile(data, p):
    if not data:
        return 0.0
    s = sorted(data)
    idx = (len(s) - 1) * p / 100.0
    lo, hi = int(idx), min(int(idx) + 1, len(s) - 1)
    return s[lo] + (idx - lo) * (s[hi] - s[lo])


def pct_above(data, threshold):
    if not data:
        return 0.0
    return 100.0 * sum(1 for v in data if v > threshold) / len(data)


def pct_below(data, threshold):
    if not data:
        return 0.0
    return 100.0 * sum(1 for v in data if v < threshold) / len(data)


def fmt_pct(v, w=7):
    sign = "+" if v > 0 else ""
    return f"{sign}{v:.1f}%".rjust(w)


def fmt_val(v, dec=2, w=8):
    sign = "+" if v > 0 else ""
    return f"{sign}{v:.{dec}f}".rjust(w)


def flag(v, warn, alarm, invert=False):
    av = abs(v) if not invert else v
    if abs(av) >= alarm:
        return " ⛔"
    if abs(av) >= warn:
        return " ⚠"
    return ""


def phase_dur_s(rows):
    """Compute per-phase duration in seconds from timestamp deltas."""
    d = defaultdict(float)
    for i in range(1, len(rows)):
        dt = (rows[i]["time_ms"] - rows[i-1]["time_ms"]) / 1000.0
        if 0 < dt < 2.0:
            d[rows[i]["phase"]] += dt
    return d


# ─── Main analysis ───────────────────────────────────────────────────────────────
def analyze(path):
    rows, col_map = load_csv(path)
    available = set(col_map.keys())

    for row in rows:
        row["phase"]     = detect_phase(row)
        row["cl_active"] = is_cl_active(row)
        row["vanos_err"] = row["vanos_act"] - row["vanos_tgt"]
        row["rb"]        = rpm_bin(row["rpm"])
        row["lb"]        = load_bin(row["load_pct"])

    n      = len(rows)
    dur_s  = ts(rows[-1]) - ts(rows[0])
    t_cool_start = rows[0]["coolant_c"]
    t_cool_end   = rows[-1]["coolant_c"]
    t_cool_max   = max(r["coolant_c"] for r in rows)

    phase_rows = defaultdict(list)
    for row in rows:
        phase_rows[row["phase"]].append(row)

    p_dur = phase_dur_s(rows)

    cl_rows  = [r for r in rows if r["cl_active"]]
    run_rows = [r for r in rows if r["rpm"] > 600]

    # Steady-state rows for VANOS
    steady_rows = []
    for i in range(1, n):
        dt = (rows[i]["time_ms"] - rows[i-1]["time_ms"]) / 1000.0
        if is_steady(rows[i], rows[i-1], dt) and rows[i]["rpm"] > 800:
            steady_rows.append(rows[i])

    has_ltft = any(abs(r["ltft_mul_b1"]) > NEAR_ZERO or abs(r["ltft_mul_b2"]) > NEAR_ZERO
                   for r in run_rows)
    has_vanos = "vanos_act" in available and "vanos_tgt" in available
    has_ign   = "ign_deg" in available
    has_inj   = "inj_ms" in available
    has_lmb   = "lmb_b1" in available and "lmb_b2" in available

    W   = 72
    SEP = "═" * W
    THN = "─" * W

    def H(n, title):
        return f"\n[{n}]  {title}\n{THN}"

    lines = []

    # ════════════════════════════════════════════════════════════════════════════
    # Header
    # ════════════════════════════════════════════════════════════════════════════
    lines += [
        SEP,
        f"  MSV70 DIAGNOSTIC — E85_v5  N52B30  9PPL921S",
        f"  File     : {path.name}",
        f"  Duration : {dur_s:.0f}s  ({dur_s/60:.1f} min)  |  {n} rows",
        f"  Coolant  : {t_cool_start:.0f}°C → {t_cool_end:.0f}°C  (peak {t_cool_max:.0f}°C)",
        SEP,
    ]

    # ════════════════════════════════════════════════════════════════════════════
    # 1 — Session overview
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(1, "SESSION OVERVIEW"))

    rpm_v  = [r["rpm"]       for r in run_rows]
    load_v = [r["load_pct"]  for r in run_rows]
    spd_v  = [r["speed_kmh"] for r in rows if r["speed_kmh"] > 2]

    if rpm_v:
        lines.append(f"  RPM        avg {statistics.mean(rpm_v):.0f}  "
                     f"p95 {percentile(rpm_v,95):.0f}  max {max(rpm_v):.0f}")
    if load_v:
        lines.append(f"  Load       avg {statistics.mean(load_v):.1f}%  "
                     f"p95 {percentile(load_v,95):.1f}%  max {max(load_v):.1f}%")
    if spd_v:
        lines.append(f"  Speed      avg {statistics.mean(spd_v):.0f}  max {max(spd_v):.0f} km/h")
    if has_inj:
        inj_v = [r["inj_ms"] for r in run_rows if r["inj_ms"] > 0.5]
        if inj_v:
            lines.append(f"  Inj C1     avg {statistics.mean(inj_v):.2f} ms  "
                         f"max {max(inj_v):.2f} ms")
    if "iat_c" in available:
        iat_v = [r["iat_c"] for r in run_rows]
        if iat_v:
            lines.append(f"  IAT        {min(iat_v):.0f}–{max(iat_v):.0f}°C")
    cl_pct = 100 * len(cl_rows) / n if n else 0
    lines.append(f"  CL active  {cl_pct:.0f}%  ({len(cl_rows)} rows / {n} total)")

    # ════════════════════════════════════════════════════════════════════════════
    # 2 — Phase distribution
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(2, "PHASE DISTRIBUTION"))
    lines.append(f"  {'Phase':<14}  {'Dur':>6}  {'%':>5}  {'Rows':>6}  "
                 f"{'RPM avg':>8}  {'Load avg':>9}  {'Coolant':>9}")

    for ph in PHASES:
        pr = phase_rows[ph]
        if not pr:
            continue
        dur = p_dur.get(ph, 0)
        pct = 100 * dur / dur_s if dur_s else 0
        avg_rpm  = statistics.mean(r["rpm"]       for r in pr)
        avg_load = statistics.mean(r["load_pct"]  for r in pr)
        avg_cool = statistics.mean(r["coolant_c"] for r in pr)
        lines.append(f"  {ph:<14}  {dur:>5.0f}s  {pct:>4.0f}%  {len(pr):>6}  "
                     f"{avg_rpm:>8.0f}  {avg_load:>8.1f}%  {avg_cool:>8.1f}°C")

    # ════════════════════════════════════════════════════════════════════════════
    # 3 — STFT deep dive
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(3, "STFT — PER PHASE  (closed-loop rows only, percentiles)"))

    hdr3 = (f"  {'Phase':<14}  {'B1 p5':>7}  {'B1 p50':>7}  {'B1 p95':>7}  "
            f"{'B2 p5':>7}  {'B2 p50':>7}  {'B2 p95':>7}  {'Δmed':>6}  {'n':>5}")
    lines.append(hdr3)

    for ph in CL_PHASES:
        pr = [r for r in cl_rows if r["phase"] == ph]
        if not pr:
            continue
        s1 = [r["stft_b1"] for r in pr]
        s2 = [r["stft_b2"] for r in pr]
        p5_1  = percentile(s1,  5); p50_1 = percentile(s1, 50); p95_1 = percentile(s1, 95)
        p5_2  = percentile(s2,  5); p50_2 = percentile(s2, 50); p95_2 = percentile(s2, 95)
        delta_med = p50_1 - p50_2

        f1 = flag(p50_1, THR["stft_note"], THR["stft_warn"])
        f2 = flag(p50_2, THR["stft_note"], THR["stft_warn"])
        fi = flag(delta_med, THR["stft_imbalance"], THR["stft_imbalance"] * 1.5)

        lines.append(f"  {ph:<14}  "
                     f"{fmt_pct(p5_1)}  {fmt_pct(p50_1)}{f1}  {fmt_pct(p95_1)}  "
                     f"{fmt_pct(p5_2)}  {fmt_pct(p50_2)}{f2}  {fmt_pct(p95_2)}  "
                     f"{fmt_pct(delta_med,6)}{fi}  {len(pr):>5}")

    # Overall CL
    if cl_rows:
        s1_all = [r["stft_b1"] for r in cl_rows]
        s2_all = [r["stft_b2"] for r in cl_rows]
        lines.append(THN)
        lines.append(f"  {'OVERALL':<14}  "
                     f"{fmt_pct(percentile(s1_all,5))}  "
                     f"{fmt_pct(percentile(s1_all,50))}  "
                     f"{fmt_pct(percentile(s1_all,95))}  "
                     f"{fmt_pct(percentile(s2_all,5))}  "
                     f"{fmt_pct(percentile(s2_all,50))}  "
                     f"{fmt_pct(percentile(s2_all,95))}  "
                     f"{fmt_pct(percentile(s1_all,50)-percentile(s2_all,50),6)}  "
                     f"{len(cl_rows):>5}")

        # Saturation info
        sat1_w = pct_above(s1_all,  THR["stft_warn"]) + pct_below(s1_all, -THR["stft_warn"])
        sat1_a = pct_above(s1_all,  THR["stft_alarm"]) + pct_below(s1_all, -THR["stft_alarm"])
        sat2_w = pct_above(s2_all,  THR["stft_warn"]) + pct_below(s2_all, -THR["stft_warn"])
        sat2_a = pct_above(s2_all,  THR["stft_alarm"]) + pct_below(s2_all, -THR["stft_alarm"])
        lines.append(f"\n  Saturation >|{THR['stft_warn']:.0f}%|:  "
                     f"B1 {sat1_w:.1f}%  B2 {sat2_w:.1f}%  of CL rows")
        lines.append(f"  Saturation >|{THR['stft_alarm']:.0f}%|:  "
                     f"B1 {sat1_a:.1f}%  B2 {sat2_a:.1f}%  of CL rows")

    # ─── STFT temporal drift ─────────────────────────────────────────────────
    lines.append(f"\n  STFT DRIFT OVER TIME  (60s buckets — shows LTFT learning direction)")
    lines.append(f"  {'t(s)':>6}  {'B1 med':>7}  {'B2 med':>7}  {'Δ':>7}  "
                 f"{'RPM':>6}  {'Load':>6}  {'Cool':>7}  {'n':>5}")

    if cl_rows:
        t0_cl  = ts(cl_rows[0])
        BUCKET = 60.0
        buckets = defaultdict(list)
        for r in cl_rows:
            buckets[int((ts(r) - t0_cl) / BUCKET)].append(r)

        for b in sorted(buckets.keys()):
            br = buckets[b]
            t_mid = t0_cl + b * BUCKET
            s1_b  = [r["stft_b1"] for r in br]
            s2_b  = [r["stft_b2"] for r in br]
            mu1   = percentile(s1_b, 50)
            mu2   = percentile(s2_b, 50)
            delta = mu1 - mu2
            avg_rpm  = statistics.mean(r["rpm"]       for r in br)
            avg_load = statistics.mean(r["load_pct"]  for r in br)
            avg_cool = statistics.mean(r["coolant_c"] for r in br)
            f1 = " ⚠" if abs(mu1) > THR["stft_warn"] else ""
            f2 = " ⚠" if abs(mu2) > THR["stft_warn"] else ""
            lines.append(f"  {t_mid:>6.0f}s  "
                         f"{fmt_pct(mu1)}{f1}  {fmt_pct(mu2)}{f2}  "
                         f"{fmt_pct(delta,7)}  "
                         f"{avg_rpm:>6.0f}  {avg_load:>5.1f}%  "
                         f"{avg_cool:>6.1f}°C  {len(br):>5}")

    # ════════════════════════════════════════════════════════════════════════════
    # 4 — STFT 2D map (RPM × Load)
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(4, "STFT 2D MAP — RPM × Load  (median B1 | median B2 | n)"))
    lines.append("  Cells with <5 rows shown as  ---  |  Bold = actionable offset")
    lines.append("  Negative = running rich (map too fat) | Positive = running lean")

    # Build cells
    cell_b1 = defaultdict(list)
    cell_b2 = defaultdict(list)
    for r in cl_rows:
        key = (r["rb"], r["lb"])
        cell_b1[key].append(r["stft_b1"])
        cell_b2[key].append(r["stft_b2"])

    n_rb = len(RPM_BINS) - 1
    n_lb = len(LOAD_BINS) - 1

    col_w = 13

    def render_map(label, cell_dict):
        out = [f"\n  {label}"]
        # Header: load bins
        header = "  RPM \\ Load  " + "".join(f"{LOAD_LABELS[j]:>{col_w}}" for j in range(n_lb))
        out.append(header)
        out.append("  " + "─" * (13 + col_w * n_lb))
        for i in range(n_rb):
            row_line = f"  {RPM_LABELS[i]:<12} "
            any_data = False
            for j in range(n_lb):
                vals = cell_dict.get((i, j), [])
                if len(vals) < 5:
                    row_line += f"{'---':>{col_w}}"
                else:
                    med = percentile(vals, 50)
                    any_data = True
                    cell_str = f"{'+' if med>=0 else ''}{med:.1f}%({len(vals)})"
                    tag = ""
                    if abs(med) > THR["stft_alarm"]:
                        tag = "⛔"
                    elif abs(med) > THR["stft_warn"]:
                        tag = "⚠"
                    elif abs(med) > THR["stft_note"]:
                        tag = "·"
                    row_line += f"{cell_str+tag:>{col_w}}"
            if any_data:
                out.append(row_line)
        return out

    lines += render_map("STFT B1 (median per cell)", cell_b1)
    lines += render_map("STFT B2 (median per cell)", cell_b2)

    # Correction table: cells where |median| > 8%
    lines.append(f"\n  TUNING DELTAS — cells requiring ip_mff_cor adjustment  (|STFT| > {THR['stft_note']:.0f}%)")
    lines.append(f"  {'RPM range':<14} {'Load range':<12} {'STFT B1':>8} {'STFT B2':>8} "
                 f"{'avg Δ':>8}  {'Action'}")
    lines.append("  " + "─" * 64)
    any_corrections = False
    for i in range(n_rb):
        for j in range(n_lb):
            v1 = cell_b1.get((i, j), [])
            v2 = cell_b2.get((i, j), [])
            if len(v1) < 5 and len(v2) < 5:
                continue
            med1 = percentile(v1, 50) if v1 else 0.0
            med2 = percentile(v2, 50) if v2 else 0.0
            avg_d = (med1 + med2) / 2 if v1 and v2 else (med1 or med2)
            if abs(avg_d) < THR["stft_note"]:
                continue
            any_corrections = True
            # Correction factor: if STFT=-10%, map too rich → reduce by 10%
            # factor_delta = avg_d / 100  (positive = need more fuel, negative = less)
            if avg_d < -THR["stft_warn"]:
                severity = "⛔ REDUCE FUEL"
            elif avg_d < -THR["stft_note"]:
                severity = "⚠ reduce fuel"
            elif avg_d > THR["stft_warn"]:
                severity = "⛔ ADD FUEL"
            else:
                severity = "· add fuel"
            lines.append(f"  {RPM_LABELS[i]:<14} {LOAD_LABELS[j]:<12} "
                         f"{fmt_pct(med1)}  {fmt_pct(med2)}  {fmt_pct(avg_d,8)}  "
                         f"{severity}")
    if not any_corrections:
        lines.append("  No cells with |STFT| > 8% — map is well calibrated in covered range")

    # ════════════════════════════════════════════════════════════════════════════
    # 5 — LTFT
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(5, "LTFT — LONG-TERM FUEL TRIM"))
    mul1_v = [r["ltft_mul_b1"] for r in run_rows]
    mul2_v = [r["ltft_mul_b2"] for r in run_rows]
    add1_v = [r["ltft_add_b1"] for r in run_rows]
    add2_v = [r["ltft_add_b2"] for r in run_rows]

    if has_ltft:
        mul1_med = percentile(mul1_v, 50)
        mul2_med = percentile(mul2_v, 50)
        add1_med = percentile(add1_v, 50)
        add2_med = percentile(add2_v, 50)

        imbal = mul1_med - mul2_med
        f_m1  = flag(mul1_med, THR["ltft_warn"], THR["ltft_alarm"])
        f_m2  = flag(mul2_med, THR["ltft_warn"], THR["ltft_alarm"])
        f_im  = flag(imbal,    THR["ltft_imbalance"], THR["ltft_imbalance"] * 1.5)

        lines.append(f"  Multiplicative:  B1 {fmt_pct(mul1_med)}{f_m1}   "
                     f"B2 {fmt_pct(mul2_med)}{f_m2}   "
                     f"Δ = {fmt_pct(imbal)}{f_im}")
        lines.append(f"  Additive:        B1 {add1_med:+.3f} mg/stk   B2 {add2_med:+.3f} mg/stk")

        # Range (stability check)
        mul1_range = max(mul1_v) - min(mul1_v)
        mul2_range = max(mul2_v) - min(mul2_v)
        lines.append(f"  Stability:       B1 range {mul1_range:.3f}%   B2 range {mul2_range:.3f}%"
                     + ("  (stable)" if mul1_range < 0.5 and mul2_range < 0.5 else "  (drifting)"))

        # Distance from ceiling (±20%)
        ceiling = 20.0
        dist1 = ceiling - abs(mul1_med)
        dist2 = ceiling - abs(mul2_med)
        lines.append(f"  Ceiling margin:  B1 {dist1:.1f}% left   B2 {dist2:.1f}% left  (ceiling ±{ceiling:.0f}%)")

        if abs(imbal) > THR["ltft_imbalance"]:
            lines.append(f"\n  ⚠ ASYMMETRY: {imbal:+.1f}% gap B1/B2 — suspect lambda sensor drift or mechanical fault")
            lines.append(f"    → Compare with swap lambda B1↔B2: if asymmetry flips → sensor issue")
            lines.append(f"    → If asymmetry stays → mechanical (injector, compression, intake)")
    else:
        lines.append("  LTFT = 0  — no adaptation built yet (session too short or reset)")
        lines.append("  → Run at least 30–40 min in CL to accumulate meaningful LTFT")

    # ════════════════════════════════════════════════════════════════════════════
    # 6 — Lambda quality
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(6, "LAMBDA ANALYSIS"))

    if has_lmb:
        lmb1_cl = [r["lmb_b1"] for r in cl_rows if 0.3 < r["lmb_b1"] < 2.5]
        lmb2_cl = [r["lmb_b2"] for r in cl_rows if 0.3 < r["lmb_b2"] < 2.5]

        if lmb1_cl:
            mu1 = statistics.mean(lmb1_cl)
            sd1 = statistics.stdev(lmb1_cl) if len(lmb1_cl) > 1 else 0
            lines.append(f"  CL B1  mean={mu1:.4f}  std={sd1:.4f}  "
                         f"p5={percentile(lmb1_cl,5):.3f}  p50={percentile(lmb1_cl,50):.3f}  "
                         f"p95={percentile(lmb1_cl,95):.3f}")
        if lmb2_cl:
            mu2 = statistics.mean(lmb2_cl)
            sd2 = statistics.stdev(lmb2_cl) if len(lmb2_cl) > 1 else 0
            lines.append(f"  CL B2  mean={mu2:.4f}  std={sd2:.4f}  "
                         f"p5={percentile(lmb2_cl,5):.3f}  p50={percentile(lmb2_cl,50):.3f}  "
                         f"p95={percentile(lmb2_cl,95):.3f}")

        # Stoichiometric crossing rate (PID health — should be ~0.5–2 Hz)
        crossings_b1 = sum(1 for i in range(1, len(lmb1_cl))
                           if (lmb1_cl[i-1] - 1.0) * (lmb1_cl[i] - 1.0) < 0)
        crossings_b2 = sum(1 for i in range(1, len(lmb2_cl))
                           if (lmb2_cl[i-1] - 1.0) * (lmb2_cl[i] - 1.0) < 0)
        cl_dur = sum(p_dur.get(ph, 0) for ph in CL_PHASES)
        if cl_dur > 0:
            hz_b1 = crossings_b1 / cl_dur
            hz_b2 = crossings_b2 / cl_dur
            ok1 = "OK" if 0.3 < hz_b1 < 3.0 else ("SLOW" if hz_b1 < 0.3 else "FAST")
            ok2 = "OK" if 0.3 < hz_b2 < 3.0 else ("SLOW" if hz_b2 < 0.3 else "FAST")
            lines.append(f"\n  λ crossings (PID health):  B1 {hz_b1:.2f} Hz [{ok1}]   "
                         f"B2 {hz_b2:.2f} Hz [{ok2}]")
            lines.append(f"  (expected 0.3–2.0 Hz for healthy wide-band PID)")

        # Lambda under different load ranges
        lines.append(f"\n  LAMBDA vs LOAD  (CL rows only)")
        lines.append(f"  {'Load range':<14} {'λ B1 med':>9} {'λ B2 med':>9} {'Δ B1-B2':>9}  {'n':>5}")
        load_ranges = [(0, 20), (20, 40), (40, 60), (60, 80), (80, 100)]
        for lo, hi in load_ranges:
            lr = [r for r in cl_rows if lo <= r["load_pct"] < hi
                  and 0.3 < r["lmb_b1"] < 2.5 and 0.3 < r["lmb_b2"] < 2.5]
            if len(lr) < 5:
                continue
            l1 = [r["lmb_b1"] for r in lr]
            l2 = [r["lmb_b2"] for r in lr]
            med1 = percentile(l1, 50)
            med2 = percentile(l2, 50)
            delta = med1 - med2
            f_l1 = " ⚠LEAN" if med1 > THR["lmb_lean_load"] else (" ⚠RICH" if med1 < THR["lmb_rich"] else "")
            f_l2 = " ⚠LEAN" if med2 > THR["lmb_lean_load"] else (" ⚠RICH" if med2 < THR["lmb_rich"] else "")
            f_d  = " ⚠IMBAL" if abs(delta) > 0.03 else ""
            lines.append(f"  {lo}-{hi}%{'':<9} {med1:>9.4f}{f_l1}  {med2:>9.4f}{f_l2}  "
                         f"{delta:>+9.4f}{f_d}  {len(lr):>5}")

        # WOT lambda (E85 target ~0.85–0.92 for power)
        wot_rows = [r for r in rows if r["phase"] == "WOT"
                    and 0.3 < r["lmb_b1"] < 2.0 and 0.3 < r["lmb_b2"] < 2.0
                    and r["rpm"] > 2500]
        if wot_rows:
            wl1 = [r["lmb_b1"] for r in wot_rows]
            wl2 = [r["lmb_b2"] for r in wot_rows]
            m1, m2 = statistics.mean(wl1), statistics.mean(wl2)
            f_w1 = " ⚠LEAN" if m1 > 0.95 else (" OK(E85)" if m1 < 0.93 else "")
            f_w2 = " ⚠LEAN" if m2 > 0.95 else (" OK(E85)" if m2 < 0.93 else "")
            lines.append(f"\n  WOT lambda (>2500 RPM): B1 {m1:.4f}{f_w1}   B2 {m2:.4f}{f_w2}")
            lines.append(f"  (E85 WOT target typically 0.85–0.92)")

        # Fuel-cut (open-loop sensor check — should go lean)
        fc_rows = [r for r in rows if r["phase"] == "FUEL_CUT"
                   and 0.5 < r["lmb_b1"] < 4.0]
        if fc_rows:
            fl1 = [r["lmb_b1"] for r in fc_rows]
            fl2 = [r["lmb_b2"] for r in fc_rows if 0.5 < r["lmb_b2"] < 4.0]
            lines.append(f"\n  Fuel-cut λ B1 : med {percentile(fl1,50):.3f}  "
                         f"range [{min(fl1):.3f}–{max(fl1):.3f}]  (open loop response)")
            if fl2:
                lines.append(f"  Fuel-cut λ B2 : med {percentile(fl2,50):.3f}  "
                             f"range [{min(fl2):.3f}–{max(fl2):.3f}]")
            if percentile(fl1, 50) < 1.3:
                lines.append(f"  ⚠ B1 not going lean on fuel cut → possible sensor sluggishness")
    else:
        lines.append("  [lambda columns not present in this log]")

    # ════════════════════════════════════════════════════════════════════════════
    # 7 — B1/B2 asymmetry deep dive
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(7, "B1 / B2 ASYMMETRY ANALYSIS"))

    if cl_rows:
        s1_all = [r["stft_b1"] for r in cl_rows]
        s2_all = [r["stft_b2"] for r in cl_rows]
        delta_all = [r["stft_b1"] - r["stft_b2"] for r in cl_rows]

        med_delta = percentile(delta_all, 50)
        p5_delta  = percentile(delta_all, 5)
        p95_delta = percentile(delta_all, 95)
        sd_delta  = statistics.stdev(delta_all) if len(delta_all) > 1 else 0

        lines.append(f"  STFT B1-B2 delta:  median {med_delta:+.2f}%  std {sd_delta:.2f}%  "
                     f"p5={p5_delta:+.1f}%  p95={p95_delta:+.1f}%")

        if abs(med_delta) > THR["stft_imbalance"]:
            lines.append(f"  ⚠ PERSISTENT IMBALANCE: {med_delta:+.2f}% — not random noise")
        elif abs(med_delta) > 3:
            lines.append(f"  · Slight systematic offset: {med_delta:+.2f}% — monitor over time")
        else:
            lines.append(f"  Banks are balanced (delta {med_delta:+.2f}% within tolerance)")

        # Asymmetry vs temperature
        lines.append(f"\n  Asymmetry vs coolant temperature:")
        temp_ranges = [(30, 60), (60, 80), (80, 95), (95, 115)]
        for t_lo, t_hi in temp_ranges:
            tr = [r["stft_b1"] - r["stft_b2"] for r in cl_rows
                  if t_lo <= r["coolant_c"] < t_hi]
            if len(tr) < 10:
                continue
            med_t = percentile(tr, 50)
            lines.append(f"  {t_lo}–{t_hi}°C : delta = {med_t:+.2f}%  (n={len(tr)})")

        # Asymmetry vs RPM range
        lines.append(f"\n  Asymmetry vs RPM:")
        rpm_ranges = [(600, 1200), (1200, 2000), (2000, 3000), (3000, 5500)]
        for r_lo, r_hi in rpm_ranges:
            rr = [r["stft_b1"] - r["stft_b2"] for r in cl_rows
                  if r_lo <= r["rpm"] < r_hi]
            if len(rr) < 10:
                continue
            med_r = percentile(rr, 50)
            lines.append(f"  {r_lo}–{r_hi} RPM : delta = {med_r:+.2f}%  (n={len(rr)})")

        # Asymmetry vs load
        lines.append(f"\n  Asymmetry vs load:")
        for lo, hi in [(0, 20), (20, 40), (40, 60), (60, 100)]:
            lr = [r["stft_b1"] - r["stft_b2"] for r in cl_rows
                  if lo <= r["load_pct"] < hi]
            if len(lr) < 10:
                continue
            med_l = percentile(lr, 50)
            lines.append(f"  {lo}–{hi}% load : delta = {med_l:+.2f}%  (n={len(lr)})")

    # ════════════════════════════════════════════════════════════════════════════
    # 8 — Injection & fueling
    # ════════════════════════════════════════════════════════════════════════════
    if has_inj:
        lines.append(H(8, "INJECTION — DUTY CYCLE & FUELING"))
        inj_run = [r for r in run_rows if r["inj_ms"] > 0.5]
        if inj_run:
            # Duty cycle = inj_ms / (60000/RPM) * 100
            duty_list = []
            for r in inj_run:
                period_ms = 60000.0 / r["rpm"] if r["rpm"] > 0 else 0
                if period_ms > 0:
                    dc = r["inj_ms"] / period_ms * 100.0
                    duty_list.append((r, dc))

            if duty_list:
                dc_vals = [dc for _, dc in duty_list]
                lines.append(f"  Duty cycle: avg {statistics.mean(dc_vals):.1f}%  "
                             f"p95 {percentile(dc_vals,95):.1f}%  "
                             f"max {max(dc_vals):.1f}%")
                if max(dc_vals) > THR["inj_duty_warn"]:
                    peak_row, peak_dc = max(duty_list, key=lambda x: x[1])
                    lines.append(f"  ⚠ Peak duty {peak_dc:.1f}% @ {peak_row['rpm']:.0f} RPM / "
                                 f"{peak_row['load_pct']:.0f}% load  — headroom limited")
                else:
                    lines.append(f"  Injector duty OK — well below {THR['inj_duty_warn']:.0f}% limit")

            # Inj time by phase
            lines.append(f"\n  {'Phase':<14}  {'Inj avg':>8}  {'Inj max':>8}  {'DC avg':>8}")
            for ph in CL_PHASES:
                pr = [r for r in inj_run if r["phase"] == ph]
                if not pr:
                    continue
                inj_v = [r["inj_ms"] for r in pr]
                dc_v  = [r["inj_ms"] / (60000.0 / r["rpm"]) * 100.0
                         for r in pr if r["rpm"] > 0]
                lines.append(f"  {ph:<14}  {statistics.mean(inj_v):>7.2f}ms  "
                             f"{max(inj_v):>7.2f}ms  "
                             f"{(statistics.mean(dc_v) if dc_v else 0):>7.1f}%")

    # ════════════════════════════════════════════════════════════════════════════
    # 9 — VANOS
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(9, "VANOS — INLET CAMSHAFT TRACKING"))
    if has_vanos:
        vanos_active = [r for r in rows if abs(r["vanos_tgt"]) > 1 and r["rpm"] > 800]
        vanos_steady = [r for r in steady_rows if abs(r["vanos_tgt"]) > 1]

        if vanos_active:
            err_all    = [r["vanos_err"] for r in vanos_active]
            err_steady = [r["vanos_err"] for r in vanos_steady]

            lines.append(f"  All active rows:  "
                         f"mean {statistics.mean(err_all):+.2f}°  "
                         f"max|{max(abs(e) for e in err_all):.1f}°|  "
                         f"p95|{percentile([abs(e) for e in err_all],95):.1f}°|  "
                         f"(n={len(err_all)})")

            if err_steady:
                mean_ss = statistics.mean(err_steady)
                max_ss  = max(abs(e) for e in err_steady)
                f_ss    = flag(mean_ss, THR["vanos_steady_warn"], THR["vanos_steady_alarm"])
                lines.append(f"  Steady-state:     "
                             f"mean {mean_ss:+.2f}°  "
                             f"max|{max_ss:.1f}°|{f_ss}  "
                             f"(n={len(err_steady)}, after transients removed)")
                ok_pct = 100 * sum(1 for e in err_steady
                                   if abs(e) < THR["vanos_steady_warn"]) / len(err_steady)
                lines.append(f"  Tracking OK:      {ok_pct:.1f}% within ±{THR['vanos_steady_warn']:.0f}°")

            # VANOS error vs RPM
            lines.append(f"\n  Steady-state error vs RPM:")
            for r_lo, r_hi in [(800, 1500), (1500, 2500), (2500, 3500), (3500, 5500)]:
                rr = [r["vanos_err"] for r in vanos_steady if r_lo <= r["rpm"] < r_hi]
                if len(rr) < 5:
                    continue
                lines.append(f"  {r_lo}–{r_hi} RPM : {statistics.mean(rr):+.2f}°  "
                             f"(max|{max(abs(e) for e in rr):.1f}°|  n={len(rr)})")

            act_v = [r["vanos_act"] for r in vanos_active]
            tgt_v = [r["vanos_tgt"] for r in vanos_active]
            lines.append(f"\n  Actual range : {min(act_v):.0f}° – {max(act_v):.0f}°")
            lines.append(f"  Target range : {min(tgt_v):.0f}° – {max(tgt_v):.0f}°")
        else:
            lines.append("  No VANOS active samples (target=0 throughout)")
    else:
        lines.append("  VANOS columns not present in this log")

    # ════════════════════════════════════════════════════════════════════════════
    # 10 — Ignition
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(10, "IGNITION — ADVANCE & KNOCK PROXY"))
    if has_ign:
        ign_run = [r for r in run_rows if abs(r["ign_deg"]) > NEAR_ZERO]
        if ign_run:
            # Build RPM/Load-local baseline to detect true retards
            # For each row, compare to p75 in its RPM × Load cell
            ign_cell = defaultdict(list)
            for r in ign_run:
                ign_cell[(r["rb"], r["lb"])].append(r["ign_deg"])
            cell_baseline = {k: percentile(v, 75) for k, v in ign_cell.items() if len(v) >= 5}

            retard_events = []
            for r in ign_run:
                key = (r["rb"], r["lb"])
                if key in cell_baseline:
                    if r["ign_deg"] < cell_baseline[key] - 5.0 and r["rpm"] > 1500:
                        retard_events.append(r)

            ret_pct = 100 * len(retard_events) / len(ign_run) if ign_run else 0
            ign_v = [r["ign_deg"] for r in ign_run]
            lines.append(f"  Ignition avg : {statistics.mean(ign_v):+.1f}°  "
                         f"range [{min(ign_v):+.1f}° – {max(ign_v):+.1f}°]")
            lines.append(f"  Retard events: {len(retard_events)} ({ret_pct:.1f}%)  "
                         f"threshold = cell p75 – 5°")

            if ret_pct > 2.0:
                lines.append(f"  ⚠ Significant knock activity — check fuel quality / octane")
            elif ret_pct > 0.5:
                lines.append(f"  · Occasional retard — normal for transients, monitor")
            else:
                lines.append(f"  Clean — no sustained knock retard detected")

            # Ignition by phase
            lines.append(f"\n  {'Phase':<14}  {'Ign avg':>8}  {'Ign p5':>8}  {'Ign max':>8}  {'Retards':>8}")
            for ph in CL_PHASES:
                pr_ign = [r for r in ign_run if r["phase"] == ph]
                if not pr_ign:
                    continue
                iv = [r["ign_deg"] for r in pr_ign]
                ret_ph = sum(1 for r in retard_events if r["phase"] == ph)
                lines.append(f"  {ph:<14}  {statistics.mean(iv):>+7.1f}°  "
                             f"{percentile(iv,5):>+7.1f}°  "
                             f"{max(iv):>+7.1f}°  {ret_ph:>8}")

            # WOT advance (E85 should allow high advance)
            wot_ign = [r["ign_deg"] for r in ign_run if r["phase"] == "WOT" and r["rpm"] > 2500]
            if wot_ign:
                lines.append(f"\n  WOT advance (>2500 RPM) : avg {statistics.mean(wot_ign):+.1f}°  "
                             f"max {max(wot_ign):+.1f}°")
                if statistics.mean(wot_ign) < 15:
                    lines.append(f"  ⚠ Low WOT advance — E85 should allow higher timing")
        else:
            lines.append("  No ignition data (all zero)")
    else:
        lines.append("  Ignition column not present in this log")

    # ════════════════════════════════════════════════════════════════════════════
    # 11 — Anomaly timeline
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(11, "ANOMALY EVENT TIMELINE"))

    events = []

    stft_b1_streak = 0
    stft_b2_streak = 0

    for i, row in enumerate(rows):
        t = ts(row)
        if not row["cl_active"]:
            stft_b1_streak = 0
            stft_b2_streak = 0
            continue

        s1, s2 = row["stft_b1"], row["stft_b2"]

        if abs(s1) > THR["stft_alarm"]:
            stft_b1_streak += 1
            if stft_b1_streak == 6:
                events.append(("STFT_B1",      t, f"B1 sustained {s1:+.1f}% (>{THR['stft_alarm']:.0f}%)  "
                               f"RPM={row['rpm']:.0f} Load={row['load_pct']:.0f}%"))
        else:
            stft_b1_streak = 0

        if abs(s2) > THR["stft_alarm"]:
            stft_b2_streak += 1
            if stft_b2_streak == 6:
                events.append(("STFT_B2",      t, f"B2 sustained {s2:+.1f}% (>{THR['stft_alarm']:.0f}%)  "
                               f"RPM={row['rpm']:.0f} Load={row['load_pct']:.0f}%"))
        else:
            stft_b2_streak = 0

        if has_lmb:
            if row["lmb_b1"] > THR["lmb_lean_load"] and row["load_pct"] > 35:
                events.append(("LEAN_B1",   t, f"λB1={row['lmb_b1']:.3f} LEAN @ Load={row['load_pct']:.0f}% RPM={row['rpm']:.0f}"))
            if row["lmb_b2"] > THR["lmb_lean_load"] and row["load_pct"] > 35:
                events.append(("LEAN_B2",   t, f"λB2={row['lmb_b2']:.3f} LEAN @ Load={row['load_pct']:.0f}% RPM={row['rpm']:.0f}"))
            if row["lmb_b1"] < THR["lmb_rich"] and s1 < -THR["stft_warn"]:
                events.append(("RICH_B1",   t, f"λB1={row['lmb_b1']:.3f} RICH + STFT {s1:+.1f}%"))
            if row["lmb_b2"] < THR["lmb_rich"] and s2 < -THR["stft_warn"]:
                events.append(("RICH_B2",   t, f"λB2={row['lmb_b2']:.3f} RICH + STFT {s2:+.1f}%"))

        if has_vanos and abs(row["vanos_err"]) > THR["vanos_steady_alarm"] * 2 and row["rpm"] > 1000:
            events.append(("VANOS",       t, f"Err={row['vanos_err']:+.1f}° (act={row['vanos_act']:.0f}° tgt={row['vanos_tgt']:.0f}°) RPM={row['rpm']:.0f}"))

    # Coolant alarm
    for row in rows:
        if row["coolant_c"] > THR["coolant_warn"]:
            events.append(("OVERTEMP",    ts(row), f"Coolant {row['coolant_c']:.1f}°C  RPM={row['rpm']:.0f}"))
            break

    # Deduplicate by type+10s window
    seen = {}
    deduped = []
    for ev_type, t, msg in sorted(events, key=lambda e: e[1]):
        key = (ev_type, int(t / 10))
        if key not in seen:
            seen[key] = True
            deduped.append((ev_type, t, msg))

    if deduped:
        for ev_type, t, msg in deduped[:60]:
            lines.append(f"  t={t:>7.1f}s  [{ev_type:<10}]  {msg}")
        if len(deduped) > 60:
            lines.append(f"  … {len(deduped)-60} more events (same types)")
    else:
        lines.append("  No anomaly events detected.")

    # ════════════════════════════════════════════════════════════════════════════
    # 12 — Verdict
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(f"\n{SEP}")
    lines.append("  VERDICT")
    lines.append(THN)

    issues, warnings, oks = [], [], []

    # STFT
    if cl_rows:
        s1a = [r["stft_b1"] for r in cl_rows]
        s2a = [r["stft_b2"] for r in cl_rows]
        m1, m2 = percentile(s1a, 50), percentile(s2a, 50)
        delta_stft = m1 - m2

        for bank, med, sat_a in [("B1", m1, pct_above(s1a, THR["stft_alarm"]) + pct_below(s1a, -THR["stft_alarm"])),
                                  ("B2", m2, pct_above(s2a, THR["stft_alarm"]) + pct_below(s2a, -THR["stft_alarm"]))]:
            if sat_a > 5:
                issues.append(f"STFT {bank} median {med:+.1f}%, {sat_a:.1f}% near saturation — map richness off")
            elif abs(med) > THR["stft_warn"]:
                warnings.append(f"STFT {bank} median {med:+.1f}% — elevated, adjust ip_mff_cor")
            else:
                oks.append(f"STFT {bank} median {med:+.1f}% — OK")

        if abs(delta_stft) > THR["stft_imbalance"]:
            issues.append(f"B1/B2 gap {delta_stft:+.1f}% — suspect sensor or mechanical asymmetry")
        elif abs(delta_stft) > 4:
            warnings.append(f"B1/B2 slight asymmetry {delta_stft:+.1f}%")

    # LTFT
    if has_ltft:
        mul1 = percentile([r["ltft_mul_b1"] for r in run_rows], 50)
        mul2 = percentile([r["ltft_mul_b2"] for r in run_rows], 50)
        if abs(mul1) > THR["ltft_alarm"] or abs(mul2) > THR["ltft_alarm"]:
            issues.append(f"LTFT Mul: B1={mul1:+.1f}% B2={mul2:+.1f}% — at ceiling, map is wrong")
        elif abs(mul1) > THR["ltft_warn"] or abs(mul2) > THR["ltft_warn"]:
            warnings.append(f"LTFT Mul: B1={mul1:+.1f}% B2={mul2:+.1f}% — elevated")
        else:
            oks.append(f"LTFT Mul: B1={mul1:+.1f}% B2={mul2:+.1f}% — OK")
        if abs(mul1 - mul2) > THR["ltft_imbalance"]:
            issues.append(f"LTFT asymmetry {mul1-mul2:+.1f}% — sensor or mechanical fault")
    else:
        oks.append("LTFT = 0 (not yet adapted)")

    # Coolant
    if t_cool_max > THR["coolant_warn"]:
        issues.append(f"Coolant {t_cool_max:.0f}°C peak — check cooling")
    else:
        oks.append(f"Coolant OK (max {t_cool_max:.0f}°C)")

    # Lambda WOT
    if has_lmb:
        wot_lmb = [r["lmb_b1"] for r in rows if r["phase"] == "WOT"
                   and 0.3 < r["lmb_b1"] < 2.0 and r["rpm"] > 2500]
        if wot_lmb:
            m = statistics.mean(wot_lmb)
            if m > 0.98:
                issues.append(f"WOT lambda B1 {m:.3f} — lean at full load, safety risk")
            elif m > 0.94:
                warnings.append(f"WOT lambda B1 {m:.3f} — slightly lean at WOT")
            else:
                oks.append(f"WOT lambda B1 {m:.3f} — rich enough for E85")

    # Injector duty
    if has_inj:
        inj_run = [r for r in run_rows if r["inj_ms"] > 0.5 and r["rpm"] > 0]
        if inj_run:
            dc_max = max(r["inj_ms"] / (60000.0 / r["rpm"]) * 100.0 for r in inj_run)
            if dc_max > THR["inj_duty_warn"]:
                issues.append(f"Injector duty {dc_max:.1f}% peak — approaching limit")
            else:
                oks.append(f"Injector duty OK (max {dc_max:.1f}%)")

    status = "⛔ CRITICAL" if issues else ("⚠ WARNINGS" if warnings else "✓ OK")
    lines.append(f"  STATUS: {status}")

    if issues:
        lines.append("")
        for x in issues:
            lines.append(f"  ✗ [ISSUE]   {x}")
    if warnings:
        lines.append("")
        for x in warnings:
            lines.append(f"  ⚠ [WARN]    {x}")
    if oks:
        lines.append("")
        for x in oks:
            lines.append(f"  ✓ [OK]      {x}")

    lines.append(SEP)
    lines.append("")

    return "\n".join(lines)


# ─── Entry point ─────────────────────────────────────────────────────────────────
def find_latest_csv():
    logs_dir = Path(__file__).parent.parent / "LOGS"
    csvs = sorted(logs_dir.glob("*.csv"), key=lambda p: p.stat().st_mtime, reverse=True)
    if not csvs:
        csvs = sorted((logs_dir / "old").glob("*.csv"),
                      key=lambda p: p.stat().st_mtime, reverse=True)
    if not csvs:
        raise FileNotFoundError(f"No CSV files in {logs_dir}")
    return csvs[0]


def main():
    if len(sys.argv) > 1:
        path = Path(sys.argv[1])
        if not path.is_absolute():
            path = Path.cwd() / path
    else:
        path = find_latest_csv()
        print(f"Auto-selected: {path.name}\n")

    if not path.exists():
        print(f"Error: {path}", file=sys.stderr)
        sys.exit(1)

    try:
        print(analyze(path))
    except Exception:
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
