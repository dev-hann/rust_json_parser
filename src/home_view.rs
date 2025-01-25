use eframe::{egui::{self}, App};

pub struct HomeView {
    input: String,
    output:String,
}

impl Default for HomeView {
    fn default() -> Self {
        return Self {
            input: r#"{"1234":"1234","a":{"b":{"c":"d"}}}"#.to_string(),
            output: "".to_string(),
        };
    }
}

impl App for HomeView {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let input = serde_json::from_str::<serde_json::Value>(&self.input);
        self.output = serde_json::to_string_pretty(&input.unwrap()).unwrap();
        
        egui::CentralPanel::default().show(ctx, |ui| {
        let available_size = ui.available_size();
        let width = available_size.x * 0.5;
        let height = available_size.y*0.9;
            ui.heading("Json Parser");
            ui.horizontal_top(|ui| {
                ui.allocate_ui(egui::Vec2::new(width, height), |ui| {
                    ui.vertical(|ui| {
                        ui.label("Json Input");
                        ui.add_sized([width, height], egui::TextEdit::multiline(&mut self.input));
                    });
                });
                ui.allocate_space(egui::Vec2::new(16.0, 16.0));
                ui.allocate_ui(egui::Vec2::new(width, height), |ui| {
                    ui.vertical(|ui| {
                        ui.label("Json Output");
                        ui.add_sized([width, height], egui::TextEdit::multiline(&mut self.output).interactive(false));
                    });
                });
            });
        });
    }
}

