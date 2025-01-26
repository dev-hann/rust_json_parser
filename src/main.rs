mod home_view;
use eframe::egui;
use eframe::Result;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "JSON Parser App",
        options,
        Box::new(|c| Ok(Box::new(home_view::HomeView::new(c)))),
    )
}
