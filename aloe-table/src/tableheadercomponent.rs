crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TableHeaderComponent.h]
pub trait TableHeaderComponentInterface: 
    ColumnClicked 
    + AddMenuItems
    + ReactToMenuItem
    + ShowColumnChooserMenu {}

/**
  | A component that displays a strip of
  | column headings for a table, and allows
  | these to be resized, dragged around,
  | etc.
  | 
  | This is just the component that goes
  | at the top of a table. You can use it directly
  | for custom components, or to create
  | a simple table, use the
  | 
  | TableListBox class.
  | 
  | To use one of these, create it and use
  | addColumn() to add all the columns that
  | you need.
  | 
  | Each column must be given a unique ID
  | number that's used to refer to it.
  | 
  | @see TableListBox, TableHeaderComponent::TableHeaderComponentListener
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TableHeaderComponent<'a> {
    base:                           Component<'a>,
    base2:                          AsyncUpdater<'a>,
    columns:                        Vec<Box<TableHeaderComponentColumnInfo>>,
    listeners:                      Vec<*mut dyn TableHeaderComponentListener>,
    drag_overlay_comp:              Box<Component<'a>>,
    columns_changed:                bool, // default = false
    columns_resized:                bool, // default = false
    sort_changed:                   bool, // default = false
    menu_active:                    bool, // default = true
    stretch_to_fit:                 bool, // default = false
    column_id_being_resized:        i32, // default = 0
    column_id_being_dragged:        i32, // default = 0
    initial_column_width:           i32, // default = 0
    column_id_under_mouse:          i32, // default = 0
    dragging_column_offset:         i32, // default = 0
    dragging_column_original_index: i32, // default = 0
    last_deliberate_width:          i32, // default = 0
}

impl<'a> Default for TableHeaderComponent<'a> {
    
    /**
      | Creates an empty table header.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TableHeaderComponent.cpp]
impl<'a> Drop for TableHeaderComponent<'a> {

    fn drop(&mut self) {

        todo!();
        /*
            dragOverlayComp.reset();
        */
    }
}

impl<'a> TableHeaderComponent<'a> {

    /**
      | Enables or disables the pop-up menu.
      | 
      | The default menu allows the user to show
      | or hide columns. You can add custom items
      | to this menu by overloading the addMenuItems()
      | and reactToMenuItem() methods.
      | 
      | By default the menu is enabled.
      | 
      | @see isPopupMenuActive, addMenuItems,
      | reactToMenuItem
      |
      */
    pub fn set_popup_menu_active(&mut self, has_menu: bool)  {
        
        todo!();
        /*
            menuActive = hasMenu;
        */
    }
    
    /**
      | Returns true if the pop-up menu is enabled.
      | @see setPopupMenuActive
      |
      */
    pub fn is_popup_menu_active(&self) -> bool {
        
        todo!();
        /*
            return menuActive;
        */
    }
    
    /**
      | Returns the number of columns in the
      | table.
      | 
      | If onlyCountVisibleColumns is true,
      | this will return the number of visible
      | columns; otherwise it'll return the
      | total number of columns, including
      | hidden ones.
      | 
      | @see isColumnVisible
      |
      */
    pub fn get_num_columns(&self, only_count_visible_columns: bool) -> i32 {
        
        todo!();
        /*
            if (onlyCountVisibleColumns)
        {
            int num = 0;

            for (auto* c : columns)
                if (c->isVisible())
                    ++num;

            return num;
        }

        return columns.size();
        */
    }
    
    /**
      | Returns the name for a column. @see setColumnName
      |
      */
    pub fn get_column_name(&self, column_id: i32) -> String {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
            return ci->name;

        return {};
        */
    }
    
    /**
      | Changes the name of a column.
      |
      */
    pub fn set_column_name(&mut self, 
        column_id: i32,
        new_name:  &String)  {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
        {
            if (ci->name != newName)
            {
                ci->name = newName;
                sendColumnsChanged();
            }
        }
        */
    }
    
