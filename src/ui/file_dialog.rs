use rfd::FileDialog;
use std::path::PathBuf;

pub(crate) fn choose_save_path(default_name: &str) -> Option<PathBuf> {
    //thr FileDialog, returns Option<PathBuf> ;
    FileDialog::new()
        .set_title("Save Note")
        .set_file_name(default_name)
        .add_filter("Text files", &["txt"])
        .save_file()

    // https://docs.rs/rfd/latest/rfd/struct.FileDialog.html
}

pub(crate) fn choose_open_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Open Note")
        .add_filter("Text files", &["txt"])
        .pick_file()
}
