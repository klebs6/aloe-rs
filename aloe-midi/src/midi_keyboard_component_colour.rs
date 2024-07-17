crate::ix!();

/**
  | The direction of the keyboard. @see
  | setOrientation
  |
  */
pub enum MidiKeyboardComponentOrientation
{
    horizontalKeyboard,
    verticalKeyboardFacingLeft,
    verticalKeyboardFacingRight,
}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the keyboard.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum MidiKeyboardComponentColourIds
{
    whiteNoteColourId               = 0x1005000,
    blackNoteColourId               = 0x1005001,
    keySeparatorLineColourId        = 0x1005002,

    /**
      | This colour will be overlaid on the normal
      | note colour.
      |
      */
    mouseOverKeyOverlayColourId     = 0x1005003,  

    /**
      | This colour will be overlaid on the normal
      | note colour.
      |
      */
    keyDownOverlayColourId          = 0x1005004,  

    textLabelColourId               = 0x1005005,
    upDownButtonBackgroundColourId  = 0x1005006,
    upDownButtonArrowColourId       = 0x1005007,
    shadowColourId                  = 0x1005008
}
