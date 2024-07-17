crate::ix!();

pub trait ListBoxInterface: CreateSnapshotOfRows {}

pub trait CreateSnapshotOfRows {

    /**
      | This fairly obscure method creates
      | an image that shows the row components
      | specified in rows (for example, these
      | could be the currently selected row
      | components).
      | 
      | It's a handy method for doing drag-and-drop,
      | as it can be passed to the
      | 
      | DragAndDropContainer for use as the
      | drag image.
      | 
      | -----------
      | @note
      | 
      | it will make the row components temporarily
      | invisible, so if you're using custom
      | components this could affect them if
      | they're sensitive to that sort of thing.
      | 
      | @see Component::createComponentSnapshot
      |
      */
    fn create_snapshot_of_rows(&mut self, 
            rows: &SparseSet<i32>,
            x:    &mut i32,
            y:    &mut i32) -> Image;

}

//---------------------------------------------
pub trait ListBoxModelInterface:
    GetNumRows 
    + PaintListBoxItem 
    + RefreshComponentForRow 
    + GetNameForRow 
    + ListBoxItemClicked 
    + ListBoxItemDoubleClicked 
    + BackgroundClicked 
    + SelectedRowsChanged 
    + DeleteKeyPressed 
    + ReturnKeyPressed 
    + ListWasScrolled 
    + GetDragSourceDescription 
    + GetTooltipForRow 
    + GetMouseCursorForRow { }

pub trait GetNumRows {

    /**
      | This has to return the number of items
      | in the list. @see ListBox::getNumRows()
      |
      */
    fn get_num_rows(&mut self) -> i32;
}

pub trait PaintListBoxItem {

    /**
      | This method must be implemented to draw
      | a row of the list.
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
    fn paint_list_box_item(&mut self, 
        row_number:      i32,
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        row_is_selected: bool);
}

pub trait RefreshComponentForRow {

    /**
      | This is used to create or update a custom
      | component to go in a row of the list.
      | 
      | Any row may contain a custom component,
      | or can just be drawn with the paintListBoxItem()
      | method and handle mouse clicks with
      | listBoxItemClicked().
      | 
      | This method will be called whenever
      | a custom component might need to be updated
      | - e.g. when the list is changed, or ListBox::updateContent()
      | is called.
      | 
      | If you don't need a custom component
      | for the specified row, then return nullptr.
      | (Bear in mind that even if you're not
      | creating a new component, you may still
      | need to delete existingComponentToUpdate
      | if it's non-null).
      | 
      | If you do want a custom component, and
      | the existingComponentToUpdate is
      | null, then this method must create a
      | suitable new component and return it.
      | 
      | If the existingComponentToUpdate
      | is non-null, it will be a pointer to a
      | component previously created by this
      | method. In this case, the method must
      | either update it to make sure it's correctly
      | representing the given row (which may
      | be different from the one that the component
      | was created for), or it can delete this
      | component and return a new one.
      | 
      | The component that your method returns
      | will be deleted by the ListBox when it
      | is no longer needed.
      | 
      | Bear in mind that if you put a custom component
      | inside the row but still want the listbox
      | to automatically handle clicking,
      | selection, etc, then you'll need to
      | make sure your custom component doesn't
      | intercept all the mouse events that
      | land on it, e.g by using Component::setInterceptsMouseClicks().
      |
      */
    fn refresh_component_for_row(&mut self, 
        row_number:                   i32,
        is_row_selected:              bool,
        existing_component_to_update: *mut Component) -> *mut Component;
}

pub trait GetNameForRow {

    /**
      | This can be overridden to return a name
      | for the specified row.
      | 
      | By default this will just return a string
      | containing the row number.
      |
      */
    fn get_name_for_row(&mut self, row_number: i32) -> String;

}

pub trait ListBoxItemClicked {

    /**
      | This can be overridden to react to the
      | user clicking on a row. @see listBoxItemDoubleClicked
      |
      */
    fn list_box_item_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent);
}

pub trait ListBoxItemDoubleClicked {

    /**
      | This can be overridden to react to the
      | user double-clicking on a row. @see
      | listBoxItemClicked
      |
      */
    fn list_box_item_double_clicked(&mut self, 
        row: i32,
        _1:  &MouseEvent);

}

pub trait BackgroundClicked {

    /**
      | This can be overridden to react to the
      | user clicking on a part of the list where
      | there are no rows. @see listBoxItemClicked
      |
      */
    fn background_clicked(&mut self, _0: &MouseEvent);
}

pub trait SelectedRowsChanged {

    /**
      | Override this to be informed when rows
      | are selected or deselected.
      | 
      | This will be called whenever a row is
      | selected or deselected. If a range of
      | rows is selected all at once, this will
      | just be called once for that event.
      | 
      | -----------
      | @param lastRowSelected
      | 
      | the last row that the user selected.
      | If no rows are currently selected, this
      | may be -1.
      |
      */
    fn selected_rows_changed(&mut self, last_row_selected: i32);
}

pub trait DeleteKeyPressed {

    /**
      | Override this to be informed when the
      | delete key is pressed.
      | 
      | If no rows are selected when they press
      | the key, this won't be called.
      | 
      | -----------
      | @param lastRowSelected
      | 
      | the last row that had been selected when
      | they pressed the key - if there are multiple
      | selections, this might not be very useful
      |
      */
    fn delete_key_pressed(&mut self, last_row_selected: i32);

}


pub trait ReturnKeyPressed {

    /**
      | Override this to be informed when the
      | return key is pressed.
      | 
      | If no rows are selected when they press
      | the key, this won't be called.
      | 
      | -----------
      | @param lastRowSelected
      | 
      | the last row that had been selected when
      | they pressed the key - if there are multiple
      | selections, this might not be very useful
      |
      */
    fn return_key_pressed(&mut self, last_row_selected: i32);

}

pub trait ListWasScrolled {

    /**
      | Override this to be informed when the
      | list is scrolled.
      | 
      | This might be caused by the user moving
      | the scrollbar, or by programmatic changes
      | to the list position.
      |
      */
    fn list_was_scrolled(&mut self);
}

pub trait GetDragSourceDescription {

    /**
      | To allow rows from your list to be dragged-and-dropped,
      | implement this method.
      | 
      | If this returns a non-null variant then
      | when the user drags a row, the listbox
      | will try to find a DragAndDropContainer
      | in its parent hierarchy, and will use
      | it to trigger a drag-and-drop operation,
      | using this string as the source description,
      | with the listbox itself as the source
      | component.
      | 
      | @see DragAndDropContainer::startDragging
      |
      */
    fn get_drag_source_description(&mut self, rows_to_describe: &SparseSet<i32>) -> Var;
}

pub trait GetTooltipForRow {

    /**
      | You can override this to provide tool
      | tips for specific rows. @see TooltipClient
      |
      */
    fn get_tooltip_for_row(&mut self, row: i32) -> String;
}

pub trait GetMouseCursorForRow {

    /**
      | You can override this to return a custom
      | mouse cursor for each row.
      |
      */
    fn get_mouse_cursor_for_row(&mut self, row: i32) -> MouseCursor;
}
