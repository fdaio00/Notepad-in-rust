use super::NotepadApp;
use eframe::egui;

impl NotepadApp {
    pub(super) fn show_edit_menu_list(&mut self, ui: &mut egui::Ui) {
        let has_selection = self.active_editor_has_selection();

        let has_selection = self.active_editor_has_selection();

        let (can_undo, can_redo) = self.active_editor_history_status(ui.ctx());

        ui.menu_button("Edit", |ui| {
            if ui
                .add_enabled(can_undo, egui::Button::new("Undo").shortcut_text("Ctrl+Z"))
                .clicked()
            {
                self.undo_active_document(ui.ctx());
                ui.close();
            }

            if ui
                .add_enabled(can_redo, egui::Button::new("Redo").shortcut_text("Ctrl+Y"))
                .clicked()
            {
                self.redo_active_document(ui.ctx());
                ui.close();
            }

            ui.separator();

            if ui
                .add_enabled(
                    has_selection,
                    egui::Button::new("Cut").shortcut_text("Ctrl+X"),
                )
                .clicked()
            {
                self.cut_selection(ui.ctx());
                ui.close();
            }

            if ui
                .add_enabled(
                    has_selection,
                    egui::Button::new("Copy").shortcut_text("Ctrl+C"),
                )
                .clicked()
            {
                self.copy_selection(ui.ctx());
                ui.close();
            }

            if ui
                .add(egui::Button::new("Paste").shortcut_text("Ctrl+V"))
                .clicked()
            {
                self.request_paste();
                ui.close();
            }

            if ui
                .add_enabled(
                    has_selection,
                    egui::Button::new("Delete").shortcut_text("Del"),
                )
                .clicked()
            {
                self.delete_selection(ui.ctx());
                ui.close();
            }

            ui.separator();

            if ui
                .add(egui::Button::new("Select All").shortcut_text("Ctrl+A"))
                .clicked()
            {
                self.select_all(ui.ctx());
                ui.close();
            }

            if ui
                .add(egui::Button::new("Find").shortcut_text("Ctrl+F"))
                .clicked()
            {
                self.open_find();
                ui.close();
            }
        });
    }
}
