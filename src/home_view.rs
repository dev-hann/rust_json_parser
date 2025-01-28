use std::time::Duration;

use eframe::{
    egui::{self, Frame, Widget},
    CreationContext,
};
use egui_notify::{Toast, Toasts};

#[derive(Default)]
pub struct HomeView {
    input: String,
    toasts: Toasts,
}

impl HomeView {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            input: r#"{"1234":"1234","a":{"b":{"c":"d"}}}"#.to_string(),
            toasts: Toasts::default(),
        }
    }
    fn show_toast(&mut self, message: &str) -> &mut Toast {
        self.toasts
            .success(message)
            .show_progress_bar(false)
            .duration(Some(Duration::from_secs(2)))
    }
    fn buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            if ui.button("format").clicked() {
                let input = serde_json::from_str::<serde_json::Value>(&self.input);
                match input {
                    Ok(obj) => {
                        let pretty = serde_json::to_string_pretty(&obj);
                        match pretty {
                            Ok(p) => self.input = p,
                            Err(e) => {
                                self.show_toast(&e.to_string());
                            }
                        }
                    }
                    Err(e) => {
                        self.show_toast(&e.to_string());
                    }
                }
            }
            if ui.button("copy").clicked() {
                ui.output_mut(|o| o.copied_text = String::from(&self.input));
                self.show_toast("Copied to clipboard");
            }
        });
    }
    fn input(&mut self, ui: &mut egui::Ui) {
        let mut numbered_text = String::new();
        let lines = self.input.lines().count();
        for line_number in 1..=lines {
            numbered_text.push_str(&format!("{:<3} {}\n", line_number, ""));
        }
        ui.label(egui::RichText::new(numbered_text));

        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::multiline(&mut self.input)
                .code_editor()
                .desired_width(f32::INFINITY),
        );
    }
}

impl eframe::App for HomeView {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().outer_margin(egui::Margin::same(16.0)))
            .show(ctx, |ui| {
                ui.heading("Json Parser");
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal_top(|ui| {
                            egui::Frame::none()
                                .outer_margin(egui::Margin::same(8.0))
                                .show(ui, |ui| {
                                    self.buttons(ui);
                                });
                        });
                        ui.horizontal_top(|ui| {
                            self.input(ui);
                        });
                    });
                });

                self.toasts.show(ctx);
            });
    }
}
