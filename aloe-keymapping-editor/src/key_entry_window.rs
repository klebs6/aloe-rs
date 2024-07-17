crate::ix!();

#[no_copy]
pub struct KeyMappingEditorComponentChangeKeyButtonKeyEntryWindow<'a> {
    base:       AlertWindow<'a>,
    last_press: KeyPress,
    owner:      &'a mut KeyMappingEditorComponent<'a>,
}

impl<'a> KeyMappingEditorComponentChangeKeyButtonKeyEntryWindow<'a> {

    pub fn new(kec: &mut KeyMappingEditorComponent<'a>) -> Self {
    
        todo!();
        /*


            : AlertWindow (TRANS("New key-mapping"),
                               TRANS("Please press a key combination now..."),
                               MessageBoxIconType::NoIcon),
                  owner (kec)

                addButton (TRANS("OK"), 1);
                addButton (TRANS("Cancel"), 0);

                // (avoid return + escape keys getting processed by the buttons..)
                for (auto* child : getChildren())
                    child->setWantsKeyboardFocus (false);

                setWantsKeyboardFocus (true);
                grabKeyboardFocus();
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            lastPress = key;
                String message (TRANS("Key") + ": " + owner.getDescriptionForKeyPress (key));

                auto previousCommand = owner.getMappings().findCommandForKeyPress (key);

                if (previousCommand != 0)
                    message << "\n\n("
                            << TRANS("Currently assigned to \"CMDN\"")
                                .replace ("CMDN", TRANS (owner.getCommandManager().getNameOfCommand (previousCommand)))
                            << ')';

                setMessage (message);
                return true;
        */
    }
    
    pub fn key_state_changed(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
