crate::ix!();

/**
  | An enum of states to describe the explicit
  | or implicit openness of an item.
  |
  */
pub enum TreeViewItemOpenness
{
    opennessDefault,
    opennessClosed,
    opennessOpen
}

/**
  | An item in a TreeView.
  | 
  | A TreeViewItem can either be a leaf-node
  | in the tree, or it can contain its own
  | sub-items.
  | 
  | To implement an item that contains sub-items,
  | override the itemOpennessChanged()
  | method so that when it is opened, it adds
  | the new sub-items to itself using the
  | addSubItem method. Depending on the
  | nature of the item it might choose to
  | only do this the first time it's opened,
  | or it might want to refresh itself each
  | time.
  | 
  | It also has the option of deleting its
  | sub-items when it is closed, or leaving
  | them in place.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TreeViewItem<'a> {
    owner_view:            *mut TreeView<'a>, // default = nullptr
    parent_item:           *mut TreeViewItem<'a>, // default = nullptr
    sub_items:             Vec<TreeViewItem<'a>>,
    openness:              TreeViewItemOpenness, //= TreeViewItemOpenness::opennessDefault;
    y:                     i32, // default = 0
    item_height:           i32, // default = 0
    total_height:          i32, // default = 0
    item_width:            i32, // default = 0
    total_width:           i32, // default = 0
    uid:                   i32, // default = 0
    selected:              bool, // default = false
    redraw_needed:         bool, // default = true
    draw_lines_inside:     bool, // default = false
    draw_lines_set:        bool, // default = false
    draws_in_left_margin:  bool, // default = false
    draws_in_right_margin: bool, // default = false
}

impl<'a> Drop for TreeViewItem<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (ownerView != nullptr)
            ownerView->viewport->getContentComp()->itemBeingDeleted (this);
        */
    }
}

impl<'a> Default for TreeViewItem<'a> {

    fn default() -> Self {
    
        todo!();
        /*
            static int nextUID = 0;
        uid = nextUID++;
        */
    }
}
    
impl<'a> TreeViewItem<'a> {

    /**
      | Adds a sub-item with a sort-comparator,
      | assuming that the existing items are
      | already sorted.
      | 
      | -----------
      | @param comparator
      | 
      | the comparator object for sorting -
      | see sortSubItems() for details about
      | the methods this class must provide.
      | ----------
      | @param newItem
      | 
      | the object to add to the item's sub-item
      | list. Once added, these can be found
      | using getSubItem(). When the items
      | are later removed with removeSubItem()
      | (or when this item is deleted), they
      | will be deleted.
      |
      */
    pub fn add_sub_item_sorted<ElementComparator>(&mut self, 
        comparator: &mut ElementComparator,
        new_item:   *mut TreeViewItem)  {
    
        todo!();
        /*
            addSubItem (newItem, findInsertIndexInSortedArray (comparator, subItems.begin(), newItem, 0, subItems.size()));
        */
    }

    /**
      | Sorts the list of sub-items using a standard
      | array comparator.
      | 
      | This will use a comparator object to
      | sort the elements into order. The comparator
      | object must have a method of the form:
      | 
      | -----------
      | @code
      | 
      | int compareElements (TreeViewItem* first, TreeViewItem* second);
      | 
      | ..and this method must return:
      | 
      | - a value of < 0 if the first comes before
      | the second
      | 
      | - a value of 0 if the two objects are equivalent
      | 
      | - a value of > 0 if the second comes before
      | the first
      | 
      | To improve performance, the compareElements()
      | method can be declared as static or const.
      |
      */
    pub fn sort_sub_items<ElementComparator>(&mut self, comparator: &mut ElementComparator)  {
    
        todo!();
        /*
            subItems.sort (comparator);
        */
    }

    /**
      | Returns the TreeView to which this item
      | belongs.
      |
      */
    pub fn get_owner_view(&self) -> *mut TreeView {
        
        todo!();
        /*
            return ownerView;
        */
    }

