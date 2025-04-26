use egui::{CentralPanel, SidePanel};

use crate::Project;

pub fn left_panel(
    ctx: &egui::Context,
    new_project_name: &mut String,
    show_new_project: &mut bool,
    selected_project: &mut Option<usize>,
    project_edit_index: &mut Option<usize>,
    project_edit_name: &mut String,
    show_project_edit: &mut bool,
    projects: &mut Vec<Project>,
    to_delete_project: &mut Option<usize>,
) {
    SidePanel::left("left_panel")
        .resizable(true)
        .default_width(500.0)
        .width_range(200.0..=700.0)
        .show(ctx, |ui| {
            ui.heading("Projects");
            ui.add_space(4.0);

            // Add Project button
            if ui.button("Add Project").clicked() {
                new_project_name.clear();
                *show_new_project = true;
            }
            ui.separator();

            for (i, project) in projects.iter().enumerate() {
                ui.horizontal(|ui| {
                    // selectable project name
                    let sel = *selected_project == Some(i);
                    if ui.selectable_label(sel, &project.name).clicked() {
                        *selected_project = Some(i);
                    }

                    // edit project
                    if ui.small_button("✒").clicked() {
                        *project_edit_index = Some(i);
                        *project_edit_name = project.name.clone();
                        *show_project_edit = true;
                    }

                    // delete project
                    if ui.small_button("🗑").clicked() {
                        *to_delete_project = Some(i);
                    }
                });
            }
        });
}

pub fn central_panel(
    ctx: &egui::Context,
    project: &mut Project,
    hide_completed: &mut bool,
    filter_text: &mut String,
    editing_task_index: &mut Option<usize>,
    edit_task_name: &mut String,
    to_delete_task: &mut Option<usize>,
    new_task_name: &mut String,
    show_task_edit: &mut bool,
    show_new_task: &mut bool,
) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading(&project.name);
        ui.add_space(4.0);

        // controls for filters
        ui.horizontal(|ui| {
            ui.checkbox(hide_completed, "Hide completed");
            ui.label("Search:");
            ui.text_edit_singleline(filter_text);
        });
        ui.separator();

        for (ti, task) in project.tasks.iter_mut().enumerate() {
            // hide completed
            if *hide_completed && task.done {
                continue;
            }
            // filter
            let needle = filter_text.to_lowercase();
            if !needle.is_empty() && !task.name.to_lowercase().contains(&needle) {
                continue;
            }

            // draw rows
            ui.horizontal(|ui| {
                ui.checkbox(&mut task.done, "");
                ui.label(&task.name);

                if ui.small_button("✒").clicked() {
                    *editing_task_index = Some(ti);
                    *edit_task_name = task.name.clone();
                    *show_task_edit = true;
                }
                if ui.small_button("🗑").clicked() {
                    *to_delete_task = Some(ti);
                }
            });
        }

        ui.add_space(8.0);
        if ui.button("Add Task").clicked() {
            new_task_name.clear();
            *show_new_task = true;
        }
    });
}
