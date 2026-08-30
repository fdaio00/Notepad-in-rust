use std::path::{Path, PathBuf};
#[derive(Debug)]
//why the pub(crate)
//this means that I can use this struct inside the application crate,
//but it is not exported explicitly out sid it
pub(crate) struct Document {
    id: usize,
    title: String,
    content: String,
    path: Option<PathBuf>,
    //this is better than String becouse it is better for path file formation
    //while a string is just an ordinary text or a string,
    //and we use otions becouse we want to make sure that if we have pahts
    //then it is a  path, otherwiese it is just a null.
    //eiather Some(path) or None
    modified: bool,
    // saved: bool,
}

//this is the default() function and what it does
//so eaither to write Documnet::new(), or Document::defaul();
impl Default for Document {
    fn default() -> Self {
        Self {
            id: 1,
            title: "Untitlted Note".to_string(),
            content: String::new(),
            path: None,
            modified: false,
        }
    }
}

impl Document {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }

    pub(crate) fn new_untitled(number: usize, id: usize) -> Self {
        Self {
            id: id,
            title: format!("Untitled {}", number),
            content: String::new(),
            path: None,
            modified: false,
        }
    }
    pub(crate) fn display_name(&self) -> &str {
        self.path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or(&self.title)
    }

    //this function is to make sure that I am able to mut the content
    //it is better not to make the members public so that they stat private
    pub(crate) fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }
    pub(crate) fn content(&self) -> &str {
        &self.content

        //String borrow &str
    }

    pub(crate) fn is_modified(&self) -> bool {
        self.modified
    }

    pub(crate) fn path(&self) -> Option<&Path> {
        self.path.as_deref()
        //this turn out from PathBuf to &path
        //we we need &path instade of PathBuf?
        //because callers generally only need to work with a path, not specifically an owned PathBuf.(what does this even mean?)
    }

    pub(crate) fn mark_as_saved(&mut self) {
        self.modified = false;
    }

    pub(crate) fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub(crate) fn mark_as_modified(&mut self) {
        self.modified = true;
    }

    pub(crate) fn from_file(path: PathBuf, content: String) -> Self {
        Self {
            id: 1,
            title: String::new(),
            content,
            path: Some(path),
            modified: false,
        }
    }
}
