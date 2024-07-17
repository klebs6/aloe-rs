crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandManager.h]

/**
  | One of these objects holds a list of all
  | the commands your app can perform, and
  | despatches these commands when needed.
  | 
  | Application commands are a good way
  | to trigger actions in your app, e.g.
  | "Quit", "Copy", "Paste", etc. Menus,
  | buttons and keypresses can all be given
  | commands to invoke automatically,
  | which means you don't have to handle
  | the result of a menu or button click manually.
  | Commands are despatched to ApplicationCommandTarget
  | objects which can choose which events
  | they want to handle.
  | 
  | This architecture also allows for nested
  | ApplicationCommandTargets, so that
  | for example you could have two different
  | objects, one inside the other, both
  | of which can respond to a "delete" command.
  | Depending on which one has focus, the
  | command will be sent to the appropriate
  | place, regardless of whether it was
  | triggered by a menu, keypress or some
  | other method.
  | 
  | To set up your app to use commands, you'll
  | need to do the following:
  | 
  | - Create a global ApplicationCommandManager
  | to hold the list of all possible commands.
  | (This will also manage a set of key-mappings
  | for them).
  | 
  | - Make some of your UI components (or
  | other objects) inherit from ApplicationCommandTarget.
  | This allows the object to provide a list
  | of commands that it can perform, and
  | to handle them.
  | 
  | - Register each type of command using
  | ApplicationCommandManager::registerAllCommandsForTarget(),
  | or ApplicationCommandManager::registerCommand().
  | 
  | - If you want key-presses to trigger
  | your commands, use the ApplicationCommandManager::getKeyMappings()
  | method to access the key-mapper object,
  | which you will need to register as a key-listener
  | in whatever top-level component you're
  | using. See the KeyPressMappingSet
  | class for more help about setting this
  | up.
  | 
  | - Use methods such as PopupMenu::addCommandItem()
  | or Button::setCommandToTrigger()
  | to cause these commands to be invoked
  | automatically.
  | 
  | - Commands can be invoked directly by
  | your code using ApplicationCommandManager::invokeDirectly().
  | 
  | When a command is invoked, the ApplicationCommandManager
  | will try to choose the best ApplicationCommandTarget
  | to receive the specified command. To
  | do this it will use the current keyboard
  | focus to see which component might be
  | interested, and will search the component
  | hierarchy for those that also implement
  | the ApplicationCommandTarget interface.
  | If an ApplicationCommandTarget isn't
  | interested in the command that is being
  | invoked, then the next one in line will
  | be tried (see the ApplicationCommandTarget::getNextCommandTarget()
  | method), and so on until ApplicationCommandTarget::getNextCommandTarget()
  | returns nullptr. At this point if the
  | command still hasn't been performed,
  | it will be passed to the current ALOEApplication
  | object (which is itself an ApplicationCommandTarget).
  | 
  | To exert some custom control over which
  | ApplicationCommandTarget is chosen
  | to invoke a command, you can override
  | the ApplicationCommandManager::getFirstCommandTarget()
  | method and choose the object yourself.
  | 
  | @see ApplicationCommandTarget, ApplicationCommandInfo
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ApplicationCommandManager<'a> {
    base:         AsyncUpdater<'a>,
    commands:     Vec<ApplicationCommandInfo>,
    listeners:    ListenerList<Rc<RefCell<dyn ApplicationCommandManagerListener>>>,
    key_mappings: Box<KeyPressMappingSet<'a>>,
    first_target: *mut ApplicationCommandTarget, // default = nullptr
}

impl<'a> FocusChangeListener for ApplicationCommandManager<'a> {

}

impl<'a> GlobalFocusChanged for ApplicationCommandManager<'a> {

    fn global_focus_changed(&mut self, _0: *mut Component)  {
        
        todo!();
        /*
            commandStatusChanged();
        */
    }
}

impl<'a> Default for ApplicationCommandManager<'a> {
    
