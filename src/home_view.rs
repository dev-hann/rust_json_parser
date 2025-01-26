use std::time::Duration;

use eframe::{
    egui::{self, Frame},
    CreationContext,
};
use egui_notify::Toasts;

#[derive(Default)]
pub struct HomeView {
    input: String,
    error: Option<String>,
    toasts: Toasts,
}

impl HomeView {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            input: r#"{"1234":"1234","a":{"b":{"c":"d"}}}"#.to_string(),
            error: None,
            toasts: Toasts::default(),
        }
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
                        ui.label("Json Input");
                        ui.horizontal_top(|ui| {
                            if ui.button("format").clicked() {
                                let input =
                                    serde_json::from_str::<serde_json::Value>(&self.input).unwrap();
                                let pretty = serde_json::to_string_pretty(&input);
                                match pretty {
                                    Ok(p) => self.input = p,
                                    Err(e) => self.error = Some(e.to_string()),
                                }
                            };
                            if ui.button("copy").clicked() {
                                ui.output_mut(|o| o.copied_text = String::from(&self.input));
                                self.toasts
                                    .success("Copied to clipboard")
                                    .show_progress_bar(false)
                                    .duration(Some(Duration::from_secs(2)));
                            };
                        });
                        ui.text_edit_multiline(&mut self.input);
                    });
                });

                self.toasts.show(ctx);
            });
    }
}
