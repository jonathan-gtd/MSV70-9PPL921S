"""
Module A — Interactive Graph Viewer.

Features:
  • Matplotlib canvas with NavigationToolbar (zoom / pan / save).
  • Engine-phase background shading: WOT=red, Cruise=green, Idle=gray,
    Transient=yellow (via axvspan runs).
  • Clickable legend to toggle individual channel visibility.
  • Floating tooltip showing all selected channel values at the hover position.
  • Channel selector sidebar with per-channel color swatches.
"""
import numpy as np
import pandas as pd
import matplotlib
import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg, NavigationToolbar2Tk
import customtkinter as ctk

matplotlib.use("TkAgg")

# State → (hex fill color, alpha)
PHASE_STYLE: dict = {
    "WOT":       ("#c0392b", 0.22),
    "Cruise":    ("#27ae60", 0.18),
    "Idle":      ("#7f8c8d", 0.18),
    "Transient": ("#f39c12", 0.22),
}

LINE_COLORS = [
    "#4fc3f7", "#81c784", "#ffb74d", "#f48fb1", "#ce93d8",
    "#80cbc4", "#fff176", "#ef9a9a", "#a5d6a7", "#90caf9",
    "#ffcc80", "#b0bec5", "#ff8a65", "#a1887f", "#e6ee9c",
]

DARK_BG  = "#141414"
DARK_FIG = "#1c1c1c"
AXIS_CLR = "#cccccc"
GRID_CLR = "#383838"


