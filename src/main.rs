use eframe::{egui, run_native, App, NativeOptions};

// Import apps
mod apps;
use apps::initiative_tracker::InitiativeTracker;

// Trait for all sub-apps in the TTRPG suite.
// Each sub-app must implement this to provide its name and UI logic.
trait TtrpgSubApp {
    fn name(&self) -> &'static str;
    fn update_ui(&mut self, ctx: &egui::Context);
}

// Implement the trait for InitiativeTracker
impl TtrpgSubApp for InitiativeTracker {
    fn name(&self) -> &'static str {
        "D&D Initiative Tracker"
    }
    fn update_ui(&mut self, ctx: &egui::Context) {
        self.update_ui(ctx);
    }
}

// Enum for current view
enum AppView {
    LandingPage,
    SubApp(usize), // Index into sub_apps
}

struct TtrpgApp {
    current_view: AppView,
    // List of all sub-apps; 
    // add new apps to this vector to make them available in the UI.
    sub_apps: Vec<Box<dyn TtrpgSubApp>>,
}

impl Default for TtrpgApp {
    fn default() -> Self {
        Self {
            current_view: AppView::LandingPage,
            // Add new sub-apps to this vector to make them appear in the sidebar.
            sub_apps: vec![
                Box::new(InitiativeTracker::default()),
                // Box::new(OtherSubApp::default()),
            ],
        }
    }
}

// Implement the `App` trait for our main application shell
impl App for TtrpgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_view {
            AppView::LandingPage => {
                egui::SidePanel::left("sidebar").show(ctx, |ui| {
                    ui.heading("TTRPG Utilities");
                    ui.separator();
                    ui.label("Select an application:");

                    // Dynamically generate a button for each sub-app in the sidebar.
                    for (i, app) in self.sub_apps.iter().enumerate() {
                        if ui.button(app.name()).clicked() {
                            self.current_view = AppView::SubApp(i);
                        }
                    }
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Welcome! Choose a tool from the sidebar.");
                });
            }
            AppView::SubApp(i) => {
                // Delegate UI to the selected sub-app
                self.sub_apps[i].update_ui(ctx);

                // Add a "Back" button to return to the landing page
                egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
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
