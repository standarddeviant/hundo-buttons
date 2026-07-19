use crate::phrases::{crowded_phrase, lonely_phrase};
use crate::random_color::random_pastel_color;
use crate::toggle_button_compact;
use egui::Color32;
use egui::UiKind::ScrollArea;
// use eframe::{Align2, Color32, Direction, Frame, Pos2, Ui, Widget};
use egui_toast::Toasts;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    hundo_buttons: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    color_list: Vec<Color32>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    message: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            hundo_buttons: false,
            color_list: vec![],
            message: "hmm..".to_owned(),
            // Example stuff:
            // label: "Hello World!".to_owned(),
            // value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let mut toasts = Toasts::new();
        // .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
        // .direction(egui::Direction::BottomUp);

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("hundo-buttons");
            ui.label("A fun example of what an immediate mode GUI is capable of.");

            ui.separator();
            ui.label("Would you like one hundred buttons?");

            let prev_hundo: bool = self.hundo_buttons;
            toggle_button_compact(ui, &mut self.hundo_buttons);
            if !prev_hundo && self.hundo_buttons {
                self.color_list = (0..100).map(|_| random_pastel_color()).collect();
            }

            ui.separator();
            ui.heading(self.message.clone());

            if self.hundo_buttons {
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("my_grid")
                        .striped(false) // Adds a subtle background color to alternate rows
                        // .col_width(100.0)
                        .min_col_width(80.0)
                        .show(ui, |ui| {
                            let mut count = 0;
                            for _row in 0..10 {
                                for _col in 0..10 {
                                    if ui
                                        .add(
                                            egui::Button::new(format!("Button {}", count + 1))
                                                .fill(self.color_list[count])
                                                .stroke(egui::Stroke::new(
                                                    2.0,
                                                    egui::Color32::BLACK,
                                                )),
                                        )
                                        .clicked()
                                    {
                                        self.message =
                                            format!("Button {} says: {}", count, crowded_phrase());
                                        // hmmm, what to do?
                                    }
                                    count += 1;
                                }
                                ui.end_row();
                            }
                        });
                });
            } else {
                if ui.button("Just a solitary button..").clicked() {
                    self.message = format!("A lonely button says: {}", lonely_phrase());
                }
            }
        });

        egui::Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/standarddeviant/hundo-buttons/blob/main/",
                "Source code."
            ));
        });

        //     // The top panel is often a good place for a menu bar:
        //     egui::MenuBar::new().ui(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ui.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }
        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });
        // });
    }
}
