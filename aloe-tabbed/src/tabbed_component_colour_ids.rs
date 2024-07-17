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
pub enum TabbedComponentColourIds
{
    /**
      | The colour to fill the background behind
      | the tabs.
      |
      */
    backgroundColourId          = 0x1005800,    

    /**
      | The colour to use to draw an outline around
      | the content. (See setOutline)
      |
      */
    outlineColourId             = 0x1005801,    
}
