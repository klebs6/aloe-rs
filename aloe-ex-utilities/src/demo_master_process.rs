crate::ix!();

/**
  | This class is used by the main process,
  | acting as the master and receiving messages
  | from the slave process.
  |
  */
pub struct DemoMasterProcess<'a> {
    base:  ChildProcessMaster<'a>,
    base2: DeletedAtShutdown,
    demo:  &'a mut ChildProcessDemo<'a>,
    count: i32, // default = 0
}

impl<'a> DemoMasterProcess<'a> {

    pub fn new(d: &mut ChildProcessDemo) -> Self {
    
        todo!();
        /*
        : demo(d),

        
        */
    }

    /**
       This gets called when a message arrives
       from the slave process..
      */
    pub fn handle_message_from_slave(&mut self, mb: &MemoryBlock)  {
        
        todo!();
        /*
            auto incomingMessage = memoryBlockToValueTree (mb);

                demo.logMessage ("Received: " + valueTreeToString (incomingMessage));
        */
    }

    /**
       This gets called if the slave process dies.
      */
    pub fn handle_connection_lost(&mut self)  {
        
        todo!();
        /*
            demo.logMessage ("Connection lost to child process!");
                demo.killChildProcess();
        */
    }

    pub fn send_ping_message_to_slave(&mut self)  {
        
        todo!();
        /*
            ValueTree message ("MESSAGE");
                message.setProperty ("count", count++, nullptr);

                demo.logMessage ("Sending: " + valueTreeToString (message));

                sendMessageToSlave (valueTreeToMemoryBlock (message));
        */
    }
}
