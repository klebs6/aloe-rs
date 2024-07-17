crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ListBoxRowComponent<'a> {
    base:                   Component<'a>,
    owner:                  &'a mut ListBox<'a>,
    custom_component:       Box<Component<'a>>,
    row:                    i32, // default = -1
    is_selected:            bool, // default = false
    is_dragging:            bool, // default = false
    is_dragging_to_scroll:  bool, // default = false
    select_row_on_mouse_up: bool, // default = false
}

impl<'a> TooltipClient for ListBoxRowComponent<'a> {

    fn get_tooltip(&mut self) -> String {
        
        todo!();
        /*
            if (auto* m = owner.getModel())
                return m->getTooltipForRow (row);

            return {};
        */
    }
}

impl<'a> ListBoxRowComponent<'a> {

    pub fn new(lb: &mut ListBox) -> Self {
    
        todo!();
        /*
        : owner(lb),

        
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (auto* m = owner.getModel())
                m->paintListBoxItem (row, g, getWidth(), getHeight(), isSelected);
        */
    }
    
    pub fn update(&mut self, 
        new_row:      i32,
        now_selected: bool)  {
        
        todo!();
        /*
            const auto rowHasChanged = (row != newRow);
            const auto selectionHasChanged = (isSelected != nowSelected);

            if (rowHasChanged || selectionHasChanged)
            {
                repaint();

                if (rowHasChanged)
                    row = newRow;

                if (selectionHasChanged)
                    isSelected = nowSelected;
            }

            if (auto* m = owner.getModel())
            {
                setMouseCursor (m->getMouseCursorForRow (row));

                customComponent.reset (m->refreshComponentForRow (newRow, nowSelected, customComponent.release()));

                if (customComponent != nullptr)
                {
                    addAndMakeVisible (customComponent.get());
                    customComponent->setBounds (getLocalBounds());

                    if (customComponent->getAccessibilityHandler() != nullptr)
                        invalidateAccessibilityHandler();
                }
            }

            if (selectionHasChanged)
                if (auto* handler = getAccessibilityHandler())
                    isSelected ? handler->grabFocus() : handler->giveAwayFocus();
        */
    }
    
    pub fn perform_selection(&mut self, 
        e:           &MouseEvent,
        is_mouse_up: bool)  {
        
        todo!();
        /*
            owner.selectRowsBasedOnModifierKeys (row, e.mods, isMouseUp);

            if (auto* m = owner.getModel())
                m->listBoxItemClicked (row, e);
        */
    }
    
    pub fn is_in_drag_to_scroll_viewport(&self) -> bool {
        
        todo!();
        /*
            if (auto* vp = owner.getViewport())
                return vp->isScrollOnDragEnabled() && (vp->canScrollVertically() || vp->canScrollHorizontally());

            return false;
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            isDragging = false;
            isDraggingToScroll = false;
            selectRowOnMouseUp = false;

            if (isEnabled())
            {
                if (owner.selectOnMouseDown && ! (isSelected || isInDragToScrollViewport()))
                    performSelection (e, false);
                else
                    selectRowOnMouseUp = true;
            }
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled() && selectRowOnMouseUp && ! (isDragging || isDraggingToScroll))
                performSelection (e, true);
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled())
                if (auto* m = owner.getModel())
                    m->listBoxItemDoubleClicked (row, e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (auto* m = owner.getModel())
            {
                if (isEnabled() && e.mouseWasDraggedSinceMouseDown() && ! isDragging)
                {
                    SparseSet<int> rowsToDrag;

                    if (owner.selectOnMouseDown || owner.isRowSelected (row))
                        rowsToDrag = owner.getSelectedRows();
                    else
                        rowsToDrag.addRange (Range<int>::withStartAndLength (row, 1));

                    if (rowsToDrag.size() > 0)
                    {
                        auto dragDescription = m->getDragSourceDescription (rowsToDrag);

                        if (! (dragDescription.isVoid() || (dragDescription.isString() && dragDescription.toString().isEmpty())))
                        {
                            isDragging = true;
                            owner.startDragAndDrop (e, rowsToDrag, dragDescription, true);
                        }
                    }
                }
            }

            if (! isDraggingToScroll)
                if (auto* vp = owner.getViewport())
                    isDraggingToScroll = vp->isCurrentlyScrollingOnDrag();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (customComponent != nullptr)
                customComponent->setBounds (getLocalBounds());
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            if (customComponent != nullptr && customComponent->getAccessibilityHandler() != nullptr)
                return nullptr;

            return std::make_unique<RowAccessibilityHandler> (*this);
        */
    }
}