    /**
      | Adds a column to the table.
      | 
      | This will add a column, and asynchronously
      | call the tableColumnsChanged() method
      | of any registered listeners.
      | 
      | -----------
      | @param columnName
      | 
      | the name of the new column. It's ok to
      | have two or more columns with the same
      | name
      | ----------
      | @param columnId
      | 
      | an ID for this column. The ID can be any
      | number apart from 0, but every column
      | must have a unique ID. This is used to
      | identify the column later on, after
      | the user may have changed the order that
      | they appear in
      | ----------
      | @param width
      | 
      | the initial width of the column, in pixels
      | ----------
      | @param maximumWidth
      | 
      | a maximum width that the column can take
      | when the user is resizing it. This only
      | applies if the 'resizable' flag is specified
      | for this column
      | ----------
      | @param minimumWidth
      | 
      | a minimum width that the column can take
      | when the user is resizing it. This only
      | applies if the 'resizable' flag is specified
      | for this column
      | ----------
      | @param propertyFlags
      | 
      | a combination of some of the values from
      | the TableHeaderComponentColumnPropertyFlags enum, to define
      | the properties of this column
      | ----------
      | @param insertIndex
      | 
      | the index at which the column should
      | be added. A value of 0 puts it at the start
      | (left-hand side) and -1 puts it at the
      | end (right-hand size) of the table.
      | Note that the index the index within
      | all columns, not just the index amongst
      | those that are currently visible
      |
      */
    pub fn add_column(
        &mut self, 
        column_name:    &String,
        column_id:      i32,
        width:          i32,
        minimum_width:  Option<i32>,
        maximum_width:  Option<i32>,
        property_flags: Option<TableHeaderComponentColumnPropertyFlags>,
        insert_index:   Option<i32>
    )  {

        let minimum_width:  i32 = minimum_width.unwrap_or(30);
        let maximum_width:  i32 = maximum_width.unwrap_or(-1);
        let insert_index:   i32 = insert_index.unwrap_or(-1);

        let property_flags = property_flags.unwrap_or(TableHeaderComponentColumnPropertyFlags::DEFAULT_FLAGS);
        
        todo!();
        /*
            // can't have a duplicate or zero ID!
        jassert (columnId != 0 && getIndexOfColumnId (columnId, false) < 0);
        jassert (width > 0);

        auto ci = new TableHeaderComponentColumnInfo();
        ci->name = columnName;
        ci->id = columnId;
        ci->width = width;
        ci->lastDeliberateWidth = width;
        ci->minimumWidth = minimumWidth;
        ci->maximumWidth = maximumWidth >= 0 ? maximumWidth : std::numeric_limits<int>::max();
        jassert (ci->maximumWidth >= ci->minimumWidth);
        ci->propertyFlags = propertyFlags;

        columns.insert (insertIndex, ci);
        sendColumnsChanged();
        */
    }
    
    /**
      | Removes a column with the given ID.
      | 
      | If there is such a column, this will asynchronously
      | call the tableColumnsChanged() method
      | of any registered listeners.
      |
      */
    pub fn remove_column(&mut self, column_id_to_remove: i32)  {
        
        todo!();
        /*
            auto index = getIndexOfColumnId (columnIdToRemove, false);

        if (index >= 0)
        {
            columns.remove (index);
            sortChanged = true;
            sendColumnsChanged();
        }
        */
    }
    
    /**
      | Deletes all columns from the table.
      | 
      | If there are any columns to remove, this
      | will asynchronously call the tableColumnsChanged()
      | method of any registered listeners.
      |
      */
    pub fn remove_all_columns(&mut self)  {
        
        todo!();
        /*
            if (columns.size() > 0)
        {
            columns.clear();
            sendColumnsChanged();
        }
        */
    }
    
    /**
      | Moves a column to a different index in
      | the table.
      | 
      | -----------
      | @param columnId
      | 
      | the column to move
      | ----------
      | @param newVisibleIndex
      | 
      | the target index for it, from 0 to the
      | number of columns currently visible.
      |
      */
    pub fn move_column(&mut self, 
        column_id: i32,
        new_index: i32)  {
        
        todo!();
        /*
            auto currentIndex = getIndexOfColumnId (columnId, false);
        newIndex = visibleIndexToTotalIndex (newIndex);

        if (columns[currentIndex] != nullptr && currentIndex != newIndex)
        {
            columns.move (currentIndex, newIndex);
            sendColumnsChanged();
        }
        */
    }
    
