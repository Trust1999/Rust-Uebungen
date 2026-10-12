use eframe::{egui, Frame};
use eframe::egui::Ui;

fn main() {
    let optiones = eframe::NativeOptions::default();
    eframe::run_native(
        "Uebung02",
        optiones,
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc))))).unwrap();
}


#[derive(Default)]
pub struct MyApp {
    label: String,
    value: f32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Meine erste Applikation");
            ui.horizontal(|ui| {
                ui.label("Text");
                ui.text_edit_singleline(&mut self.label);
            });
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("Wert"));
            if ui.button("Eröhen").clicked(){
                self.value = self.value + 0.1;
            }
            ui.label(format!("Hallo {}, der Wert ist {}", self.label, self.value));
        });
    }
}