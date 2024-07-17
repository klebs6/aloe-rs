crate::ix!();

pub trait TreeViewItemInterface:
MightContainSubItems
+ GetUniqueName 
+ ItemOpennessChanged
+ GetItemWidth
+ GetItemHeight
+ CanBeSelected
+ CreateItemComponent
+ PaintItem
+ PaintOpenCloseButton
+ PaintHorizontalConnectingLine
+ PaintVerticalConnectingLine
+ ItemClicked
+ ItemDoubleClicked
+ ItemSelectionChanged
+ OwnerViewChanged
+ GetToolTip
+ GetAccessibilityName
+ GetDragSourceDescription
+ IsInterestedInFileDrag
+ FilesDropped
+ IsInterestedInDragSource
+ ItemDropped
{}

pub trait MightContainSubItems {

    /**
      | Tells the tree whether this item can
      | potentially be opened.
      | 
      | If your item could contain sub-items,
      | this should return true; if it returns
      | false then the tree will not try to open
      | the item. This determines whether or
      | not the item will be drawn with a 'plus'
      | button next to it.
      |
      */
    fn might_contain_sub_items(&mut self) -> bool;
}

pub trait GetUniqueName {

    /**
      | Returns a string to uniquely identify
      | this item.
      | 
      | If you're planning on using the TreeView::getOpennessState()
      | method, then these strings will be used
      | to identify which nodes are open. The
      | string should be unique amongst the
      | item's sibling items, but it's ok for
      | there to be duplicates at other levels
      | of the tree.
      | 
      | If you're not going to store the state,
      | then it's ok not to bother implementing
      | this method.
      |
      */
    fn get_unique_name(&self) -> String;
}

pub trait ItemOpennessChanged {

    /**
      | Called when an item is opened or closed.
      | 
      | When setOpen() is called and the item
      | has specified that it might have sub-items
      | with the mightContainSubItems() method,
      | this method is called to let the item
      | create or manage its sub-items.
      | 
      | So when this is called with isNowOpen
      | set to true (i.e. when the item is being
      | opened), a subclass might choose to
      | use clearSubItems() and addSubItem()
      | to refresh its sub-item list.
      | 
      | When this is called with isNowOpen set
      | to false, the subclass might want to
      | use clearSubItems() to save on space,
      | or it might choose to leave them, depending
      | on the nature of the tree.
      | 
      | You could also use this callback as a
      | trigger to start a background process
      | which asynchronously creates sub-items
      | and adds them, if that's more appropriate
      | for the task in hand.
      | 
      | @see mightContainSubItems
      |
      */
    fn item_openness_changed(&mut self, is_now_open: bool);
}

pub trait GetItemWidth {

    /**
      | Must return the width required by this
      | item.
      | 
      | If your item needs to have a particular
      | width in pixels, return that value;
      | if you'd rather have it just fill whatever
      | space is available in the TreeView,
      | return -1.
      | 
      | If all your items return -1, no horizontal
      | scrollbar will be shown, but if any items
      | have fixed widths and extend beyond
      | the width of the TreeView, a scrollbar
      | will appear.
      | 
      | Each item can be a different width, but
      | if they change width, you should call
      | treeHasChanged() to update the tree.
      |
      */
    fn get_item_width(&self) -> i32 {
        
        todo!();
        /*
            return -1;
        */
    }
}

pub trait GetItemHeight {

    /**
      | Must return the height required by this
      | item.
      | 
      | This is the height in pixels that the
      | item will take up. Items in the tree can
      | be different heights, but if they change
      | height, you should call treeHasChanged()
      | to update the tree.
      |
      */
    fn get_item_height(&self) -> i32 { 20 }
}

pub trait CanBeSelected {

