use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use egui::{Context, RichText, Ui};

use crate::analysis::{compute_session, generate_report, percentile};
use crate::config::AppConfig;
use crate::data::SessionData;
use crate::data::types::Phase;
use crate::gui::{palette::*, report_panel, tab_jobs::JobsState};
use crate::jobs::{load_customjobs, load_pid_db};

// ── Navigation ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nav {
    Overview, StftMap, TimeSeries, Trend, CustomJobs, Settings,
}

impl Nav {
    fn label(&self) -> &'static str {
        match self {
            Nav::Overview   => "Overview",
            Nav::StftMap    => "STFT Map",
            Nav::TimeSeries => "Time Series",
            Nav::Trend      => "Trend",
            Nav::CustomJobs => "Custom Jobs",
            Nav::Settings   => "Settings",
        }
    }
    fn is_analysis(&self) -> bool {
        matches!(self, Nav::Overview | Nav::StftMap | Nav::TimeSeries)
    }
}

// ── App state ─────────────────────────────────────────────────────────────────

type AsyncResult = Result<(SessionData, String), String>;

pub struct MSV70App {
    pub config:       AppConfig,
    pub config_dirty: bool,

    pub session:      Option<SessionData>,
    pub report:       String,
    pub all_sessions: Vec<SessionData>,
    pub nav:          Nav,
    pub status:       String,
    pub tips:         Vec<String>,
    pub tip_idx:      usize,

    pub jobs_state:      JobsState,
    pub customjobs_path: Option<PathBuf>,

    pub pending:     Option<Arc<Mutex<Option<AsyncResult>>>>,
    pub pending_all: Option<Arc<Mutex<Option<Vec<SessionData>>>>>,

    pub logs_dir:     Option<PathBuf>,
    pub log_files:    Vec<PathBuf>,
    pub selected_log: Option<usize>,
}