    /**
      | Creates an ApplicationCommandManager.
      | 
      | Once created, you'll need to register
      | all your app's commands with it, using
      | ApplicationCommandManager::registerAllCommandsForTarget()
      | or ApplicationCommandManager::registerCommand().
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        keyMappings.reset (new KeyPressMappingSet (*this));
        Desktop::getInstance().addFocusChangeListener (this);
        */
    }
}

impl<'a> Drop for ApplicationCommandManager<'a> {

    /**
      | Destructor.
      | 
      | Make sure that you don't delete this
      | if pointers to it are still being used
      | by objects such as PopupMenus or Buttons.
      |
      */
    fn drop(&mut self) {

        todo!();

        /* 
        Desktop::getInstance().removeFocusChangeListener (this);
        keyMappings.reset();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandManager.cpp]
impl<'a> ApplicationCommandManager<'a> {
    
    /**
      | Returns the number of commands that
      | have been registered. @see registerCommand
      |
      */
    pub fn get_num_commands(&self) -> i32 {
        
        todo!();
        /*
            return commands.size();
        */
    }

    /**
      | Returns the details about one of the
      | registered commands. The index is between
      | 0 and (getNumCommands() - 1).
      |
      */
    pub fn get_command_for_index(&self, index: i32) -> *const ApplicationCommandInfo {
        
        todo!();
        /*
            return commands [index];
        */
    }

    /**
      | Returns the manager's internal set
      | of key mappings.
      | 
      | This object can be used to edit the keypresses.
      | To actually link this object up to invoke
      | commands when a key is pressed, see the
      | comments for the KeyPressMappingSet
      | class.
      | 
      | @see KeyPressMappingSet
      |
      */
    pub fn get_key_mappings(&self) -> *mut KeyPressMappingSet {
        
        todo!();
        /*
            return keyMappings.get();
        */
    }

    /**
      | Clears the current list of all commands.
      | 
      | -----------
      | @note
      | 
      | this will also clear the contents of
      | the KeyPressMappingSet.
      |
      */
    pub fn clear_commands(&mut self)  {
        
        todo!();
        /*
            commands.clear();
        keyMappings->clearAllKeyPresses();
        triggerAsyncUpdate();
        */
    }
    
    /**
      | Adds a command to the list of registered
      | commands. @see registerAllCommandsForTarget
      |
      */
    pub fn register_command(&mut self, new_command: &ApplicationCommandInfo)  {
        
        todo!();
        /*
            // zero isn't a valid command ID!
        jassert (newCommand.commandID != 0);

        // the name isn't optional!
        jassert (newCommand.shortName.isNotEmpty());

        if (auto* command = getMutableCommandForID (newCommand.commandID))
        {
            // Trying to re-register the same command ID with different parameters can often indicate a typo.
            // This assertion is here because I've found it useful catching some mistakes, but it may also cause
            // false alarms if you're deliberately updating some flags for a command.
            jassert (newCommand.shortName == getCommandForID (newCommand.commandID)->shortName
                      && newCommand.categoryName == getCommandForID (newCommand.commandID)->categoryName
                      && newCommand.defaultKeypresses == getCommandForID (newCommand.commandID)->defaultKeypresses
                      && (newCommand.flags & (ApplicationCommandInfo::wantsKeyUpDownCallbacks | ApplicationCommandInfo::hiddenFromKeyEditor | ApplicationCommandInfo::readOnlyInKeyEditor))
                           == (getCommandForID (newCommand.commandID)->flags & (ApplicationCommandInfo::wantsKeyUpDownCallbacks | ApplicationCommandInfo::hiddenFromKeyEditor | ApplicationCommandInfo::readOnlyInKeyEditor)));

            *command = newCommand;
        }
        else
        {
            auto* newInfo = new ApplicationCommandInfo (newCommand);
            newInfo->flags &= ~ApplicationCommandInfo::isTicked;
            commands.add (newInfo);

            keyMappings->resetToDefaultMapping (newCommand.commandID);

            triggerAsyncUpdate();
        }
        */
    }
    
    /**
      | Adds all the commands that this target
      | publishes to the manager's list.
      | 
      | This will use ApplicationCommandTarget::getAllCommands()
      | and ApplicationCommandTarget::getCommandInfo()
      | to get details about all the commands
      | that this target can do, and will call
      | registerCommand() to add each one to
      | the manger's list.
      | 
      | @see registerCommand
      |
      */
    pub fn register_all_commands_for_target(&mut self, target: *mut ApplicationCommandTarget)  {
        
        todo!();
        /*
            if (target != nullptr)
        {
            Vec<CommandID> commandIDs;
            target->getAllCommands (commandIDs);

            for (int i = 0; i < commandIDs.size(); ++i)
            {
                ApplicationCommandInfo info (commandIDs.getUnchecked(i));
                target->getCommandInfo (info.commandID, info);

                registerCommand (info);
            }
        }
        */
    }
    
    /**
      | Removes the command with a specified
      | ID.
      | 
      | -----------
      | @note
      | 
      | this will also remove any key mappings
      | that are mapped to the command.
      |
      */
    pub fn remove_command(&mut self, commandid: CommandID)  {
        
        todo!();
        /*
            for (int i = commands.size(); --i >= 0;)
        {
            if (commands.getUnchecked (i)->commandID == commandID)
            {
                commands.remove (i);
                triggerAsyncUpdate();

                const Vec<KeyPress> keys (keyMappings->getKeyPressesAssignedToCommand (commandID));

                for (int j = keys.size(); --j >= 0;)
                    keyMappings->removeKeyPress (keys.getReference (j));
            }
        }
        */
    }
    
    /**
      | This should be called to tell the manager
      | that one of its registered commands
      | may have changed its active status.
      | 
      | Because the command manager only finds
      | out whether a command is active or inactive
      | by querying the current ApplicationCommandTarget,
      | this is used to tell it that things may
      | have changed. It allows things like
      | buttons to update their enablement,
      | etc.
      | 
      | This method will cause an asynchronous
      | call to ApplicationCommandManagerListener::applicationCommandListChanged()
      | for any registered listeners.
      |
      */
    pub fn command_status_changed(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn get_mutable_command_forid(&self, commandid: CommandID) -> *mut ApplicationCommandInfo {
        
        todo!();
        /*
            for (int i = commands.size(); --i >= 0;)
            if (commands.getUnchecked(i)->commandID == commandID)
                return commands.getUnchecked(i);

        return nullptr;
        */
    }
    
    /**
      | Returns the details about a given command
      | ID.
      | 
      | This will search the list of registered
      | commands for one with the given command
      | ID number, and return its associated
      | info. If no matching command is found,
      | this will return nullptr.
      |
      */
    pub fn get_command_forid(&self, commandid: CommandID) -> *const ApplicationCommandInfo {
        
        todo!();
        /*
            return getMutableCommandForID (commandID);
        */
    }
    
    /**
      | Returns the name field for a command.
      | 
      | An empty string is returned if no command
      | with this ID has been registered. @see
      | getDescriptionOfCommand
      |
      */
    pub fn get_name_of_command(&self, commandid: CommandID) -> String {
        
        todo!();
        /*
            if (auto* ci = getCommandForID (commandID))
            return ci->shortName;

        return {};
        */
    }
    
    /**
      | Returns the description field for a
      | command.
      | 
      | An empty string is returned if no command
      | with this ID has been registered. If
      | the command has no description, this
      | will return its short name field instead.
      | 
      | @see getNameOfCommand
      |
      */
    pub fn get_description_of_command(&self, commandid: CommandID) -> String {
        
        todo!();
        /*
            if (auto* ci = getCommandForID (commandID))
            return ci->description.isNotEmpty() ? ci->description
                                                : ci->shortName;

        return {};
        */
    }
    
    /**
      | Returns the list of categories.
      | 
      | This will go through all registered
      | commands, and return a list of all the
      | distinct categoryName values from
      | their ApplicationCommandInfo structure.
      | 
      | @see getCommandsInCategory()
      |
      */
    pub fn get_command_categories(&self) 
        -> Vec<String> 
    {
        todo!();
        /*
            StringArray s;

        for (int i = 0; i < commands.size(); ++i)
            s.addIfNotAlreadyThere (commands.getUnchecked(i)->categoryName, false);

        return s;
        */
    }
    
    /**
      | Returns a list of all the command UIDs
      | in a particular category. @see getCommandCategories()
      |
      */
    pub fn get_commands_in_category(&self, category_name: &String) 
        -> Vec<CommandID> 
    {
        todo!();
        /*
            Vec<CommandID> results;

        for (int i = 0; i < commands.size(); ++i)
            if (commands.getUnchecked(i)->categoryName == categoryName)
                results.add (commands.getUnchecked(i)->commandID);

        return results;
        */
    }
    
    /**
      | Invokes the given command directly,
      | sending it to the default target. This
      | is just an easy way to call invoke() without
      | having to fill out the InvocationInfo
      | structure.
      |
      */
    pub fn invoke_directly(&mut self, 
        commandid:      CommandID,
        asynchronously: bool) -> bool {
        
        todo!();
        /*
            ApplicationCommandTarget::InvocationInfo info (commandID);
        info.invocationMethod = ApplicationCommandTarget::InvocationInfo::direct;

        return invoke (info, asynchronously);
        */
    }
    
    /**
      | Sends a command to the default target.
      | 
      | This will choose a target using getFirstCommandTarget(),
      | and send the specified command to it
      | using the ApplicationCommandTarget::invoke()
      | method. This means that if the first
      | target can't handle the command, it
      | will be passed on to targets further
      | down the chain (see ApplicationCommandTarget::invoke()
      | for more info).
      | 
      | -----------
      | @param invocationInfo
      | 
      | this must be correctly filled-in, describing
      | the context for the invocation.
      | ----------
      | @param asynchronously
      | 
      | if false, the command will be performed
      | before this method returns. If true,
      | a message will be posted so that the command
      | will be performed later on the message
      | thread, and this method will return
      | immediately.
      | 
      | @see ApplicationCommandTarget::invoke
      |
      */
    pub fn invoke(
        &mut self, 
        inf:            &ApplicationCommandTargetInvocationInfo,
        asynchronously: bool

    ) -> bool {
        
        todo!();
        /*
            // This call isn't thread-safe for use from a non-UI thread without locking the message
        // manager first..
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        bool ok = false;
        ApplicationCommandInfo commandInfo (0);

        if (auto* target = getTargetForCommand (inf.commandID, commandInfo))
        {
            ApplicationCommandTarget::InvocationInfo info (inf);
            info.commandFlags = commandInfo.flags;

            sendListenerInvokeCallback (info);
            ok = target->invoke (info, asynchronously);
            commandStatusChanged();
        }

        return ok;
        */
    }
    
    pub fn get_first_command_target(&mut self, _0: CommandID) 
        -> *mut ApplicationCommandTarget 
    {
        todo!();

        /*
            return firstTarget != nullptr ? firstTarget
                                      : findDefaultComponentTarget();
        */
    }
    
    /**
      | Sets a target to be returned by getFirstCommandTarget().
      | 
      | If this is set to nullptr, then getFirstCommandTarget()
      | will by default return the result of
      | findDefaultComponentTarget().
      | 
      | If you use this to set a target, make sure
      | you call setFirstCommandTarget(nullptr)
      | before deleting the target object.
      |
      */
    pub fn set_first_command_target(
        &mut self, 
        new_target: *mut ApplicationCommandTarget

    ) {
        
        todo!();
        /*
            firstTarget = newTarget;
        */
    }
    
    /**
      | Tries to find the best target to use to
      | perform a given command.
      | 
      | This will call getFirstCommandTarget()
      | to find the preferred target, and will
      | check whether that target can handle
      | the given command. If it can't, then
      | it'll use ApplicationCommandTarget::getNextCommandTarget()
      | to find the next one to try, and so on until
      | no more are available.
      | 
      | If no targets are found that can perform
      | the command, this method will return
      | nullptr.
      | 
      | If a target is found, then it will get
      | the target to fill-in the upToDateInfo
      | structure with the latest info about
      | that command, so that the caller can
      | see whether the command is disabled,
      | ticked, etc.
      |
      */
    pub fn get_target_for_command(&mut self, 
        commandid:       CommandID,
        up_to_date_info: &mut ApplicationCommandInfo) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            auto* target = getFirstCommandTarget (commandID);

        if (target == nullptr)
            target = ALOEApplication::getInstance();

        if (target != nullptr)
            target = target->getTargetForCommand (commandID);

        if (target != nullptr)
        {
            upToDateInfo.commandID = commandID;
            target->getCommandInfo (commandID, upToDateInfo);
        }

        return target;
        */
    }
    
    /**
      | Examines this component and all its
      | parents in turn, looking for the first
      | one which is an ApplicationCommandTarget.
      | 
      | Returns the first ApplicationCommandTarget
      | that it finds, or nullptr if none of them
      | implement that class.
      |
      */
    pub fn find_target_for_component(&mut self, c: *mut Component) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            auto* target = dynamic_cast<ApplicationCommandTarget*> (c);

        if (target == nullptr && c != nullptr)
            target = c->findParentComponentOfClass<ApplicationCommandTarget>();

        return target;
        */
    }
    
