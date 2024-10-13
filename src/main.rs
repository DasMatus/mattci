use app::UI;
mod app;
mod cfg;
fn main() {
    println!("Hello, world!");
    let mut app = egui_web::AppRunner::new("MattCI", Box::new(UI::default())).unwrap();
    app.warm_up().unwrap();
}