impl MSV70App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        apply_theme(&cc.egui_ctx);

        let config = AppConfig::load(&AppConfig::config_path());
        let logs_dir  = find_logs_dir();
        let log_files = scan_logs(logs_dir.as_deref());

        let customjobs_path = find_customjobs();
        let mut jobs_state  = JobsState::default();
        if let Some(ref p) = find_pid_db()     { if let Ok(db)   = load_pid_db(p)    { jobs_state.pid_db = db; } }
        if let Some(ref p) = customjobs_path   { if let Ok(jobs) = load_customjobs(p){ jobs_state.jobs = jobs; } }

        let mut app = MSV70App {
            config, config_dirty: false,
            session: None, report: String::new(), all_sessions: vec![],
            nav: Nav::Overview,
            status: "Ready — select a log from the sidebar".into(),
            tips: vec![], tip_idx: 0,
            jobs_state, customjobs_path,
            pending: None, pending_all: None,
            logs_dir, log_files, selected_log: None,
        };

        if let Some(path) = app.log_files.last().cloned() {
            app.selected_log = Some(app.log_files.len() - 1);
            app.start_analyze(path);
        }
        app
    }

    pub fn start_analyze(&mut self, path: PathBuf) {
        self.status = format!("Analyzing {}…", path.file_name().unwrap_or_default().to_string_lossy());
        let cfg   = self.config.clone();
        let shared: Arc<Mutex<Option<AsyncResult>>> = Arc::new(Mutex::new(None));
        let clone = shared.clone();
        std::thread::spawn(move || {
            let res = compute_session(&path, &cfg)
                .map(|d| { let r = generate_report(&d, &cfg); (d, r) });
            *clone.lock().unwrap() = Some(res);
        });
        self.pending = Some(shared);
    }

    pub fn start_load_all(&mut self) {
        if self.log_files.is_empty() { return; }
        let paths = self.log_files.clone();
        let cfg   = self.config.clone();
        let shared: Arc<Mutex<Option<Vec<SessionData>>>> = Arc::new(Mutex::new(None));
        let clone = shared.clone();
        self.status = "Loading all logs…".into();
        std::thread::spawn(move || {
            let results: Vec<SessionData> = paths.into_iter()
                .filter_map(|p| compute_session(&p, &cfg).ok())
                .filter(|d| !d.run_idx.is_empty())
                .collect();
            *clone.lock().unwrap() = Some(results);
        });
        self.pending_all = Some(shared);
    }

    fn poll_background(&mut self) {
        if let Some(ref pending) = self.pending.clone() {
            if let Ok(mut g) = pending.try_lock() {
                if let Some(result) = g.take() {
                    self.pending = None;
                    match result {
                        Ok((d, report)) => {
                            let (n, dur, n_corr, n_ev, name) =
                                (d.n_rows, d.dur_s, d.corrections.len(), d.events.len(), d.file_name.clone());
                            self.tips    = build_tips(&d, &self.config);
                            self.tip_idx = 0;
                            self.report  = report;
                            self.session = Some(d);
                            self.status  = format!("✓  {name}  ·  {n} rows  ·  {dur:.0}s  ·  {n_corr} cells  ·  {n_ev} events");
                        }
                        Err(e) => { self.status = format!("Error: {e}"); }
                    }
                }
            }
        }
        if let Some(ref pending) = self.pending_all.clone() {
            if let Ok(mut g) = pending.try_lock() {
                if let Some(sessions) = g.take() {
                    self.pending_all = None;
                    let n = sessions.len();
                    self.all_sessions = sessions;
                    self.status = format!("Trend: {n} logs loaded");
                    self.nav = Nav::Trend;
                }
            }
        }
    }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for MSV70App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.poll_background();

        if self.pending.is_some() || self.pending_all.is_some() {
            ctx.request_repaint();
        }
        if self.config_dirty {
            let _ = self.config.save(&AppConfig::config_path());
            self.config_dirty = false;
        }

        egui::TopBottomPanel::bottom("tip_bar").show(ctx, |ui| {
            show_tip_bar(ui, self);
        });

        let sidebar_w = self.config.sidebar_width;
        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(sidebar_w)
            .min_width(160.0)
            .show(ctx, |ui| {
                show_sidebar(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(&self.status).color(FG_DIM).small());
            });
            ui.separator();

            let nav = self.nav.clone();
            match nav {
                Nav::CustomJobs => {
                    crate::gui::tab_jobs::show(ui, &mut self.jobs_state, &self.customjobs_path);
                }
                Nav::Settings => {
                    crate::gui::tab_settings::show(ui, &mut self.config, &mut self.config_dirty);
                }
                Nav::Trend => {
                    let cfg = self.config.clone();
                    crate::gui::tab_trend::show(ui, &self.all_sessions, &cfg);
                }
                _ => {
                    // Analysis tabs: report on left, chart on right
                    let report_w = self.config.report_width;
                    let report   = self.report.clone();
                    let cfg      = self.config.clone();
                    let session  = self.session.clone();

                    egui::SidePanel::left("report_panel")
                        .resizable(true)
                        .default_width(report_w)
                        .min_width(200.0)
                        .show_inside(ui, |ui| {
                            report_panel::show(ui, &report);
                        });

                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        if let Some(ref d) = session {
                            match self.nav {
                                Nav::Overview   => crate::gui::tab_overview::show(ui, d, &cfg),
                                Nav::StftMap    => crate::gui::tab_stft_map::show(ui, d, &cfg),
                                Nav::TimeSeries => crate::gui::tab_timeseries::show(ui, d, &cfg),
                                _ => {}
                            }
                        } else {
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new("Select a log from the sidebar to begin analysis")
                                    .color(FG_DIM));
                            });
                        }
                    });
                }
            }
        });
    }
}

// ── Sidebar ───────────────────────────────────────────────────────────────────

