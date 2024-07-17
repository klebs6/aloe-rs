crate::ix!();

/**
  | Simple multi-document panel that manages
  | a number of notes that you can store to
  | files.
  | 
  | By default this will look for notes saved
  | to the desktop and load them up.
  |
  */
#[no_copy]
#[leak_detector]
pub struct MDIDemo<'a> {
    base:                     Component<'a>,
    show_in_tabs_button:      ToggleButton<'a>, // default = { "Show with tabs"  }
    add_note_button:          TextButton<'a>, // default = { "Create a new note"  }
    close_application_button: TextButton<'a>, // default = { "Close app"  }
    multi_document_panel:     DemoMultiDocumentPanel<'a>,
}

impl<'a> FileDragAndDropTarget for MDIDemo<'a> {

}

impl<'a> IsInterestedInDragSource for MDIDemo<'a> {

    fn is_interested_in_drag_source(&mut self, _: &aloe_drag_and_drop::DragAndDropTargetSourceDetails<'_>) -> bool { todo!() }
}

impl<'a> ItemDragEnter for MDIDemo<'a> {

}

impl<'a> ItemDragMove for MDIDemo<'a> {

}

impl<'a> ItemDragExit for MDIDemo<'a> {

}

impl<'a> ItemDropped for MDIDemo<'a> {

    fn item_dropped(&mut self, _: &aloe_drag_and_drop::DragAndDropTargetSourceDetails<'_>) { todo!() }
}

impl<'a> ShouldDrawDragImageWhenOver for MDIDemo<'a> {

}

impl<'a> Default for MDIDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            showInTabsButton.setToggleState (false, dontSendNotification);
            showInTabsButton.onClick = [this] { updateLayoutMode(); };
            addAndMakeVisible (showInTabsButton);

            addNoteButton.onClick = [this] { addNote ("Note " + String (multiDocumentPanel.getNumDocuments() + 1), "Hello World!"); };
            addAndMakeVisible (addNoteButton);

            closeApplicationButton.onClick = [this]
            {
                multiDocumentPanel.closeAllDocumentsAsync (true, [] (bool allSaved)
                {
                    if (allSaved)
                        ALOEApplicationBase::quit();
                });
            };
            addAndMakeVisible (closeApplicationButton);

            addAndMakeVisible (multiDocumentPanel);
            multiDocumentPanel.setBackgroundColour (Colours::transparentBlack);

            updateLayoutMode();
            addNote ("Notes Demo", "You can drag-and-drop text files onto this page to open them as notes..");
            addExistingNotes();

            setSize (500, 500)
        */
    }
}

impl<'a> MDIDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            auto buttonArea = area.removeFromTop (28).reduced (2);
            closeApplicationButton.setBounds (buttonArea.removeFromRight (150));
            addNoteButton         .setBounds (buttonArea.removeFromRight (150));
            showInTabsButton      .setBounds (buttonArea);

            multiDocumentPanel.setBounds (area);
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, _0: &StringArray) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn files_dropped(&mut self, 
        filenames: &StringArray,
        x:         i32,
        y:         i32)  {
        
        todo!();
        /*
            Vec<File> files;

            for (auto& f : filenames)
                files.add ({ f });

            createNotesForFiles (files);
        */
    }
    
    pub fn create_notes_for_files(&mut self, files: &[File])  {
        
        todo!();
        /*
            for (auto& file : files)
            {
                auto content = file.loadFileAsString();

                if (content.length() > 20000)
                    content = "Too long!";

                addNote (file.getFileName(), content);
            }
        */
    }
    
    pub fn update_layout_mode(&mut self)  {
        
        todo!();
        /*
            multiDocumentPanel.setLayoutMode (showInTabsButton.getToggleState() ? MultiDocumentPanel::MaximisedWindowsWithTabs
                                                                                : MultiDocumentPanel::FloatingWindows);
        */
    }
    
    pub fn add_note(&mut self, 
        name:    &String,
        content: &String)  {
        
        todo!();
        /*
            auto* newNote = new Note (name, content);
            newNote->setSize (200, 200);

            multiDocumentPanel.addDocument (newNote, Colours::lightblue.withAlpha (0.6f), true);
        */
    }
    
    pub fn add_existing_notes(&mut self)  {
        
        todo!();
        /*
            Vec<File> files;
            File::getSpecialLocation (File::userDesktopDirectory).findChildFiles (files, File::findFiles, false, "*.jnote");
            createNotesForFiles (files);
        */
    }
}
