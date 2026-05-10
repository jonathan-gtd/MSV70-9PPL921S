"""
DataEngine — CSV loading, cleaning, column mapping, lambda delay
compensation, engine state machine, and derived metrics.

Testo CSV quirk: near-zero artifacts appear as 2.22045e-14 (machine
epsilon × 10). Values below NEAR_ZERO_THRESH are clamped to 0.
"""
import numpy as np
import pandas as pd
from typing import Optional

NEAR_ZERO_THRESH = 1e-10


class DataEngine:
    def __init__(self, config_manager):
        self.cfg = config_manager
        self.df_raw: Optional[pd.DataFrame] = None
        self.df: Optional[pd.DataFrame] = None      # processed
        self.col_map: dict = {}                      # alias -> actual column
        self.states: Optional[pd.Series] = None
        self.source_path: str = ""

    # ------------------------------------------------------------------ #
    # Public: loading                                                      #
    # ------------------------------------------------------------------ #

    def load_csv(self, path: str) -> bool:
        try:
            df = pd.read_csv(path, sep=";", encoding="utf-8-sig", low_memory=False)
            df.columns = [c.strip() for c in df.columns]
            self.df_raw = df
            self.source_path = path
            self._build_col_map()
            self._process()
            return True
        except Exception as exc:
            print(f"[DataEngine] CSV load error: {exc}")
            return False

    # ------------------------------------------------------------------ #
    # Internal: processing pipeline                                        #
    # ------------------------------------------------------------------ #

    def _build_col_map(self) -> None:
        """Map internal aliases to the first matching actual CSV column."""
        self.col_map = {}
        if self.df_raw is None:
            return
        actual = set(self.df_raw.columns)
        for alias, candidates in self.cfg.get_all_mappings().items():
            for c in candidates:
                if c in actual:
                    self.col_map[alias] = c
                    break

    def _process(self) -> None:
        if self.df_raw is None:
            return
        df = self.df_raw.copy()

        # Clamp Testo near-zero artifacts to 0.0
        num_cols = df.select_dtypes(include=[np.number]).columns
        for col in num_cols:
            df[col] = df[col].where(df[col].abs() > NEAR_ZERO_THRESH, 0.0)

        # Derived time axis in seconds
        if "time" in self.col_map:
            df["_time_s"] = df[self.col_map["time"]] / 1000.0

        # Lambda transport delay compensation (shift lambda columns back in time)
        delay_ms = self.cfg.get_threshold("lambda_delay_ms", 500)
        n_shift = self._ms_to_rows(df, delay_ms)
        for alias in ("lambda_b1", "lambda_b2"):
            if alias in self.col_map:
                col = self.col_map[alias]
                df[col] = df[col].shift(-n_shift)

        self.df = df
        self._detect_states()

    def _ms_to_rows(self, df: pd.DataFrame, delay_ms: float) -> int:
        """Convert a ms delay to rows using the median sampling interval."""
        if len(df) < 2 or "time" not in self.col_map:
            return 0
        interval = df[self.col_map["time"]].diff().median()
        if pd.isna(interval) or interval <= 0:
            return 0
        return max(0, int(round(delay_ms / interval)))

    def _detect_states(self) -> None:
        """
        Classify each sample into: WOT / Idle / Transient / Cruise.
        Priority (highest first): WOT > Idle > Transient > Cruise.
        TPS rate of change is computed per-sample (units: %/sample).
        """
        if self.df is None:
            return
        df = self.df
        n = len(df)

        tps_thresh  = self.cfg.get_threshold("wot_tps_pct", 85.0)
        idle_rpm    = self.cfg.get_threshold("idle_rpm_max", 900)
        idle_load   = self.cfg.get_threshold("idle_load_max", 12.0)
        trans_rate  = self.cfg.get_threshold("transient_tps_rate", 5.0)

        def _s(alias):
            col = self.col_map.get(alias)
            return df[col] if col else pd.Series(np.nan, index=df.index)

        tps  = _s("tps")
        rpm  = _s("rpm")
        load = _s("load")

        tps_rate = tps.diff().abs()

        states = pd.Series(["Cruise"] * n, index=df.index)
        states[tps_rate > trans_rate]                   = "Transient"
        states[(rpm < idle_rpm) | (load < idle_load)]   = "Idle"
        states[tps > tps_thresh]                        = "WOT"

        self.states = states

    # ------------------------------------------------------------------ #
    # Public: accessors                                                    #
    # ------------------------------------------------------------------ #

    def get_col(self, alias: str) -> Optional[pd.Series]:
        if self.df is None or alias not in self.col_map:
            return None
        return self.df[self.col_map[alias]]

    def has_col(self, alias: str) -> bool:
        return alias in self.col_map and self.df is not None

    def available_cols(self) -> list:
        return list(self.df.columns) if self.df is not None else []

    def get_time_s(self) -> Optional[pd.Series]:
        if self.df is None:
            return None
        if "_time_s" in self.df.columns:
            return self.df["_time_s"]
        return self.get_col("time")

    def get_warm_mask(self) -> pd.Series:
        """Boolean mask: samples where coolant >= min analysis temperature."""
        coolant  = self.get_col("coolant")
        min_temp = self.cfg.get_threshold("min_analysis_temp_coolant", 70.0)
        if coolant is None:
            idx = self.df.index if self.df is not None else pd.RangeIndex(0)
            return pd.Series(True, index=idx)
        return coolant >= min_temp

    def get_state_summary(self) -> dict:
        if self.states is None:
            return {}
        total = len(self.states)
        return {s: cnt / total * 100
                for s, cnt in self.states.value_counts().items()}

    # ------------------------------------------------------------------ #
    # Public: derived metrics                                              #
    # ------------------------------------------------------------------ #

    def compute_combined_trim(self, bank: int) -> Optional[pd.Series]:
        """
        Combined fuel trim = STFT + LTFT_add + LTFT_mul (all in %).
        Any missing channel is treated as 0 rather than discarding the result.
        """
        if self.df is None:
            return None
        base = pd.Series(0.0, index=self.df.index)
        for alias in (f"stft_b{bank}", f"ltft_add_b{bank}", f"ltft_mul_b{bank}"):
            col = self.get_col(alias)
            if col is not None:
                base = base + col.fillna(0.0)
        return base

    def compute_bank_balance(self) -> Optional[pd.Series]:
        t1 = self.compute_combined_trim(1)
        t2 = self.compute_combined_trim(2)
        if t1 is None or t2 is None:
            return None
        return (t1 - t2).abs()

    def compute_injector_duty(self, inj_time_ms: pd.Series, rpm: pd.Series) -> pd.Series:
        """
        Injector duty cycle (%) = inj_time[ms] × RPM / 1200.
        Valid for a 4-stroke engine (one injection per 2 revolutions).
        """
        cycle_time_ms = 120_000.0 / rpm.replace(0, np.nan)
        return (inj_time_ms / cycle_time_ms * 100.0).clip(0, 100)

    def compute_rpm_delta(self) -> Optional[pd.Series]:
        """Rate of RPM rise (RPM/s) — proxy for relative power demand."""
        rpm = self.get_col("rpm")
        t_s = self.get_time_s()
        if rpm is None or t_s is None:
            return None
        dt = t_s.diff().replace(0, np.nan)
        return rpm.diff() / dt

    # ------------------------------------------------------------------ #
    # Public: analysis aggregates                                          #
    # ------------------------------------------------------------------ #

    def analyze_fuel_trims(self) -> dict:
        warm = self.get_warm_mask()
        result: dict = {}

        for bank in (1, 2):
            for alias in (f"stft_b{bank}", f"ltft_add_b{bank}", f"ltft_mul_b{bank}"):
                col = self.get_col(alias)
                if col is not None:
                    s = col[warm].dropna()
                    result[f"{alias}_mean"] = s.mean()
                    result[f"{alias}_std"]  = s.std()
                    result[f"{alias}_max"]  = s.abs().max()

        balance = self.compute_bank_balance()
        if balance is not None:
            result["bank_delta_mean"] = balance[warm].mean()
            result["bank_delta_max"]  = balance[warm].max()

        return result

    def analyze_maf_load(self) -> dict:
        maf  = self.get_col("maf")
        load = self.get_col("load")
        warm = self.get_warm_mask()
        if maf is None or load is None:
            return {}
        mask = warm & maf.notna() & load.notna() & (maf > 0) & (load > 0)
        if mask.sum() < 10:
            return {}
        return {"maf_load_correlation": maf[mask].corr(load[mask])}

    def build_rpm_load_heatmap(
        self, value_alias: str, rpm_bins: int = 12, load_bins: int = 10
    ) -> Optional[pd.DataFrame]:
        """
        Pivot table indexed by RPM bin (rows) × Load bin (columns).
        Cell value = mean of value_alias for warm-engine samples only.
        """
        rpm   = self.get_col("rpm")
        load  = self.get_col("load")
        value = self.get_col(value_alias)
        warm  = self.get_warm_mask()

        if any(x is None for x in (rpm, load, value)):
            return None

        mask = warm & rpm.notna() & load.notna() & value.notna()
        tmp = pd.DataFrame(
            {"rpm": rpm[mask], "load": load[mask], "v": value[mask]}
        )
        if tmp.empty:
            return None

        tmp["rpm_bin"]  = pd.cut(tmp["rpm"],  bins=rpm_bins)
        tmp["load_bin"] = pd.cut(tmp["load"], bins=load_bins)
        pivot = tmp.groupby(["rpm_bin", "load_bin"])["v"].mean().unstack("load_bin")
        return pivot
