use super::NotepadApp;
use eframe::egui;

impl NotepadApp {
    //The Ui here is to build the UI.
    pub(super) fn show_editor(&mut self, ui: &mut egui::Ui) {
        let document_id = self.workspace.active_document().id();
        let editor_id = egui::Id::new(("editor", document_id));

        let response = {
            let document = self.workspace.active_document_mut();

            let response = ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(document.content_mut()).id(editor_id),
            );

            if response.changed() {
                document.mark_as_modified();
            }

            response
        };

        if response.has_focus() {
            if let Some(state) = egui::TextEdit::load_state(ui.ctx(), editor_id) {
                if let Some(cursor_range) = state.cursor.char_range() {
                    self.last_editor_selection = Some((document_id, cursor_range));
                }
            }
        }
    }

    pub(super) fn update_window_title(&self, ctx: &egui::Context) {
        let document = self.workspace.active_document();
        let modified_marker = if document.is_modified() { "*" } else { "" };

        let window_title = format!("{}{} - Notepad", document.display_name(), modified_marker);

        ctx.send_viewport_cmd(egui::ViewportCommand::Title(window_title))
    }

    pub(super) fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.request_new_document();
                    ui.close();
                };
                if ui.button("Open").clicked() {
                    self.open_file();
                    ui.close();
                };
                if ui.button("Save").clicked() {
                    let index = self.workspace.active_tab();
                    self.save_document(index);
                    ui.close()
                }
                if ui.button("Save As").clicked() {
                    let index = self.workspace.active_tab();
                    self.save_document_as(index);
                    ui.close()
                }
                if ui.button("Exit").clicked() {
                    self.request_exit(ui.ctx());
                    ui.close()
                }
            });
            self.show_edit_menu_list(ui);
        });
    }

    //this is for the tab
    pub(super) fn show_tabs(&mut self, ui: &mut egui::Ui) {
        let active_tab = self.workspace.active_tab();
        let tab_count = self.workspace.len();

        let mut requested_tab = None;
        let mut requested_close = None;
        let mut create_new_tab = false;

        ui.horizontal(|ui| {
            for index in 0..tab_count {
                if let Some(document) = self.workspace.document(index) {
                    let title = if document.is_modified() {
                        format!("{} *", document.display_name())
                    } else {
                        document.display_name().to_string()
                    };

                    if ui.selectable_label(index == active_tab, title).clicked() {
                        requested_tab = Some(index);
                    }

                    if ui.button("x").clicked() {
                        requested_close = Some(index);
                    }
                }
            }
            if ui.button("+").clicked() {
                create_new_tab = true;
            }
        });

        if let Some(index) = requested_close {
            self.request_close_tab(index);
        } else if let Some(index) = requested_tab {
            self.workspace.set_active_tab(index);
        }
        if create_new_tab {
            self.workspace.new_tab();
        }
    }
}