    /**
      | You can override this method to return
      | false if you don't want to allow the user
      | to select this item.
      |
      */
    fn can_be_selected(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

pub trait CreateItemComponent {

    /**
      | Creates a component that will be used
      | to represent this item.
      | 
      | You don't have to implement this method
      | - if it returns nullptr then no component
      | will be used for the item, and you can
      | just draw it using the paintItem() callback.
      | But if you do return a component, it will
      | be positioned in the
      | 
      | TreeView so that it can be used to represent
      | this item.
      | 
      | The component returned will be managed
      | by the TreeView and will be deleted later
      | when it goes off the screen or is no longer
      | needed. Its position and size will be
      | completely managed by the tree, so don't
      | attempt to move it around.
      | 
      | Something you may want to do with your
      | component is to give it a pointer to the
      | TreeView that created it. This is perfectly
      | safe, and there's no danger of it becoming
      | a dangling pointer because the TreeView
      | will always delete the component before
      | it is itself deleted.
      | 
      | As long as you stick to these rules you
      | can return whatever kind of component
      | you like. It's most useful if you're
      | doing things like drag-and-drop of
      | items, or want to use a Label component
      | to edit item names, etc.
      |
      */
    fn create_item_component(&mut self) -> Box<Component> {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

pub trait PaintItem {

    /**
      | Draws the item's contents.
      | 
      | You can choose to either implement this
      | method and draw each item, or you can
      | use createItemComponent() to create
      | a component that will represent the
      | item.
      | 
      | If all you need in your tree is to be able
      | to draw the items and detect when the
      | user selects or double-clicks one of
      | them, it's probably enough to use paintItem(),
      | itemClicked() and itemDoubleClicked().
      | If you need more complicated interactions,
      | you may need to use createItemComponent()
      | instead.
      | 
      | -----------
      | @param g
      | 
      | the graphics context to draw into
      | ----------
      | @param width
      | 
      | the width of the area available for drawing
      | ----------
      | @param height
      | 
      | the height of the area available for
      | drawing
      |
      */
    fn paint_item(&mut self, 
            g:      &mut Graphics,
            width:  i32,
            height: i32);
}

pub trait PaintOpenCloseButton {

    /**
      | Draws the item's open/close button.
      | 
      | If you don't implement this method,
      | the default behaviour is to call
      | 
      | LookAndFeel::drawTreeviewPlusMinusBox(),
      | but you can override it for custom effects.
      | You may want to override it and call the
      | base-class implementation with a different
      | backgroundColour parameter, if your
      | implementation has a background colour
      | other than the default (white).
      |
      */
    fn paint_open_close_button(&mut self, 
            _0:                &mut Graphics,
            area:              &Rectangle<f32>,
            background_colour: Colour,
            is_mouse_over:     bool);
}

pub trait PaintHorizontalConnectingLine {

    /**
      | Draws the line that connects this item
      | to the vertical line extending below
      | its parent.
      |
      */
    fn paint_horizontal_connecting_line(&mut self, 
            _0:   &mut Graphics,
            line: &Line<f32>);
}
    
pub trait PaintVerticalConnectingLine {

    /**
      | Draws the line that extends vertically
      | up towards one of its parents, or down
      | to one of its children.
      |
      */
    fn paint_vertical_connecting_line(&mut self, 
            _0:   &mut Graphics,
            line: &Line<f32>);
}

pub trait ItemClicked {

    /**
      | Called when the user clicks on this item.
      | 
      | If you're using createItemComponent()
      | to create a custom component for the
      | item, the mouse-clicks might not make
      | it through to the TreeView, but this
      | is how you find out about clicks when
      | just drawing each item individually.
      | 
      | The associated mouse-event details
      | are passed in, so you can find out about
      | which button, where it was, etc.
      | 
      | @see itemDoubleClicked
      |
      */
    fn item_clicked(&mut self, _0: &MouseEvent);
}

pub trait ItemDoubleClicked {

    /**
      | Called when the user double-clicks
      | on this item.
      | 
      | If you're using createItemComponent()
      | to create a custom component for the
      | item, the mouse-clicks might not make
      | it through to the TreeView, but this
      | is how you find out about clicks when
      | just drawing each item individually.
      | 
      | The associated mouse-event details
      | are passed in, so you can find out about
      | which button, where it was, etc.
      | 
      | If not overridden, the base class method
      | here will open or close the item as if
      | the 'plus' button had been clicked.
      | 
      | @see itemClicked
      |
      */
    fn item_double_clicked(&mut self, _0: &MouseEvent);
}

pub trait ItemSelectionChanged {

    /**
      | Called when the item is selected or deselected.
      | 
      | Use this if you want to do something special
      | when the item's selectedness changes.
      | By default it'll get repainted when
      | this happens.
      |
      */
    fn item_selection_changed(&mut self, is_now_selected: bool);
}

pub trait OwnerViewChanged {

    /**
      | Called when the owner view changes
      |
      */
    fn owner_view_changed(&mut self, new_owner: *mut TreeView);
}

pub trait GetToolTip {

    /**
      | The item can return a tool tip string
      | here if it wants to.
      | 
      | @see TooltipClient
      |
      */
    fn get_tooltip(&mut self) -> String;
}

pub trait GetAccessibilityName {

    /**
      | Use this to set the name for this item
      | that will be read out by accessibility
      | clients.
      | 
      | The default implementation will return
      | the tooltip string from getTooltip()
      | if it is not empty, otherwise it will
      | return a description of the nested level
      | and row number of the item.
      | 
      | @see AccessibilityHandler
      |
      */
    fn get_accessibility_name(&mut self) -> String;
}

pub trait GetDragSourceDescription {

    /**
      | To allow items from your TreeView to
      | be dragged-and-dropped, implement
      | this method.
      | 
      | If this returns a non-null variant then
      | when the user drags an item, the TreeView
      | will try to find a DragAndDropContainer
      | in its parent hierarchy, and will use
      | it to trigger a drag-and-drop operation,
      | using this string as the source description,
      | with the TreeView itself as the source
      | component.
      | 
      | If you need more complex drag-and-drop
      | behaviour, you can use custom components
      | for the items, and use those to trigger
      | the drag.
      | 
      | To accept drag-and-drop in your tree,
      | see isInterestedInDragSource(),
      | isInterestedInFileDrag(), etc.
      | 
      | @see DragAndDropContainer::startDragging
      |
      */
    fn get_drag_source_description(&mut self) -> Var;
}

pub trait IsInterestedInFileDrag {

    /**
      | If you want your item to be able to have
      | files drag-and-dropped onto it, implement
      | this method and return true.
      | 
      | If you return true and allow some files
      | to be dropped, you'll also need to implement
      | the filesDropped() method to do something
      | with them.
      | 
      | -----------
      | @note
      | 
      | this will be called often, so make your
      | implementation very quick! There's
      | certainly no time to try opening the
      | files and having a think about what's
      | inside them!
      | 
      | For responding to internal drag-and-drop
      | of other types of object, see isInterestedInDragSource().
      | 
      | @see FileDragAndDropTarget::isInterestedInFileDrag,
      | isInterestedInDragSource
      |
      */
    fn is_interested_in_file_drag(&mut self, files: &Vec<String>) -> bool;
}

pub trait FilesDropped {

    /**
      | When files are dropped into this item,
      | this callback is invoked.
      | 
      | For this to work, you'll need to have
      | also implemented isInterestedInFileDrag().
      | 
      | The insertIndex value indicates where
      | in the list of sub-items the files were
      | dropped.
      | 
      | If files are dropped onto an area of the
      | tree where there are no visible items,
      | this method is called on the root item
      | of the tree, with an insert index of 0.
      | 
      | @see FileDragAndDropTarget::filesDropped,
      | isInterestedInFileDrag
      |
      */
    fn files_dropped(&mut self, 
            files:        &Vec<String>,
            insert_index: i32);
}

pub trait IsInterestedInDragSource {

    /**
      | If you want your item to act as a DragAndDropTarget,
      | implement this method and return true.
      | 
      | If you implement this method, you'll
      | also need to implement itemDropped()
      | in order to handle the items when they
      | are dropped.
      | 
      | To respond to drag-and-drop of files
      | from external applications, see isInterestedInFileDrag().
      | 
      | @see DragAndDropTarget::isInterestedInDragSource,
      | itemDropped
      |
      */
    fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) -> bool;
}

pub trait ItemDropped {

    /**
      | When a things are dropped into this item,
      | this callback is invoked.
      | 
      | For this to work, you need to have also
      | implemented isInterestedInDragSource().
      | 
      | The insertIndex value indicates where
      | in the list of sub-items the new items
      | should be placed.
      | 
      | If files are dropped onto an area of the
      | tree where there are no visible items,
      | this method is called on the root item
      | of the tree, with an insert index of 0.
      | 
      | @see isInterestedInDragSource, DragAndDropTarget::itemDropped
      |
      */
    fn item_dropped(&mut self, 
            drag_source_details: &DragAndDropTargetSourceDetails,
            insert_index:        i32);
}
