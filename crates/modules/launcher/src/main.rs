use eframe;
use launcher::intialise_frame::create_frame;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Launcher")
            .with_inner_size([800.0, 95.0])
            .with_decorations(false)
            .with_active(true)
            .with_fullscreen(false),
        centered: true,
        ..Default::default()
    };

    create_frame(options);
}
