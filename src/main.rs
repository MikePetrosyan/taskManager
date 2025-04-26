use std::{collections::BTreeMap, sync::Arc};

use eframe::egui;
use eframe::egui::CentralPanel;

use egui::{FontData, FontFamily, FontId, Key};
use serde::{Deserialize, Serialize};
use ui::panels::{central_panel, left_panel};
use ui::projects::{edit_project_modal, new_project_modal};
use ui::tasks::{show_new_task, show_task_edit};
mod ui;
mod persistence;
use crate::persistence::{load_state, save_state};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
}

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
        app.projects = load_state();
        app
    }
}

impl Drop for MyEguiApp {
    fn drop(&mut self) {
        save_state(&self.projects);
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
                self.project_edit_name = self.projects[idx].name.clone();
                self.show_project_edit = true;
            }
        }

        if self.show_new_project {
            new_project_modal(
                ctx,
                &mut self.new_project_name,
                &mut self.show_new_project,
                &mut self.projects,
            );
        }

        //edit project
        if self.show_project_edit {
            edit_project_modal(
                ctx,
                &mut self.project_edit_name,
                &mut self.project_edit_index,
                &mut self.projects,
                &mut self.show_project_edit,
            );
        }
        //side panel
        left_panel(
            ctx,
            &mut self.new_project_name,
            &mut self.show_new_project,
            &mut self.selected_project,
            &mut self.project_edit_index,
            &mut self.project_edit_name,
            &mut self.show_project_edit,
            &mut self.projects,
            &mut to_delete_project,
        );

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
                show_new_task(
                    ctx,
                    &mut self.new_task_name,
                    project,
                    &mut self.show_new_task,
                );
            }
            //edit task modal
            if self.show_task_edit {
                show_task_edit(
                    ctx,
                    &mut self.edit_task_name,
                    project,
                    &mut self.editing_task_index,
                    &mut self.show_task_edit,
                );
            }

            // task list + controls
            let mut to_delete_task: Option<usize> = None;
            central_panel(
                ctx,
                project,
                &mut self.hide_completed,
                &mut self.filter_text,
                &mut self.editing_task_index,
                &mut self.edit_task_name,
                &mut to_delete_task,
                &mut self.new_task_name,
                &mut self.show_task_edit,
                &mut self.show_new_task,
            );

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
