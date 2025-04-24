use std::fs;
use std::path::PathBuf;
use std::{collections::BTreeMap, sync::Arc};

use eframe::egui;
use eframe::egui::{
    CentralPanel, Color32, Frame, SidePanel, containers::modal::Modal,
};

use egui::{FontData, FontFamily, FontId, Key};
use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}
#[derive(Serialize, Deserialize)]
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
    filter_text: String,
    hide_completed: bool,
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
            filter_text: String::new(),
            hide_completed: false,
        }
    }
}
//styling
impl MyEguiApp {
    const STATE_FILE: &'static str = "projects.json";
    fn load_state() -> Vec<Project> {
        // look in a standard config directory (create if needed)
        let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push(Self::STATE_FILE);
        if let Ok(json) = std::fs::read_to_string(&path) {
            if let Ok(state) = serde_json::from_str::<Vec<Project>>(&json) {
                return state;
            }
        }
        Vec::new()
    }
    fn save_state(&self) {
        let mut path: PathBuf =
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let _ = fs::create_dir_all(&path);
        path.push(Self::STATE_FILE);
        if let Ok(json) = serde_json::to_string_pretty(&self.projects) {
            let _ = fs::write(path, json);
        }
    }
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

        let mut app = Self::default();
        app.projects = Self::load_state();
        app
    }

}

impl Drop for MyEguiApp {
    fn drop(&mut self) {
        self.save_state();
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // new project
        let mut to_delete_project: Option<usize> = None;
        
        if ctx.input(|i| i.key_pressed(Key::Delete)) {
            if let Some(idx) = self.selected_project {
                to_delete_project = Some(idx);
            }
        }
        let mods = ctx.input(|i| i.modifiers);
        // Ctrl/Cmd + N - new project
        if ctx.input(|i| i.key_pressed(Key::N)) && (mods.ctrl || mods.command) {
            self.new_project_name.clear();
            self.show_new_project = true;
        }
        // Ctrl/Cmd + T - new task
        if ctx.input(|i| i.key_pressed(Key::T)) && (mods.ctrl || mods.command) {
            if self.selected_project.is_some() {
                self.new_task_name.clear();
                self.show_new_task = true;
            }
        }
        //F2 - rename
        if ctx.input(|i| i.key_pressed(Key::F2)) {
            if let Some(idx) = self.selected_project {
                self.project_edit_index = Some(idx);
                self.project_edit_name  = self.projects[idx].name.clone();
                self.show_project_edit = true;
            }
        }
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

        //edit project
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
        //side panel
  

        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(500.0)
            .width_range(200.0..=700.0)
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
        //task area
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
            //edit task modal
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

            // task list + controls
            let mut to_delete_task: Option<usize> = None;
            CentralPanel::default().show(ctx, |ui| {
                ui.heading(&project.name);
                ui.add_space(4.0);
            
                // controls for filters
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.hide_completed, "Hide completed");
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.filter_text);
                });
                ui.separator();
            
                // ── single, combined loop ──
                for (ti, task) in project.tasks.iter_mut().enumerate() {
                    // hide completed 
                    if self.hide_completed && task.done {
                        continue;
                    }
                    // filter
                    let needle = self.filter_text.to_lowercase();
                    if !needle.is_empty() &&
                       !task.name.to_lowercase().contains(&needle)
                    {
                        continue;
                    }
            
                    // draw rows
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut task.done, "");
                        ui.label(&task.name);
            
                        if ui.small_button("✒").clicked() {
                            self.editing_task_index = Some(ti);
                            self.edit_task_name      = task.name.clone();
                            self.show_task_edit      = true;
                        }
                        if ui.small_button("🗑").clicked() {
                            to_delete_task = Some(ti);
                        }
                    });
                }
            
                ui.add_space(8.0);
                if ui.button("Add Task").clicked() {
                    self.new_task_name.clear();
                    self.show_new_task = true;
                }
            });
            if let Some(ti) = to_delete_task {
                project.tasks.remove(ti);
            }
        } else {
         //Fallback 
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Select a project on the left");
            });
        }
    }
}