fn show_sidebar(ui: &mut Ui, app: &mut MSV70App) {
    ui.add_space(6.0);
    ui.label(RichText::new("MSV70 ANALYZER").color(ACCENT).strong().small());
    ui.label(RichText::new("N52B30 · E85_v5 · 9PPL921S").color(FG_DIM).small());
    ui.separator();

    ui.add_space(4.0);
    ui.label(RichText::new("ANALYSIS").color(FG_DIM).small().strong());
    for item in &[Nav::Overview, Nav::StftMap, Nav::TimeSeries, Nav::Trend] {
        let sel = app.nav == *item;
        if ui.selectable_label(sel, RichText::new(item.label()).color(if sel { ACCENT } else { FG }).strong()).clicked() {
            app.nav = item.clone();
        }
    }

    ui.add_space(4.0);
    ui.label(RichText::new("TOOLS").color(FG_DIM).small().strong());
    for item in &[Nav::CustomJobs, Nav::Settings] {
        let sel = app.nav == *item;
        if ui.selectable_label(sel, RichText::new(item.label()).color(if sel { ACCENT } else { FG })).clicked() {
            app.nav = item.clone();
        }
    }

    ui.separator();

    ui.horizontal(|ui| {
        ui.label(RichText::new("LOGS").color(FG_DIM).small().strong());
        if ui.small_button("⟳").on_hover_text("Rescan LOGS folder").clicked() {
            app.log_files = scan_logs(app.logs_dir.as_deref());
        }
    });

    let list_h = ui.available_height() - 56.0;
    let mut clicked_idx: Option<usize> = None;
    egui::ScrollArea::vertical()
        .id_salt("log_list")
        .max_height(list_h)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            for (idx, path) in app.log_files.iter().enumerate() {
                let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");
                let sel  = app.selected_log == Some(idx);
                if ui.selectable_label(sel, RichText::new(name).color(if sel { ACCENT } else { FG_DIM }).small()).clicked() {
                    clicked_idx = Some(idx);
                }
            }
            if app.log_files.is_empty() {
                ui.label(RichText::new("No CSV files found").color(FG_DIM).small());
            }
        });
    if let Some(idx) = clicked_idx {
        let path = app.log_files[idx].clone();
        app.selected_log = Some(idx);
        app.start_analyze(path);
        if !app.nav.is_analysis() && app.nav != Nav::Trend {
            app.nav = Nav::Overview;
        }
    }

    ui.separator();
    if ui.button("⊕ All Logs (Trend)").clicked() && app.pending_all.is_none() {
        app.start_load_all();
    }
    if ui.button("Browse…").clicked() {
        if let Some(path) = rfd::FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
            if !app.log_files.contains(&path) { app.log_files.push(path.clone()); }
            app.selected_log = app.log_files.iter().position(|p| p == &path);
            app.start_analyze(path);
            if !app.nav.is_analysis() { app.nav = Nav::Overview; }
        }
    }
    if app.pending.is_some() || app.pending_all.is_some() {
        ui.horizontal(|ui| { ui.spinner(); ui.label(RichText::new("working…").color(FG_DIM).small()); });
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

// ── Tips builder ──────────────────────────────────────────────────────────────

fn build_tips(d: &SessionData, cfg: &AppConfig) -> Vec<String> {
    let rows = &d.rows;
    let mut tips = vec![];

    let wot_lmb: Vec<f64> = d.cl_b1_idx.iter()
        .filter(|&&i| rows[i].phase == Phase::Wot && rows[i].rpm > 2500.0 && rows[i].coolant_c > 75.0)
        .map(|&i| rows[i].lmb_b1).collect();
    if !wot_lmb.is_empty() {
        let m = wot_lmb.iter().sum::<f64>() / wot_lmb.len() as f64;
        if m > cfg.lmb_wot_warn + 0.01 {
            tips.push(format!("WOT λ={:.3} LEAN — check ip_mff_cor at 75–100% load", m));
        } else if m > cfg.lmb_wot_target + 0.01 {
            tips.push(format!("WOT λ={:.3} slightly lean — target <{:.2}", m, cfg.lmb_wot_target));
        }
    }

    if let Some(top) = d.corrections.first() {
        let rpm_lbl  = d.rpm_labels.get(top.rpm_idx).map(|s| s.as_str()).unwrap_or("?");
        let load_lbl = d.load_labels.get(top.load_idx).map(|s| s.as_str()).unwrap_or("?");
        tips.push(format!("Strongest correction: {} / {}% load → ×{:.3} ({:+.1}%)",
            rpm_lbl, load_lbl, top.factor, top.avg_delta));
    }

    if d.has_inj && !d.run_idx.is_empty() {
        let dc_max = d.run_idx.iter().filter_map(|&i| rows[i].inj_duty()).fold(f64::NEG_INFINITY, f64::max);
        if dc_max > cfg.inj_duty_alarm {
            tips.push(format!("Injector DC {:.0}% — near physical limit; upgrade injectors", dc_max));
        } else if dc_max > cfg.inj_duty_warn {
            tips.push(format!("Injector DC {:.0}% — watch at high RPM", dc_max));
        }
    }

    if d.has_ltft && !d.run_idx.is_empty() {
        let mut m1v: Vec<f64> = d.run_idx.iter().map(|&i| rows[i].ltft_mul_b1).collect();
        let mut m2v: Vec<f64> = d.run_idx.iter().map(|&i| rows[i].ltft_mul_b2).collect();
        let mul1 = percentile(&mut m1v, 50.0);
        let mul2 = percentile(&mut m2v, 50.0);
        if mul1.abs() > cfg.ltft_alarm || mul2.abs() > cfg.ltft_alarm {
            tips.push(format!("LTFT Mul B1={:+.1}% B2={:+.1}% — near ceiling, reflash urgently", mul1, mul2));
        } else if mul1.abs() > cfg.ltft_warn || mul2.abs() > cfg.ltft_warn {
            tips.push(format!("LTFT Mul B1={:+.1}% B2={:+.1}% — elevated", mul1, mul2));
        }
    }

    if !d.cl_b1_idx.is_empty() {
        let mut vals: Vec<f64> = d.cl_b1_idx.iter().map(|&i| rows[i].stft_b1).collect();
        let med = percentile(&mut vals, 50.0);
        if med.abs() < cfg.stft_note / 2.0 {
            tips.push("STFT median within tight band — base map well calibrated".into());
        }
    }

    if !d.retard_idx.is_empty() {
        let cands = d.hot_idx.iter()
            .filter(|&&i| rows[i].rpm > cfg.knock_rpm_min && rows[i].load_pct > cfg.knock_load_min)
            .count();
        if cands > 0 {
            let ret_pct = 100.0 * d.retard_idx.len() as f64 / cands as f64;
            if ret_pct > 5.0 {
                tips.push(format!("Knock proxy {:.1}% retard rate — verify fuel quality", ret_pct));
            }
        }
    }

    tips
}

// ── Discovery ─────────────────────────────────────────────────────────────────

fn scan_logs(dir: Option<&Path>) -> Vec<PathBuf> {
    let Some(dir) = dir else { return vec![]; };
    let mut csvs: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|rd| rd.flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "csv").unwrap_or(false))
            .collect())
        .unwrap_or_default();
    csvs.sort();
    csvs
}

