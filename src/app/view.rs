use super::NotepadApp;
use eframe::egui;

impl NotepadApp {
    //The Ui here is to build the UI.
    pub(super) fn show_editor(&mut self, ui: &mut egui::Ui) {
        let document_id = self.workspace.active_document().id();

        // Stable ID keeps editor state separate for every document.
        let editor_id = egui::Id::new(("editor", document_id));

        let available_size = ui.available_size();
        let word_wrap = self.word_wrap;

        // Horizontal scrolling is only needed when Word Wrap is turned off.
        let output = {
            let document = self.workspace.active_document_mut();

            egui::ScrollArea::new([!word_wrap, false])
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let editor_width = if word_wrap {
                        ui.available_width()
                    } else {
                        f32::INFINITY
                    };

                    egui::TextEdit::multiline(document.content_mut())
                        .id(editor_id)
                        .desired_width(editor_width)
                        .min_size(available_size)
                        .show(ui)
                })
                .inner
        };

        if output.response.changed() {
            self.workspace.active_document_mut().mark_as_modified();
        }

        // None means the editor lost focus; keep the previous selection.
        if let Some(cursor_range) = output.cursor_range {
            let range = cursor_range.as_sorted_char_range();

            if range.start != range.end {
                self.last_editor_selection = Some((document_id, cursor_range));
            } else {
                self.last_editor_selection = None;
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
                if ui
                    .add(egui::Button::new("New").shortcut_text("Ctrl+N"))
                    .clicked()
                {
                    self.request_new_document();
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Open").shortcut_text("Ctrl+O"))
                    .clicked()
                {
                    self.open_file();
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Save").shortcut_text("Ctrl+S"))
                    .clicked()
                {
                    let index = self.workspace.active_tab();
                    self.save_document(index);
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Save As").shortcut_text("Ctrl+Shift+S"))
                    .clicked()
                {
                    let index = self.workspace.active_tab();
                    self.save_document_as(index);
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("Close Tab").shortcut_text("Ctrl+W"))
                    .clicked()
                {
                    let index = self.workspace.active_tab();
                    self.request_close_tab(index);
                    ui.close();
                }
            });
            self.show_edit_menu_list(ui);
            ui.menu_button("View", |ui| {
                // The checkbox directly turns Word Wrap on or off.
                if ui.checkbox(&mut self.word_wrap, "Word Wrap").clicked() {
                    ui.close();
                }
            });
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
