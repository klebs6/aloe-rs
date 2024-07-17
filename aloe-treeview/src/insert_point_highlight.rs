crate::ix!();

#[no_copy]
pub struct InsertPointHighlight<'a> {
    base:       Component<'a>,
    last_item:  *mut TreeViewItem<'a>, // default = nullptr
    last_index: i32, // default = 0
}

impl<'a> Default for InsertPointHighlight<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setSize (100, 12);
            setAlwaysOnTop (true);
            setInterceptsMouseClicks (false, false);
        */
    }
}

impl<'a> InsertPointHighlight<'a> {
    
    pub fn set_target_position(&mut self, 
        insert_pos: &TreeViewInsertPoint,
        width:      i32)  {
        
        todo!();
        /*
            lastItem = insertPos.item;
            lastIndex = insertPos.insertIndex;
            auto offset = getHeight() / 2;
            setBounds (insertPos.pos.x - offset, insertPos.pos.y - offset,
                       width - (insertPos.pos.x - offset), getHeight());
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            Path p;
            auto h = (float) getHeight();
            p.addEllipse (2.0f, 2.0f, h - 4.0f, h - 4.0f);
            p.startNewSubPath (h - 2.0f, h / 2.0f);
            p.lineTo ((float) getWidth(), h / 2.0f);

            g.setColour (findColour (TreeView::dragAndDropIndicatorColourId, true));
            g.strokePath (p, PathStrokeType (2.0f));
        */
    }
}
