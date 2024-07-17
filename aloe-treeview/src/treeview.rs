crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TreeView.h]

/**
  | A tree-view component.
  | 
  | Use one of these to hold and display a
  | structure of TreeViewItem objects.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TreeView<'a> {
    base:                        Component<'a>,
    base2:                       SettableTooltipClient,
    viewport:                    Box<TreeViewport<'a>>,
    root_item:                   *mut TreeViewItem<'a>, // default = nullptr
    drag_insert_point_highlight: Box<InsertPointHighlight<'a>>,
    drag_target_group_highlight: Box<TreeViewTargetGroupHighlight<'a>>,
    indent_size:                 i32, // default = -1
    default_openness:            bool, // default = false
    root_item_visible:           bool, // default = true
    multi_select_enabled:        bool, // default = false
    open_close_buttons_visible:  bool, // default = true
}

impl<'a> FileDragAndDropTarget       for TreeView<'a> { }
impl<'a> ShouldDrawDragImageWhenOver for TreeView<'a> { }
impl<'a> DragAndDropTarget           for TreeView<'a> { }

impl<'a> ItemDropped for TreeView<'a> { 

    fn item_dropped(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            handleDrop (StringArray(), dragSourceDetails);
        */
    }
}

impl<'a> ItemDragExit for TreeView<'a> { 

    fn item_drag_exit(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            hideDragHighlight();
        */
    }
}

impl<'a> IsInterestedInDragSource for TreeView<'a> { 

    fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl<'a> ItemDragEnter for TreeView<'a> { 

    fn item_drag_enter(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            itemDragMove (dragSourceDetails);
        */
    }
}

impl<'a> ItemDragMove for TreeView<'a> { 

    fn item_drag_move(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            handleDrag (StringArray(), dragSourceDetails);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_TreeView.cpp]
impl<'a> Drop for TreeView<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (rootItem != nullptr)
            rootItem->setOwnerView (nullptr);
        */
    }
}

impl<'a> TreeView<'a> {

    pub fn is_interested_in_file_drag(&mut self, _0: &Vec<String>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }

    /**
      | Returns the tree's root item.
      | 
      | This will be the last object passed to
      | setRootItem(), or nullptr if none has
      | been set.
      |
      */
    pub fn get_root_item(&self) -> *mut TreeViewItem {
        
        todo!();
        /*
            return rootItem;
        */
    }

    /**
      | Returns true if the root item is visible.
      | 
      | @see setRootItemVisible
      |
      */
    pub fn is_root_item_visible(&self) -> bool {
        
        todo!();
        /*
            return rootItemVisible;
        */
    }

    /**
      | Returns true if the tree's items default
      | to being open.
      | 
      | @see setDefaultOpenness
      |
      */
    pub fn are_items_open_by_default(&self) -> bool {
        
        todo!();
        /*
            return defaultOpenness;
        */
    }

    /**
      | Returns whether multi-select has been
      | enabled for the tree.
      | 
      | @see setMultiSelectEnabled
      |
      */
    pub fn is_multi_select_enabled(&self) -> bool {
        
        todo!();
        /*
            return multiSelectEnabled;
        */
    }

    /**
      | Returns whether open/close buttons
      | are shown.
      | 
      | @see setOpenCloseButtonsVisible
      |
      */
    pub fn are_open_close_buttons_visible(&self) -> bool {
        
        todo!();
        /*
            return openCloseButtonsVisible;
        */
    }

    pub fn items_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn update_button_under_mouse(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Creates an empty TreeView.
      | 
      | Once you've got a TreeView component,
      | you'll need to give it something to display,
      | using the setRootItem() method.
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : component(name),

            viewport = std::make_unique<TreeViewport>();
        addAndMakeVisible (viewport.get());
        viewport->setViewedComponent (new TreeViewContentComponent (*this));

        setWantsKeyboardFocus (true);
        setFocusContainerType (FocusContainerType::focusContainer);
        */
    }
    
