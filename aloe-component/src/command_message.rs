crate::ix!();

pub trait HandleCommandMessage {

    /**
      | Called to handle a command that was sent
      | by postCommandMessage().
      | 
      | This is called by the message thread
      | when a command message arrives, and
      | the component can override this method
      | to process it in any way it needs to.
      | 
      | @see postCommandMessage
      |
      */
    fn handle_command_message(&mut self, command_id: i32);
}
