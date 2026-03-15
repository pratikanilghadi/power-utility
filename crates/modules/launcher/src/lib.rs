pub mod search;

pub mod intialise_frame {
    use std::{thread, time::Duration};

    use crate::search::search;

    #[derive(Default)]
    struct Launcher {
        had_focus: bool,
        input_text: String,
    }

    impl eframe::App for Launcher {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            let has_focus = ctx.input(|i| i.focused);

            // If we had focus but lost it, close the window
            if self.had_focus && !has_focus {
                println!("Focus lost - closing window");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            self.had_focus = has_focus;

            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.input_text)
                        .desired_width(790.0)
                        .font(egui::FontId::proportional(60.0))
                        .desired_rows(1),
                );
                response.request_focus();
            });

            let result = search::pattern_match(&self.input_text);

            match (result) {
                search::Pattern::Application => {
                    println!("Application");
                }
                search::Pattern::Equation => {
                    println!("Equation");
                }

                _ => {
                    print!("Google Search");
                }
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn create_frame(options: eframe::NativeOptions) {
        eframe::run_native(
            "Launcher",
            options,
            Box::new(|_cc| Ok(Box::new(Launcher::default()))),
        );
    }
}
