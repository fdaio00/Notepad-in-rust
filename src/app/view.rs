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
        let mut editor_font = egui::TextStyle::Body.resolve(ui.style());

        // Only the document text changes size when the user changes the zoom.
        editor_font.size *= self.zoom_percentage as f32 / 100.0;

        // Calculates enough rows to keep the editor filling the window.
        let row_height = ui.fonts_mut(|fonts| fonts.row_height(&editor_font))
            + ui.spacing().extra_text_line_spacing;
        let editor_rows = ((available_size.y - 4.0) / row_height).floor().max(1.0) as usize;

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
                        .font(editor_font)
                        .desired_rows(editor_rows)
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
            let cursor_index = cursor_range.primary.index;

            // Counts lines and columns before the current cursor position.
            let (cursor_line, cursor_column) = {
                let text = self.workspace.active_document().content();
                let mut line = 1;
                let mut column = 1;

                for character in text.chars().take(cursor_index.into()) {
                    if character == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }

                (line, column)
            };

            self.cursor_line = cursor_line;
            self.cursor_column = cursor_column;

            let range = cursor_range.as_sorted_char_range();

            if range.start != range.end {
                self.last_editor_selection = Some((document_id, cursor_range));
            } else {
                self.last_editor_selection = None;
            }
        }
    }

    // Draws the current cursor position and zoom at the bottom of the window.
    pub(super) fn show_status_bar(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(format!("{}%", self.zoom_percentage));
            ui.separator();
            ui.label(format!(
                "Ln {}, Col {}",
                self.cursor_line, self.cursor_column
            ));
        });
    }

    // Makes the editor text 10 percent larger, up to 500 percent.
    pub(super) fn zoom_in(&mut self) {
        self.zoom_percentage = (self.zoom_percentage + 10).min(500);
    }

    // Makes the editor text 10 percent smaller, down to 10 percent.
    pub(super) fn zoom_out(&mut self) {
        self.zoom_percentage = (self.zoom_percentage - 10).max(10);
    }

    // Returns the editor text to its normal size.
    pub(super) fn reset_zoom(&mut self) {
        self.zoom_percentage = 100;
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
                    self.request_close_tab(index, ui.ctx());
                    ui.close();
                }
            });
            self.show_edit_menu_list(ui);
            ui.menu_button("View", |ui| {
                // The checkbox directly turns Word Wrap on or off.
                if ui.checkbox(&mut self.word_wrap, "Word Wrap").clicked() {
                    ui.close();
                }

                // The checkbox shows or hides the status bar.
                if ui
                    .checkbox(&mut self.show_status_bar, "Status Bar")
                    .clicked()
                {
                    ui.close();
                }

                ui.separator();

                ui.menu_button("Zoom", |ui| {
                    if ui
                        .add(egui::Button::new("Zoom In").shortcut_text("Ctrl++"))
                        .clicked()
                    {
                        self.zoom_in();
                        ui.close();
                    }

                    if ui
                        .add(egui::Button::new("Zoom Out").shortcut_text("Ctrl+-"))
                        .clicked()
                    {
                        self.zoom_out();
                        ui.close();
                    }

                    if ui
                        .add(egui::Button::new("Restore Default Zoom").shortcut_text("Ctrl+0"))
                        .clicked()
                    {
                        self.reset_zoom();
                        ui.close();
                    }

                    ui.separator();
                    ui.label(format!("{}%", self.zoom_percentage));
                });
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
            self.request_close_tab(index, ui.ctx());
        } else if let Some(index) = requested_tab {
            self.workspace.set_active_tab(index);
        }
        if create_new_tab {
            self.workspace.new_tab();
        }
    }
}
