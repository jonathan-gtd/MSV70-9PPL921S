use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use egui::{Context, RichText, Ui};

use crate::analysis::{compute_session, generate_report};
use crate::data::SessionData;
use crate::gui::{
    palette::*,
    report_panel,
    tab_jobs::JobsState,
};
use crate::jobs::{load_customjobs, load_pid_db};

// ── Background analysis result ────────────────────────────────────────────────
type AsyncResult = Result<(SessionData, String), String>;

pub struct MSV70App {
    // State
    pub csv_path:    String,
    pub session:     Option<SessionData>,
    pub report:      String,
    pub all_sessions: Vec<SessionData>,
    pub tab:         usize,
    pub status:      String,
    pub tips:        Vec<String>,
    pub tip_idx:     usize,

    // Jobs tab state
    pub jobs_state:     JobsState,
    pub customjobs_path: Option<PathBuf>,

    // Background analysis
    pub pending:     Option<Arc<Mutex<Option<AsyncResult>>>>,
    pub pending_all: Option<Arc<Mutex<Option<Vec<SessionData>>>>>,

    // Config
    pub logs_dir: Option<PathBuf>,
}

impl MSV70App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        apply_theme(&cc.egui_ctx);

        let logs_dir = find_logs_dir();
        let customjobs_path = find_customjobs();
        let mut jobs_state = JobsState::default();

        if let Some(ref p) = find_pid_db() {
            match load_pid_db(p) {
                Ok(db) => jobs_state.pid_db = db,
                Err(e) => eprintln!("PID DB: {e}"),
            }
        }
        if let Some(ref p) = customjobs_path {
            match load_customjobs(p) {
                Ok(jobs) => jobs_state.jobs = jobs,
                Err(e) => eprintln!("customjobs: {e}"),
            }
        }

        let mut app = MSV70App {
            csv_path: String::new(),
            session: None,
            report: String::new(),
            all_sessions: vec![],
            tab: 0,
            status: "Ready — select a CSV log or click Analyze to auto-load".into(),
            tips: vec![],
            tip_idx: 0,
            jobs_state,
            customjobs_path,
            pending: None,
            pending_all: None,
            logs_dir: logs_dir.clone(),
        };

        // Auto-load latest CSV
        if let Some(ref dir) = logs_dir {
            if let Some(latest) = find_latest_csv(dir) {
                app.csv_path = latest.display().to_string();
                app.start_analyze(latest);
            }
        }

        app
    }

    pub fn start_analyze(&mut self, path: PathBuf) {
        self.status = format!("Analyzing {}…", path.file_name().unwrap_or_default().to_string_lossy());
        let shared: Arc<Mutex<Option<AsyncResult>>> = Arc::new(Mutex::new(None));
        let shared_clone = shared.clone();
        std::thread::spawn(move || {
            let result = compute_session(&path)
                .map(|d| {
                    let report = generate_report(&d);
                    (d, report)
                });
            *shared_clone.lock().unwrap() = Some(result);
        });
        self.pending = Some(shared);
    }

    pub fn start_load_all(&mut self) {
        if let Some(ref dir) = self.logs_dir.clone() {
            let dir = dir.clone();
            let shared: Arc<Mutex<Option<Vec<SessionData>>>> = Arc::new(Mutex::new(None));
            let shared_clone = shared.clone();
            self.status = "Loading all logs…".into();
            std::thread::spawn(move || {
                let mut results = vec![];
                if let Ok(entries) = std::fs::read_dir(&dir) {
                    let mut paths: Vec<PathBuf> = entries
                        .flatten()
                        .map(|e| e.path())
                        .filter(|p| p.extension().map(|e| e == "csv").unwrap_or(false))
                        .collect();
                    paths.sort();
                    for p in paths {
                        if let Ok(d) = compute_session(&p) {
                            if !d.run_idx.is_empty() {
                                results.push(d);
                            }
                        }
                    }
                }
                *shared_clone.lock().unwrap() = Some(results);
            });
            self.pending_all = Some(shared);
        }
    }

    pub fn poll_background(&mut self) {
        // Single file
        if let Some(ref pending) = self.pending.clone() {
            if let Ok(mut guard) = pending.try_lock() {
                if let Some(result) = guard.take() {
                    self.pending = None;
                    match result {
                        Ok((d, report)) => {
                            let n_corr = d.corrections.len();
                            let n_ev   = d.events.len();
                            let name   = d.file_name.clone();
                            let dur    = d.dur_s;
                            let n      = d.n_rows;
                            self.tips  = build_tips(&d);
                            self.tip_idx = 0;
                            self.report  = report;
                            self.session = Some(d);
                            self.status  = format!("✓  {name}  ·  {n} rows  ·  {dur:.0}s  ·  {n_corr} cells to correct  ·  {n_ev} anomaly events");
                        }
                        Err(e) => {
                            self.status = format!("Error: {e}");
                        }
                    }
                }
            }
        }

        // All logs
        if let Some(ref pending) = self.pending_all.clone() {
            if let Ok(mut guard) = pending.try_lock() {
                if let Some(sessions) = guard.take() {
                    self.pending_all = None;
                    let n = sessions.len();
                    self.all_sessions = sessions;
                    self.status = format!("Trend: {n} logs loaded");
                    self.tab = 4; // Switch to Trend tab
                }
            }
        }
    }
}

