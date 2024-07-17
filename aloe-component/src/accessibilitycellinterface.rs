crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/interfaces/aloe_AccessibilityCellInterface.h]

/**
  | An abstract interface which represents
  | a UI element that supports a cell interface.
  | 
  | This typically represents a single
  | cell inside of a UI element which implements
  | an AccessibilityTableInterface.
  | 
  | @tags{Accessibility}
  |
  */
pub trait AccessibilityCellInterface {

    /**
      | Returns the column index of the cell
      | in the table.
      |
      */
    fn get_column_index(&self) -> i32;

    /**
      | Returns the number of columns occupied
      | by the cell in the table.
      |
      */
    fn get_column_span(&self) -> i32;

    /**
      | Returns the row index of the cell in the
      | table.
      |
      */
    fn get_row_index(&self) -> i32;

    /**
      | Returns the number of rows occupied
      | by the cell in the table.
      |
      */
    fn get_row_span(&self) -> i32;

    /**
      | Returns the indentation level for the
      | cell.
      |
      */
    fn get_disclosure_level(&self) -> i32;

    /**
      | Returns the AccessibilityHandler
      | of the table which contains the cell.
      |
      */
    fn get_table_handler(&self) -> *const AccessibilityHandler;
}
