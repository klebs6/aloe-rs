crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_ConnectedChildProcess.h]

pub trait ChildProcessMasterInterface {

    /**
      | This will be called to deliver a message
      | from the slave process.
      | 
      | The call will probably be made on a background
      | thread, so be careful with your thread-safety!
      |
      */
    fn handle_message_from_slave(&mut self, _0: &MemoryBlock) {}

    /**
      | This will be called when the slave process
      | dies or is somehow disconnected.
      | 
      | The call will probably be made on a background
      | thread, so be careful with your thread-safety!
      |
      */
    fn handle_connection_lost(&mut self) {}
}

pub trait ChildProcessSlaveInterface {

    /**
      | This will be called to deliver messages
      | from the master process.
      | 
      | The call will probably be made on a background
      | thread, so be careful with your thread-safety!
      | You may want to respond by sending back
      | a message with sendMessageToMaster()
      |
      */
    fn handle_message_from_master(&mut self, _0: &MemoryBlock) {}

    /**
      | This will be called when the master process
      | finishes connecting to this slave.
      | 
      | The call will probably be made on a background
      | thread, so be careful with your thread-safety!
      |
      */
    fn handle_connection_made(&mut self) {}

    /**
      | This will be called when the connection
      | to the master process is lost.
      | 
      | The call may be made from any thread (including
      | the message thread).
      | 
      | Typically, if your process only exists
      | to act as a slave, you should probably
      | exit when this happens.
      |
      */
    fn handle_connection_lost(&mut self) {}
}

///-----------------------
#[leak_detector]
#[no_copy]
pub struct ChildProcessSlaveConnection<'a> {
    base:  InterprocessConnection<'a>,
    base2: ChildProcessPingThread<'a>,
    owner: &'a mut ChildProcessSlave<'a>,
}

impl<'a> Drop for ChildProcessSlaveConnection<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            stopThread (10000);
         */
    }
}

impl<'a> ChildProcessSlaveConnection<'a> {

    pub fn new(
        p:         &mut ChildProcessSlave,
        pipe_name: &String,
        timeout:   i32) -> Self {
    
        todo!();
        /*


            : InterprocessConnection (false, magicMastSlaveConnectionHeader),
              ChildProcessPingThread (timeout),
              owner (p)

            connectToPipe (pipeName, timeoutMs);
            startThread (4);
        */
    }
    
    pub fn connection_lost(&mut self)  {
        
        todo!();
        /*
            owner.handleConnectionLost();
        */
    }
    
    pub fn send_ping_message(&mut self, m: &MemoryBlock) -> bool {
        
        todo!();
        /*
            return owner.sendMessageToMaster (m);
        */
    }
    
    pub fn ping_failed(&mut self)  {
        
        todo!();
        /*
            connectionLost();
        */
    }
    
    pub fn message_received(&mut self, m: &MemoryBlock)  {
        
        todo!();
        /*
            pingReceived();

            if (isMessageType (m, pingMessage))
                return;

            if (isMessageType (m, killMessage))
                return triggerConnectionLostMessage();

            if (isMessageType (m, startMessage))
                return owner.handleConnectionMade();

            owner.handleMessageFromMaster (m);
        */
    }
}