impl eframe::App for MSV70App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.poll_background();

        // Keep repainting while background work is in progress
        if self.pending.is_some() || self.pending_all.is_some() {
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            show_toolbar(ui, self);
        });

        egui::TopBottomPanel::bottom("tip_bar").show(ctx, |ui| {
            show_tip_bar(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tab == 5 {
                // Jobs tab — full width
                crate::gui::tab_jobs::show(ui, &mut self.jobs_state, &self.customjobs_path);
            } else {
                // Left (report) | Right (charts)
                egui::SidePanel::left("report_panel")
                    .resizable(true)
                    .default_width(430.0)
                    .min_width(250.0)
                    .show_inside(ui, |ui| {
                        report_panel::show(ui, &self.report);
                    });

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    show_tabs(ui, self);
                });
            }
        });

    }
}

// ── Toolbar ───────────────────────────────────────────────────────────────────

fn show_toolbar(ui: &mut Ui, app: &mut MSV70App) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("MSV70 ANALYZER").color(ACCENT).strong());
        ui.separator();
        ui.label(RichText::new("Log:").color(FG_DIM));
        ui.add(egui::TextEdit::singleline(&mut app.csv_path).desired_width(360.0));

        if ui.button("Browse…").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .pick_file()
            {
                app.csv_path = path.display().to_string();
            }
        }

        let analyze_btn = egui::Button::new(RichText::new("▶ Analyze").color(egui::Color32::BLACK))
            .fill(ACCENT);
        if ui.add(analyze_btn).clicked() && app.pending.is_none() {
            let path = PathBuf::from(&app.csv_path);
            if path.exists() {
                app.start_analyze(path);
            } else {
                app.status = "File not found".into();
            }
        }

        if ui.button("⊕ All Logs").clicked() && app.pending_all.is_none() {
            app.start_load_all();
        }

        if app.pending.is_some() || app.pending_all.is_some() {
            ui.spinner();
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(&app.status).color(FG_DIM).small());
        });
    });
}

// ── Tab bar ───────────────────────────────────────────────────────────────────

fn show_tabs(ui: &mut Ui, app: &mut MSV70App) {
    let tabs = ["Overview", "STFT Map", "Time Series", "Trend", "Custom Jobs"];
    ui.horizontal(|ui| {
        for (i, &name) in tabs.iter().enumerate() {
            let sel = app.tab == i || (i == 4 && app.tab == 5);
            let rich = RichText::new(name).color(if sel { ACCENT } else { FG_DIM });
            if ui.selectable_label(sel, rich).clicked() {
                app.tab = i;
                if i == 4 { app.tab = 5; }
            }
        }
    });
    ui.separator();

    if let Some(ref d) = app.session {
        match app.tab {
            0 => crate::gui::tab_overview::show(ui, d),
            1 => crate::gui::tab_stft_map::show(ui, d),
            2 => crate::gui::tab_timeseries::show(ui, d),
            3 => crate::gui::tab_trend::show(ui, &app.all_sessions),
            _ => {}
        }
    } else {
        match app.tab {
            3 => crate::gui::tab_trend::show(ui, &app.all_sessions),
            _ => {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("No session loaded — select a CSV and click Analyze").color(FG_DIM));
                });
            }
        }
    }
}

// ── Tip bar ───────────────────────────────────────────────────────────────────

fn show_tip_bar(ui: &mut Ui, app: &mut MSV70App) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("💡").color(YELLOW));
        let tip = app.tips.get(app.tip_idx)
            .cloned()
            .unwrap_or_else(|| "All systems nominal — no corrections flagged".into());
        ui.label(RichText::new(&tip).color(ORANGE).small());
        if !app.tips.is_empty() && ui.small_button("▸").clicked() {
            app.tip_idx = (app.tip_idx + 1) % app.tips.len();
        }
    });
}

// ── Discovery helpers ─────────────────────────────────────────────────────────

fn find_logs_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let candidates = [
        exe.parent()?.parent()?.join("LOGS"),
        exe.parent()?.join("LOGS"),
        PathBuf::from("LOGS"),
    ];
    candidates.into_iter().find(|p| p.is_dir())
}

fn find_latest_csv(dir: &Path) -> Option<PathBuf> {
    let mut csvs: Vec<PathBuf> = std::fs::read_dir(dir).ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "csv").unwrap_or(false))
        .collect();
    csvs.sort();
    csvs.pop()
}

fn find_customjobs() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let candidates = [
        exe.parent()?.parent()?.join("TestO Datalogger/config/customjobs.xml"),
        PathBuf::from("TestO Datalogger/config/customjobs.xml"),
    ];
    candidates.into_iter().find(|p| p.exists())
}

