use eframe::egui;

/// The state of the whole application. One copy of this struct exists for
/// as long as the window is open, and whatever we store here survives
/// between frames (a "frame" = one redraw of the UI).
///
/// - `name` is a `String`: a growable, owned piece of text.
/// - `age` is a `u32`: an unsigned (never negative) 32-bit integer.
pub struct MyApp {
    name: String,
    age: u32,
}

/// `Default` is a standard Rust trait meaning "this type has an obvious
/// starting value". Calling `MyApp::default()` (which main.rs does) runs
/// the function below. A *trait* is like an interface in other languages.
impl Default for MyApp {
    /// Builds the starting version of the app state: name "Arthur", age 42.
    fn default() -> Self {
        // `Self` is shorthand for "MyApp" (the type this impl block is about).
        Self {
            name: "Arthur".to_owned(), // turn a text literal into an owned String
            age: 42,
        }
    }
}

/// Here we implement the `eframe::App` trait for `MyApp`. eframe only knows
/// how to run types that promise "I have a `ui` method" — this line is that
/// promise, and it's what makes MyApp a real app.
impl eframe::App for MyApp {
    /// Called by eframe every frame to redraw the UI (~60 times per second,
    /// or whenever something changes and a redraw is needed).
    ///
    /// - `&mut self` means we can read AND change our state here (the text
    ///   boxes below write straight into `self.name` / `self.age`).
    /// - `ui` is the object we draw with.
    /// - `_frame` starts with `_` because we don't use it (Rust convention:
    ///   the underscore silences the "unused variable" warning).
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // A panel that fills the whole window. The closure `|ui| { ... }` is
        // where we add widgets. egui is "immediate mode": this code runs top
        // to bottom every frame, rebuilding the UI from scratch each time.
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("My egui Application");

            // Lay the next widgets out side by side (in a horizontal row).
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                // A one-line text box. Passing `&mut self.name` lets the box
                // edit our String directly — whatever the user types ends
                // up stored there.
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            // A slider bound to `self.age`, allowed range 0 to 120 (`0..=120`
            // is an inclusive range). Dragging it updates `self.age`.
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            // `ui.button(...).clicked()` is true only on the frame where the
            // user pressed it, so the counter goes up by exactly 1 per click.
            if ui.button("Increment").clicked() {
                self.age += 1;
            }

            // Show the current state. `format!` builds a String out of
            // text mixed with values (here: name and age).
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
