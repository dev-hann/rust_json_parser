use eframe::{egui, App};

pub struct HomeView {
    json_input: String,
}

impl Default for HomeView {
    fn default() -> Self {
        return Self {
            json_input: "".to_string(),
        };
    }
}

impl App for HomeView {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
        let available_size = ui.available_size();
        let width = available_size.x * 0.5;
        let height = available_size.y;
            ui.heading("Json Parser");
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::Vec2::new(width, height), |ui| {
                    ui.vertical(|ui| {
                        ui.label("Json Input");
                        ui.add_sized([width, height], egui::TextEdit::multiline(&mut self.json_input));
                    });
                });
                ui.allocate_space(egui::Vec2::new(16.0, 16.0));
                ui.allocate_ui(egui::Vec2::new(width, height), |ui| {
                    ui.vertical(|ui| {
                        ui.label("Json Output");
                        ui.selectable_label(false, self.json_input.clone());
                    });
                });
            });
        });
    }
}
