use eframe::egui;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct PCInfo {
    name: String,
    hp: i32,
}

fn load_pcs_from_file() -> Result<Vec<PCInfo>, Box<dyn Error>> {
    // 1. Read the file's contents into a string.
    let data_string = std::fs::read_to_string("resources/dnd_pc_info.json")?;

    // 2. Parse the string of JSON data into a Vec<InitiativeTrackerEntry>.
    //    The `?` operator will automatically handle any parsing errors.
    let entries = serde_json::from_str(&data_string)?;

    // 3. Return the successfully parsed data.
    Ok(entries)
}

#[derive(Clone)]
pub struct InitiativeTrackerEntry {
    name: String,
    initiative: i32,
    hp_current: i32,
    hp_total: i32,
    hp_update: i32,
    update_sign: i32, // +1 or -1
    conditions: String,
}

// Add a default impl for InitiativeTrackerEntry to make adding new ones easier
impl Default for InitiativeTrackerEntry {
    fn default() -> Self {
        Self {
            name: "New Combatant".to_string(),
            initiative: 0,
            hp_current: 10,
            hp_total: 10,
            hp_update: 0,
            update_sign: -1,
            conditions: String::new(),
        }
    }
}

pub struct InitiativeTracker {
    /// The list of all combatants in the tracker.
    entries: Vec<InitiativeTrackerEntry>,
    /// The index of the active combatant in the `entries` vector.
    /// `None` means combat has not started.
    active_index: Option<usize>,
    /// The current round number.
    round_count: u32,
    /// If true, the tie-breaker pop-up window should be displayed.
    show_tie_breaker: bool,
    /// The initiative scores which are tied.
    tied_init: Vec<i32>,
}

// "We are now starting an implementation block..."
impl Default for InitiativeTracker {
    // "...where we will implement the 'Default' trait for the 'InitiativeTracker' struct."
    // "Here is the 'default' function that the 'Default' trait requires."
    // "It takes no arguments and returns an instance of 'Self' (which is shorthand for InitiativeTracker)."
    fn default() -> Self {
        // "We are creating and returning a new instance of 'Self' (InitiativeTracker)."
        Self {
            // "Initialize each field of the struct with its own default value."
            entries: Vec::new(),     // The default for a Vec is an empty vector.
            active_index: None, // The default for our optional index is None (nothing is active).
            round_count: 1,     // We decided a sensible default for the round count is 1.
            show_tie_breaker: false, // The default is to not show the pop-up.
            tied_init: Vec::new(), // The default for tied initiatives is an empty vector.
        }
    }
}

