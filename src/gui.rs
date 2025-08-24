use eframe::egui;
use crate::tasks::Task;
use std::fs;
use std::path::Path;

const TASKS_FILE: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    if Path::new(TASKS_FILE).exists() {
        let data = fs::read_to_string(TASKS_FILE).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        let _ = fs::write(TASKS_FILE, json);
    }
}

pub struct TodoApp {
    tasks: Vec<Task>,
    next_id: usize,
    new_task: String,
    edit_id: Option<usize>,
    edit_text: String,
    show_completed: bool,
}

impl Default for TodoApp {
    fn default() -> Self {
        let tasks = load_tasks();
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Self {
            tasks,
            next_id,
            new_task: String::new(),
            edit_id: None,
            edit_text: String::new(),
            show_completed: false,
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw a custom gradient background using the painter
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(
            screen_rect,
            0.0,
            egui::epaint::Color32::from_rgb(110, 60, 180), // deep purple
        );
        painter.rect_filled(
            egui::Rect::from_min_max(
                screen_rect.left_top(),
                egui::pos2(screen_rect.right(), screen_rect.bottom() * 0.6),
            ),
            0.0,
            egui::epaint::Color32::from_rgba_unmultiplied(255, 255, 255, 40), // subtle white overlay
        );

        // Centered panel with rounded corners and shadow
        egui::CentralPanel::default().frame(
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(110, 60, 180)) // solid purple panel
                .rounding(egui::Rounding::same(24.0))
                .shadow(egui::epaint::Shadow::default()),
        ).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);
                ui.heading(
                    egui::RichText::new("📝 To-Do List Manager")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(160, 170, 225)) // almost white
                );
                ui.add_space(16.0);
            });

            // Add Task Section
            ui.horizontal(|ui| {
                let add_task = ui.add(
                    egui::TextEdit::singleline(&mut self.new_task)
                        .hint_text("What needs to be done?")
                ).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Add Task").clicked() || add_task {
                    if !self.new_task.trim().is_empty() {
                        self.tasks.push(Task::new(self.next_id, self.new_task.trim().to_string()));
                        self.next_id += 1;
                        self.new_task.clear();
                        save_tasks(&self.tasks); // <-- Save after adding
                    }
                }
            });

            // Button to open the save file
            if ui.button("Open tasks.json file").clicked() {
                #[cfg(target_os = "linux")]
                {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(TASKS_FILE)
                        .spawn();
                }
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", "start", TASKS_FILE])
                        .spawn();
                }
                #[cfg(target_os = "macos")]
                {
                    let _ = std::process::Command::new("open")
                        .arg(TASKS_FILE)
                        .spawn();
                }
            }

            // Show tasks.json contents in a collapsible section
            ui.add_space(8.0);
            ui.collapsing("Show tasks.json contents", |ui| {
                match std::fs::read_to_string(TASKS_FILE) {
                    Ok(data) => {
                        ui.code(&data);
                    }
                    Err(e) => {
                        ui.label(format!("Could not read file: {}", e));
                    }
                }
            });

            ui.separator();

            // Tabs for Pending and Completed
            ui.horizontal(|ui| {
                if ui.selectable_label(!self.show_completed, "Pending").clicked() {
                    self.show_completed = false;
                }
                if ui.selectable_label(self.show_completed, "Completed").clicked() {
                    self.show_completed = true;
                }
            });

            ui.separator();

            // Filter tasks based on tab
            let tasks_iter = self.tasks.iter_mut()
                .filter(|task| task.completed == self.show_completed);

            for task in tasks_iter {
                ui.horizontal(|ui| {
                    // Complete/Uncomplete Button
                    if ui.button(if task.completed { "Uncomplete" } else { "Complete" }).clicked() {
                        task.completed = !task.completed;
                    }

                    // Edit Mode
                    if self.edit_id == Some(task.id) {
                        if ui.text_edit_singleline(&mut self.edit_text).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            task.description = self.edit_text.clone();
                            self.edit_id = None;
                        }
                        if ui.button("Save").clicked() {
                            task.description = self.edit_text.clone();
                            self.edit_id = None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.edit_id = None;
                        }
                    } else {
                        ui.label(
                            egui::RichText::new(format!("{}: {}", task.id, task.description))
                                .color(egui::Color32::from_rgb(230, 220, 255)) // light lavender
                        );
                        if ui.button("Edit").clicked() {
                            self.edit_id = Some(task.id);
                            self.edit_text = task.description.clone();
                        }
                    }
                });
            }
        });
    }
}