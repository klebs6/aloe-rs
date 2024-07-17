crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_TextInputTarget.h]

/**
  | A set of possible on-screen keyboard
  | types, for use in the getKeyboardType()
  | method.
  |
  */
pub enum VirtualKeyboardType
{
    textKeyboard = 0,
    numericKeyboard,
    decimalKeyboard,
    urlKeyboard,
    emailAddressKeyboard,
    phoneNumberKeyboard
}

/**
  | An abstract base class which can be implemented
  | by components that function as text
  | editors.
  | 
  | This class allows different types of
  | text editor component to provide a uniform
  | interface, which can be used by things
  | like OS-specific input methods, on-screen
  | keyboards, etc.
  | 
  | @tags{GUI}
  |
  */
pub trait TextInputTarget {
    
    /**
      | Returns true if this input target is
      | currently accepting input.
      | 
      | For example, a text editor might return
      | false if it's in read-only mode.
      |
      */
    fn is_text_input_active(&self) -> bool;

    /**
      | Returns the extents of the selected
      | text region, or an empty range if nothing
      | is selected,
      |
      */
    fn get_highlighted_region(&self) -> Range<i32>;

    /**
      | Sets the currently-selected text region.
      |
      */
    fn set_highlighted_region(&mut self, new_range: &Range<i32>);

    /**
      | Sets a number of temporarily underlined
      | sections.
      | 
      | This is needed by MS Windows input method
      | UI.
      |
      */
    fn set_temporary_underlining(&mut self, underlined_regions: &[Range<i32>]);

    /**
      | Returns a specified sub-section of
      | the text.
      |
      */
    fn get_text_in_range(&self, range: &Range<i32>) -> String;

    /**
      | Inserts some text, overwriting the
      | selected text region, if there is one.
      |
      */
    fn insert_text_at_caret(&mut self, text_to_insert: &String);

    /**
      | Returns the position of the caret, relative
      | to the component's origin.
      |
      */
    fn get_caret_rectangle(&mut self) -> Rectangle<i32>;

    /**
      | Returns the target's preference for
      | the type of keyboard that would be most
      | appropriate.
      | 
      | This may be ignored, depending on the
      | capabilities of the OS.
      |
      */
    fn get_keyboard_type(&mut self) -> VirtualKeyboardType {
        
        todo!();
        /*
            return textKeyboard;
        */
    }
}
