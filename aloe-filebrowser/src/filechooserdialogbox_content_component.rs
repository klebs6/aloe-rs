crate::ix!();

pub struct FileChooserDialogBoxContentComponent<'a> {
    base:              Component<'a>,
    chooser_component: &'a mut FileBrowserComponent<'a>,
    ok_button:         TextButton<'a>,
    cancel_button:     TextButton<'a>,
    new_folder_button: TextButton<'a>,
    instructions:      String,
    text:              TextLayout,
}

impl<'a> FileChooserDialogBoxContentComponent<'a> {

    pub fn new(
        name:    &String,
        desc:    &String,
        chooser: &mut FileBrowserComponent) -> Self {
    
        todo!();
        /*


            : Component (name),
                  chooserComponent (chooser),
                  okButton (chooser.getActionVerb()),
                  cancelButton (TRANS ("Cancel")),
                  newFolderButton (TRANS ("New Folder")),
                  instructions (desc)

                addAndMakeVisible (chooserComponent);

                addAndMakeVisible (okButton);
                okButton.addShortcut (KeyPress (KeyPress::returnKey));

                addAndMakeVisible (cancelButton);
                cancelButton.addShortcut (KeyPress (KeyPress::escapeKey));

                addChildComponent (newFolderButton);

                setInterceptsMouseClicks (false, true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            text.draw (g, getLocalBounds().reduced (6)
                                .removeFromTop ((int) text.getHeight()).toFloat());
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            const int buttonHeight = 26;

                auto area = getLocalBounds();

                text.createLayout (getLookAndFeel().createFileChooserHeaderText (getName(), instructions),
                                   (float) getWidth() - 12.0f);

                area.removeFromTop (roundToInt (text.getHeight()) + 10);

                chooserComponent.setBounds (area.removeFromTop (area.getHeight() - buttonHeight - 20));
                auto buttonArea = area.reduced (16, 10);

                okButton.changeWidthToFitText (buttonHeight);
                okButton.setBounds (buttonArea.removeFromRight (okButton.getWidth() + 16));

                buttonArea.removeFromRight (16);

                cancelButton.changeWidthToFitText (buttonHeight);
                cancelButton.setBounds (buttonArea.removeFromRight (cancelButton.getWidth()));

                newFolderButton.changeWidthToFitText (buttonHeight);
                newFolderButton.setBounds (buttonArea.removeFromLeft (newFolderButton.getWidth()));
        */
    }
}