    /**
      | Sets the item that is displayed in the
      | TreeView.
      | 
      | A tree has a single root item which contains
      | as many sub-items as it needs. If you
      | want the tree to contain a number of root
      | items, you should still use a single
      | root item above these, but hide it using
      | setRootItemVisible().
      | 
      | You can pass nullptr to this method to
      | clear the tree and remove its current
      | root item.
      | 
      | The object passed in will not be deleted
      | by the TreeView, it's up to the caller
      | to delete it when no longer needed. BUT
      | make absolutely sure that you don't
      | delete this item until you've removed
      | it from the tree, either by calling setRootItem
      | (nullptr), or by deleting the tree first.
      | You can also use deleteRootItem() as
      | a quick way to delete it.
      |
      */
    pub fn set_root_item(&mut self, new_root_item: *mut TreeViewItem)  {
        
        todo!();
        /*
            if (rootItem != newRootItem)
        {
            if (newRootItem != nullptr)
            {
                // can't use a tree item in more than one tree at once..
                jassert (newRootItem->ownerView == nullptr);

                if (newRootItem->ownerView != nullptr)
                    newRootItem->ownerView->setRootItem (nullptr);
            }

            if (rootItem != nullptr)
                rootItem->setOwnerView (nullptr);

            rootItem = newRootItem;

            if (newRootItem != nullptr)
                newRootItem->setOwnerView (this);

            if (rootItem != nullptr && (defaultOpenness || ! rootItemVisible))
            {
                rootItem->setOpen (false); // force a re-open
                rootItem->setOpen (true);
            }

            updateVisibleItems();
        }
        */
    }
    
    /**
      | This will remove and delete the current
      | root item.
      | 
      | It's a convenient way of deleting the
      | item and calling setRootItem (nullptr).
      |
      */
    pub fn delete_root_item(&mut self)  {
        
        todo!();
        /*
            const std::unique_ptr<TreeViewItem> deleter (rootItem);
        setRootItem (nullptr);
        */
    }
    
    /**
      | Changes whether the tree's root item
      | is shown or not.
      | 
      | If the root item is hidden, only its sub-items
      | will be shown in the TreeView - this lets
      | you make the tree look as if it's got many
      | root items. If it's hidden, this call
      | will also make sure the root item is open
      | (otherwise the TreeView would look
      | empty).
      |
      */
    pub fn set_root_item_visible(&mut self, should_be_visible: bool)  {
        
        todo!();
        /*
            rootItemVisible = shouldBeVisible;

        if (rootItem != nullptr && (defaultOpenness || ! rootItemVisible))
        {
            rootItem->setOpen (false); // force a re-open
            rootItem->setOpen (true);
        }

        updateVisibleItems();
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            setOpaque (findColour (backgroundColourId).isOpaque());
        repaint();
        */
    }
    
    /**
      | Changes the distance by which each nested
      | level of the tree is indented.
      | 
      | @see getIndentSize
      |
      */
    pub fn set_indent_size(&mut self, new_indent_size: i32)  {
        
        todo!();
        /*
            if (indentSize != newIndentSize)
        {
            indentSize = newIndentSize;
            resized();
        }
        */
    }
    
    /**
      | Returns the number of pixels by which
      | each nested level of the tree is indented.
      | 
      | @see setIndentSize
      |
      */
    pub fn get_indent_size(&mut self) -> i32 {
        
        todo!();
        /*
            return indentSize >= 0 ? indentSize
                               : getLookAndFeel().getTreeViewIndentSize (*this);
        */
    }
    
    /**
      | Sets whether items are open or closed
      | by default.
      | 
      | Normally, items are closed until the
      | user opens them, but you can use this
      | to make them default to being open until
      | explicitly closed.
      | 
      | @see areItemsOpenByDefault
      |
      */
    pub fn set_default_openness(&mut self, is_open_by_default: bool)  {
        
        todo!();
        /*
            if (defaultOpenness != isOpenByDefault)
        {
            defaultOpenness = isOpenByDefault;
            updateVisibleItems();
        }
        */
    }
    
