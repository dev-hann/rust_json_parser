mod home_view;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "JSON Parser App",
        options,
        Box::new(|c| Ok(Box::new(home_view::HomeView::new(c)))),
    )
}
