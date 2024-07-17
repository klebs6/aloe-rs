crate::ix!();

/**
  | A type used to hold the unique ID for an
  | application command.
  | 
  | This is a numeric type, so it can be stored
  | as an integer.
  | 
  | @see ApplicationCommandInfo, ApplicationCommandManager,
  | ApplicationCommandTarget, KeyPressMappingSet
  |
  */
pub type CommandID = i32;

pub trait CommandManagerInterface
: RemoveCommandManagerListener
+ GetKeyMappings
+ CommandManagerInvoke
+ AddCommandManagerListener
+ GetTargetForCommand
{ }

pub trait CommandManagerListenerInterface {}
pub trait KeyPressMappingSetInterface {}

pub trait RemoveCommandManagerListener {

    fn remove_listener(&mut self, listener: &mut dyn CommandManagerListenerInterface);
}

pub trait AddCommandManagerListener {

    fn add_listener(
        &mut self, 
        listener: &mut dyn CommandManagerListenerInterface
    );
}

pub trait CommandManagerInvoke {

    fn invoke(
        &mut self, 
        invocation_info: &ApplicationCommandTargetInvocationInfo<'_>,
        asynchronously:  bool) -> bool;
}

pub trait GetTargetForCommand {

    fn get_target_for_command(
        &mut self, 
        commandid:       CommandID,
        up_to_date_info: &mut ApplicationCommandInfo
    ) -> *mut ApplicationCommandTarget;
}

pub trait GetKeyMappings {

    fn get_key_mappings(&self) -> *mut dyn KeyPressMappingSetInterface;
}

/**
  | A listener that receives callbacks
  | from an ApplicationCommandManager
  | when commands are invoked or the command
  | list is changed.
  | 
  | @see ApplicationCommandManager::addListener,
  | ApplicationCommandManager::removeListener
  | 
  | @tags{GUI}
  |
  */
pub trait ApplicationCommandManagerListener {

    /**
      | Called when an app command is about to
      | be invoked.
      |
      */
    fn application_command_invoked(&mut self, _0: &ApplicationCommandTargetInvocationInfo);

    /**
      | Called when commands are registered
      | or deregistered from the command manager,
      | or when commands are made active or inactive.
      | 
      | -----------
      | @note
      | 
      | if you're using this to watch for changes
      | to whether a command is disabled, you'll
      | need to make sure that ApplicationCommandManager::commandStatusChanged()
      | is called whenever the status of your
      | command might have changed.
      |
      */
    fn application_command_list_changed(&mut self);
}

/**
  | Contains contextual details about
  | the invocation of a command.
  |
  */
pub struct ApplicationCommandTargetInvocationInfo<'a> {

    /**
      | The UID of the command that should be
      | performed.
      |
      */
    commandid:      CommandID,

    /**
      | The command's flags.
      | 
      | See ApplicationCommandInfo for a description
      | of these flag values.
      |
      */
    command_flags: i32,

    /**
      | The type of event that triggered this
      | command.
      |
      */
    invocation_method: InvocationMethod,

    /**
      | If triggered by a keypress or menu, this
      | will be the component that had the keyboard
      | focus at the time.
      | 
      | If triggered by a button, it may be set
      | to that component, or it may be null.
      |
      */
    originating_component: *mut Component<'a>,

    /**
      | The keypress that was used to invoke
      | it.
      | 
      | -----------
      | @note
      | 
      | this will be an invalid keypress if the
      | command was invoked by some other means
      | than a keyboard shortcut.
      |
      */
    key_press: KeyPress,

    /**
      | True if the callback is being invoked
      | when the key is pressed, false if the
      | key is being released.
      | 
      | @see KeyPressMappingSet::addCommand()
      |
      */
    is_key_down: bool,

    /**
      | If the key is being released, this indicates
      | how long it had been held down for.
      | 
      | (Only relevant if isKeyDown is false.)
      |
      */
    millisecs_since_key_pressed: i32,
}

impl<'a> ApplicationCommandTargetInvocationInfo<'a> {

    pub fn new(command: CommandID) -> Self {
    
        todo!();
        /*
        : commandid(command),
        : command_flags(0),
        : invocation_method(direct),
        : originating_component(nullptr),
        : is_key_down(false),
        : millisecs_since_key_pressed(0),

        
        */
    }
}

/**
  | The types of context in which the command
  | might be called.
  |
  */
pub enum InvocationMethod
{
    /**
      | The command is being invoked directly
      | by a piece of code.
      |
      */
    direct = 0,     

    /**
      | The command is being invoked by a key-press.
      |
      */
    fromKeyPress,   

    /**
      | The command is being invoked by a menu
      | selection.
      |
      */
    fromMenu,       

    /**
      | The command is being invoked by a button
      | click.
      |
      */
    fromButton,      
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandTarget.h]