fn find_logs_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let candidates = [
        exe.parent()?.parent()?.join("LOGS"),
        exe.parent()?.join("LOGS"),
        PathBuf::from("LOGS"),
    ];
    candidates.into_iter().find(|p| p.is_dir())
}

fn find_customjobs() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    [
        exe.parent()?.parent()?.join("TestO Datalogger/config/customjobs.xml"),
        PathBuf::from("TestO Datalogger/config/customjobs.xml"),
    ].into_iter().find(|p| p.exists())
}

fn find_pid_db() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    [
        exe.parent()?.parent()?.join("TestO Datalogger/PERSO/MSV70_jobs.txt"),
        PathBuf::from("TestO Datalogger/PERSO/MSV70_jobs.txt"),
    ].into_iter().find(|p| p.exists())
}

// ── Theme / fonts ─────────────────────────────────────────────────────────────

fn setup_fonts(ctx: &Context) {
    ctx.set_fonts(egui::FontDefinitions::default());
}

fn apply_theme(ctx: &Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill                     = BG;
    v.window_fill                    = BG;
    v.extreme_bg_color               = BG2;
    v.code_bg_color                  = BG2;
    v.widgets.noninteractive.bg_fill = BG2;
    v.widgets.inactive.bg_fill       = BG3;
    v.widgets.hovered.bg_fill        = BG3;
    v.widgets.active.bg_fill         = ACCENT;
    v.selection.bg_fill              = ACCENT;
    ctx.set_visuals(v);
}
