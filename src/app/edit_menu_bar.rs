use super::NotepadApp;
use eframe::egui;

impl NotepadApp {
    pub(super) fn show_edit_menu_list(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Edit", |ui| {
            let has_selection = self.active_editor_has_selection(ui.ctx());

            if ui.button("Undo").clicked() {
                self.undo_active_document(ui.ctx());
                ui.close();
            }

            if ui.button("Redo").clicked() {
                self.redo_active_document(ui.ctx());
                ui.close();
            }

            ui.separator();

            // Copy is disabled when nothing is selected.
            if ui
                .add_enabled(has_selection, egui::Button::new("Copy"))
                .clicked()
            {
                self.copy_selection(ui.ctx());
                ui.close();
            }

            // Cut also requires a selection.
            if ui
                .add_enabled(has_selection, egui::Button::new("Cut"))
                .clicked()
            {
                self.cut_selection(ui.ctx());
                ui.close();
            }

            // Paste is discussed below.
            if ui.button("Paste").clicked() {
                self.paste(ui.ctx());
                ui.close();
            }
        });
    }
}
