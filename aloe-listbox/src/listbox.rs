crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ListBox.h]

/**
  | A list of items that can be scrolled vertically.
  | 
  | To create a list, you'll need to create
  | a subclass of ListBoxModel. This can
  | either paint each row of the list and
  | respond to events via callbacks, or
  | for more specialised tasks, it can supply
  | a custom component to fill each row.
  | 
  | @see ComboBox, TableListBox
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ListBox<'a> {
    base:                    Component<'a>,
    base2:                   SettableTooltipClient,
    model:                   *mut ListBoxModel,
    viewport:                Box<ListViewport<'a>>,
    header_component:        Box<Component<'a>>,
    mouse_move_selector:     Box<dyn MouseListener>,
    selected:                SparseSet<i32>,
    total_items:             i32, // default = 0
    row_height:              i32, // default = 22
    minimum_row_width:       i32, // default = 0
    outline_thickness:       i32, // default = 0
    last_row_selected:       i32, // default = -1
    multiple_selection:      bool, // default = false
    always_flip_selection:   bool, // default = false
    has_done_initial_update: bool, // default = false
    select_on_mouse_down:    bool, // default = true
}

impl<'a> Drop for ListBox<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            headerComponent.reset();
        viewport.reset();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ListBox.cpp]
impl<'a> ListBox<'a> {
    
    /**
      | Returns the current list model.
      |
      */
    pub fn get_model(&self) -> *mut ListBoxModel {
        
        todo!();
        /*
            return model;
        */
    }

    /**
      | Returns the height of a row in the list.
      | @see setRowHeight
      |
      */
    pub fn get_row_height(&self) -> i32 {
        
        todo!();
        /*
            return rowHeight;
        */
    }

    /**
      | Returns the thickness of outline that
      | will be drawn around the listbox. @see
      | setOutlineColour
      |
      */
    pub fn get_outline_thickness(&self) -> i32 {
        
        todo!();
        /*
            return outlineThickness;
        */
    }

    /**
      | Returns whatever header component
      | was set with setHeaderComponent().
      |
      */
    pub fn get_header_component(&self) -> *mut Component {
        
        todo!();
        /*
            return headerComponent.get();
        */
    }

    /**
      | Creates a ListBox.
      | 
      | The model pointer passed-in can be null,
      | in which case you can set it later with
      | setModel().
      |
      */
    pub fn new(
        name: &String,
        m:    *mut ListBoxModel) -> Self {

        todo!();
        /*
        : component(name),
        : model(m),

            viewport.reset (new ListViewport (*this));
        addAndMakeVisible (viewport.get());

        setWantsKeyboardFocus (true);
        setFocusContainerType (FocusContainerType::focusContainer);
        colourChanged();
        */
    }
    
    /**
      | Changes the current data model to display.
      |
      */
    pub fn set_model(&mut self, new_model: *mut ListBoxModel)  {
        
        todo!();
        /*
            if (model != newModel)
        {
            model = newModel;
            repaint();
            updateContent();
        }
        */
    }
    
    /**
      | Turns on multiple-selection of rows.
      | 
      | By default this is disabled.
      | 
      | When your row component gets clicked
      | you'll need to call the selectRowsBasedOnModifierKeys()
      | method to tell the list that it's been
      | clicked and to get it to do the appropriate
      | selection based on whether the ctrl/shift
      | keys are held down.
      |
      */
    pub fn set_multiple_selection_enabled(&mut self, b: bool)  {
        
        todo!();
        /*
            multipleSelection = b;
        */
    }
    
    /**
      | If enabled, this makes the listbox flip
      | the selection status of each row that
      | the user clicks, without affecting
      | other selected rows.
      | 
      | (This only has an effect if multiple
      | selection is also enabled).
      | 
      | If not enabled, you can still get the
      | same row-flipping behaviour by holding
      | down CMD or CTRL when clicking.
      |
      */
    pub fn set_clicking_toggles_row_selection(&mut self, b: bool)  {
        
        todo!();
        /*
            alwaysFlipSelection = b;
        */
    }
    
    /**
      | Sets whether a row should be selected
      | when the mouse is pressed or released.
      | 
      | By default this is true, but you may want
      | to turn it off.
      |
      */
    pub fn set_row_selected_on_mouse_down(&mut self, b: bool)  {
        
        todo!();
        /*
            selectOnMouseDown = b;
        */
    }
    
    /**
      | Makes the list react to mouse moves by
      | selecting the row that the mouse if over.
      | 
      | This function is here primarily for
      | the ComboBox class to use, but might
      | be useful for some other purpose too.
      |
      */
    pub fn set_mouse_move_selects_rows(&mut self, b: bool)  {
        
        todo!();
        /*
            if (b)
        {
            if (mouseMoveSelector == nullptr)
                mouseMoveSelector.reset (new ListBoxMouseMoveSelector (*this));
        }
        else
        {
            mouseMoveSelector.reset();
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (! hasDoneInitialUpdate)
            updateContent();

        g.fillAll (findColour (backgroundColourId));
        */
    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (outlineThickness > 0)
        {
            g.setColour (findColour (outlineColourId));
            g.drawRect (getLocalBounds(), outlineThickness);
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            viewport->setBoundsInset (BorderSize<int> (outlineThickness + (headerComponent != nullptr ? headerComponent->getHeight() : 0),
                                                   outlineThickness, outlineThickness, outlineThickness));

        viewport->setSingleStepSizes (20, getRowHeight());

        viewport->updateVisibleArea (false);
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            viewport->updateVisibleArea (true);
        */
    }
    
    /**
      | Returns the viewport that this ListBox
      | uses.
      | 
      | You may need to use this to change parameters
      | such as whether scrollbars are shown,
      | etc.
      |
      */
    pub fn get_viewport(&self) -> *mut Viewport {
        
        todo!();
        /*
            return viewport.get();
        */
    }
    
    /**
      | Causes the list to refresh its content.
      | 
      | Call this when the number of rows in the
      | list changes, or if you want it to call
      | refreshComponentForRow() on all the
      | row components.
      | 
      | This must only be called from the main
      | message thread.
      |
      */
    pub fn update_content(&mut self)  {
        
        todo!();
        /*
            hasDoneInitialUpdate = true;
        totalItems = (model != nullptr) ? model->getNumRows() : 0;

        bool selectionChanged = false;

        if (selected.size() > 0 && selected [selected.size() - 1] >= totalItems)
        {
            selected.removeRange ({ totalItems, std::numeric_limits<int>::max() });
            lastRowSelected = getSelectedRow (0);
            selectionChanged = true;
        }

        viewport->updateVisibleArea (isVisible());
        viewport->resized();

        if (selectionChanged)
        {
            if (model != nullptr)
                model->selectedRowsChanged (lastRowSelected);

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
        }
        */
    }
    
    /**
      | Selects a row.
      | 
      | If the row is already selected, this
      | won't do anything.
      | 
      | -----------
      | @param rowNumber
      | 
      | the row to select
      | ----------
      | @param dontScrollToShowThisRow
      | 
      | if true, the list's position won't change;
      | if false and the selected row is off-screen,
      | it'll scroll to make sure that row is
      | on-screen
      | ----------
      | @param deselectOthersFirst
      | 
      | if true and there are multiple selections,
      | these will first be deselected before
      | this item is selected @see isRowSelected,
      | selectRowsBasedOnModifierKeys,
      | flipRowSelection, deselectRow, deselectAllRows,
      | selectRangeOfRows
      |
      */
    pub fn select_row(
        &mut self, 
        row:                   i32,
        dont_scroll:           Option<bool>,
        deselect_others_first: Option<bool>

    ) {

        let dont_scroll:           bool = dont_scroll.unwrap_or(false);
        let deselect_others_first: bool = deselect_others_first.unwrap_or(true);
        
        todo!();
        /*
            selectRowInternal (row, dontScroll, deselectOthersFirst, false);
        */
    }
    
    pub fn select_row_internal(&mut self, 
        row:                   i32,
        dont_scroll:           bool,
        deselect_others_first: bool,
        is_mouse_click:        bool)  {
        
        todo!();
        /*
            if (! multipleSelection)
            deselectOthersFirst = true;

        if ((! isRowSelected (row))
             || (deselectOthersFirst && getNumSelectedRows() > 1))
        {
            if (isPositiveAndBelow (row, totalItems))
            {
                if (deselectOthersFirst)
                    selected.clear();

                selected.addRange ({ row, row + 1 });

                if (getHeight() == 0 || getWidth() == 0)
                    dontScroll = true;

                viewport->selectRow (row, getRowHeight(), dontScroll,
                                     lastRowSelected, totalItems, isMouseClick);

                lastRowSelected = row;
                model->selectedRowsChanged (row);

                if (auto* handler = getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
            }
            else
            {
                if (deselectOthersFirst)
                    deselectAllRows();
            }
        }
        */
    }
    
    /**
      | Deselects a row.
      | 
      | If it's not currently selected, this
      | will do nothing. @see selectRow, deselectAllRows
      |
      */
    pub fn deselect_row(&mut self, row: i32)  {
        
        todo!();
        /*
            if (selected.contains (row))
        {
            selected.removeRange ({ row, row + 1 });

            if (row == lastRowSelected)
                lastRowSelected = getSelectedRow (0);

            viewport->updateContents();
            model->selectedRowsChanged (lastRowSelected);

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
        }
        */
    }
    
    /**
      | Sets the rows that should be selected,
      | based on an explicit set of ranges.
      | 
      | If sendNotificationEventToModel
      | is true, the ListBoxModel::selectedRowsChanged()
      | method will be called. If it's false,
      | no notification will be sent to the model.
      | 
      | @see getSelectedRows
      |
      */
    pub fn set_selected_rows(
        &mut self, 
        set_of_rows_to_be_selected:       &SparseSet<i32>,
        send_notification_event_to_model: Option<NotificationType>

    ) {

        let send_notification_event_to_model
            = send_notification_event_to_model.unwrap_or(NotificationType::sendNotification);
        
        todo!();
        /*
            selected = setOfRowsToBeSelected;
        selected.removeRange ({ totalItems, std::numeric_limits<int>::max() });

        if (! isRowSelected (lastRowSelected))
            lastRowSelected = getSelectedRow (0);

        viewport->updateContents();

        if (model != nullptr && sendNotificationEventToModel == sendNotification)
            model->selectedRowsChanged (lastRowSelected);

        if (auto* handler = getAccessibilityHandler())
            handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
        */
    }
    
    /**
      | Returns a sparse set indicating the
      | rows that are currently selected. @see
      | setSelectedRows
      |
      */
    pub fn get_selected_rows(&self) -> SparseSet<i32> {
        
        todo!();
        /*
            return selected;
        */
    }
    
    /**
      | Selects a set of rows.
      | 
      | This will add these rows to the current
      | selection, so you might need to clear
      | the current selection first with deselectAllRows()
      | 
      | -----------
      | @param firstRow
      | 
      | the first row to select (inclusive)
      | ----------
      | @param lastRow
      | 
      | the last row to select (inclusive)
      | ----------
      | @param dontScrollToShowThisRange
      | 
      | if true, the list's position won't change;
      | if false and the selected range is off-screen,
      | it'll scroll to make sure that the range
      | of rows is on-screen
      |
      */
    pub fn select_range_of_rows(
        &mut self, 
        first_row:                      i32,
        last_row:                       i32,
        dont_scroll_to_show_this_range: Option<bool>

    ) {

        let dont_scroll_to_show_this_range: bool = dont_scroll_to_show_this_range.unwrap_or(false);
        
        todo!();
        /*
            if (multipleSelection && (firstRow != lastRow))
        {
            const int numRows = totalItems - 1;
            firstRow = jlimit (0, jmax (0, numRows), firstRow);
            lastRow  = jlimit (0, jmax (0, numRows), lastRow);

            selected.addRange ({ jmin (firstRow, lastRow),
                                 jmax (firstRow, lastRow) + 1 });

            selected.removeRange ({ lastRow, lastRow + 1 });
        }

        selectRowInternal (lastRow, dontScrollToShowThisRange, false, true);
        */
    }
    
    /**
      | Selects or deselects a row.
      | 
      | If the row's currently selected, this
      | deselects it, and vice-versa.
      |
      */
    pub fn flip_row_selection(&mut self, row: i32)  {
        
        todo!();
        /*
            if (isRowSelected (row))
            deselectRow (row);
        else
            selectRowInternal (row, false, false, true);
        */
    }
    
    /**
      | Deselects any currently selected rows.
      | @see deselectRow
      |
      */
    pub fn deselect_all_rows(&mut self)  {
        
        todo!();
        /*
            if (! selected.isEmpty())
        {
            selected.clear();
            lastRowSelected = -1;

            viewport->updateContents();

            if (model != nullptr)
                model->selectedRowsChanged (lastRowSelected);

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
        }
        */
    }
    
    /**
      | Multiply-selects rows based on the
      | modifier keys.
      | 
      | If no modifier keys are down, this will
      | select the given row and deselect any
      | others.
      | 
      | If the ctrl (or command on the Mac) key
      | is down, it'll flip the state of the selected
      | row.
      | 
      | If the shift key is down, it'll select
      | up to the given row from the last row selected.
      | 
      | @see selectRow
      |
      */
    pub fn select_rows_based_on_modifier_keys(&mut self, 
        row:               i32,
        mods:              ModifierKeys,
        is_mouse_up_event: bool)  {
        
        todo!();
        /*
            if (multipleSelection && (mods.isCommandDown() || alwaysFlipSelection))
        {
            flipRowSelection (row);
        }
        else if (multipleSelection && mods.isShiftDown() && lastRowSelected >= 0)
        {
            selectRangeOfRows (lastRowSelected, row);
        }
        else if ((! mods.isPopupMenu()) || ! isRowSelected (row))
        {
            selectRowInternal (row, false, ! (multipleSelection && (! isMouseUpEvent) && isRowSelected (row)), true);
        }
        */
    }
    
    /**
      | Returns the number of rows that are currently
      | selected. @see getSelectedRow, isRowSelected,
      | getLastRowSelected
      |
      */
    pub fn get_num_selected_rows(&self) -> i32 {
        
        todo!();
        /*
            return selected.size();
        */
    }
    
    /**
      | Returns the row number of a selected
      | row.
      | 
      | This will return the row number of the
      | Nth selected row. The row numbers returned
      | will be sorted in order from low to high.
      | 
      | -----------
      | @param index
      | 
      | the index of the selected row to return,
      | (from 0 to getNumSelectedRows() - 1)
      | 
      | -----------
      | @return
      | 
      | the row number, or -1 if the index was
      | out of range or if there aren't any rows
      | selected @see getNumSelectedRows,
      | isRowSelected, getLastRowSelected
      |
      */
    pub fn get_selected_row(&self, index: Option<i32>) -> i32 {

        let index: i32 = index.unwrap_or(0);
        
        todo!();
        /*
            return (isPositiveAndBelow (index, selected.size()))
                    ? selected [index] : -1;
        */
    }
    
    /**
      | Checks whether a row is selected.
      |
      */
    pub fn is_row_selected(&self, row: i32) -> bool {
        
        todo!();
        /*
            return selected.contains (row);
        */
    }
    
    /**
      | Returns the last row that the user selected.
      | 
      | This isn't the same as the highest row
      | number that is currently selected -
      | if the user had multiply-selected rows
      | 10, 5 and then 6 in that order, this would
      | return 6.
      | 
      | If nothing is selected, it will return
      | -1.
      |
      */
    pub fn get_last_row_selected(&self) -> i32 {
        
        todo!();
        /*
            return isRowSelected (lastRowSelected) ? lastRowSelected : -1;
        */
    }
    
    /**
      | Finds the row index that contains a given
      | x,y position.
      | 
      | The position is relative to the ListBox's
      | top-left.
      | 
      | If no row exists at this position, the
      | method will return -1. @see getComponentForRowNumber
      |
      */
    pub fn get_row_containing_position(&self, x: i32, y: i32) -> i32 {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, getWidth()))
        {
            const int row = (viewport->getViewPositionY() + y - viewport->getY()) / rowHeight;

            if (isPositiveAndBelow (row, totalItems))
                return row;
        }

        return -1;
        */
    }
    
    /**
      | Finds a row index that would be the most
      | suitable place to insert a new item for
      | a given position.
      | 
      | This is useful when the user is e.g. dragging
      | and dropping onto the listbox, because
      | it lets you easily choose the best position
      | to insert the item that they drop, based
      | on where they drop it.
      | 
      | If the position is out of range, this
      | will return -1. If the position is beyond
      | the end of the list, it will return getNumRows()
      | to indicate the end of the list.
      | 
      | @see getComponentForRowNumber
      |
      */
    pub fn get_insertion_index_for_position(&self, x: i32, y: i32) -> i32 {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, getWidth()))
            return jlimit (0, totalItems, (viewport->getViewPositionY() + y + rowHeight / 2 - viewport->getY()) / rowHeight);

        return -1;
        */
    }
    
    /**
      | Finds the row component for a given row
      | in the list.
      | 
      | The component returned will have been
      | created using ListBoxModel::refreshComponentForRow().
      | 
      | If the component for this row is off-screen
      | or if the row is out-of-range, this will
      | return nullptr.
      | 
      | @see getRowContainingPosition
      |
      */
    pub fn get_component_for_row_number(&self, row: i32) -> *mut Component {
        
        todo!();
        /*
            if (auto* listRowComp = viewport->getComponentForRowIfOnscreen (row))
            return listRowComp->customComponent.get();

        return nullptr;
        */
    }
    
    /**
      | Returns the row number that the given
      | component represents.
      | 
      | If the component isn't one of the list's
      | rows, this will return -1.
      |
      */
    pub fn get_row_number_of_component(&self, row_component: *mut Component) -> i32 {
        
        todo!();
        /*
            return viewport->getRowNumberOfComponent (rowComponent);
        */
    }
    
    /**
      | Returns the position of one of the rows,
      | relative to the top-left of the listbox.
      | 
      | This may be off-screen, and the range
      | of the row number that is passed-in is
      | not checked to see if it's a valid row.
      |
      */
    pub fn get_row_position(&self, 
        row_number:                     i32,
        relative_to_component_top_left: bool) -> Rectangle<i32> {
        
        todo!();
        /*
            auto y = viewport->getY() + rowHeight * rowNumber;

        if (relativeToComponentTopLeft)
            y -= viewport->getViewPositionY();

        return { viewport->getX(), y,
                 viewport->getViewedComponent()->getWidth(), rowHeight };
        */
    }
    
    /**
      | Scrolls the list to a particular position.
      | 
      | The proportion is between 0 and 1.0,
      | so 0 scrolls to the top of the list, 1.0
      | scrolls to the bottom.
      | 
      | If the total number of rows all fit onto
      | the screen at once, then this method
      | won't do anything.
      | 
      | @see getVerticalPosition
      |
      */
    pub fn set_vertical_position(&mut self, proportion: f64)  {
        
        todo!();
        /*
            auto offscreen = viewport->getViewedComponent()->getHeight() - viewport->getHeight();

        viewport->setViewPosition (viewport->getViewPositionX(),
                                   jmax (0, roundToInt (proportion * offscreen)));
        */
    }
    
    /**
      | Returns the current vertical position
      | as a proportion of the total.
      | 
      | This can be used in conjunction with
      | setVerticalPosition() to save and
      | restore the list's position. It returns
      | a value in the range 0 to 1.
      | 
      | @see setVerticalPosition
      |
      */
    pub fn get_vertical_position(&self) -> f64 {
        
        todo!();
        /*
            auto offscreen = viewport->getViewedComponent()->getHeight() - viewport->getHeight();

        return offscreen > 0 ? viewport->getViewPositionY() / (double) offscreen
                             : 0;
        */
    }
    
    /**
      | Returns the width of a row (which may
      | be less than the width of this component
      | if there's a scrollbar).
      |
      */
    pub fn get_visible_row_width(&self) -> i32 {
        
        todo!();
        /*
            return viewport->getViewWidth();
        */
    }
    
    /**
      | Scrolls if necessary to make sure that
      | a particular row is visible.
      |
      */
    pub fn scroll_to_ensure_row_is_onscreen(&mut self, row: i32)  {
        
        todo!();
        /*
            viewport->scrollToEnsureRowIsOnscreen (row, getRowHeight());
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            const int numVisibleRows = viewport->getHeight() / getRowHeight();

        const bool multiple = multipleSelection
                                && lastRowSelected >= 0
                                && key.getModifiers().isShiftDown();

        if (key.isKeyCode (KeyPress::upKey))
        {
            if (multiple)
                selectRangeOfRows (lastRowSelected, lastRowSelected - 1);
            else
                selectRow (jmax (0, lastRowSelected - 1));
        }
        else if (key.isKeyCode (KeyPress::downKey))
        {
            if (multiple)
                selectRangeOfRows (lastRowSelected, lastRowSelected + 1);
            else
                selectRow (jmin (totalItems - 1, jmax (0, lastRowSelected + 1)));
        }
        else if (key.isKeyCode (KeyPress::pageUpKey))
        {
            if (multiple)
                selectRangeOfRows (lastRowSelected, lastRowSelected - numVisibleRows);
            else
                selectRow (jmax (0, jmax (0, lastRowSelected) - numVisibleRows));
        }
        else if (key.isKeyCode (KeyPress::pageDownKey))
        {
            if (multiple)
                selectRangeOfRows (lastRowSelected, lastRowSelected + numVisibleRows);
            else
                selectRow (jmin (totalItems - 1, jmax (0, lastRowSelected) + numVisibleRows));
        }
        else if (key.isKeyCode (KeyPress::homeKey))
        {
            if (multiple)
                selectRangeOfRows (lastRowSelected, 0);
            else
                selectRow (0);
        }
        else if (key.isKeyCode (KeyPress::endKey))
        {
            if (multiple)
                selectRangeOfRows (lastRowSelected, totalItems - 1);
            else
                selectRow (totalItems - 1);
        }
        else if (key.isKeyCode (KeyPress::returnKey) && isRowSelected (lastRowSelected))
        {
            if (model != nullptr)
                model->returnKeyPressed (lastRowSelected);
        }
        else if ((key.isKeyCode (KeyPress::deleteKey) || key.isKeyCode (KeyPress::backspaceKey))
                   && isRowSelected (lastRowSelected))
        {
            if (model != nullptr)
                model->deleteKeyPressed (lastRowSelected);
        }
        else if (multipleSelection && key == KeyPress ('a', ModifierKeys::commandModifier, 0))
        {
            selectRangeOfRows (0, std::numeric_limits<int>::max());
        }
        else
        {
            return false;
        }

        return true;
        */
    }
    
    pub fn key_state_changed(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            return isKeyDown
                && (KeyPress::isKeyCurrentlyDown (KeyPress::upKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::pageUpKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::downKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::pageDownKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::homeKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::endKey)
                    || KeyPress::isKeyCurrentlyDown (KeyPress::returnKey));
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            bool eventWasUsed = false;

        if (wheel.deltaX != 0.0f && getHorizontalScrollBar().isVisible())
        {
            eventWasUsed = true;
            getHorizontalScrollBar().mouseWheelMove (e, wheel);
        }

        if (wheel.deltaY != 0.0f && getVerticalScrollBar().isVisible())
        {
            eventWasUsed = true;
            getVerticalScrollBar().mouseWheelMove (e, wheel);
        }

        if (! eventWasUsed)
            Component::mouseWheelMove (e, wheel);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.mouseWasClicked() && model != nullptr)
            model->backgroundClicked (e);
        */
    }
    
    /**
      | Sets the height of each row in the list.
      | 
      | The default height is 22 pixels. @see
      | getRowHeight
      |
      */
    pub fn set_row_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            rowHeight = jmax (1, newHeight);
        viewport->setSingleStepSizes (20, rowHeight);
        updateContent();
        */
    }
    
    /**
      | Returns the number of rows actually
      | visible.
      | 
      | This is the number of whole rows which
      | will fit on-screen, so the value might
      | be more than the actual number of rows
      | in the list.
      |
      */
    pub fn get_num_rows_on_screen(&self) -> i32 {
        
        todo!();
        /*
            return viewport->getMaximumVisibleHeight() / rowHeight;
        */
    }
    
    /**
      | Changes the width of the rows in the list.
      | 
      | This can be used to make the list's row
      | components wider than the list itself
      | - the width of the rows will be either
      | the width of the list or this value, whichever
      | is greater, and if the rows become wider
      | than the list, a horizontal scrollbar
      | will appear.
      | 
      | The default value for this is 0, which
      | means that the rows will always be the
      | same width as the list.
      |
      */
    pub fn set_minimum_content_width(&mut self, new_minimum_width: i32)  {
        
        todo!();
        /*
            minimumRowWidth = newMinimumWidth;
        updateContent();
        */
    }
    
    /**
      | Returns the space currently available
      | for the row items, taking into account
      | borders, scrollbars, etc.
      |
      */
    pub fn get_visible_content_width(&self) -> i32 {
        
        todo!();
        /*
            return viewport->getMaximumVisibleWidth();
        */
    }
    
    /**
      | Returns a reference to the vertical
      | scrollbar.
      |
      */
    pub fn get_vertical_scroll_bar(&self) -> &mut ScrollBar {
        
        todo!();
        /*
            return viewport->getVerticalScrollBar();
        */
    }
    
    /**
      | Returns a reference to the horizontal
      | scrollbar.
      |
      */
    pub fn get_horizontal_scroll_bar(&self) -> &mut ScrollBar {
        
        todo!();
        /*
            return viewport->getHorizontalScrollBar();
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            setOpaque (findColour (backgroundColourId).isOpaque());
        viewport->setOpaque (isOpaque());
        repaint();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            colourChanged();
        */
    }
    
    /**
      | Sets the thickness of a border that will
      | be drawn around the box.
      | 
      | To set the colour of the outline, use
      | @code setColour (ListBox::outlineColourId,
      | colourXYZ); @endcode @see outlineColourId
      |
      */
    pub fn set_outline_thickness(&mut self, new_thickness: i32)  {
        
        todo!();
        /*
            outlineThickness = newThickness;
        resized();
        */
    }
    
    /**
      | Sets a component that the list should
      | use as a header.
      | 
      | This will position the given component
      | at the top of the list, maintaining the
      | height of the component passed-in,
      | but rescaling it horizontally to match
      | the width of the items in the listbox.
      | 
      | The component will be deleted when setHeaderComponent()
      | is called with a different component,
      | or when the listbox is deleted.
      |
      */
    pub fn set_header_component(&mut self, new_header_component: Box<Component>)  {
        
        todo!();
        /*
            headerComponent = std::move (newHeaderComponent);
        addAndMakeVisible (headerComponent.get());
        ListBox::resized();
        invalidateAccessibilityHandler();
        */
    }
    
    pub fn has_accessible_header_component(&self) -> bool {
        
        todo!();
        /*
            return headerComponent != nullptr
                && headerComponent->getAccessibilityHandler() != nullptr;
        */
    }
    
    /**
      | Repaints one of the rows.
      | 
      | This does not invoke updateContent(),
      | it just invokes a straightforward repaint
      | for the area covered by this row.
      |
      */
    pub fn repaint_row(&mut self, row_number: i32)  {
        
        todo!();
        /*
            repaint (getRowPosition (rowNumber, true));
        */
    }
    
    pub fn create_snapshot_of_rows(&mut self, 
        rows:   &SparseSet<i32>,
        imagex: &mut i32,
        imagey: &mut i32) -> Image {
        
        todo!();
        /*
            Rectangle<int> imageArea;
        auto firstRow = getRowContainingPosition (0, viewport->getY());

        for (int i = getNumRowsOnScreen() + 2; --i >= 0;)
        {
            if (rows.contains (firstRow + i))
            {
                if (auto* rowComp = viewport->getComponentForRowIfOnscreen (firstRow + i))
                {
                    auto pos = getLocalPoint (rowComp, Point<int>());

                    imageArea = imageArea.getUnion ({ pos.x, pos.y, rowComp->getWidth(), rowComp->getHeight() });
                }
            }
        }

        imageArea = imageArea.getIntersection (getLocalBounds());
        imageX = imageArea.getX();
        imageY = imageArea.getY();

        auto listScale = Component::getApproximateScaleFactorForComponent (this);
        Image snapshot (Image::ARGB,
                        roundToInt ((float) imageArea.getWidth() * listScale),
                        roundToInt ((float) imageArea.getHeight() * listScale),
                        true);

        for (int i = getNumRowsOnScreen() + 2; --i >= 0;)
        {
            if (rows.contains (firstRow + i))
            {
                if (auto* rowComp = viewport->getComponentForRowIfOnscreen (firstRow + i))
                {
                    Graphics g (snapshot);
                    g.setOrigin (getLocalPoint (rowComp, Point<int>()) - imageArea.getPosition());

                    auto rowScale = Component::getApproximateScaleFactorForComponent (rowComp);

                    if (g.reduceClipRegion (rowComp->getLocalBounds() * rowScale))
                    {
                        g.beginTransparencyLayer (0.6f);
                        g.addTransform (AffineTransform::scale (rowScale));
                        rowComp->paintEntireComponent (g, false);
                        g.endTransparencyLayer();
                    }
                }
            }
        }

        return snapshot;
        */
    }
    
    pub fn start_drag_and_drop(
        &mut self, 
        e:                               &MouseEvent,
        rows_to_drag:                    &SparseSet<i32>,
        drag_description:                &Var,
        allow_dragging_to_other_windows: bool

    ) {
        
        todo!();
        /*
            if (auto* dragContainer = DragAndDropContainer::findParentDragContainerFor (this))
        {
            int x, y;
            auto dragImage = createSnapshotOfRows (rowsToDrag, x, y);

            auto p = Point<int> (x, y) - e.getEventRelativeTo (this).position.toInt();
            dragContainer->startDragging (dragDescription, this, dragImage, allowDraggingToOtherWindows, &p, &e.source);
        }
        else
        {
            // to be able to do a drag-and-drop operation, the listbox needs to
            // be inside a component which is also a DragAndDropContainer.
            jassertfalse;
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class TableInterface  : public AccessibilityTableInterface
        {
        
            explicit TableInterface (ListBox& listBoxToWrap)
                : listBox (listBoxToWrap)
            {
            }

            int getNumRows() const override
            {
                if (listBox.model == nullptr)
                    return 0;

                const auto numRows = listBox.model->getNumRows();

                if (listBox.hasAccessibleHeaderComponent())
                    return numRows + 1;

                return numRows;
            }

            int getNumColumns() const override
            {
                return 1;
            }

            const AccessibilityHandler* getCellHandler (int row, int) const override
            {
                if (auto* headerHandler = getHeaderHandler())
                {
                    if (row == 0)
                        return headerHandler;

                    --row;
                }

                if (auto* rowComponent = listBox.viewport->getComponentForRow (row))
                    return rowComponent->getAccessibilityHandler();

                return nullptr;
            }

        
            const AccessibilityHandler* getHeaderHandler() const
            {
                if (listBox.hasAccessibleHeaderComponent())
                    return listBox.headerComponent->getAccessibilityHandler();

                return nullptr;
            }

            ListBox& listBox;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (TableInterface)
        };

        return std::make_unique<AccessibilityHandler> (*this,
                                                       AccessibilityRole::list,
                                                       AccessibilityActions{},
                                                       AccessibilityHandler::Interfaces { std::make_unique<TableInterface> (*this) });
        */
    }
}
