crate::ix!();

pub type LoopPointMarkerMouseCallback = fn(_0: &mut LoopPointMarker, _1: &MouseEvent) -> ();

pub struct LoopPointMarker<'a> {
    base:          Component<'a>,
    text:          String,
    path:          Path,
    on_mouse_down: LoopPointMarkerMouseCallback,
    on_mouse_drag: LoopPointMarkerMouseCallback,
    on_mouse_up:   LoopPointMarkerMouseCallback,
}

impl<'a> LoopPointMarker<'a> {

    pub fn new(
        marker:           String,
        on_mouse_down_in: LoopPointMarkerMouseCallback,
        on_mouse_drag_in: LoopPointMarkerMouseCallback,
        on_mouse_up_in:   LoopPointMarkerMouseCallback) -> Self {
    
        todo!();
        /*


            : text (std::move (marker)),
              onMouseDown (std::move (onMouseDownIn)),
              onMouseDrag (std::move (onMouseDragIn)),
              onMouseUp (std::move (onMouseUpIn))

            setMouseCursor (MouseCursor::LeftRightResizeCursor);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto height = 20;
            auto triHeight = 6;

            auto bounds = getLocalBounds();
            Path newPath;
            newPath.addRectangle (bounds.removeFromBottom (height));

            newPath.startNewSubPath (bounds.getBottomLeft().toFloat());
            newPath.lineTo (bounds.getBottomRight().toFloat());
            Point<float> apex (static_cast<float> (bounds.getX() + (bounds.getWidth() / 2)),
                               static_cast<float> (bounds.getBottom() - triHeight));
            newPath.lineTo (apex);
            newPath.closeSubPath();

            newPath.addLineSegment (Line<float> (apex, Point<float> (apex.getX(), 0)), 1);

            path = newPath;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::deepskyblue);
            g.fillPath (path);

            auto height = 20;
            g.setColour (Colours::white);
            g.drawText (text, getLocalBounds().removeFromBottom (height), Justification::centred);
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            return path.contains ((float) x, (float) y);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            onMouseDown (*this, e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            onMouseDrag (*this, e);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            onMouseUp (*this, e);
        */
    }
}
