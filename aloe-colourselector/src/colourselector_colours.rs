crate::ix!();

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
pub enum ColourSelectorColourIds
{
    /**
      | the colour used to fill the component's
      | background.
      |
      */
    backgroundColourId              = 0x1007000,    

    /**
      | the colour used for the labels next to
      | the sliders.
      |
      */
    labelTextColourId               = 0x1007001,     
}
