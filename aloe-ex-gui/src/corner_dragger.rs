crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CornerDragger<'a> {
    base:         Component<'a>,
    relative_pos: Point<f32>,
    constrainer:  ComponentBoundsConstrainer,
    dragger:      ComponentDragger,
}

impl<'a> Default for CornerDragger<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setSize (30, 30);
                setRepaintsOnMouseActivity (true)
        */
    }
}

impl<'a> CornerDragger<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::white.withAlpha (isMouseOverOrDragging() ? 0.9f : 0.5f));
                g.fillEllipse (getLocalBounds().reduced (3).toFloat());

                g.setColour (Colours::darkgreen);
                g.drawEllipse (getLocalBounds().reduced (3).toFloat(), 2.0f);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            constrainer.setMinimumOnscreenAmounts (getHeight(), getWidth(), getHeight(), getWidth());
        */
    }
    
    pub fn moved(&mut self)  {
        
        todo!();
        /*
            if (isMouseButtonDown())
                    relativePos = getBounds().getCentre().toFloat() / Point<int> (getParentWidth(), getParentHeight()).toFloat();
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            dragger.startDraggingComponent (this, e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            dragger.dragComponent (this, e, &constrainer);
        */
    }
}
