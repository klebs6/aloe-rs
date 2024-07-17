crate::ix!();

pub trait GetRoots {

    /**
      | Returns a list of names and paths for
      | the default places the user might want
      | to look.
      | 
      | By default this just calls getDefaultRoots(),
      | but you may want to override it to return
      | a custom list.
      |
      */
    fn get_roots(&mut self, 
        root_names: &mut Vec<String>,
        root_paths: &mut Vec<String>);

}

pub trait GetActionVerb {

    /**
      | Returns a verb to describe what should
      | happen when the file is accepted.
      | 
      | E.g. if browsing in "load file" mode,
      | this will be "Open", if in "save file"
      | mode, it'll be "Save", etc.
      |
      */
    fn get_action_verb(&self) -> String;
}

pub trait GetNumSelectedFiles {

    /**
      | Returns the number of files the user
      | has got selected. @see getSelectedFile
      |
      */
    fn get_num_selected_files(&self) -> i32;
}

pub trait GetSelectedFile {

    /**
      | Returns one of the files that the user
      | has currently selected.
      | 
      | The index should be in the range 0 to (getNumSelectedFiles()
      | - 1). @see getNumSelectedFiles
      |
      */
    fn get_selected_file(&self, index: i32) -> File;
}

pub trait DeselectAllFiles {

    /**
      | Deselects any selected files.
      |
      */
    fn deselect_all_files(&mut self);
}

pub trait ScrollToTop {

    /**
      | Scrolls this view to the top.
      |
      */
    fn scroll_to_top(&mut self);
}

pub trait SetSelectedFile {

    /**
      | If the specified file is in the list,
      | it will become the only selected item
      | (and if the file isn't in the list, all
      | other items will be deselected).
      |
      */
    fn set_selected_file(&mut self, _0: &File);
}
