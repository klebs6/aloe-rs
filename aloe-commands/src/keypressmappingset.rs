crate::ix!();

pub fn add_key_presses(
        set: &mut KeyPressMappingSet,
        ci:  *const ApplicationCommandInfo)  {
    
    todo!();
    /*
        for (int j = 0; j < ci->defaultKeypresses.size(); ++j)
            set.addKeyPress (ci->commandID, ci->defaultKeypresses.getReference (j));
    */
}

pub struct KeyPressMappingSetCommandMapping
{
    commandid:                   CommandID,
    keypresses:                  Vec<KeyPress>,
    wants_key_up_down_callbacks: bool,
}

pub struct KeyPressMappingSetKeyPressTime
{
    key:                         KeyPress,
    time_when_pressed:           u32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_KeyPressMappingSet.h]

/**
  | Manages and edits a list of keypresses,
  | which it uses to invoke the appropriate
  | command in an ApplicationCommandManager.
  | 
  | Normally, you won't actually create
  | a KeyPressMappingSet directly, because
  | each ApplicationCommandManager contains
  | its own KeyPressMappingSet, so typically
  | you'd create yourself an ApplicationCommandManager,
  | and call its ApplicationCommandManager::getKeyMappings()
  | method to get a pointer to its KeyPressMappingSet.
  | 
  | For one of these to actually use keypresses,
  | you'll need to add it as a KeyListener
  | to the top-level component for which
  | you want to handle keystrokes. So for
  | example:
  | 
  | -----------
  | @code
  | 
  | class MyMainWindow  : public Component
  | {
  |     ApplicationCommandManager* myCommandManager;
  | 
  |     MyMainWindow()
  |     {
  |         myCommandManager = new ApplicationCommandManager();
  | 
  |         // first, make sure the command manager has registered all the commands that its
  |         // targets can perform..
  |         myCommandManager->registerAllCommandsForTarget (myCommandTarget1);
  |         myCommandManager->registerAllCommandsForTarget (myCommandTarget2);
  | 
  |         // this will use the command manager to initialise the KeyPressMappingSet with
  |         // the default keypresses that were specified when the targets added their commands
  |         // to the manager.
  |         myCommandManager->getKeyMappings()->resetToDefaultMappings();
  | 
  |         // having set up the default key-mappings, you might now want to load the last set
  |         // of mappings that the user configured.
  |         myCommandManager->getKeyMappings()->restoreFromXml (lastSavedKeyMappingsXML);
  | 
  |         // Now tell our top-level window to send any keypresses that arrive to the
  |         // KeyPressMappingSet, which will use them to invoke the appropriate commands.
  |         addKeyListener (myCommandManager->getKeyMappings());
  |     }
  | 
  |     ...
  | }
  | 
  | KeyPressMappingSet derives from ChangeBroadcaster
  | so that interested parties can register
  | to be told when a command or mapping is
  | added, removed, etc.
  | 
  | There's also a UI component called KeyMappingEditorComponent
  | that can be used to easily edit the key
  | mappings.
  | 
  | @see Component::addKeyListener(),
  | KeyMappingEditorComponent, ApplicationCommandManager
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct KeyPressMappingSet<'a> {
    base2:           ChangeBroadcaster<'a>,
    command_manager: &'a mut ApplicationCommandManager<'a>,
    mappings:        Vec<Box<KeyPressMappingSetCommandMapping>>,
    keys_down:       Vec<Box<KeyPressMappingSetKeyPressTime>>,
}

impl<'a> KeyListener for KeyPressMappingSet<'a> { 

    fn key_pressed(
        &mut self, 
        key:                   &KeyPress,
        originating_component: *mut Component

    ) -> bool {
        
        todo!();
        /*
            bool commandWasDisabled = false;

        for (int i = 0; i < mappings.size(); ++i)
        {
            KeyPressMappingSetCommandMapping& cm = *mappings.getUnchecked(i);

            if (cm.keypresses.contains (key))
            {
                if (const ApplicationCommandInfo* const ci = commandManager.getCommandForID (cm.commandID))
                {
                    if ((ci->flags & ApplicationCommandInfo::wantsKeyUpDownCallbacks) == 0)
                    {
                        ApplicationCommandInfo info (0);

                        if (commandManager.getTargetForCommand (cm.commandID, info) != nullptr)
                        {
                            if ((info.flags & ApplicationCommandInfo::isDisabled) == 0)
                            {
                                invokeCommand (cm.commandID, key, true, 0, originatingComponent);
                                return true;
                            }

                            commandWasDisabled = true;
                        }
                    }
                }
            }
        }

        if (originatingComponent != nullptr && commandWasDisabled)
            originatingComponent->getLookAndFeel().playAlertSound();

        return false;
        */
    }
}

