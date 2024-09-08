use chrono::{Date, DateTime, Utc};
use egui::{vec2, Sense, Separator, Shadow, Ui, Vec2};
use todolist::db;

#[derive(PartialEq, Debug, Default, Clone, PartialOrd)]
enum TaskPriority {
    #[default]
    Low,
    Medium,
    High
}

#[derive(Default, Debug, Clone)]
pub struct Task {
    id: i32,
    done: bool,
    title: String,
    description: String,
    deadline: DateTime<Utc>,
    deadline_string : String,
    priority: TaskPriority
}

#[derive(Default)]
pub struct Todolist {
    tasks: Vec<Task>,
    pub new_task: Task,
    errors: Vec<&'static str>,
    pub sort_by: String,
}

impl Todolist {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self{
            tasks: Todolist::fetch_tasks(),
            ..Default::default()
        }
    }

    pub fn fetch_tasks() -> Vec<Task> {
        let query_tasks = db::fetch_all(&mut db::establish_connection());
        query_tasks.iter().map(|task| {
            let date = format!("20{}:00 UTC", task.deadline);
            let deadline = date.parse::<DateTime<Utc>>().unwrap(); 
            Task{
                id: task.id,
                done: task.done,
                title: task.title.clone(),
                description: task.description.clone(),
                deadline,
                deadline_string : task.deadline.clone(),
                priority: match task.priority {
                    1 => TaskPriority::Low,
                    2 => TaskPriority::Medium,
                    _ => TaskPriority::High
                }
            }
        }).collect()
    }

    pub fn render_tasks(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context){
        if self.sort_by == "Priority" {
            self.tasks.sort_by(|a, b| {b.priority.partial_cmp(&a.priority).unwrap()});
        }
        else if self.sort_by == "Deadline" {
            self.tasks.sort_by(|a, b| {a.deadline.partial_cmp(&b.deadline).unwrap()});
        }
        for task in self.tasks.iter_mut(){
            if !task.done {
                render_task(ui, ctx, task);
                ui.add_space(10.);
            }
        }
    }

    pub fn render_modal(&mut self, modal: &egui_modal::Modal, ctx: &egui::Context){
        modal.show(|ui| {
        ui.label("Task Title:");
        ui.add(egui::TextEdit::singleline(&mut self.new_task.title));
        ui.label("Task Description:");
        ui.add_sized(vec2(300.,200.), egui::TextEdit::multiline(&mut self.new_task.description));
        ui.label("Priority:");
        let mut priority = &mut self.new_task.priority;
        egui::ComboBox::from_id_source("priority")
            .selected_text(format!("{:?}", priority))
            .show_ui(ui, |ui| {
                ui.selectable_value(priority, TaskPriority::Low, "Low");
                ui.selectable_value(priority, TaskPriority::Medium, "Medium");
                ui.selectable_value(priority, TaskPriority::High, "High");
            });
            
        ui.label("Deadline: (yy-mm-dd hh:mm)");
        ui.add(egui::TextEdit::singleline(&mut self.new_task.deadline_string));
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {

            if ui.button("Close").clicked(){
                modal.close();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let create_button = ui.button("Create");
                if create_button.clicked(){
                    check_new_task(&mut self.new_task, &mut self.errors);
                    if self.errors.is_empty(){
                        self.tasks.push(self.new_task.clone());
                        self.new_task = Task::default();
                        println!("{:?}", self.tasks);
                        modal.close();
                    }
                }

            });
        });
        if !self.errors.is_empty(){
            for &error in self.errors.iter() {
                ui.label(error);
            }
        }
    });
}
}

fn check_new_task (new_task: &mut Task, errors: &mut Vec<&'static str>) {
    errors.clear();
    let pp = |x: i32, y: i32| {x + y};
    if new_task.title.is_empty(){
        errors.push("Task title should be specified.");
    }

    if new_task.description.is_empty(){
        errors.push("Task description should not be empty.");
    }
    
    let date = format!("20{}:00 UTC", new_task.deadline_string);

    // TODO: check if the time has already passed
    match date.parse::<DateTime<Utc>>(){
        Err(_) => errors.push("Wrong deadline format."),
        Ok(date) => new_task.deadline = date
    }
}

fn render_task(ui: &mut Ui, ctx: &eframe::egui::Context, task: &mut Task) {
    let mut shadow = Shadow::default();
    shadow.offset = vec2(0.0, 20.0);
    shadow.color = egui::Color32::from_hex("#00000022").unwrap();
    shadow.blur = 20.0;
    egui::Frame::default()
        .inner_margin(8.0)
        .stroke((0.5, egui::Color32::WHITE))
        .shadow(shadow)
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label(&task.title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    match task.priority {
                        TaskPriority::Low => ui.add(
                            egui::Image::new(egui::include_image!("../assets/low_priority.png"))
                                .max_width(20.0)
                                .rounding(10.0),
                        ),
                        TaskPriority::Medium => ui.add(
                            egui::Image::new(egui::include_image!("../assets/medium_priority.png"))
                                .max_width(20.0)
                                .rounding(10.0),
                        ),
                        TaskPriority::High => ui.add(
                            egui::Image::new(egui::include_image!("../assets/high_priority.png"))
                                .max_width(20.0)
                                .rounding(10.0),
                        ),
                    };
                    
                    ui.label(format!("Deadline: {}", &task.deadline));
                });
            });
            ui.label(&task.description);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let mark_done = ui.button("Mark as done");
                // TODO: incorporate Delete task button. 
                // let delete_task = ui.button("Delete task");

                if mark_done.clicked() {
                    task.done = true;
                }
            });
        });
}
