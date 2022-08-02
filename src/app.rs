use std::collections::HashSet;

use crate::task::Task;

const TASK_MARGIN: f32 = 16.0;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tasks: Vec<Task>,
    #[serde(skip)]
    tasks_to_delete: HashSet<usize>,
    #[serde(skip)]
    curr_task_input: String,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn ui_task_list(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for task_idx in 0..self.tasks.len() {
                self.ui_task(ui, task_idx);
                ui.add_space(TASK_MARGIN);
            }
        });
    }

    fn ui_task(&mut self, ui: &mut egui::Ui, task_idx: usize) {
        ui.style_mut().spacing.item_spacing = egui::vec2(16.0, 0.0);
        let task = &mut self.tasks[task_idx];
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    ui.label(&task.name);
                });
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("Delete").clicked() {
                        self.tasks_to_delete.insert(task_idx);
                    };
                    ui.checkbox(&mut task.is_done, "Completed");
                });
            });
        });
    }

    fn ui_task_add_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Enter Task: ");
                let on_submit = |app: &mut TemplateApp| {
                    app.add_task();
                    app.curr_task_input.clear();
                };
                let lost_focus = ui
                    .text_edit_singleline(&mut self.curr_task_input)
                    .lost_focus();
                if lost_focus && ui.input().key_pressed(egui::Key::Enter) {
                    on_submit(self);
                };
                if ui.button("Add").clicked() {
                    on_submit(self)
                }
            });
        });
    }

    fn delete_tasks(&mut self) {
        self.tasks = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(idx, _)| !self.tasks_to_delete.contains(idx))
            .map(|(_, task)| task.clone())
            .collect();
        self.tasks_to_delete.clear();
    }

    fn add_task(&mut self) {
        self.tasks.push(Task::new(&self.curr_task_input));
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.delete_tasks();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Egui Todolist");
            });
            ui.add_space(32.0);
            self.ui_task_add_section(ui);
            ui.add_space(32.0);
            self.ui_task_list(ui);
        });
    }
}
