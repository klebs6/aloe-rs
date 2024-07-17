crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the toolbar.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ToolbarColourIds
{
    /**
      | A colour to use to fill the toolbar's
      | background. For more control over this,
      | override LookAndFeel::paintToolbarBackground().
      |
      */
    backgroundColourId                = 0x1003200,  

    /**
      | A colour to use to draw the separator
      | lines.
      |
      */
    separatorColourId                 = 0x1003210,  

    /**
      | A colour used to paint the background
      | of buttons when the mouse is over them.
      |
      */
    buttonMouseOverBackgroundColourId = 0x1003220,  

    /**
      | A colour used to paint the background
      | of buttons when the mouse is held down
      | on them.
      |
      */
    buttonMouseDownBackgroundColourId = 0x1003230,  

    /**
      | A colour to use for drawing the text under
      | buttons when the style is set to iconsWithText
      | or textOnly.
      |
      */
    labelTextColourId                 = 0x1003240,  

    /**
      | A colour to use for an outline around
      | buttons when the customisation dialog
      | is active and the mouse moves over them.
      |
      */
    editingModeOutlineColourId        = 0x1003250,   
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait ToolbarLookAndFeelMethods {

    fn paint_toolbar_background(&mut self, 
        _0:     &mut Graphics,
        width:  i32,
        height: i32,
        _3:     &mut Toolbar);

    fn create_toolbar_missing_items_button(&mut self, 
        _0: &mut Toolbar) -> *mut Button;

    fn paint_toolbar_button_background(&mut self, 
        _0:            &mut Graphics,
        width:         i32,
        height:        i32,
        is_mouse_over: bool,
        is_mouse_down: bool,
        _5:            &mut dyn ToolbarItemComponent);

    fn paint_toolbar_button_label(&mut self, 
        _0:     &mut Graphics,
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32,
        text:   &String,
        _6:     &mut dyn ToolbarItemComponent);
}

