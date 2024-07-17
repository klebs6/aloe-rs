crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the editor.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | NB: You can also set the caret colour
  | using CaretComponent::caretColourId
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TextEditorColourIds
{
    /**
      | The colour to use for the text component's
      | background - this can be transparent
      | if necessary.
      |
      */
    backgroundColourId       = 0x1000200, 

    /**
      | The colour that will be used when text
      | is added to the editor. Note that because
      | the editor can contain multiple colours,
      | calling this method won't change the
      | colour of existing text - to do that,
      | use the applyColourToAllText() method
      |
      */
    textColourId             = 0x1000201, 

    /**
      | The colour with which to fill the background
      | of highlighted sections of the text
      | - this can be transparent if you don't
      | want to show any highlighting.
      |
      */
    highlightColourId        = 0x1000202, 

    /**
      | The colour with which to draw the text
      | in highlighted sections.
      |
      */
    highlightedTextColourId  = 0x1000203, 

    /**
      | If this is non-transparent, it will
      | be used to draw a box around the edge of
      | the component.
      |
      */
    outlineColourId          = 0x1000205, 

    /**
      | If this is non-transparent, it will
      | be used to draw a box around the edge of
      | the component when it has focus.
      |
      */
    focusedOutlineColourId   = 0x1000206, 

    /**
      | If this is non-transparent, it'll be
      | used to draw an inner shadow around the
      | edge of the editor.
      |
      */
    shadowColourId           = 0x1000207, 
}


/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide
  | 
  | TextEditor drawing functionality.
  |
  */
pub trait TextEditorLookAndFeelMethods {

    fn fill_text_editor_background(&mut self, 
            _0:     &mut Graphics,
            width:  i32,
            height: i32,
            _3:     &mut TextEditor);


    fn draw_text_editor_outline(&mut self, 
            _0:     &mut Graphics,
            width:  i32,
            height: i32,
            _3:     &mut TextEditor);


    fn create_caret_component(&mut self, 
        key_focus_owner: *mut Component) -> *mut CaretComponent;
}
