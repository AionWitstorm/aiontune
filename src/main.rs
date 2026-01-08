use gstreamer as gst;
use gstreamer::prelude::*; // brings in set_state, set_property, etc.
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, FileChooserDialog, HeaderBar, Label,
    ListBox, ListBoxRow, Orientation, ResponseType,
};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

fn main() {
    // Initialize GTK
    gtk4::init().expect("Failed to initialize GTK");
    // Initialize GStreamer
    gst::init().expect("Failed to initialize GStreamer");

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

        // Vertical layout
        let vbox = GtkBox::new(Orientation::Vertical, 5);

        // HeaderBar
        let header = HeaderBar::builder()
            .title_widget(&Label::new(Some("AionTune")))
            .show_title_buttons(true)
            .build();

        // Play and Open Folder buttons
        let play_button = Button::with_label("Play");
        let pause_button = Button::with_label("Pause");
        let open_button = Button::with_label("Open Folder");

        header.pack_start(&play_button);
        header.pack_start(&pause_button);
        header.pack_end(&open_button);
        vbox.append(&header);

        // Playlist ListBox
        let playlist_box = ListBox::new();
        vbox.append(&playlist_box);

        // Add main box to window
        window.set_child(Some(&vbox));

        // ----------------------------
        // Shared state: playlist folder path
        // ----------------------------
        let current_folder: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

        // ----------------------------
        // Playlist storage: Vec<PathBuf>
        // ----------------------------
        let playlist_paths: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));
        let is_playing = Rc::new(RefCell::new(false));

        // ----------------------------
        // GStreamer player
        // ----------------------------
        let player = gst::ElementFactory::make("playbin")
            .build()
            .expect("Failed to create playbin element");

        // ----------------------------
        // Open Folder button logic
        // ----------------------------
        let window_clone = window.clone();
        let playlist_box_clone = playlist_box.clone();
        let current_folder_clone = current_folder.clone();
        let playlist_paths_clone = playlist_paths.clone();

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

            let playlist_box_inner = playlist_box_clone.clone();
            let current_folder_inner = current_folder_clone.clone();
            let playlist_paths_inner = playlist_paths_clone.clone();

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(folder) = dialog.file() {
                        if let Some(folder_path) = folder.path() {
                            println!("Selected folder: {:?}", folder_path);

                            // Update shared folder
                            *current_folder_inner.borrow_mut() = Some(folder_path.clone());

                            // Clear previous playlist UI
                            while let Some(row) = playlist_box_inner.first_child() {
                                playlist_box_inner.remove(&row);
                            }

                            // Clear previous paths
                            playlist_paths_inner.borrow_mut().clear();

                            // Scan folder for audio files
                            if let Ok(entries) = fs::read_dir(&folder_path) {
                                for entry in entries.flatten() {
                                    let path: PathBuf = entry.path();
                                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                        if ["mp3", "flac", "wav", "ogg"].contains(&ext) {
                                            let filename =
                                                path.file_name().unwrap().to_string_lossy();

                                            // Add to playlist UI
                                            let row = ListBoxRow::new();
                                            let label = Label::new(Some(&filename));
                                            row.set_child(Some(&label));
                                            playlist_box_inner.append(&row);

                                            // Add to playlist paths
                                            playlist_paths_inner.borrow_mut().push(path);
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

        // ----------------------------
        // Play button logic: play first song or selected song
        // ----------------------------
        let playlist_box_clone2 = playlist_box.clone();
        let playlist_paths_clone2 = playlist_paths.clone();
        let player_clone = player.clone();
        let is_playing_clone = is_playing.clone();
        let pause_button_clone = pause_button.clone();

        play_button.connect_clicked(move |_| {
            // Get selected row or first row
            let selected_row = playlist_box_clone2
                .selected_row()
                .and_then(|w| w.downcast::<gtk4::ListBoxRow>().ok())
                .or_else(|| {
                    playlist_box_clone2
                        .first_child()
                        .and_then(|w| w.downcast::<gtk4::ListBoxRow>().ok())
                });

            if let Some(row) = selected_row {
                let index = row.index(); // index is i32

                if let Some(song_path) = playlist_paths_clone2.borrow().get(index as usize) {
                    let uri = format!("file://{}", song_path.display());
                    println!("Playing: {}", uri);
                    player_clone.set_state(gst::State::Null).unwrap();
                    player_clone.set_property("uri", &uri); // returns ()
                    player_clone.set_state(gst::State::Playing).unwrap(); // .unwrap() still needed here

                    *is_playing_clone.borrow_mut() = true;
                    pause_button_clone.set_label("Pause");
                }
            }
        });

        // ----------------------------
        // Playlist click logic: play clicked song
        // ----------------------------
        let playlist_paths_clone3 = playlist_paths.clone();
        let player_clone2 = player.clone();
        let is_playing_clone2 = is_playing.clone();
        let pause_button_clone2 = pause_button.clone();

        playlist_box.connect_row_activated(move |_, row| {
            let playlist_paths = playlist_paths_clone3.clone();
            let player = player_clone2.clone();

            let index = row.index() as usize;

            if let Some(song_path) = playlist_paths.borrow().get(index) {
                let uri = format!("file://{}", song_path.display());
                println!("Playing: {}", uri);

                player.set_property("uri", &uri);
                // Stop current playback
                player.set_state(gst::State::Null).unwrap();

                // Set new song URI
                player.set_property("uri", &uri);

                // Start playback
                player.set_state(gst::State::Playing).unwrap();

                *is_playing_clone2.borrow_mut() = true;
                pause_button_clone2.set_label("Pause");
            }
        });

        // ----------------------------
        // Pause / Resume button logic
        // ----------------------------
        let player_clone_for_pause = player.clone();
        let is_playing_clone3 = is_playing.clone();
        let pause_button_clone3 = pause_button.clone();

        pause_button.connect_clicked(move |_| {
            let mut playing = is_playing_clone3.borrow_mut();
            if *playing {
                player_clone_for_pause
                    .set_state(gst::State::Paused)
                    .unwrap();
                *playing = false;
                pause_button_clone3.set_label("Resume");
            } else {
                player_clone_for_pause
                    .set_state(gst::State::Playing)
                    .unwrap();
                *playing = true;
                pause_button_clone3.set_label("Pause");
            }
        });

        // Show window
        window.present();
    });

    app.run();
}
