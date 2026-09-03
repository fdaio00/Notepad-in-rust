use eframe::egui;

mod actions;
mod edit_actions;
mod edit_menu_bar;
mod file_actions;
mod session;
mod view;
mod workspace;

use crate::app::actions::PendingAction;
use crate::ui::dialogs::dialog::{show_confirmation_dialog, DialogResult};

use workspace::Workspace;

pub(crate) struct NotepadApp {
    workspace: Workspace,
    pending_action: Option<PendingAction>,
    allow_exit: bool,
    last_editor_selection: Option<(usize, egui::text::CCursorRange)>,
    // Paste cannot safely happen while the menu owns focus.
    // We queue it and process it when show_editor() runs.
    pending_paste: bool,
}

impl NotepadApp {
    //CreationContext: this is used for making configuration..etc.
    // (ONE-TIME APP INITIALIZATION)
    //     configure fonts
    // configure visuals
    // restore saved state
    // initialize graphics resources
    // inspect startup environment

    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            workspace: Workspace::new(),
            pending_action: None,
            allow_exit: false,
            last_editor_selection: None,
            pending_paste: false,
        }
    }
}
impl eframe::App for NotepadApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.process_pending_editor_actions(ui.ctx());
        self.update_window_title(ui.ctx());

        egui::Panel::top("menu_bar").show(ui, |ui| {
            self.show_menu_bar(ui);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            self.show_tabs(ui);
            ui.separator();
            self.show_editor(ui);
        });

        self.show_unsaved_changes_dialog(ui.ctx());
        self.handle_close_request(ui.ctx());
    }
}
