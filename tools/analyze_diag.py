#!/usr/bin/env python3
"""
MSV70 Diagnostic Log Analyzer — deep precision mode
Analyzes CSV logs from the custom MSV70_DIAG TestO job.

Usage:
    python analyze_diag.py [file.csv]
    If no file given, uses the most recent CSV in ../LOGS/
"""

import sys
import csv
import statistics
from pathlib import Path
from collections import defaultdict

if sys.platform == "win32":
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

# ─── Column aliases ──────────────────────────────────────────────────────────────
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

# ─── Thresholds ──────────────────────────────────────────────────────────────────
THR = {
    "stft_note":          8.0,  # % — notable, worth mentioning
    "stft_warn":         15.0,  # % — sustained deviation
    "stft_alarm":        22.0,  # % — approaching ±40% ceiling
    "stft_imbalance":     8.0,  # % — |B1-B2| persistent gap
    "ltft_warn":         12.0,  # % — multiplicative LTFT
    "ltft_alarm":        18.0,  # % — near ±20% ceiling
    "ltft_imbalance":    12.0,  # % — |B1-B2|
    "lmb_lean_load":      1.04, # λ — lean alarm under load (sustained)
    "lmb_rich":           0.93, # λ — rich alarm with high negative STFT
    "vanos_steady_warn":  5.0,  # °CRK — steady-state tracking error
    "vanos_steady_alarm": 10.0,
    "inj_duty_warn":     80.0,  # % — injector duty cycle (120000/RPM formula)
    "coolant_warn":     114.0,  # °C — N52 thermostat 112°C normal, alarm >114
    "knock_rpm_min":   2000.0,  # only flag knock above this RPM
    "knock_load_min":    35.0,  # and above this load %
    "knock_delta":        6.0,  # °CRK below per-cell hot-engine p50 = retard event
    "knock_hot_coolant": 85.0,  # °C — baseline only from hot-engine rows
}

# ─── RPM / Load bins for 2D map ──────────────────────────────────────────────────
RPM_BINS   = [0, 800, 1200, 1600, 2000, 2500, 3000, 3800, 5000, 9999]
LOAD_BINS  = [0, 10,  20,   30,   40,   50,   60,   75,   100]
RPM_LABELS = ["<800", "800-1200", "1200-1600", "1600-2000",
              "2000-2500", "2500-3000", "3000-3800", "3800-5000", "5000+"]
LOAD_LABELS = ["0-10", "10-20", "20-30", "30-40",
               "40-50", "50-60", "60-75", "75-100"]

# ─── Phases ──────────────────────────────────────────────────────────────────────
PHASES     = ["ENGINE_OFF", "CRANKING", "WARMUP", "HOT_IDLE", "IDLE",
              "CRUISE", "ACCEL", "WOT", "FUEL_CUT"]
CL_PHASES  = ["WARMUP", "HOT_IDLE", "IDLE", "CRUISE", "ACCEL", "WOT"]


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


def is_steady(row, prev_row, dt_s):
    if dt_s <= 0 or dt_s > 1.5:
        return False
    return (abs(row["rpm"]      - prev_row["rpm"])      / dt_s < 300 and
            abs(row["load_pct"] - prev_row["load_pct"]) / dt_s < 20)


# ─── Data loading ─────────────────────────────────────────────────────────────────
def safe(v):
    """Collapse floating-point noise around zero (TestO artefact)."""
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
            raise ValueError(f"Missing required column '{req}'")

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


# ─── Statistics ───────────────────────────────────────────────────────────────────
def percentile(data, p):
    if not data:
        return 0.0
    s = sorted(data)
    idx = (len(s) - 1) * p / 100.0
    lo, hi = int(idx), min(int(idx) + 1, len(s) - 1)
    return s[lo] + (idx - lo) * (s[hi] - s[lo])


def pct_outside(data, threshold):
    """Percentage of values where |v| > threshold."""
    if not data:
        return 0.0
    return 100.0 * sum(1 for v in data if abs(v) > threshold) / len(data)


def fmt_pct(v, w=7):
    return f"{'+' if v > 0 else ''}{v:.1f}%".rjust(w)


def fmt_lmb(v):
    return f"{v:.4f}"


def flag(v, warn, alarm):
    if abs(v) >= alarm:
        return " ⛔"
    if abs(v) >= warn:
        return " ⚠"
    return ""


def phase_dur_s(rows):
    d = defaultdict(float)
    for i in range(1, len(rows)):
        dt = (rows[i]["time_ms"] - rows[i-1]["time_ms"]) / 1000.0
        if 0 < dt < 2.0:
            d[rows[i]["phase"]] += dt
    return d


