crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/interfaces/aloe_AccessibilityTextInterface.h]

/**
  | An abstract interface which represents
  | a UI element that supports a text interface.
  | 
  | A UI element can use this interface to
  | provide extended textual information
  | which cannot be conveyed using just
  | the title, description, and help text
  | properties of AccessibilityHandler.
  | This is typically for text that an accessibility
  | client might want to read line-by-line,
  | or provide text selection and input
  | for.
  | 
  | @tags{Accessibility}
  |
  */
pub trait AccessibilityTextInterface {

    /**
      | Returns true if the text being displayed
      | is protected and should not be exposed
      | to the user, for example a password entry
      | field.
      |
      */
    fn is_displaying_protected_text(&self) -> bool;

    /**
      | Returns true if the text being displayed
      | is read-only or false if editable.
      |
      */
    fn is_read_only(&self) -> bool;

    /**
      | Returns the total number of characters
      | in the text element.
      |
      */
    fn get_total_num_characters(&self) -> i32;

    /**
      | Returns the range of characters that
      | are currently selected, or an empty
      | range if nothing is selected.
      |
      */
    fn get_selection(&self) -> Range<i32>;

    /**
      | Selects a section of the text.
      |
      */
    fn set_selection(&mut self, new_range: Range<i32>);

    /**
      | Gets the current text insertion position,
      | if supported.
      |
      */
    fn get_text_insertion_offset(&self) -> i32;

    /**
      | Returns a section of text.
      |
      */
    fn get_text(&self, range: Range<i32>) -> String;

    /**
      | Replaces the text with a new string.
      |
      */
    fn set_text(&mut self, new_text: &String);

    /**
      | Returns the bounding box in screen coordinates
      | for a range of text. As the range may span
      | multiple lines, this method returns
      | a RectangleList.
      |
      */
    fn get_text_bounds(&self, text_range: Range<i32>) -> RectangleList<i32>;

    /**
      | Returns the index of the character at
      | a given position in screen coordinates.
      |
      */
    fn get_offset_at_point(&self, point: Point<i32>) -> i32;
}