    /**
      | Looks for a suitable command target
      | based on which Components have the keyboard
      | focus.
      | 
      | This is used by the default implementation
      | of ApplicationCommandTarget::getFirstCommandTarget(),
      | but is exposed here in case it's useful.
      | 
      | It tries to pick the best ApplicationCommandTarget
      | by looking at focused components, top
      | level windows, etc., and using the findTargetForComponent()
      | method.
      |
      */
    pub fn find_default_component_target(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            auto* c = Component::getCurrentlyFocusedComponent();

        if (c == nullptr)
        {
            if (auto* activeWindow = TopLevelWindow::getActiveTopLevelWindow())
            {
                if (auto* peer = activeWindow->getPeer())
                {
                    c = peer->getLastFocusedSubcomponent();

                    if (c == nullptr)
                        c = activeWindow;
                }
            }
        }

        if (c == nullptr)
        {
            auto& desktop = Desktop::getInstance();

            // getting a bit desperate now: try all desktop comps..
            for (int i = desktop.getNumComponents(); --i >= 0;)
                if (auto* component = desktop.getComponent (i))
                    if (isForegroundOrEmbeddedProcess (component))
                        if (auto* peer = component->getPeer())
                            if (auto* target = findTargetForComponent (peer->getLastFocusedSubcomponent()))
                                return target;
        }

        if (c != nullptr)
        {
            // if we're focused on a ResizableWindow, chances are that it's the content
            // component that really should get the event. And if not, the event will
            // still be passed up to the top level window anyway, so let's send it to the
            // content comp.
            if (auto* resizableWindow = dynamic_cast<ResizableWindow*> (c))
                if (auto* content = resizableWindow->getContentComponent())
                    c = content;

            if (auto* target = findTargetForComponent (c))
                return target;
        }

        return ALOEApplication::getInstance();
        */
    }
    
    /**
      | Registers a listener that will be called
      | when various events occur.
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn ApplicationCommandManagerListener)  {
        
        todo!();
        /*
            listeners.add (listener);
        */
    }
    
    /**
      | Deregisters a previously-added listener.
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn ApplicationCommandManagerListener)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */
    }
    
    pub fn send_listener_invoke_callback(
        &mut self, 
        info: &ApplicationCommandTargetInvocationInfo

    ) {
        
        todo!();
        /*
            listeners.call ([&] (ApplicationCommandManagerListener& l) { l.applicationCommandInvoked (info); });
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            listeners.call ([] (ApplicationCommandManagerListener& l) { l.applicationCommandListChanged(); });
        */
    }
    
}
