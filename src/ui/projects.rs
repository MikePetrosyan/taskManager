use egui::{Color32, Frame, Key, Modal};

use crate::Project;

pub fn new_project_modal(
    ctx: &egui::Context,
    new_project_name: &mut String,
    show_new_project: &mut bool,
    projects: &mut Vec<Project>,
) {
    Modal::new(egui::Id::new("new_project"))
        .backdrop_color(Color32::from_black_alpha(180))
        .frame(Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.label("Enter project name:");
            let resp = ui.text_edit_singleline(new_project_name);
            resp.request_focus();

            // ENTER = create
            if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                let name = new_project_name.trim();
                if !name.is_empty() {
                    projects.push(Project {
                        name: name.to_string(),
                        tasks: Vec::new(),
                    });
                }
                new_project_name.clear();
                *show_new_project = false;
            }
            if ui.input(|i| i.key_pressed(Key::Escape)) {
                new_project_name.clear();
                *show_new_project = false;
            }

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    let name = new_project_name.trim();
                    if !name.is_empty() {
                        projects.push(Project {
                            name: name.to_string(),
                            tasks: Vec::new(),
                        });
                    }
                    new_project_name.clear();
                    *show_new_project = false;
                }
                if ui.button("Cancel").clicked() {
                    new_project_name.clear();
                    *show_new_project = false;
                }
            });
        });
}

pub fn edit_project_modal(
    ctx: &egui::Context,
    project_edit_name: &mut String,
    project_edit_index: &mut Option<usize>,
    projects: &mut Vec<Project>,
    show_project_edit: &mut bool,
) {
    Modal::new(egui::Id::new("edit_project"))
        .backdrop_color(Color32::from_black_alpha(180))
        .frame(Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.label("Edit project name:");
            let resp = ui.text_edit_singleline(project_edit_name);
            resp.request_focus();

            // ENTER = save
            if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                if let Some(idx) = project_edit_index {
                    let name = project_edit_name.trim();
                    if !name.is_empty() {
                        projects[*idx].name = name.to_string();
                    }
                }
                *show_project_edit = false;
                *project_edit_index = None;
            }
            if ui.input(|i| i.key_pressed(Key::Escape)) {
                project_edit_name.clear();
                *show_project_edit = false;
            }
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    if let Some(idx) = project_edit_index {
                        let name = project_edit_name.trim();
                        if !name.is_empty() {
                            projects[*idx].name = name.to_string();
                        }
                    }
                    *show_project_edit = false;
                    *project_edit_index = None;
                }
                if ui.button("Cancel").clicked() {
                    *show_project_edit = false;
                    *project_edit_index = None;
                }
            });
        });
}
