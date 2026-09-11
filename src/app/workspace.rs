use crate::document::Document;
use std::path::{Path, PathBuf};

pub(crate) struct Workspace {
    documents: Vec<Document>,
    active_tab: usize,
    next_untitled_id: usize,
    next_document_id: usize,
}

impl Workspace {
    pub(crate) fn new() -> Self {
        Self {
            //here whne we intilize the application ,
            //at least one document is being created

            //and this way is equivilant to :
            //let mut documents = Vec::new();
            //documents.push(Document::new());
            documents: vec![Document::new_untitled(1, 1)],
            active_tab: 0,
            next_untitled_id: 2,
            next_document_id: 2,
        }
    }
    fn create_untitled_document(&mut self) -> Document {
        let document = Document::new_untitled(self.next_untitled_id, self.next_document_id);

        self.next_untitled_id += 1;
        self.next_document_id += 1;

        document
    }
    pub(crate) fn new_tab(&mut self) {
        let document = self.create_untitled_document();
        self.documents.push(document);
        self.active_tab = self.documents.len() - 1;
        //here is mines 1 becouse if we have 4 doucments and we
        //created a new one then the no comes 4, but it is with index
        //therefore we are doing mines one so that it get to the active one
        //which is the new one or the new document created
    }

    pub(crate) fn close_tab(&mut self, index: usize) {
        if index >= self.documents.len() {
            return;
        }

        self.documents.remove(index);

        if self.documents.is_empty() {
            let document = self.create_untitled_document();

            self.documents.push(document);
            self.active_tab = 0;

            return;
        }

        if index < self.active_tab {
            self.active_tab -= 1;
        } else if self.active_tab >= self.documents.len() {
            self.active_tab = self.documents.len() - 1;
        }
    }

    // pub(crate) fn close_tab(&mut self, index: usize) {
    //     self.documents.remove(index);

    //     let new_index = if self.active_tab > index {
    //         index - 1
    //     } else {
    //         index + 1
    //     };

    //     self.active_tab = new_index;
    //     //here is mines 1 becouse if we have 4 doucments and we
    //     //created a new one then the no comes 4, but it is with index
    //     //therefore we are doing mines one so that it get to the active one
    //     //which is the new one or the new document created
    // }
    // Opens a file as a document and gives it the next unique ID.
    pub(crate) fn open_document(&mut self, path: PathBuf, content: String) {
        let document = Document::from_file(path, content, self.next_document_id);

        self.next_document_id += 1;
        self.documents.push(document);
        self.active_tab = self.documents.len() - 1;
    }
    pub(crate) fn active_tab(&self) -> usize {
        self.active_tab
    }

    pub(crate) fn set_active_tab(&mut self, index: usize) {
        if index < self.documents.len() {
            self.active_tab = index;
        }
    }

    pub(crate) fn active_document(&self) -> &Document {
        &self.documents[self.active_tab]
        //just borrow without mutting
    }

    pub(crate) fn active_document_mut(&mut self) -> &mut Document {
        &mut self.documents[self.active_tab]
        //borrow with abillity to chang the workspace.
    }

    pub(crate) fn document(&self, index: usize) -> Option<&Document> {
        self.documents.get(index)

        //not self.document[index] becouse this will crash the app
        // the get() works becouse it returns Option<&type>
    }

    pub(crate) fn document_mut(&mut self, index: usize) -> Option<&mut Document> {
        self.documents.get_mut(index)
    }

    pub(crate) fn len(&self) -> usize {
        self.documents.len()
    }

    pub(crate) fn find_document_by_path(&self, path: &Path) -> Option<usize> {
        for (index, document) in self.documents.iter().enumerate()
        //enumator gives the indexes(0,document), (1,document)
        {
            if document.path() == Some(path) {
                return Some(index);
            }
        }

        None
    }

    pub(crate) fn has_modified_documents(&self) -> bool {
        self.documents.iter().any(|doc| doc.is_modified())
    }
}
