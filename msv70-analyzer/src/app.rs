use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use egui::{Color32, Context, RichText, Ui};

use crate::analysis::{compute_session, generate_report, percentile};
use crate::config::AppConfig;
use crate::data::SessionData;
use crate::data::types::Phase;
use crate::gui::{palette::*, report_panel, tab_jobs::JobsState};
use crate::gui::tab_timeseries::{ChartPanel, ALL_CHANNELS, default_panels};
use crate::jobs::{load_customjobs, load_pid_db};

// ── Navigation ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nav {
    TimeSeries, StftMap, LambdaMap, Overview, Trend, Rapport, CustomJobs, Settings, BatchExport,
}

impl Nav {
    fn label(&self) -> &'static str {
        match self {
            Nav::TimeSeries => "Time Series",
            Nav::StftMap    => "STFT Map",
            Nav::LambdaMap  => "Lambda Map",
            Nav::Overview   => "Overview",
            Nav::Trend      => "Trend",
            Nav::Rapport    => "Rapport",
            Nav::CustomJobs => "Custom Jobs",
            Nav::Settings   => "Settings",
            Nav::BatchExport => "Export Masse",
        }
    }
    fn is_analysis(&self) -> bool {
        matches!(self, Nav::TimeSeries | Nav::StftMap | Nav::LambdaMap | Nav::Overview | Nav::Rapport)
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
    // Playback state
    pub cursor_s:        Option<f64>,
    pub is_playing:      bool,
    pub playback_speed:  f64,
    pub last_frame_time: Option<std::time::Instant>,
    // Dynamic chart system
    pub chart_panels:  Vec<ChartPanel>,
    pub next_panel_id: usize,
    pub show_phases:   bool,
    pub batch_state:   crate::gui::tab_batch::BatchState,
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
        if let Some(ref p) = find_pid_db()   { if let Ok(db)   = load_pid_db(p)    { jobs_state.pid_db = db; } }
        if let Some(ref p) = customjobs_path { if let Ok(jobs) = load_customjobs(p){ jobs_state.jobs = jobs; } }

        let mut app = MSV70App {
            config, config_dirty: false,
            session: None, report: String::new(), all_sessions: vec![],
            nav: Nav::TimeSeries,
            status: "Ready — select a log from the sidebar".into(),
            tips: vec![], tip_idx: 0,
            jobs_state, customjobs_path,
            pending: None, pending_all: None,
            logs_dir, log_files, selected_log: None,
            cursor_s: None, is_playing: false, playback_speed: 1.0, last_frame_time: None,
            chart_panels: default_panels(),
            next_panel_id: 3,
            show_phases: true,
            batch_state: crate::gui::tab_batch::BatchState::default(),
        };

        if let Some(path) = app.log_files.last().cloned() {
            app.selected_log = Some(app.log_files.len() - 1);
            app.start_analyze(path);
        }
        app
    }

    pub fn start_analyze(&mut self, path: PathBuf) {
        self.status = format!("Analyzing {}…", path.file_name().unwrap_or_default().to_string_lossy());
        self.cursor_s = None;
        self.is_playing = false;
        self.last_frame_time = None;
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

    fn advance_playback(&mut self) {
        if !self.is_playing { return; }
        let Some(ref d) = self.session else { return; };
        if d.rows.is_empty() { return; }
        let t0    = d.rows[0].ts();
        let t_max = d.rows.last().map(|r| r.ts() - t0).unwrap_or(0.0);
        let now   = std::time::Instant::now();
        if let Some(last) = self.last_frame_time {
            let dt  = now.duration_since(last).as_secs_f64() * self.playback_speed;
            let cur = self.cursor_s.get_or_insert(0.0);
            *cur += dt;
            if *cur >= t_max {
                *cur = t_max;
                self.is_playing = false;
                self.last_frame_time = None;
                return;
            }
        }
        self.last_frame_time = Some(now);
    }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for MSV70App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.poll_background();
        self.advance_playback();

        // File drop detection
        let dropped_paths: Vec<PathBuf> = ctx.input(|i| {
            i.raw.dropped_files.iter()
                .filter_map(|f| f.path.clone())
                .filter(|p| p.extension().map(|e| e == "csv").unwrap_or(false))
                .collect()
        });
        for path in dropped_paths {
            if !self.log_files.contains(&path) { self.log_files.push(path.clone()); }
            self.selected_log = self.log_files.iter().position(|p| p == &path);
            self.start_analyze(path.clone());
            if !self.nav.is_analysis() { self.nav = Nav::TimeSeries; }
        }

        if self.pending.is_some() || self.pending_all.is_some() || self.is_playing {
            ctx.request_repaint();
        }
        if self.config_dirty {
            let _ = self.config.save(&AppConfig::config_path());
            self.config_dirty = false;
        }

        // ── Chrome-style nav tab bar ───────────────────────────────────────────
        let current_nav = self.nav.clone();
        let mut new_nav: Option<Nav> = None;

        egui::TopBottomPanel::top("nav_tabs")
            .frame(egui::Frame::NONE.fill(BG2))
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("MSV70").color(OLIVE).strong().size(14.0));
                    ui.add_space(14.0);
                    for item in [
                        Nav::TimeSeries, Nav::StftMap, Nav::LambdaMap, Nav::Overview,
                        Nav::Trend, Nav::Rapport, Nav::CustomJobs, Nav::BatchExport, Nav::Settings,
                    ] {
                        if nav_tab(ui, item.label(), current_nav == item) {
                            new_nav = Some(item);
                        }
                        ui.add_space(2.0);
                    }
                });
                ui.add_space(2.0);
            });
        if let Some(n) = new_nav { self.nav = n; }

        // ── Bottom tip bar ─────────────────────────────────────────────────────
        egui::TopBottomPanel::bottom("tip_bar")
            .frame(egui::Frame::NONE.fill(BG2))
            .show(ctx, |ui| {
                ui.add_space(3.0);
                show_tip_bar(ui, self);
                ui.add_space(3.0);
            });

        // ── Left sidebar ───────────────────────────────────────────────────────
        let sidebar_w = self.config.sidebar_width;
        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(sidebar_w)
            .min_width(160.0)
            .frame(egui::Frame::NONE.fill(BG2).inner_margin(egui::Margin::symmetric(6, 6)))
            .show(ctx, |ui| {
                show_sidebar(ui, self);
            });

        // ── Central content panel ──────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.pending.is_some() || self.pending_all.is_some() {
                    ui.spinner();
                    ui.add_space(4.0);
                }
                ui.label(RichText::new(&self.status).color(FG_DIM).small());
            });

            let nav     = self.nav.clone();
            let cfg     = self.config.clone();
            let session = self.session.clone();

            match nav {
                Nav::CustomJobs => {
                    crate::gui::tab_jobs::show(ui, &mut self.jobs_state, &self.customjobs_path);
                }
                Nav::BatchExport => {
                    crate::gui::tab_batch::show(ui, &mut self.batch_state, &cfg);
                }
                Nav::Settings => {
                    crate::gui::tab_settings::show(ui, &mut self.config, &mut self.config_dirty);
                }
                Nav::Trend => {
                    crate::gui::tab_trend::show(ui, &self.all_sessions, &cfg);
                }
                Nav::Rapport => {
                    report_panel::show(ui, &self.report);
                }
                Nav::TimeSeries => {
                    if let Some(ref d) = session {
                        crate::gui::tab_timeseries::show(
                            ui, d, &cfg,
                            &mut self.chart_panels,
                            &mut self.next_panel_id,
                            &mut self.cursor_s,
                            &mut self.is_playing,
                            &mut self.playback_speed,
                            &mut self.last_frame_time,
                            &mut self.show_phases,
                        );
                    } else if show_empty(ui, ctx) {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("CSV", &["csv"]).pick_file()
                        {
                            if !self.log_files.contains(&path) { self.log_files.push(path.clone()); }
                            self.selected_log = self.log_files.iter().position(|p| p == &path);
                            self.start_analyze(path);
                        }
                    }
                }
                Nav::Overview => {
                    if let Some(ref d) = session {
                        crate::gui::tab_overview::show(ui, d, &cfg);
                    } else { let _ = show_empty(ui, ctx); }
                }
                Nav::StftMap => {
                    if let Some(ref d) = session {
                        crate::gui::tab_stft_map::show(ui, d, &cfg);
                    } else { let _ = show_empty(ui, ctx); }
                }
                Nav::LambdaMap => {
                    if let Some(ref d) = session {
                        crate::gui::tab_lambda_map::show(ui, d, &cfg);
                    } else { let _ = show_empty(ui, ctx); }
                }
            }
        });
    }
}

