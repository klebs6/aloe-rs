crate::ix!();

#[no_copy]
pub struct TreeViewTargetGroupHighlight<'a> {
    base: Component<'a>,
}

impl<'a> Default for TreeViewTargetGroupHighlight<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setAlwaysOnTop (true);
            setInterceptsMouseClicks (false, false);
        */
    }
}

impl<'a> TreeViewTargetGroupHighlight<'a> {

    pub fn set_target_position(&mut self, item: *mut TreeViewItem)  {
        
        todo!();
        /*
            setBounds (item->getItemPosition (true)
                         .withHeight (item->getItemHeight()));
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (findColour (TreeView::dragAndDropIndicatorColourId, true));
            g.drawRoundedRectangle (1.0f, 1.0f, (float) getWidth() - 2.0f, (float) getHeight() - 2.0f, 3.0f, 2.0f);
        */
    }
}
