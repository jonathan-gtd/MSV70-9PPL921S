"""
LogMaster Suite Pro — Main entry point.

Architecture:
  ConfigManager  → loads/saves config.json and parses jobs definitions.
  DataEngine     → cleans, maps, and analyses Testo CSV logs.
  TestoFactory   → generates customjobs.xml for the Testo datalogger.
  MainGUI        → CustomTkinter window with a sidebar and four modules.
"""
import os
import sys

import customtkinter as ctk
from tkinter import filedialog, messagebox

from config_manager import ConfigManager
from data_engine import DataEngine
from testo_factory import TestoFactory
from gui.module_a import GraphViewerModule
from gui.module_b import ExpertAnalysisModule
from gui.module_c import JobCreatorModule
from gui.module_d import ConfigurationModule

ctk.set_appearance_mode("dark")
ctk.set_default_color_theme("dark-blue")

# ─────────────────────────────────────────────────────────────────────────────
# Constants
# ─────────────────────────────────────────────────────────────────────────────

MODULES: dict = {
    "A": ("◈", "Graph Viewer",    GraphViewerModule),
    "B": ("◉", "Expert Analysis", ExpertAnalysisModule),
    "C": ("◎", "Job Creator",     JobCreatorModule),
    "D": ("◇", "Configuration",   ConfigurationModule),
}

SIDEBAR_W   = 224
SIDEBAR_BG  = "#0f172a"
NAV_ACTIVE  = "#1e3a5f"
NAV_TEXT    = "#94a3b8"
NAV_ACCENT  = "#38bdf8"
WIN_BG      = "#0a1120"


# ─────────────────────────────────────────────────────────────────────────────
# MainGUI
# ─────────────────────────────────────────────────────────────────────────────