impl<'a> FocusChangeListener for KeyPressMappingSet<'a> { 

}

impl<'a> GlobalFocusChanged for KeyPressMappingSet<'a> { 

    fn global_focus_changed(&mut self, focused_component: *mut Component)  {
        
        todo!();
        /*
            if (focusedComponent != nullptr)
            focusedComponent->keyStateChanged (false);
        */
    }
}

impl<'a> KeyPressMappingSetInterface for KeyPressMappingSet<'a> { }

impl<'a> Drop for KeyPressMappingSet<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        Desktop::getInstance().removeFocusChangeListener (this);
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_KeyPressMappingSet.cpp]
impl<'a> KeyPressMappingSet<'a> {

    pub fn get_command_manager(&self) -> &mut ApplicationCommandManager {
        
        todo!();
        /*
            return commandManager;
        */
    }

    /**
      | Creates a KeyPressMappingSet for a
      | given command manager.
      | 
      | Normally, you won't actually create
      | a KeyPressMappingSet directly, because
      | each ApplicationCommandManager contains
      | its own KeyPressMappingSet, so the
      | best thing to do is to create your ApplicationCommandManager,
      | and use the ApplicationCommandManager::getKeyMappings()
      | method to access its mappings.
      | 
      | When a suitable keypress happens, the
      | manager's invoke() method will be used
      | to invoke the appropriate command.
      | 
      | @see ApplicationCommandManager
      |
      */
    pub fn new_from_application_commandman(cm: &mut ApplicationCommandManager) -> Self {
    
        todo!();
        /*
        : command_manager(cm),

            Desktop::getInstance().addFocusChangeListener (this);
        */
    }
    
    /**
      | Creates an copy of a KeyPressMappingSet.
      |
      */
    pub fn new_from_keypress_mapping_set(other: &KeyPressMappingSet) -> Self {
    
        todo!();
        /*
        : key_listener(),
        : change_broadcaster(),
        : focus_change_listener(),
        : command_manager(other.commandManager),

            Desktop::getInstance().addFocusChangeListener (this);
        */
    }
    
    /**
      | Returns a list of keypresses that are
      | assigned to a particular command.
      | 
      | -----------
      | @param commandID
      | 
      | the command's ID
      |
      */
    pub fn get_key_presses_assigned_to_command(&self, commandid: CommandID) -> Vec<KeyPress> {
        
        todo!();
        /*
            for (int i = 0; i < mappings.size(); ++i)
            if (mappings.getUnchecked(i)->commandID == commandID)
                return mappings.getUnchecked (i)->keypresses;

        return {};
        */
    }
    
