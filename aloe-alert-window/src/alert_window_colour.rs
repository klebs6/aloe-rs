crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the alert
  | box.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum AlertWindowColourIds
{
    /**
      | The background colour for the window.
      |
      */
    backgroundColourId          = 0x1001800,  

    /**
      | The colour for the text.
      |
      */
    textColourId                = 0x1001810,  

    /**
      | An optional colour to use to draw a border
      | around the window.
      |
      */
    outlineColourId             = 0x1001820,   
}
