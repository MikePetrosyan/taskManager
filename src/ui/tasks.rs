use egui::{Color32, Frame, Key, Modal};

use crate::{Project, Task};

pub fn show_new_task(
    ctx: &egui::Context,
    new_task_name: &mut String,
    project: &mut Project,
    show_new_task: &mut bool,
) {
    Modal::new(egui::Id::new("new_task"))
        .backdrop_color(Color32::from_black_alpha(180))
        .frame(Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.label("Enter task name:");
            let resp = ui.text_edit_singleline(new_task_name);
            resp.request_focus();

            if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                let t = new_task_name.trim();
                if !t.is_empty() {
                    project.tasks.push(Task {
                        name: t.to_string(),
                        done: false,
                    });
                }
                new_task_name.clear();
                *show_new_task = false;
            }

            if ui.input(|i| i.key_pressed(Key::Escape)) {
                new_task_name.clear();
                *show_new_task = false;
            }

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    let t = new_task_name.trim();
                    if !t.is_empty() {
                        project.tasks.push(Task {
                            name: t.to_string(),
                            done: false,
                        });
                    }
                    new_task_name.clear();
                    *show_new_task = false;
                }
                if ui.button("Cancel").clicked() {
                    new_task_name.clear();
                    *show_new_task = false;
                }
            });
        });
}

pub fn show_task_edit(
    ctx: &egui::Context,
    edit_task_name: &mut String,
    project: &mut Project,
    editing_task_index: &mut Option<usize>,
    show_task_edit: &mut bool,
) {
    Modal::new(egui::Id::new("edit_task"))
        .backdrop_color(Color32::from_black_alpha(180))
        .frame(Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.label("Edit task name:");
            let resp = ui.text_edit_singleline(edit_task_name);
            resp.request_focus();

            if resp.has_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                if let Some(ti) = editing_task_index {
                    let t = edit_task_name.trim();
                    if !t.is_empty() {
                        project.tasks[*ti].name = t.to_string();
                    }
                }
                *show_task_edit = false;
                *editing_task_index = None;
            }
            if ui.input(|i| i.key_pressed(Key::Escape)) {
                edit_task_name.clear();
                *show_task_edit = false;
            }

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    if let Some(ti) = editing_task_index {
                        let t = edit_task_name.trim();
                        if !t.is_empty() {
                            project.tasks[*ti].name = t.to_string();
                        }
                    }
                    *show_task_edit = false;
                    *editing_task_index = None;
                }
                if ui.button("Cancel").clicked() {
                    *show_task_edit = false;
                    *editing_task_index = None;
                }
            });
        });
}