    /**
      | Assigns a keypress to a command.
      | 
      | If the keypress is already assigned
      | to a different command, it will first
      | be removed from that command, to avoid
      | it triggering multiple functions.
      | 
      | -----------
      | @param commandID
      | 
      | the ID of the command that you want to
      | add a keypress to. If this is 0, the keypress
      | will be removed from anything that it
      | was previously assigned to, but not
      | re-assigned
      | ----------
      | @param newKeyPress
      | 
      | the new key-press
      | ----------
      | @param insertIndex
      | 
      | if this is less than zero, the key will
      | be appended to the end of the list of keypresses;
      | otherwise the new keypress will be inserted
      | into the existing list at this index
      |
      */
    pub fn add_key_press(
        &mut self, 
        commandid:     CommandID,
        new_key_press: &KeyPress,
        insert_index:  Option<i32>

    ) {

        let insert_index: i32 = insert_index.unwrap_or(-1);
        
        todo!();
        /*
            // If you specify an upper-case letter but no shift key, how is the user supposed to press it!?
        // Stick to lower-case letters when defining a keypress, to avoid ambiguity.
        jassert (! (CharacterFunctions::isUpperCase (newKeyPress.getTextCharacter())
                     && ! newKeyPress.getModifiers().isShiftDown()));

        if (findCommandForKeyPress (newKeyPress) != commandID)
        {
            if (newKeyPress.isValid())
            {
                for (int i = mappings.size(); --i >= 0;)
                {
                    if (mappings.getUnchecked(i)->commandID == commandID)
                    {
                        mappings.getUnchecked(i)->keypresses.insert (insertIndex, newKeyPress);

                        sendChangeMessage();
                        return;
                    }
                }

                if (const ApplicationCommandInfo* const ci = commandManager.getCommandForID (commandID))
                {
                    KeyPressMappingSetCommandMapping* const cm = new KeyPressMappingSetCommandMapping();
                    cm->commandID = commandID;
                    cm->keypresses.add (newKeyPress);
                    cm->wantsKeyUpDownCallbacks = (ci->flags & ApplicationCommandInfo::wantsKeyUpDownCallbacks) != 0;

                    mappings.add (cm);
                    sendChangeMessage();
                }
                else
                {
                    // If you hit this, you're trying to attach a keypress to a command ID that
                    // doesn't exist, so the key is not being attached.
                    jassertfalse;
                }
            }
        }
        */
    }
    
    /**
      | Reset all mappings to the defaults,
      | as dictated by the ApplicationCommandManager.
      | @see resetToDefaultMapping
      |
      */
    pub fn reset_to_default_mappings(&mut self)  {
        
        todo!();
        /*
            mappings.clear();

        for (int i = 0; i < commandManager.getNumCommands(); ++i)
            addKeyPresses (*this, commandManager.getCommandForIndex (i));

        sendChangeMessage();
        */
    }
    
    /**
      | Resets all key-mappings to the defaults
      | for a particular command. @see resetToDefaultMappings
      |
      */
    pub fn reset_to_default_mapping(&mut self, commandid: CommandID)  {
        
        todo!();
        /*
            clearAllKeyPresses (commandID);

        if (const ApplicationCommandInfo* const ci = commandManager.getCommandForID (commandID))
            addKeyPresses (*this, ci);
        */
    }
    
    /**
      | Removes all keypresses that are assigned
      | to any commands.
      |
      */
    pub fn clear_all_key_presses(&mut self)  {
        
        todo!();
        /*
            if (mappings.size() > 0)
        {
            sendChangeMessage();
            mappings.clear();
        }
        */
    }
    
    /**
      | Removes all keypresses that are assigned
      | to a particular command.
      |
      */
    pub fn clear_all_key_presses_with_command_id(&mut self, commandid: CommandID)  {
        
        todo!();
        /*
            for (int i = mappings.size(); --i >= 0;)
        {
            if (mappings.getUnchecked(i)->commandID == commandID)
            {
                mappings.remove (i);
                sendChangeMessage();
            }
        }
        */
    }
    
    /**
      | Removes a keypress from any command
      | that it may be assigned to.
      |
      */
    pub fn remove_key_press(&mut self, keypress: &KeyPress)  {
        
        todo!();
        /*
            if (keypress.isValid())
        {
            for (int i = mappings.size(); --i >= 0;)
            {
                KeyPressMappingSetCommandMapping& cm = *mappings.getUnchecked(i);

                for (int j = cm.keypresses.size(); --j >= 0;)
                {
                    if (keypress == cm.keypresses [j])
                    {
                        cm.keypresses.remove (j);
                        sendChangeMessage();
                    }
                }
            }
        }
        */
    }
    
    /**
      | Removes one of the keypresses that are
      | assigned to a command. See the getKeyPressesAssignedToCommand()
      | for the list of keypresses to which the
      | keyPressIndex refers.
      |
      */
    pub fn remove_key_press_with_command_id(
        &mut self, 
        commandid:       CommandID,
        key_press_index: i32

    ) {
        
        todo!();
        /*
            for (int i = mappings.size(); --i >= 0;)
        {
            if (mappings.getUnchecked(i)->commandID == commandID)
            {
                mappings.getUnchecked(i)->keypresses.remove (keyPressIndex);
                sendChangeMessage();
                break;
            }
        }
        */
    }
    
