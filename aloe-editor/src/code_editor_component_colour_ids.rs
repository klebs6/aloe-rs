crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the editor.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ColourIds
{
    /**
      | A colour to use to fill the editor's
      | background.
      |
      */
    backgroundColourId          = 0x1004500,  

    /**
      | The colour to use for the highlighted
      | background under selected text.
      |
      */
    highlightColourId           = 0x1004502,  

    /**
      | The colour to use for text when no syntax
      | colouring is enabled.
      |
      */
    defaultTextColourId         = 0x1004503,  

    /**
      | The colour to use for filling the background
      | of the line-number gutter.
      |
      */
    lineNumberBackgroundId      = 0x1004504,  

    /**
      | The colour to use for drawing the line
      | numbers.
      |
      */
    lineNumberTextId            = 0x1004505,  
}

