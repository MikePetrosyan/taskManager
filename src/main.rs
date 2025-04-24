use std::{collections::BTreeMap, sync::Arc};

use eframe::egui;
use eframe::egui::{
    CentralPanel, Color32, Context, Frame, Id, SidePanel, containers::modal::Modal,
};

use egui::{FontData, FontFamily, FontId, Key, Sense, Window};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Projects and Task Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}

pub struct FontDefinitions {
    pub font_data: BTreeMap<String, Arc<FontData>>,
    pub families: BTreeMap<FontFamily, Vec<String>>,
}

struct Task {
    name: String,
    done: bool,
}

struct Project {
    name: String,
    tasks: Vec<Task>,
}

struct MyEguiApp {
    projects: Vec<Project>,
    selected_project: Option<usize>,

    show_new_project: bool,
    new_project_name: String,
    show_project_edit: bool,
    project_edit_name: String,
    project_edit_index: Option<usize>,

    show_new_task: bool,
    new_task_name: String,
    show_task_edit: bool,
    edit_task_name: String,
    editing_task_index: Option<usize>,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            selected_project: None,

            show_new_project: false,
            new_project_name: String::new(),
            show_project_edit: false,
            project_edit_name: String::new(),
            project_edit_index: None,

            show_new_task: false,
            new_task_name: String::new(),
            show_task_edit: false,
            edit_task_name: String::new(),
            editing_task_index: None,
        }
    }
}
//styling
impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // --- Custom font & style setup ---
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "custom".to_owned(),
            Arc::new(FontData::from_static(include_bytes!("../assets/font.ttf"))),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "custom".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("custom".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.override_font_id = Some(FontId::new(16.0, FontFamily::Proportional));
        cc.egui_ctx.set_style(style);

        Self::default()
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── 1) NEW PROJECT MODAL ─────────────────────────────────────────
        if self.show_new_project {
            Modal::new(egui::Id::new("new_project"))
                .backdrop_color(Color32::from_black_alpha(180))
                .frame(Frame::popup(&ctx.style()))
                .show(ctx, |ui| {
                    ui.label("Enter project name:");
                    let resp = ui.text_edit_singleline(&mut self.new_project_name);
                    resp.request_focus();

                    // ENTER = create
                    if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        let name = self.new_project_name.trim();
                        if !name.is_empty() {
                            self.projects.push(Project {
                                name: name.to_string(),
                                tasks: Vec::new(),
                            });
                        }
                        self.new_project_name.clear();
                        self.show_new_project = false;
                    }
                    if ui.input(|i| i.key_pressed(Key::Escape)) {
                        self.new_project_name.clear();
                        self.show_new_project = false;
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let name = self.new_project_name.trim();
                            if !name.is_empty() {
                                self.projects.push(Project {
                                    name: name.to_string(),
                                    tasks: Vec::new(),
                                });
                            }
                            self.new_project_name.clear();
                            self.show_new_project = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.new_project_name.clear();
                            self.show_new_project = false;
                        }
                    });
                });
        }

        // ── 2) EDIT PROJECT MODAL ────────────────────────────────────────
        if self.show_project_edit {
            Modal::new(egui::Id::new("edit_project"))
                .backdrop_color(Color32::from_black_alpha(180))
                .frame(Frame::popup(&ctx.style()))
                .show(ctx, |ui| {
                    ui.label("Edit project name:");
                    let resp = ui.text_edit_singleline(&mut self.project_edit_name);
                    resp.request_focus();

                    // ENTER = save
                    if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        if let Some(idx) = self.project_edit_index {
                            let name = self.project_edit_name.trim();
                            if !name.is_empty() {
                                self.projects[idx].name = name.to_string();
                            }
                        }
                        self.show_project_edit = false;
                        self.project_edit_index = None;
                    }
                    if ui.input(|i| i.key_pressed(Key::Escape)) {
                        self.project_edit_name.clear();
                        self.show_project_edit = false;
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            if let Some(idx) = self.project_edit_index {
                                let name = self.project_edit_name.trim();
                                if !name.is_empty() {
                                    self.projects[idx].name = name.to_string();
                                }
                            }
                            self.show_project_edit = false;
                            self.project_edit_index = None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_project_edit = false;
                            self.project_edit_index = None;
                        }
                    });
                });
        }

        // ── 3) SIDE PANEL: PROJECT LIST + CONTROLS ───────────────────────
        let mut to_delete_project: Option<usize> = None;

        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(100.0..=600.0)
            .show(ctx, |ui| {
                ui.heading("Projects");
                ui.add_space(4.0);

                // Add Project button
                if ui.button("Add Project").clicked() {
                    self.new_project_name.clear();
                    self.show_new_project = true;
                }
                ui.separator();

                for (i, project) in self.projects.iter().enumerate() {
                    ui.horizontal(|ui| {
                        // selectable project name
                        let sel = self.selected_project == Some(i);
                        if ui.selectable_label(sel, &project.name).clicked() {
                            self.selected_project = Some(i);
                        }

                        // edit project
                        if ui.small_button("✒").clicked() {
                            self.project_edit_index = Some(i);
                            self.project_edit_name = project.name.clone();
                            self.show_project_edit = true;
                        }

                        // delete project
                        if ui.small_button("🗑").clicked() {
                            to_delete_project = Some(i);
                        }
                    });
                }
            });

        if let Some(i) = to_delete_project {
            self.projects.remove(i);
            if let Some(sel) = self.selected_project {
                self.selected_project = sel.checked_sub(1).filter(|&j| j != i || sel != i);
            }
        }

        // ── 4) TASK AREA ────────────────────────────────────────────────
        if let Some(proj_idx) = self.selected_project {
            let project = &mut self.projects[proj_idx];

            if self.show_new_task {
                Modal::new(egui::Id::new("new_task"))
                    .backdrop_color(Color32::from_black_alpha(180))
                    .frame(Frame::popup(&ctx.style()))
                    .show(ctx, |ui| {
                        ui.label("Enter task name:");
                        let resp = ui.text_edit_singleline(&mut self.new_task_name);
                        resp.request_focus();

                        if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                            let t = self.new_task_name.trim();
                            if !t.is_empty() {
                                project.tasks.push(Task {
                                    name: t.to_string(),
                                    done: false,
                                });
                            }
                            self.new_task_name.clear();
                            self.show_new_task = false;
                        }

                        if ui.input(|i| i.key_pressed(Key::Escape)) {
                            self.new_task_name.clear();
                            self.show_new_task = false;
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Create").clicked() {
                                let t = self.new_task_name.trim();
                                if !t.is_empty() {
                                    project.tasks.push(Task {
                                        name: t.to_string(),
                                        done: false,
                                    });
                                }
                                self.new_task_name.clear();
                                self.show_new_task = false;
                            }
                            if ui.button("Cancel").clicked() {
                                self.new_task_name.clear();
                                self.show_new_task = false;
                            }
                        });
                    });
            }

            // Edit‐task modal
            if self.show_task_edit {
                Modal::new(egui::Id::new("edit_task"))
                    .backdrop_color(Color32::from_black_alpha(180))
                    .frame(Frame::popup(&ctx.style()))
                    .show(ctx, |ui| {
                        ui.label("Edit task name:");
                        let resp = ui.text_edit_singleline(&mut self.edit_task_name);
                        resp.request_focus();

                        if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                            if let Some(ti) = self.editing_task_index {
                                let t = self.edit_task_name.trim();
                                if !t.is_empty() {
                                    project.tasks[ti].name = t.to_string();
                                }
                            }
                            self.show_task_edit = false;
                            self.editing_task_index = None;
                        }
                        if ui.input(|i| i.key_pressed(Key::Escape)) {
                            self.edit_task_name.clear();
                            self.show_task_edit = false;
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                if let Some(ti) = self.editing_task_index {
                                    let t = self.edit_task_name.trim();
                                    if !t.is_empty() {
                                        project.tasks[ti].name = t.to_string();
                                    }
                                }
                                self.show_task_edit = false;
                                self.editing_task_index = None;
                            }
                            if ui.button("Cancel").clicked() {
                                self.show_task_edit = false;
                                self.editing_task_index = None;
                            }
                        });
                    });
            }

            // Task list + controls
            let mut to_delete_task: Option<usize> = None;
            CentralPanel::default().show(ctx, |ui| {
                ui.heading(&project.name);
                ui.add_space(4.0);

                if ui.button("Add Task").clicked() {
                    self.new_task_name.clear();
                    self.show_new_task = true;
                }
                ui.separator();

                for (ti, task) in project.tasks.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        // checkbox + label
                        ui.checkbox(&mut task.done, "");
                        ui.label(&task.name);

                        // edit task
                        if ui.small_button("✒").clicked() {
                            self.editing_task_index = Some(ti);
                            self.edit_task_name = task.name.clone();
                            self.show_task_edit = true;
                        }

                        // delete task
                        if ui.small_button("🗑").clicked() {
                            to_delete_task = Some(ti);
                        }
                    });
                }
            });
            if let Some(ti) = to_delete_task {
                project.tasks.remove(ti);
            }
        } else {
            // ── 5) FALLBACK CENTRAL PANEL ──────────────────────────────────
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Select a project on the left");
            });
        }
    }
}