    /**
      | Looks for a command that corresponds
      | to a keypress.
      | 
      | 
      | -----------
      | @return
      | 
      | the UID of the command or 0 if none was
      | found
      |
      */
    pub fn find_command_for_key_press(&self, key_press: &KeyPress) -> CommandID {
        
        todo!();
        /*
            for (int i = 0; i < mappings.size(); ++i)
            if (mappings.getUnchecked(i)->keypresses.contains (keyPress))
                return mappings.getUnchecked(i)->commandID;

        return 0;
        */
    }
    
    /**
      | Returns true if the given command is
      | linked to this key.
      |
      */
    pub fn contains_mapping(&self, 
        commandid: CommandID,
        key_press: &KeyPress) -> bool {
        
        todo!();
        /*
            for (int i = mappings.size(); --i >= 0;)
            if (mappings.getUnchecked(i)->commandID == commandID)
                return mappings.getUnchecked(i)->keypresses.contains (keyPress);

        return false;
        */
    }
    
    pub fn invoke_command(&self, 
        commandid:                   CommandID,
        key:                         &KeyPress,
        is_key_down:                 bool,
        millisecs_since_key_pressed: i32,
        originating_component:       *mut Component)  {
        
        todo!();
        /*
            ApplicationCommandTarget::InvocationInfo info (commandID);

        info.invocationMethod = ApplicationCommandTarget::InvocationInfo::fromKeyPress;
        info.isKeyDown = isKeyDown;
        info.keyPress = key;
        info.millisecsSinceKeyPressed = millisecsSinceKeyPressed;
        info.originatingComponent = originatingComponent;

        commandManager.invoke (info, false);
        */
    }
    
    /**
      | Tries to recreate the mappings from
      | a previously stored state.
      | 
      | The XML passed in must have been created
      | by the createXml() method.
      | 
      | If the stored state makes any reference
      | to commands that aren't currently available,
      | these will be ignored.
      | 
      | If the set of mappings being loaded was
      | a set of differences (using createXml
      | (true)), then this will call resetToDefaultMappings()
      | and then merge the saved mappings on
      | top. If the saved set was created with
      | createXml (false), then this method
      | will first clear all existing mappings
      | and load the saved ones as a complete
      | set.
      | 
      | 
      | -----------
      | @return
      | 
      | true if it manages to load the XML correctly
      | @see createXml
      |
      */
    pub fn restore_from_xml(&mut self, xml_version: &XmlElement) -> bool {
        
        todo!();
        /*
            if (xmlVersion.hasTagName ("KEYMAPPINGS"))
        {
            if (xmlVersion.getBoolAttribute ("basedOnDefaults", true))
            {
                // if the XML was created as a set of differences from the default mappings,
                // (i.e. by calling createXml (true)), then we need to first restore the defaults.
                resetToDefaultMappings();
            }
            else
            {
                // if the XML was created calling createXml (false), then we need to clear all
                // the keys and treat the xml as describing the entire set of mappings.
                clearAllKeyPresses();
            }

            for (auto* map : xmlVersion.getChildIterator())
            {
                const CommandID commandId = map->getStringAttribute ("commandId").getHexValue32();

                if (commandId != 0)
                {
                    auto key = KeyPress::createFromDescription (map->getStringAttribute ("key"));

                    if (map->hasTagName ("MAPPING"))
                    {
                        addKeyPress (commandId, key);
                    }
                    else if (map->hasTagName ("UNMAPPING"))
                    {
                        for (auto& m : mappings)
                            if (m->commandID == commandId)
                                m->keypresses.removeAllInstancesOf (key);
                    }
                }
            }

            return true;
        }

        return false;
        */
    }
    