// "Now we are implementing the 'App' trait for our 'InitiativeTracker' struct."
impl InitiativeTracker {
    // "This is where we define how our app will behave when it is run."
    pub fn update_ui(&mut self, ctx: &eframe::egui::Context) {
        if self.show_tie_breaker {
            // This Area covers the whole screen and darkens it, creating a modal effect.
            egui::Area::new(egui::Id::new("tie_breaker_modal_layer"))
                .fixed_pos(egui::pos2(0.0, 0.0))
                .order(egui::Order::Foreground) // Ensure it's drawn on top
                .show(ctx, |ui| {
                    // Now, draw the actual window in the center of the screen
                    egui::Window::new("Resolve Initiative Tie")
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .collapsible(false)
                        .resizable(false)
                        .show(ui.ctx(), |ui| {
                            ui.label("Click the arrows to re-order the combatants.");
                            ui.separator();

                            // Let's find the indices of the tied combatants for self.tied_init[0]
                            let tied_initiative = self.tied_init[0];
                            // Loop through the entries to find all combatants with this initiative
                            let tied_indices: Vec<usize> = self
                                .entries
                                .iter()
                                .enumerate()
                                .filter_map(|(i, entry)| {
                                    if entry.initiative == tied_initiative {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            // The logic for displaying and reordering the tied combatants remains the same
                            // We must use a temporary variable for swapping to avoid borrowing issues
                            let mut swap_indices = None;
                            for i in 0..tied_indices.len() {
                                ui.horizontal(|ui| {
                                    if ui.add_enabled(i > 0, egui::Button::new("^")).clicked() {
                                        // Mark indices to be swapped after the loop
                                        swap_indices = Some((tied_indices[i], tied_indices[i - 1]));
                                    }
                                    if ui
                                        .add_enabled(
                                            i < tied_indices.len() - 1,
                                            egui::Button::new("v"),
                                        )
                                        .clicked()
                                    {
                                        // Mark indices to be swapped after the loop
                                        swap_indices = Some((tied_indices[i], tied_indices[i + 1]));
                                    }
                                    let real_index = tied_indices[i];
                                    ui.label(&self.entries[real_index].name);
                                });
                            }

                            // Perform the swap outside the loop
                            if let Some((index_a, index_b)) = swap_indices {
                                self.entries.swap(index_a, index_b);
                            }

                            ui.separator();
                            if ui.button("Confirm Order").clicked() {
                                self.tied_init.retain(|&init| init != tied_initiative);
                            }
                        });
                });
            if self.tied_init.len() == 0 {
                // If there are no tied initiatives, hide the tie breaker window
                self.show_tie_breaker = false;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Add the 'Add Track' button at the top
                if ui.button("Add New Track").clicked() {
                    self.entries.push(InitiativeTrackerEntry::default());
                }
                if ui.button("Add PCs").clicked() {
                    match load_pcs_from_file() {
                        Ok(pcs) => {
                            for pcs in pcs {
                                // For each PC loaded, create a new InitiativeTrackerEntry
                                // and push it to the entries vector.
                                // We use the default values for initiative and conditions.
                                self.entries.push(InitiativeTrackerEntry {
                                    name: pcs.name,
                                    hp_current: pcs.hp,
                                    hp_total: pcs.hp,
                                    ..InitiativeTrackerEntry::default() // Use default values for the rest
                                })
                            }
                        }
                        Err(e) => {
                            // If it fails, print the error to the console.
                            // A more advanced app might show a pop-up.
                            eprintln!("Failed to load PCs from file: {}", e);
                        }
                    }
                }
                // Add the 'Sort by Initiative' button
                if ui.button("Sort by Initiative").clicked() {
                    self.entries.sort_by_key(|e| -e.initiative); // A slightly shorter way to sort descending// Reset the active index to the first entry after sorting
                    self.active_index = if self.entries.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.tied_init.clear();
                    // Find the first group of ties.
                    // `windows(2)` gives us overlapping pairs of entries to compare.
                    // This should give the list of initiatives with mutiple entries.
                    for (_i, pair) in self.entries.windows(2).enumerate() {
                        if pair[0].initiative == pair[1].initiative {
                            let tied_initiative: i32 = pair[0].initiative;
                            if self.tied_init.last() != Some(&tied_initiative) {
                                // If the last entry in tied_init is not the same as the current initiative, add it
                                self.tied_init.push(tied_initiative);
                            }
                        }
                    }
                    if self.tied_init.len() > 0 {
                        self.show_tie_breaker = true;
                    } else {
                        self.show_tie_breaker = false;
                    }
                }
                // Add the 'Next Turn' button
                if ui.button("Next Turn").clicked() {
                    // If there are no entries, do nothing
                    if self.entries.is_empty() {
                        return;
                    }
                    // Move to the next combatant
                    if let Some(active_index) = self.active_index {
                        let next_index = (active_index + 1) % self.entries.len();
                        self.active_index = Some(next_index);
                        // if next index is 0, increment the round count
                        if next_index == 0 {
                            self.round_count += 1;
                        }
                    } else {
                        // If no active index, set it to the first entry
                        self.active_index = Some(0);
                    }
                }
                if ui.button("Reset Combat").clicked() {
                    self.round_count = 1; // Reset the round count to 1
                    self.active_index = None; // Reset the active index
                    self.entries.clear(); // Clear all entries
                }
                // Add Round Counter
                ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                    ui.label(format!("Round: {}", self.round_count));
                });
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut index_to_remove: Option<usize> = None;
                for i in 0..self.entries.len() {
                    let is_active = self.active_index == Some(i);
                    let stroke_color = if is_active {
                        egui::Color32::from_rgb(255, 0, 0) // Red for active combatant
                    } else {
                        egui::Color32::from_rgb(255, 255, 255) // White for inactive combatants
                    };
                    let stroke = egui::Stroke::new(1.0, stroke_color);

                    egui::Frame::group(ui.style())
                        .stroke(stroke)
                        .show(ui, |ui| {
                            // You can keep the delete button separate, aligned to the right of the whole entry
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                if ui.button("X").on_hover_text("Delete this entry").clicked() {
                                    index_to_remove = Some(i);
                                }
                            });
                            egui::Grid::new(format!("entry_grid_{}", i)) // Each grid needs a unique ID
                                .num_columns(2)
                                .spacing([10.0, 4.0]) // Add some spacing between columns and rows
                                .show(ui, |ui| {
                                    // -- Row 1: Name --
                                    ui.label("Name:");
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.entries[i].name)
                                            .desired_width(f32::INFINITY), // This makes the widget fill the available space
                                    );
                                    ui.end_row(); // Finish the first row

                                    // -- Row 2: Initiative & HP --
                                    ui.label("Stats:");
                                    ui.horizontal(|ui| {
                                        ui.label("Init:");
                                        ui.add(egui::DragValue::new(
                                            &mut self.entries[i].initiative,
                                        ));

                                        ui.separator(); // A small vertical line

                                        ui.label("HP:");
                                        // A DragValue for the current HP
                                        ui.add(egui::DragValue::new(
                                            &mut self.entries[i].hp_current,
                                        ));
                                        ui.label("/");
                                        // A new DragValue for the total HP, making it editable
                                        ui.add(egui::DragValue::new(&mut self.entries[i].hp_total));
                                    });
                                    ui.end_row();

                                    // -- Row 3: HP Update Form --
                                    ui.label("Damage/Heal:");
                                    ui.horizontal(|ui| {
                                        ui.add(egui::DragValue::new(
                                            &mut self.entries[i].hp_update,
                                        ));
                                        ui.radio_value(&mut self.entries[i].update_sign, -1, "−"); // Using a proper minus sign
                                        ui.radio_value(&mut self.entries[i].update_sign, 1, "+");
                                        if ui.button("Update").clicked() {
                                            self.entries[i].hp_current += self.entries[i].hp_update
                                                * self.entries[i].update_sign;
                                            self.entries[i].hp_current = self.entries[i]
                                                .hp_current
                                                .clamp(0, self.entries[i].hp_total);
                                        }
                                    });
                                    ui.end_row();

                                    // -- Row 4: Conditions --
                                    ui.label("Conditions:");
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.entries[i].conditions)
                                            .hint_text("e.g., Poisoned, Prone")
                                            .desired_width(f32::INFINITY), // This also fills the available space
                                    );
                                    ui.end_row();
                                });
                        }); // end of the frame for each entry
                } // end of the loop over entries
                  // After the loop, remove the marked entry if any.
                if let Some(index) = index_to_remove {
                    self.entries.remove(index);
                }
            }); // end of the scroll area
        }); // end of the central panel
    } // end of the update function
} // end of the App trait implementation