# ─── Main analysis ────────────────────────────────────────────────────────────────
def analyze(path):
    rows, col_map = load_csv(path)
    available = set(col_map.keys())

    n = len(rows)
    for row in rows:
        row["phase"]     = detect_phase(row)
        row["rb"]        = rpm_bin(row["rpm"])
        row["lb"]        = load_bin(row["load_pct"])
        row["vanos_err"] = row["vanos_act"] - row["vanos_tgt"]
        # Closed-loop active per bank (B2 can be in CL independently)
        row["cl_b1"] = (row["rpm"] > 600 and row["coolant_c"] > 55
                        and row["load_pct"] > 2 and abs(row["stft_b1"]) > NEAR_ZERO)
        row["cl_b2"] = (row["rpm"] > 600 and row["coolant_c"] > 55
                        and row["load_pct"] > 2 and abs(row["stft_b2"]) > NEAR_ZERO)
        row["cl_active"] = row["cl_b1"] or row["cl_b2"]

    dur_s        = ts(rows[-1]) - ts(rows[0])
    cool_start   = rows[0]["coolant_c"]
    cool_end     = rows[-1]["coolant_c"]
    cool_max     = max(r["coolant_c"] for r in rows)

    phase_rows   = defaultdict(list)
    for row in rows:
        phase_rows[row["phase"]].append(row)
    p_dur        = phase_dur_s(rows)

    # Per-bank CL row sets
    cl_b1_rows   = [r for r in rows if r["cl_b1"]]
    cl_b2_rows   = [r for r in rows if r["cl_b2"]]
    cl_both_rows = [r for r in rows if r["cl_b1"] and r["cl_b2"]]
    run_rows     = [r for r in rows if r["rpm"] > 600]
    hot_rows     = [r for r in rows if r["coolant_c"] > THR["knock_hot_coolant"]]

    # Steady-state rows for VANOS (no rapid transients)
    steady_rows = []
    for i in range(1, n):
        dt = (rows[i]["time_ms"] - rows[i-1]["time_ms"]) / 1000.0
        if is_steady(rows[i], rows[i-1], dt) and rows[i]["rpm"] > 800:
            steady_rows.append(rows[i])

    has_ltft  = any(abs(r["ltft_mul_b1"]) > NEAR_ZERO or abs(r["ltft_mul_b2"]) > NEAR_ZERO
                    for r in run_rows)
    has_vanos = "vanos_act" in available and "vanos_tgt" in available
    has_ign   = "ign_deg" in available
    has_inj   = "inj_ms" in available
    has_lmb   = "lmb_b1" in available and "lmb_b2" in available

    # ── Build ignition knock baseline from hot-engine rows only ─────────────────
    ign_cell_hot = defaultdict(list)
    if has_ign:
        for r in hot_rows:
            if abs(r["ign_deg"]) > NEAR_ZERO and r["rpm"] > 600:
                ign_cell_hot[(r["rb"], r["lb"])].append(r["ign_deg"])
    cell_ign_p50 = {k: percentile(v, 50) for k, v in ign_cell_hot.items() if len(v) >= 10}

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
        f"  Coolant  : {cool_start:.0f}°C → {cool_end:.0f}°C  (peak {cool_max:.0f}°C)",
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
                     f"p50 {percentile(rpm_v,50):.0f}  p95 {percentile(rpm_v,95):.0f}  "
                     f"max {max(rpm_v):.0f}")
    if load_v:
        lines.append(f"  Load       avg {statistics.mean(load_v):.1f}%  "
                     f"p50 {percentile(load_v,50):.1f}%  p95 {percentile(load_v,95):.1f}%  "
                     f"max {max(load_v):.1f}%")
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

    b1_cl_pct = 100 * len(cl_b1_rows) / n if n else 0
    b2_cl_pct = 100 * len(cl_b2_rows) / n if n else 0
    cl_note = ""
    if abs(b1_cl_pct - b2_cl_pct) > 15:
        lesser = "B2" if b2_cl_pct < b1_cl_pct else "B1"
        cl_note = f"  [{lesser} enters CL less often — monitor STFT asymmetry]"
    lines.append(f"  CL B1      {b1_cl_pct:.0f}% of rows ({len(cl_b1_rows)})")
    lines.append(f"  CL B2      {b2_cl_pct:.0f}% of rows ({len(cl_b2_rows)}){cl_note}")

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
    # 3 — STFT deep dive (per-bank filter: only rows where that bank is in CL)
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(3, "STFT — PER PHASE  (each bank filtered to its own CL rows, percentiles)"))
    lines.append(f"  {'Phase':<14}  {'B1 p5':>7}  {'B1 p50':>7}  {'B1 p95':>7}  "
                 f"{'nB1':>5}  {'B2 p5':>7}  {'B2 p50':>7}  {'B2 p95':>7}  {'nB2':>5}  {'Δmed':>6}")

    for ph in CL_PHASES:
        r1 = [r for r in cl_b1_rows if r["phase"] == ph]
        r2 = [r for r in cl_b2_rows if r["phase"] == ph]
        if not r1 and not r2:
            continue

        s1 = [r["stft_b1"] for r in r1] if r1 else [0.0]
        s2 = [r["stft_b2"] for r in r2] if r2 else [0.0]

        p5_1  = percentile(s1,  5)
        p50_1 = percentile(s1, 50)
        p95_1 = percentile(s1, 95)
        p5_2  = percentile(s2,  5)
        p50_2 = percentile(s2, 50)
        p95_2 = percentile(s2, 95)
        delta = p50_1 - p50_2

        f1 = flag(p50_1, THR["stft_note"], THR["stft_warn"])
        f2 = flag(p50_2, THR["stft_note"], THR["stft_warn"])
        fi = flag(delta,  THR["stft_imbalance"], THR["stft_imbalance"] * 1.5)

        lines.append(f"  {ph:<14}  "
                     f"{fmt_pct(p5_1)}  {fmt_pct(p50_1)}{f1}  {fmt_pct(p95_1)}  {len(r1):>5}  "
                     f"{fmt_pct(p5_2)}  {fmt_pct(p50_2)}{f2}  {fmt_pct(p95_2)}  {len(r2):>5}  "
                     f"{fmt_pct(delta, 6)}{fi}")

    # Overall
    if cl_b1_rows or cl_b2_rows:
        s1_all = [r["stft_b1"] for r in cl_b1_rows]
        s2_all = [r["stft_b2"] for r in cl_b2_rows]
        p50_1  = percentile(s1_all, 50) if s1_all else 0
        p50_2  = percentile(s2_all, 50) if s2_all else 0
        lines.append(THN)
        lines.append(f"  {'OVERALL':<14}  "
                     f"{fmt_pct(percentile(s1_all,5)) if s1_all else '    ---'}  "
                     f"{fmt_pct(p50_1)}  "
                     f"{fmt_pct(percentile(s1_all,95)) if s1_all else '    ---'}  "
                     f"{len(cl_b1_rows):>5}  "
                     f"{fmt_pct(percentile(s2_all,5)) if s2_all else '    ---'}  "
                     f"{fmt_pct(p50_2)}  "
                     f"{fmt_pct(percentile(s2_all,95)) if s2_all else '    ---'}  "
                     f"{len(cl_b2_rows):>5}  "
                     f"{fmt_pct(p50_1 - p50_2, 6)}")

        sat_b1 = pct_outside(s1_all, THR["stft_alarm"]) if s1_all else 0
        sat_b2 = pct_outside(s2_all, THR["stft_alarm"]) if s2_all else 0
        lines.append(f"\n  Saturation >|{THR['stft_warn']:.0f}%| :  "
                     f"B1 {pct_outside(s1_all, THR['stft_warn']):.1f}%  "
                     f"B2 {pct_outside(s2_all, THR['stft_warn']):.1f}%  of their CL rows")
        lines.append(f"  Saturation >|{THR['stft_alarm']:.0f}%| :  "
                     f"B1 {sat_b1:.1f}%  B2 {sat_b2:.1f}%  of their CL rows")

    # ── STFT temporal drift (60s buckets) ────────────────────────────────────
    lines.append(f"\n  STFT DRIFT OVER TIME  (60s buckets, CL rows per bank)")
    lines.append(f"  {'t(s)':>6}  {'B1 p50':>7}  {'nB1':>5}  {'B2 p50':>7}  {'nB2':>5}  "
                 f"{'Δmed':>7}  {'RPM':>6}  {'Load':>6}  {'Cool':>7}")

    if cl_b1_rows or cl_b2_rows:
        all_cl = sorted(cl_b1_rows + cl_b2_rows, key=lambda r: r["time_ms"])
        t0     = ts(all_cl[0])
        BUCKET = 60.0
        bkt_b1 = defaultdict(list)
        bkt_b2 = defaultdict(list)
        for r in cl_b1_rows:
            bkt_b1[int((ts(r) - t0) / BUCKET)].append(r)
        for r in cl_b2_rows:
            bkt_b2[int((ts(r) - t0) / BUCKET)].append(r)

        all_keys = sorted(set(bkt_b1.keys()) | set(bkt_b2.keys()))
        for b in all_keys:
            br1 = bkt_b1.get(b, [])
            br2 = bkt_b2.get(b, [])
            br_any = br1 or br2
            if not br_any:
                continue
            t_mid    = t0 + b * BUCKET
            p50_1    = percentile([r["stft_b1"] for r in br1], 50) if br1 else None
            p50_2    = percentile([r["stft_b2"] for r in br2], 50) if br2 else None
            delta    = (p50_1 - p50_2) if (p50_1 is not None and p50_2 is not None) else None
            all_r    = br1 + br2
            avg_rpm  = statistics.mean(r["rpm"]       for r in all_r)
            avg_load = statistics.mean(r["load_pct"]  for r in all_r)
            avg_cool = statistics.mean(r["coolant_c"] for r in all_r)

            s1_str = fmt_pct(p50_1) if p50_1 is not None else "    ---"
            s2_str = fmt_pct(p50_2) if p50_2 is not None else "    ---"
            d_str  = fmt_pct(delta, 7) if delta is not None else "     ---"

            f1 = " ⚠" if p50_1 is not None and abs(p50_1) > THR["stft_warn"] else ""
            f2 = " ⚠" if p50_2 is not None and abs(p50_2) > THR["stft_warn"] else ""
            lines.append(f"  {t_mid:>6.0f}s  "
                         f"{s1_str}{f1}  {len(br1):>5}  "
                         f"{s2_str}{f2}  {len(br2):>5}  "
                         f"{d_str}  "
                         f"{avg_rpm:>6.0f}  {avg_load:>5.1f}%  {avg_cool:>6.1f}°C")

    # ════════════════════════════════════════════════════════════════════════════
    # 4 — STFT 2D map (RPM × Load, per-bank filtered)
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(4, "STFT 2D MAP — RPM × Load  (median, per-bank CL rows only)"))
    lines.append("  <5 rows = ---  |  · = |8%|  ⚠ = |15%|  ⛔ = |22%|")
    lines.append("  Negative = too rich (reduce ip_mff_cor)  |  Positive = too lean (add fuel)")

    cell_b1 = defaultdict(list)
    cell_b2 = defaultdict(list)
    for r in cl_b1_rows:
        cell_b1[(r["rb"], r["lb"])].append(r["stft_b1"])
    for r in cl_b2_rows:
        cell_b2[(r["rb"], r["lb"])].append(r["stft_b2"])

    n_rb = len(RPM_BINS) - 1
    n_lb = len(LOAD_BINS) - 1
    COL_W = 13

    def render_map(label, cell_dict):
        out = [f"\n  {label}"]
        header = "  RPM \\ Load  " + "".join(f"{LOAD_LABELS[j]:>{COL_W}}" for j in range(n_lb))
        out.append(header)
        out.append("  " + "─" * (13 + COL_W * n_lb))
        for i in range(n_rb):
            row_line = f"  {RPM_LABELS[i]:<12} "
            any_data = False
            for j in range(n_lb):
                vals = cell_dict.get((i, j), [])
                if len(vals) < 5:
                    row_line += f"{'---':>{COL_W}}"
                else:
                    med = percentile(vals, 50)
                    any_data = True
                    s = f"{'+' if med >= 0 else ''}{med:.1f}%({len(vals)})"
                    tag = ("⛔" if abs(med) > THR["stft_alarm"]
                           else "⚠" if abs(med) > THR["stft_warn"]
                           else "·" if abs(med) > THR["stft_note"]
                           else "")
                    row_line += f"{s+tag:>{COL_W}}"
            if any_data:
                out.append(row_line)
        return out

    lines += render_map("STFT B1 — median per cell (B1 CL rows only)", cell_b1)
    lines += render_map("STFT B2 — median per cell (B2 CL rows only)", cell_b2)

    # ── Tuning correction table ───────────────────────────────────────────────
    lines.append(f"\n  TUNING CORRECTIONS — cells where |avg STFT| > {THR['stft_note']:.0f}%")
    lines.append(f"  Action on ip_mff_cor cells : multiply by correction factor")
    lines.append(f"  {'RPM range':<14} {'Load':>7} {'B1 med':>8} {'B2 med':>8} "
                 f"{'avg Δ':>8}  {'Factor':>7}  {'Severity'}")
    lines.append("  " + "─" * 66)

    corrections = []
    for i in range(n_rb):
        for j in range(n_lb):
            v1 = cell_b1.get((i, j), [])
            v2 = cell_b2.get((i, j), [])
            med1 = percentile(v1, 50) if len(v1) >= 5 else None
            med2 = percentile(v2, 50) if len(v2) >= 5 else None
            if med1 is None and med2 is None:
                continue
            vals = [v for v in [med1, med2] if v is not None]
            avg_d = sum(vals) / len(vals)
            if abs(avg_d) < THR["stft_note"]:
                continue
            factor = 1.0 + avg_d / 100.0
            corrections.append((i, j, med1, med2, avg_d, factor))

    # Sort by |avg_d| descending
    corrections.sort(key=lambda x: abs(x[4]), reverse=True)
    for i, j, med1, med2, avg_d, factor in corrections:
        severity = ("⛔ CRITICAL" if abs(avg_d) > THR["stft_alarm"]
                    else "⚠  HIGH   " if abs(avg_d) > THR["stft_warn"]
                    else "·  notable")
        s1_str = fmt_pct(med1) if med1 is not None else "    ---"
        s2_str = fmt_pct(med2) if med2 is not None else "    ---"
        lines.append(f"  {RPM_LABELS[i]:<14} {LOAD_LABELS[j]:>7} "
                     f"{s1_str}  {s2_str}  {fmt_pct(avg_d, 8)}  "
                     f"×{factor:.3f}   {severity}")

    if not corrections:
        lines.append("  No cells with |STFT| > 8% — map well calibrated in covered range")

    # ════════════════════════════════════════════════════════════════════════════
    # 5 — LTFT
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(5, "LTFT — LONG-TERM FUEL TRIM"))
    mul1_v = [r["ltft_mul_b1"] for r in run_rows]
    mul2_v = [r["ltft_mul_b2"] for r in run_rows]
    add1_v = [r["ltft_add_b1"] for r in run_rows]
    add2_v = [r["ltft_add_b2"] for r in run_rows]

    if has_ltft:
        mul1 = percentile(mul1_v, 50)
        mul2 = percentile(mul2_v, 50)
        add1 = percentile(add1_v, 50)
        add2 = percentile(add2_v, 50)
        imbal = mul1 - mul2
        f_m1 = flag(mul1,  THR["ltft_warn"], THR["ltft_alarm"])
        f_m2 = flag(mul2,  THR["ltft_warn"], THR["ltft_alarm"])
        f_im = flag(imbal, THR["ltft_imbalance"], THR["ltft_imbalance"] * 1.5)

        lines.append(f"  Multiplicative:  B1 {fmt_pct(mul1)}{f_m1}   "
                     f"B2 {fmt_pct(mul2)}{f_m2}   Δ = {fmt_pct(imbal)}{f_im}")
        lines.append(f"  Additive:        B1 {add1:+.3f} mg/stk   B2 {add2:+.3f} mg/stk")

        mul1_range = max(mul1_v) - min(mul1_v)
        mul2_range = max(mul2_v) - min(mul2_v)
        stable_str = ("stable" if mul1_range < 0.5 and mul2_range < 0.5 else "still drifting")
        lines.append(f"  Stability:       B1 range {mul1_range:.3f}%  B2 range {mul2_range:.3f}%  ({stable_str})")

        ceiling = 20.0
        lines.append(f"  Ceiling margin:  B1 {ceiling-abs(mul1):.1f}% left   "
                     f"B2 {ceiling-abs(mul2):.1f}% left  (ceiling ±{ceiling:.0f}%)")

        if abs(imbal) > THR["ltft_imbalance"]:
            lines.append(f"\n  ⚠ ASYMMETRY: {imbal:+.1f}% gap B1/B2")
            lines.append(f"    → Swap B1↔B2 lambda sensors: if asymmetry flips → sensor issue")
            lines.append(f"    → If asymmetry stays → mechanical (injector, compression, intake)")
    else:
        lines.append("  LTFT = 0  — no adaptation yet (reset or session too short)")
        lines.append("  → Need 30–40+ min of CL operation to accumulate meaningful LTFT")

    # ════════════════════════════════════════════════════════════════════════════
    # 6 — Lambda
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(6, "LAMBDA ANALYSIS"))

    if has_lmb:
        # Per-bank lambda: filter to rows where that bank's CL is active
        lmb1_cl = [r["lmb_b1"] for r in cl_b1_rows if 0.3 < r["lmb_b1"] < 2.5]
        lmb2_cl = [r["lmb_b2"] for r in cl_b2_rows if 0.3 < r["lmb_b2"] < 2.5]

        if lmb1_cl:
            mu1 = statistics.mean(lmb1_cl)
            sd1 = statistics.stdev(lmb1_cl) if len(lmb1_cl) > 1 else 0
            lines.append(f"  CL B1  mean={mu1:.4f}  std={sd1:.4f}  "
                         f"p5={percentile(lmb1_cl,5):.3f}  "
                         f"p50={percentile(lmb1_cl,50):.3f}  "
                         f"p95={percentile(lmb1_cl,95):.3f}  (n={len(lmb1_cl)})")
        if lmb2_cl:
            mu2 = statistics.mean(lmb2_cl)
            sd2 = statistics.stdev(lmb2_cl) if len(lmb2_cl) > 1 else 0
            lines.append(f"  CL B2  mean={mu2:.4f}  std={sd2:.4f}  "
                         f"p5={percentile(lmb2_cl,5):.3f}  "
                         f"p50={percentile(lmb2_cl,50):.3f}  "
                         f"p95={percentile(lmb2_cl,95):.3f}  (n={len(lmb2_cl)})")

        # Stoichiometric crossing rate
        cross_b1 = sum(1 for i in range(1, len(lmb1_cl))
                       if (lmb1_cl[i-1] - 1.0) * (lmb1_cl[i] - 1.0) < 0)
        cross_b2 = sum(1 for i in range(1, len(lmb2_cl))
                       if (lmb2_cl[i-1] - 1.0) * (lmb2_cl[i] - 1.0) < 0)
        cl_dur = sum(p_dur.get(ph, 0) for ph in CL_PHASES)
        if cl_dur > 0 and (lmb1_cl or lmb2_cl):
            hz1 = cross_b1 / cl_dur if lmb1_cl else 0
            hz2 = cross_b2 / cl_dur if lmb2_cl else 0
            ok1 = "OK" if 0.3 < hz1 < 3.0 else ("SLOW" if hz1 < 0.3 else "FAST")
            ok2 = "OK" if 0.3 < hz2 < 3.0 else ("SLOW" if hz2 < 0.3 else "FAST")
            lines.append(f"\n  λ crossings:  B1 {hz1:.2f} Hz [{ok1}]   B2 {hz2:.2f} Hz [{ok2}]")
            lines.append(f"  (stoich crossings — healthy PID oscillates 0.3–2 Hz)")

        # Lambda vs load (B2 filter: B2 CL rows only)
        lines.append(f"\n  LAMBDA vs LOAD  (each bank's CL rows only)")
        lines.append(f"  {'Load':>8}  {'λB1 p50':>9}  {'nB1':>5}  "
                     f"{'λB2 p50':>9}  {'nB2':>5}  {'ΔB1-B2':>9}")
        load_ranges = [(0, 20), (20, 40), (40, 60), (60, 80), (80, 100)]
        for lo, hi in load_ranges:
            r1 = [r for r in cl_b1_rows if lo <= r["load_pct"] < hi
                  and 0.3 < r["lmb_b1"] < 2.5]
            r2 = [r for r in cl_b2_rows if lo <= r["load_pct"] < hi
                  and 0.3 < r["lmb_b2"] < 2.5]
            if len(r1) < 5 and len(r2) < 5:
                continue
            med1 = percentile([r["lmb_b1"] for r in r1], 50) if r1 else None
            med2 = percentile([r["lmb_b2"] for r in r2], 50) if r2 else None
            delta = (med1 - med2) if med1 and med2 else None

            s1 = f"{med1:.4f}" if med1 else "---"
            s2 = f"{med2:.4f}" if med2 else "---"
            sd = f"{delta:+.4f}" if delta else "---"

            f1 = (" ⚠LEAN" if med1 and med1 > THR["lmb_lean_load"]
                  else " ⚠RICH" if med1 and med1 < THR["lmb_rich"] else "")
            f2 = (" ⚠LEAN" if med2 and med2 > THR["lmb_lean_load"]
                  else " ⚠RICH" if med2 and med2 < THR["lmb_rich"] else "")
            fd = (" ⚠IMBAL" if delta and abs(delta) > 0.04 else "")

            lines.append(f"  {lo:>3}–{hi:<3}%  {s1:>9}{f1}  {len(r1):>5}  "
                         f"{s2:>9}{f2}  {len(r2):>5}  {sd:>9}{fd}")

        # WOT lambda — only hot engine, RPM > 2500
        wot_r1 = [r for r in cl_b1_rows if r["phase"] == "WOT"
                  and 0.3 < r["lmb_b1"] < 2.0 and r["rpm"] > 2500
                  and r["coolant_c"] > 75]
        wot_r2 = [r for r in cl_b2_rows if r["phase"] == "WOT"
                  and 0.3 < r["lmb_b2"] < 2.0 and r["rpm"] > 2500
                  and r["coolant_c"] > 75]
        if wot_r1 or wot_r2:
            m1 = statistics.mean(r["lmb_b1"] for r in wot_r1) if wot_r1 else None
            m2 = statistics.mean(r["lmb_b2"] for r in wot_r2) if wot_r2 else None
            s1 = f"{m1:.4f}" if m1 else "---"
            s2 = f"{m2:.4f}" if m2 else "---"
            f1 = (" ⚠LEAN" if m1 and m1 > 0.95 else " OK(E85)" if m1 and m1 < 0.93 else "")
            f2 = (" ⚠LEAN" if m2 and m2 > 0.95 else " OK(E85)" if m2 and m2 < 0.93 else "")
            lines.append(f"\n  WOT λ (>2500 RPM, hot): B1 {s1}{f1}   B2 {s2}{f2}")
            lines.append(f"  (E85 WOT target ~0.85–0.92 for power)")

        # Fuel-cut open-loop (sensor check)
        fc_r1 = [r for r in rows if r["phase"] == "FUEL_CUT"
                 and 0.5 < r["lmb_b1"] < 4.0 and r["rpm"] > 1200]
        fc_r2 = [r for r in rows if r["phase"] == "FUEL_CUT"
                 and 0.5 < r["lmb_b2"] < 4.0 and r["rpm"] > 1200]
        if fc_r1:
            fl1 = [r["lmb_b1"] for r in fc_r1]
            p90_1 = percentile(fl1, 90)
            # p90 < 1.3 means sensor never reaches lean territory even on long FC → suspect
            f_fc = " ⚠ sensor never goes lean — check sensor" if p90_1 < 1.3 else ""
            lines.append(f"\n  Fuel-cut λ B1 : p50={percentile(fl1,50):.3f}  "
                         f"p90={p90_1:.3f}  "
                         f"max={max(fl1):.3f}  "
                         f"(open loop{f_fc})")
        if fc_r2:
            fl2 = [r["lmb_b2"] for r in fc_r2]
            p90_2 = percentile(fl2, 90)
            f_fc = " ⚠ sensor never goes lean — check sensor" if p90_2 < 1.3 else ""
            lines.append(f"  Fuel-cut λ B2 : p50={percentile(fl2,50):.3f}  "
                         f"p90={p90_2:.3f}  "
                         f"max={max(fl2):.3f}{f_fc}")
    else:
        lines.append("  [lambda columns not present]")

    # ════════════════════════════════════════════════════════════════════════════
    # 7 — B1/B2 asymmetry (only rows where BOTH banks are in CL)
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(7, "B1 / B2 ASYMMETRY  (rows where BOTH banks active simultaneously)"))

    if cl_both_rows:
        delta_all = [r["stft_b1"] - r["stft_b2"] for r in cl_both_rows]
        med_d  = percentile(delta_all, 50)
        p5_d   = percentile(delta_all, 5)
        p95_d  = percentile(delta_all, 95)
        sd_d   = statistics.stdev(delta_all) if len(delta_all) > 1 else 0

        lines.append(f"  n={len(cl_both_rows)} rows with both banks active simultaneously")
        lines.append(f"  STFT B1-B2 :  p50={med_d:+.2f}%  std={sd_d:.2f}%  "
                     f"p5={p5_d:+.1f}%  p95={p95_d:+.1f}%")
        f_asym = flag(med_d, THR["stft_imbalance"], THR["stft_imbalance"] * 1.5)
        if abs(med_d) > THR["stft_imbalance"]:
            lines.append(f"  ⚠ PERSISTENT IMBALANCE: {med_d:+.2f}%{f_asym}")
        elif abs(med_d) > 3:
            lines.append(f"  · Slight systematic offset: {med_d:+.2f}%  — monitor")
        else:
            lines.append(f"  Banks balanced: {med_d:+.2f}%  within tolerance ✓")

        lines.append(f"\n  Asymmetry vs coolant:")
        for t_lo, t_hi in [(30, 60), (60, 80), (80, 95), (95, 115)]:
            tr = [r["stft_b1"] - r["stft_b2"] for r in cl_both_rows
                  if t_lo <= r["coolant_c"] < t_hi]
            if len(tr) < 10:
                continue
            lines.append(f"  {t_lo}–{t_hi}°C : Δ={percentile(tr,50):+.2f}%  (n={len(tr)})")

        lines.append(f"\n  Asymmetry vs RPM:")
        for r_lo, r_hi in [(600, 1200), (1200, 2000), (2000, 3000), (3000, 6000)]:
            rr = [r["stft_b1"] - r["stft_b2"] for r in cl_both_rows
                  if r_lo <= r["rpm"] < r_hi]
            if len(rr) < 10:
                continue
            lines.append(f"  {r_lo}–{r_hi} RPM : Δ={percentile(rr,50):+.2f}%  (n={len(rr)})")

        lines.append(f"\n  Asymmetry vs load:")
        for lo, hi in [(0, 20), (20, 40), (40, 60), (60, 100)]:
            lr = [r["stft_b1"] - r["stft_b2"] for r in cl_both_rows
                  if lo <= r["load_pct"] < hi]
            if len(lr) < 10:
                continue
            lines.append(f"  {lo}–{hi}% load : Δ={percentile(lr,50):+.2f}%  (n={len(lr)})")
    else:
        lines.append("  No rows where B1 and B2 are simultaneously in CL")

    # ════════════════════════════════════════════════════════════════════════════
    # 8 — Injection / duty cycle
    # ════════════════════════════════════════════════════════════════════════════
    if has_inj:
        lines.append(H(8, "INJECTION — DUTY CYCLE"))
        lines.append("  DC formula: inj_ms / (120000/RPM) × 100  [4-stroke, 1 fire per 720°]")
        inj_run = [r for r in run_rows if r["inj_ms"] > 0.5 and r["rpm"] > 0]
        if inj_run:
            duties = [r["inj_ms"] / (120000.0 / r["rpm"]) * 100.0 for r in inj_run]
            d_avg = statistics.mean(duties)
            d_p95 = percentile(duties, 95)
            d_max = max(duties)
            f_dc  = flag(d_max, THR["inj_duty_warn"], 95.0)
            lines.append(f"  Overall :  avg {d_avg:.1f}%  p95 {d_p95:.1f}%  max {d_max:.1f}%{f_dc}")

            if d_max > THR["inj_duty_warn"]:
                peak_r = max(inj_run, key=lambda r: r["inj_ms"] / (120000.0 / r["rpm"]))
                peak_dc = peak_r["inj_ms"] / (120000.0 / peak_r["rpm"]) * 100.0
                lines.append(f"  Peak DC {peak_dc:.1f}% @ RPM={peak_r['rpm']:.0f}  "
                             f"Inj={peak_r['inj_ms']:.2f}ms  Load={peak_r['load_pct']:.0f}%"
                             f"  t={ts(peak_r):.0f}s")

            lines.append(f"\n  {'Phase':<14}  {'Inj avg':>8}  {'Inj max':>8}  {'DC avg':>8}  {'DC max':>8}")
            for ph in CL_PHASES:
                pr = [r for r in inj_run if r["phase"] == ph]
                if not pr:
                    continue
                iv  = [r["inj_ms"] for r in pr]
                dcv = [r["inj_ms"] / (120000.0 / r["rpm"]) * 100.0 for r in pr if r["rpm"] > 0]
                lines.append(f"  {ph:<14}  {statistics.mean(iv):>7.2f}ms  "
                             f"{max(iv):>7.2f}ms  "
                             f"{statistics.mean(dcv) if dcv else 0:>7.1f}%  "
                             f"{max(dcv) if dcv else 0:>7.1f}%")

    # ════════════════════════════════════════════════════════════════════════════
    # 9 — VANOS
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(9, "VANOS — INLET CAMSHAFT TRACKING"))
    if has_vanos:
        v_active = [r for r in rows if abs(r["vanos_tgt"]) > 1 and r["rpm"] > 800]
        v_steady = [r for r in steady_rows if abs(r["vanos_tgt"]) > 1]

        if v_active:
            err_a = [r["vanos_err"] for r in v_active]
            err_s = [r["vanos_err"] for r in v_steady]
            abs_a = [abs(e) for e in err_a]
            abs_s = [abs(e) for e in err_s]

            lines.append(f"  All active (n={len(err_a)}):")
            lines.append(f"    mean {statistics.mean(err_a):+.2f}°  "
                         f"p50 {percentile(err_a,50):+.2f}°  "
                         f"p95 |{percentile(abs_a,95):.1f}°|  "
                         f"max |{max(abs_a):.1f}°|")

            if err_s:
                mean_ss = statistics.mean(err_s)
                f_ss    = flag(mean_ss, THR["vanos_steady_warn"], THR["vanos_steady_alarm"])
                ok_pct  = 100 * sum(1 for e in err_s
                                    if abs(e) < THR["vanos_steady_warn"]) / len(err_s)
                p99_ss = percentile(abs_s, 99)
                lines.append(f"  Steady-state (n={len(err_s)}, transients removed):")
                lines.append(f"    mean {mean_ss:+.2f}°{f_ss}  "
                             f"p95 |{percentile(abs_s,95):.1f}°|  "
                             f"p99 |{p99_ss:.1f}°|  —  "
                             f"{ok_pct:.1f}% within ±{THR['vanos_steady_warn']:.0f}°")

            lines.append(f"\n  Steady-state error vs RPM:")
            for r_lo, r_hi in [(800, 1500), (1500, 2500), (2500, 3500), (3500, 6000)]:
                rr = [r["vanos_err"] for r in v_steady if r_lo <= r["rpm"] < r_hi]
                if len(rr) < 5:
                    continue
                lines.append(f"  {r_lo}–{r_hi} RPM :  mean {statistics.mean(rr):+.2f}°  "
                             f"max|{max(abs(e) for e in rr):.1f}°|  n={len(rr)}")

            act_v = [r["vanos_act"] for r in v_active]
            tgt_v = [r["vanos_tgt"] for r in v_active]
            lines.append(f"\n  Actual : {min(act_v):.0f}° – {max(act_v):.0f}°")
            lines.append(f"  Target : {min(tgt_v):.0f}° – {max(tgt_v):.0f}°")
        else:
            lines.append("  No VANOS active samples")
    else:
        lines.append("  VANOS columns not present in this log")

    # ════════════════════════════════════════════════════════════════════════════
    # 10 — Ignition / knock
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(10, "IGNITION — ADVANCE & KNOCK PROXY"))
    lines.append(f"  Knock detection: hot engine only (>{THR['knock_hot_coolant']:.0f}°C), "
                 f"RPM>{THR['knock_rpm_min']:.0f}, Load>{THR['knock_load_min']:.0f}%, "
                 f">{THR['knock_delta']:.0f}° below local p50")

    if has_ign:
        ign_run = [r for r in run_rows if abs(r["ign_deg"]) > NEAR_ZERO]
        ign_hot = [r for r in ign_run if r["coolant_c"] > THR["knock_hot_coolant"]]

        if ign_run:
            ign_v = [r["ign_deg"] for r in ign_run]
            lines.append(f"\n  Overall:  avg {statistics.mean(ign_v):+.1f}°  "
                         f"p5 {percentile(ign_v,5):+.1f}°  "
                         f"p50 {percentile(ign_v,50):+.1f}°  "
                         f"p95 {percentile(ign_v,95):+.1f}°  "
                         f"max {max(ign_v):+.1f}°")

        if ign_hot:
            # Knock: only on hot engine, RPM > threshold, Load > threshold
            candidates = [r for r in ign_hot
                          if r["rpm"] > THR["knock_rpm_min"]
                          and r["load_pct"] > THR["knock_load_min"]
                          and (r["rb"], r["lb"]) in cell_ign_p50]
            retard_events = [r for r in candidates
                             if r["ign_deg"] < cell_ign_p50[(r["rb"], r["lb"])] - THR["knock_delta"]]
            ret_pct = 100 * len(retard_events) / len(candidates) if candidates else 0
            lines.append(f"  Retard events: {len(retard_events)}/{len(candidates)} "
                         f"= {ret_pct:.1f}%  (hot, RPM>{THR['knock_rpm_min']:.0f}, "
                         f"Load>{THR['knock_load_min']:.0f}%)")

            if ret_pct > 5:
                lines.append(f"  ⚠ Significant retard activity — check fuel quality")
            elif ret_pct > 1:
                lines.append(f"  · Some retard events — transients, monitor")
            else:
                lines.append(f"  Clean — no sustained knock retard detected ✓")

            # By phase
            lines.append(f"\n  {'Phase':<14}  {'Ign avg':>8}  {'p5':>8}  {'p50':>8}  {'p95':>8}  {'Retards':>8}")
            for ph in CL_PHASES:
                pi = [r for r in ign_hot if r["phase"] == ph]
                if not pi:
                    continue
                iv = [r["ign_deg"] for r in pi]
                ret = sum(1 for r in retard_events if r["phase"] == ph)
                lines.append(f"  {ph:<14}  "
                             f"{statistics.mean(iv):>+7.1f}°  "
                             f"{percentile(iv,5):>+7.1f}°  "
                             f"{percentile(iv,50):>+7.1f}°  "
                             f"{percentile(iv,95):>+7.1f}°  {ret:>8}")

            # WOT advance
            wot_ign = [r["ign_deg"] for r in ign_hot
                       if r["phase"] == "WOT" and r["rpm"] > 2500]
            if wot_ign:
                lines.append(f"\n  WOT (>2500 RPM, hot): "
                             f"avg {statistics.mean(wot_ign):+.1f}°  "
                             f"max {max(wot_ign):+.1f}°")
                if statistics.mean(wot_ign) < 15:
                    lines.append(f"  ⚠ Low WOT advance — E85 should allow higher timing")
        else:
            lines.append("  No hot-engine ignition data")
    else:
        lines.append("  Ignition column not present")

    # ════════════════════════════════════════════════════════════════════════════
    # 11 — Anomaly timeline
    # ════════════════════════════════════════════════════════════════════════════
    lines.append(H(11, "ANOMALY EVENT TIMELINE"))

    events = []
    stft_b1_streak = 0
    stft_b2_streak = 0
    lean_b1_streak = 0
    lean_b2_streak = 0
    LEAN_DEBOUNCE = 3  # require N consecutive samples before flagging

    for i, row in enumerate(rows):
        t = ts(row)

        # STFT saturation (sustained, per-bank CL filter)
        if row["cl_b1"]:
            s1 = row["stft_b1"]
            if abs(s1) > THR["stft_alarm"]:
                stft_b1_streak += 1
                if stft_b1_streak == 6:
                    events.append(("STFT_B1", t,
                                   f"B1 sustained {s1:+.1f}% (>{THR['stft_alarm']:.0f}%)  "
                                   f"RPM={row['rpm']:.0f} Load={row['load_pct']:.0f}%"))
            else:
                stft_b1_streak = 0

        if row["cl_b2"]:
            s2 = row["stft_b2"]
            if abs(s2) > THR["stft_alarm"]:
                stft_b2_streak += 1
                if stft_b2_streak == 6:
                    events.append(("STFT_B2", t,
                                   f"B2 sustained {s2:+.1f}% (>{THR['stft_alarm']:.0f}%)  "
                                   f"RPM={row['rpm']:.0f} Load={row['load_pct']:.0f}%"))
            else:
                stft_b2_streak = 0

        # Lambda lean (debounced — require 3 consecutive samples)
        if has_lmb and row["load_pct"] > 35 and row["coolant_c"] > 60:
            if row["lmb_b1"] > THR["lmb_lean_load"]:
                lean_b1_streak += 1
                if lean_b1_streak == LEAN_DEBOUNCE:
                    events.append(("LEAN_B1", t,
                                   f"λB1={row['lmb_b1']:.3f} LEAN (>{THR['lmb_lean_load']:.2f}) "
                                   f"@ Load={row['load_pct']:.0f}% RPM={row['rpm']:.0f}"))
            else:
                lean_b1_streak = 0

            if row["lmb_b2"] > THR["lmb_lean_load"]:
                lean_b2_streak += 1
                if lean_b2_streak == LEAN_DEBOUNCE:
                    events.append(("LEAN_B2", t,
                                   f"λB2={row['lmb_b2']:.3f} LEAN "
                                   f"@ Load={row['load_pct']:.0f}% RPM={row['rpm']:.0f}"))
            else:
                lean_b2_streak = 0

        # Lambda rich + high negative STFT (both banks independently)
        if has_lmb and row["cl_b1"]:
            if row["lmb_b1"] < THR["lmb_rich"] and row["stft_b1"] < -THR["stft_warn"]:
                events.append(("RICH_B1", t,
                               f"λB1={row['lmb_b1']:.3f}  STFT {row['stft_b1']:+.1f}%"))
        if has_lmb and row["cl_b2"]:
            if row["lmb_b2"] < THR["lmb_rich"] and row["stft_b2"] < -THR["stft_warn"]:
                events.append(("RICH_B2", t,
                               f"λB2={row['lmb_b2']:.3f}  STFT {row['stft_b2']:+.1f}%"))

        # VANOS — large error (transient spike threshold = steady_alarm × 2.5)
        if has_vanos and row["rpm"] > 1000:
            if abs(row["vanos_err"]) > THR["vanos_steady_alarm"] * 2.5:
                events.append(("VANOS", t,
                               f"err={row['vanos_err']:+.1f}°  "
                               f"act={row['vanos_act']:.0f}° tgt={row['vanos_tgt']:.0f}°  "
                               f"RPM={row['rpm']:.0f}"))

        # Coolant
        if row["coolant_c"] > THR["coolant_warn"]:
            events.append(("OVERTEMP", t, f"Coolant {row['coolant_c']:.1f}°C"))

    # Deduplicate: keep first per type per time-window
    DEDUP_WINDOW = {"STFT_B1": 15, "STFT_B2": 15,
                    "LEAN_B1": 20, "LEAN_B2": 20,
                    "RICH_B1": 15, "RICH_B2": 15,
                    "VANOS":   30, "OVERTEMP": 60}
    seen = {}
    deduped = []
    for ev_type, t, msg in sorted(events, key=lambda e: e[1]):
        w = DEDUP_WINDOW.get(ev_type, 15)
        key = (ev_type, int(t / w))
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
    if cl_b1_rows:
        med1 = percentile([r["stft_b1"] for r in cl_b1_rows], 50)
        sat1 = pct_outside([r["stft_b1"] for r in cl_b1_rows], THR["stft_alarm"])
        if sat1 > 5:
            issues.append(f"STFT B1 {sat1:.1f}% rows >|{THR['stft_alarm']:.0f}%| — map richness off")
        elif abs(med1) > THR["stft_warn"]:
            warnings.append(f"STFT B1 median {med1:+.1f}% — elevated, adjust ip_mff_cor")
        else:
            oks.append(f"STFT B1 median {med1:+.1f}% — OK")

    if cl_b2_rows:
        med2 = percentile([r["stft_b2"] for r in cl_b2_rows], 50)
        sat2 = pct_outside([r["stft_b2"] for r in cl_b2_rows], THR["stft_alarm"])
        if sat2 > 5:
            issues.append(f"STFT B2 {sat2:.1f}% rows >|{THR['stft_alarm']:.0f}%| — map richness off")
        elif abs(med2) > THR["stft_warn"]:
            warnings.append(f"STFT B2 median {med2:+.1f}% — elevated, adjust ip_mff_cor")
        else:
            oks.append(f"STFT B2 median {med2:+.1f}% — OK")

    # STFT imbalance (both-active rows)
    if cl_both_rows:
        delta_med = percentile([r["stft_b1"] - r["stft_b2"] for r in cl_both_rows], 50)
        if abs(delta_med) > THR["stft_imbalance"]:
            issues.append(f"B1/B2 persistent gap {delta_med:+.1f}% — sensor or mechanical")
        elif abs(delta_med) > 4:
            warnings.append(f"B1/B2 slight asymmetry {delta_med:+.1f}%")

    # LTFT
    if has_ltft:
        mul1 = percentile([r["ltft_mul_b1"] for r in run_rows], 50)
        mul2 = percentile([r["ltft_mul_b2"] for r in run_rows], 50)
        if abs(mul1) > THR["ltft_alarm"] or abs(mul2) > THR["ltft_alarm"]:
            issues.append(f"LTFT near ceiling: B1={mul1:+.1f}% B2={mul2:+.1f}% — map wrong")
        elif abs(mul1) > THR["ltft_warn"] or abs(mul2) > THR["ltft_warn"]:
            warnings.append(f"LTFT elevated: B1={mul1:+.1f}% B2={mul2:+.1f}%")
        else:
            oks.append(f"LTFT: B1={mul1:+.1f}% B2={mul2:+.1f}% — OK")
        if abs(mul1 - mul2) > THR["ltft_imbalance"]:
            issues.append(f"LTFT asymmetry {mul1-mul2:+.1f}% — sensor or mechanical fault")
    else:
        oks.append("LTFT = 0 (not yet adapted)")

    # Lambda WOT
    if has_lmb and wot_r1:
        m = statistics.mean(r["lmb_b1"] for r in wot_r1)
        if m > 0.98:
            issues.append(f"WOT lambda B1 {m:.3f} — LEAN at full load, safety risk ⛔")
        elif m > 0.94:
            warnings.append(f"WOT lambda B1 {m:.3f} — slightly lean at WOT")
        else:
            oks.append(f"WOT lambda B1 {m:.3f} — rich enough for E85")

    # Injector duty
    if has_inj:
        inj_r = [r for r in run_rows if r["inj_ms"] > 0.5 and r["rpm"] > 0]
        if inj_r:
            dc_max = max(r["inj_ms"] / (120000.0 / r["rpm"]) * 100.0 for r in inj_r)
            if dc_max > 90:
                issues.append(f"Injector duty {dc_max:.1f}% peak — at physical limit ⛔")
            elif dc_max > THR["inj_duty_warn"]:
                warnings.append(f"Injector duty {dc_max:.1f}% peak — headroom limited")
            else:
                oks.append(f"Injector duty OK (max {dc_max:.1f}%)")

    # Coolant
    if cool_max > THR["coolant_warn"]:
        issues.append(f"Coolant {cool_max:.0f}°C peak — check cooling system")
    else:
        oks.append(f"Coolant OK (max {cool_max:.0f}°C)")

    # Knock
    if has_ign and ign_hot:
        cands = [r for r in ign_hot
                 if r["rpm"] > THR["knock_rpm_min"] and r["load_pct"] > THR["knock_load_min"]
                 and (r["rb"], r["lb"]) in cell_ign_p50]
        ret   = [r for r in cands
                 if r["ign_deg"] < cell_ign_p50[(r["rb"], r["lb"])] - THR["knock_delta"]]
        ret_p = 100 * len(ret) / len(cands) if cands else 0
        if ret_p > 5:
            warnings.append(f"Knock proxy: {len(ret)} retard events ({ret_p:.1f}%) — monitor")
        else:
            oks.append(f"Ignition retard: {len(ret)} events ({ret_p:.1f}%) — clean")

    status = ("⛔ CRITICAL" if issues else "⚠  WARNINGS" if warnings else "✓  OK      ")
    lines.append(f"  STATUS : {status}")

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


# ─── Entry point ──────────────────────────────────────────────────────────────────
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
