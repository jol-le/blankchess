use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "blankchess",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

enum DisplayMode {
    Dark,
    Light,
}

struct Settings {
    color_mode: DisplayMode,
}

#[derive(Default)]
struct MyApp {
    settings: Settings;
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Hello from the root viewport");

            ui.checkbox(
                &mut self.show_immediate_viewport,
                "Show immediate child viewport",
            );

            {
                let mut show_deferred_viewport =
                    self.show_deferred_viewport.load(Ordering::Relaxed);
                ui.checkbox(&mut show_deferred_viewport, "Show deferred child viewport");
                self.show_deferred_viewport
                    .store(show_deferred_viewport, Ordering::Relaxed);
            }

            ui.add_space(16.0);
            {
                let mut embedded = ui.embed_viewports();
                ui.checkbox(&mut embedded, "Embed all viewports");
                ui.set_embed_viewports(embedded);
            }
        });

        if self.show_immediate_viewport {
            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Immediate Viewport")
                    .with_inner_size([200.0, 100.0]),
                |ui, class| {
                    if class == egui::ViewportClass::EmbeddedWindow {
                        ui.label(
                            "This viewport is embedded in the parent window, and cannot be moved outside of it.",
                        );
                    } else {
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            ui.label("Hello from immediate viewport");

                            if ui.input(|i| i.viewport().close_requested()) {
                                // Tell parent viewport that we should not show next frame:
                                self.show_immediate_viewport = false;
                            }
                        });
                    }
                },
            );
        }

        if self.show_deferred_viewport.load(Ordering::Relaxed) {
            let show_deferred_viewport = Arc::clone(&self.show_deferred_viewport);
            ui.ctx().show_viewport_deferred(
                egui::ViewportId::from_hash_of("deferred_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Deferred Viewport")
                    .with_inner_size([200.0, 100.0]),
                move |ui, class| {
                    if class == egui::ViewportClass::EmbeddedWindow {
                        ui.label(
                            "This viewport is embedded in the parent window, and cannot be moved outside of it.",
                        );
                    } else {
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            ui.label("Hello from deferred viewport");

                            if ui.input(|i| i.viewport().close_requested()) {
                                // Tell parent to close us.
                                show_deferred_viewport.store(false, Ordering::Relaxed);
                            }
                        });
                    }
                },
            );
        }
    }
}