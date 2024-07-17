crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/ChildProcessDemo.h]

/**
  | This class gets instantiated in the
  | child process, and receives messages
  | from the master process.
  |
  */
#[derive(Default)]
pub struct DemoSlaveProcess<'a> {
    base:  ChildProcessSlave<'a>,
    base2: DeletedAtShutdown,
}
impl<'a> DemoSlaveProcess<'a> {

    pub fn handle_message_from_master(&mut self, mb: &MemoryBlock)  {
        
        todo!();
        /*
            ValueTree incomingMessage (memoryBlockToValueTree (mb));

            /*  In this demo we're only expecting one type of message, which will contain a 'count' parameter -
                we'll just increment that number and send back a new message containing the new number.

                Obviously in a real app you'll probably want to look at the type of the message, and do
                some more interesting behaviour.
            */

            ValueTree reply ("REPLY");
            reply.setProperty ("countPlusOne", static_cast<int> (incomingMessage["count"]) + 1, nullptr);

            sendMessageToMaster (valueTreeToMemoryBlock (reply));
        */
    }

    pub fn handle_connection_made(&mut self)  {
        
        todo!();
        /*
            // This method is called when the connection is established, and in response, we'll just
            // send off a message to say hello.
            ValueTree reply ("HelloWorld");
            sendMessageToMaster (valueTreeToMemoryBlock (reply));
        */
    }

    /**
      | If no pings are received from the master
      | process for a number of seconds, then
      | this will get invoked.
      | 
      | Typically you'll want to use this as
      | a signal to kill the process as quickly
      | as possible, as you don't want to leave
      | it hanging around as a zombie..
      |
      */
    pub fn handle_connection_lost(&mut self)  {
        
        todo!();
        /*
            ALOEApplication::quit();
        */
    }
}
