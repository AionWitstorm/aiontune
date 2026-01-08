use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, FileChooserDialog, HeaderBar, Label,
    ListBox, Orientation, ResponseType,
};
use std::fs;
use std::path::PathBuf;

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
        header.pack_end(&open_button); // right side

        // Add header to main box
        vbox.append(&header);

        // Create playlist ListBox
        let playlist_box = ListBox::new();
        vbox.append(&playlist_box);

        // Add main box to window
        window.set_child(Some(&vbox));

        // ----------------------------
        // Open Folder button logic
        // ----------------------------
        let window_clone = window.clone();
        let playlist_box_clone = playlist_box.clone();
        open_button.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(
                Some("Select Music Folder"),
                Some(&window_clone),
                gtk4::FileChooserAction::SelectFolder,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Open", ResponseType::Accept),
                ],
            );

            let playlist_box_clone_inner = playlist_box_clone.clone();
            dialog.connect_response(move |dialog, response| {
                let playlist_box_clone = playlist_box_clone_inner.clone(); // clone for this closure
                if response == ResponseType::Accept {
                    if let Some(folder) = dialog.file() {
                        if let Some(folder_path) = folder.path() {
                            println!("Selected folder: {:?}", folder_path);

                            // Clear previous playlist
                            while let Some(row) = playlist_box_clone.first_child() {
                                playlist_box_clone.remove(&row);
                            }

                            // Scan folder for audio files
                            if let Ok(entries) = fs::read_dir(&folder_path) {
                                for entry in entries.flatten() {
                                    let path: PathBuf = entry.path();
                                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                        if ["mp3", "flac", "wav", "ogg"].contains(&ext) {
                                            let filename =
                                                path.file_name().unwrap().to_string_lossy();
                                            let row = gtk4::ListBoxRow::new();
                                            let label = gtk4::Label::new(Some(&filename));
                                            row.set_child(Some(&label));
                                            playlist_box_clone.append(&row);
                                        }
                                    }
                                }
                            }
                        }
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
