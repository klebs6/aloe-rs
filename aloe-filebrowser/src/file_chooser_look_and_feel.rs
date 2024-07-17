crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide various
  | file-browser layout and drawing methods.
  |
  */
pub trait FileBrowserComponentLookAndFeelMethods
{
    /**
      | These return a pointer to an internally
      | cached drawable - make sure you don't keep
      | a copy of this pointer anywhere, as it may
      | become invalid in the future.
      */
    fn get_default_folder_image(&mut self) -> *const Drawable;

    fn get_default_document_file_image(&mut self) -> *const Drawable;

    fn create_file_chooser_header_text(&mut self, 
        title:        &String,
        instructions: &String) -> AttributedString;

    fn draw_file_browser_row(&mut self, 
        _0:                    &mut Graphics,
        width:                 i32,
        height:                i32,
        file:                  &File,
        filename:              &String,
        optional_icon:         *mut Image,
        file_size_description: &String,
        file_time_description: &String,
        is_directory:          bool,
        is_item_selected:      bool,
        item_index:            i32,
        _11:                   &mut DirectoryContentsDisplayComponent);

    fn create_file_browser_go_up_button(&mut self) -> *mut Button;

    fn layout_file_browser_component(&mut self, 
        browser_comp:        &mut FileBrowserComponent,
        file_list_component: *mut DirectoryContentsDisplayComponent,
        preview_comp:        *mut FilePreviewComponent,
        current_path_box:    *mut ComboBox,
        filename_box:        *mut TextEditor,
        go_up_button:        *mut Button);
}
