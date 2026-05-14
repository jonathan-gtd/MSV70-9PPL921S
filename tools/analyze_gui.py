#!/usr/bin/env python3
"""
MSV70 Analyzer GUI — tkinter + matplotlib dark theme
Run: python tools/analyze_gui.py [optional_file.csv]
"""

import sys
import threading
import statistics
from pathlib import Path
import tkinter as tk
from tkinter import ttk, filedialog

import matplotlib
matplotlib.use("TkAgg")
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
from matplotlib.figure import Figure
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
import numpy as np

sys.path.insert(0, str(Path(__file__).parent))
from analyze_diag import (
    compute_data, analyze, find_latest_csv,
    THR, percentile, RPM_LABELS, LOAD_LABELS, RPM_BINS, LOAD_BINS, NEAR_ZERO,
)

# ─── Palette (VS Code dark) ───────────────────────────────────────────────────
BG      = "#1e1e1e"
BG2     = "#252526"
BG3     = "#2d2d30"
FG      = "#d4d4d4"
FG_DIM  = "#6a6a6a"
ACCENT  = "#569cd6"
GREEN   = "#4ec9b0"
YELLOW  = "#dcdcaa"
RED     = "#f44747"
ORANGE  = "#ce9178"
PURPLE  = "#c586c0"

plt.rcParams.update({
    "figure.facecolor":  BG,
    "axes.facecolor":    BG2,
    "axes.edgecolor":    BG3,
    "axes.labelcolor":   FG_DIM,
    "xtick.color":       FG_DIM,
    "ytick.color":       FG_DIM,
    "text.color":        FG,
    "grid.color":        BG3,
    "grid.linewidth":    0.5,
    "lines.linewidth":   1.2,
    "font.size":         8,
    "axes.titlesize":    9,
    "axes.titlecolor":   FG,
    "legend.framealpha": 0,
    "legend.labelcolor": FG,
    "legend.fontsize":   7,
})

PHASE_COLORS = {
    "HOT_IDLE": GREEN, "WARMUP": "#9cdcfe", "CRUISE": ACCENT,
    "ACCEL": YELLOW,   "WOT": RED,          "FUEL_CUT": FG_DIM,
}


# ─── Helpers ──────────────────────────────────────────────────────────────────
def _clear_tab(frame):
    for w in frame.winfo_children():
        w.destroy()


def _embed(fig, parent):
    canvas = FigureCanvasTkAgg(fig, master=parent)
    canvas.draw()
    widget = canvas.get_tk_widget()
    widget.pack(fill="both", expand=True)
    return canvas


def _stft_color(v):
    if v is None or np.isnan(v):
        return FG_DIM
    if abs(v) > THR["stft_alarm"]:
        return RED
    if abs(v) > THR["stft_warn"]:
        return ORANGE
    if abs(v) > THR["stft_note"]:
        return YELLOW
    return GREEN


# ─── Smart diagnostics ────────────────────────────────────────────────────────
def smart_tips(d):
    tips = []

    # WOT lean
    if d["wot_r1"]:
        wot_m = statistics.mean(r["lmb_b1"] for r in d["wot_r1"])
        if wot_m > 0.96:
            tips.append(
                f"WOT λ={wot_m:.3f} LEAN — injector may be maxed out or ip_mff_cor too low at 75–100% load"
            )
        elif wot_m > 0.93:
            tips.append(f"WOT λ={wot_m:.3f} slightly lean — monitor, target <0.92 for E85 power")

    # Corrections
    if d["corrections"]:
        top = d["corrections"][0]
        tips.append(
            f"Strongest correction: {RPM_LABELS[top[0]]} / {LOAD_LABELS[top[1]]}% load → ×{top[5]:.3f} on ip_mff_cor  ({top[4]:+.1f}%)"
        )

    # Injector DC
    if d["has_inj"] and d["run_rows"]:
        dc_max = max(
            (r["inj_ms"] / (120000.0 / r["rpm"]) * 100.0
             for r in d["run_rows"] if r["inj_ms"] > 0.5 and r["rpm"] > 0),
            default=0,
        )
        if dc_max > 90:
            tips.append(f"Injector DC {dc_max:.0f}% — physical limit reached; upgrade injectors or reduce WOT fuel demand")
        elif dc_max > 80:
            tips.append(f"Injector DC {dc_max:.0f}% — headroom <20%; watch carefully at high RPM")

    # LTFT
    if d["has_ltft"] and d["run_rows"]:
        mul1 = percentile([r["ltft_mul_b1"] for r in d["run_rows"]], 50)
        mul2 = percentile([r["ltft_mul_b2"] for r in d["run_rows"]], 50)
        if abs(mul1) > 15 or abs(mul2) > 15:
            tips.append(f"LTFT Mul B1={mul1:+.1f}% B2={mul2:+.1f}% — near ±20% ceiling, reflash base map urgently")
        elif abs(mul1) > 10 or abs(mul2) > 10:
            tips.append(f"LTFT Mul B1={mul1:+.1f}% B2={mul2:+.1f}% — elevated, schedule a base map correction")

    # Knock
    if d["knock_candidates"]:
        ret_pct = 100 * len(d["retard_events"]) / len(d["knock_candidates"])
        if ret_pct > 5:
            tips.append(f"Knock proxy {ret_pct:.1f}% retard rate — verify fuel quality and ignition map")

    # STFT overall
    if d["cl_b1_rows"]:
        med = percentile([r["stft_b1"] for r in d["cl_b1_rows"]], 50)
        if abs(med) < 3:
            tips.append("STFT median within ±3% — base map well calibrated for logged conditions")

    return tips