/**
  | A command target publishes a list of
  | command IDs that it can perform.
  | 
  | An ApplicationCommandManager despatches
  | commands to targets, which must be able
  | to provide information about what commands
  | they can handle.
  | 
  | To create a target, you'll need to inherit
  | from this class, implementing all of
  | its pure virtual methods.
  | 
  | For info about how a target is chosen
  | to receive a command, see ApplicationCommandManager::getFirstCommandTarget().
  | 
  | @see ApplicationCommandManager,
  | ApplicationCommandInfo
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct ApplicationCommandTarget {

}

impl Default for ApplicationCommandTarget {
    
    /**
      | Creates a command target.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandTarget.cpp]
impl ApplicationCommandTarget {

    pub fn try_to_invoke(&mut self, 
        info:  &ApplicationCommandTargetInvocationInfo,
        async_: bool) -> bool {
        
        todo!();
        /*
            if (isCommandActive (info.commandID))
        {
            if (async)
            {
                (new ApplicationCommandTargetCommandMessage (this, info))->post();
                return true;
            }

            if (perform (info))
                return true;

            // Hmm.. your target claimed that it could perform this command, but failed to do so.
            // If it can't do it at the moment for some reason, it should clear the 'isActive' flag
            // when it returns the command's info.
            jassertfalse;
        }

        return false;
        */
    }
    
    /**
      | If this object is a Component, this method
      | will search upwards in its current UI
      | hierarchy for the next parent component
      | that implements the ApplicationCommandTarget
      | class.
      | 
      | If your target is a Component, this is
      | a very handy method to use in your getNextCommandTarget()
      | implementation.
      |
      */
    pub fn find_first_target_parent_component(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            if (Component* const c = dynamic_cast<Component*> (this))
            return c->findParentComponentOfClass<ApplicationCommandTarget>();

        return nullptr;
        */
    }
    
    /**
      | Searches this target and all subsequent
      | ones for the first one that can handle
      | the specified command.
      | 
      | This will use getNextCommandTarget()
      | to determine the chain of targets to
      | try after this one.
      |
      */
    pub fn get_target_for_command(&mut self, commandid: CommandID) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            ApplicationCommandTarget* target = this;
        int depth = 0;

        while (target != nullptr)
        {
            Vec<CommandID> commandIDs;
            target->getAllCommands (commandIDs);

            if (commandIDs.contains (commandID))
                return target;

            target = target->getNextCommandTarget();

            ++depth;
            jassert (depth < 100); // could be a recursive command chain??
            jassert (target != this); // definitely a recursive command chain!

            if (depth > 100 || target == this)
                break;
        }

        if (target == nullptr)
        {
            target = ALOEApplication::getInstance();

            if (target != nullptr)
            {
                Vec<CommandID> commandIDs;
                target->getAllCommands (commandIDs);

                if (commandIDs.contains (commandID))
                    return target;
            }
        }

        return nullptr;
        */
    }
    
    /**
      | Checks whether this command can currently
      | be performed by this target.
      | 
      | This will return true only if a call to
      | getCommandInfo() doesn't set the isDisabled
      | flag to indicate that the command is
      | inactive.
      |
      */
    pub fn is_command_active(&mut self, commandid: CommandID) -> bool {
        
        todo!();
        /*
            ApplicationCommandInfo info (commandID);
        info.flags = ApplicationCommandInfo::isDisabled;

        getCommandInfo (commandID, info);

        return (info.flags & ApplicationCommandInfo::isDisabled) == 0;
        */
    }
    
    /**
      | Makes this target invoke a command.
      | 
      | Your code can call this method to invoke
      | a command on this target, but normally
      | you'd call it indirectly via ApplicationCommandManager::invoke()
      | or ApplicationCommandManager::invokeDirectly().
      | 
      | If this target can perform the given
      | command, it will call its perform()
      | method to do so. If not, then getNextCommandTarget()
      | will be used to determine the next target
      | to try, and the command will be passed
      | along to it.
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
      | immediately. @see perform, ApplicationCommandManager::invoke
      |
      */
    pub fn invoke(&mut self, 
        info:  &ApplicationCommandTargetInvocationInfo,
        async_: bool) -> bool {
        
        todo!();
        /*
            ApplicationCommandTarget* target = this;
        int depth = 0;

        while (target != nullptr)
        {
            if (target->tryToInvoke (info, async))
                return true;

            target = target->getNextCommandTarget();

            ++depth;
            jassert (depth < 100); // could be a recursive command chain??
            jassert (target != this); // definitely a recursive command chain!

            if (depth > 100 || target == this)
                break;
        }

        if (target == nullptr)
        {
            target = ALOEApplication::getInstance();

            if (target != nullptr)
                return target->tryToInvoke (info, async);
        }

        return false;
        */
    }
    
    /**
      | Invokes a given command directly on
      | this target.
      | 
      | This is just an easy way to call invoke()
      | without having to fill out the ApplicationCommandTargetInvocationInfo
      | structure.
      |
      */
    pub fn invoke_directly(&mut self, 
        commandid:      CommandID,
        asynchronously: bool) -> bool {
        
        todo!();
        /*
            ApplicationCommandTarget::ApplicationCommandTargetInvocationInfo info (commandID);
        info.invocationMethod = ApplicationCommandTarget::ApplicationCommandTargetInvocationInfo::direct;

        return invoke (info, asynchronously);
        */
    }
}