class GraphViewerModule(ctk.CTkFrame):
    def __init__(self, parent, config_manager, data_engine):
        super().__init__(parent, fg_color="transparent")
        self.cfg    = config_manager
        self.engine = data_engine

        self._col_vars: dict  = {}      # col_name -> BooleanVar
        self._col_order: list = []      # ordered column list (mirrors sidebar)
        self._legend_map: dict = {}     # legend_line -> axes_line

        self._build_ui()

    # ------------------------------------------------------------------ #
    # Layout                                                               #
    # ------------------------------------------------------------------ #

    def _build_ui(self) -> None:
        # ── Header ──────────────────────────────────────────────────────
        hdr = ctk.CTkFrame(self, fg_color="transparent")
        hdr.pack(fill="x", padx=20, pady=(16, 6))

        ctk.CTkLabel(
            hdr, text="Interactive Graph Viewer",
            font=ctk.CTkFont(size=20, weight="bold"),
        ).pack(side="left")

        # Phase legend chips (right-aligned)
        for state, (color, _) in reversed(PHASE_STYLE.items()):
            ctk.CTkLabel(hdr, text=state, font=ctk.CTkFont(size=11),
                         text_color=color).pack(side="right", padx=4)
        ctk.CTkLabel(hdr, text="Phases →",
                     font=ctk.CTkFont(size=11), text_color="#666").pack(side="right", padx=(12, 4))

        # ── Toolbar ─────────────────────────────────────────────────────
        bar = ctk.CTkFrame(self, fg_color="transparent")
        bar.pack(fill="x", padx=20, pady=(0, 8))

        ctk.CTkButton(bar, text="▶  Plot Selected", command=self.plot,
                      fg_color="#0f4c81", hover_color="#1565c0",
                      width=140, height=34).pack(side="left", padx=(0, 8))
        ctk.CTkButton(bar, text="✕  Clear", command=self._clear_plot,
                      fg_color="#4a0000", hover_color="#7f0000",
                      width=90, height=34).pack(side="left", padx=(0, 8))

        self._shading_var = ctk.BooleanVar(value=True)
        ctk.CTkCheckBox(bar, text="Phase shading",
                        variable=self._shading_var,
                        font=ctk.CTkFont(size=12)).pack(side="left", padx=8)

        # ── Main split: sidebar + canvas ────────────────────────────────
        body = ctk.CTkFrame(self, fg_color="transparent")
        body.pack(fill="both", expand=True, padx=20, pady=(0, 16))
        body.columnconfigure(1, weight=1)
        body.rowconfigure(0, weight=1)

        # Sidebar
        sidebar = ctk.CTkFrame(body, width=210, fg_color="#111827")
        sidebar.grid(row=0, column=0, sticky="nsew", padx=(0, 8))
        sidebar.grid_propagate(False)

        ctk.CTkLabel(sidebar, text="Channels",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=10, pady=(10, 4))

        self._all_var = ctk.BooleanVar(value=False)
        ctk.CTkCheckBox(sidebar, text="Select all", variable=self._all_var,
                        command=self._toggle_all,
                        font=ctk.CTkFont(size=11)).pack(anchor="w", padx=10, pady=(0, 4))

        self._ch_scroll = ctk.CTkScrollableFrame(sidebar, fg_color="transparent")
        self._ch_scroll.pack(fill="both", expand=True, padx=4, pady=(0, 8))

        # Canvas area
        canvas_frame = ctk.CTkFrame(body, fg_color="#1c1c1c", corner_radius=8)
        canvas_frame.grid(row=0, column=1, sticky="nsew")

        self._fig, self._ax = plt.subplots(figsize=(10, 6))
        self._apply_dark_theme()
        self._ax.set_title("Load a CSV log to begin", color=AXIS_CLR, pad=10)

        self._canvas = FigureCanvasTkAgg(self._fig, master=canvas_frame)
        self._canvas.get_tk_widget().pack(fill="both", expand=True)

        # Navigation toolbar
        nav_bg = ctk.CTkFrame(canvas_frame, fg_color="#181818", height=32)
        nav_bg.pack(fill="x")
        self._nav = NavigationToolbar2Tk(self._canvas, nav_bg)
        self._nav.config(background="#181818")
        for child in self._nav.winfo_children():
            try:
                child.config(background="#181818", foreground=AXIS_CLR)
            except Exception:
                pass
        self._nav.update()

        # Tooltip overlay
        self._tip = ctk.CTkLabel(
            canvas_frame, text="", fg_color="#0d1117", corner_radius=6,
            font=ctk.CTkFont(family="Courier New", size=10),
            justify="left", padx=8, pady=6,
        )
        self._canvas.mpl_connect("motion_notify_event", self._on_hover)
        self._canvas.mpl_connect("axes_leave_event",    self._on_leave)

    # ------------------------------------------------------------------ #
    # Channel management                                                   #
    # ------------------------------------------------------------------ #

    def _populate_channels(self) -> None:
        for w in self._ch_scroll.winfo_children():
            w.destroy()
        self._col_vars.clear()
        self._col_order.clear()

        if self.engine.df is None:
            return

        skip = {"_time_s"}
        cols = [c for c in self.engine.df.columns if c not in skip]
        self._col_order = cols

        for i, col in enumerate(cols):
            color = LINE_COLORS[i % len(LINE_COLORS)]
            var = ctk.BooleanVar(value=False)
            self._col_vars[col] = var

            row = ctk.CTkFrame(self._ch_scroll, fg_color="transparent")
            row.pack(fill="x", pady=1)

            # Color swatch
            ctk.CTkFrame(row, fg_color=color, width=8, height=14,
                         corner_radius=2).pack(side="left", padx=(2, 4))
            ctk.CTkCheckBox(
                row, text=col[:24], variable=var,
                font=ctk.CTkFont(size=11),
                checkbox_width=15, checkbox_height=15,
            ).pack(side="left")

    def _toggle_all(self) -> None:
        v = self._all_var.get()
        for var in self._col_vars.values():
            var.set(v)

    def _selected_cols(self) -> list:
        return [c for c in self._col_order if self._col_vars.get(c, ctk.BooleanVar()).get()]

    # ------------------------------------------------------------------ #
    # Plotting                                                             #
    # ------------------------------------------------------------------ #

    def plot(self) -> None:
        if self.engine.df is None:
            return
        cols = self._selected_cols()
        if not cols:
            return

        self._ax.clear()
        self._apply_dark_theme()
        self._legend_map.clear()

        t = self.engine.get_time_s()
        if t is None:
            t = pd.Series(range(len(self.engine.df)), index=self.engine.df.index)

        # Phase shading
        if self._shading_var.get():
            self._draw_phases(t)

        # Signal lines
        lines = []
        for i, col in enumerate(cols):
            color = LINE_COLORS[self._col_order.index(col) % len(LINE_COLORS)]
            series = self.engine.df[col]
            mask   = series.notna()
            line,  = self._ax.plot(
                t[mask], series[mask],
                color=color, linewidth=1.3, label=col, alpha=0.92,
            )
            lines.append(line)

        self._ax.set_xlabel("Time (s)", color=AXIS_CLR, fontsize=10)
        self._ax.set_ylabel("Value",    color=AXIS_CLR, fontsize=10)
        self._ax.set_title(
            f"Log — {len(cols)} channel(s) | {len(self.engine.df)} samples",
            color=AXIS_CLR, fontsize=11,
        )

        # Clickable legend
        leg = self._ax.legend(
            loc="upper right", fontsize=8.5,
            facecolor="#1e293b", edgecolor="#334155",
            labelcolor=AXIS_CLR,
        )
        for leg_line, orig_line in zip(leg.get_lines(), lines):
            leg_line.set_picker(6)
            self._legend_map[leg_line] = orig_line

        self._canvas.mpl_connect("pick_event", self._on_pick)
        self._fig.tight_layout()
        self._canvas.draw()

    def _draw_phases(self, time_s: pd.Series) -> None:
        if self.engine.states is None:
            return
        states = self.engine.states
        if len(states) != len(time_s):
            return

        t  = time_s.values
        s  = states.values.astype(str)

        # Find run boundaries
        changes = np.where(np.diff(s) != "")[0] + 1
        starts  = np.concatenate([[0], changes])
        ends    = np.concatenate([changes, [len(s)]])

        for start, end in zip(starts, ends):
            state = s[start]
            color, alpha = PHASE_STYLE.get(state, ("#888", 0.1))
            x0 = t[start]
            x1 = t[min(end, len(t) - 1)]
            self._ax.axvspan(x0, x1, facecolor=color, alpha=alpha, zorder=0)

    # ------------------------------------------------------------------ #
    # Interactivity                                                        #
    # ------------------------------------------------------------------ #

    def _on_pick(self, event) -> None:
        """Toggle a channel's visibility when its legend entry is clicked."""
        leg_line = event.artist
        orig = self._legend_map.get(leg_line)
        if orig is None:
            return
        vis = not orig.get_visible()
        orig.set_visible(vis)
        leg_line.set_alpha(1.0 if vis else 0.25)
        self._canvas.draw_idle()

    def _on_hover(self, event) -> None:
        if event.inaxes != self._ax or self.engine.df is None:
            self._tip.place_forget()
            return

        x_val = event.xdata
        if x_val is None:
            return

        t = self.engine.get_time_s()
        if t is None:
            return

        idx = (t - x_val).abs().idxmin()
        cols = self._selected_cols()[:12]           # cap tooltip rows
        if not cols:
            return

        state = (self.engine.states.loc[idx]
                 if self.engine.states is not None else "—")
        lines = [f"t = {t.loc[idx]:.2f} s   [{state}]", "─" * 26]
        for col in cols:
            try:
                v = self.engine.df.loc[idx, col]
                lines.append(f"{col[:18]:<18} {v:>8.3f}")
            except Exception:
                pass

        self._tip.configure(text="\n".join(lines))
        # Place tooltip near the cursor but avoid going off-screen
        wx = event.x + 18
        wy = max(event.y - 100, 4)
        self._tip.place(x=wx, y=wy)

    def _on_leave(self, event) -> None:
        self._tip.place_forget()

    # ------------------------------------------------------------------ #
    # Helpers                                                              #
    # ------------------------------------------------------------------ #

    def _apply_dark_theme(self) -> None:
        self._fig.patch.set_facecolor(DARK_FIG)
        self._ax.set_facecolor(DARK_BG)
        self._ax.tick_params(colors=AXIS_CLR)
        for spine in self._ax.spines.values():
            spine.set_color("#444444")
        self._ax.grid(True, alpha=0.18, color=GRID_CLR, linestyle="--")

    def _clear_plot(self) -> None:
        self._ax.clear()
        self._apply_dark_theme()
        self._ax.set_title("Plot cleared", color=AXIS_CLR)
        self._canvas.draw()

    def refresh(self) -> None:
        """Called by MainGUI after a new CSV is loaded."""
        self._populate_channels()
        self._clear_plot()