    /**
      | Creates an XML representation of the
      | current mappings.
      | 
      | This will produce a lump of XML that can
      | be later reloaded using restoreFromXml()
      | to recreate the current mapping state.
      | 
      | -----------
      | @param saveDifferencesFromDefaultSet
      | 
      | if this is false, then all keypresses
      | will be saved into the XML. If it's true,
      | then the XML will only store the differences
      | between the current mappings and the
      | default mappings you'd get from calling
      | resetToDefaultMappings(). The advantage
      | of saving a set of differences from the
      | default is that if you change the default
      | mappings (in a new version of your app,
      | for example), then these will be merged
      | into a user's saved preferences.
      | 
      | @see restoreFromXml
      |
      */
    pub fn create_xml(&self, save_differences_from_default_set: bool) -> Box<XmlElement> {
        
        todo!();
        /*
            std::unique_ptr<KeyPressMappingSet> defaultSet;

        if (saveDifferencesFromDefaultSet)
        {
            defaultSet = std::make_unique<KeyPressMappingSet> (commandManager);
            defaultSet->resetToDefaultMappings();
        }

        auto doc = std::make_unique<XmlElement> ("KEYMAPPINGS");

        doc->setAttribute ("basedOnDefaults", saveDifferencesFromDefaultSet);

        for (int i = 0; i < mappings.size(); ++i)
        {
            auto& cm = *mappings.getUnchecked(i);

            for (int j = 0; j < cm.keypresses.size(); ++j)
            {
                if (defaultSet == nullptr
                     || ! defaultSet->containsMapping (cm.commandID, cm.keypresses.getReference (j)))
                {
                    auto map = doc->createNewChildElement ("MAPPING");

                    map->setAttribute ("commandId", String::toHexString ((int) cm.commandID));
                    map->setAttribute ("description", commandManager.getDescriptionOfCommand (cm.commandID));
                    map->setAttribute ("key", cm.keypresses.getReference (j).getTextDescription());
                }
            }
        }

        if (defaultSet != nullptr)
        {
            for (int i = 0; i < defaultSet->mappings.size(); ++i)
            {
                auto& cm = *defaultSet->mappings.getUnchecked(i);

                for (int j = 0; j < cm.keypresses.size(); ++j)
                {
                    if (! containsMapping (cm.commandID, cm.keypresses.getReference (j)))
                    {
                        auto map = doc->createNewChildElement ("UNMAPPING");

                        map->setAttribute ("commandId", String::toHexString ((int) cm.commandID));
                        map->setAttribute ("description", commandManager.getDescriptionOfCommand (cm.commandID));
                        map->setAttribute ("key", cm.keypresses.getReference (j).getTextDescription());
                    }
                }
            }
        }

        return doc;
        */
    }
    
    
    pub fn key_state_changed(&mut self, 
        is_key_down:           bool,
        originating_component: *mut Component) -> bool {
        
        todo!();
        /*
            bool used = false;
        const uint32 now = Time::getMillisecondCounter();

        for (int i = mappings.size(); --i >= 0;)
        {
            KeyPressMappingSetCommandMapping& cm = *mappings.getUnchecked(i);

            if (cm.wantsKeyUpDownCallbacks)
            {
                for (int j = cm.keypresses.size(); --j >= 0;)
                {
                    const KeyPress key (cm.keypresses.getReference (j));
                    const bool isDown = key.isCurrentlyDown();

                    int keyPressEntryIndex = 0;
                    bool wasDown = false;

                    for (int k = keysDown.size(); --k >= 0;)
                    {
                        if (key == keysDown.getUnchecked(k)->key)
                        {
                            keyPressEntryIndex = k;
                            wasDown = true;
                            used = true;
                            break;
                        }
                    }

                    if (isDown != wasDown)
                    {
                        int millisecs = 0;

                        if (isDown)
                        {
                            KeyPressMappingSetKeyPressTime* const k = new KeyPressMappingSetKeyPressTime();
                            k->key = key;
                            k->timeWhenPressed = now;

                            keysDown.add (k);
                        }
                        else
                        {
                            const uint32 pressTime = keysDown.getUnchecked (keyPressEntryIndex)->timeWhenPressed;

                            if (now > pressTime)
                                millisecs = (int) (now - pressTime);

                            keysDown.remove (keyPressEntryIndex);
                        }

                        invokeCommand (cm.commandID, key, isDown, millisecs, originatingComponent);
                        used = true;
                    }
                }
            }
        }

        return used;
        */
    }
    
}
