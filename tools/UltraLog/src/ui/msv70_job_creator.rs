//! MSV70 Custom Job Creator Panel.
//!
//! Floating egui window that lets the user:
//!  1. Load an `MSV70_jobs.txt` definition file (tab-separated, 9 columns)
//!  2. Browse / filter opcodes in a scrollable table with checkboxes
//!  3. Assemble a named job from the selected parameters
//!  4. Queue multiple jobs and export them all to `customjobs.xml`
//!
//! XML argument format: `3;0xADDR1;0xADDR2;...`
//! — "3" is the fixed service byte for `STATUS_MESSWERTBLOCK_LESEN`.

use eframe::egui;
use std::collections::HashSet;

use crate::app::UltraLogApp;

// ─────────────────────────────────────────────────────────────────────────────
// Data types
// ─────────────────────────────────────────────────────────────────────────────

/// One opcode row from MSV70_jobs.txt
#[derive(Clone, Debug)]
pub struct MsvJob {
    pub hex: String,
    pub stat_name: String,
    pub desc_en: String,
    pub unit: String,
    pub ecu_var: String,
    pub type_str: String,
}

/// A queued job ready to be written to XML
#[derive(Clone, Debug)]
pub struct QueuedJob {
    pub name: String,
    /// Hex addresses of the selected opcodes (e.g. ["0x01FA", "0x01FB"])
    pub addresses: Vec<String>,
}

