crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the box.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum FileChooserDialogBoxColourIds
{
    /**
      | The colour to use to draw the box's title.
      |
      */
    titleTextColourId      = 0x1000850, 
}