    /**
      | Returns the width of one of the columns.
      |
      */
    pub fn get_column_width(&self, column_id: i32) -> i32 {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
            return ci->width;

        return 0;
        */
    }
    
    /**
      | Changes the width of a column.
      | 
      | This will cause an asynchronous callback
      | to the tableColumnsResized() method
      | of any registered listeners.
      |
      */
    pub fn set_column_width(&mut self, 
        column_id: i32,
        new_width: i32)  {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
        {
            if (ci->width != newWidth)
            {
                auto numColumns = getNumColumns (true);

                ci->lastDeliberateWidth = ci->width
                    = jlimit (ci->minimumWidth, ci->maximumWidth, newWidth);

                if (stretchToFit)
                {
                    auto index = getIndexOfColumnId (columnId, true) + 1;

                    if (isPositiveAndBelow (index, numColumns))
                    {
                        auto x = getColumnPosition (index).getX();

                        if (lastDeliberateWidth == 0)
                            lastDeliberateWidth = getTotalWidth();

                        resizeColumnsToFit (visibleIndexToTotalIndex (index), lastDeliberateWidth - x);
                    }
                }

                repaint();
                columnsResized = true;
                triggerAsyncUpdate();
            }
        }
        */
    }
    
    /**
      | Returns the index of a given column.
      | 
      | If there's no such column ID, this will
      | return -1.
      | 
      | If onlyCountVisibleColumns is true,
      | this will return the index amongst the
      | visible columns; otherwise it'll return
      | the index amongst all the columns, including
      | any hidden ones.
      |
      */
    pub fn get_index_of_column_id(&self, 
        column_id:                  i32,
        only_count_visible_columns: bool) -> i32 {
        
        todo!();
        /*
            int n = 0;

        for (auto* c : columns)
        {
            if ((! onlyCountVisibleColumns) || c->isVisible())
            {
                if (c->id == columnId)
                    return n;

                ++n;
            }
        }

        return -1;
        */
    }
    
    /**
      | Returns the ID of the column at a given
      | index.
      | 
      | If onlyCountVisibleColumns is true,
      | this will count the index amongst the
      | visible columns; otherwise it'll count
      | it amongst all the columns, including
      | any hidden ones.
      | 
      | If the index is out-of-range, it'll
      | return 0.
      |
      */
    pub fn get_column_id_of_index(&self, 
        index:                      i32,
        only_count_visible_columns: bool) -> i32 {
        
        todo!();
        /*
            if (onlyCountVisibleColumns)
            index = visibleIndexToTotalIndex (index);

        if (auto* ci = columns [index])
            return ci->id;

        return 0;
        */
    }
    
    /**
      | Returns the rectangle containing of
      | one of the columns.
      | 
      | The index is an index from 0 to the number
      | of columns that are currently visible
      | (hidden ones are not counted). It returns
      | a rectangle showing the position of
      | the column relative to this component's
      | top-left. If the index is out-of-range,
      | an empty rectangle is returned.
      |
      */
    pub fn get_column_position(&self, index: i32) -> Rectangle<i32> {
        
        todo!();
        /*
            int x = 0, width = 0, n = 0;

        for (auto* c : columns)
        {
            x += width;

            if (c->isVisible())
            {
                width = c->width;

                if (n++ == index)
                    break;
            }
            else
            {
                width = 0;
            }
        }

        return { x, 0, width, getHeight() };
        */
    }
    
    /**
      | Finds the column ID at a given x-position
      | in the component.
      | 
      | If there is a column at this point this
      | returns its ID, or if not, it will return
      | 0.
      |
      */
    pub fn get_column_id_atx(&self, x_to_find: i32) -> i32 {
        
        todo!();
        /*
            if (xToFind >= 0)
        {
            int x = 0;

            for (auto* ci : columns)
            {
                if (ci->isVisible())
                {
                    x += ci->width;

                    if (xToFind < x)
                        return ci->id;
                }
            }
        }

        return 0;
        */
    }
    