class MainGUI(ctk.CTk):
    def __init__(self):
        super().__init__()
        self.title("LogMaster Suite Pro — MSV70 Edition")
        self.geometry("1480x880")
        self.minsize(1100, 680)
        self.configure(fg_color=WIN_BG)

        # Services
        cfg_path    = os.path.join(os.path.dirname(__file__), "config.json")
        self.cfg    = ConfigManager(cfg_path)
        self.engine = DataEngine(self.cfg)
        self.factory = TestoFactory(self.cfg)

        # State
        self._active_key: str   = ""
        self._nav_btns: dict    = {}
        self._modules: dict     = {}

        self._build_layout()
        self._build_sidebar()
        self._build_statusbar()
        self.switch_module("A")

    # ------------------------------------------------------------------ #
    # Window layout                                                        #
    # ------------------------------------------------------------------ #

    def _build_layout(self) -> None:
        self.sidebar = ctk.CTkFrame(
            self, width=SIDEBAR_W, corner_radius=0, fg_color=SIDEBAR_BG,
        )
        self.sidebar.pack(side="left", fill="y")
        self.sidebar.pack_propagate(False)

        self.content = ctk.CTkFrame(self, corner_radius=0, fg_color="#0b1628")
        self.content.pack(side="right", fill="both", expand=True)

    def _build_sidebar(self) -> None:
        # ── Brand block ─────────────────────────────────────────────────
        brand = ctk.CTkFrame(self.sidebar, fg_color="#070d1a", corner_radius=0)
        brand.pack(fill="x")

        ctk.CTkLabel(
            brand, text="LogMaster",
            font=ctk.CTkFont(size=24, weight="bold"),
            text_color=NAV_ACCENT,
        ).pack(pady=(18, 0))
        ctk.CTkLabel(
            brand, text="Suite Pro",
            font=ctk.CTkFont(size=13),
            text_color="#64748b",
        ).pack()
        ctk.CTkLabel(
            brand, text="MSV70 · Siemens",
            font=ctk.CTkFont(size=10),
            text_color="#1e3a5f",
        ).pack(pady=(0, 16))

        # ── Open CSV button ─────────────────────────────────────────────
        ctk.CTkButton(
            self.sidebar, text="📂  Open CSV Log",
            command=self._open_csv,
            fg_color="#0369a1", hover_color="#0284c7",
            height=40, font=ctk.CTkFont(size=13, weight="bold"),
        ).pack(fill="x", padx=14, pady=(4, 8))

        self._file_lbl = ctk.CTkLabel(
            self.sidebar, text="No file loaded",
            font=ctk.CTkFont(size=10), text_color="#334155",
            wraplength=198,
        )
        self._file_lbl.pack(padx=10, pady=(0, 10))

        # ── Divider ─────────────────────────────────────────────────────
        ctk.CTkFrame(self.sidebar, height=1, fg_color="#1e293b").pack(
            fill="x", padx=14, pady=6,
        )

        # ── Navigation buttons ──────────────────────────────────────────
        nav = ctk.CTkFrame(self.sidebar, fg_color="transparent")
        nav.pack(fill="x", padx=10, pady=8)

        for key, (icon, label, _cls) in MODULES.items():
            btn = ctk.CTkButton(
                nav,
                text=f"  {icon}  {label}",
                anchor="w",
                command=lambda k=key: self.switch_module(k),
                fg_color="transparent",
                hover_color="#1e293b",
                text_color=NAV_TEXT,
                height=44,
                font=ctk.CTkFont(size=13),
                corner_radius=8,
            )
            btn.pack(fill="x", pady=3)
            self._nav_btns[key] = btn

        # ── Version footer ───────────────────────────────────────────────
        ctk.CTkLabel(
            self.sidebar, text="v1.0.0  ·  2026",
            font=ctk.CTkFont(size=9), text_color="#1e293b",
        ).pack(side="bottom", pady=10)

    def _build_statusbar(self) -> None:
        bar = ctk.CTkFrame(self.content, height=26, corner_radius=0,
                           fg_color="#060c18")
        bar.pack(side="bottom", fill="x")
        bar.pack_propagate(False)

        self._status_lbl = ctk.CTkLabel(
            bar, text="Ready — no data loaded",
            font=ctk.CTkFont(size=10), text_color="#334155", anchor="w",
        )
        self._status_lbl.pack(side="left", padx=14)

        self._stats_lbl = ctk.CTkLabel(
            bar, text="",
            font=ctk.CTkFont(size=10), text_color=NAV_ACCENT, anchor="e",
        )
        self._stats_lbl.pack(side="right", padx=14)

    # ------------------------------------------------------------------ #
    # Module switching                                                     #
    # ------------------------------------------------------------------ #

    def switch_module(self, key: str) -> None:
        if self._active_key:
            self._nav_btns[self._active_key].configure(
                fg_color="transparent", text_color=NAV_TEXT,
            )

        self._active_key = key
        self._nav_btns[key].configure(fg_color=NAV_ACTIVE, text_color=NAV_ACCENT)

        for mod in self._modules.values():
            mod.pack_forget()

        if key not in self._modules:
            self._modules[key] = self._instantiate(key)

        self._modules[key].pack(fill="both", expand=True)

    def _instantiate(self, key: str) -> ctk.CTkFrame:
        _icon, _label, cls = MODULES[key]
        if key == "A":
            return cls(self.content, self.cfg, self.engine)
        if key == "B":
            return cls(self.content, self.cfg, self.engine)
        if key == "C":
            return cls(self.content, self.cfg, self.factory)
        if key == "D":
            return cls(self.content, self.cfg)
        raise ValueError(f"Unknown module key: {key!r}")

    # ------------------------------------------------------------------ #
    # CSV loading                                                          #
    # ------------------------------------------------------------------ #

    def _open_csv(self) -> None:
        path = filedialog.askopenfilename(
            title="Open Testo log (CSV)",
            filetypes=[("CSV files", "*.csv"), ("All files", "*.*")],
        )
        if not path:
            return

        ok = self.engine.load_csv(path)
        if not ok:
            messagebox.showerror("Load error",
                                 f"Could not parse CSV file:\n{path}")
            return

        fname   = os.path.basename(path)
        n_rows  = len(self.engine.df)
        n_cols  = len(self.engine.df.columns)
        n_mapped = len(self.engine.col_map)

        self._file_lbl.configure(text=fname, text_color=NAV_ACCENT)
        self._status_lbl.configure(
            text=(f"Loaded: {fname}  │  {n_rows} samples  │"
                  f"  {n_cols} columns  │  {n_mapped} mapped"),
        )

        state_summary = self.engine.get_state_summary()
        parts = [f"{s}: {p:.0f}%" for s, p in sorted(state_summary.items())]
        self._stats_lbl.configure(text="  ".join(parts))

        for mod in self._modules.values():
            if hasattr(mod, "refresh"):
                mod.refresh()


# ─────────────────────────────────────────────────────────────────────────────
# Entry point
# ─────────────────────────────────────────────────────────────────────────────

def main() -> None:
    app = MainGUI()
    app.mainloop()


if __name__ == "__main__":
    main()
