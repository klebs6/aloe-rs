crate::ix!();

/**
  | One of these is used by a TableListBox
  | as the data model for the table's contents.
  | 
  | The virtual methods that you override
  | in this class take care of drawing the
  | table cells, and reacting to events.
  | 
  | @see TableListBox
  | 
  | @tags{GUI}
  |
  */
pub trait TableListBoxModel: 
TableListBoxGetNumRows 
+ TableListBoxPaintRowBackground 
+ TableListBoxRefreshComponentForCell
+ TableListBoxCellClicked
+ TableListBoxCellDoubleClicked
+ TableListBoxBackgroundClicked
+ TableListBoxSortOrderChanged
+ TableListBoxGetColumnAutoSizeWidth
+ TableListBoxGetCellTooltip
+ TableListBoxSelectedRowsChanged
+ TableListBoxDeleteKeyPressed
+ TableListBoxReturnKeyPressed
+ TableListBoxListWasScrolled
+ TableListBoxGetDragSourceDescription
+ TableListBoxPaintCell { }

pub trait TableListBoxGetDragSourceDescription {

    /**
      | To allow rows from your table to be dragged-and-dropped,
      | implement this method.
      | 
      | If this returns a non-null variant then
      | when the user drags a row, the table will
      | try to find a DragAndDropContainer
      | in its parent hierarchy, and will use
      | it to trigger a drag-and-drop operation,
      | using this string as the source description,
      | and the listbox itself as the source
      | component.
      | 
      | @see getDragSourceCustomData, DragAndDropContainer::startDragging
      |
      */
    fn get_drag_source_description(
        &mut self, 
        currently_selected_rows: &SparseSet<i32>
    ) -> Var 
    {
        Var::default()
    }
}

pub trait TableListBoxListWasScrolled {

    /**
      | Override this to be informed when the
      | list is scrolled.
      | 
      | This might be caused by the user moving
      | the scrollbar, or by programmatic changes
      | to the list position.
      |
      */
    fn list_was_scrolled(&mut self) { }
}

pub trait TableListBoxReturnKeyPressed {

    /**
      | Override this to be informed when the
      | return key is pressed. @see ListBox::returnKeyPressed()
      |
      */
    fn return_key_pressed(&mut self, last_row_selected: i32) { }

}

pub trait TableListBoxDeleteKeyPressed {

    /**
      | Override this to be informed when the
      | delete key is pressed. @see ListBox::deleteKeyPressed()
      |
      */
    fn delete_key_pressed(&mut self, last_row_selected: i32) { }
}

pub trait TableListBoxSelectedRowsChanged {

    /**
      | Override this to be informed when rows
      | are selected or deselected. @see ListBox::selectedRowsChanged()
      |
      */
    fn selected_rows_changed(&mut self, last_row_selected: i32) { }
}

pub trait TableListBoxGetCellTooltip {

    /**
      | Returns a tooltip for a particular cell
      | in the table.
      |
      */
    fn get_cell_tooltip(&mut self, 
            row_number: i32,
            column_id:  i32) -> String { 
        String::new() 
    }
}

pub trait TableListBoxGetColumnAutoSizeWidth {
    
    /**
      | Returns the best width for one of the
      | columns.
      | 
      | If you implement this method, you should
      | measure the width of all the items in
      | this column, and return the best size.
      | 
      | Returning 0 means that the column shouldn't
      | be changed.
      | 
      | This is used by TableListBox::autoSizeColumn()
      | and TableListBox::autoSizeAllColumns().
      |
      */
    fn get_column_auto_size_width(&mut self, column_id: i32) -> i32 { 0 }
}
    
pub trait TableListBoxSortOrderChanged {

    /**
      | This callback is made when the table's
      | sort order is changed.
      | 
      | This could be because the user has clicked
      | a column header, or because the
      | 
      | TableHeaderComponent::setSortColumnId()
      | method was called.
      | 
      | If you implement this, your method should
      | re-sort the table using the given column
      | as the key.
      |
      */
    fn sort_order_changed(
        &mut self, 
        new_sort_column_id: i32,
        is_forwards:        bool) {}
}

pub trait TableListBoxBackgroundClicked {

