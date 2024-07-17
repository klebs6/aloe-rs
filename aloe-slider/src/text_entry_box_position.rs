crate::ix!();

/**
  | The position of the slider's text-entry
  | box. @see setTextBoxStyle
  |
  */
pub enum SliderTextEntryBoxPosition
{
    /**
      | Doesn't display a text box.
      |
      */
    NoTextBox,              

    /**
      | Puts the text box to the left of the slider,
      | vertically centred.
      |
      */
    TextBoxLeft,            

    /**
      | Puts the text box to the right of the slider,
      | vertically centred.
      |
      */
    TextBoxRight,           

    /**
      | Puts the text box above the slider, horizontally
      | centred.
      |
      */
    TextBoxAbove,           

    /**
      | Puts the text box below the slider, horizontally
      | centred.
      |
      */
    TextBoxBelow,            
}
