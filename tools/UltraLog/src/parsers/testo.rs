//! Testo Datalogger / MSV70 log file parser.
//!
//! Parses semicolon-delimited CSV files exported by the Testo ECU datalogger
//! connected to Siemens MSV70 ECUs.
//!
//! Format specifics:
//!  - First column header: "time" (lowercase, unlike ECUMaster which uses "TIME")
//!  - Time values are in **milliseconds** → divided by 1000 to produce seconds
//!  - Near-zero float artefacts (|v| < 1e-10) are clamped to 0.0 — Testo emits
//!    2.22045e-14 (machine epsilon) instead of a true zero for initialised channels

use serde::Serialize;
use std::error::Error;

use super::types::{Channel, Log, Meta, Parseable, Value};

const NEAR_ZERO: f64 = 1e-10;

/// Metadata for a Testo MSV70 log file
#[derive(Clone, Debug, Default, Serialize)]
pub struct TestoMeta {
    /// Number of channels (excluding the time column)
    pub channel_count: usize,
    /// Number of data rows
    pub data_points: usize,
}

/// A single Testo channel definition
#[derive(Clone, Debug, Default, Serialize)]
pub struct TestoChannel {
    /// Exact header name from the CSV (e.g. "Lambda B1", "STFT B1")
    pub name: String,
    /// Inferred physical unit
    pub unit: String,
}

impl TestoChannel {
    pub fn new(header: &str) -> Self {
        let name = header.trim().to_string();
        let unit = infer_unit(&name);
        Self { name, unit }
    }

    pub fn unit(&self) -> &str {
        &self.unit
    }
}

/// Infer a unit string from a Testo/MSV70 channel header name.
fn infer_unit(name: &str) -> String {
    let n = name.to_lowercase();

    // RPM
    if n == "rpm" || n.ends_with(" rpm") {
        return "RPM".to_string();
    }
    // Lambda / AFR
    if n.contains("lambda") || n == "afr" {
        return "λ".to_string();
    }
    // Fuel trims (STFT / LTFT) and generic % channels
    if n.contains("stft")
        || n.contains("ltft")
        || n.contains("pedal")
        || n.contains("load")
        || n.contains("pwm")
        || n.contains("duty")
        || n.contains("trim")
        || n.contains("adaption")
        || n.contains("integrator")
    {
        return "%".to_string();
    }
    // Temperature
    if n.contains("coolant") || n.contains("temp") || n.contains("oil") || n == "iat" {
        return "°C".to_string();
    }
    // Injection time
    if n.contains("inj") && (n.contains("time") || n.contains("tm")) {
        return "ms".to_string();
    }
    // Mass air flow
    if n.contains("airmas") || n.contains("maf") {
        return "kg/h".to_string();
    }
    // Speed
    if n.contains("speed") || n.contains("km/h") {
        return "km/h".to_string();
    }
    // Voltage
    if n.contains("voltage") || n.contains("volt") || n.contains("battery") {
        return "V".to_string();
    }
    // Angle / ignition
    if n.contains("angle") || n.contains("timing") || n.contains("vanos") {
        return "°".to_string();
    }
    // Pressure
    if n.contains("pressure") || n.contains("pres") || n.contains("baro") {
        return "kPa".to_string();
    }
    "".to_string()
}

// ─────────────────────────────────────────────────────────────────────────────

/// Testo / MSV70 log file parser
pub struct Testo;

impl Testo {
    /// Returns true when the file looks like a Testo export.
    ///
    /// Detection signature: first header field is exactly "time" (lowercase).
    /// This distinguishes it from ECUMaster which uses "TIME" (uppercase).
    pub fn detect(contents: &str) -> bool {
        if let Some(first_line) = contents.lines().next() {
            let first_col = first_line.split(';').next().unwrap_or("").trim();
            first_col == "time"
        } else {
            false
        }
    }
}

