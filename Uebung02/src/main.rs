use std::fmt::format;
use eframe::{egui, Frame};
use eframe::egui::{CentralPanel, Id, Ui};

fn main() {
    let optiones = eframe::NativeOptions::default();
    eframe::run_native(
        "Uebung02",
        optiones,
        Box::new(|_cc| Ok(Box::new(MyApp::default())))).unwrap();
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ColorChoice {
    Red,
    Green,
    Blue,
    Yellow
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AppMode{
    View,
    Edit,
    Settings
}

pub struct MyApp {
    label: String,
    value: f32,
    show_external_information: bool,
    select_color: ColorChoice,
    counter: i32,
    current_mode: AppMode,
}

impl Default for MyApp {
    fn default() -> Self {
        Self{
            label: "Start Text".to_string(),
            value: 5.0,
            show_external_information: false,
            select_color: ColorChoice::Red,
            counter: 0,
            current_mode: AppMode::View,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top(Id::new("my_top_panel")).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Mode");
                ui.radio_value(&mut self.current_mode, AppMode::View, "View");
                ui.radio_value(&mut self.current_mode, AppMode::Edit, "Edit");
                ui.radio_value(&mut self.current_mode, AppMode::Settings, "Settings");
            })
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!("Current Mode: {:?}", self.current_mode)); // Show the mode
            ui.separator();

            match self.current_mode {
                AppMode::View => {
                    ui.label("View Data");
                    ui.label(format!("Label: {}", self.label));
                    ui.label(format!("Value: {:.5}", self.value));

                    if self.show_external_information {
                        ui.label(format!("Counter: {}", self.counter));
                        ui.label(format!("Color: {:?}", self.select_color));
                    } else {
                        ui.label("Für mehr Informatione Erweiterte Sicht aktivieren");
                    }

                    if ui.button("Switch to Edit Mode").clicked() {
                        self.current_mode = AppMode::Edit;
                    }

                    let button_response = ui.button("Hover or click me");

                    if button_response.hovered() {
                        button_response.clone().on_hover_text("Bitte drücke mich");
                    }
                    if  button_response.clicked() {
                        self.label = "Ich würde gedrückt".to_string();
                    }
                    if button_response.secondary_clicked() {
                        self.label ="Wurde mit rechts geklickt".to_string();
                    }

                }

                AppMode::Edit => {
                    ui.label("Edit Data");

                    egui::Grid::new("edit_grid").num_columns(2).spacing([10.0,4.0]).show(ui, |ui|{
                        ui.label("Label:");
                        ui.text_edit_singleline(& mut self.label);

                        ui.label("Value:");
                        ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0));

                        ui.label("Counter");
                        ui.horizontal(|ui|{
                            if ui.button("+").clicked(){
                                self.counter += 1
                            }
                            if ui.button("-").clicked(){
                                self.counter -= 1
                            }
                            ui.label(format!("Counter: {}", self.counter))
                        });
                        ui.end_row();
                    });

                    let sidebar_response = ui
                        .add(egui::Slider::new(&mut self.value, 0.0..=10.0)
                            .text("Sidebar mit änderbaren Werten"));

                    if sidebar_response.dragged(){
                        self.label ="Slider Bewegt".to_string();
                    }

                    if sidebar_response.changed(){
                        println!("Slider Wert geändert")
                    }
                }
                AppMode::Settings => {
                    ui.label("Application Setting");
                    ui.separator();

                    ui.checkbox(&mut self.show_external_information, "Show Advance Information");

                    ui.separator();

                    ui.label("Color Schema");
                    ui.horizontal(|ui|{
                        ui.radio_value(&mut self.select_color, ColorChoice::Red, "red");
                        ui.radio_value(&mut self.select_color, ColorChoice::Green, "green");
                        ui.radio_value(&mut self.select_color, ColorChoice::Blue, "blue");
                        ui.radio_value(&mut self.select_color, ColorChoice::Yellow, "yellow");
                    });
                    ui.label(format!("Selected Colore: {:?}", self.select_color));

                    ui.separator();

                    if ui.button("Reset").clicked(){
                        *self = MyApp::default();
                        self.current_mode = AppMode::Settings;
                    }
                }
            }
        });
    }
}