    /**
      | This sets a flag to indicate that the
      | tree can be used for multi-selection.
      | 
      | You can always select multiple items
      | internally by calling the
      | 
      | TreeViewItem::setSelected() method,
      | but this flag indicates whether the
      | user is allowed to multi-select by clicking
      | on the tree.
      | 
      | By default it is disabled.
      | 
      | @see isMultiSelectEnabled
      |
      */
    pub fn set_multi_select_enabled(&mut self, can_multi_select: bool)  {
        
        todo!();
        /*
            multiSelectEnabled = canMultiSelect;
        */
    }
    
    /**
      | Sets a flag to indicate whether to hide
      | the open/close buttons.
      | 
      | @see areOpenCloseButtonsVisible
      |
      */
    pub fn set_open_close_buttons_visible(&mut self, should_be_visible: bool)  {
        
        todo!();
        /*
            if (openCloseButtonsVisible != shouldBeVisible)
        {
            openCloseButtonsVisible = shouldBeVisible;
            updateVisibleItems();
        }
        */
    }
    
    /**
      | Returns the TreeView's Viewport object.
      |
      */
    pub fn get_viewport(&self) -> *mut Viewport {
        
        todo!();
        /*
            return viewport.get();
        */
    }
    
    /**
      | Deselects any items that are currently
      | selected.
      |
      */
    pub fn clear_selected_items(&mut self)  {
        
        todo!();
        /*
            if (rootItem != nullptr)
            rootItem->deselectAllRecursively (nullptr);
        */
    }
    
    /**
      | Returns the number of items that are
      | currently selected.
      | 
      | If maximumDepthToSearchTo is >= 0,
      | it lets you specify a maximum depth to
      | which the tree will be recursed.
      | 
      | @see getSelectedItem, clearSelectedItems
      |
      */
    pub fn get_num_selected_items(&self, maximum_depth_to_search_to: Option<i32>) -> i32 {

        let maximum_depth_to_search_to: i32 = maximum_depth_to_search_to.unwrap_or(-1);
        
        todo!();
        /*
            return rootItem != nullptr ? rootItem->countSelectedItemsRecursively (maximumDepthToSearchTo) : 0;
        */
    }
    
    /**
      | Returns one of the selected items in
      | the tree.
      | 
      | -----------
      | @param index
      | 
      | the index, 0 to (getNumSelectedItems()
      | - 1)
      |
      */
    pub fn get_selected_item(&self, index: i32) -> *mut TreeViewItem {
        
        todo!();
        /*
            return rootItem != nullptr ? rootItem->getSelectedItemWithIndex (index) : nullptr;
        */
    }
    
    /**
      | Returns the number of rows the tree is
      | using, depending on which items are
      | open.
      | 
      | @see TreeViewItem::getRowNumberInTree()
      |
      */
    pub fn get_num_rows_in_tree(&self) -> i32 {
        
        todo!();
        /*
            return rootItem != nullptr ? (rootItem->getNumRows() - (rootItemVisible ? 0 : 1)) : 0;
        */
    }
    
    /**
      | Returns the item on a particular row
      | of the tree.
      | 
      | If the index is out of range, this will
      | return nullptr.
      | 
      | @see getNumRowsInTree, TreeViewItem::getRowNumberInTree()
      |
      */
    pub fn get_item_on_row(&self, index: i32) -> *mut TreeViewItem {
        
        todo!();
        /*
            if (! rootItemVisible)
            ++index;

        if (rootItem != nullptr && index >= 0)
            return rootItem->getItemOnRow (index);

        return nullptr;
        */
    }
    
    /**
      | Returns the item that contains a given
      | y-position relative to the top of the
      | TreeView component.
      |
      */
    pub fn get_item_at(&self, y: i32) -> *mut TreeViewItem {
        
        todo!();
        /*
            if (auto* contentComp = viewport->getContentComp())
            if (auto* itemComponent = contentComp->getItemComponentAt (contentComp->getLocalPoint (this, Point<int> (0, y))))
                return &itemComponent->getRepresentedItem();

        return nullptr;
        */
    }
    
    /**
      | Searches the tree for an item with the
      | specified identifier.
      | 
      | The identifier string must have been
      | created by calling TreeViewItem::getItemIdentifierString().
      | 
      | If no such item exists, this will return
      | false. If the item is found, all of its
      | items will be automatically opened.
      |
      */
    pub fn find_item_from_identifier_string(&self, identifier_string: &String) -> *mut TreeViewItem {
        
        todo!();
        /*
            if (rootItem == nullptr)
            return nullptr;

        return rootItem->findItemFromIdentifierString (identifierString);
        */
    }
    
