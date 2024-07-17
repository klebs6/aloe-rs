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
pub enum LabelColourIds
{
    /**
      | The background colour to fill the label
      | with.
      |
      */
    backgroundColourId             = 0x1000280, 

    /**
      | The colour for the text.
      |
      */
    textColourId                   = 0x1000281, 

    /**
      | An optional colour to use to draw a border
      | around the label. Leave this transparent
      | to not have an outline.
      |
      */
    outlineColourId                = 0x1000282, 

    /**
      | The background colour when the label
      | is being edited.
      |
      */
    backgroundWhenEditingColourId  = 0x1000283, 

    /**
      | The colour for the text when the label
      | is being edited.
      |
      */
    textWhenEditingColourId        = 0x1000284, 

    /**
      | An optional border colour when the label
      | is being edited.
      |
      */
    outlineWhenEditingColourId     = 0x1000285,  
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide label
  | drawing functionality.
  |
  */
pub trait LabelLookAndFeelMethods
{
    fn draw_label(
        &mut self, 
        _0: &mut Graphics,
        _1: &mut Label
    );

    fn get_label_font(&mut self, _0: &mut Label) -> Font;

    fn get_label_border_size(&mut self, _0: &mut Label) -> BorderSize<i32>;
}
