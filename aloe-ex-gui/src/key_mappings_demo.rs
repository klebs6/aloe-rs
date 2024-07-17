crate::ix!();

#[no_copy]
#[leak_detector]
pub struct KeyMappingsDemo<'a> {
    base:               Component<'a>,

    #[cfg(feature = "demorunner")]
    command_manager:    &'a mut ApplicationCommandManager<'a>, // default = getGlobalCommandManager()

    #[cfg(not(feature = "demorunner"))]
    command_manager:    ApplicationCommandManager<'a>,

    key_mapping_editor: KeyMappingEditorComponent<'a>, //{ *commandManager.getKeyMappings(), true};
    key_target:         KeyPressTarget<'a>,
}

impl<'a> Default for KeyMappingsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            // register the commands that the target component can perform
            commandManager.registerAllCommandsForTarget (&keyTarget);

            setOpaque (true);
            addAndMakeVisible (keyMappingEditor);
            addAndMakeVisible (keyTarget);

            // add command manager key mappings as a KeyListener to the top-level component
            // so it is notified of key presses
            getTopLevelComponent()->addKeyListener (commandManager.getKeyMappings());

            setSize (500, 500);

            Timer::callAfterDelay (300, [this] { keyTarget.grabKeyboardFocus(); }); // ensure that key presses are sent to the KeyPressTarget objec
        */
    }
}

impl<'a> Paint for KeyMappingsDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colour::greyLevel (0.93f)));
        */
    }
}
    
impl<'a> Resized for KeyMappingsDemo<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

            keyTarget       .setBounds (bounds.removeFromTop (bounds.getHeight() / 2).reduced (4));
            keyMappingEditor.setBounds (bounds.reduced (4));
        */
    }
}

impl<'a> LookAndFeelChanged for KeyMappingsDemo<'a> {

    fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto* lf = &LookAndFeel::getDefaultLookAndFeel();

            keyMappingEditor.setColours (lf->findColour (KeyMappingEditorComponent::backgroundColourId),
                                         lf->findColour (KeyMappingEditorComponent::textColourId));
        */
    }
}
