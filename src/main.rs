use egui::{ScrollArea, TextStyle::*, TopBottomPanel};
use eframe::{run_native, App, NativeOptions};
use egui::{vec2, FontDefinitions, FontFamily, Label, Separator, Ui};

use egui::FontId;
mod todolist;

impl App for todolist::Todolist{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_header(ui, &mut self.sort_by);
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_tasks(ui, ctx);
                ui.add_space(80.);
            });
            render_footer(ctx, self);
         });
    }
}

fn render_header(ui: &mut Ui, sorter: &mut String){
    ui.vertical_centered(|ui| {
        ui.heading("Todolist");
    });
    ui.add_space(20.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        egui::ComboBox::from_id_source("sorter")
            .selected_text(format!("{}", sorter))
            .show_ui(ui, |ui| {
                ui.selectable_value(sorter, "Priority".to_string(), "Priority");
                ui.selectable_value(sorter, "Deadline".to_string(), "Deadline");
            });
        ui.label("Sort by: ");
    });

    ui.add(Separator::default().spacing(20.0));
}

fn render_footer(ctx: &eframe::egui::Context, todolist: &mut todolist::Todolist){
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.add_space(20.0); 
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            let modal = egui_modal::Modal::new(ctx, "task_creation");
            todolist.render_modal(&modal, ctx);
            let create_task = ui.button("Create Task");
            if create_task.clicked(){
                modal.open();
            }
        });
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.add_space(20.0); 
        });
    });
}

fn setup_fonts(ctx: &egui::Context){
    let mut font_def = FontDefinitions::default();
    let mut style = (*ctx.style()).clone();
    
    // Redefine text_styles
    style.text_styles = [
        (Heading, FontId::new(30.0, FontFamily::Proportional)),
        (Name("Heading2".into()), FontId::new(25.0, FontFamily::Proportional)),
        (Name("Context".into()), FontId::new(23.0, FontFamily::Proportional)),
        (Body, FontId::new(18.0, FontFamily::Proportional)),
        (Monospace, FontId::new(14.0, FontFamily::Proportional)),
        (Button, FontId::new(14.0, FontFamily::Proportional)),
        (Small, FontId::new(10.0, FontFamily::Proportional)),
    ].into();

    font_def.font_data.insert(
        "MesloLGS".to_string(),
        egui::FontData::from_static(include_bytes!("../MesloLGS_NF_Regular.ttf")),
    );

    font_def.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "MesloLGS".to_owned());

    ctx.set_fonts(font_def);
    ctx.set_style(style);
}

fn main() -> eframe::Result {
    let mut window_options = NativeOptions::default();
    window_options.viewport.inner_size = Some(vec2(540.0,960.0));
    run_native(
        "Todo List",
        window_options, 
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(todolist::Todolist::new(cc)))
        }))
}