/**
  | Acts as the slave end of a master/slave
  | pair of connected processes.
  | 
  | The ChildProcessSlave and ChildProcessMaster
  | classes make it easy for an app to spawn
  | a child process, and to manage a 2-way
  | messaging connection to control it.
  | 
  | To use the system, you need to create
  | subclasses of both ChildProcessSlave
  | and ChildProcessMaster. To instantiate
  | the ChildProcessSlave object, you
  | must add some code to your main() or ALOEApplication::initialise()
  | function that calls the initialiseFromCommandLine()
  | method to check the app's command-line
  | parameters to see whether it's being
  | launched as a child process. If this
  | returns true then the slave process
  | can be allowed to run, and its handleMessageFromMaster()
  | method will be called whenever a message
  | arrives.
  | 
  | The aloe demo app has a good example of
  | this class in action.
  | 
  | @see ChildProcessMaster, InterprocessConnection,
  | ChildProcess
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ChildProcessSlave<'a> {
    connection: Box<ChildProcessSlaveConnection<'a>>,
}

impl<'a> Default for ChildProcessSlave<'a> {
    
    /**
      | Creates a non-connected slave process.
      | 
      | Use initialiseFromCommandLine to
      | connect to a master process.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a> ChildProcessSlave<'a> {

    /**
      | Tries to send a message to the master
      | process.
      | 
      | This returns true if the message was
      | sent, but doesn't check that it actually
      | gets delivered at the other end. If successful,
      | the data will emerge in a call to your
      | ChildProcessMaster::handleMessageFromSlave().
      |
      */
    pub fn send_message_to_master(&mut self, mb: &MemoryBlock) -> bool {
        
        todo!();
        /*
            if (connection != nullptr)
            return connection->sendMessage (mb);

        jassertfalse; // this can only be used when the connection is active!
        return false;
        */
    }
    
    /**
      | This checks some command-line parameters
      | to see whether they were generated by
      | ChildProcessMaster::launchSlaveProcess(),
      | and if so, connects to that master process.
      | 
      | In an exe that can be used as a child process,
      | you should add some code to your main()
      | or ALOEApplication::initialise()
      | that calls this method.
      | 
      | The commandLineUniqueID should be
      | a short alphanumeric identifier (no
      | spaces!) that matches the string passed
      | to ChildProcessMaster::launchSlaveProcess().
      | 
      | The timeoutMs parameter lets you specify
      | how long the child process is allowed
      | to run without receiving a ping from
      | the master before the master is considered
      | to have died, and handleConnectionLost()
      | will be called. Passing <= 0 for this
      | timeout makes it use a default value.
      | 
      | Returns true if the command-line matches
      | and the connection is made successfully.
      |
      */
    pub fn initialise_from_command_line(
        &mut self, 
        command_line:          &String,
        command_line_uniqueid: &String,
        timeout_ms:            Option<i32>

    ) -> bool {

        let timeout_ms: i32 = timeout_ms.unwrap_or(0);
        
        todo!();
        /*
            auto prefix = getCommandLinePrefix (commandLineUniqueID);

        if (commandLine.trim().startsWith (prefix))
        {
            auto pipeName = commandLine.fromFirstOccurrenceOf (prefix, false, false)
                                       .upToFirstOccurrenceOf (" ", false, false).trim();

            if (pipeName.isNotEmpty())
            {
                connection.reset (new Connection (*this, pipeName, timeoutMs <= 0 ? defaultTimeoutMs : timeoutMs));

                if (! connection->isConnected())
                    connection.reset();
            }
        }

        return connection != nullptr;
        */
    }
}

//------------------------------
#[no_copy]
#[leak_detector]
pub struct ChildProcessMasterConnection<'a> {
    base:  InterprocessConnection<'a>,
    base2: ChildProcessPingThread<'a>,
    owner: &'a mut ChildProcessMaster<'a>,
}

impl<'a> Drop for ChildProcessMasterConnection<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            stopThread (10000);
         */
    }
}

impl<'a> ChildProcessMasterConnection<'a> {

    pub fn new(
        m:         &mut ChildProcessMaster,
        pipe_name: &String,
        timeout:   i32) -> Self {
    
        todo!();
        /*


            : InterprocessConnection (false, magicMastSlaveConnectionHeader),
                  ChildProcessPingThread (timeout),
                  owner (m)

                if (createPipe (pipeName, timeoutMs))
                    startThread (4);
        */
    }
    
    pub fn connection_made(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn connection_lost(&mut self)  {
        
        todo!();
        /*
            owner.handleConnectionLost();
        */
    }
    
    pub fn send_ping_message(&mut self, m: &MemoryBlock) -> bool {
        
        todo!();
        /*
            return owner.sendMessageToSlave (m);
        */
    }
    
    pub fn ping_failed(&mut self)  {
        
        todo!();
        /*
            connectionLost();
        */
    }
    
    pub fn message_received(&mut self, m: &MemoryBlock)  {
        
        todo!();
        /*
            pingReceived();

                if (m.getSize() != specialMessageSize || ! isMessageType (m, pingMessage))
                    owner.handleMessageFromSlave (m);
        */
    }
}

/**
  | Acts as the master in a master/slave
  | pair of connected processes.
  | 
  | The ChildProcessSlave and ChildProcessMaster
  | classes make it easy for an app to spawn
  | a child process, and to manage a 2-way
  | messaging connection to control it.
  | 
  | To use the system, you need to create
  | subclasses of both ChildProcessSlave
  | and ChildProcessMaster. When you want
  | your master process to launch the slave,
  | you just call launchSlaveProcess(),
  | and it'll attempt to launch the executable
  | that you specify (which may be the same
  | exe), and assuming it has been set-up
  | to correctly parse the command-line
  | parameters (see ChildProcessSlave)
  | then a two-way connection will be created.
  | 
  | The aloe demo app has a good example of
  | this class in action.
  | 
  | @see ChildProcessSlave, InterprocessConnection,
  | ChildProcess
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ChildProcessMaster<'a> {
    child_process: Box<ChildProcess>,
    connection:    Box<ChildProcessMasterConnection<'a>>,
}

impl<'a> Default for ChildProcessMaster<'a> {
    
    /**
      | Creates an uninitialised master process
      | object.
      | 
      | Use launchSlaveProcess to launch and
      | connect to a child process.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for ChildProcessMaster<'a> {
    /**
      | Destructor.
      | 
      | Note that the destructor calls killSlaveProcess(),
      | but doesn't wait for the child process
      | to finish terminating.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       
                 killSlaveProcess();
                 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_ConnectedChildProcess.cpp]
impl<'a> ChildProcessMaster<'a> {

    /**
      | Attempts to send a message to the slave
      | process.
      | 
      | This returns true if the message was
      | dispatched, but doesn't check that
      | it actually gets delivered at the other
      | end. If successful, the data will emerge
      | in a call to your ChildProcessSlave::handleMessageFromMaster().
      |
      */
    pub fn send_message_to_slave(&mut self, mb: &MemoryBlock) -> bool {
        
        todo!();
        /*
            if (connection != nullptr)
            return connection->sendMessage (mb);

        jassertfalse; // this can only be used when the connection is active!
        return false;
        */
    }
    
