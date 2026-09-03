use super::NotepadApp;
use eframe::egui;
use eframe::egui::TextBuffer;

impl NotepadApp {
    pub(super) fn undo_active_document(&mut self, ctx: &egui::Context) {
        let active_document_id = self.workspace.active_document().id();
        let editor_id = egui::Id::new(("editor", active_document_id));

        let mut state = match egui::TextEdit::load_state(ctx, editor_id) {
            //Option<TextEditState>
            Some(state) => state,

            None => return,
        };

        let cursor_range = match state.cursor.char_range() {
            // return Option<CCursorRange>
            Some(range) => range,
            None => return,
        };

        let current_text = self.workspace.active_document_mut().content().to_owned();

        let current_state = (cursor_range, current_text);

        let mut undoer = state.undoer();

        let previous_state = match undoer.undo(&current_state).cloned() {
            //that makes it owned
            Some(previous_state) => previous_state,
            None => return,
        };

        //deconstruct it
        let (previous_cursor, previous_text) = previous_state;

        // Put the old text back into our Document.
        {
            let document = self.workspace.active_document_mut();
            *document.content_mut() = previous_text;
            document.mark_as_modified();
        }

        // Restore the old cursor/selection.
        state.cursor.set_char_range(Some(previous_cursor));

        // Put the changed Undoer back into TextEditState.
        state.set_undoer(undoer);

        // Save the changed TextEditState back into egui.
        state.store(ctx, editor_id);
    }

    pub(super) fn redo_active_document(&mut self, ctx: &egui::Context) {
        let active_document_id = self.workspace.active_document().id();

        let editor_id = egui::Id::new(("editor", active_document_id));

        let mut state = match egui::TextEdit::load_state(ctx, editor_id) {
            Some(state) => state,
            None => return,
        };

        let cursor_range = match state.cursor.char_range() {
            Some(range) => range,
            None => return,
        };

        let current_text = self.workspace.active_document().content().to_owned();

        let current_state = (cursor_range, current_text);

        let mut undoer = state.undoer();

        let next_state = match undoer.redo(&current_state).cloned() {
            Some(next_state) => next_state,
            None => return,
        };

        let (next_cursor, next_text) = next_state;

        {
            let document = self.workspace.active_document_mut();
            *document.content_mut() = next_text;
            document.mark_as_modified();
        }

        state.cursor.set_char_range(Some(next_cursor));
        state.set_undoer(undoer);
        state.store(ctx, editor_id);
    }

    pub(super) fn delete_selection(&mut self, ctx: &egui::Context) {
        let document_id = self.workspace.active_document().id();

        let (selection_document_id, cursor_range) = match self.last_editor_selection {
            Some(value) => value,
            None => return,
        };

        if selection_document_id != document_id {
            return;
        }

        let range = cursor_range.as_sorted_char_range();

        if range.start == range.end {
            return;
        }

        {
            let document = self.workspace.active_document_mut();

            // TextBuffer uses character indexes, matching egui's cursor indexes.
            document.content_mut().delete_char_range(range.clone());

            document.mark_as_modified();
        }

        let editor_id = egui::Id::new(("editor", document_id));

        if let Some(mut state) = egui::TextEdit::load_state(ctx, editor_id) {
            // Collapse the cursor where the deleted selection started.
            let cursor = egui::text::CCursor::new(range.start);

            state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(cursor)));

            state.store(ctx, editor_id);
        }

        self.last_editor_selection = None;

        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });
    }
    pub(super) fn copy_selection(&self, ctx: &egui::Context) {
        let document = self.workspace.active_document();

        let (document_id, cursor_range) = match self.last_editor_selection {
            Some(value) => value,
            None => return,
        };

        // Cached selection must belong to this document.
        if document_id != document.id() {
            return;
        }

        let range = cursor_range.as_sorted_char_range();

        if range.start == range.end {
            return;
        }

        // egui cursor positions are CHARACTER positions,
        // so use TextBuffer::char_range instead of normal
        // Rust byte slicing.
        let selected_text = document.content().char_range(range).to_owned();

        // Send the selected text directly to the OS clipboard.
        ctx.copy_text(selected_text);
    }

    pub(super) fn cut_selection(&mut self, ctx: &egui::Context) {
        let document_id = self.workspace.active_document().id();

        let (selection_document_id, cursor_range) = match self.last_editor_selection {
            Some(value) => value,
            None => return,
        };

        if selection_document_id != document_id {
            return;
        }

        let range = cursor_range.as_sorted_char_range();

        if range.start == range.end {
            return;
        }

        // Copy the selected text before removing it.
        let selected_text = self
            .workspace
            .active_document()
            .content()
            .char_range(range.clone())
            .to_owned();

        ctx.copy_text(selected_text);

        {
            let document = self.workspace.active_document_mut();

            // TextBuffer uses character indexes, matching egui's cursor indexes.
            document.content_mut().delete_char_range(range.clone());

            document.mark_as_modified();
        }

        let editor_id = egui::Id::new(("editor", document_id));

        if let Some(mut state) = egui::TextEdit::load_state(ctx, editor_id) {
            // After Cut, collapse the selection at its starting position.
            let cursor = egui::text::CCursor::new(range.start);

            state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(cursor)));

            state.store(ctx, editor_id);
        }

        self.last_editor_selection = None;

        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });
    }

    pub(super) fn request_paste(&mut self) {
        // Don't paste immediately because the menu currently
        // owns focus.
        //
        // show_editor() will perform the actual paste after
        // returning focus to the TextEdit.
        self.pending_paste = true;
    }

    pub(super) fn active_editor_has_selection(&self) -> bool {
        let active_document_id = self.workspace.active_document().id();

        let (document_id, cursor_range) = match self.last_editor_selection {
            Some(value) => value,
            None => return false,
        };

        // Never use a selection belonging to another tab.
        if document_id != active_document_id {
            return false;
        }

        let range = cursor_range.as_sorted_char_range();

        range.start != range.end
    }

    pub(super) fn process_pending_editor_actions(&mut self, ctx: &egui::Context) {
        if !self.pending_paste {
            return;
        }

        let document_id = self.workspace.active_document().id();

        let editor_id = egui::Id::new(("editor", document_id));

        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });

        ctx.send_viewport_cmd(egui::ViewportCommand::RequestPaste);

        ctx.request_repaint();

        self.pending_paste = false;
    }
}