    /**
      | Returns the component that currently
      | represents a given TreeViewItem.
      |
      */
    pub fn get_item_component(&self, item: *const TreeViewItem) -> *mut Component {
        
        todo!();
        /*
            return viewport->getContentComp()->getComponentForItem (item);
        */
    }
    
    /**
      | Saves the current state of open/closed
      | nodes so it can be restored later.
      | 
      | This takes a snapshot of which nodes
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
      | -----------
      | @param alsoIncludeScrollPosition
      | 
      | if this is true, the state will also include
      | information about where the tree has
      | been scrolled to vertically, so this
      | can also be restored @see restoreOpennessState
      |
      */
    pub fn get_openness_state(&self, also_include_scroll_position: bool) -> Box<XmlElement> {
        
        todo!();
        /*
            if (rootItem != nullptr)
        {
            if (auto rootOpenness = rootItem->getOpennessState (false))
            {
                if (alsoIncludeScrollPosition)
                    rootOpenness->setAttribute ("scrollPos", viewport->getViewPositionY());

                addAllSelectedItemIds (rootItem, *rootOpenness);
                return rootOpenness;
            }
        }

        return {};
        */
    }
    
    /**
      | Restores a previously saved arrangement
      | of open/closed nodes.
      | 
      | This will try to restore a snapshot of
      | the tree's state that was created by
      | the getOpennessState() method. If
      | any of the nodes named in the original
      | 
      | XML aren't present in this tree, they
      | will be ignored.
      | 
      | If restoreStoredSelection is true,
      | it will also try to re-select any items
      | that were selected in the stored state.
      | 
      | @see getOpennessState
      |
      */
    pub fn restore_openness_state(&mut self, 
        new_state:                &XmlElement,
        restore_stored_selection: bool)  {
        
        todo!();
        /*
            if (rootItem != nullptr)
        {
            rootItem->restoreOpennessState (newState);

            if (newState.hasAttribute ("scrollPos"))
                viewport->setViewPosition (viewport->getViewPositionX(),
                                           newState.getIntAttribute ("scrollPos"));

            if (restoreStoredSelection)
            {
                clearSelectedItems();

                for (auto* e : newState.getChildWithTagNameIterator ("SELECTED"))
                    if (auto* item = rootItem->findItemFromIdentifierString (e->getStringAttribute ("id")))
                        item->setSelected (true, false);
            }

            updateVisibleItems();
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            viewport->setBounds (getLocalBounds());
        updateVisibleItems();
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    /**
      | Moves the selected row up or down by the
      | specified number of rows.
      |
      */
    pub fn move_selected_row(&mut self, delta: i32)  {
        
        todo!();
        /*
            auto numRowsInTree = getNumRowsInTree();

        if (numRowsInTree > 0)
        {
            int rowSelected = 0;

            if (auto* firstSelected = getSelectedItem (0))
                rowSelected = firstSelected->getRowNumberInTree();

            rowSelected = jlimit (0, numRowsInTree - 1, rowSelected + delta);

            for (;;)
            {
                if (auto* item = getItemOnRow (rowSelected))
                {
                    if (! item->canBeSelected())
                    {
                        // if the row we want to highlight doesn't allow it, try skipping
                        // to the next item..
                        auto nextRowToTry = jlimit (0, numRowsInTree - 1, rowSelected + (delta < 0 ? -1 : 1));

                        if (rowSelected != nextRowToTry)
                        {
                            rowSelected = nextRowToTry;
                            continue;
                        }

                        break;
                    }

                    item->setSelected (true, true);
                    scrollToKeepItemVisible (item);
                }

                break;
            }
        }
        */
    }
    
    /**
      | Tries to scroll the tree so that this
      | item is on-screen somewhere.
      |
      */
    pub fn scroll_to_keep_item_visible(&mut self, item: *mut TreeViewItem)  {
        
        todo!();
        /*
            if (item != nullptr && item->ownerView == this)
        {
            updateVisibleItems();

            item = item->getDeepestOpenParentItem();

            auto y = item->y;
            auto viewTop = viewport->getViewPositionY();

            if (y < viewTop)
            {
                viewport->setViewPosition (viewport->getViewPositionX(), y);
            }
            else if (y + item->itemHeight > viewTop + viewport->getViewHeight())
            {
                viewport->setViewPosition (viewport->getViewPositionX(),
                                           (y + item->itemHeight) - viewport->getViewHeight());
            }
        }
        */
    }
    
    pub fn toggle_open_selected_item(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* firstSelected = getSelectedItem (0))
        {
            if (firstSelected->mightContainSubItems())
            {
                firstSelected->setOpen (! firstSelected->isOpen());
                return true;
            }
        }

        return false;
        */
    }
    
