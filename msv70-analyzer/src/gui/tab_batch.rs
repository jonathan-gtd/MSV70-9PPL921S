use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use egui::{RichText, Ui};
use crate::analysis::engine::compute_session;
use crate::config::AppConfig;
use super::palette::*;
use super::tab_stft_map::build_stft_csv;

// ── State ─────────────────────────────────────────────────────────────────────

pub struct BatchState {
    pub files:      Vec<PathBuf>,
    pub out_dir:    Option<PathBuf>,
    pub log:        Vec<String>,
    pub running:    Option<Arc<Mutex<Option<BatchResult>>>>,
    // Async file dialogs
    pub file_dlg:   Option<Arc<Mutex<Option<Option<Vec<PathBuf>>>>>>,
    pub dir_dlg:    Option<Arc<Mutex<Option<Option<PathBuf>>>>>,
}

struct BatchResult {
    ok:   Vec<String>,
    fail: Vec<String>,
}

impl Default for BatchState {
    fn default() -> Self {
        Self { files: vec![], out_dir: None, log: vec![], running: None,
               file_dlg: None, dir_dlg: None }
    }
}

// ── UI ────────────────────────────────────────────────────────────────────────

pub fn show(ui: &mut Ui, state: &mut BatchState, cfg: &AppConfig) {
    // Poll background task
    poll(state);

    egui::ScrollArea::vertical().id_salt("batch_scroll").show(ui, |ui| {
        ui.heading(RichText::new("Export STFT en masse").color(ACCENT));
        ui.label(RichText::new(
            "Sélectionne des fichiers CSV → choisis un dossier de sortie → Export All.\n\
             Chaque log génère un fichier <logname>_stft.csv."
        ).color(FG_DIM).small());
        ui.add_space(8.0);

        // ── File selection ────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            ui.label(RichText::new("Fichiers CSV").color(FG).strong());
            if ui.button("Ajouter…").clicked() && state.file_dlg.is_none() {
                let shared = Arc::new(Mutex::new(None));
                let clone = shared.clone();
                state.file_dlg = Some(shared);
                std::thread::spawn(move || {
                    let r = rfd::FileDialog::new().add_filter("CSV", &["csv"]).pick_files();
                    *clone.lock().unwrap() = Some(r);
                });
            }
            if !state.files.is_empty() {
                if ui.add(egui::Button::new(RichText::new("Tout effacer").color(RUST)).small()).clicked() {
                    state.files.clear();
                }
            }
        });

        if state.files.is_empty() {
            ui.label(RichText::new("Aucun fichier sélectionné.").color(FG_DIM).small().italics());
        } else {
            egui::ScrollArea::vertical().id_salt("batch_files").max_height(200.0).show(ui, |ui| {
                let mut to_remove: Option<usize> = None;
                for (i, path) in state.files.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                        if ui.add(egui::Button::new(RichText::new("x").color(RUST).size(10.0)).small().frame(false)).clicked() {
                            to_remove = Some(i);
                        }
                        ui.label(RichText::new(name).color(FG_DIM).small());
                    });
                }
                if let Some(i) = to_remove { state.files.remove(i); }
            });
        }

        ui.add_space(6.0);

        // ── Output directory ──────────────────────────────────────────────────
        ui.horizontal(|ui| {
            ui.label(RichText::new("Dossier de sortie").color(FG).strong());
            if ui.button("Choisir…").clicked() && state.dir_dlg.is_none() {
                let shared = Arc::new(Mutex::new(None));
                let clone = shared.clone();
                state.dir_dlg = Some(shared);
                std::thread::spawn(move || {
                    let r = rfd::FileDialog::new().pick_folder();
                    *clone.lock().unwrap() = Some(r);
                });
            }
        });
        match &state.out_dir {
            Some(d) => { ui.label(RichText::new(d.display().to_string()).color(ACCENT).small()); }
            None    => { ui.label(RichText::new("Non défini — les fichiers seront créés à côté de chaque CSV.").color(FG_DIM).small().italics()); }
        }

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);

        // ── Action ───────────────────────────────────────────────────────────
        let busy = state.running.is_some();
        let can_run = !state.files.is_empty() && !busy;

        ui.horizontal(|ui| {
            if ui.add_enabled(can_run,
                egui::Button::new(RichText::new(
                    if busy { "Export en cours…" } else { "Export All" }
                ).color(if can_run { LGREEN } else { FG_DIM }).size(13.0))
            ).clicked() {
                start_batch(state, cfg);
            }
            if busy { ui.spinner(); }
        });

        // ── Log ──────────────────────────────────────────────────────────────
        if !state.log.is_empty() {
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(2.0);
            egui::ScrollArea::vertical().id_salt("batch_log").max_height(240.0)
                .stick_to_bottom(true).show(ui, |ui| {
                    for line in &state.log {
                        let col = if line.starts_with("OK") { LGREEN }
                                  else if line.starts_with("ERR") { RED }
                                  else { FG_DIM };
                        ui.label(RichText::new(line).color(col).small());
                    }
                });
        }
    });
}

// ── Background task ───────────────────────────────────────────────────────────

fn start_batch(state: &mut BatchState, cfg: &AppConfig) {
    let files   = state.files.clone();
    let out_dir = state.out_dir.clone();
    let cfg     = cfg.clone();
    state.log.push(format!("Démarrage — {} fichiers…", files.len()));

    let shared: Arc<Mutex<Option<BatchResult>>> = Arc::new(Mutex::new(None));
    let clone = shared.clone();

    std::thread::spawn(move || {
        let mut ok   = vec![];
        let mut fail = vec![];

        for path in &files {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("log");
            match compute_session(path, &cfg) {
                Ok(session) => {
                    let csv_name = format!("{stem}_stft.csv");
                    let dest = match &out_dir {
                        Some(dir) => dir.join(&csv_name),
                        None      => path.with_file_name(&csv_name),
                    };
                    let content = build_stft_csv(&session, &cfg);
                    match std::fs::write(&dest, content) {
                        Ok(_)  => ok.push(format!("OK  {} → {}", stem, dest.display())),
                        Err(e) => fail.push(format!("ERR {} — écriture: {e}", stem)),
                    }
                }
                Err(e) => fail.push(format!("ERR {} — analyse: {e}", stem)),
            }
        }
        *clone.lock().unwrap() = Some(BatchResult { ok, fail });
    });

    state.running = Some(shared);
}

fn poll(state: &mut BatchState) {
    let result = if let Some(ref shared) = state.running {
        shared.try_lock().ok().and_then(|mut g| g.take())
    } else { None };
    if let Some(r) = result {
        state.running = None;
        for line in r.ok   { state.log.push(line); }
        for line in r.fail { state.log.push(line); }
        state.log.push("--- Terminé ---".to_string());
    }

    // Async file dialog
    let file_done = if let Some(ref fd) = state.file_dlg {
        fd.try_lock().ok().and_then(|mut g| g.take())
    } else { None };
    if let Some(result) = file_done {
        state.file_dlg = None;
        if let Some(paths) = result {
            for p in paths {
                if !state.files.contains(&p) { state.files.push(p); }
            }
        }
    }

    // Async dir dialog
    let dir_done = if let Some(ref dd) = state.dir_dlg {
        dd.try_lock().ok().and_then(|mut g| g.take())
    } else { None };
    if let Some(result) = dir_done {
        state.dir_dlg = None;
        if let Some(dir) = result {
            state.out_dir = Some(dir);
        }
    }
}
