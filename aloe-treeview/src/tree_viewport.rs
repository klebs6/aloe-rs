crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TreeViewport<'a> {
    base:  Viewport<'a>,
    base2: Timer,
    lastx: i32, // default = -1
}

impl<'a> TreeViewport<'a> {

    pub fn update_components(&mut self, trigger_resize: bool)  {
        
        todo!();
        /*
            if (auto* tvc = getContentComp())
            {
                if (triggerResize)
                    tvc->resized();
                else
                    tvc->updateComponents();
            }

            repaint();
        */
    }
    
    pub fn visible_area_changed(&mut self, new_visible_area: &Rectangle<i32>)  {
        
        todo!();
        /*
            const auto hasScrolledSideways = (newVisibleArea.getX() != lastX);
            lastX = newVisibleArea.getX();
            updateComponents (hasScrolledSideways);

            startTimer (50);
        */
    }
    
    pub fn get_content_comp(&self) -> *mut TreeViewContentComponent {
        
        todo!();
        /*
            return static_cast<TreeViewContentComponent*> (getViewedComponent());
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (auto* tree = getParentComponent())
                if (tree->keyPressed (key))
                    return true;

            return Viewport::keyPressed (key);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

            if (auto* tree = getParentComponent())
                if (auto* handler = tree->getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::structureChanged);
        */
    }
}
