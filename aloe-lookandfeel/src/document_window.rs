crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the window.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum DocumentWindowColourIds
{
    /**
      | The colour to draw any text with. It's
      | up to the look and feel class how this
      | is used.
      |
      */
    textColourId                = 0x1005701,  
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide window
  | drawing functionality.
  |
  */
pub trait DocumentWindowLookAndFeelMethods {

    fn draw_document_window_title_bar(
        &mut self, 
        _0:                      &mut DocumentWindow,
        _1:                      &mut Graphics,
        w:                       i32,
        h:                       i32,
        title_spacex:            i32,
        title_spacew:            i32,
        icon:                    *const Image,
        draw_title_text_on_left: bool
    );

    fn create_document_window_button(&mut self, button_type: i32) -> *mut Button;

    fn position_document_window_buttons(
        &mut self, 
        _0:                                 &mut DocumentWindow,
        title_barx:                         i32,
        title_bary:                         i32,
        title_barw:                         i32,
        title_barh:                         i32,
        minimise_button:                    *mut Button,
        maximise_button:                    *mut Button,
        close_button:                       *mut Button,
        position_title_bar_buttons_on_left: bool
    );
}
