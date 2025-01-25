mod home_view;

use home_view::HomeView;

fn main() -> eframe::Result {
    let option = eframe::NativeOptions {
        // viewport: eframe::egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    return eframe::run_native(
        "JsonParser",
        option,
        Box::new(|_| Ok(Box::<HomeView>::default())),
    );
}
