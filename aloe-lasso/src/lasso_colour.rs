crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the label.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | -----------
  | @note
  | 
  | you can also use the constants from TextEditor::ColourIds
  | to change the colour of the text editor
  | that is opened when a label is editable.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum LassoComponentColourIds
{
    /**
      | The colour to fill the lasso rectangle
      | with.
      |
      */
    lassoFillColourId       = 0x1000440, 

    /**
      | The colour to draw the outline with.
      |
      */
    lassoOutlineColourId    = 0x1000441, 
}
