crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ItemDragAndDropOverlayComponent<'a> {
    base:        Component<'a>,
    is_dragging: bool,
}

impl<'a> Default for ItemDragAndDropOverlayComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : is_dragging(false),

            setAlwaysOnTop (true);
            setRepaintsOnMouseActivity (true);
            setMouseCursor (MouseCursor::DraggingHandCursor);
        */
    }
}

impl<'a> ItemDragAndDropOverlayComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (ToolbarItemComponent* const tc = getToolbarItemComponent())
            {
                if (isMouseOverOrDragging()
                      && tc->getEditingMode() == ToolbarItemComponent::editableOnToolbar)
                {
                    g.setColour (findColour (Toolbar::editingModeOutlineColourId, true));
                    g.drawRect (getLocalBounds(), jmin (2, (getWidth() - 1) / 2,
                                                           (getHeight() - 1) / 2));
                }
            }
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            isDragging = false;

            if (ToolbarItemComponent* const tc = getToolbarItemComponent())
            {
                tc->dragOffsetX = e.x;
                tc->dragOffsetY = e.y;
            }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.mouseWasDraggedSinceMouseDown() && ! isDragging)
            {
                isDragging = true;

                if (DragAndDropContainer* const dnd = DragAndDropContainer::findParentDragContainerFor (this))
                {
                    dnd->startDragging (Toolbar::toolbarDragDescriptor, getParentComponent(), Image(), true, nullptr, &e.source);

                    if (ToolbarItemComponent* const tc = getToolbarItemComponent())
                    {
                        tc->isBeingDragged = true;

                        if (tc->getEditingMode() == ToolbarItemComponent::editableOnToolbar)
                            tc->setVisible (false);
                    }
                }
            }
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            isDragging = false;

            if (ToolbarItemComponent* const tc = getToolbarItemComponent())
            {
                tc->isBeingDragged = false;

                if (Toolbar* const tb = tc->getToolbar())
                    tb->updateAllItemPositions (true);
                else if (tc->getEditingMode() == ToolbarItemComponent::editableOnToolbar)
                    delete tc;
            }
        */
    }
    
    pub fn parent_size_changed(&mut self)  {
        
        todo!();
        /*
            setBounds (0, 0, getParentWidth(), getParentHeight());
        */
    }
    
    pub fn get_toolbar_item_component(&self) -> *mut dyn ToolbarItemComponent {
        
        todo!();
        /*
            return dynamic_cast<ToolbarItemComponent*> (getParentComponent());
        */
    }
}
