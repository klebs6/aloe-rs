crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait FilenameComponentLookAndFeelMethods {

    fn create_filename_component_browse_button(&mut self, text: &String) -> *mut Button;

    fn layout_filename_component(
        &mut self, 
        _0:            &mut FilenameComponent,
        filename_box:  *mut ComboBox,
        browse_button: *mut Button
    );
}
