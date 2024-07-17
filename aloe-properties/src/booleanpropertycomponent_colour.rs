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
pub enum BooleanPropertyComponentColourIds
{
    /**
      | The colour to fill the background of
      | the button area.
      |
      */
    backgroundColourId          = 0x100e801,    

    /**
      | The colour to use to draw an outline around
      | the text area.
      |
      */
    outlineColourId             = 0x100e803,    
}
