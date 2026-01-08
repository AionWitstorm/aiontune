use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.aion.aiontune")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("AionTune")
            .default_width(800)
            .default_height(500)
            .build();

        window.present();
    });

    app.run();
}