    /**
      | Returns the item within which this item
      | is contained.
      |
      */
    pub fn get_parent_item(&self) -> *mut TreeViewItem {
        
        todo!();
        /*
            return parentItem;
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn item_openness_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the number of sub-items that
      | have been added to this item.
      | 
      | -----------
      | @note
      | 
      | this doesn't mean much if the node isn't
      | open.
      | 
      | @see getSubItem, mightContainSubItems,
      | addSubItem
      |
      */
    pub fn get_num_sub_items(&self) -> i32 {
        
        todo!();
        /*
            return subItems.size();
        */
    }
    
    /**
      | Returns one of the item's sub-items.
      | 
      | Remember that the object returned might
      | get deleted at any time when its parent
      | item is closed or refreshed, depending
      | on the nature of the items you're using.
      | 
      | @see getNumSubItems
      |
      */
    pub fn get_sub_item(&self, index: i32) -> *mut TreeViewItem {
        
        todo!();
        /*
            return subItems[index];
        */
    }
    
    /**
      | Removes any sub-items.
      |
      */
    pub fn clear_sub_items(&mut self)  {
        
        todo!();
        /*
            if (ownerView != nullptr)
        {
            if (! subItems.isEmpty())
            {
                removeAllSubItemsFromList();
                treeHasChanged();
            }
        }
        else
        {
            removeAllSubItemsFromList();
        }
        */
    }
    
    pub fn remove_all_sub_items_from_list(&mut self)  {
        
        todo!();
        /*
            for (int i = subItems.size(); --i >= 0;)
            removeSubItemFromList (i, true);
        */
    }
    
    /**
      | Adds a sub-item.
      | 
      | -----------
      | @param newItem
      | 
      | the object to add to the item's sub-item
      | list. Once added, these can be found
      | using getSubItem(). When the items
      | are later removed with removeSubItem()
      | (or when this item is deleted), they
      | will be deleted.
      | ----------
      | @param insertPosition
      | 
      | the index which the new item should have
      | when it's added. If this value is less
      | than 0, the item will be added to the end
      | of the list.
      |
      */
    pub fn add_sub_item(
        &mut self, 
        new_item:        *mut TreeViewItem,
        insert_position: Option<i32>

    ) {

        let insert_position: i32 = insert_position.unwrap_or(-1);
        
        todo!();
        /*
            if (newItem != nullptr)
        {
            newItem->parentItem = nullptr;
            newItem->setOwnerView (ownerView);
            newItem->y = 0;
            newItem->itemHeight = newItem->getItemHeight();
            newItem->totalHeight = 0;
            newItem->itemWidth = newItem->getItemWidth();
            newItem->totalWidth = 0;
            newItem->parentItem = this;

            if (ownerView != nullptr)
            {
                subItems.insert (insertPosition, newItem);
                treeHasChanged();

                if (newItem->isOpen())
                    newItem->itemOpennessChanged (true);
            }
            else
            {
                subItems.insert (insertPosition, newItem);

                if (newItem->isOpen())
                    newItem->itemOpennessChanged (true);
            }
        }
        */
    }
    
    /**
      | Removes one of the sub-items.
      | 
      | -----------
      | @param index
      | 
      | the item to remove
      | ----------
      | @param deleteItem
      | 
      | if true, the item that is removed will
      | also be deleted.
      |
      */
    pub fn remove_sub_item(
        &mut self, 
        index:       i32,
        delete_item: Option<bool>

    ) {

        let delete_item: bool = delete_item.unwrap_or(true);
        
        todo!();
        /*
            if (ownerView != nullptr)
        {
            if (removeSubItemFromList (index, deleteItem))
                treeHasChanged();
        }
        else
        {
            removeSubItemFromList (index, deleteItem);
        }
        */
    }
    
    pub fn remove_sub_item_from_list(&mut self, 
        index:       i32,
        delete_item: bool) -> bool {
        
        todo!();
        /*
            if (auto* child = subItems[index])
        {
            child->parentItem = nullptr;
            subItems.remove (index, deleteItem);

            return true;
        }

        return false;
        */
    }
    
    /**
      | Returns the openness state of this item.
      | 
      | @see isOpen
      |
      */
    pub fn get_openness(&self) -> TreeViewItemOpenness {
        
        todo!();
        /*
            return openness;
        */
    }
    
    /**
      | Opens or closes the item.
      | 
      | If this causes the value of isOpen()
      | to change, then the item's itemOpennessChanged()
      | method will be called, and a subclass
      | should use this callback to create and
      | add any sub-items that it needs to.
      | 
      | @see setOpen
      |
      */
    pub fn set_openness(&mut self, new_openness: TreeViewItemOpenness)  {
        
        todo!();
        /*
            auto wasOpen = isOpen();
        openness = newOpenness;
        auto isNowOpen = isOpen();

        if (isNowOpen != wasOpen)
        {
            treeHasChanged();
            itemOpennessChanged (isNowOpen);
        }
        */
    }
    
    /**
      | True if this item is currently open in
      | the TreeView.
      | 
      | @see getOpenness
      |
      */
    pub fn is_open(&self) -> bool {
        
        todo!();
        /*
            if (openness == TreeViewItemOpenness::opennessDefault)
            return ownerView != nullptr && ownerView->defaultOpenness;

        return openness == TreeViewItemOpenness::opennessOpen;
        */
    }
    
    /**
      | Opens or closes the item.
      | 
      | When opened or closed, the item's itemOpennessChanged()
      | method will be called, and a subclass
      | should use this callback to create and
      | add any sub-items that it needs to.
      | 
      | -----------
      | @note
      | 
      | if this is called when the item is in its
      | default openness state, and this call
      | would not change whether it's open or
      | closed, then no change will be stored.
      | If you want to explicitly set the openness
      | state to be non-default then you should
      | use setOpenness instead.
      | 
      | @see setOpenness, itemOpennessChanged,
      | mightContainSubItems
      |
      */
    pub fn set_open(&mut self, should_be_open: bool)  {
        
        todo!();
        /*
            if (isOpen() != shouldBeOpen)
            setOpenness (shouldBeOpen ? TreeViewItemOpenness::opennessOpen
                                      : TreeViewItemOpenness::opennessClosed);
        */
    }
    
    pub fn is_fully_open(&self) -> bool {
        
        todo!();
        /*
            if (! isOpen())
            return false;

        for (auto* i : subItems)
            if (! i->isFullyOpen())
                return false;

        return true;
        */
    }
    
    pub fn restore_to_default_openness(&mut self)  {
        
        todo!();
        /*
            setOpenness (TreeViewItemOpenness::opennessDefault);
        */
    }
    
    /**
      | True if this item is currently selected.
      | 
      | Use this when painting the node, to decide
      | whether to draw it as selected or not.
      |
      */
    pub fn is_selected(&self) -> bool {
        
        todo!();
        /*
            return selected;
        */
    }
    
    pub fn deselect_all_recursively(&mut self, item_to_ignore: *mut TreeViewItem)  {
        
        todo!();
        /*
            if (this != itemToIgnore)
            setSelected (false, false);

        for (auto* i : subItems)
            i->deselectAllRecursively (itemToIgnore);
        */
    }
    
    /**
      | Selects or deselects the item.
      | 
      | If shouldNotify == sendNotification,
      | then a callback will be made to itemSelectionChanged()
      | if the item's selection has changed.
      |
      */
    pub fn set_selected(
        &mut self, 
        should_be_selected:         bool,
        deselect_other_items_first: bool,
        notify:                     Option<NotificationType>

    ) {

        let notify: NotificationType = notify.unwrap_or(NotificationType::sendNotification);
        
        todo!();
        /*
            if (shouldBeSelected && ! canBeSelected())
            return;

        if (deselectOtherItemsFirst)
            getTopLevelItem()->deselectAllRecursively (this);

        if (shouldBeSelected != selected)
        {
            selected = shouldBeSelected;

            if (ownerView != nullptr)
            {
                ownerView->repaint();

                if (selected)
                {
                    if (auto* itemComponent = ownerView->getItemComponent (this))
                        if (auto* itemHandler = itemComponent->getAccessibilityHandler())
                            itemHandler->grabFocus();
                }

                if (auto* handler = ownerView->getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
            }

            if (notify != dontSendNotification)
                itemSelectionChanged (shouldBeSelected);
        }
        */
    }
    
    pub fn paint_item(
        &mut self, 
        _0: &mut Graphics,
        _1: i32,
        _2: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn paint_open_close_button(&mut self, 
        g:                 &mut Graphics,
        area:              &Rectangle<f32>,
        background_colour: Colour,
        is_mouse_over:     bool)  {
        
        todo!();
        /*
            getOwnerView()->getLookAndFeel()
           .drawTreeviewPlusMinusBox (g, area, backgroundColour, isOpen(), isMouseOver);
        */
    }
    
    pub fn paint_horizontal_connecting_line(&mut self, 
        g:    &mut Graphics,
        line: &Line<f32>)  {
        
        todo!();
        /*
            g.setColour (ownerView->findColour (TreeView::linesColourId));
       g.drawLine (line);
        */
    }
    
    pub fn paint_vertical_connecting_line(&mut self, 
        g:    &mut Graphics,
        line: &Line<f32>)  {
        
        todo!();
        /*
            g.setColour (ownerView->findColour (TreeView::linesColourId));
       g.drawLine (line);
        */
    }
    
    pub fn item_clicked(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn item_double_clicked(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (mightContainSubItems())
            setOpen (! isOpen());
        */
    }
    
    pub fn item_selection_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_tooltip(&mut self) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_accessibility_name(&mut self) -> String {
        
        todo!();
        /*
            auto tooltipString = getTooltip();

        return tooltipString.isNotEmpty()
          ? tooltipString
          : "Level " + String (getItemDepth (this)) + " row " + String (getIndexInParent());
        */
    }
    
    pub fn owner_view_changed(&mut self, _0: *mut TreeView)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_drag_source_description(&mut self) -> Var {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, _0: &Vec<String>) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn files_dropped(&mut self, 
        files:        &Vec<String>,
        insert_index: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn item_dropped(&mut self, 
        drag_source_details: &DragAndDropTargetSourceDetails,
        insert_index:        i32)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the rectangle that this item
      | occupies.
      | 
      | If relativeToTreeViewTopLeft is true,
      | the coordinates are relative to the
      | top-left of the TreeView comp, so this
      | will depend on the scroll-position
      | of the tree. If false, it is relative
      | to the top-left of the topmost item in
      | the tree (so this would be unaffected
      | by scrolling the view).
      |
      */
    pub fn get_item_position(&self, relative_to_tree_view_top_left: bool) -> Rectangle<i32> {
        
        todo!();
        /*
            auto indentX = getIndentX();
        auto width = itemWidth;

        if (ownerView != nullptr && width < 0)
            width = ownerView->viewport->getViewWidth() - indentX;

        Rectangle<int> r (indentX, y, jmax (0, width), totalHeight);

        if (relativeToTreeViewTopLeft && ownerView != nullptr)
            r -= ownerView->viewport->getViewPosition();

        return r;
        */
    }
    
    /**
      | Sends a signal to the TreeView to make
      | it refresh itself.
      | 
      | Call this if your items have changed
      | and you want the tree to update to reflect
      | this.
      |
      */
    pub fn tree_has_changed(&self)  {
        
        todo!();
        /*
            if (ownerView != nullptr)
            ownerView->updateVisibleItems();
        */
    }
    
    /**
      | Sends a repaint message to redraw just
      | this item.
      | 
      | -----------
      | @note
      | 
      | you should only call this if you want
      | to repaint a superficial change. If
      | you're altering the tree's nodes, you
      | should instead call treeHasChanged().
      |
      */
    pub fn repaint_item(&self)  {
        
        todo!();
        /*
            if (ownerView != nullptr && areAllParentsOpen())
            if (auto* component = ownerView->getItemComponent (this))
                component->repaint();
        */
    }
    
    /**
      | Returns true if all the item's parent
      | nodes are open.
      | 
      | This is useful to check whether the item
      | might actually be visible or not.
      |
      */
    pub fn are_all_parents_open(&self) -> bool {
        
        todo!();
        /*
            return parentItem == nullptr
                || (parentItem->isOpen() && parentItem->areAllParentsOpen());
        */
    }
    
    pub fn update_positions(&mut self, newy: i32)  {
        
        todo!();
        /*
            y = newY;
        itemHeight = getItemHeight();
        totalHeight = itemHeight;
        itemWidth = getItemWidth();
        totalWidth = jmax (itemWidth, 0) + getIndentX();

        if (isOpen())
        {
            newY += totalHeight;

            for (auto* i : subItems)
            {
                i->updatePositions (newY);
                newY += i->totalHeight;
                totalHeight += i->totalHeight;
                totalWidth = jmax (totalWidth, i->totalWidth);
            }
        }
        */
    }
    
    pub fn get_deepest_open_parent_item(&mut self) -> *mut TreeViewItem {
        
        todo!();
        /*
            auto* result = this;
        auto* item = this;

        while (item->parentItem != nullptr)
        {
            item = item->parentItem;

            if (! item->isOpen())
                result = item;
        }

        return result;
        */
    }
    
    pub fn set_owner_view(&mut self, new_owner: *mut TreeView)  {
        
        todo!();
        /*
            ownerView = newOwner;

        for (auto* i : subItems)
        {
            i->setOwnerView (newOwner);
            i->ownerViewChanged (newOwner);
        }
        */
    }
    
    pub fn get_indentx(&self) -> i32 {
        
        todo!();
        /*
            int x = ownerView->rootItemVisible ? 1 : 0;

        if (! ownerView->openCloseButtonsVisible)
            --x;

        for (auto* p = parentItem; p != nullptr; p = p->parentItem)
            ++x;

        return x * ownerView->getIndentSize();
        */
    }
    
    /**
      | Sets a flag to indicate that the item
      | wants to be allowed to draw all the way
      | across to the left edge of the TreeView.
      | 
      | By default this is false, which means
      | that when the paintItem() method is
      | called, its graphics context is clipped
      | to only allow drawing within the item's
      | rectangle. If this flag is set to true,
      | then the graphics context isn't clipped
      | on its left side, so it can draw all the
      | way across to the left margin. Note that
      | the context will still have its origin
      | in the same place though, so the coordinates
      | of anything to its left will be negative.
      | It's mostly useful if you want to draw
      | a wider bar behind the highlighted item.
      |
      */
    pub fn set_draws_in_left_margin(&mut self, can_draw_in_left_margin: bool)  {
        
        todo!();
        /*
            drawsInLeftMargin = canDrawInLeftMargin;
        */
    }
    
    /**
      | Sets a flag to indicate that the item
      | wants to be allowed to draw all the way
      | across to the right edge of the TreeView.
      | 
      | Similar to setDrawsInLeftMargin:
      | when this flag is set to true, then the
      | graphics context isn't clipped on the
      | right side. Unlike setDrawsInLeftMargin,
      | you will very rarely need to use this
      | function, as this method won't clip
      | the right margin unless your TreeViewItem
      | overrides getItemWidth to return a
      | positive value.
      | 
      | @see setDrawsInLeftMargin, getItemWidth
      |
      */
    pub fn set_draws_in_right_margin(&mut self, can_draw_in_right_margin: bool)  {
        
        todo!();
        /*
            drawsInRightMargin = canDrawInRightMargin;
        */
    }
    
    pub fn are_lines_drawn(&self) -> bool {
        
        todo!();
        /*
            return drawLinesSet ? drawLinesInside
                            : (ownerView != nullptr && ownerView->getLookAndFeel().areLinesDrawnForTreeView (*ownerView));
        */
    }
    
    /**
      | Returns true if this item is the last
      | of its parent's sub-items.
      |
      */
    pub fn is_last_of_siblings(&self) -> bool {
        
        todo!();
        /*
            return parentItem == nullptr
                || parentItem->subItems.getLast() == this;
        */
    }
    
    /**
      | Returns the index of this item in its
      | parent's sub-items.
      |
      */
    pub fn get_index_in_parent(&self) -> i32 {
        
        todo!();
        /*
            return parentItem == nullptr ? 0
                                     : parentItem->subItems.indexOf (this);
        */
    }
    
    pub fn get_top_level_item(&mut self) -> *mut TreeViewItem {
        
        todo!();
        /*
            return parentItem == nullptr ? this
                                     : parentItem->getTopLevelItem();
        */
    }
    
    pub fn get_num_rows(&self) -> i32 {
        
        todo!();
        /*
            int num = 1;

        if (isOpen())
            for (auto* i : subItems)
                num += i->getNumRows();

        return num;
        */
    }
    
    pub fn get_item_on_row(&mut self, index: i32) -> *mut TreeViewItem {
        
        todo!();
        /*
            if (index == 0)
            return this;

        if (index > 0 && isOpen())
        {
            --index;

            for (auto* i : subItems)
            {
                if (index == 0)
                    return i;

                auto numRows = i->getNumRows();

                if (numRows > index)
                    return i->getItemOnRow (index);

                index -= numRows;
            }
        }

        return nullptr;
        */
    }
    
    pub fn count_selected_items_recursively(&self, depth: i32) -> i32 {
        
        todo!();
        /*
            int total = isSelected() ? 1 : 0;

        if (depth != 0)
            for (auto* i : subItems)
                total += i->countSelectedItemsRecursively (depth - 1);

        return total;
        */
    }
    
    pub fn get_selected_item_with_index(&mut self, index: i32) -> *mut TreeViewItem {
        
        todo!();
        /*
            if (isSelected())
        {
            if (index == 0)
                return this;

            --index;
        }

        if (index >= 0)
        {
            for (auto* i : subItems)
            {
                if (auto* found = i->getSelectedItemWithIndex (index))
                    return found;

                index -= i->countSelectedItemsRecursively (-1);
            }
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the row number of this item in
      | the tree.
      | 
      | The row number of an item will change
      | according to which items are open.
      | 
      | @see TreeView::getNumRowsInTree(),
      | TreeView::getItemOnRow()
      |
      */
    pub fn get_row_number_in_tree(&self) -> i32 {
        
        todo!();
        /*
            if (parentItem != nullptr && ownerView != nullptr)
        {
            if (! parentItem->isOpen())
                return parentItem->getRowNumberInTree();

            auto n = 1 + parentItem->getRowNumberInTree();

            auto ourIndex = parentItem->subItems.indexOf (this);
            jassert (ourIndex >= 0);

            while (--ourIndex >= 0)
                n += parentItem->subItems [ourIndex]->getNumRows();

            if (parentItem->parentItem == nullptr
                 && ! ownerView->rootItemVisible)
                --n;

            return n;
        }

        return 0;
        */
    }
    
    /**
      | Changes whether lines are drawn to connect
      | any sub-items to this item.
      | 
      | By default, line-drawing is turned
      | on according to LookAndFeel::areLinesDrawnForTreeView().
      |
      */
    pub fn set_lines_drawn_for_sub_items(&mut self, draw_lines: bool)  {
        
        todo!();
        /*
            drawLinesInside = drawLines;
        drawLinesSet = true;
        */
    }
    
    /**
      | Creates a string that can be used to uniquely
      | retrieve this item in the tree.
      | 
      | The string that is returned can be passed
      | to TreeView::findItemFromIdentifierString().
      | 
      | The string takes the form of a path, constructed
      | from the getUniqueName() of this item
      | and all its parents, so these must all
      | be correctly implemented for it to work.
      | 
      | @see TreeView::findItemFromIdentifierString,
      | getUniqueName
      |
      */
    pub fn get_item_identifier_string(&self) -> String {
        
        todo!();
        /*
            String s;

        if (parentItem != nullptr)
            s = parentItem->getItemIdentifierString();

        return s + "/" + escapeSlashesInTreeViewItemName (getUniqueName());
        */
    }
    
    pub fn find_item_from_identifier_string(&mut self, identifier_string: &String) -> *mut TreeViewItem {
        
        todo!();
        /*
            auto thisId = "/" + escapeSlashesInTreeViewItemName (getUniqueName());

        if (thisId == identifierString)
            return this;

        if (identifierString.startsWith (thisId + "/"))
        {
            auto remainingPath = identifierString.substring (thisId.length());

            const auto wasOpen = isOpen();
            setOpen (true);

            for (auto* i : subItems)
                if (auto* item = i->findItemFromIdentifierString (remainingPath))
                    return item;

            setOpen (wasOpen);
        }

        return nullptr;
        */
    }
    
    /**
      | Restores the openness of this item and
      | all its sub-items from a saved state.
      | 
      | See TreeView::restoreOpennessState
      | for more details.
      | 
      | You'd normally want to use TreeView::restoreOpennessState()
      | rather than call it for a specific item,
      | but this can be handy if you need to briefly
      | save the state for a section of the tree.
      | 
      | @see TreeView::restoreOpennessState,
      | getOpennessState
      |
      */
    pub fn restore_openness_state(&mut self, e: &XmlElement)  {
        
        todo!();
        /*
            if (e.hasTagName ("CLOSED"))
        {
            setOpen (false);
        }
        else if (e.hasTagName ("OPEN"))
        {
            setOpen (true);

            Vec<TreeViewItem*> items;
            items.addArray (subItems);

            for (auto* n : e.getChildIterator())
            {
                auto id = n->getStringAttribute ("id");

                for (int i = 0; i < items.size(); ++i)
                {
                    auto* ti = items.getUnchecked (i);

                    if (ti->getUniqueName() == id)
                    {
                        ti->restoreOpennessState (*n);
                        items.remove (i);
                        break;
                    }
                }
            }

            // for any items that weren't mentioned in the XML, reset them to default:
            for (auto* i : items)
                i->restoreToDefaultOpenness();
        }
        */
    }
    
    /**
      | Saves the current state of open/closed
      | nodes so it can be restored later.
      | 
      | This takes a snapshot of which sub-nodes
      | have been explicitly opened or closed,
      | and records it as XML. To identify node
      | objects it uses the
      | 
      | TreeViewItem::getUniqueName() method
      | to create named paths. This means that
      | the same state of open/closed nodes
      | can be restored to a completely different
      | instance of the tree, as long as it contains
      | nodes whose unique names are the same.
      | 
      | You'd normally want to use TreeView::getOpennessState()
      | rather than call it for a specific item,
      | but this can be handy if you need to briefly
      | save the state for a section of the tree.
      | 
      | -----------
      | @note
      | 
      | if all nodes of the tree are in their default
      | state, then this may return a nullptr.
      | 
      | @see TreeView::getOpennessState,
      | restoreOpennessState
      |
      */
    pub fn get_openness_state(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            return getOpennessState (true);
        */
    }
    
    pub fn get_openness_state_and_possibly_return_null(&self, can_return_null: bool) -> Box<XmlElement> {
        
        todo!();
        /*
            auto name = getUniqueName();

        if (name.isNotEmpty())
        {
            std::unique_ptr<XmlElement> e;

            if (isOpen())
            {
                if (canReturnNull && ownerView != nullptr && ownerView->defaultOpenness && isFullyOpen())
                    return nullptr;

                e = std::make_unique<XmlElement> ("OPEN");

                for (int i = subItems.size(); --i >= 0;)
                    e->prependChildElement (subItems.getUnchecked (i)->getOpennessState (true).release());
            }
            else
            {
                if (canReturnNull && ownerView != nullptr && ! ownerView->defaultOpenness)
                    return nullptr;

                e = std::make_unique<XmlElement> ("CLOSED");
            }

            e->setAttribute ("id", name);
            return e;
        }

        // trying to save the openness for an element that has no name - this won't
        // work because it needs the names to identify what to open.
        jassertfalse;
        return {};
        */
    }
    
    pub fn draw(&mut self, 
        g:                    &mut Graphics,
        width:                i32,
        is_mouse_over_button: bool)  {
        
        todo!();
        /*
            const auto indent = getIndentX();
        const auto itemW = (itemWidth < 0 || drawsInRightMargin) ? width - indent : itemWidth;

        {
            Graphics::ScopedSaveState ss (g);
            g.setOrigin (indent, 0);

            if (g.reduceClipRegion (drawsInLeftMargin ? -indent : 0, 0,
                                    drawsInLeftMargin ? itemW + indent : itemW, itemHeight))
            {
                if (isSelected())
                    g.fillAll (ownerView->findColour (TreeView::selectedItemBackgroundColourId));
                else
                    g.fillAll ((getRowNumberInTree() % 2 == 0) ? ownerView->findColour (TreeView::oddItemsColourId)
                                                               : ownerView->findColour (TreeView::evenItemsColourId));

                paintItem (g, itemWidth < 0 ? width - indent : itemWidth, itemHeight);
            }
        }

        const auto halfH = (float) itemHeight * 0.5f;
        const auto indentWidth = ownerView->getIndentSize();
        const auto depth = getItemDepth (this);

        if (depth >= 0 && ownerView->openCloseButtonsVisible)
        {
            auto x = ((float) depth + 0.5f) * (float) indentWidth;
            const auto parentLinesDrawn = parentItem != nullptr && parentItem->areLinesDrawn();

            if (parentLinesDrawn)
                paintVerticalConnectingLine (g, Line<float> (x, 0, x, isLastOfSiblings() ? halfH : (float) itemHeight));

            if (parentLinesDrawn || (parentItem == nullptr && areLinesDrawn()))
                paintHorizontalConnectingLine (g, Line<float> (x, halfH, x + (float) indentWidth * 0.5f, halfH));

            {
                auto* p = parentItem;
                auto d = depth;

                while (p != nullptr && --d >= 0)
                {
                    x -= (float) indentWidth;

                    if ((p->parentItem == nullptr || p->parentItem->areLinesDrawn()) && ! p->isLastOfSiblings())
                        p->paintVerticalConnectingLine (g, Line<float> (x, 0, x, (float) itemHeight));

                    p = p->parentItem;
                }
            }

            if (mightContainSubItems())
            {
                auto backgroundColour = ownerView->findColour (TreeView::backgroundColourId);

                paintOpenCloseButton (g, Rectangle<float> ((float) (depth * indentWidth), 0, (float) indentWidth, (float) itemHeight),
                                      backgroundColour.isTransparent() ? Colours::white : backgroundColour,
                                      isMouseOverButton);
            }
        }
        */
    }
}