    /**
      | Attempts to launch and connect to a slave
      | process. This will start the given executable,
      | passing it a special command-line parameter
      | based around the commandLineUniqueID
      | string, which must be a short alphanumeric
      | string (no spaces!) that identifies
      | your app. The exe that gets launched
      | must respond by calling ChildProcessSlave::initialiseFromCommandLine()
      | in its startup code, and must use a matching
      | ID to commandLineUniqueID.
      | 
      | The timeoutMs parameter lets you specify
      | how long the child process is allowed
      | to go without sending a ping before it
      | is considered to have died and handleConnectionLost()
      | will be called. Passing <= 0 for this
      | timeout makes it use a default value.
      | 
      | If this all works, the method returns
      | true, and you can begin sending and receiving
      | messages with the slave process.
      | 
      | If a child process is already running,
      | this will call killSlaveProcess()
      | and start a new one.
      |
      */
    pub fn launch_slave_process(&mut self, 
        executable:            &File,
        command_line_uniqueid: &String,
        timeout_ms:            Option<i32>,
        stream_flags:          Option<ChildProcessStreamFlags>) -> bool {

        let timeout_ms: i32 = timeout_ms.unwrap_or(0);

        let stream_flags = stream_flags.unwrap_or(
            ChildProcessStreamFlags::wantStdOut | ChildProcessStreamFlags::wantStdErr
        );
        
        todo!();
        /*
            killSlaveProcess();

        auto pipeName = "p" + String::toHexString (Random().nextInt64());

        StringArray args;
        args.add (executable.getFullPathName());
        args.add (getCommandLinePrefix (commandLineUniqueID) + pipeName);

        childProcess.reset (new ChildProcess());

        if (childProcess->start (args, streamFlags))
        {
            connection.reset (new Connection (*this, pipeName, timeoutMs <= 0 ? defaultTimeoutMs : timeoutMs));

            if (connection->isConnected())
            {
                sendMessageToSlave ({ startMessage, specialMessageSize });
                return true;
            }

            connection.reset();
        }

        return false;
        */
    }
    
    /**
      | Sends a kill message to the slave, and
      | disconnects from it.
      | 
      | -----------
      | @note
      | 
      | this won't wait for it to terminate.
      |
      */
    pub fn kill_slave_process(&mut self)  {
        
        todo!();
        /*
            if (connection != nullptr)
        {
            sendMessageToSlave ({ killMessage, specialMessageSize });
            connection->disconnect();
            connection.reset();
        }

        childProcess.reset();
        */
    }
}

pub const magicMastSlaveConnectionHeader: u32 = 0x712baf04;

pub const startMessage: &'static str = "__ipc_st";
pub const killMessage:  &'static str = "__ipc_k_";
pub const pingMessage:  &'static str = "__ipc_p_";

pub const specialMessageSize: usize = 8;
pub const defaultTimeoutMs:   usize = 8000;

pub fn is_message_type(
        mb:           &MemoryBlock,
        message_type: *const u8) -> bool {
    
    todo!();
    /*
        return mb.matches (messageType, (size_t) specialMessageSize);
    */
}

pub fn get_command_line_prefix(command_line_uniqueid: &String) -> String {
    
    todo!();
    /*
        return "--" + commandLineUniqueID + ":";
    */
}

/**
   This thread sends and receives ping messages
   every second, so that it can find out if the
   other process has stopped running.
  */
#[no_copy]
#[leak_detector]
pub struct ChildProcessPingThread<'a> {
    base:       Thread,
    base2:      AsyncUpdater<'a>,
    timeout_ms: i32,
    countdown:  AtomicI32,
}

pub trait ChildProcessPingThreadInterface {

    fn send_ping_message(&mut self, _0: &MemoryBlock) -> bool;

    fn ping_failed(&mut self);
}

impl<'a> ChildProcessPingThread<'a> {

    pub fn new(timeout: i32) -> Self {
    
        todo!();
        /*
        : thread("IPC ping"),
        : timeout_ms(timeout),

            pingReceived();
        */
    }
    
    pub fn ping_received(&mut self)  {
        
        todo!();
        /*
            countdown = timeoutMs / 1000 + 1;
        */
    }
    
    pub fn trigger_connection_lost_message(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            pingFailed();
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
            {
                if (--countdown <= 0 || ! sendPingMessage ({ pingMessage, specialMessageSize }))
                {
                    triggerConnectionLostMessage();
                    break;
                }

                wait (1000);
            }
        */
    }
}
