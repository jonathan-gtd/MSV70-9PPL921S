"""
ConfigManager — loads/saves config.json and parses MSV70_jobs.txt.
The jobs file is tab-separated: ID | Hex | StatName | DescEN | Unit | EcuVar | Type | - | DescDE
"""
import json
import os
from typing import Optional


class ConfigManager:
    DEFAULT_CONFIG: dict = {
        "column_mapping": {
            "time":            ["time"],
            "rpm":             ["RPM"],
            "load":            ["CalcLoad", "Load %"],
            "tps":             ["Pedal %"],
            "lambda_b1":       ["Lambda B1"],
            "lambda_b2":       ["Lambda B2"],
            "lambda_tgt_b1":   ["Lambda B1 tgt"],
            "stft_b1":         ["STFT B1"],
            "stft_b2":         ["STFT B2"],
            "ltft_add_b1":     ["LTFT add. B1"],
            "ltft_add_b2":     ["LTFT add. B2"],
            "ltft_mul_b1":     ["LTFT mul. B1"],
            "ltft_mul_b2":     ["LTFT mul. B2"],
            "ltft_mul_hi_b1":  ["MixAdHiB1"],
            "ltft_mul_hi_b2":  ["MixAdHiB2"],
            "coolant":         ["Coolant"],
            "oil_temp":        ["Oil temp"],
            "maf":             ["AirMass"],
            "inj_time":        ["InjTmCyl1"],
            "lambda_shift_b1": ["LamShftB1"],
            "lambda_shift_b2": ["LamShftB2"],
            "speed":           ["Speed km/h"],
            "iat":             ["IAT °C"],
        },
        "thresholds": {
            "wot_tps_pct":               85.0,
            "idle_rpm_max":              900,
            "idle_load_max":             12.0,
            "cruise_tps_stability_pct":  3.0,
            "transient_tps_rate":        5.0,
            "lambda_delay_ms":           500,
            "min_analysis_temp_coolant": 70.0,
            "bank_balance_alert_pct":    5.0,
            "ltft_warning_pct":          5.0,
            "ltft_critical_pct":         10.0,
            "stft_warning_pct":          5.0,
            "max_job_params":            20,
        },
        "ecu": "MSV70",
        "jobs_file": "",
    }

    def __init__(self, config_path: str = "config.json"):
        self.config_path = config_path
        self.config = self._load()
        self._jobs_cache: Optional[list] = None

    # ------------------------------------------------------------------ #
    # Persistence                                                          #
    # ------------------------------------------------------------------ #

    def _load(self) -> dict:
        if os.path.exists(self.config_path):
            with open(self.config_path, "r", encoding="utf-8") as fh:
                saved = json.load(fh)
            merged = {
                **self.DEFAULT_CONFIG,
                **saved,
                "thresholds": {
                    **self.DEFAULT_CONFIG["thresholds"],
                    **saved.get("thresholds", {}),
                },
                "column_mapping": {
                    **self.DEFAULT_CONFIG["column_mapping"],
                    **saved.get("column_mapping", {}),
                },
            }
            return merged
        return {k: (v.copy() if isinstance(v, dict) else v)
                for k, v in self.DEFAULT_CONFIG.items()}

    def save(self) -> None:
        with open(self.config_path, "w", encoding="utf-8") as fh:
            json.dump(self.config, fh, indent=2, ensure_ascii=False)

    # ------------------------------------------------------------------ #
    # Threshold helpers                                                    #
    # ------------------------------------------------------------------ #

    def get_threshold(self, key: str, default=None):
        return self.config["thresholds"].get(key, default)

    def set_threshold(self, key: str, value) -> None:
        self.config["thresholds"][key] = value

    # ------------------------------------------------------------------ #
    # Column mapping helpers                                               #
    # ------------------------------------------------------------------ #

    def get_column_aliases(self, internal: str) -> list:
        return self.config["column_mapping"].get(internal, [])

    def get_all_mappings(self) -> dict:
        return self.config["column_mapping"]

    def set_column_aliases(self, internal: str, aliases: list) -> None:
        self.config["column_mapping"][internal] = aliases

    # ------------------------------------------------------------------ #
    # Jobs file parser                                                     #
    # ------------------------------------------------------------------ #

    def load_jobs(self, path: Optional[str] = None) -> list:
        """
        Parse MSV70_jobs.txt (tab-separated, 9 columns).
        Returns list of dicts with keys: id, hex, stat_name, desc_en,
        unit, ecu_var, data_type, desc_de.
        Results are cached until invalidate_jobs_cache() is called.
        """
        jobs_path = path or self.config.get("jobs_file", "")
        if not jobs_path or not os.path.exists(jobs_path):
            return []

        if self._jobs_cache is not None:
            return self._jobs_cache

        jobs: list = []
        with open(jobs_path, "r", encoding="utf-8", errors="replace") as fh:
            for line in fh:
                parts = line.strip().split("\t")
                if len(parts) < 4:
                    continue
                # Pad missing columns
                while len(parts) < 9:
                    parts.append("")
                jobs.append({
                    "id":        parts[0].strip(),
                    "hex":       parts[1].strip(),
                    "stat_name": parts[2].strip(),
                    "desc_en":   parts[3].strip(),
                    "unit":      parts[4].strip(),
                    "ecu_var":   parts[5].strip(),
                    "data_type": parts[6].strip(),
                    "desc_de":   parts[8].strip(),
                })

        self.config["jobs_file"] = jobs_path
        self._jobs_cache = jobs
        return jobs

    def invalidate_jobs_cache(self) -> None:
        self._jobs_cache = None
