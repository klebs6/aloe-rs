crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ViewportDragToScrollListener<'a> {
    viewport:                 &'a mut Viewport<'a>,
    offsetx:                  ViewportDragPosition,
    offsety:                  ViewportDragPosition,
    original_view_pos:        Point<i32>,
    is_dragging:              bool, // default = false
    is_global_mouse_listener: bool, // default = false
}

impl<'a> ViewportDragPositionListener for ViewportDragToScrollListener<'a> {

}

impl<'a> AnimatedPositionListener<ContinuousWithMomentum> for ViewportDragToScrollListener<'a> {

    fn position_changed(&mut self, 
        _0: &mut /* ViewportDragPosition */ ContinuousWithMomentum,
        _1: f64)  {
        
        todo!();
        /*
            viewport.setViewPosition (originalViewPos - Point<int> ((int) offsetX.getPosition(),
                (int) offsetY.getPosition()));
        */
    }
}

impl<'a> MouseListener for ViewportDragToScrollListener<'a> {

}

impl<'a> MouseMagnify     for ViewportDragToScrollListener<'a> { }
impl<'a> MouseWheelMove   for ViewportDragToScrollListener<'a> { }
impl<'a> MouseDoubleClick for ViewportDragToScrollListener<'a> { }

impl<'a> MouseUp for ViewportDragToScrollListener<'a> { 

    fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (isGlobalMouseListener && Desktop::getInstance().getNumDraggingMouseSources() == 0)
                    endDragAndClearGlobalMouseListener();
        */
    }
}

impl<'a> MouseDown for ViewportDragToScrollListener<'a> { 

    fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (! isGlobalMouseListener)
                {
                    offsetX.setPosition (offsetX.getPosition());
                    offsetY.setPosition (offsetY.getPosition());

                    // switch to a global mouse listener so we still receive mouseUp events
                    // if the original event component is deleted
                    viewport.contentHolder.removeMouseListener (this);
                    Desktop::getInstance().addGlobalMouseListener (this);

                    isGlobalMouseListener = true;
                }
        */
    }
}

impl<'a> MouseExit  for ViewportDragToScrollListener<'a> { }
impl<'a> MouseEnter for ViewportDragToScrollListener<'a> { }
impl<'a> MouseMove  for ViewportDragToScrollListener<'a> { }

impl<'a> MouseDrag for ViewportDragToScrollListener<'a> { 

    fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (Desktop::getInstance().getNumDraggingMouseSources() == 1 && ! doesMouseEventComponentBlockViewportDrag (e.eventComponent))
                {
                    auto totalOffset = e.getOffsetFromDragStart().toFloat();

                    if (! isDragging && totalOffset.getDistanceFromOrigin() > 8.0f)
                    {
                        isDragging = true;

                        originalViewPos = viewport.getViewPosition();
                        offsetX.setPosition (0.0);
                        offsetX.beginDrag();
                        offsetY.setPosition (0.0);
                        offsetY.beginDrag();
                    }

                    if (isDragging)
                    {
                        offsetX.drag (totalOffset.x);
                        offsetY.drag (totalOffset.y);
                    }
                }
        */
    }
}

impl<'a> Drop for ViewportDragToScrollListener<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
            viewport.contentHolder.removeMouseListener (this);
            Desktop::getInstance().removeGlobalMouseListener (this);
         */
    }
}

impl<'a> ViewportDragToScrollListener<'a> {

    pub fn new(v: &mut Viewport) -> Self {
    
        todo!();
        /*
        : viewport(v),

            viewport.contentHolder.addMouseListener (this, true);
                offsetX.addListener (this);
                offsetY.addListener (this);
                offsetX.behaviour.setMinimumVelocity (60);
                offsetY.behaviour.setMinimumVelocity (60);
        */
    }
    
    pub fn end_drag_and_clear_global_mouse_listener(&mut self)  {
        
        todo!();
        /*
            offsetX.endDrag();
                offsetY.endDrag();
                isDragging = false;

                viewport.contentHolder.addMouseListener (this, true);
                Desktop::getInstance().removeGlobalMouseListener (this);

                isGlobalMouseListener = false;
        */
    }
    
    pub fn does_mouse_event_component_block_viewport_drag(&mut self, event_comp: *const Component) -> bool {
        
        todo!();
        /*
            for (auto c = eventComp; c != nullptr && c != &viewport; c = c->getParentComponent())
                    if (c->getViewportIgnoreDragFlag())
                        return true;

                return false;
        */
    }
}