    pub fn move_out_of_selected_item(&mut self)  {
        
        todo!();
        /*
            if (auto* firstSelected = getSelectedItem (0))
        {
            if (firstSelected->isOpen())
            {
                firstSelected->setOpen (false);
            }
            else
            {
                auto* parent = firstSelected->parentItem;

                if ((! rootItemVisible) && parent == rootItem)
                    parent = nullptr;

                if (parent != nullptr)
                {
                    parent->setSelected (true, true);
                    scrollToKeepItemVisible (parent);
                }
            }
        }
        */
    }
    
    pub fn move_into_selected_item(&mut self)  {
        
        todo!();
        /*
            if (auto* firstSelected = getSelectedItem (0))
        {
            if (firstSelected->isOpen() || ! firstSelected->mightContainSubItems())
                moveSelectedRow (1);
            else
                firstSelected->setOpen (true);
        }
        */
    }
    
    pub fn move_by_pages(&mut self, num_pages: i32)  {
        
        todo!();
        /*
            if (auto* currentItem = getSelectedItem (0))
        {
            auto pos = currentItem->getItemPosition (false);
            auto targetY = pos.getY() + numPages * (getHeight() - pos.getHeight());
            auto currentRow = currentItem->getRowNumberInTree();

            for (;;)
            {
                moveSelectedRow (numPages);
                currentItem = getSelectedItem (0);

                if (currentItem == nullptr)
                    break;

                auto y = currentItem->getItemPosition (false).getY();

                if ((numPages < 0 && y <= targetY) || (numPages > 0 && y >= targetY))
                    break;

                auto newRow = currentItem->getRowNumberInTree();

                if (newRow == currentRow)
                    break;

                currentRow = newRow;
            }
        }
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (rootItem != nullptr)
        {
            if (key == KeyPress::upKey)       { moveSelectedRow (-1); return true; }
            if (key == KeyPress::downKey)     { moveSelectedRow (1);  return true; }
            if (key == KeyPress::homeKey)     { moveSelectedRow (-0x3fffffff); return true; }
            if (key == KeyPress::endKey)      { moveSelectedRow (0x3fffffff);  return true; }
            if (key == KeyPress::pageUpKey)   { moveByPages (-1); return true; }
            if (key == KeyPress::pageDownKey) { moveByPages (1);  return true; }
            if (key == KeyPress::returnKey)   { return toggleOpenSelectedItem(); }
            if (key == KeyPress::leftKey)     { moveOutOfSelectedItem();  return true; }
            if (key == KeyPress::rightKey)    { moveIntoSelectedItem();   return true; }
        }

        return false;
        */
    }
    
    pub fn update_visible_items(&mut self)  {
        
        todo!();
        /*
            if (rootItem != nullptr)
        {
            rootItem->updatePositions (rootItemVisible ? 0 : -rootItem->itemHeight);

            viewport->getViewedComponent()
                ->setSize (jmax (viewport->getMaximumVisibleWidth(), rootItem->totalWidth + 50),
                           rootItem->totalHeight - (rootItemVisible ? 0 : rootItem->itemHeight));
        }
        else
        {
            viewport->getViewedComponent()->setSize (0, 0);
        }

        viewport->updateComponents (false);
        */
    }
    