// ── Chrome-style nav tab ──────────────────────────────────────────────────────

fn nav_tab(ui: &mut egui::Ui, label: &str, is_active: bool) -> bool {
    let bg       = if is_active { BG4 } else { Color32::from_rgb(40, 40, 40) };
    let text_col = if is_active { Color32::WHITE } else { Color32::from_rgb(170, 170, 170) };
    let border   = if is_active { OLIVE } else { BG4 };
    let sw       = if is_active { 2.0_f32 } else { 1.0_f32 };

    let mut clicked = false;
    egui::Frame::NONE
        .fill(bg)
        .corner_radius(egui::CornerRadius { nw: 6, ne: 6, sw: 0, se: 0 })
        .stroke(egui::Stroke::new(sw, border))
        .inner_margin(egui::Margin { left: 12, right: 12, top: 5, bottom: 5 })
        .show(ui, |ui| {
            let r = ui.add(
                egui::Label::new(RichText::new(label).color(text_col).size(13.0))
                    .sense(egui::Sense::click()),
            );
            if r.clicked() { clicked = true; }
            if r.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
        });
    clicked
}

// ── Sidebar ───────────────────────────────────────────────────────────────────

fn show_sidebar(ui: &mut Ui, app: &mut MSV70App) {
    ui.add_space(4.0);

    // ── Channel palette ────────────────────────────────────────────────────
    ui.label(RichText::new("CHANNELS").color(FG_DIM).small().strong());
    ui.label(RichText::new("Drag onto a chart").color(FG_DIM).size(9.5).italics());
    ui.add_space(4.0);

    for key in ALL_CHANNELS {
        let col = key.color();
        ui.dnd_drag_source(
            egui::Id::new(format!("sidebar_{}", key.label())),
            key.clone(),
            |ui| {
                egui::Frame::NONE
                    .fill(BG3)
                    .stroke(egui::Stroke::new(1.2, col))
                    .corner_radius(3.0)
                    .inner_margin(egui::Margin { left: 6, right: 6, top: 3, bottom: 3 })
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width() - 4.0);
                        ui.horizontal(|ui| {
                            let (r, _) = ui.allocate_exact_size(
                                egui::vec2(8.0, 8.0), egui::Sense::hover(),
                            );
                            ui.painter().circle_filled(r.center(), 4.0, col);
                            ui.label(RichText::new(key.label()).color(FG).size(11.5));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(key.unit()).color(FG_DIM).size(9.5));
                            });
                        });
                    });
            },
        );
        ui.add_space(2.0);
    }

    ui.add_space(6.0);
    ui.separator();
    ui.add_space(4.0);

    // ── Log file list ──────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.label(RichText::new("LOGS").color(FG_DIM).small().strong());
        if ui.small_button("↺").on_hover_text("Rescan LOGS folder").clicked() {
            app.log_files = scan_logs(app.logs_dir.as_deref());
        }
    });
    ui.add_space(2.0);

    let list_h = ui.available_height() - 58.0;
    let mut clicked_idx: Option<usize> = None;
    egui::ScrollArea::vertical()
        .id_salt("log_list")
        .max_height(list_h)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            for (idx, path) in app.log_files.iter().enumerate() {
                let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");
                let sel  = app.selected_log == Some(idx);
                let color = if sel { ACCENT } else { FG_DIM };
                if ui.selectable_label(sel, RichText::new(name).color(color).small()).clicked() {
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
            app.nav = Nav::TimeSeries;
        }
    }

    ui.separator();
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        if ui.button("+ Trend").clicked() && app.pending_all.is_none() {
            app.start_load_all();
        }
        if ui.button("📂 Browse").clicked() {
            if let Some(path) = rfd::FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
                if !app.log_files.contains(&path) { app.log_files.push(path.clone()); }
                app.selected_log = app.log_files.iter().position(|p| p == &path);
                app.start_analyze(path);
                if !app.nav.is_analysis() { app.nav = Nav::TimeSeries; }
            }
        }
    });
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