impl Parseable for Testo {
    fn parse(&self, file_contents: &str) -> Result<Log, Box<dyn Error>> {
        let line_count = file_contents.lines().count();
        let estimated_rows = line_count.saturating_sub(1);

        let mut channels: Vec<Channel> = Vec::new();
        let mut times: Vec<f64> = Vec::with_capacity(estimated_rows);
        let mut data: Vec<Vec<Value>> = Vec::with_capacity(estimated_rows);

        let mut lines = file_contents.lines();

        // ── Header row ────────────────────────────────────────────────────────
        let header = lines.next().ok_or("Testo: empty file")?;
        let cols: Vec<&str> = header.split(';').collect();

        if cols.is_empty() {
            return Err("Testo: no columns in header".into());
        }
        if cols[0].trim() != "time" {
            return Err("Testo: first column must be 'time'".into());
        }

        // Build channel list (skip col 0 = time)
        for col in cols.iter().skip(1) {
            channels.push(Channel::Testo(TestoChannel::new(col)));
        }

        let n_channels = channels.len();
        let mut last_values: Vec<f64> = vec![0.0; n_channels];

        // ── Data rows ─────────────────────────────────────────────────────────
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(';').collect();
            if parts.is_empty() {
                continue;
            }

            // Time column is in milliseconds — convert to seconds
            let time_ms: f64 = match parts[0].trim().parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            times.push(time_ms / 1000.0);

            let mut row: Vec<Value> = Vec::with_capacity(n_channels);

            for i in 0..n_channels {
                let raw = parts.get(i + 1).map(|s| s.trim()).unwrap_or("");
                let val: f64 = raw.parse().unwrap_or(last_values[i]);

                // Clamp Testo's machine-epsilon artefacts to exactly 0.0
                let val = if val.abs() < NEAR_ZERO { 0.0 } else { val };

                last_values[i] = val;
                row.push(Value::Float(val));
            }

            data.push(row);
        }

        tracing::info!(
            "Parsed Testo/MSV70 log: {} channels, {} samples",
            n_channels,
            data.len()
        );

        Ok(Log {
            meta: Meta::Testo(TestoMeta {
                channel_count: n_channels,
                data_points: data.len(),
            }),
            channels,
            times,
            data,
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "time;RPM;Lambda B1;STFT B1;Coolant\n\
                          13;0;1;2.22045e-14;21\n\
                          53;661;1.034;-1.005;108\n";

    #[test]
    fn detect_testo() {
        assert!(Testo::detect(SAMPLE));
        assert!(!Testo::detect("TIME;engine/rpm\n0;1000"));
        assert!(!Testo::detect("timestamp,rpm\n0,1000"));
    }

    #[test]
    fn parse_basic() {
        let log = Testo.parse(SAMPLE).unwrap();
        assert_eq!(log.channels.len(), 4);
        assert_eq!(log.channels[0].name(), "RPM");
        assert_eq!(log.channels[1].name(), "Lambda B1");
        // Time converted from ms to s
        assert!((log.times[0] - 0.013).abs() < 1e-9);
        assert!((log.times[1] - 0.053).abs() < 1e-9);
        // Near-zero clamped
        assert_eq!(log.data[0][2].as_f64(), 0.0); // 2.22045e-14 → 0
        // Normal value preserved
        assert!((log.data[1][1].as_f64() - 1.034).abs() < 1e-6);
    }

    #[test]
    fn unit_inference() {
        assert_eq!(infer_unit("RPM"), "RPM");
        assert_eq!(infer_unit("Lambda B1"), "λ");
        assert_eq!(infer_unit("STFT B1"), "%");
        assert_eq!(infer_unit("LTFT mul. B1"), "%");
        assert_eq!(infer_unit("Coolant"), "°C");
        assert_eq!(infer_unit("InjTmCyl1"), "ms");
        assert_eq!(infer_unit("AirMass"), "kg/h");
    }
}
