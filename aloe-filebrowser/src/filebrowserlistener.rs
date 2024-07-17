crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FileBrowserListener.h]

/**
  | A listener for user selection events
  | in a file browser.
  | 
  | This is used by a FileBrowserComponent
  | or FileListComponent.
  | 
  | @tags{GUI}
  |
  */
pub trait FileBrowserListener {
    
    /**
      | Callback when the user selects a different
      | file in the browser.
      |
      */
    fn selection_changed(&mut self);

    /**
      | Callback when the user clicks on a file
      | in the browser.
      |
      */
    fn file_clicked(&mut self, 
            file: &File,
            e:    &MouseEvent);

    /**
      | Callback when the user double-clicks
      | on a file in the browser.
      |
      */
    fn file_double_clicked(&mut self, file: &File);

    /**
      | Callback when the browser's root folder
      | changes.
      |
      */
    fn browser_root_changed(&mut self, new_root: &File);
}
