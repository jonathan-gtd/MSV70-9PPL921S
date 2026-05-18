use std::collections::HashMap;
use std::path::Path;
use csv::ReaderBuilder;

use super::types::{Row, NEAR_ZERO};

/// Canonical internal name → list of CSV header aliases (case-insensitive match)
static COL_ALIASES: &[(&str, &[&str])] = &[
    ("time_ms",     &["time"]),
    ("rpm",         &["RPM"]),
    ("load_pct",    &["Load %"]),
    ("pedal_pct",   &["Pedal %"]),
    ("speed_kmh",   &["Speed km/h"]),
    ("coolant_c",   &["Cool degC"]),
    ("stft_b1",     &["STFTGrp1", "STFT B1"]),
    ("stft_b2",     &["STFTGrp2", "STFT B2"]),
    ("ltft_add_b1", &["LTFT add. B1"]),
    ("ltft_add_b2", &["LTFT add. B2"]),
    ("ltft_mul_b1", &["LTFT Mul B1"]),
    ("ltft_mul_b2", &["LTFT Mul B2"]),
    ("lmb_b1",      &["Lmb Act B1"]),
    ("lmb_b2",      &["Lmb Act B2"]),
    ("lmb_tgt_b1",  &["Lmb Tgt B1"]),
    ("maf_kgh",     &["AirMass", "MAF kg/h"]),
    ("inj_ms",      &["Inj C1 ms"]),
    ("vanos_act",   &["Vanos InAct"]),
    ("vanos_tgt",   &["Vanos InTgt"]),
    ("ign_deg",     &["Ign Cyl1 deg"]),
    ("iat_c",       &["IAT degC"]),
    ("oil_c",       &["Oil degC"]),
];

fn safe(v: f64) -> f64 {
    if v.abs() < NEAR_ZERO { 0.0 } else { v }
}

fn parse_f64(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0)
}

pub struct LoadResult {
    pub rows:      Vec<Row>,
    pub available: HashMap<String, bool>,
}

pub fn load_csv(path: &Path) -> Result<LoadResult, String> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .from_path(path)
        .map_err(|e| format!("Cannot open CSV: {e}"))?;

    // Build header → column-index map (case-insensitive)
    let headers: Vec<String> = rdr.headers()
        .map_err(|e| format!("Cannot read headers: {e}"))?
        .iter()
        .map(|h| h.to_lowercase())
        .collect();

    // Map internal name → column index
    let mut col_idx: HashMap<&str, usize> = HashMap::new();
    for &(internal, aliases) in COL_ALIASES {
        for alias in aliases {
            let lc = alias.to_lowercase();
            if let Some(pos) = headers.iter().position(|h| h == &lc) {
                col_idx.insert(internal, pos);
                break;
            }
        }
    }

    if !col_idx.contains_key("time_ms") || !col_idx.contains_key("rpm") {
        return Err("Missing required column 'time' or 'RPM'".to_string());
    }

    let get = |record: &csv::StringRecord, name: &str| -> f64 {
        col_idx.get(name)
            .and_then(|&i| record.get(i))
            .map(parse_f64)
            .unwrap_or(0.0)
    };

    let mut rows: Vec<Row> = Vec::new();

    for result in rdr.records() {
        let rec = result.map_err(|e| format!("CSV parse error: {e}"))?;

        let mut row = Row::default();
        row.time_ms     = get(&rec, "time_ms");
        row.rpm         = get(&rec, "rpm");
        row.load_pct    = get(&rec, "load_pct");
        row.pedal_pct   = get(&rec, "pedal_pct");
        row.speed_kmh   = get(&rec, "speed_kmh");
        row.coolant_c   = if col_idx.contains_key("coolant_c") { get(&rec, "coolant_c") } else { 20.0 };
        row.lmb_b1      = if col_idx.contains_key("lmb_b1")    { let v = get(&rec, "lmb_b1"); if v == 0.0 { 1.0 } else { v } } else { 1.0 };
        row.lmb_b2      = if col_idx.contains_key("lmb_b2")    { let v = get(&rec, "lmb_b2"); if v == 0.0 { 1.0 } else { v } } else { 1.0 };
        row.lmb_tgt_b1  = if col_idx.contains_key("lmb_tgt_b1"){ let v = get(&rec, "lmb_tgt_b1"); if v == 0.0 { 1.0 } else { v } } else { 1.0 };
        row.maf_kgh     = get(&rec, "maf_kgh");
        row.inj_ms      = get(&rec, "inj_ms");
        row.vanos_act   = get(&rec, "vanos_act");
        row.vanos_tgt   = get(&rec, "vanos_tgt");
        row.ign_deg     = get(&rec, "ign_deg");
        row.iat_c       = if col_idx.contains_key("iat_c") { get(&rec, "iat_c") } else { 20.0 };
        row.oil_c       = get(&rec, "oil_c");

        // Apply NEAR_ZERO suppression to fuel trim fields
        row.stft_b1     = safe(get(&rec, "stft_b1"));
        row.stft_b2     = safe(get(&rec, "stft_b2"));
        row.ltft_add_b1 = safe(get(&rec, "ltft_add_b1"));
        row.ltft_add_b2 = safe(get(&rec, "ltft_add_b2"));
        row.ltft_mul_b1 = safe(get(&rec, "ltft_mul_b1"));
        row.ltft_mul_b2 = safe(get(&rec, "ltft_mul_b2"));

        rows.push(row);
    }

    if rows.is_empty() {
        return Err("CSV is empty".to_string());
    }

    let mut available: HashMap<String, bool> = HashMap::new();
    for &(name, _) in COL_ALIASES {
        available.insert(name.to_string(), col_idx.contains_key(name));
    }

    Ok(LoadResult { rows, available })
}
