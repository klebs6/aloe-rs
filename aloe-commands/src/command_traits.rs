crate::ix!();

pub trait ApplicationCommandTargetGetNextCommandTarget {
    
    /**
      | This must return the next target to try
      | after this one.
      | 
      | When a command is being sent, and the
      | first target can't handle that command,
      | this method is used to determine the
      | next target that should be tried.
      | 
      | It may return nullptr if it doesn't know
      | of another target.
      | 
      | If your target is a Component, you would
      | usually use the findFirstTargetParentComponent()
      | method to return a parent component
      | that might want to handle it.
      | 
      | @see invoke
      |
      */
    fn get_next_command_target(&mut self) -> *mut ApplicationCommandTarget;
}

pub trait ApplicationCommandTargetGetAllCommands {

    /**
      | This must return a complete list of commands
      | that this target can handle.
      | 
      | Your target should add all the command
      | IDs that it handles to the array that
      | is passed-in.
      |
      */
    fn get_all_commands(&mut self, commands: &mut Vec<CommandID>);
}

pub trait ApplicationCommandTargetGetCommandInfo {

    /**
      | This must provide details about one
      | of the commands that this target can
      | perform.
      | 
      | This will be called with one of the command
      | IDs that the target provided in its getAllCommands()
      | methods.
      | 
      | It should fill-in all appropriate fields
      | of the ApplicationCommandInfo structure
      | with suitable information about the
      | command. (The commandID field will
      | already have been filled-in by the caller).
      | 
      | The easiest way to set the info is using
      | the ApplicationCommandInfo::setInfo()
      | method to set all the fields at once.
      | 
      | If the command is currently inactive
      | for some reason, this method must use
      | ApplicationCommandInfo::setActive()
      | to make that clear, (or it should set
      | the isDisabled bit of the ApplicationCommandInfo::flags
      | field).
      | 
      | Any default key-presses for the command
      | should be appended to the ApplicationCommandInfo::defaultKeypresses
      | field.
      | 
      | -----------
      | @note
      | 
      | if you change something that affects
      | the status of the commands that would
      | be returned by this method (e.g. something
      | that makes some commands active or inactive),
      | you should call ApplicationCommandManager::commandStatusChanged()
      | to cause the manager to refresh its status.
      |
      */
    fn get_command_info(&mut self, 
            commandid: CommandID,
            result:    &mut ApplicationCommandInfo);
}

pub trait ApplicationCommandTargetPerform {

    /**
      | This must actually perform the specified
      | command.
      | 
      | If this target is able to perform the
      | command specified by the commandID
      | field of the ApplicationCommandTargetInvocationInfo structure,
      | then it should do so, and must return
      | true.
      | 
      | If it can't handle this command, it should
      | return false, which tells the caller
      | to pass the command on to the next target
      | in line.
      | 
      | @see invoke, ApplicationCommandManager::invoke
      |
      */
    fn perform(&mut self, info: &ApplicationCommandTargetInvocationInfo) -> bool;

}