    pub fn show_drag_highlight(&mut self, insert_pos: &TreeViewInsertPoint)  {
        
        todo!();
        /*
            beginDragAutoRepeat (100);

        if (dragInsertPointHighlight == nullptr)
        {
            dragInsertPointHighlight = std::make_unique<InsertPointHighlight>();
            dragTargetGroupHighlight = std::make_unique<TreeViewTargetGroupHighlight>();

            addAndMakeVisible (dragInsertPointHighlight.get());
            addAndMakeVisible (dragTargetGroupHighlight.get());
        }

        dragInsertPointHighlight->setTargetPosition (insertPos, viewport->getViewWidth());
        dragTargetGroupHighlight->setTargetPosition (insertPos.item);
        */
    }
    
    pub fn hide_drag_highlight(&mut self)  {
        
        todo!();
        /*
            dragInsertPointHighlight = nullptr;
        dragTargetGroupHighlight = nullptr;
        */
    }
    
    pub fn handle_drag(&mut self, 
        files:               &Vec<String>,
        drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            const auto scrolled = viewport->autoScroll (dragSourceDetails.localPosition.x,
                                                    dragSourceDetails.localPosition.y, 20, 10);

        TreeViewInsertPoint insertPos (*this, files, dragSourceDetails);

        if (insertPos.item != nullptr)
        {
            if (scrolled || dragInsertPointHighlight == nullptr
                 || dragInsertPointHighlight->lastItem != insertPos.item
                 || dragInsertPointHighlight->lastIndex != insertPos.insertIndex)
            {
                if (files.size() > 0 ? insertPos.item->isInterestedInFileDrag (files)
                                     : insertPos.item->isInterestedInDragSource (dragSourceDetails))
                    showDragHighlight (insertPos);
                else
                    hideDragHighlight();
            }
        }
        else
        {
            hideDragHighlight();
        }
        */
    }
    
    pub fn handle_drop(&mut self, 
        files:               &Vec<String>,
        drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            hideDragHighlight();

        TreeViewInsertPoint insertPos (*this, files, dragSourceDetails);

        if (insertPos.item == nullptr)
            insertPos.item = rootItem;

        if (insertPos.item != nullptr)
        {
            if (files.size() > 0)
            {
                if (insertPos.item->isInterestedInFileDrag (files))
                    insertPos.item->filesDropped (files, insertPos.insertIndex);
            }
            else
            {
                if (insertPos.item->isInterestedInDragSource (dragSourceDetails))
                    insertPos.item->itemDropped (dragSourceDetails, insertPos.insertIndex);
            }
        }
        */
    }
    
    pub fn file_drag_enter(&mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32)  {
        
        todo!();
        /*
            fileDragMove (files, x, y);
        */
    }

    pub fn file_drag_move(&mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32)  {
        
        todo!();
        /*
            handleDrag (files, SourceDetails (var(), this, { x, y }));
        */
    }
    
    pub fn file_drag_exit(&mut self, _0: &Vec<String>)  {
        
        todo!();
        /*
            hideDragHighlight();
        */
    }
    
    pub fn files_dropped(&mut self, 
        files: &Vec<String>,
        x:     i32,
        y:     i32)  {
        
        todo!();
        /*
            handleDrop (files, SourceDetails (var(), this, { x, y }));
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class TableInterface  : public AccessibilityTableInterface
        {
        
            explicit TableInterface (TreeView& treeViewToWrap)  : treeView (treeViewToWrap) {}

            int getNumRows() const override     { return treeView.getNumRowsInTree(); }
            int getNumColumns() const override  { return 1; }

            const AccessibilityHandler* getCellHandler (int row, int) const override
            {
                if (auto* itemComp = treeView.getItemComponent (treeView.getItemOnRow (row)))
                    return itemComp->getAccessibilityHandler();

                return nullptr;
            }

        
            TreeView& treeView;

            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (TableInterface)
        };

        return std::make_unique<AccessibilityHandler> (*this,
                                                       AccessibilityRole::tree,
                                                       AccessibilityActions{},
                                                       AccessibilityHandler::Interfaces { std::make_unique<TableInterface> (*this) });
        */
    }
}
