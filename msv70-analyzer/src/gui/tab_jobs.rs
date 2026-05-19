use egui::{RichText, ScrollArea, TextEdit, Ui};
use crate::jobs::{CustomJob, PidEntry};
use super::palette::*;

pub struct JobsState {
    pub pid_db:       Vec<PidEntry>,
    pub jobs:         Vec<CustomJob>,
    pub search:       String,
    pub editing:      CustomJob,
    pub selected_job: Option<usize>,
    pub status:       String,
}

impl Default for JobsState {
    fn default() -> Self {
        Self {
            pid_db: vec![],
            jobs: vec![],
            search: String::new(),
            editing: CustomJob {
                ecu: "MSV70".into(),
                jobname: "STATUS_MESSWERTBLOCK_LESEN".into(),
                virtualname: String::new(),
                comment: String::new(),
                pids: vec![],
            },
            selected_job: None,
            status: String::new(),
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut JobsState, customjobs_path: &Option<std::path::PathBuf>) {
    let avail = ui.available_height();
    let top_h = avail * 0.65;
    let bot_h = avail * 0.35 - 8.0;

    ui.horizontal(|ui| {
        // ── Left panel: PID database ──────────────────────────────────────
        ui.allocate_ui([ui.available_width() * 0.5, top_h].into(), |ui| {
            ui.label(RichText::new("MSV70 PID Database").color(ACCENT).strong());
            ui.add(TextEdit::singleline(&mut state.search).hint_text("Search PID / description…").desired_width(f32::INFINITY));
            ui.add_space(4.0);
            ScrollArea::vertical().id_salt("pid_scroll").max_height(top_h - 60.0).show(ui, |ui| {
                let q = state.search.clone();
                for pid in state.pid_db.iter().filter(|p| p.matches(&q)) {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&pid.pid_hex).color(YELLOW).monospace().small());
                        ui.label(RichText::new(&pid.short_name).color(FG_DIM).small());
                        let desc = if pid.description_en.is_empty() { &pid.description_de } else { &pid.description_en };
                        if ui.button(RichText::new(desc).small()).clicked() {
                            if state.editing.pid_count() < 20 && !state.editing.pids.contains(&pid.pid_num) {
                                state.editing.pids.push(pid.pid_num);
                            }
                        }
                        ui.label(RichText::new(&pid.unit).color(FG_DIM).small());
                    });
                }
            });
        });

        // ── Right panel: Job editor ───────────────────────────────────────
        ui.allocate_ui([ui.available_width(), top_h].into(), |ui| {
            ui.label(RichText::new("Job Editor").color(ACCENT).strong());
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut state.editing.virtualname);
            });
            ui.horizontal(|ui| {
                ui.label("Comment:");
                ui.text_edit_singleline(&mut state.editing.comment);
            });
            ui.add_space(4.0);
            ui.label(RichText::new(format!("PIDs: {}/20", state.editing.pid_count())).color(
                if state.editing.pid_count() >= 20 { RED } else { FG }
            ));

            let mut to_remove: Option<usize> = None;
            ScrollArea::vertical().id_salt("pid_list_scroll").max_height(top_h * 0.4).show(ui, |ui| {
                for (idx, &pid_num) in state.editing.pids.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("0x{:04X}", pid_num)).color(YELLOW).monospace().small());
                        // Try to find description
                        if let Some(entry) = state.pid_db.iter().find(|p| p.pid_num == pid_num) {
                            let desc = if entry.description_en.is_empty() { &entry.description_de } else { &entry.description_en };
                            ui.label(RichText::new(desc).color(FG_DIM).small());
                        }
                        if ui.small_button(RichText::new("x").color(RED)).clicked() {
                            to_remove = Some(idx);
                        }
                    });
                }
            });
            if let Some(idx) = to_remove {
                state.editing.pids.remove(idx);
            }

            ui.add_space(4.0);
            ui.label(RichText::new("Argument preview:").color(FG_DIM).small());
            ui.label(RichText::new(state.editing.argument_string()).color(YELLOW).monospace().small());

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button(RichText::new("Clear").color(ORANGE)).clicked() {
                    state.editing.pids.clear();
                    state.editing.virtualname.clear();
                    state.editing.comment.clear();
                }
                if ui.button(RichText::new("Add to file →").color(GREEN)).clicked() {
                    if !state.editing.virtualname.is_empty() && !state.editing.pids.is_empty() {
                        let job = state.editing.clone();
                        if let Some(existing) = state.jobs.iter_mut().find(|j| j.virtualname == job.virtualname) {
                            *existing = job;
                        } else {
                            state.jobs.push(job);
                        }
                        if let Some(path) = customjobs_path {
                            match crate::jobs::save_customjobs(path, &state.jobs) {
                                Ok(_)  => state.status = format!("Saved {} jobs to {}", state.jobs.len(), path.display()),
                                Err(e) => state.status = format!("Save error: {e}"),
                            }
                        }
                    } else {
                        state.status = "Set a virtualname and add at least one PID".into();
                    }
                }
            });

            if !state.status.is_empty() {
                ui.label(RichText::new(&state.status).color(GREEN).small());
            }
        });
    });

    // ── Bottom: existing jobs ─────────────────────────────────────────────
    ui.separator();
    ui.label(RichText::new("Existing Jobs").color(ACCENT).strong());
    ScrollArea::vertical().id_salt("jobs_scroll").max_height(bot_h).show(ui, |ui| {
        let mut to_delete: Option<usize> = None;
        for (idx, job) in state.jobs.iter().enumerate() {
            ui.horizontal(|ui| {
                let sel = state.selected_job == Some(idx);
                let label = RichText::new(format!("{} ({} PIDs)", job.virtualname, job.pid_count()))
                    .color(if sel { GREEN } else { FG });
                if ui.button(label).clicked() {
                    state.editing = job.clone();
                    state.selected_job = Some(idx);
                }
                ui.label(RichText::new(&job.comment).color(FG_DIM).small());
                if ui.small_button(RichText::new("x").color(RED)).clicked() {
                    to_delete = Some(idx);
                }
            });
        }
        if let Some(idx) = to_delete {
            state.jobs.remove(idx);
            if let Some(path) = customjobs_path {
                let _ = crate::jobs::save_customjobs(path, &state.jobs);
            }
        }
    });
}
