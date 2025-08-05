use eframe::{egui, App, NativeOptions, run_native};

// Importapps
mod apps;
use apps::initiative_tracker::InitiativeTracker;

// Define the possible states/views of our main application
enum AppView {
    LandingPage,
    InitiativeTracker,
}

// The new main application struct
struct TtrpgApp {
    current_view: AppView,
    // It holds an instance of our initiative tracker app
    initiative_tracker: InitiativeTracker,
}

impl Default for TtrpgApp {
    fn default() -> Self {
        Self {
            current_view: AppView::LandingPage,
            initiative_tracker: InitiativeTracker::default(),
        }
    }
}

// Implement the `App` trait for our main application shell
impl App for TtrpgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Use a `match` statement to decide which view to show
        match self.current_view {
            AppView::LandingPage => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("TTRPG Utilities");
                    ui.separator();
                    ui.label("Welcome! Please select an application to launch.");

                    if ui.button("D&D Initiative Tracker").clicked() {
                        // Switch the view when the button is clicked
                        self.current_view = AppView::InitiativeTracker;
                    }
                    // You can add more buttons for future apps here
                });
            }
            AppView::InitiativeTracker => {
                // If the view is InitiativeTracker, delegate the UI drawing
                // to our initiative_tracker instance.
                self.initiative_tracker.update_ui(ctx);

                // We can also add a "Back" button to return to the landing page
                egui::TopBottomPanel::bottom("bottom_panel")
                    .show(ctx, |ui| {
                        if ui.button("⬅ Back to Main Menu").clicked() {
                            self.current_view = AppView::LandingPage;
                        }
                    });
            }
        }
    }
}

// The main function now launches our new TtrpgApp
fn main() {
    let window_options = NativeOptions::default();
    let _ = run_native(
        "TTRPG Utilities",
        window_options,
        Box::new(|_cc| Ok(Box::new(TtrpgApp::default()))),
    );
}

