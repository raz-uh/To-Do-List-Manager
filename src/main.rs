mod tasks;
mod gui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do List Manager",
        options,
        Box::new(|_cc| Box::new(gui::TodoApp::default())),
    )
}