# ─── Main Application ─────────────────────────────────────────────────────────
class MSV70App(tk.Tk):
    def __init__(self, initial_path=None):
        super().__init__()
        self.title("MSV70 Analyzer  ·  N52B30 E85_v5  ·  9PPL921S")
        self.geometry("1480x880")
        self.minsize(1100, 640)
        self.configure(bg=BG)

        self._data = None
        self._logs_dir = Path(__file__).parent.parent / "LOGS"
        self._tip_idx = 0
        self._tips = []

        self._build_styles()
        self._build_ui()

        if initial_path:
            self._path_var.set(str(initial_path))
            self.after(100, self._run_analyze)
        else:
            self.after(200, self._auto_load)

    # ── Styles ────────────────────────────────────────────────────────────────
    def _build_styles(self):
        s = ttk.Style(self)
        s.theme_use("clam")
        s.configure(".",            background=BG,  foreground=FG, fieldbackground=BG2,
                    troughcolor=BG2, selectbackground=ACCENT, selectforeground=BG)
        s.configure("TFrame",       background=BG)
        s.configure("TLabel",       background=BG,  foreground=FG, font=("Segoe UI", 9))
        s.configure("Dim.TLabel",   background=BG,  foreground=FG_DIM, font=("Segoe UI", 9))
        s.configure("TButton",      background=BG3, foreground=FG, relief="flat",
                    padding=(10, 5), font=("Segoe UI", 9))
        s.map("TButton",
              background=[("active", "#3c3c3c"), ("pressed", BG2)],
              foreground=[("active", FG)])
        s.configure("Accent.TButton", background=ACCENT, foreground=BG, font=("Segoe UI", 9, "bold"))
        s.map("Accent.TButton",
              background=[("active", "#6aaedf"), ("pressed", "#3e7aa6")])
        s.configure("TEntry",       fieldbackground=BG2, foreground=FG,
                    insertcolor=FG, relief="flat", padding=4)
        s.configure("TNotebook",    background=BG, tabmargins=[2, 4, 0, 0])
        s.configure("TNotebook.Tab", background=BG2, foreground=FG_DIM,
                    padding=[14, 6], font=("Segoe UI", 9))
        s.map("TNotebook.Tab",
              background=[("selected", BG3)],
              foreground=[("selected", FG)])
        s.configure("TScrollbar",   background=BG3, troughcolor=BG2,
                    arrowcolor=FG_DIM, relief="flat")
        s.configure("TProgressbar", background=ACCENT, troughcolor=BG2)
        s.configure("TSeparator",   background=BG3)

    # ── UI Layout ─────────────────────────────────────────────────────────────
    def _build_ui(self):
        # Top bar
        top = ttk.Frame(self)
        top.pack(fill="x", padx=10, pady=(8, 0))

        ttk.Label(top, text="MSV70 ANALYZER", font=("Segoe UI", 11, "bold"),
                  foreground=ACCENT).pack(side="left", padx=(0, 18))

        ttk.Label(top, text="Log:", style="Dim.TLabel").pack(side="left")
        self._path_var = tk.StringVar()
        ttk.Entry(top, textvariable=self._path_var, width=52).pack(side="left", padx=(4, 6))
        ttk.Button(top, text="Browse…",    command=self._browse).pack(side="left", padx=2)
        ttk.Button(top, text="▶ Analyze",  command=self._run_analyze,
                   style="Accent.TButton").pack(side="left", padx=6)
        ttk.Button(top, text="⊕ All Logs", command=self._run_compare).pack(side="left", padx=2)

        self._status_var = tk.StringVar(value="Ready — select a CSV or click Analyze to auto-load")
        ttk.Label(top, textvariable=self._status_var, style="Dim.TLabel").pack(side="right", padx=8)

        ttk.Separator(self, orient="horizontal").pack(fill="x", padx=10, pady=(6, 0))

        # Main pane: text left, plots right
        pane = ttk.PanedWindow(self, orient="horizontal")
        pane.pack(fill="both", expand=True, padx=10, pady=6)

        # Left — report
        left = ttk.Frame(pane, width=530)
        left.pack_propagate(False)
        pane.add(left, weight=38)

        self._txt = tk.Text(
            left, bg=BG2, fg=FG, font=("Consolas", 8, "normal"),
            insertbackground=FG, selectbackground="#264f78",
            relief="flat", wrap="none", state="disabled",
            padx=6, pady=6,
        )
        vsb = ttk.Scrollbar(left, orient="vertical",   command=self._txt.yview)
        hsb = ttk.Scrollbar(left, orient="horizontal", command=self._txt.xview)
        self._txt.configure(yscrollcommand=vsb.set, xscrollcommand=hsb.set)
        vsb.pack(side="right",  fill="y")
        hsb.pack(side="bottom", fill="x")
        self._txt.pack(fill="both", expand=True)

        self._txt.tag_configure("alarm",  foreground=RED,    font=("Consolas", 8, "bold"))
        self._txt.tag_configure("warn",   foreground=ORANGE)
        self._txt.tag_configure("ok",     foreground=GREEN)
        self._txt.tag_configure("hdr",    foreground=ACCENT, font=("Consolas", 8, "bold"))
        self._txt.tag_configure("dim",    foreground=FG_DIM)
        self._txt.tag_configure("normal", foreground=FG)

        # Right — plots
        right = ttk.Frame(pane)
        pane.add(right, weight=62)

        self._nb = ttk.Notebook(right)
        self._nb.pack(fill="both", expand=True)

        self._tab_ov  = ttk.Frame(self._nb)
        self._tab_map = ttk.Frame(self._nb)
        self._tab_ts  = ttk.Frame(self._nb)
        self._tab_tr  = ttk.Frame(self._nb)

        self._nb.add(self._tab_ov,  text="  Overview  ")
        self._nb.add(self._tab_map, text="  STFT Map  ")
        self._nb.add(self._tab_ts,  text="  Time Series  ")
        self._nb.add(self._tab_tr,  text="  Trend  ")

        # Tip bar
        self._tip_var = tk.StringVar(value="")
        tip = ttk.Frame(self)
        tip.pack(fill="x", padx=10, pady=(0, 6))
        ttk.Label(tip, text="💡", font=("Segoe UI", 10)).pack(side="left")
        ttk.Label(tip, textvariable=self._tip_var,
                  foreground=ORANGE, font=("Segoe UI", 9),
                  wraplength=1200, justify="left").pack(side="left", padx=6)
        ttk.Button(tip, text="▸", width=3, command=self._next_tip).pack(side="left")

    # ── File / auto-load ──────────────────────────────────────────────────────
    def _browse(self):
        p = filedialog.askopenfilename(
            title="Select MSV70 diagnostic log",
            filetypes=[("CSV", "*.csv"), ("All files", "*.*")],
            initialdir=str(self._logs_dir) if self._logs_dir.exists() else ".",
        )
        if p:
            self._path_var.set(p)

    def _auto_load(self):
        try:
            p = find_latest_csv()
            self._path_var.set(str(p))
            self._run_analyze()
        except FileNotFoundError:
            self._status_var.set("No CSV in LOGS/ — use Browse to select a file")

    # ── Analysis ──────────────────────────────────────────────────────────────
    def _run_analyze(self):
        ps = self._path_var.get().strip()
        if not ps:
            return
        path = Path(ps)
        if not path.exists():
            self._status_var.set(f"Not found: {path.name}")
            return
        self._status_var.set(f"Analyzing {path.name}…")
        self.configure(cursor="watch")
        threading.Thread(target=self._analyze_thread, args=(path,), daemon=True).start()

    def _analyze_thread(self, path):
        try:
            d      = compute_data(path)
            report = analyze(path)
            self.after(0, self._on_analyze_done, d, report)
        except Exception as exc:
            import traceback
            msg = traceback.format_exc()
            self.after(0, self._status_var.set, f"Error: {exc}")
            self.after(0, self.configure, {"cursor": ""})
            print(msg, file=sys.stderr)

    def _on_analyze_done(self, d, report):
        self._data = d
        self._show_report(report)
        self._plot_overview(d)
        self._plot_stft_map(d)
        self._plot_timeseries(d)
        self._tips = smart_tips(d)
        self._tip_idx = 0
        self._tip_var.set(self._tips[0] if self._tips else "All systems nominal — no corrections flagged")
        n_corr = len(d["corrections"])
        n_ev   = len(d["events"])
        self._status_var.set(
            f"✓  {d['path'].name}  ·  {d['n']} rows  ·  {d['dur_s']:.0f}s  ·  "
            f"{n_corr} cells to correct  ·  {n_ev} anomaly events"
        )
        self.configure(cursor="")

    def _next_tip(self):
        if not self._tips:
            return
        self._tip_idx = (self._tip_idx + 1) % len(self._tips)
        self._tip_var.set(self._tips[self._tip_idx])

    # ── Report rendering ──────────────────────────────────────────────────────
    def _show_report(self, text):
        self._txt.configure(state="normal")
        self._txt.delete("1.0", "end")
        for line in text.splitlines():
            if any(c in line for c in ("⛔", "✗", "CRITICAL")):
                tag = "alarm"
            elif any(c in line for c in ("⚠", "WARN", "LEAN", "RICH")):
                tag = "warn"
            elif any(c in line for c in ("✓", "[OK]")):
                tag = "ok"
            elif line.startswith("  [") or "═" in line or "VERDICT" in line:
                tag = "hdr"
            elif "─" in line or line.strip() == "":
                tag = "dim"
            else:
                tag = "normal"
            self._txt.insert("end", line + "\n", tag)
        self._txt.configure(state="disabled")
        self._txt.see("1.0")

    # ── Overview tab ──────────────────────────────────────────────────────────
    def _plot_overview(self, d):
        _clear_tab(self._tab_ov)
        fig = Figure(figsize=(9, 5.5), tight_layout=True)

        ax_phase  = fig.add_subplot(2, 2, 1)
        ax_stft   = fig.add_subplot(2, 2, 2)
        ax_lmb    = fig.add_subplot(2, 2, 3)
        ax_dc     = fig.add_subplot(2, 2, 4)

        # Phase bar
        ph_order = ["HOT_IDLE", "WARMUP", "CRUISE", "ACCEL", "WOT", "FUEL_CUT"]
        names, vals, colors = [], [], []
        for ph in ph_order:
            dur = d["p_dur"].get(ph, 0)
            if dur > 1:
                names.append(ph.replace("_", " "))
                vals.append(dur)
                colors.append(PHASE_COLORS.get(ph, ACCENT))
        if vals:
            bars = ax_phase.barh(names, vals, color=colors, height=0.55, edgecolor="none")
            for bar, v in zip(bars, vals):
                ax_phase.text(bar.get_width() + 1, bar.get_y() + bar.get_height() / 2,
                              f"{v:.0f}s", va="center", fontsize=7, color=FG_DIM)
        ax_phase.set_xlabel("Seconds")
        ax_phase.set_title("Phase Distribution")
        ax_phase.invert_yaxis()

        # STFT grouped bar per phase
        cl_phases = [ph for ph in ["HOT_IDLE", "WARMUP", "CRUISE", "ACCEL", "WOT"]
                     if d["phase_rows"].get(ph)]
        if cl_phases and d["cl_b1_rows"]:
            xs = range(len(cl_phases))
            w = 0.38
            b1m = [percentile([r["stft_b1"] for r in d["cl_b1_rows"] if r["phase"] == ph], 50)
                   if any(r["phase"] == ph for r in d["cl_b1_rows"]) else 0
                   for ph in cl_phases]
            b2m = [percentile([r["stft_b2"] for r in d["cl_b2_rows"] if r["phase"] == ph], 50)
                   if any(r["phase"] == ph for r in d["cl_b2_rows"]) else 0
                   for ph in cl_phases]
            ax_stft.bar([i - w/2 for i in xs], b1m, w, label="B1",
                        color=[_stft_color(v) for v in b1m], alpha=0.85, edgecolor="none")
            ax_stft.bar([i + w/2 for i in xs], b2m, w, label="B2",
                        color=[_stft_color(v) for v in b2m], alpha=0.6,  edgecolor="none")
            ax_stft.axhline(0, color=FG_DIM, lw=0.7, ls="--")
            for thr_v, col in [(-THR["stft_warn"], RED), (THR["stft_warn"], RED),
                                (-THR["stft_note"], YELLOW), (THR["stft_note"], YELLOW)]:
                ax_stft.axhline(thr_v, color=col, lw=0.5, ls=":", alpha=0.5)
            ax_stft.set_xticks(list(xs))
            ax_stft.set_xticklabels([p.replace("_", "\n") for p in cl_phases], fontsize=6.5)
            ax_stft.set_ylabel("STFT %")
            ax_stft.legend(loc="lower right", fontsize=7)
        ax_stft.set_title("STFT Median by Phase")

        # Lambda vs load
        if d["has_lmb"] and d["cl_b1_rows"]:
            load_x, med1, med2 = [], [], []
            for lo in range(0, 95, 5):
                hi = lo + 5
                r1 = [r["lmb_b1"] for r in d["cl_b1_rows"]
                      if lo <= r["load_pct"] < hi and 0.5 < r["lmb_b1"] < 2.0]
                r2 = [r["lmb_b2"] for r in d["cl_b2_rows"]
                      if lo <= r["load_pct"] < hi and 0.5 < r["lmb_b2"] < 2.0]
                if len(r1) >= 5:
                    load_x.append(lo + 2.5)
                    med1.append(percentile(r1, 50))
                    med2.append(percentile(r2, 50) if len(r2) >= 5 else None)
            if load_x:
                ax_lmb.plot(load_x, med1, "o-", color=ACCENT,  ms=3.5, lw=1.4, label="λ B1")
                m2_x = [load_x[i] for i, v in enumerate(med2) if v is not None]
                m2_y = [v for v in med2 if v is not None]
                if m2_y:
                    ax_lmb.plot(m2_x, m2_y, "s-", color=ORANGE, ms=3.5, lw=1.4, label="λ B2")
                ax_lmb.axhline(1.00, color=GREEN,  lw=0.8, ls="--", label="stoich")
                ax_lmb.axhline(0.92, color=YELLOW, lw=0.6, ls=":",  alpha=0.7, label="WOT target")
                ax_lmb.axhline(THR["lmb_lean_load"], color=RED, lw=0.5, ls=":", alpha=0.5)
                ax_lmb.set_ylim(0.85, 1.08)
                ax_lmb.legend(loc="lower left", fontsize=7)
        ax_lmb.set_xlabel("Load %")
        ax_lmb.set_ylabel("λ")
        ax_lmb.set_title("Lambda vs Load (CL rows)")

        # Injector DC scatter
        if d["has_inj"] and d["run_rows"]:
            pts = [(r["rpm"], r["inj_ms"] / (120000.0 / r["rpm"]) * 100.0)
                   for r in d["run_rows"] if r["inj_ms"] > 0.5 and r["rpm"] > 0]
            if pts:
                rpms, dcs = zip(*pts)
                cols = [RED if dc > 80 else (ORANGE if dc > 60 else ACCENT) for dc in dcs]
                ax_dc.scatter(rpms, dcs, c=cols, s=1.0, alpha=0.35, linewidths=0)
                ax_dc.axhline(80, color=RED,    lw=0.8, ls="--", alpha=0.7, label="80% warn")
                ax_dc.axhline(60, color=ORANGE, lw=0.6, ls=":",  alpha=0.5, label="60% note")
                ax_dc.set_ylim(0, min(105, max(dcs) + 5))
                ax_dc.legend(loc="upper left", fontsize=7)
        ax_dc.set_xlabel("RPM")
        ax_dc.set_ylabel("DC %")
        ax_dc.set_title("Injector Duty Cycle vs RPM")

        for ax in [ax_phase, ax_stft, ax_lmb, ax_dc]:
            ax.tick_params(labelsize=7)

        _embed(fig, self._tab_ov)

    # ── STFT 2D Map tab ───────────────────────────────────────────────────────
    def _plot_stft_map(self, d):
        _clear_tab(self._tab_map)

        n_rb = len(RPM_BINS) - 1
        n_lb = len(LOAD_BINS) - 1

        def to_mat(cell_dict):
            mat = np.full((n_rb, n_lb), np.nan)
            cnt = np.zeros((n_rb, n_lb), dtype=int)
            for (i, j), vals in cell_dict.items():
                if len(vals) >= 5:
                    mat[i, j] = percentile(vals, 50)
                    cnt[i, j] = len(vals)
            return mat, cnt

        m1, c1 = to_mat(d["cell_b1"])
        m2, c2 = to_mat(d["cell_b2"])

        fig = Figure(figsize=(12, 5.2), tight_layout={"pad": 1.5, "w_pad": 2.5})
        axes = [fig.add_subplot(1, 2, k) for k in (1, 2)]

        cmap = plt.get_cmap("RdYlGn_r")   # dark-red = rich (neg), dark-green = lean (pos)
        # We invert: STFT negative=rich=we want to shrink fuel → display red (bad)
        #            STFT positive=lean=add fuel → display orange/yellow
        # RdYlGn_r: vmin=-25 = dark green (confusing). Let's use seismic or bwr:
        # Actually use a custom diverging: center=0=white, negative=red (rich), positive=blue (lean)
        # "RdBu": negative=blue, positive=red  →  "RdBu_r": negative=red✓ positive=blue
        cmap = plt.get_cmap("RdBu_r")
        vmax = 25.0
        norm = plt.Normalize(vmin=-vmax, vmax=vmax)

        for ax, mat, cnt, title in [(axes[0], m1, c1, "STFT B1 (CL rows)"),
                                    (axes[1], m2, c2, "STFT B2 (CL rows)")]:
            im = ax.imshow(mat, cmap=cmap, norm=norm, aspect="auto", origin="upper",
                           interpolation="nearest")

            # Cell text annotation
            for i in range(n_rb):
                for j in range(n_lb):
                    v = mat[i, j]
                    n_s = cnt[i, j]
                    if np.isnan(v):
                        continue
                    sign = "+" if v > 0 else ""
                    # text color: white on dark cells, dark on light cells
                    txt_col = "white" if abs(v) > 10 else "#1a1a1a"
                    weight  = "bold" if abs(v) > THR["stft_warn"] else "normal"
                    ax.text(j, i, f"{sign}{v:.1f}\n({n_s})",
                            ha="center", va="center", fontsize=5.5,
                            color=txt_col, fontweight=weight)

            ax.set_xticks(range(n_lb))
            ax.set_xticklabels(LOAD_LABELS, rotation=40, ha="right", fontsize=6.5)
            ax.set_yticks(range(n_rb))
            ax.set_yticklabels(RPM_LABELS, fontsize=6.5)
            ax.set_xlabel("Load %", fontsize=8)
            ax.set_ylabel("RPM", fontsize=8)
            ax.set_title(title, fontsize=9)
            ax.tick_params(length=0)

            cb = fig.colorbar(im, ax=ax, fraction=0.04, pad=0.02,
                              label="STFT %  (red=rich, blue=lean)")
            cb.ax.tick_params(labelsize=7, colors=FG_DIM)
            cb.set_label("STFT % (red=rich, blue=lean)", color=FG_DIM, fontsize=7)

        fig.suptitle(
            f"{d['path'].name}  —  STFT 2D Map  "
            f"(B1 p50={percentile([r['stft_b1'] for r in d['cl_b1_rows']], 50) if d['cl_b1_rows'] else 0:+.1f}%  "
            f"B2 p50={percentile([r['stft_b2'] for r in d['cl_b2_rows']], 50) if d['cl_b2_rows'] else 0:+.1f}%)",
            fontsize=9, color=FG, y=1.0,
        )
        _embed(fig, self._tab_map)

    # ── Time Series tab ───────────────────────────────────────────────────────
    def _plot_timeseries(self, d):
        _clear_tab(self._tab_ts)

        rows = d["rows"]
        t0 = rows[0]["time_ms"] / 1000.0
        t  = np.array([r["time_ms"] / 1000.0 - t0 for r in rows])

        # Thin arrays for speed (keep every Nth point for heavy logs)
        step = max(1, len(rows) // 4000)
        t_s  = t[::step]
        rows_s = rows[::step]

        fig = Figure(figsize=(10, 8))
        gs  = fig.add_gridspec(4, 1, hspace=0.06, top=0.95, bottom=0.07, left=0.07, right=0.97)
        ax_rpm, ax_stft, ax_lmb, ax_ign = [fig.add_subplot(gs[i]) for i in range(4)]

        # Phase background shading (only on ax_rpm)
        phase_map = {r["time_ms"] / 1000.0 - t0: r["phase"] for r in rows}
        ph_times  = sorted(phase_map)
        cur_ph, cur_t = None, 0
        for tt in ph_times:
            ph = phase_map[tt]
            if ph != cur_ph:
                if cur_ph and cur_ph in PHASE_COLORS:
                    ax_rpm.axvspan(cur_t, tt, alpha=0.08,
                                   color=PHASE_COLORS[cur_ph], linewidth=0)
                cur_ph, cur_t = ph, tt

        # Panel 1: RPM + Load
        rpm_a = np.array([r["rpm"] for r in rows_s])
        load_a = np.array([r["load_pct"] for r in rows_s])
        ax_rpm.plot(t_s, rpm_a, color=ACCENT, lw=0.9, label="RPM")
        ax_r = ax_rpm.twinx()
        ax_r.plot(t_s, load_a, color=YELLOW, lw=0.7, alpha=0.6, label="Load %")
        ax_r.set_ylim(0, 110)
        ax_r.tick_params(colors=FG_DIM, labelsize=7)
        ax_r.spines["right"].set_color(BG3)
        ax_r.spines["top"].set_visible(False)
        ax_rpm.set_ylabel("RPM", color=ACCENT)
        ax_r.set_ylabel("Load %", color=YELLOW)
        lines1, labs1 = ax_rpm.get_legend_handles_labels()
        lines2, labs2 = ax_r.get_legend_handles_labels()
        ax_rpm.legend(lines1 + lines2, labs1 + labs2, loc="upper right", fontsize=7)
        ax_rpm.set_title(f"Time Series — {d['path'].name}  ({d['dur_s']:.0f}s)", fontsize=9)

        # Panel 2: STFT
        stft1 = np.array([r["stft_b1"] if r["cl_b1"] else np.nan for r in rows_s])
        stft2 = np.array([r["stft_b2"] if r["cl_b2"] else np.nan for r in rows_s])
        ax_stft.plot(t_s, stft1, color="#ff6b6b", lw=0.8, label="STFT B1", alpha=0.95)
        ax_stft.plot(t_s, stft2, color="#74b9ff", lw=0.8, label="STFT B2", alpha=0.75)
        ax_stft.axhline(0, color=FG_DIM, lw=0.6, ls="--")
        for tv, col in [(-THR["stft_alarm"], RED), (THR["stft_alarm"], RED),
                        (-THR["stft_warn"], ORANGE), (THR["stft_warn"], ORANGE)]:
            ax_stft.axhline(tv, color=col, lw=0.5, ls=":", alpha=0.55)
        # Overlay STFT bucket medians
        if d["stft_buckets"]:
            bt = [b["t"] - t0 for b in d["stft_buckets"]]
            bm1 = [b["p50_b1"] if b["p50_b1"] is not None else np.nan for b in d["stft_buckets"]]
            bm2 = [b["p50_b2"] if b["p50_b2"] is not None else np.nan for b in d["stft_buckets"]]
            ax_stft.plot(bt, bm1, "o-", color="#ff6b6b", lw=2.0, ms=5, alpha=0.9, label="B1 60s med")
            ax_stft.plot(bt, bm2, "s-", color="#74b9ff", lw=2.0, ms=5, alpha=0.75, label="B2 60s med")
        ax_stft.set_ylim(-35, 35)
        ax_stft.set_ylabel("STFT %")
        ax_stft.legend(loc="lower right", fontsize=6.5, ncol=2)

        # Panel 3: Lambda
        if d["has_lmb"]:
            lmb1 = np.array([r["lmb_b1"] if 0.5 < r["lmb_b1"] < 2.5 else np.nan for r in rows_s])
            lmb2 = np.array([r["lmb_b2"] if 0.5 < r["lmb_b2"] < 2.5 else np.nan for r in rows_s])
            ax_lmb.plot(t_s, lmb1, color="#a29bfe", lw=0.8, label="λ B1", alpha=0.9)
            ax_lmb.plot(t_s, lmb2, color="#fd79a8", lw=0.8, label="λ B2", alpha=0.75)
            ax_lmb.axhline(1.00,                   color=GREEN, lw=0.8, ls="--")
            ax_lmb.axhline(THR["lmb_lean_load"],   color=RED,   lw=0.5, ls=":", alpha=0.6)
            ax_lmb.axhline(THR["lmb_rich"],        color=ORANGE,lw=0.5, ls=":", alpha=0.6)
            ax_lmb.set_ylim(0.78, 1.18)
            ax_lmb.set_ylabel("λ")
            ax_lmb.legend(loc="upper right", fontsize=7)
        else:
            ax_lmb.set_visible(False)

        # Panel 4: Ignition + Coolant
        if d["has_ign"]:
            ign_a = np.array([r["ign_deg"] if abs(r["ign_deg"]) > 0.001 else np.nan for r in rows_s])
            cool_a = np.array([r["coolant_c"] for r in rows_s])
            ax_ign.plot(t_s, ign_a, color="#55efc4", lw=0.85, label="Ign °BTDC", alpha=0.95)
            ax_c = ax_ign.twinx()
            ax_c.plot(t_s, cool_a, color=ORANGE, lw=0.7, alpha=0.55, label="Coolant °C")
            ax_c.axhline(THR["coolant_warn"], color=RED, lw=0.5, ls=":", alpha=0.5)
            ax_c.tick_params(colors=FG_DIM, labelsize=7)
            ax_c.spines["right"].set_color(BG3)
            ax_c.spines["top"].set_visible(False)
            ax_ign.set_ylabel("Ign °", color="#55efc4")
            ax_c.set_ylabel("°C", color=ORANGE)
            lines1, labs1 = ax_ign.get_legend_handles_labels()
            lines2, labs2 = ax_c.get_legend_handles_labels()
            ax_ign.legend(lines1 + lines2, labs1 + labs2, loc="upper right", fontsize=7)
        else:
            ax_ign.set_visible(False)

        ax_ign.set_xlabel("Time (s)")

        # Annotate anomaly events on STFT panel
        ev_y = {"LEAN_B1": 28, "LEAN_B2": 26, "RICH_B1": -28, "RICH_B2": -26,
                "STFT_B1": 32, "STFT_B2": 30, "VANOS": 24, "OVERTEMP": 22}
        ev_col = {"LEAN_B1": RED, "LEAN_B2": ORANGE, "RICH_B1": PURPLE, "RICH_B2": "#b48ead",
                  "STFT_B1": RED, "STFT_B2": RED, "VANOS": YELLOW, "OVERTEMP": RED}
        for ev_type, ev_t, _ in d["events"][:40]:
            et = ev_t - t0
            if 0 <= et <= t[-1]:
                y = ev_y.get(ev_type, 0)
                c = ev_col.get(ev_type, FG_DIM)
                ax_stft.scatter([et], [y], s=12, c=c, marker="|", zorder=5, linewidths=1.2)

        for ax in [ax_rpm, ax_stft, ax_lmb, ax_ign]:
            ax.tick_params(labelsize=7, length=3)
            ax.grid(True, axis="y", alpha=0.3)
            if ax is not ax_ign:
                ax.tick_params(labelbottom=False)

        _embed(fig, self._tab_ts)

    # ── Trend tab (multi-log comparison) ─────────────────────────────────────
    def _run_compare(self):
        self._status_var.set("Scanning all logs…")
        self.configure(cursor="watch")
        threading.Thread(target=self._compare_thread, daemon=True).start()

    def _compare_thread(self):
        try:
            csvs = sorted(self._logs_dir.glob("*.csv"))
            results = []
            for p in csvs:
                try:
                    d = compute_data(p)
                    if d["run_rows"]:
                        results.append(d)
                except Exception:
                    pass
            self.after(0, self._plot_trend, results)
            self.after(0, self._status_var.set, f"Trend: {len(results)} logs loaded from LOGS/")
        except Exception as exc:
            self.after(0, self._status_var.set, f"Trend error: {exc}")
        finally:
            self.after(0, self.configure, {"cursor": ""})

    def _plot_trend(self, results):
        _clear_tab(self._tab_tr)
        if not results:
            ttk.Label(self._tab_tr, text="No logs found in LOGS/", foreground=FG_DIM).pack()
            return

        names  = [d["path"].stem[-12:] for d in results]
        xs     = range(len(results))

        def safe_med(rows, key):
            vals = [r[key] for r in rows]
            return percentile(vals, 50) if vals else 0

        b1_meds  = [safe_med(d["cl_b1_rows"], "stft_b1") if d["cl_b1_rows"] else np.nan for d in results]
        b2_meds  = [safe_med(d["cl_b2_rows"], "stft_b2") if d["cl_b2_rows"] else np.nan for d in results]
        wot_lmbs = []
        for d in results:
            if d["wot_r1"]:
                wot_lmbs.append(statistics.mean(r["lmb_b1"] for r in d["wot_r1"]))
            else:
                wot_lmbs.append(np.nan)

        dc_maxes = []
        for d in results:
            if d["has_inj"] and d["run_rows"]:
                dc = max(
                    (r["inj_ms"] / (120000.0 / r["rpm"]) * 100.0
                     for r in d["run_rows"] if r["inj_ms"] > 0.5 and r["rpm"] > 0),
                    default=0,
                )
                dc_maxes.append(dc)
            else:
                dc_maxes.append(np.nan)

        n_events = [len(d["events"]) for d in results]
        durations = [d["dur_s"] / 60 for d in results]

        fig = Figure(figsize=(10, 7), tight_layout={"pad": 1.5})
        gs  = fig.add_gridspec(3, 2, hspace=0.45, wspace=0.35,
                               top=0.93, bottom=0.12, left=0.07, right=0.97)
        ax_stft = fig.add_subplot(gs[0, :])   # full width
        ax_wot  = fig.add_subplot(gs[1, 0])
        ax_dc   = fig.add_subplot(gs[1, 1])
        ax_ev   = fig.add_subplot(gs[2, 0])
        ax_dur  = fig.add_subplot(gs[2, 1])

        # STFT trend
        ax_stft.plot(xs, b1_meds, "o-", color="#ff6b6b", lw=1.8, ms=6, label="B1 STFT p50")
        ax_stft.plot(xs, b2_meds, "s-", color="#74b9ff", lw=1.8, ms=6, label="B2 STFT p50")
        ax_stft.axhline(0, color=FG_DIM, lw=0.7, ls="--")
        for tv, col in [(-THR["stft_warn"], RED), (THR["stft_warn"], RED),
                        (-THR["stft_note"], YELLOW), (THR["stft_note"], YELLOW)]:
            ax_stft.axhline(tv, color=col, lw=0.5, ls=":", alpha=0.5)
        ax_stft.fill_between(xs, [-THR["stft_note"]]*len(xs), [THR["stft_note"]]*len(xs),
                             color=GREEN, alpha=0.08)
        ax_stft.set_ylabel("STFT p50 %")
        ax_stft.set_title("STFT Trend Across Sessions — target: ±8% band", fontsize=9)
        ax_stft.legend(fontsize=7, loc="upper right")
        ax_stft.set_xticks(list(xs))
        ax_stft.set_xticklabels(names, rotation=25, ha="right", fontsize=7)

        # WOT lambda trend
        wot_x = [i for i, v in enumerate(wot_lmbs) if not np.isnan(v)]
        wot_y = [v for v in wot_lmbs if not np.isnan(v)]
        if wot_x:
            ax_wot.plot(wot_x, wot_y, "D-", color=YELLOW, lw=1.5, ms=5)
            ax_wot.fill_between(wot_x, [0.92]*len(wot_x), wot_y,
                                where=[v > 0.92 for v in wot_y],
                                color=RED, alpha=0.2, label="lean zone")
            ax_wot.axhline(0.92, color=GREEN, lw=0.8, ls="--", label="E85 target")
            ax_wot.axhline(0.95, color=RED,   lw=0.5, ls=":", alpha=0.7)
            ax_wot.legend(fontsize=7)
        ax_wot.set_ylabel("WOT λ B1")
        ax_wot.set_title("WOT Lambda Trend")
        ax_wot.set_xticks(wot_x)
        ax_wot.set_xticklabels([names[i] for i in wot_x], rotation=20, ha="right", fontsize=6.5)

        # DC max
        dc_x = [i for i, v in enumerate(dc_maxes) if not np.isnan(v)]
        dc_y = [v for v in dc_maxes if not np.isnan(v)]
        if dc_x:
            dc_cols = [RED if v > 80 else (ORANGE if v > 60 else GREEN) for v in dc_y]
            ax_dc.bar(dc_x, dc_y, color=dc_cols, edgecolor="none", alpha=0.85)
            ax_dc.axhline(80, color=RED, lw=0.8, ls="--", alpha=0.7)
        ax_dc.set_ylabel("Max DC %")
        ax_dc.set_title("Peak Injector Duty Cycle")
        ax_dc.set_xticks(dc_x)
        ax_dc.set_xticklabels([names[i] for i in dc_x], rotation=20, ha="right", fontsize=6.5)

        # Event counts
        ev_cols = [RED if n > 10 else (ORANGE if n > 4 else GREEN) for n in n_events]
        ax_ev.bar(xs, n_events, color=ev_cols, edgecolor="none", alpha=0.85)
        ax_ev.set_ylabel("Anomaly events")
        ax_ev.set_title("Anomaly Event Count")
        ax_ev.set_xticks(list(xs))
        ax_ev.set_xticklabels(names, rotation=20, ha="right", fontsize=6.5)

        # Session durations
        ax_dur.bar(xs, durations, color=ACCENT, edgecolor="none", alpha=0.75)
        ax_dur.set_ylabel("Duration (min)")
        ax_dur.set_title("Log Duration")
        ax_dur.set_xticks(list(xs))
        ax_dur.set_xticklabels(names, rotation=20, ha="right", fontsize=6.5)

        for ax in [ax_stft, ax_wot, ax_dc, ax_ev, ax_dur]:
            ax.tick_params(labelsize=7, length=3)

        fig.suptitle(f"Session Trend  ·  {len(results)} logs  ·  LOGS/", fontsize=10, color=FG)
        _embed(fig, self._tab_tr)
        self._nb.select(self._tab_tr)


# ─── Entry point ──────────────────────────────────────────────────────────────
def main():
    initial = None
    if len(sys.argv) > 1:
        p = Path(sys.argv[1])
        initial = p if p.is_absolute() else Path.cwd() / p

    app = MSV70App(initial_path=initial)
    app.mainloop()


if __name__ == "__main__":
    main()