// ── Empty state (UltraLog-style drop zone) ────────────────────────────────────

fn show_empty(ui: &mut Ui, ctx: &Context) -> bool {
    let is_hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
    let mut browse_clicked = false;

    ui.centered_and_justified(|ui| {
        let avail = ui.available_rect_before_wrap();
        let card_w = 320.0_f32;
        let card_h = 290.0_f32;
        let card   = egui::Rect::from_center_size(avail.center(), egui::vec2(card_w, card_h));

        // Card background + border
        ui.painter().rect_filled(card, 10.0, BG2);
        ui.painter().rect_stroke(
            card, 10.0,
            egui::Stroke::new(if is_hovering { 2.0 } else { 1.0 },
                if is_hovering { OLIVE } else { BG4 }),
            egui::StrokeKind::Middle,
        );

        // Upload arrow in circle
        let icon_c = egui::pos2(card.center().x, card.top() + 78.0);
        let icon_col = if is_hovering { OLIVE } else { FG_DIM };
        ui.painter().circle_stroke(icon_c, 28.0, egui::Stroke::new(1.5, icon_col));
        ui.painter().text(icon_c, egui::Align2::CENTER_CENTER,
            if is_hovering { "↓" } else { "↑" },
            egui::FontId::proportional(26.0), icon_col);

        // "Sélectionner un fichier" button
        let btn_rect = egui::Rect::from_center_size(
            egui::pos2(card.center().x, icon_c.y + 58.0),
            egui::vec2(210.0, 36.0),
        );
        let btn_resp = ui.allocate_rect(btn_rect, egui::Sense::click());
        let btn_fill = if btn_resp.hovered() { OLIVE.gamma_multiply(1.15) } else { OLIVE };
        ui.painter().rect_filled(btn_rect, 6.0, btn_fill);
        ui.painter().text(btn_rect.center(), egui::Align2::CENTER_CENTER,
            "Sélectionner un fichier",
            egui::FontId::proportional(13.0), Color32::WHITE);
        if btn_resp.hovered() {
            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if btn_resp.clicked() { browse_clicked = true; }

        // "ou" separator
        ui.painter().text(
            egui::pos2(card.center().x, btn_rect.bottom() + 14.0),
            egui::Align2::CENTER_CENTER, "ou",
            egui::FontId::proportional(11.0), FG_DIM);

        // Drop label
        ui.painter().text(
            egui::pos2(card.center().x, btn_rect.bottom() + 32.0),
            egui::Align2::CENTER_CENTER,
            if is_hovering { "Relâcher pour charger" } else { "Déposer le fichier ici" },
            egui::FontId::proportional(14.0),
            if is_hovering { OLIVE } else { FG });

        // File formats
        ui.painter().text(
            egui::pos2(card.center().x, card.bottom() - 16.0),
            egui::Align2::CENTER_CENTER,
            "CSV • LOG • TXT • MLG • LLG",
            egui::FontId::proportional(10.0),
            FG_DIM.gamma_multiply(0.6));

        let _ = ui.allocate_rect(card, egui::Sense::hover());
    });

    browse_clicked
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
    v.widgets.hovered.bg_fill        = BG4;
    v.widgets.active.bg_fill         = ACCENT;
    v.selection.bg_fill              = ACCENT;
    v.widgets.inactive.fg_stroke     = egui::Stroke::new(1.0, FG_DIM);
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, FG_DIM);
    ctx.set_visuals(v);
}
