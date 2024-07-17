crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/interfaces/aloe_AccessibilityTableInterface.h]

/**
  | An abstract interface which represents
  | a UI element that supports a table interface.
  | 
  | Examples of UI elements which typically
  | support a table interface are lists,
  | tables, and trees.
  | 
  | @tags{Accessibility}
  |
  */
pub trait AccessibilityTableInterface {

    /**
      | Returns the total number of rows in the
      | table.
      |
      */
    fn get_num_rows(&self) -> i32;

    /**
      | Returns the total number of columns
      | in the table.
      |
      */
    fn get_num_columns(&self) -> i32;

    /**
      | Returns the AccessibilityHandler
      | for one of the cells in the table, or nullptr
      | if there is no cell at the specified position.
      |
      */
    fn get_cell_handler(&self, 
            column: i32) -> *const AccessibilityHandler;
}
