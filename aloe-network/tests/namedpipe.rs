crate::ix!();

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
pub struct NamedPipeTests {
    base: UnitTest,
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl Default for NamedPipeTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("NamedPipe", UnitTestCategories::networking
        */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl NamedPipeTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            const auto pipeName = "TestPipe" + String ((intptr_t) Thread::getCurrentThreadId());

            beginTest ("Pre test cleanup");
            {
                NamedPipe pipe;
                expect (pipe.createNewPipe (pipeName, false));
            }

            beginTest ("Create pipe");
            {
                NamedPipe pipe;
                expect (! pipe.isOpen());

                expect (pipe.createNewPipe (pipeName, true));
                expect (pipe.isOpen());

                expect (pipe.createNewPipe (pipeName, false));
                expect (pipe.isOpen());

                NamedPipe otherPipe;
                expect (! otherPipe.createNewPipe (pipeName, true));
                expect (! otherPipe.isOpen());
            }

            beginTest ("Existing pipe");
            {
                NamedPipe pipe;

                expect (! pipe.openExisting (pipeName));
                expect (! pipe.isOpen());

                expect (pipe.createNewPipe (pipeName, true));

                NamedPipe otherPipe;
                expect (otherPipe.openExisting (pipeName));
                expect (otherPipe.isOpen());
            }

            int sendData = 4684682;

            beginTest ("Receive message created pipe");
            {
                NamedPipe pipe;
                expect (pipe.createNewPipe (pipeName, true));

                WaitableEvent senderFinished;
                SenderThread sender (pipeName, false, senderFinished, sendData);

                sender.startThread();

                int recvData = -1;
                auto bytesRead = pipe.read (&recvData, sizeof (recvData), 2000);

                expect (senderFinished.wait (4000));

                expectEquals (bytesRead, (int) sizeof (recvData));
                expectEquals (sender.result, (int) sizeof (sendData));
                expectEquals (recvData, sendData);
            }

            beginTest ("Receive message existing pipe");
            {
                WaitableEvent senderFinished;
                SenderThread sender (pipeName, true, senderFinished, sendData);

                NamedPipe pipe;
                expect (pipe.openExisting (pipeName));

                sender.startThread();

                int recvData = -1;
                auto bytesRead = pipe.read (&recvData, sizeof (recvData), 2000);

                expect (senderFinished.wait (4000));

                expectEquals (bytesRead, (int) sizeof (recvData));
                expectEquals (sender.result, (int) sizeof (sendData));
                expectEquals (recvData, sendData);
            }

            beginTest ("Send message created pipe");
            {
                NamedPipe pipe;
                expect (pipe.createNewPipe (pipeName, true));

                WaitableEvent receiverFinished;
                ReceiverThread receiver (pipeName, false, receiverFinished);

                receiver.startThread();

                auto bytesWritten = pipe.write (&sendData, sizeof (sendData), 2000);

                expect (receiverFinished.wait (4000));

                expectEquals (bytesWritten, (int) sizeof (sendData));
                expectEquals (receiver.result, (int) sizeof (receiver.recvData));
                expectEquals (receiver.recvData, sendData);
            }

            beginTest ("Send message existing pipe");
            {
                WaitableEvent receiverFinished;
                ReceiverThread receiver (pipeName, true, receiverFinished);

                NamedPipe pipe;
                expect (pipe.openExisting (pipeName));

                receiver.startThread();

                auto bytesWritten = pipe.write (&sendData, sizeof (sendData), 2000);

                expect (receiverFinished.wait (4000));

                expectEquals (bytesWritten, (int) sizeof (sendData));
                expectEquals (receiver.result, (int) sizeof (receiver.recvData));
                expectEquals (receiver.recvData, sendData);
            }
        */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
pub struct NamedPipeThread {
    base:           Thread,
    pipe:           NamedPipe,
    pipe_name:      &String,
    work_completed: &mut WaitableEvent,
    result:         i32, // default = -2
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl NamedPipeThread {

    pub fn new(
        t_name:             &String,
        name:               &String,
        should_create_pipe: bool,
        completed:          &mut WaitableEvent) -> Self {
    
        todo!();
        /*


            : Thread (tName), pipeName (pName), workCompleted (completed)

                if (shouldCreatePipe)
                    pipe.createNewPipe (pipeName);
                else
                    pipe.openExisting (pipeName);
        */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
pub struct SenderThread {
    base:      NamedPipeThread,
    send_data: i32,
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl Drop for SenderThread {
    fn drop(&mut self) {
        todo!();
        /* 
                stopThread (100);
             */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl SenderThread {

    pub fn new(
        name:               &String,
        should_create_pipe: bool,
        completed:          &mut WaitableEvent,
        data:               i32) -> Self {
    
        todo!();
        /*


            : NamedPipeThread ("NamePipeSender", pName, shouldCreatePipe, completed),
                  sendData (sData)
        */
    }

    pub fn run(&mut self)  {
        
        todo!();
        /*
            result = pipe.write (&sendData, sizeof (sendData), 2000);
                workCompleted.signal();
        */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
pub struct ReceiverThread {
    base:      NamedPipeThread,
    recv_data: i32, // default = -2
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl Drop for ReceiverThread {
    fn drop(&mut self) {
        todo!();
        /* 
                stopThread (100);
             */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
impl ReceiverThread {

    pub fn new(
        name:               &String,
        should_create_pipe: bool,
        completed:          &mut WaitableEvent) -> Self {
    
        todo!();
        /*


            : NamedPipeThread ("NamePipeSender", pName, shouldCreatePipe, completed)
        */
    }

    pub fn run(&mut self)  {
        
        todo!();
        /*
            result = pipe.read (&recvData, sizeof (recvData), 2000);
                workCompleted.signal();
        */
    }
}

#[cfg(not(ALOE_WASM))]
#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static NamedPipeTests namedPipeTests;
    */
}
