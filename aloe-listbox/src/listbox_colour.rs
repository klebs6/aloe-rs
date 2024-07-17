crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the label.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ListBoxColourIds
{
    /**
      | The background colour to fill the list
      | with. Make this transparent if you don't
      | want the background to be filled.
      |
      */
    backgroundColourId      = 0x1002800, 

    /**
      | An optional colour to use to draw a border
      | around the list. Make this transparent
      | to not have an outline.
      |
      */
    outlineColourId         = 0x1002810, 

    /**
      | The preferred colour to use for drawing
      | text in the listbox.
      |
      */
    textColourId            = 0x1002820,  
}
