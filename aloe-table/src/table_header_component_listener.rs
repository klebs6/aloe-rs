crate::ix!();

/**
  | Receives events from a TableHeaderComponent
  | when columns are resized, moved, etc.
  | 
  | You can register one of these objects
  | for table events using TableHeaderComponent::addListener()
  | and TableHeaderComponent::removeListener().
  | 
  | @see TableHeaderComponent
  |
  */
pub trait TableHeaderComponentListener {

    /**
      | This is called when some of the table's
      | columns are added, removed, hidden,
      | or rearranged.
      |
      */
    fn table_columns_changed(&mut self, table_header: *mut TableHeaderComponent);

    /**
      | This is called when one or more of the
      | table's columns are resized.
      |
      */
    fn table_columns_resized(&mut self, table_header: *mut TableHeaderComponent);

    /**
      | This is called when the column by which
      | the table should be sorted is changed.
      |
      */
    fn table_sort_order_changed(&mut self, table_header: *mut TableHeaderComponent);

    /**
      | This is called when the user begins or
      | ends dragging one of the columns around.
      | 
      | When the user starts dragging a column,
      | this is called with the ID of that column.
      | When they finish dragging, it is called
      | again with 0 as the ID.
      |
      */
    fn table_column_dragging_changed(&mut self, 
            table_header:                *mut TableHeaderComponent,
            column_id_now_being_dragged: i32) {}

}