    /**
      | Returns the total width of all the visible
      | columns in the table.
      |
      */
    pub fn get_total_width(&self) -> i32 {
        
        todo!();
        /*
            int w = 0;

        for (auto* c : columns)
            if (c->isVisible())
                w += c->width;

        return w;
        */
    }
    
    /**
      | If set to true, this indicates that the
      | columns should be expanded or shrunk
      | to fill the entire width of the component.
      | 
      | By default this is disabled. Turning
      | it on also means that when resizing a
      | column, those on the right will be squashed
      | to fit.
      |
      */
    pub fn set_stretch_to_fit_active(&mut self, should_stretch_to_fit: bool)  {
        
        todo!();
        /*
            stretchToFit = shouldStretchToFit;
        lastDeliberateWidth = getTotalWidth();
        resized();
        */
    }
    
    /**
      | Returns true if stretch-to-fit has
      | been enabled. @see setStretchToFitActive
      |
      */
    pub fn is_stretch_to_fit_active(&self) -> bool {
        
        todo!();
        /*
            return stretchToFit;
        */
    }
    
    /**
      | If stretch-to-fit is enabled, this
      | will resize all the columns to make them
      | fit into the specified width, keeping
      | their relative proportions the same.
      | 
      | If the minimum widths of the columns
      | are too wide to fit into this space, it
      | may actually end up wider.
      |
      */
    pub fn resize_all_columns_to_fit(&mut self, target_total_width: i32)  {
        
        todo!();
        /*
            if (stretchToFit && getWidth() > 0
             && columnIdBeingResized == 0 && columnIdBeingDragged == 0)
        {
            lastDeliberateWidth = targetTotalWidth;
            resizeColumnsToFit (0, targetTotalWidth);
        }
        */
    }
    
    pub fn resize_columns_to_fit(&mut self, 
        first_column_index: i32,
        target_total_width: i32)  {
        
        todo!();
        /*
            targetTotalWidth = jmax (targetTotalWidth, 0);
        StretchableObjectResizer sor;

        for (int i = firstColumnIndex; i < columns.size(); ++i)
        {
            auto* ci = columns.getUnchecked(i);

            if (ci->isVisible())
                sor.addItem (ci->lastDeliberateWidth, ci->minimumWidth, ci->maximumWidth);
        }

        sor.resizeToFit (targetTotalWidth);
        int visIndex = 0;

        for (int i = firstColumnIndex; i < columns.size(); ++i)
        {
            auto* ci = columns.getUnchecked(i);

            if (ci->isVisible())
            {
                auto newWidth = jlimit (ci->minimumWidth, ci->maximumWidth,
                                        (int) std::floor (sor.getItemSize (visIndex++)));

                if (newWidth != ci->width)
                {
                    ci->width = newWidth;
                    repaint();
                    columnsResized = true;
                    triggerAsyncUpdate();
                }
            }
        }
        */
    }
    
    /**
      | Shows or hides a column.
      | 
      | This can cause an asynchronous callback
      | to the tableColumnsChanged() method
      | of any registered listeners. @see isColumnVisible
      |
      */
    pub fn set_column_visible(&mut self, 
        column_id:         i32,
        should_be_visible: bool)  {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
        {
            if (shouldBeVisible != ci->isVisible())
            {
                if (shouldBeVisible)
                    ci->propertyFlags |= visible;
                else
                    ci->propertyFlags &= ~visible;

                sendColumnsChanged();
                resized();
            }
        }
        */
    }
    
    /**
      | Returns true if this column is currently
      | visible. @see setColumnVisible
      |
      */
    pub fn is_column_visible(&self, column_id: i32) -> bool {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
            return ci->isVisible();

        return false;
        */
    }
    
    /**
      | Changes the column which is the sort
      | column.
      | 
      | This can cause an asynchronous callback
      | to the tableSortOrderChanged() method
      | of any registered listeners.
      | 
      | If this method doesn't actually change
      | the column ID, then no re-sort will take
      | place (you can call reSortTable() to
      | force a re-sort to happen if you've modified
      | the table's contents).
      | 
      | @see getSortColumnId, isSortedForwards,
      | reSortTable
      |
      */
    pub fn set_sort_column_id(&mut self, 
        column_id:     i32,
        sort_forwards: bool)  {
        
        todo!();
        /*
            if (getSortColumnId() != columnId || isSortedForwards() != sortForwards)
        {
            for (auto* c : columns)
                c->propertyFlags &= ~(sortedForwards | sortedBackwards);

            if (auto* ci = getInfoForId (columnId))
                ci->propertyFlags |= (sortForwards ? sortedForwards : sortedBackwards);

            reSortTable();
        }
        */
    }
    
