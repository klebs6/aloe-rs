crate::ix!();

/**
  | This is a derivative of DocumentWindow
  | that is used inside a MultiDocumentPanel
  | component.
  | 
  | It's like a normal DocumentWindow but
  | has some extra functionality to make
  | sure everything works nicely inside
  | a MultiDocumentPanel.
  | 
  | @see MultiDocumentPanel
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MultiDocumentPanelWindow<'a> {
    base: DocumentWindow<'a>,
}

impl<'a> MultiDocumentPanelWindow<'a> {
    
    pub fn new(background_colour: Colour) -> Self {
    
        todo!();
        /*


            : DocumentWindow (String(), backgroundColour,
                          DocumentWindow::maximiseButton | DocumentWindow::closeButton, false)
        */
    }
    
    pub fn maximise_button_pressed(&mut self)  {
        
        todo!();
        /*
            if (auto* owner = getOwner())
            owner->setLayoutMode (MultiDocumentPanel::MaximisedWindowsWithTabs);
        else
            jassertfalse; // these windows are only designed to be used inside a MultiDocumentPanel!
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            if (auto* owner = getOwner())
            owner->closeDocumentAsync (getContentComponent(), true, nullptr);
        else
            jassertfalse; // these windows are only designed to be used inside a MultiDocumentPanel!
        */
    }
    
    pub fn active_window_status_changed(&mut self)  {
        
        todo!();
        /*
            DocumentWindow::activeWindowStatusChanged();
        updateOrder();
        */
    }
    
    pub fn brought_to_front(&mut self)  {
        
        todo!();
        /*
            DocumentWindow::broughtToFront();
        updateOrder();
        */
    }
    
    pub fn update_order(&mut self)  {
        
        todo!();
        /*
            if (auto* owner = getOwner())
            owner->updateOrder();
        */
    }
    
    pub fn get_owner(&self) -> *mut MultiDocumentPanel {
        
        todo!();
        /*
            return findParentComponentOfClass<MultiDocumentPanel>();
        */
    }
}
