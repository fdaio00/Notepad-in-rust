use super::NotepadApp;
use eframe::egui;

impl NotepadApp {
    pub(super) fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Check the more specific shortcut before Ctrl+S.
        let save_as = egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
            egui::Key::S,
        );

        let save = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::S);

        let new_document = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::N);

        let open = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::O);

        let close_tab = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::W);

        let find = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::F);

        if ctx.input_mut(|input| input.consume_shortcut(&save_as)) {
            let index = self.workspace.active_tab();
            self.save_document_as(index);
            return;
        }

        if ctx.input_mut(|input| input.consume_shortcut(&find)) {
            self.open_find();
            return;
        }
        if ctx.input_mut(|input| input.consume_shortcut(&save)) {
            let index = self.workspace.active_tab();
            self.save_document(index);
            return;
        }

        if ctx.input_mut(|input| input.consume_shortcut(&new_document)) {
            self.request_new_document();
            return;
        }

        if ctx.input_mut(|input| input.consume_shortcut(&open)) {
            self.open_file();
            return;
        }

        if ctx.input_mut(|input| input.consume_shortcut(&close_tab)) {
            let index = self.workspace.active_tab();
            self.request_close_tab(index);
        }
    }
}
