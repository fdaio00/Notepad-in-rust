//here are all things that is realted to the
//actions that has been doing to the app
//not UI elements, or proporties of a document
//not file systems or somthing else
//it is applicatin level actions

// src/app/actions.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PendingAction {
    CloseTab(usize),
    Exit,
}