    /**
      | Returns the column ID by which the table
      | is currently sorted, or 0 if it is unsorted.
      | 
      | @see setSortColumnId, isSortedForwards
      |
      */
    pub fn get_sort_column_id(&self) -> i32 {
        
        todo!();
        /*
            for (auto* c : columns)
            if ((c->propertyFlags & (sortedForwards | sortedBackwards)) != 0)
                return c->id;

        return 0;
        */
    }
    
    /**
      | Returns true if the table is currently
      | sorted forwards, or false if it's backwards.
      | @see setSortColumnId
      |
      */
    pub fn is_sorted_forwards(&self) -> bool {
        
        todo!();
        /*
            for (auto* c : columns)
            if ((c->propertyFlags & (sortedForwards | sortedBackwards)) != 0)
                return (c->propertyFlags & sortedForwards) != 0;

        return true;
        */
    }
    
    /**
      | Triggers a re-sort of the table according
      | to the current sort-column.
      | 
      | If you modify the table's contents,
      | you can call this to signal that the table
      | needs to be re-sorted.
      | 
      | (This doesn't do any sorting synchronously
      | - it just asynchronously sends a call
      | to the tableSortOrderChanged() method
      | of any listeners).
      |
      */
    pub fn re_sort_table(&mut self)  {
        
        todo!();
        /*
            sortChanged = true;
        repaint();
        triggerAsyncUpdate();
        */
    }
    
