crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the component.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TextPropertyComponentColourIds
{
    /**
      | The colour to fill the background of
      | the text area.
      |
      */
    backgroundColourId          = 0x100e401,    

    /**
      | The colour to use for the editable text.
      |
      */
    textColourId                = 0x100e402,    

    /**
      | The colour to use to draw an outline around
      | the text area.
      |
      */
    outlineColourId             = 0x100e403,    
}
