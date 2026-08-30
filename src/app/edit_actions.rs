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

        let undo_result = undoer.undo(&current_state); //takes (CCorserRange, String)
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

    pub(super) fn copy_selection(&self, ctx: &egui::Context) {
        // Get the active document so we can rebuild
        // the exact same TextEdit ID used in show_editor().
        let document_id = self.workspace.active_document().id();

        let editor_id = egui::Id::new(("editor", document_id));

        // The Edit menu temporarily takes focus away from
        // the TextEdit. Give focus back to the active editor.
        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });

        // Ask egui to perform the same operation as Ctrl+C.
        // The TextEdit will handle its own current selection.
        ctx.send_viewport_cmd(egui::ViewportCommand::RequestCopy);
    }

    pub(super) fn cut_selection(&self, ctx: &egui::Context) {
        let document_id = self.workspace.active_document().id();
        let editor_id = egui::Id::new(("editor", document_id));

        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });

        ctx.send_viewport_cmd(egui::ViewportCommand::RequestCut);
    }

    pub(super) fn paste(&self, ctx: &egui::Context) {
        let document_id = self.workspace.active_document().id();
        let editor_id = egui::Id::new(("editor", document_id));

        ctx.memory_mut(|memory| {
            memory.request_focus(editor_id);
        });

        ctx.send_viewport_cmd(egui::ViewportCommand::RequestPaste);
    }

    pub(super) fn active_editor_has_selection(&self, ctx: &egui::Context) -> bool {
        // Get the active document so we can rebuild its editor ID.
        let active_document_id = self.workspace.active_document().id();

        let (document_id, cursor_range) = match self.last_editor_selection {
            Some(value) => value,
            None => return false,
        };

        if document_id != active_document_id {
            return false;
        };
        let range = cursor_range.as_sorted_char_range();

        range.start != range.end
    }
}
