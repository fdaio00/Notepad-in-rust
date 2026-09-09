use super::NotepadApp;
use eframe::egui;

pub(super) struct SearchState {
    query: String,
    visible: bool,
}

impl SearchState {
    pub(super) fn new() -> Self {
        Self {
            query: String::new(),
            visible: false,
        }
    }
}

impl NotepadApp {
    pub(super) fn open_find(&mut self) {
        self.search.visible = true;
    }

    pub(super) fn close_find(&mut self) {
        self.search.visible = false;
    }

    pub(super) fn show_find_bar(&mut self, ui: &mut egui::Ui) {
        if !self.search.visible {
            return;
        }

        let mut close_requested = false;

        ui.horizontal(|ui| {
            ui.label("Find:");

            ui.text_edit_singleline(&mut self.search.query);

            // Search behavior comes in the next step.
            if ui.button("Next").clicked() {}
        });

        if close_requested {
            self.close_find();
        }
    }
}
