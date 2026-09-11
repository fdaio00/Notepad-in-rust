use eframe::egui;

mod actions;
mod edit_actions;
mod edit_menu_bar;
mod file_actions;
mod search;
mod session;
mod shortcuts;
mod view;
mod workspace;

use search::SearchState;

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
    search: SearchState,
    // Controls whether long lines continue on the next visual line.
    word_wrap: bool,
    // Stores the editor zoom as a percentage, such as 100 or 120.
    zoom_percentage: i32,
    // Stores the current line number shown in the status bar.
    cursor_line: usize,
    // Stores the current column number shown in the status bar.
    cursor_column: usize,
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
            search: SearchState::new(),
            word_wrap: true,
            zoom_percentage: 100,
            cursor_line: 1,
            cursor_column: 1,
        }
    }
}
impl eframe::App for NotepadApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.process_pending_editor_actions(ui.ctx());
        self.update_window_title(ui.ctx());
        self.handle_shortcuts(ui.ctx());

        egui::Panel::top("menu_bar").show(ui, |ui| {
            self.show_menu_bar(ui);
        });
        egui::Panel::bottom("status_bar").show(ui, |ui| {
            self.show_status_bar(ui);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            self.show_tabs(ui);
            self.show_find_bar(ui);
            ui.separator();
            self.show_editor(ui);
        });

        self.show_unsaved_changes_dialog(ui.ctx());
        self.handle_close_request(ui.ctx());
    }
}
