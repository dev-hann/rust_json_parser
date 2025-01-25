mod home_view;

use home_view::HomeView;

fn main() -> eframe::Result {
    return eframe::run_native(
        "JsonParser",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::<HomeView>::default())),
    );
}
