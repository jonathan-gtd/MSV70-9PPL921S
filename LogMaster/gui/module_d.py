"""
Module D — Configuration & Calibration.

Two-column layout:
  • Left  — Engine state thresholds (all numeric, editable).
  • Right — Column mapping editor: internal alias → comma-separated
            CSV header candidates.

Changes are written to config.json on Save.
"""
from tkinter import messagebox
import customtkinter as ctk


class ConfigurationModule(ctk.CTkFrame):
    def __init__(self, parent, config_manager):
        super().__init__(parent, fg_color="transparent")
        self.cfg = config_manager
        self._thresh_vars: dict  = {}   # key -> StringVar
        self._map_vars: dict     = {}   # alias -> StringVar
        self._build_ui()

    # ------------------------------------------------------------------ #
    # Layout                                                               #
    # ------------------------------------------------------------------ #

    def _build_ui(self) -> None:
        ctk.CTkLabel(
            self, text="Configuration & Calibration",
            font=ctk.CTkFont(size=20, weight="bold"),
        ).pack(anchor="w", padx=20, pady=(16, 6))

        body = ctk.CTkFrame(self, fg_color="transparent")
        body.pack(fill="both", expand=True, padx=20, pady=(0, 16))
        body.columnconfigure(0, weight=1)
        body.columnconfigure(1, weight=1)
        body.rowconfigure(0, weight=1)

        self._build_thresholds(body)
        self._build_mapping(body)
        self._build_footer()

    def _build_thresholds(self, parent: ctk.CTkFrame) -> None:
        frame = ctk.CTkFrame(parent, fg_color="#0d1117")
        frame.grid(row=0, column=0, sticky="nsew", padx=(0, 8))

        ctk.CTkLabel(frame, text="Engine State Thresholds",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=14, pady=(12, 6))

        THRESHOLDS = [
            ("wot_tps_pct",               "WOT pedal threshold (%)",            "0 – 100"),
            ("idle_rpm_max",              "Idle RPM max",                        "rpm"),
            ("idle_load_max",             "Idle load max (%)",                   "0 – 100"),
            ("transient_tps_rate",        "Transient TPS rate (%/sample)",       "0 – 50"),
            ("cruise_tps_stability_pct",  "Cruise TPS stability window (%)",     "0 – 20"),
            ("lambda_delay_ms",           "Lambda transport delay (ms)",         "0 – 2000"),
            ("min_analysis_temp_coolant", "Min coolant temp for analysis (°C)",  "°C"),
            ("bank_balance_alert_pct",    "Bank balance alert threshold (%)",    "0 – 20"),
            ("ltft_warning_pct",          "LTFT warning level (%)",              "0 – 30"),
            ("ltft_critical_pct",         "LTFT critical level (%)",             "0 – 30"),
            ("stft_warning_pct",          "STFT warning level (%)",              "0 – 30"),
            ("max_job_params",            "Max params per Testo job",            "1 – 50"),
        ]

        scroll = ctk.CTkScrollableFrame(frame, fg_color="transparent")
        scroll.pack(fill="both", expand=True, padx=8, pady=(0, 8))

        for key, label, hint in THRESHOLDS:
            row = ctk.CTkFrame(scroll, fg_color="transparent")
            row.pack(fill="x", pady=5)

            ctk.CTkLabel(row, text=label, anchor="w",
                         font=ctk.CTkFont(size=11)).pack(side="left", fill="x",
                                                          expand=True)
            var = ctk.StringVar(value=str(self.cfg.get_threshold(key, "")))
            self._thresh_vars[key] = var
            ctk.CTkEntry(row, textvariable=var, width=90,
                         placeholder_text=hint,
                         font=ctk.CTkFont(size=11)).pack(side="right", padx=(8, 0))

    def _build_mapping(self, parent: ctk.CTkFrame) -> None:
        frame = ctk.CTkFrame(parent, fg_color="#0d1117")
        frame.grid(row=0, column=1, sticky="nsew", padx=(8, 0))

        ctk.CTkLabel(frame, text="Column Mapping  (CSV headers → internal alias)",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=14, pady=(12, 2))
        ctk.CTkLabel(frame,
                     text="Comma-separated list of accepted CSV column names.",
                     font=ctk.CTkFont(size=10), text_color="#64748b").pack(
            anchor="w", padx=14, pady=(0, 6))

        scroll = ctk.CTkScrollableFrame(frame, fg_color="transparent")
        scroll.pack(fill="both", expand=True, padx=8, pady=(0, 8))

        for alias, candidates in self.cfg.get_all_mappings().items():
            row = ctk.CTkFrame(scroll, fg_color="transparent")
            row.pack(fill="x", pady=4)

            ctk.CTkLabel(row, text=alias, width=150, anchor="w",
                         font=ctk.CTkFont(size=11, weight="bold"),
                         text_color="#38bdf8").pack(side="left")
            var = ctk.StringVar(value=", ".join(candidates))
            self._map_vars[alias] = var
            ctk.CTkEntry(row, textvariable=var,
                         font=ctk.CTkFont(size=11)).pack(
                side="left", fill="x", expand=True, padx=(6, 0))

    def _build_footer(self) -> None:
        footer = ctk.CTkFrame(self, fg_color="transparent")
        footer.pack(fill="x", padx=20, pady=(0, 12))

        ctk.CTkButton(
            footer, text="💾  Save Configuration",
            command=self._save,
            fg_color="#166534", hover_color="#14532d",
            width=200, height=36,
        ).pack(side="right")

        ctk.CTkButton(
            footer, text="↩  Reset to Defaults",
            command=self._reset,
            fg_color="#581c87", hover_color="#3b0764",
            width=160, height=36,
        ).pack(side="right", padx=(0, 10))

    # ------------------------------------------------------------------ #
    # Actions                                                              #
    # ------------------------------------------------------------------ #

    INT_KEYS = {"idle_rpm_max", "lambda_delay_ms", "max_job_params"}

    def _save(self) -> None:
        errors = []

        for key, var in self._thresh_vars.items():
            raw = var.get().strip()
            try:
                val: float | int = int(raw) if key in self.INT_KEYS else float(raw)
                self.cfg.set_threshold(key, val)
            except ValueError:
                errors.append(f"Invalid value for '{key}': {raw!r}")

        for alias, var in self._map_vars.items():
            candidates = [c.strip() for c in var.get().split(",") if c.strip()]
            self.cfg.set_column_aliases(alias, candidates)

        if errors:
            messagebox.showerror("Validation errors",
                                 "\n".join(errors) + "\n\nFix the highlighted fields.")
            return

        self.cfg.save()
        messagebox.showinfo("Saved", "Configuration saved to config.json.")

    def _reset(self) -> None:
        if not messagebox.askyesno(
            "Reset configuration",
            "Restore all thresholds and column mappings to factory defaults?",
        ):
            return

        import copy
        self.cfg.config["thresholds"]     = copy.deepcopy(
            self.cfg.DEFAULT_CONFIG["thresholds"])
        self.cfg.config["column_mapping"] = copy.deepcopy(
            self.cfg.DEFAULT_CONFIG["column_mapping"])
        self.cfg.save()

        # Refresh displayed values
        for key, var in self._thresh_vars.items():
            var.set(str(self.cfg.get_threshold(key, "")))
        for alias, var in self._map_vars.items():
            var.set(", ".join(self.cfg.get_column_aliases(alias)))

        messagebox.showinfo("Reset", "Defaults restored and saved.")
