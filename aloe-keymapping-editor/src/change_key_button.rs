crate::ix!();

#[no_copy]
#[leak_detector]
pub struct KeyMappingEditorComponentChangeKeyButton<'a> {
    base:                     Button<'a>,
    owner:                    &'a mut KeyMappingEditorComponent<'a>,
    commandid:                CommandID,
    key_num:                  i32,
    current_key_entry_window: Box<KeyMappingEditorComponentChangeKeyButtonKeyEntryWindow<'a>>,
}

impl<'a> KeyMappingEditorComponentChangeKeyButton<'a> {

    pub fn new(
        kec:       &'a mut KeyMappingEditorComponent<'a>,
        command:   CommandID,
        key_name:  &String,
        key_index: i32) -> Self {
    
        todo!();
        /*


            : Button (keyName),
              owner (kec),
              commandID (command),
              keyNum (keyIndex)

            setWantsKeyboardFocus (false);
            setTriggeredOnMouseDown (keyNum >= 0);

            setTooltip (keyIndex < 0 ? TRANS("Adds a new key-mapping")
                                     : TRANS("Click to change this key-mapping"));
        */
    }
    
    pub fn paint_button(&mut self, 
        g:       &mut Graphics,
        is_over: bool,
        is_down: bool)  {
        
        todo!();
        /*
            getLookAndFeel().drawKeymapChangeButton (g, getWidth(), getHeight(), *this,
                                                     keyNum >= 0 ? getName() : String());
        */
    }
    
    pub fn clicked(&mut self)  {
        
        todo!();
        /*
            if (keyNum >= 0)
            {
                Component::SafePointer<KeyMappingEditorComponentChangeKeyButton> button (this);
                PopupMenu m;

                m.addItem (TRANS("Change this key-mapping"),
                           [button]
                           {
                               if (button != nullptr)
                                   button.getComponent()->assignNewKey();
                           });

                m.addSeparator();

                m.addItem (TRANS("Remove this key-mapping"),
                           [button]
                           {
                               if (button != nullptr)
                                   button->owner.getMappings().removeKeyPress (button->commandID,
                                                                               button->keyNum);
                           });

                m.showMenuAsync (typename PopupMenu::Options().withTargetComponent (this));
            }
            else
            {
                assignNewKey();  // + button pressed..
            }
        */
    }
    
    pub fn fit_to_content(&mut self, h: i32)  {
        
        todo!();
        /*
            if (keyNum < 0)
                setSize (h, h);
            else
                setSize (jlimit (h * 4, h * 8, 6 + Font ((float) h * 0.6f).getStringWidth (getName())), h);
        */
    }
    
    pub fn assign_new_key_callback(
        result:  i32,
        button:  *mut KeyMappingEditorComponentChangeKeyButton<'a>,
        new_key: KeyPress)  {
        
        todo!();
        /*
            if (result != 0 && button != nullptr)
                button->setNewKey (newKey, true);
        */
    }
    
    pub fn set_new_key(&mut self, 
        new_key:       &KeyPress,
        dont_ask_user: bool)  {
        
        todo!();
        /*
            if (newKey.isValid())
            {
                auto previousCommand = owner.getMappings().findCommandForKeyPress (newKey);

                if (previousCommand == 0 || dontAskUser)
                {
                    owner.getMappings().removeKeyPress (newKey);

                    if (keyNum >= 0)
                        owner.getMappings().removeKeyPress (commandID, keyNum);

                    owner.getMappings().addKeyPress (commandID, newKey, keyNum);
                }
                else
                {
                    AlertWindow::showOkCancelBox (MessageBoxIconType::WarningIcon,
                                                  TRANS("Change key-mapping"),
                                                  TRANS("This key is already assigned to the command \"CMDN\"")
                                                      .replace ("CMDN", owner.getCommandManager().getNameOfCommand (previousCommand))
                                                    + "\n\n"
                                                    + TRANS("Do you want to re-assign it to this new command instead?"),
                                                  TRANS("Re-assign"),
                                                  TRANS("Cancel"),
                                                  this,
                                                  ModalCallbackFunction::forComponent (assignNewKeyCallback,
                                                                                       this, KeyPress (newKey)));
                }
            }
        */
    }
    
    pub fn key_chosen(
        result: i32,
        button: *mut KeyMappingEditorComponentChangeKeyButton<'a>
    )  {
        
        todo!();
        /*
            if (button != nullptr && button->currentKeyEntryWindow != nullptr)
            {
                if (result != 0)
                {
                    button->currentKeyEntryWindow->setVisible (false);
                    button->setNewKey (button->currentKeyEntryWindow->lastPress, false);
                }

                button->currentKeyEntryWindow.reset();
            }
        */
    }
    
    pub fn assign_new_key(&mut self)  {
        
        todo!();
        /*
            currentKeyEntryWindow.reset (new KeyMappingEditorComponentChangeKeyButtonKeyEntryWindow (owner));
            currentKeyEntryWindow->enterModalState (true, ModalCallbackFunction::forComponent (keyChosen, this));
        */
    }
}
