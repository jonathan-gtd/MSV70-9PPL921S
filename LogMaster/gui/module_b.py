"""
Module B — Expert Analysis & Verdict.

Panels:
  • Left  — scrollable diagnostic report (text) covering fuel trims,
            bank balance, MAF/Load correlation, state distribution,
            and temperature adequacy.
  • Right — RPM × Load heatmap (pivot table) for any trim channel,
            rendered with a diverging colormap centred at 0.
"""
import numpy as np
import pandas as pd
import matplotlib
import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
import customtkinter as ctk

matplotlib.use("TkAgg")

DARK_FIG = "#1a1a2a"
DARK_AX  = "#111122"


class ExpertAnalysisModule(ctk.CTkFrame):
    def __init__(self, parent, config_manager, data_engine):
        super().__init__(parent, fg_color="transparent")
        self.cfg    = config_manager
        self.engine = data_engine
        self._build_ui()

    # ------------------------------------------------------------------ #
    # Layout                                                               #
    # ------------------------------------------------------------------ #

    def _build_ui(self) -> None:
        ctk.CTkLabel(
            self, text="Expert Analysis & Verdict",
            font=ctk.CTkFont(size=20, weight="bold"),
        ).pack(anchor="w", padx=20, pady=(16, 6))

        # Toolbar
        bar = ctk.CTkFrame(self, fg_color="transparent")
        bar.pack(fill="x", padx=20, pady=(0, 10))

        ctk.CTkButton(
            bar, text="▶  Run Analysis", command=self.run_analysis,
            fg_color="#0f4c81", hover_color="#1565c0", width=150, height=34,
        ).pack(side="left", padx=(0, 12))

        ctk.CTkLabel(bar, text="Heatmap channel:").pack(side="left", padx=(0, 6))
        self._heat_var = ctk.StringVar(value="stft_b1")
        ctk.CTkOptionMenu(
            bar, variable=self._heat_var, width=180,
            values=["stft_b1", "stft_b2",
                    "ltft_add_b1", "ltft_add_b2",
                    "ltft_mul_b1", "ltft_mul_b2",
                    "ltft_mul_hi_b1", "ltft_mul_hi_b2"],
            command=lambda _: self._draw_heatmap(),
        ).pack(side="left")

        # Body: report | heatmap
        body = ctk.CTkFrame(self, fg_color="transparent")
        body.pack(fill="both", expand=True, padx=20, pady=(0, 16))
        body.columnconfigure(0, weight=1)
        body.columnconfigure(1, weight=2)
        body.rowconfigure(0, weight=1)

        # ── Left: report ────────────────────────────────────────────────
        left = ctk.CTkFrame(body, fg_color="#0d1117")
        left.grid(row=0, column=0, sticky="nsew", padx=(0, 8))

        ctk.CTkLabel(left, text="Diagnostic Report",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=12, pady=(10, 4))

        self._report = ctk.CTkTextbox(
            left, font=ctk.CTkFont(family="Courier New", size=11),
            fg_color="#0d1117", text_color="#d0d0d0",
            wrap="none",
        )
        self._report.pack(fill="both", expand=True, padx=8, pady=(0, 8))
        self._report.configure(state="disabled")
        self._write_report("Run an analysis to see results.")

        # ── Right: heatmap ───────────────────────────────────────────────
        right = ctk.CTkFrame(body, fg_color="#111122")
        right.grid(row=0, column=1, sticky="nsew", padx=(8, 0))

        ctk.CTkLabel(right, text="RPM × Load Heatmap (warm engine)",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=12, pady=(10, 4))

        self._fig_h = plt.figure(figsize=(7, 5))
        self._fig_h.patch.set_facecolor(DARK_FIG)
        self._ax_h  = self._fig_h.add_subplot(111)
        self._ax_h.set_facecolor(DARK_AX)

        self._canvas_h = FigureCanvasTkAgg(self._fig_h, master=right)
        self._canvas_h.get_tk_widget().pack(fill="both", expand=True,
                                            padx=8, pady=(0, 8))

    # ------------------------------------------------------------------ #
    # Analysis                                                             #
    # ------------------------------------------------------------------ #

    def run_analysis(self) -> None:
        if self.engine.df is None:
            self._write_report("No CSV loaded. Use 'Open CSV Log' first.")
            return

        lines = []
        w   = self.cfg.get_threshold
        sep = "─" * 48

        # ── Header ──────────────────────────────────────────────────────
        lines += [
            "╔" + "═" * 46 + "╗",
            "║   LOGMASTER SUITE PRO — DIAGNOSTIC REPORT   ║",
            "╚" + "═" * 46 + "╝",
            "",
        ]

        # ── Engine state distribution ────────────────────────────────────
        summary = self.engine.get_state_summary()
        if summary:
            lines.append("[ ENGINE STATE DISTRIBUTION ]")
            for state in ("WOT", "Cruise", "Transient", "Idle"):
                pct = summary.get(state, 0.0)
                bar = "█" * int(pct / 4) + "░" * (25 - int(pct / 4))
                lines.append(f"  {state:<12} {bar}  {pct:5.1f}%")
            lines += ["", sep, ""]

        # ── Temperature ─────────────────────────────────────────────────
        coolant  = self.engine.get_col("coolant")
        min_temp = w("min_analysis_temp_coolant", 70.0)
        lines.append("[ TEMPERATURE ]")
        if coolant is not None:
            cold_pct = (coolant < min_temp).mean() * 100
            icon = "⚠" if cold_pct > 20 else "✓"
            lines.append(f"  {icon}  Cold samples (< {min_temp}°C): {cold_pct:.1f}%")
            if cold_pct > 50:
                lines.append("     Engine was cold for most of the log.")
                lines.append("     Trim results below may not be representative.")
        else:
            lines.append("  —  Coolant column not found in this log.")
        lines += ["", sep, ""]

        # ── Fuel trims ──────────────────────────────────────────────────
        stats    = self.engine.analyze_fuel_trims()
        warn_pct = w("ltft_warning_pct", 5.0)
        crit_pct = w("ltft_critical_pct", 10.0)
        stft_w   = w("stft_warning_pct", 5.0)

        lines.append("[ FUEL TRIM ANALYSIS ]")
        for bank in (1, 2):
            stft_m = stats.get(f"stft_b{bank}_mean")
            stft_x = stats.get(f"stft_b{bank}_max")
            ltft_a = stats.get(f"ltft_add_b{bank}_mean")
            ltft_m = stats.get(f"ltft_mul_b{bank}_mean")

            if stft_m is None and ltft_m is None:
                lines.append(f"  Bank {bank}: no trim data available.")
                continue

            lines.append(f"  ┌─ Bank {bank} {'─'*38}")
            if stft_m is not None:
                flag = "  ✓" if abs(stft_m) < stft_w else ("  ⚠" if abs(stft_m) < crit_pct else "  ✗ CRITICAL")
                lines.append(f"  │  STFT  mean={stft_m:+.2f}%  max={stft_x:.2f}%{flag}")
            if ltft_a is not None:
                flag = "  ✓" if abs(ltft_a) < warn_pct else "  ⚠"
                lines.append(f"  │  LTFT add  mean={ltft_a:+.2f}%{flag}")
            if ltft_m is not None:
                flag = "  ✓" if abs(ltft_m) < warn_pct else ("  ⚠" if abs(ltft_m) < crit_pct else "  ✗ CRITICAL")
                lines.append(f"  │  LTFT mul  mean={ltft_m:+.2f}%{flag}")

            # Global verdict for this bank
            total = (abs(stft_m or 0) + abs(ltft_m or 0))
            if total < 3.5:
                verdict = "✓ HEALTHY — Mixture well calibrated"
            elif total < 7.0:
                verdict = "~ MARGINAL — Minor drift, monitor closely"
            else:
                verdict = "✗ ACTION — Significant mixture error, remap needed"
            lines.append(f"  └─ Verdict: {verdict}")
            lines.append("")
        lines += [sep, ""]

        # ── Bank balance ─────────────────────────────────────────────────
        alert   = w("bank_balance_alert_pct", 5.0)
        b_mean  = stats.get("bank_delta_mean")
        b_max   = stats.get("bank_delta_max")
        lines.append("[ BANK BALANCE (B1 vs B2) ]")
        if b_mean is not None:
            flag = "✗ ALERT" if b_mean > alert else "✓ OK"
            lines.append(f"  Mean Δ trim : {b_mean:.2f}%  {flag}")
            lines.append(f"  Max  Δ trim : {b_max:.2f}%")
            if b_mean > alert:
                lines.append("  → Check injectors, upstream O2 sensors,")
                lines.append("    or intake air asymmetry (DISA fault).")
        else:
            lines.append("  —  Insufficient data (need both banks).")
        lines += ["", sep, ""]

        # ── MAF / Load ──────────────────────────────────────────────────
        maf_r = self.engine.analyze_maf_load()
        lines.append("[ AIRFLOW CHECK — MAF vs Calculated Load ]")
        if maf_r:
            corr = maf_r["maf_load_correlation"]
            if corr > 0.88:
                v = "✓ Excellent — No air leak detected"
            elif corr > 0.70:
                v = "~ Marginal — Possible minor leak or DISA issue"
            else:
                v = "✗ Poor — Check MAF sensor, DISA flaps, intake seals"
            lines.append(f"  Pearson r = {corr:.4f}")
            lines.append(f"  Verdict   : {v}")
        else:
            lines.append("  —  MAF or Load column missing from this log.")
        lines += ["", sep, ""]

        # ── Injector duty (derived) ──────────────────────────────────────
        inj  = self.engine.get_col("inj_time")
        rpm  = self.engine.get_col("rpm")
        if inj is not None and rpm is not None:
            warm = self.engine.get_warm_mask()
            duty = self.engine.compute_injector_duty(inj[warm], rpm[warm])
            lines.append("[ INJECTOR DUTY CYCLE (Cyl 1, warm) ]")
            lines.append(f"  Mean : {duty.mean():.1f}%")
            lines.append(f"  Max  : {duty.max():.1f}%")
            if duty.max() > 85:
                lines.append("  ⚠  Injectors near saturation at peak load.")
                lines.append("     Consider higher-flow injectors or fuel pressure.")
            lines += ["", sep]

        lines.append("\n  Analysis complete.")
        self._write_report("\n".join(lines))
        self._draw_heatmap()

    # ------------------------------------------------------------------ #
    # Heatmap                                                              #
    # ------------------------------------------------------------------ #

    def _draw_heatmap(self) -> None:
        self._fig_h.clear()
        ax = self._fig_h.add_subplot(111)
        ax.set_facecolor(DARK_AX)
        self._fig_h.patch.set_facecolor(DARK_FIG)

        alias = self._heat_var.get()
        pivot = self.engine.build_rpm_load_heatmap(alias)

        if pivot is None or pivot.empty:
            ax.text(0.5, 0.5, "No data for this channel.",
                    ha="center", va="center", color="#888",
                    transform=ax.transAxes, fontsize=13)
            self._canvas_h.draw()
            return

        data = pivot.values.astype(float)
        data_valid = data[~np.isnan(data)]
        vmax = max(np.abs(data_valid).max() if len(data_valid) else 1.0, 0.5)

        im = ax.imshow(
            data, aspect="auto", origin="lower",
            cmap="RdYlGn_r", vmin=-vmax, vmax=vmax,
            interpolation="nearest",
        )

        # Axis tick labels (interval midpoints)
        def _midlabels(intervals) -> list:
            out = []
            for iv in intervals:
                try:
                    mid = (iv.left + iv.right) / 2
                    out.append(f"{mid:.0f}")
                except Exception:
                    out.append(str(iv)[:6])
            return out

        ax.set_xticks(range(len(pivot.columns)))
        ax.set_xticklabels(_midlabels(pivot.columns), rotation=45,
                           ha="right", fontsize=7, color="#ccc")
        ax.set_yticks(range(len(pivot.index)))
        ax.set_yticklabels(_midlabels(pivot.index), fontsize=7, color="#ccc")

        ax.set_xlabel("Load %", color="#ccc", fontsize=9)
        ax.set_ylabel("RPM",    color="#ccc", fontsize=9)
        ax.set_title(f"{alias}  (mean, warm engine)", color="#ccc", fontsize=10)
        for sp in ax.spines.values():
            sp.set_color("#444")

        cbar = self._fig_h.colorbar(im, ax=ax, fraction=0.046, pad=0.04)
        cbar.ax.tick_params(labelcolor="#ccc", color="#ccc")
        cbar.ax.yaxis.label.set_color("#ccc")

        self._fig_h.tight_layout()
        self._canvas_h.draw()

    # ------------------------------------------------------------------ #
    # Helpers                                                              #
    # ------------------------------------------------------------------ #

    def _write_report(self, text: str) -> None:
        self._report.configure(state="normal")
        self._report.delete("1.0", "end")
        self._report.insert("1.0", text)
        self._report.configure(state="disabled")

    def refresh(self) -> None:
        self._write_report("New file loaded. Press '▶ Run Analysis'.")
        # Reset heatmap
        self._fig_h.clear()
        ax = self._fig_h.add_subplot(111)
        ax.set_facecolor(DARK_AX)
        self._fig_h.patch.set_facecolor(DARK_FIG)
        ax.text(0.5, 0.5, "Run Analysis to populate heatmap.",
                ha="center", va="center", color="#888",
                transform=ax.transAxes, fontsize=11)
        self._canvas_h.draw()
