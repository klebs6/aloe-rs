crate::ix!();

/**
  | and this is a component that can have
  | things dropped onto it..
  |
  */
#[derive(Default)]
pub struct DragAndDropDemoTarget<'a> {
    base:                            Component<'a>,
    message:                         String, // default = { "Drag-and-drop some rows from the top-left box onto this component!\n\n You can also drag-and-drop files and text from other apps" }
    something_is_being_dragged_over: bool,   // default = false
}

impl<'a> DragAndDropTarget for DragAndDropDemoTarget<'a> {

}

impl<'a> IsInterestedInDragSource for DragAndDropDemoTarget<'a> {

    /*
       | These methods implement the
       | DragAndDropTarget interface, and allow our
       | component to accept drag-and-drop of
       | objects from other Aloe components..
       */
    fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails) -> bool {
        
        todo!();
        /*
            // normally you'd check the sourceDescription value to see if it's the
                // sort of object that you're interested in before returning true, but for
                // the demo, we'll say yes to anything..
                return true;
        */
    }
}

impl<'a> ItemDragEnter for DragAndDropDemoTarget<'a> {

}

impl<'a> ItemDragMove for DragAndDropDemoTarget<'a> {

}

impl<'a> ItemDragExit for DragAndDropDemoTarget<'a> {

}

impl<'a> ItemDropped for DragAndDropDemoTarget<'a> {

    fn item_dropped(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            message = "Items dropped: " + dragSourceDetails.description.toString();

                somethingIsBeingDraggedOver = false;
                repaint();
        */
    }
}

impl<'a> FileDragAndDropTarget for DragAndDropDemoTarget<'a> {

}

impl<'a> ShouldDrawDragImageWhenOver for DragAndDropDemoTarget<'a> {

}

impl<'a> TextDragAndDropTarget for DragAndDropDemoTarget<'a> {

}

impl<'a> IsInterestedInTextDrag for DragAndDropDemoTarget<'a> {

    fn is_interested_in_text_drag(&mut self, text: &str) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl<'a> TextDragEnter for DragAndDropDemoTarget<'a> {

}

impl<'a> TextDragMove for DragAndDropDemoTarget<'a> {

}

impl<'a> TextDragExit for DragAndDropDemoTarget<'a> {

}

impl<'a> TextDropped for DragAndDropDemoTarget<'a> {

    fn text_dropped(
        &mut self, 
        text: &str,
        x:    i32,
        y:    i32)  {
        
        todo!();
        /*
            message = "Text dropped:\n" + text;

                somethingIsBeingDraggedOver = false;
                repaint();
        */
    }
}

impl<'a> DragAndDropDemoTarget<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::green.withAlpha (0.2f));

                // draw a red line around the comp if the user's currently dragging something over it..
                if (somethingIsBeingDraggedOver)
                {
                    g.setColour (Colours::red);
                    g.drawRect (getLocalBounds(), 3);
                }

                g.setColour (getLookAndFeel().findColour (Label::textColourId));
                g.setFont (14.0f);
                g.drawFittedText (message, getLocalBounds().reduced (10, 0), Justification::centred, 4);
        */
    }

    pub fn item_drag_enter(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            somethingIsBeingDraggedOver = true;
                repaint();
        */
    }
    
    pub fn item_drag_move(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn item_drag_exit(&mut self, drag_source_details: &DragAndDropTargetSourceDetails)  {
        
        todo!();
        /*
            somethingIsBeingDraggedOver = false;
                repaint();
        */
    }
    
    /*
      | These methods implement the
      | FileDragAndDropTarget interface, and
      | allow our component to accept
      | drag-and-drop of files..
      */
    
    pub fn is_interested_in_file_drag(&mut self, files: &StringArray) -> bool {
        
        todo!();
        /*
            // normally you'd check these files to see if they're something that you're
                // interested in before returning true, but for the demo, we'll say yes to anything..
                return true;
        */
    }
    
    pub fn file_drag_enter(&mut self, 
        files: &StringArray,
        x:     i32,
        y:     i32)  {
        
        todo!();
        /*
            somethingIsBeingDraggedOver = true;
                repaint();
        */
    }
    
    pub fn file_drag_move(&mut self, 
        files: &StringArray,
        x:     i32,
        y:     i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn file_drag_exit(&mut self, files: &StringArray)  {
        
        todo!();
        /*
            somethingIsBeingDraggedOver = false;
                repaint();
        */
    }
    
    pub fn files_dropped(&mut self, 
        files: &StringArray,
        x:     i32,
        y:     i32)  {
        
        todo!();
        /*
            message = "Files dropped: " + files.joinIntoString ("\n");

                somethingIsBeingDraggedOver = false;
                repaint();
        */
    }

    /*
      | These methods implement the
      | TextDragAndDropTarget interface, and
      | allow our component to accept
      | drag-and-drop of text..
      */

    pub fn text_drag_enter(&mut self, 
        text: &String,
        x:    i32,
        y:    i32)  {
        
        todo!();
        /*
            somethingIsBeingDraggedOver = true;
                repaint();
        */
    }
    
    pub fn text_drag_move(&mut self, 
        text: &String,
        x:    i32,
        y:    i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn text_drag_exit(&mut self, text: &String)  {
        
        todo!();
        /*
            somethingIsBeingDraggedOver = false;
                repaint();
        */
    }
    
}