impl QueuedJob {
    /// Build the XML `<argument>` string: `3;0xADDR1;0xADDR2;...`
    pub fn argument_string(&self) -> String {
        let mut parts = vec!["3".to_string()];
        parts.extend(self.addresses.iter().cloned());
        parts.join(";")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Panel state
// ─────────────────────────────────────────────────────────────────────────────

pub struct Msv70JobCreatorState {
    /// Path to the loaded MSV70_jobs.txt
    pub jobs_path: String,
    /// Parsed opcode definitions (empty until a file is loaded)
    pub loaded_jobs: Vec<MsvJob>,
    /// Free-text filter applied to the table
    pub filter_text: String,
    /// Currently checked hex addresses
    pub selected_addrs: HashSet<String>,
    /// Name for the next job being assembled
    pub job_name: String,
    /// Batch queue (name → addresses)
    pub queue: Vec<QueuedJob>,
    /// Destination file for XML export
    pub export_path: String,
    /// Whether to merge into an existing customjobs.xml instead of overwriting
    pub merge_existing: bool,
    /// One-line status message shown after load/export actions
    pub status_msg: String,
}

impl Default for Msv70JobCreatorState {
    fn default() -> Self {
        Self {
            jobs_path: String::new(),
            loaded_jobs: Vec::new(),
            filter_text: String::new(),
            selected_addrs: HashSet::new(),
            job_name: String::new(),
            queue: Vec::new(),
            export_path: "customjobs.xml".to_string(),
            merge_existing: false,
            status_msg: String::new(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Parsing MSV70_jobs.txt
// ─────────────────────────────────────────────────────────────────────────────

/// Parse an `MSV70_jobs.txt` file.
///
/// Format: tab-separated, 9 columns per row (may have more tabs):
/// `ID \t Hex \t StatName \t DescEN \t Unit \t EcuVar \t Type \t - \t DescDE`
fn parse_jobs_txt(contents: &str) -> Vec<MsvJob> {
    let mut jobs = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        let cols: Vec<&str> = line.splitn(10, '\t').collect();
        if cols.len() < 7 {
            continue;
        }

        let hex = cols[1].trim().to_string();
        // Must look like a hex address (0x…) to be an opcode row
        if !hex.starts_with("0x") && !hex.starts_with("0X") {
            continue;
        }

        jobs.push(MsvJob {
            hex,
            stat_name: cols[2].trim().to_string(),
            desc_en:   cols[3].trim().to_string(),
            unit:      cols[4].trim().to_string(),
            ecu_var:   cols[5].trim().to_string(),
            type_str:  cols[6].trim().to_string(),
        });
    }

    jobs
}

// ─────────────────────────────────────────────────────────────────────────────
// XML generation
// ─────────────────────────────────────────────────────────────────────────────

/// Build the full XML string for a list of queued jobs.
fn build_xml(jobs: &[QueuedJob]) -> String {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<customjobs>\n");
    for job in jobs {
        xml.push_str(&format!(
            "  <job name=\"{}\">\n    <argument>{}</argument>\n  </job>\n",
            escape_xml(&job.name),
            escape_xml(&job.argument_string()),
        ));
    }
    xml.push_str("</customjobs>\n");
    xml
}

/// Minimal XML character escaping for attribute/text values.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

/// Merge new jobs into an existing XML string, replacing jobs with duplicate
/// names and appending new ones.
fn merge_xml(existing: &str, new_jobs: &[QueuedJob]) -> String {
    // Quick-and-dirty: keep existing <job> blocks whose names are NOT in new_jobs,
    // then append the new ones.
    let new_names: HashSet<&str> = new_jobs.iter().map(|j| j.name.as_str()).collect();

    // Strip outer wrapper, collect surviving blocks
    let mut kept = Vec::new();
    let mut in_job = false;
    let mut current_name = String::new();
    let mut block = String::new();

    for line in existing.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<job ") || trimmed.starts_with("<job\t") {
            in_job = true;
            block.clear();
            block.push_str(line);
            block.push('\n');
            // Extract name="..."
            current_name = trimmed
                .split("name=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .unwrap_or("")
                .to_string();
        } else if trimmed == "</job>" && in_job {
            block.push_str(line);
            block.push('\n');
            in_job = false;
            if !new_names.contains(current_name.as_str()) {
                kept.push(block.clone());
            }
            block.clear();
        } else if in_job {
            block.push_str(line);
            block.push('\n');
        }
    }

    // Rebuild
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<customjobs>\n");
    for b in &kept {
        for line in b.lines() {
            xml.push_str("  ");
            xml.push_str(line.trim());
            xml.push('\n');
        }
    }
    for job in new_jobs {
        xml.push_str(&format!(
            "  <job name=\"{}\">\n    <argument>{}</argument>\n  </job>\n",
            escape_xml(&job.name),
            escape_xml(&job.argument_string()),
        ));
    }
    xml.push_str("</customjobs>\n");
    xml
}

// ─────────────────────────────────────────────────────────────────────────────
// Panel rendering
// ─────────────────────────────────────────────────────────────────────────────

impl UltraLogApp {
    /// Render the MSV70 Custom Job Creator floating window.
    pub fn render_msv70_job_creator(&mut self, ctx: &egui::Context) {
        if !self.show_msv70_job_creator {
            return;
        }

        let mut open = true;
        egui::Window::new("MSV70 — Custom Job Creator")
            .open(&mut open)
            .resizable(true)
            .default_width(760.0)
            .default_height(620.0)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                self.render_msv70_job_creator_content(ui, ctx);
            });

        if !open {
            self.show_msv70_job_creator = false;
        }
    }

    fn render_msv70_job_creator_content(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // ── Load file row ─────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            ui.label("MSV70_jobs.txt:");
            ui.add(
                egui::TextEdit::singleline(&mut self.msv70_job_creator.jobs_path)
                    .desired_width(360.0)
                    .hint_text("path/to/MSV70_jobs.txt"),
            );

            if ui.button("Browse…").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Text files", &["txt"])
                    .pick_file()
                {
                    self.msv70_job_creator.jobs_path = path.to_string_lossy().to_string();
                    self.msv70_job_creator_load_file();
                }
            }

            if ui.button("Load").clicked() {
                self.msv70_job_creator_load_file();
            }
        });

        // Status bar (load result / export result)
        if !self.msv70_job_creator.status_msg.is_empty() {
            let color = if self.msv70_job_creator.status_msg.starts_with('✓') {
                egui::Color32::from_rgb(80, 200, 120)
            } else if self.msv70_job_creator.status_msg.starts_with('✗') {
                egui::Color32::from_rgb(220, 80, 80)
            } else {
                egui::Color32::GRAY
            };
            ui.colored_label(color, &self.msv70_job_creator.status_msg.clone());
        }

        if self.msv70_job_creator.loaded_jobs.is_empty() {
            ui.add_space(12.0);
            ui.colored_label(egui::Color32::GRAY, "Load an MSV70_jobs.txt file to browse opcodes.");
            return;
        }

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        // Split the remaining area into left (opcode table) and right (assembly/queue)
        let avail = ui.available_size();
        let left_w = (avail.x * 0.60).max(300.0);

        ui.horizontal_top(|ui| {
            // ── Left: opcode browser ──────────────────────────────────────────
            ui.vertical(|ui| {
                ui.set_width(left_w);

                // Filter bar
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.msv70_job_creator.filter_text)
                            .desired_width(220.0)
                            .hint_text("name / address / unit…"),
                    );
                    if ui.small_button("✕").clicked() {
                        self.msv70_job_creator.filter_text.clear();
                    }
                    let sel_count = self.msv70_job_creator.selected_addrs.len();
                    if sel_count > 0 {
                        ui.label(
                            egui::RichText::new(format!("{} selected", sel_count))
                                .color(egui::Color32::from_rgb(113, 180, 113)),
                        );
                    }
                });

