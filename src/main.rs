use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, HeaderBar, Button, Box as GtkBox, Orientation, Label, FileChooserDialog, ResponseType};

fn main() {
    let app = Application::builder()
        .application_id("com.aion.aiontune")
        .build();

    app.connect_activate(|app| {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("AionTune")
            .default_width(800)
            .default_height(500)
            .build();

        // Create vertical layout box
        let vbox = GtkBox::new(Orientation::Vertical, 5);

        // Create HeaderBar
        let header = HeaderBar::builder()
            .title_widget(&Label::new(Some("AionTune")))
            .show_title_buttons(true)
            .build();

        // Create Play button
        let play_button = Button::with_label("Play");

        // Create Open Folder button
        let open_button = Button::with_label("Open Folder");

        // Add buttons to header
        header.pack_start(&play_button); // left side
        header.pack_end(&open_button);   // right side

        // Add header to main box
        vbox.append(&header);

        // Add main box to window
        window.set_child(Some(&vbox));

        // ----------------------------
        // Open Folder button logic
        // ----------------------------
       let window_clone = window.clone();
open_button.connect_clicked(move |_| {
    let dialog = FileChooserDialog::new(
        Some("Select Music Folder"),
        Some(&window_clone),
        gtk4::FileChooserAction::SelectFolder,
        &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
    );

    dialog.connect_response(|dialog, response| {
        if response == ResponseType::Accept {
            if let Some(folder) = dialog.file() {
                println!("Selected folder: {:?}", folder.path());
                // TODO: Load music files from this folder
            }
        }
        dialog.close();
    });

    dialog.show();
});


        // Show window
        window.present();
    });

    app.run();
}
