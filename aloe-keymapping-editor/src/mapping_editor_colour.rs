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
pub enum KeyMappingEditorComponentColourIds
{
    /**
      | The background colour to fill the editor
      | background.
      |
      */
    backgroundColourId  = 0x100ad00,    

    /**
      | The colour for the text.
      |
      */
    textColourId        = 0x100ad01,    
}
