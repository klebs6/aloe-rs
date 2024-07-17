crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/filebrowser/aloe_FilePreviewComponent.h]

/**
  | Base class for components that live
  | inside a file chooser dialog box and
  | show previews of the files that get selected.
  | 
  | One of these allows special extra information
  | to be displayed for files in a dialog
  | box as the user selects them. Each time
  | the current file or directory is changed,
  | the selectedFileChanged() method
  | will be called to allow it to update itself
  | appropriately.
  | 
  | @see FileChooser, ImagePreviewComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FilePreviewComponent<'a> {
    base: Component<'a>,
}

pub trait SelectedFileChanged {

    /**
      | Called to indicate that the user's currently
      | selected file has changed.
      | 
      | -----------
      | @param newSelectedFile
      | 
      | the newly selected file or directory,
      | which may be a default File() object
      | if none is selected.
      |
      */
    fn selected_file_changed(&mut self, new_selected_file: &File);
}
