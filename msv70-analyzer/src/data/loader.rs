use std::path::Path;
use csv::ReaderBuilder;
use crate::config::AppConfig;
use super::types::Row;

pub struct LoadResult {
    pub rows:      Vec<Row>,
    pub available: std::collections::HashMap<String, bool>,
}

pub fn load_csv(path: &Path, cfg: &AppConfig) -> Result<LoadResult, String> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .from_path(path)
        .map_err(|e| format!("Cannot open CSV: {e}"))?;

    let headers: Vec<String> = rdr.headers()
        .map_err(|e| format!("Cannot read headers: {e}"))?
        .iter()
        .map(|h| h.to_lowercase())
        .collect();

    // Map internal name → column index using config aliases
    let mut col_idx: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (internal, aliases) in &cfg.col_aliases {
        for alias in aliases {
            let lc = alias.to_lowercase();
            if let Some(pos) = headers.iter().position(|h| h == &lc) {
                col_idx.insert(internal.clone(), pos);
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
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0)
    };

    let safe = |v: f64| -> f64 {
        if v.abs() < cfg.near_zero { 0.0 } else { v }
    };

    let mut rows: Vec<Row> = Vec::new();
    for result in rdr.records() {
        let rec = result.map_err(|e| format!("CSV parse error: {e}"))?;
        let mut row = Row::default();

        row.time_ms   = get(&rec, "time_ms");
        row.rpm       = get(&rec, "rpm");
        row.load_pct  = get(&rec, "load_pct");
        row.pedal_pct = get(&rec, "pedal_pct");
        row.speed_kmh = get(&rec, "speed_kmh");
        row.coolant_c = if col_idx.contains_key("coolant_c") { get(&rec, "coolant_c") } else { 20.0 };
        row.iat_c     = if col_idx.contains_key("iat_c") { get(&rec, "iat_c") } else { 20.0 };
        row.oil_c     = get(&rec, "oil_c");
        row.maf_kgh   = get(&rec, "maf_kgh");
        row.inj_ms    = get(&rec, "inj_ms");
        row.vanos_act = get(&rec, "vanos_act");
        row.vanos_tgt = get(&rec, "vanos_tgt");
        row.ign_deg   = get(&rec, "ign_deg");

        // Lambda: treat 0.0 as missing → default 1.0
        let lmb1 = get(&rec, "lmb_b1");
        let lmb2 = get(&rec, "lmb_b2");
        row.lmb_b1    = if lmb1 == 0.0 { 1.0 } else { lmb1 };
        row.lmb_b2    = if lmb2 == 0.0 { 1.0 } else { lmb2 };
        let lmbt = get(&rec, "lmb_tgt_b1");
        row.lmb_tgt_b1 = if lmbt == 0.0 { 1.0 } else { lmbt };

        // Fuel trims: suppress near-zero noise
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

    let mut available: std::collections::HashMap<String, bool> = std::collections::HashMap::new();
    for internal in cfg.col_aliases.keys() {
        available.insert(internal.clone(), col_idx.contains_key(internal.as_str()));
    }

    Ok(LoadResult { rows, available })
}
