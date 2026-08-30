use super::actions::PendingAction; //something in our parent app module
use super::NotepadApp;
use eframe::egui;

use crate::ui::dialogs::dialog::{show_confirmation_dialog, DialogResult};
impl NotepadApp {
    pub(super) fn request_close_tab(&mut self, index: usize) {
        let modified = self
            .workspace
            .document(index)
            .map(|document| document.is_modified())
            .unwrap_or(false);

        if modified {
            self.pending_action = Some(PendingAction::CloseTab(index));
        } else {
            self.workspace.close_tab(index);
        }
    }

    pub(super) fn handle_close_request(&mut self, ctx: &egui::Context) {
        let close_requested = ctx.input(|input| input.viewport().close_requested());

        if !close_requested {
            return;
        }
        if self.allow_exit {
            return;
        }

        if !self.workspace.has_modified_documents() {
            return;
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        self.pending_action = Some(PendingAction::Exit);
    }

    fn execute_pending_action(&mut self, ctx: &egui::Context) {
        match self.pending_action.take() {
            // Some(PendingAction::New) => {
            //     self.document = Document::new();
            // }
            Some(PendingAction::CloseTab(index)) => self.workspace.close_tab(index),

            // Some(PendingAction::Open) => {

            // }
            Some(PendingAction::Exit) => {
                self.allow_exit = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            None => {}
        }
    }
    pub(super) fn request_exit(&mut self, ctx: &egui::Context) {
        if self.workspace.has_modified_documents() {
            self.pending_action = Some(PendingAction::Exit);
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    pub(super) fn show_unsaved_changes_dialog(&mut self, ctx: &egui::Context) {
        if self.pending_action.is_none() {
            return;
        }

        let message = match self.pending_action {
            Some(PendingAction::CloseTab(_)) => "Do you want to save your changes?",

            Some(PendingAction::Exit) => "Do you want to save all unsaved changes before exiting?",

            None => return,
        };

        let result = show_confirmation_dialog(
            ctx,
            "unsaved_change_dialog",
            "Unsaved Changes",
            message,
            "Save",
            "Don't Save",
            "Cancel",
        );

        match result {
            Some(DialogResult::Primary) => match self.pending_action {
                Some(PendingAction::CloseTab(index)) => {
                    if self.save_document(index) {
                        self.execute_pending_action(ctx);
                    } else {
                        self.pending_action = None;
                    }
                }

                Some(PendingAction::Exit) => {
                    if self.save_all_modified_documents() {
                        self.execute_pending_action(ctx);
                    } else {
                        self.pending_action = None;
                    }
                }

                None => {}
            },

            Some(DialogResult::Secondary) => {
                self.execute_pending_action(ctx);
            }

            Some(DialogResult::Cancel) => {
                self.pending_action = None;
            }

            None => {}
        }
    }
}