fn find_pid_db() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let candidates = [
        exe.parent()?.parent()?.join("TestO Datalogger/PERSO/MSV70_jobs.txt"),
        PathBuf::from("TestO Datalogger/PERSO/MSV70_jobs.txt"),
    ];
    candidates.into_iter().find(|p| p.exists())
}

// ── Tips builder ──────────────────────────────────────────────────────────────

fn build_tips(d: &SessionData) -> Vec<String> {
    use crate::analysis::percentile;
    use crate::data::types::Phase;
    use crate::analysis::Thresholds as THR;

    let rows = &d.rows;
    let mut tips = vec![];

    // WOT lambda
    let wot_lmb: Vec<f64> = d.cl_b1_idx.iter()
        .filter(|&&i| rows[i].phase == Phase::Wot && rows[i].rpm > 2500.0 && rows[i].coolant_c > 75.0)
        .map(|&i| rows[i].lmb_b1).collect();
    if !wot_lmb.is_empty() {
        let m = wot_lmb.iter().sum::<f64>() / wot_lmb.len() as f64;
        if m > 0.96 {
            tips.push(format!("WOT λ={:.3} LEAN — injector may be maxed out or ip_mff_cor too low at 75–100% load", m));
        } else if m > 0.93 {
            tips.push(format!("WOT λ={:.3} slightly lean — monitor, target <0.92 for E85 power", m));
        }
    }

    // Top correction
    if let Some(top) = d.corrections.first() {
        tips.push(format!("Strongest correction: {} / {}% load → ×{:.3} on ip_mff_cor  ({:+.1}%)",
            crate::data::types::RPM_LABELS[top.rpm_idx],
            crate::data::types::LOAD_LABELS[top.load_idx],
            top.factor, top.avg_delta));
    }

    // DC
    if d.has_inj && !d.run_idx.is_empty() {
        let dc_max = d.run_idx.iter().filter_map(|&i| rows[i].inj_duty()).fold(f64::NEG_INFINITY, f64::max);
        if dc_max > 90.0 {
            tips.push(format!("Injector DC {:.0}% — physical limit reached; upgrade injectors or reduce WOT fuel demand", dc_max));
        } else if dc_max > 80.0 {
            tips.push(format!("Injector DC {:.0}% — headroom <20%; watch carefully at high RPM", dc_max));
        }
    }

    // LTFT
    if d.has_ltft && !d.run_idx.is_empty() {
        let mut m1v: Vec<f64> = d.run_idx.iter().map(|&i| rows[i].ltft_mul_b1).collect();
        let mut m2v: Vec<f64> = d.run_idx.iter().map(|&i| rows[i].ltft_mul_b2).collect();
        let mul1 = percentile(&mut m1v, 50.0);
        let mul2 = percentile(&mut m2v, 50.0);
        if mul1.abs() > 15.0 || mul2.abs() > 15.0 {
            tips.push(format!("LTFT Mul B1={:+.1}% B2={:+.1}% — near ±20% ceiling, reflash base map urgently", mul1, mul2));
        } else if mul1.abs() > 10.0 || mul2.abs() > 10.0 {
            tips.push(format!("LTFT Mul B1={:+.1}% B2={:+.1}% — elevated, schedule a base map correction", mul1, mul2));
        }
    }

    // STFT overall
    if !d.cl_b1_idx.is_empty() {
        let mut vals: Vec<f64> = d.cl_b1_idx.iter().map(|&i| rows[i].stft_b1).collect();
        let med = percentile(&mut vals, 50.0);
        if med.abs() < 3.0 {
            tips.push("STFT median within ±3% — base map well calibrated for logged conditions".into());
        }
    }

    // Knock
    if !d.retard_idx.is_empty() {
        let knock_cands = d.hot_idx.iter()
            .filter(|&&i| rows[i].rpm > THR::KNOCK_RPM_MIN && rows[i].load_pct > THR::KNOCK_LOAD_MIN)
            .count();
        if knock_cands > 0 {
            let ret_pct = 100.0 * d.retard_idx.len() as f64 / knock_cands as f64;
            if ret_pct > 5.0 {
                tips.push(format!("Knock proxy {:.1}% retard rate — verify fuel quality and ignition map", ret_pct));
            }
        }
    }

    tips
}

// ── Theme / fonts ─────────────────────────────────────────────────────────────

fn setup_fonts(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();
    // Use built-in fonts; system Segoe UI will be used as fallback on Windows
    ctx.set_fonts(fonts.clone());
    let _ = fonts; // suppress warning
}

fn apply_theme(ctx: &Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill            = BG;
    visuals.window_fill           = BG;
    visuals.extreme_bg_color      = BG2;
    visuals.code_bg_color         = BG2;
    visuals.widgets.noninteractive.bg_fill = BG2;
    visuals.widgets.inactive.bg_fill       = BG3;
    visuals.widgets.hovered.bg_fill        = BG3;
    visuals.widgets.active.bg_fill         = ACCENT;
    visuals.selection.bg_fill             = ACCENT;
    ctx.set_visuals(visuals);
}
