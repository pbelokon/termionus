use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        run_and_return: true,
        initial_window_size: Some(egui::vec2(640.0, 288.0)),
        ..Default::default()
    };

    eprintln!("Starting first window…");
    eframe::run_native(
        "Termionus",
        options.clone(),
        Box::new(|_cc| Box::new(MyApp { has_next: false })),
    )
}

struct MyApp {
    pub(crate) has_next: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let label_text = "Hello";
            ui.label(label_text);
            
        });
    }
}
