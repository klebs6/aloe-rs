crate::ix!();

pub trait ApplicationCommandManagerGetFirstCommandTarget {

    /**
      | Chooses the ApplicationCommandTarget
      | to which a command should be sent.
      | 
      | Whenever the manager needs to know which
      | target a command should be sent to, it
      | calls this method to determine the first
      | one to try.
      | 
      | By default, this method will return
      | the target that was set by calling setFirstCommandTarget().
      | If no target is set, it will return the
      | result of findDefaultComponentTarget().
      | 
      | If you need to make sure all commands
      | go via your own custom target, then you
      | can either use setFirstCommandTarget()
      | to specify a single target, or override
      | this method if you need more complex
      | logic to choose one.
      | 
      | It may return nullptr if no targets are
      | available.
      | 
      | @see getTargetForCommand, invoke,
      | invokeDirectly
      |
      */
    fn get_first_command_target(&mut self, commandid: CommandID) -> *mut ApplicationCommandTarget;
}
