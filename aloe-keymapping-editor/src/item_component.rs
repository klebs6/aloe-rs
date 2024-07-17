crate::ix!();

#[no_copy]
#[leak_detector]
pub struct KeyMappingEditorComponentItemComponent<'a> {
    base:               Component<'a>,
    owner:              &'a mut KeyMappingEditorComponent<'a>,
    key_change_buttons: Vec<Box<KeyMappingEditorComponentChangeKeyButton<'a>>>,
    commandid:          CommandID,
}

pub const ITEM_COMPONENT_MAX_NUM_ASSIGNMENTS: usize = 3;

impl<'a> KeyMappingEditorComponentItemComponent<'a> {

    pub fn new(
        kec:     &mut KeyMappingEditorComponent,
        command: CommandID) -> Self {
    
        todo!();
        /*
        : owner(kec),
        : commandid(command),

            setInterceptsMouseClicks (false, true);

            const bool isReadOnly = owner.isCommandReadOnly (commandID);

            auto keyPresses = owner.getMappings().getKeyPressesAssignedToCommand (commandID);

            for (int i = 0; i < jmin ((int) maxNumAssignments, keyPresses.size()); ++i)
                addKeyPressButton (owner.getDescriptionForKeyPress (keyPresses.getReference (i)), i, isReadOnly);

            addKeyPressButton ("Change Key Mapping", -1, isReadOnly);
        */
    }
    
    pub fn add_key_press_button(&mut self, 
        desc:         &String,
        index:        i32,
        is_read_only: bool)  {
        
        todo!();
        /*
            auto* b = new KeyMappingEditorComponentChangeKeyButton (owner, commandID, desc, index);
            keyChangeButtons.add (b);

            b->setEnabled (! isReadOnly);
            b->setVisible (keyChangeButtons.size() <= (int) maxNumAssignments);
            addChildComponent (b);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setFont ((float) getHeight() * 0.7f);
            g.setColour (owner.findColour (KeyMappingEditorComponent::textColourId));

            g.drawFittedText (TRANS (owner.getCommandManager().getNameOfCommand (commandID)),
                              4, 0, jmax (40, getChildComponent (0)->getX() - 5), getHeight(),
                              Justification::centredLeft, true);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            int x = getWidth() - 4;

            for (int i = keyChangeButtons.size(); --i >= 0;)
            {
                auto* b = keyChangeButtons.getUnchecked(i);

                b->fitToContent (getHeight() - 2);
                b->setTopRightPosition (x, 1);
                x = b->getX() - 5;
            }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
}