                ui.add_space(4.0);

                // Column headers
                ui.horizontal(|ui| {
                    ui.add_space(22.0); // checkbox column
                    ui.label(egui::RichText::new("Address").strong().size(12.0));
                    ui.add_space(60.0);
                    ui.label(egui::RichText::new("Name").strong().size(12.0));
                    ui.add_space(80.0);
                    ui.label(egui::RichText::new("Unit").strong().size(12.0));
                });
                ui.separator();

                let filter = self.msv70_job_creator.filter_text.to_lowercase();

                // Collect filtered rows to avoid borrow conflict during rendering
                let rows: Vec<(String, String, String, String)> = self
                    .msv70_job_creator
                    .loaded_jobs
                    .iter()
                    .filter(|j| {
                        if filter.is_empty() {
                            return true;
                        }
                        j.hex.to_lowercase().contains(&filter)
                            || j.stat_name.to_lowercase().contains(&filter)
                            || j.desc_en.to_lowercase().contains(&filter)
                            || j.unit.to_lowercase().contains(&filter)
                    })
                    .map(|j| {
                        (
                            j.hex.clone(),
                            j.stat_name.clone(),
                            j.desc_en.clone(),
                            j.unit.clone(),
                        )
                    })
                    .collect();

                let row_h = 20.0;
                egui::ScrollArea::vertical()
                    .id_salt("msv70_opcode_list")
                    .max_height(avail.y - 160.0)
                    .show_rows(ui, row_h, rows.len(), |ui, range| {
                        for i in range {
                            let (hex, stat, desc, unit) = &rows[i];
                            let mut checked = self.msv70_job_creator.selected_addrs.contains(hex);
                            let label = if desc.is_empty() {
                                format!("{} ({})", stat, unit)
                            } else {
                                format!("{} — {} ({})", stat, desc, unit)
                            };

                            ui.horizontal(|ui| {
                                if ui.checkbox(&mut checked, "").changed() {
                                    if checked {
                                        self.msv70_job_creator.selected_addrs.insert(hex.clone());
                                    } else {
                                        self.msv70_job_creator.selected_addrs.remove(hex);
                                    }
                                }
                                ui.label(
                                    egui::RichText::new(hex)
                                        .monospace()
                                        .color(egui::Color32::from_rgb(150, 180, 220))
                                        .size(12.0),
                                );
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new(label).size(12.0));
                            });
                        }
                    });

                // Select-all / clear buttons
                ui.horizontal(|ui| {
                    if ui.small_button("Select all visible").clicked() {
                        for (hex, _, _, _) in &rows {
                            self.msv70_job_creator.selected_addrs.insert(hex.clone());
                        }
                    }
                    if ui.small_button("Clear selection").clicked() {
                        self.msv70_job_creator.selected_addrs.clear();
                    }
                });
            });

            ui.separator();

            // ── Right: job assembly + queue ───────────────────────────────────
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Assemble Job").strong());
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label("Job name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.msv70_job_creator.job_name)
                            .desired_width(180.0)
                            .hint_text("e.g. LTFT_B1"),
                    );
                });

                let sel_count = self.msv70_job_creator.selected_addrs.len();
                ui.label(
                    egui::RichText::new(format!("{} address(es) selected", sel_count))
                        .size(12.0)
                        .color(egui::Color32::GRAY),
                );

                if sel_count > 0 && !self.msv70_job_creator.job_name.is_empty() {
                    // Preview the argument string
                    let sorted_addrs: Vec<String> = {
                        let mut v: Vec<String> = self
                            .msv70_job_creator
                            .selected_addrs
                            .iter()
                            .cloned()
                            .collect();
                        v.sort();
                        v
                    };
                    let preview = format!(
                        "3;{}",
                        sorted_addrs.join(";")
                    );
                    ui.label(
                        egui::RichText::new("Preview:")
                            .size(11.0)
                            .color(egui::Color32::GRAY),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut preview.as_str())
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(2)
                            .interactive(false),
                    );
                }

                ui.add_space(4.0);

                let can_add = sel_count > 0 && !self.msv70_job_creator.job_name.trim().is_empty();
                ui.add_enabled_ui(can_add, |ui| {
                    if ui.button("➕  Add to Queue").clicked() {
                        let mut addresses: Vec<String> = self
                            .msv70_job_creator
                            .selected_addrs
                            .iter()
                            .cloned()
                            .collect();
                        addresses.sort();

                        let name = self.msv70_job_creator.job_name.trim().to_string();
                        // Replace existing job with same name
                        self.msv70_job_creator.queue.retain(|q| q.name != name);
                        self.msv70_job_creator.queue.push(QueuedJob { name, addresses });
                        self.msv70_job_creator.selected_addrs.clear();
                        self.msv70_job_creator.job_name.clear();
                        self.msv70_job_creator.status_msg =
                            "Job added to queue.".to_string();
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // Queue list
                ui.label(egui::RichText::new("Queue").strong());

                let queue_len = self.msv70_job_creator.queue.len();
                if queue_len == 0 {
                    ui.colored_label(egui::Color32::GRAY, "No jobs queued yet.");
                } else {
                    let mut remove_idx: Option<usize> = None;

                    egui::ScrollArea::vertical()
                        .id_salt("msv70_queue_scroll")
                        .max_height(160.0)
                        .show(ui, |ui| {
                            for (i, job) in self.msv70_job_creator.queue.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&job.name)
                                            .strong()
                                            .size(12.0),
                                    );
                                    ui.label(
                                        egui::RichText::new(
                                            format!("({} addr)", job.addresses.len()),
                                        )
                                        .size(11.0)
                                        .color(egui::Color32::GRAY),
                                    );
                                    if ui.small_button("✕").clicked() {
                                        remove_idx = Some(i);
                                    }
                                });
                            }
                        });

                    if let Some(idx) = remove_idx {
                        self.msv70_job_creator.queue.remove(idx);
                    }
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // Export section
                ui.label(egui::RichText::new("Export XML").strong());
                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label("Output:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.msv70_job_creator.export_path)
                            .desired_width(180.0)
                            .hint_text("customjobs.xml"),
                    );
                    if ui.small_button("…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("XML files", &["xml"])
                            .set_file_name("customjobs.xml")
                            .save_file()
                        {
                            self.msv70_job_creator.export_path =
                                path.to_string_lossy().to_string();
                        }
                    }
                });

                ui.checkbox(&mut self.msv70_job_creator.merge_existing, "Merge into existing file");

                let can_export = !self.msv70_job_creator.queue.is_empty()
                    && !self.msv70_job_creator.export_path.trim().is_empty();

                ui.add_space(4.0);
                ui.add_enabled_ui(can_export, |ui| {
                    if ui.button("💾  Export customjobs.xml").clicked() {
                        self.msv70_job_creator_export();
                    }
                });

                // Repaint so status messages appear immediately
                ctx.request_repaint();
            });
        });
    }

    /// Load the jobs file from `self.msv70_job_creator.jobs_path`.
    fn msv70_job_creator_load_file(&mut self) {
        let path = self.msv70_job_creator.jobs_path.trim().to_string();
        if path.is_empty() {
            self.msv70_job_creator.status_msg = "✗  No path specified.".to_string();
            return;
        }

        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                let jobs = parse_jobs_txt(&contents);
                let count = jobs.len();
                self.msv70_job_creator.loaded_jobs = jobs;
                self.msv70_job_creator.selected_addrs.clear();
                self.msv70_job_creator.status_msg =
                    format!("✓  Loaded {} opcodes from {}", count, path);
            }
            Err(e) => {
                self.msv70_job_creator.status_msg = format!("✗  {}", e);
            }
        }
    }

    /// Write (or merge) the queued jobs to `self.msv70_job_creator.export_path`.
    fn msv70_job_creator_export(&mut self) {
        let path = self.msv70_job_creator.export_path.trim().to_string();
        if path.is_empty() {
            self.msv70_job_creator.status_msg = "✗  No output path specified.".to_string();
            return;
        }

        let xml = if self.msv70_job_creator.merge_existing {
            match std::fs::read_to_string(&path) {
                Ok(existing) => merge_xml(&existing, &self.msv70_job_creator.queue),
                Err(_) => build_xml(&self.msv70_job_creator.queue),
            }
        } else {
            build_xml(&self.msv70_job_creator.queue)
        };

        match std::fs::write(&path, xml) {
            Ok(()) => {
                self.msv70_job_creator.status_msg =
                    format!("✓  Exported {} job(s) to {}", self.msv70_job_creator.queue.len(), path);
            }
            Err(e) => {
                self.msv70_job_creator.status_msg = format!("✗  Write failed: {}", e);
            }
        }
    }
}