    /**
      | Returns a string that encapsulates
      | the table's current layout.
      | 
      | This can be restored later using restoreFromString().
      | It saves the order of the columns, the
      | currently-sorted column, and the widths.
      | 
      | @see restoreFromString
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            String s;

        XmlElement doc ("TABLELAYOUT");

        doc.setAttribute ("sortedCol", getSortColumnId());
        doc.setAttribute ("sortForwards", isSortedForwards());

        for (auto* ci : columns)
        {
            auto* e = doc.createNewChildElement ("COLUMN");
            e->setAttribute ("id", ci->id);
            e->setAttribute ("visible", ci->isVisible());
            e->setAttribute ("width", ci->width);
        }

        return doc.toString (XmlElement::TextFormat().singleLine().withoutHeader());
        */
    }
    
    /**
      | Restores the state of the table, based
      | on a string previously created with
      | toString().
      | 
      | @see toString
      |
      */
    pub fn restore_from_string(&mut self, stored_version: &String)  {
        
        todo!();
        /*
            if (auto storedXML = parseXMLIfTagMatches (storedVersion, "TABLELAYOUT"))
        {
            int index = 0;

            for (auto* col : storedXML->getChildIterator())
            {
                auto tabId = col->getIntAttribute ("id");

                if (auto* ci = getInfoForId (tabId))
                {
                    columns.move (columns.indexOf (ci), index);
                    ci->width = col->getIntAttribute ("width");
                    setColumnVisible (tabId, col->getBoolAttribute ("visible"));
                }

                ++index;
            }

            columnsResized = true;
            sendColumnsChanged();

            setSortColumnId (storedXML->getIntAttribute ("sortedCol"),
                             storedXML->getBoolAttribute ("sortForwards", true));
        }
        */
    }
    
    /**
      | Adds a listener to be informed about
      | things that happen to the header.
      |
      */
    pub fn add_listener(&mut self, new_listener: *mut dyn TableHeaderComponentListener)  {
        
        todo!();
        /*
            listeners.addIfNotAlreadyThere (newListener);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn TableHeaderComponentListener)  {
        
        todo!();
        /*
            listeners.removeFirstMatchingValue (listenerToRemove);
        */
    }
    
    pub fn column_clicked(&mut self, 
        column_id: i32,
        mods:      &ModifierKeys)  {
        
        todo!();
        /*
            if (auto* ci = getInfoForId (columnId))
            if ((ci->propertyFlags & sortable) != 0 && ! mods.isPopupMenu())
                setSortColumnId (columnId, (ci->propertyFlags & sortedForwards) == 0);
        */
    }
    
    pub fn add_menu_items(
        &mut self, 
        menu:              &mut PopupMenu,
        column_id_clicked: i32

    ) {
        
        todo!();
        /*
            for (auto* ci : columns)
            if ((ci->propertyFlags & appearsOnColumnMenu) != 0)
                menu.addItem (ci->id, ci->name,
                              (ci->propertyFlags & (sortedForwards | sortedBackwards)) == 0,
                              isColumnVisible (ci->id));
        */
    }
    
    pub fn react_to_menu_item(&mut self, 
        menu_return_id:    i32,
        column_id_clicked: i32)  {
        
        todo!();
        /*
            if (getIndexOfColumnId (menuReturnId, false) >= 0)
            setColumnVisible (menuReturnId, ! isColumnVisible (menuReturnId));
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        lf.drawTableHeaderBackground (g, *this);

        auto clip = g.getClipBounds();

        int x = 0;

        for (auto* ci : columns)
        {
            if (ci->isVisible())
            {
                if (x + ci->width > clip.getX()
                     && (ci->id != columnIdBeingDragged
                          || dragOverlayComp == nullptr
                          || ! dragOverlayComp->isVisible()))
                {
                    Graphics::ScopedSaveState ss (g);

                    g.setOrigin (x, 0);
                    g.reduceClipRegion (0, 0, ci->width, getHeight());

                    lf.drawTableHeaderColumn (g, *this, ci->name, ci->id, ci->width, getHeight(),
                                              ci->id == columnIdUnderMouse,
                                              ci->id == columnIdUnderMouse && isMouseButtonDown(),
                                              ci->propertyFlags);
                }

                x += ci->width;

                if (x >= clip.getRight())
                    break;
            }
        }
        */
    }
    
    pub fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateColumnUnderMouse (e);
        */
    }
    
    pub fn mouse_enter(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateColumnUnderMouse (e);
        */
    }
    
    pub fn mouse_exit(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            setColumnUnderMouse (0);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            repaint();
        columnIdBeingResized = 0;
        columnIdBeingDragged = 0;

        if (columnIdUnderMouse != 0)
        {
            draggingColumnOffset = e.x - getColumnPosition (getIndexOfColumnId (columnIdUnderMouse, true)).getX();

            if (e.mods.isPopupMenu())
                columnClicked (columnIdUnderMouse, e.mods);
        }

        if (menuActive && e.mods.isPopupMenu())
            showColumnChooserMenu (columnIdUnderMouse);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (columnIdBeingResized == 0
             && columnIdBeingDragged == 0
             && e.mouseWasDraggedSinceMouseDown()
             && ! e.mods.isPopupMenu())
        {
            dragOverlayComp.reset();

            columnIdBeingResized = getResizeDraggerAt (e.getMouseDownX());

            if (columnIdBeingResized != 0)
            {
                if (auto* ci = getInfoForId (columnIdBeingResized))
                    initialColumnWidth = ci->width;
                else
                    jassertfalse;
            }
            else
            {
                beginDrag (e);
            }
        }

        if (columnIdBeingResized != 0)
        {
            if (auto* ci = getInfoForId (columnIdBeingResized))
            {
                auto w = jlimit (ci->minimumWidth, ci->maximumWidth,
                                 initialColumnWidth + e.getDistanceFromDragStartX());

                if (stretchToFit)
                {
                    // prevent us dragging a column too far right if we're in stretch-to-fit mode
                    int minWidthOnRight = 0;

                    for (int i = getIndexOfColumnId (columnIdBeingResized, false) + 1; i < columns.size(); ++i)
                        if (columns.getUnchecked (i)->isVisible())
                            minWidthOnRight += columns.getUnchecked (i)->minimumWidth;

                    auto currentPos = getColumnPosition (getIndexOfColumnId (columnIdBeingResized, true));
                    w = jmax (ci->minimumWidth, jmin (w, lastDeliberateWidth - minWidthOnRight - currentPos.getX()));
                }

                setColumnWidth (columnIdBeingResized, w);
            }
        }
        else if (columnIdBeingDragged != 0)
        {
            if (e.y >= -50 && e.y < getHeight() + 50)
            {
                if (dragOverlayComp != nullptr)
                {
                    dragOverlayComp->setVisible (true);
                    dragOverlayComp->setBounds (jlimit (0,
                                                        jmax (0, getTotalWidth() - dragOverlayComp->getWidth()),
                                                        e.x - draggingColumnOffset),
                                                0,
                                                dragOverlayComp->getWidth(),
                                                getHeight());

                    for (int i = columns.size(); --i >= 0;)
                    {
                        const int currentIndex = getIndexOfColumnId (columnIdBeingDragged, true);
                        int newIndex = currentIndex;

                        if (newIndex > 0)
                        {
                            // if the previous column isn't draggable, we can't move our column
                            // past it, because that'd change the undraggable column's position..
                            auto* previous = columns.getUnchecked (newIndex - 1);

                            if ((previous->propertyFlags & draggable) != 0)
                            {
                                auto leftOfPrevious = getColumnPosition (newIndex - 1).getX();
                                auto rightOfCurrent = getColumnPosition (newIndex).getRight();

                                if (std::abs (dragOverlayComp->getX() - leftOfPrevious)
                                     < std::abs (dragOverlayComp->getRight() - rightOfCurrent))
                                {
                                    --newIndex;
                                }
                            }
                        }

                        if (newIndex < columns.size() - 1)
                        {
                            // if the next column isn't draggable, we can't move our column
                            // past it, because that'd change the undraggable column's position..
                            auto* nextCol = columns.getUnchecked (newIndex + 1);

                            if ((nextCol->propertyFlags & draggable) != 0)
                            {
                                auto leftOfCurrent = getColumnPosition (newIndex).getX();
                                auto rightOfNext = getColumnPosition (newIndex + 1).getRight();

                                if (std::abs (dragOverlayComp->getX() - leftOfCurrent)
                                     > std::abs (dragOverlayComp->getRight() - rightOfNext))
                                {
                                    ++newIndex;
                                }
                            }
                        }

                        if (newIndex != currentIndex)
                            moveColumn (columnIdBeingDragged, newIndex);
                        else
                            break;
                    }
                }
            }
            else
            {
                endDrag (draggingColumnOriginalIndex);
            }
        }
        */
    }
    
    pub fn begin_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (columnIdBeingDragged == 0)
        {
            columnIdBeingDragged = getColumnIdAtX (e.getMouseDownX());

            auto* ci = getInfoForId (columnIdBeingDragged);

            if (ci == nullptr || (ci->propertyFlags & draggable) == 0)
            {
                columnIdBeingDragged = 0;
            }
            else
            {
                draggingColumnOriginalIndex = getIndexOfColumnId (columnIdBeingDragged, true);

                auto columnRect = getColumnPosition (draggingColumnOriginalIndex);
                auto temp = columnIdBeingDragged;
                columnIdBeingDragged = 0;

                dragOverlayComp.reset (new TableHeaderComponentDragOverlayComp (createComponentSnapshot (columnRect, false)));
                addAndMakeVisible (dragOverlayComp.get());
                columnIdBeingDragged = temp;

                dragOverlayComp->setBounds (columnRect);

                for (int i = listeners.size(); --i >= 0;)
                {
                    listeners.getUnchecked(i)->tableColumnDraggingChanged (this, columnIdBeingDragged);
                    i = jmin (i, listeners.size() - 1);
                }
            }
        }
        */
    }
    
    pub fn end_drag(&mut self, final_index: i32)  {
        
        todo!();
        /*
            if (columnIdBeingDragged != 0)
        {
            moveColumn (columnIdBeingDragged, finalIndex);

            columnIdBeingDragged = 0;
            repaint();

            for (int i = listeners.size(); --i >= 0;)
            {
                listeners.getUnchecked(i)->tableColumnDraggingChanged (this, 0);
                i = jmin (i, listeners.size() - 1);
            }
        }
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDrag (e);

        for (auto* c : columns)
            if (c->isVisible())
                c->lastDeliberateWidth = c->width;

        columnIdBeingResized = 0;
        repaint();

        endDrag (getIndexOfColumnId (columnIdBeingDragged, true));

        updateColumnUnderMouse (e);

        if (columnIdUnderMouse != 0 && ! (e.mouseWasDraggedSinceMouseDown() || e.mods.isPopupMenu()))
            columnClicked (columnIdUnderMouse, e.mods);

        dragOverlayComp.reset();
        */
    }
    
    pub fn get_mouse_cursor(&mut self) -> MouseCursor {
        
        todo!();
        /*
            if (columnIdBeingResized != 0 || (getResizeDraggerAt (getMouseXYRelative().getX()) != 0 && ! isMouseButtonDown()))
            return MouseCursor (MouseCursor::LeftRightResizeCursor);

        return Component::getMouseCursor();
        */
    }
    
    pub fn get_info_for_id(&self, id: i32) -> *mut TableHeaderComponentColumnInfo {
        
        todo!();
        /*
            for (auto* c : columns)
            if (c->id == id)
                return c;

        return nullptr;
        */
    }
    
    pub fn visible_index_to_total_index(&self, visible_index: i32) -> i32 {
        
        todo!();
        /*
            int n = 0;

        for (int i = 0; i < columns.size(); ++i)
        {
            if (columns.getUnchecked(i)->isVisible())
            {
                if (n == visibleIndex)
                    return i;

                ++n;
            }
        }

        return -1;
        */
    }
    
    pub fn send_columns_changed(&mut self)  {
        
        todo!();
        /*
            if (stretchToFit && lastDeliberateWidth > 0)
            resizeAllColumnsToFit (lastDeliberateWidth);

        repaint();
        columnsChanged = true;
        triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            const bool changed = columnsChanged || sortChanged;
        const bool sized = columnsResized || changed;
        const bool sorted = sortChanged;
        columnsChanged = false;
        columnsResized = false;
        sortChanged = false;

        if (sorted)
        {
            for (int i = listeners.size(); --i >= 0;)
            {
                listeners.getUnchecked(i)->tableSortOrderChanged (this);
                i = jmin (i, listeners.size() - 1);
            }
        }

        if (changed)
        {
            for (int i = listeners.size(); --i >= 0;)
            {
                listeners.getUnchecked(i)->tableColumnsChanged (this);
                i = jmin (i, listeners.size() - 1);
            }
        }

        if (sized)
        {
            for (int i = listeners.size(); --i >= 0;)
            {
                listeners.getUnchecked(i)->tableColumnsResized (this);
                i = jmin (i, listeners.size() - 1);
            }
        }
        */
    }
    
    pub fn get_resize_dragger_at(&self, mousex: i32) -> i32 {
        
        todo!();
        /*
            if (isPositiveAndBelow (mouseX, getWidth()))
        {
            const int draggableDistance = 3;
            int x = 0;

            for (auto* ci : columns)
            {
                if (ci->isVisible())
                {
                    if (std::abs (mouseX - (x + ci->width)) <= draggableDistance
                         && (ci->propertyFlags & resizable) != 0)
                        return ci->id;

                    x += ci->width;
                }
            }
        }

        return 0;
        */
    }
    
    pub fn set_column_under_mouse(&mut self, new_col: i32)  {
        
        todo!();
        /*
            if (newCol != columnIdUnderMouse)
        {
            columnIdUnderMouse = newCol;
            repaint();
        }
        */
    }
    
    pub fn update_column_under_mouse(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            setColumnUnderMouse (reallyContains (e.getPosition(), true) && getResizeDraggerAt (e.x) == 0
                                ? getColumnIdAtX (e.x) : 0);
        */
    }
    
    pub fn show_column_chooser_menu(&mut self, column_id_clicked: i32)  {
        
        todo!();
        /*
            PopupMenu m;
        addMenuItems (m, columnIdClicked);

        if (m.getNumItems() > 0)
        {
            m.setLookAndFeel (&getLookAndFeel());

            m.showMenuAsync (typename PopupMenu::Options(),
                             ModalCallbackFunction::forComponent (tableHeaderMenuCallback, this, columnIdClicked));
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::tableHeader);
        */
    }
}
