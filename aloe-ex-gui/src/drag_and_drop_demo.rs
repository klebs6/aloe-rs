crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DragAndDropDemo<'a> {
    base:            Component<'a>,
    base2:           DragAndDropContainer<'a>,
    source_list_box: ListBox<'a>, // default = { "D+D source", nullptr  }
    source_model:    SourceItemListboxContents,
    target:          DragAndDropDemoTarget<'a>,
}

impl<'a> Default for DragAndDropDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setName ("Drag-and-Drop");

            sourceListBox.setModel (&sourceModel);
            sourceListBox.setMultipleSelectionEnabled (true);

            addAndMakeVisible (sourceListBox);
            addAndMakeVisible (target)
        */
    }
}

impl<'a> DragAndDropDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (8);

            sourceListBox.setBounds (r.withSize (250, 180));
            target       .setBounds (r.removeFromBottom (150).removeFromRight (250));
        */
    }
}
