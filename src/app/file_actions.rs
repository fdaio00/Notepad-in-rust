use super::NotepadApp; //go to my parent modal
use crate::document::Document;
use crate::file::service::{read_text, write_text};
use crate::ui::file_dialog::{choose_open_path, choose_save_path};

use std::path::PathBuf;
impl NotepadApp {
    //crate is usable anyware inside the root
    //super is used inside the parent modal whcih is NotepadApp;

    pub(super) fn request_new_document(&mut self) {
        self.workspace.new_tab();
        // if self.document.is_modified() {
        //     self.pending_action = Some(PendingAction::New)
        // } else {
        //     self.document = Document::new();
        // }
    }

    pub(super) fn open_file(&mut self) {
        let path = match choose_open_path() {
            Some(path) => path,
            None => return,
        };

        if let Some(index) = self.workspace.find_document_by_path(&path) {
            self.workspace.set_active_tab(index);
            return;
        }

        match read_text(&path) {
            Ok(content) => {
                let document = Document::from_file(path, content);

                self.workspace.open_document(document);
            }

            Err(error) => {
                eprintln!("Failed to open document: {error}");
            }
        }
    }

    pub(super) fn save_all_modified_documents(&mut self) -> bool {
        for index in 0..self.workspace.len() {
            let modified = self
                .workspace
                .document(index)
                .map(|doc| doc.is_modified())
                .unwrap_or(false);

            if modified && !self.save_document(index) {
                return false;
            }
        }

        true
    }

    fn save_document_to(&mut self, index: usize, path: PathBuf) -> bool {
        let save_result = {
            let document = match self.workspace.document(index) {
                Some(document) => document,
                None => return false,
            };

            write_text(&path, document.content())
        };

        match save_result {
            Ok(()) => {
                let document = match self.workspace.document_mut(index) {
                    Some(document) => document,
                    None => return false,
                };

                document.set_path(path);
                document.mark_as_saved();

                true
            }

            Err(error) => {
                eprintln!("Failed to save document: {error}");
                false
            }
        }
    }

    pub(super) fn save_document(&mut self, index: usize) -> bool {
        let existing_path = self
            .workspace
            .document(index)
            .and_then(|document| document.path())
            .map(|path| path.to_path_buf());

        let path = match existing_path {
            Some(path) => path,

            None => {
                let default_name = match self.workspace.document(index) {
                    Some(document) => document.display_name().to_string(),
                    None => return false,
                };

                match choose_save_path(&default_name) {
                    Some(path) => path,
                    None => return false,
                }
            }
        };

        self.save_document_to(index, path)
    }

    pub(super) fn save_document_as(&mut self, index: usize) -> bool {
        let default_name = match self.workspace.document(index) {
            Some(document) => document.display_name().to_string(),
            None => return false,
        };

        let path = match choose_save_path(&default_name) {
            Some(path) => path,
            None => return false,
        };

        self.save_document_to(index, path)
    }
}
