#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

mod app;
use app::app::MyApp;

/// The program's entry point — where the app starts running.
/// It opens the native window and hands control to eframe, which runs
/// the "event loop": it waits for mouse/keyboard input and redraws the UI.
fn main() -> eframe::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        // Configure the window: start it 320 x 240 pixels in size.
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        // `..Default::default()` = "take all other settings from the defaults".
        ..Default::default()
    };
    // Actually start the app:
    // - "My egui App" is the window title
    // - `options` are the window settings above
    // - the closure creates the app state; eframe calls it once at startup,
    //   then calls that state's `ui` method (see app.rs) every frame.
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_ccc| {
            /*
            This gives us image support:
            */

            // Create the initial app state, using the `Default` impl in app.rs.
            Ok(Box::<MyApp>::default())
        }),
    )
}
