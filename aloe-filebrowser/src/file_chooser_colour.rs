crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the FileBrowserComponent.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum FileBrowserComponentColourIds
{
    /**
      | The colour to use to fill the background
      | of the current path ComboBox.
      |
      */
    currentPathBoxBackgroundColourId    = 0x1000640, 

    /**
      | The colour to use for the text of the current
      | path ComboBox.
      |
      */
    currentPathBoxTextColourId          = 0x1000641, 

    /**
      | The colour to use to draw the arrow of
      | the current path ComboBox.
      |
      */
    currentPathBoxArrowColourId         = 0x1000642, 

    /**
      | The colour to use to fill the background
      | of the filename TextEditor.
      |
      */
    filenameBoxBackgroundColourId       = 0x1000643, 

    /**
      | The colour to use for the text of the filename
      | TextEditor.
      |
      */
    filenameBoxTextColourId             = 0x1000644,  
}
