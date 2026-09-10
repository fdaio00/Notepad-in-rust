use super::NotepadApp;
use eframe::egui;

pub(super) struct SearchState {
    query: String,
    visible: bool,
    // This focuses the search box only when Ctrl+F opens it.
    focus_requested: bool,
    not_found: bool,
    // The search can continue again from the opposite side of the document.
    wrap_around: bool,
    // Apply the match after the button or Enter event has finished.
    pending_selection: Option<(usize, egui::text::CCursorRange)>,
}

impl SearchState {
    // Creates the default values used before the Find bar is opened.
    pub(super) fn new() -> Self {
        Self {
            query: String::new(),
            visible: false,
            focus_requested: false,
            not_found: false,
            wrap_around: true,
            pending_selection: None,
        }
    }
}

// Converts an egui character position into a Rust String byte position.
fn char_to_byte_index(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(byte_index, _)| byte_index)
        .unwrap_or(text.len())
}

// Converts a Rust String byte position back into an egui character position.
fn byte_to_char_index(text: &str, byte_index: usize) -> usize {
    text[..byte_index].chars().count()
}

impl NotepadApp {
    // Opens the Find bar and moves the keyboard focus to its text box.
    pub(super) fn open_find(&mut self) {
        self.search.visible = true;
        self.search.focus_requested = true;
        self.search.not_found = false;
    }

    // Hides the Find bar and clears its temporary messages and actions.
    pub(super) fn close_find(&mut self) {
        self.search.visible = false;
        self.search.not_found = false;
        self.search.pending_selection = None;
    }

    // Draws the Find bar and reads its buttons and keyboard input.
    pub(super) fn show_find_bar(&mut self, ui: &mut egui::Ui) {
        if !self.search.visible {
            return;
        }

        self.apply_pending_selection(ui.ctx());

        let mut find_next_requested = false;
        let mut find_previous_requested = false;
        let mut close_requested = ui.input(|input| input.key_pressed(egui::Key::Escape));

        ui.horizontal(|ui| {
            ui.label("Find:");

            let query_response = ui.text_edit_singleline(&mut self.search.query);

            if self.search.focus_requested {
                query_response.request_focus();
                self.search.focus_requested = false;
            }

            if query_response.changed() {
                self.search.not_found = false;
            }

            // Pressing Enter has the same result as clicking Next.
            if (query_response.has_focus() || query_response.lost_focus())
                && ui.input(|input| input.key_pressed(egui::Key::Enter))
            {
                find_next_requested = true;
            }

            if ui.button("Previous").clicked() {
                find_previous_requested = true;
            }

            if ui.button("Next").clicked() {
                find_next_requested = true;
            }

            if ui
                .checkbox(&mut self.search.wrap_around, "Wrap around")
                .changed()
            {
                self.search.not_found = false;
            }

            if ui.button("x").on_hover_text("Close find").clicked() {
                close_requested = true;
            }

            if self.search.not_found {
                if self.search.wrap_around {
                    ui.label("No results found");
                } else {
                    ui.label("No more results");
                }
            }
        });

        if close_requested {
            self.close_find();
        } else if find_previous_requested {
            self.find_previous(ui.ctx());
        } else if find_next_requested {
            self.find_next(ui.ctx());
        }
    }

    // Finds the closest matching text before the current selection.
    pub(super) fn find_previous(&mut self, ctx: &egui::Context) {
        if self.search.query.is_empty() {
            self.search.not_found = false;
            return;
        }

        let document = self.workspace.active_document();

        let document_id = document.id();
        let text = document.content().to_owned();
        let query = self.search.query.clone();

        // Continue before the previous found/selected text.
        let start_char = self
            .last_editor_selection
            .filter(|(id, _)| *id == document_id)
            .map(|(_, range)| {
                let start: usize = range.as_sorted_char_range().start.into();

                start
            })
            .unwrap_or_else(|| text.chars().count());

        let start_byte = char_to_byte_index(&text, start_char);

        // First search only before the current selection.
        let mut match_byte = text[..start_byte].rfind(&query);

        // If enabled, continue again from the document's end.
        if match_byte.is_none() && self.search.wrap_around {
            match_byte = text.rfind(&query);
        }

        let match_byte = match match_byte {
            Some(index) => index,
            None => {
                self.search.not_found = true;
                return;
            }
        };

        self.search.not_found = false;

        let match_start = byte_to_char_index(&text, match_byte);
        let match_end = match_start + query.chars().count();

        let cursor_range = egui::text::CCursorRange::two(
            egui::text::CCursor::new(match_start),
            egui::text::CCursor::new(match_end),
        );

        self.search.pending_selection = Some((document_id, cursor_range));
        ctx.request_repaint();
    }

    // Finds the closest matching text after the current selection.
    pub(super) fn find_next(&mut self, ctx: &egui::Context) {
        if self.search.query.is_empty() {
            self.search.not_found = false;
            return;
        }

        let document = self.workspace.active_document();

        let document_id = document.id();
        let text = document.content().to_owned();
        let query = self.search.query.clone();

        // Continue after the previous found/selected text.
        let start_char = self
            .last_editor_selection
            .filter(|(id, _)| *id == document_id)
            .map(|(_, range)| {
                let end: usize = range.as_sorted_char_range().end.into();

                end
            })
            .unwrap_or(0);

        let start_byte = char_to_byte_index(&text, start_char);

        // First search only after the current selection.
        let mut match_byte = text[start_byte..]
            .find(&query)
            .map(|index| start_byte + index);

        // If enabled, continue again from the document's beginning.
        if match_byte.is_none() && self.search.wrap_around {
            match_byte = text.find(&query);
        }

        let match_byte = match match_byte {
            Some(index) => index,
            None => {
                self.search.not_found = true;
                return;
            }
        };

        self.search.not_found = false;

        let match_start = byte_to_char_index(&text, match_byte);

        let match_end = match_start + query.chars().count();

        let cursor_range = egui::text::CCursorRange::two(
            egui::text::CCursor::new(match_start),
            egui::text::CCursor::new(match_end),
        );

        self.search.pending_selection = Some((document_id, cursor_range));
        ctx.request_repaint();
    }

    // Selects a found match after the click or Enter event has finished.
    fn apply_pending_selection(&mut self, ctx: &egui::Context) {
        let Some((document_id, cursor_range)) = self.search.pending_selection.take() else {
            return;
        };

        if self.workspace.active_document().id() != document_id {
            return;
        }

        let editor_id = egui::Id::new(("editor", document_id));
        let Some(mut state) = egui::TextEdit::load_state(ctx, editor_id) else {
            // The editor creates its state when it is shown for the first time.
            self.search.pending_selection = Some((document_id, cursor_range));
            ctx.request_repaint();
            return;
        };

        state.cursor.set_char_range(Some(cursor_range));
        state.store(ctx, editor_id);

        self.last_editor_selection = Some((document_id, cursor_range));

        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });
    }
}
