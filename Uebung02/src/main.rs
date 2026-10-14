use eframe::egui::{CentralPanel, Id, IntoAtoms, Ui};
use eframe::{Frame, egui};
use std::fmt::format;

fn main() {
    let optiones = eframe::NativeOptions::default();
    eframe::run_native(
        "Uebung02",
        optiones,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
    .unwrap();
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ColorChoice {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AppMode {
    View,
    Edit,
    Settings,
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
        Self {
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
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.render_menu_bar(ui, frame);
        self.render_side_panel(ui, frame);
        self.render_centreal_panel(ui, frame);
    }
}

impl MyApp {
    pub fn render_menu_bar(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Reset Counter").clicked() {
                        self.counter = 0;
                    }
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Mode", |ui| {
                    if ui
                        .radio_value(&mut self.current_mode, AppMode::View, "View")
                        .clicked()
                    {
                        ui.close();
                    }

                    if ui
                        .radio_value(&mut self.current_mode, AppMode::Settings, "Settings")
                        .clicked()
                    {
                        ui.close();
                    }

                    if ui
                        .radio_value(&mut self.current_mode, AppMode::Edit, "Edit")
                        .clicked()
                    {
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Counter: {}", self.counter));
                })
            });
        });
    }

    pub fn render_side_panel(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("infp_panel").show(ui, |ui| {
            ui.heading("Info");
            ui.label("This panel can hold help text or tool buttons.");
            ui.separator();

            ui.label(format!("Selected Color: {:?}", self.select_color));
            if ui.button("Reset Label").clicked() {
                self.label = String::default();
            }
        });
    }

    pub fn render_centreal_panel(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!("Mode: {:?}", self.current_mode));
            ui.separator();

            match self.current_mode{
                AppMode::View => {
                    ui.vertical_centered(|ui|{
                        ui.heading("VIEW DATA")
                    });
                    ui.label(format!("Label: {}", self.label));
                    ui.label(format!("Value: {}", self.value));

                    ui.collapsing("show more", |ui|{
                        ui.label(format!("Counter: {}", self.counter));
                        ui.label(format!("Color: {:?}", self.select_color));
                        ui.label(format!("Show Extra Info Setting: {}", self.show_external_information));
                    });
                }
                AppMode::Edit => {
                    ui.heading("EDIT DATA");

                    egui::Grid::new("edit_form_grid")
                        .num_columns(2)
                        .spacing({[20.0, 8.0]})
                        .striped(true)
                        .show(ui, |ui|{
                            ui.label("Edit Label:"); // Col 1
                            ui.text_edit_singleline(&mut self.label); // Col 2
                            ui.end_row();

                            ui.label("Adjust Value:"); // Col 1
                            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0)); // Col 2
                            ui.end_row();

                            ui.label("Counter:"); // Col 1
                            ui.horizontal(|ui|{ // Nested layout in grid cell
                                if ui.button("+").clicked() { self.counter += 1; }
                                ui.label(format!("{}", self.counter));
                                if ui.button("-").clicked() { self.counter -= 1; }
                            }); // Col 2
                            ui.end_row();
                        });
                }
                AppMode::Settings => {
                    ui.heading("Settings");

                    ui.group(|ui|{
                        ui.checkbox(&mut self.show_external_information, "Show Advanced Info in View");
                        ui.separator();
                        ui.label("Color Scheme:");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.select_color, ColorChoice::Red, "Red");
                            ui.radio_value(&mut self.select_color, ColorChoice::Green, "Green");
                            ui.radio_value(&mut self.select_color, ColorChoice::Blue, "Blue");
                        });
                    }); // End group

                    ui.separator();
                    if ui.button("Reset All State").clicked() {
                        *self = MyApp::default();
                        self.current_mode = AppMode::Settings; // Stay in settings after reset
                    }
                }
            }
        });
    }
}
