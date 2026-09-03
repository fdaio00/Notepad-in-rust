use super::NotepadApp;
use eframe::egui;

impl NotepadApp {
    pub(super) fn show_edit_menu_list(&mut self, ui: &mut egui::Ui) {
        let has_selection = self.active_editor_has_selection();

        ui.menu_button("Edit", |ui| {
            if ui.button("Undo").clicked() {
                self.undo_active_document(ui.ctx());
                ui.close();
            }

            if ui.button("Redo").clicked() {
                self.redo_active_document(ui.ctx());
                ui.close();
            }

            ui.separator();

            if ui
                .add_enabled(has_selection, egui::Button::new("Copy"))
                .clicked()
            {
                self.copy_selection(ui.ctx());
                ui.close();
            }

            if ui
                .add_enabled(has_selection, egui::Button::new("Cut"))
                .clicked()
            {
                self.cut_selection(ui.ctx());
                ui.close();
            }

            if ui
                .add_enabled(has_selection, egui::Button::new("Delete"))
                .clicked()
            {
                self.delete_selection(ui.ctx());
                ui.close();
            }

            // Paste does NOT require a selection.
            if ui.button("Paste").clicked() {
                self.request_paste();
                ui.close();
            }
        });
    }
}