    /**
      | This can be overridden to react to the
      | user double-clicking on a part of the
      | list where there are no rows.
      | 
      | @see cellClicked
      |
      */
    fn background_clicked(&mut self, _0: &MouseEvent) {}
}

pub trait TableListBoxCellDoubleClicked {

    /**
      | This callback is made when the user clicks
      | on one of the cells in the table.
      | 
      | The mouse event's coordinates will
      | be relative to the entire table row.
      | @see cellClicked, backgroundClicked
      |
      */
    fn cell_double_clicked(
        &mut self, 
        row_number: i32,
        column_id:  i32,
        _2:         &MouseEvent) {}
}

pub trait TableListBoxCellClicked {

    /**
      | This callback is made when the user clicks
      | on one of the cells in the table.
      | 
      | The mouse event's coordinates will
      | be relative to the entire table row.
      | @see cellDoubleClicked, backgroundClicked
      |
      */
    fn cell_clicked(
        &mut self, 
        row_number: i32,
        column_id:  i32,
        _2:         &MouseEvent) {}
}

pub trait TableListBoxRefreshComponentForCell {

    /**
      | This is used to create or update a custom
      | component to go in a cell.
      | 
      | Any cell may contain a custom component,
      | or can just be drawn with the paintCell()
      | method and handle mouse clicks with
      | cellClicked().
      | 
      | This method will be called whenever
      | a custom component might need to be updated
      | - e.g. when the table is changed, or TableListBox::updateContent()
      | is called.
      | 
      | If you don't need a custom component
      | for the specified cell, then return
      | nullptr. (Bear in mind that even if you're
      | not creating a new component, you may
      | still need to delete existingComponentToUpdate
      | if it's non-null).
      | 
      | If you do want a custom component, and
      | the existingComponentToUpdate is
      | null, then this method must create a
      | new component suitable for the cell,
      | and return it.
      | 
      | If the existingComponentToUpdate
      | is non-null, it will be a pointer to a
      | component previously created by this
      | method. In this case, the method must
      | either update it to make sure it's correctly
      | representing the given cell (which
      | may be different from the one that the
      | component was created for), or it can
      | delete this component and return a new
      | one.
      |
      */
    fn refresh_component_for_cell(
        &mut self, 
        row_number:                   i32,
        column_id:                    i32,
        is_row_selected:              bool,
        existing_component_to_update: *mut Component

    ) -> *mut Component<'_> {

        todo!();
        /*
           ignoreUnused (existingComponentToUpdate);
           jassert (existingComponentToUpdate == nullptr); // indicates a failure in the code that recycles the components
           return nullptr;
           */
    }
}

pub trait TableListBoxPaintCell {

    /**
      | This must draw one of the cells.
      | 
      | The graphics context's origin will
      | already be set to the top-left of the
      | cell, whose size is specified by (width,
      | height).
      | 
      | -----------
      | @note
      | 
      | the rowNumber value may be greater than
      | the number of rows in your list, so be
      | careful that you don't assume it's less
      | than getNumRows().
      |
      */
    fn paint_cell(&mut self, 
            _0:              &mut Graphics,
            row_number:      i32,
            column_id:       i32,
            width:           i32,
            height:          i32,
            row_is_selected: bool);
}

pub trait TableListBoxPaintRowBackground {

    /**
      | This must draw the background behind
      | one of the rows in the table.
      | 
      | The graphics context has its origin
      | at the row's top-left, and your method
      | should fill the area specified by the
      | width and height parameters.
      | 
      | -----------
      | @note
      | 
      | the rowNumber value may be greater than
      | the number of rows in your list, so be
      | careful that you don't assume it's less
      | than getNumRows().
      |
      */
    fn paint_row_background(&mut self, 
            _0:              &mut Graphics,
            row_number:      i32,
            width:           i32,
            height:          i32,
            row_is_selected: bool);
}

pub trait TableListBoxGetNumRows {

    /**
      | This must return the number of rows currently
      | in the table.
      | 
      | If the number of rows changes, you must
      | call TableListBox::updateContent()
      | to cause it to refresh the list.
      |
      */
    fn get_num_rows(&mut self) -> i32;
}
