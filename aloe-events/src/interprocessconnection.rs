crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_InterprocessConnection.h]
pub struct InterprocessCommunicationSafeAction<'a> {
    base: SafeActionImpl<'a>,
}

///---------------------------
#[no_copy]
#[leak_detector]
pub struct InterprocessCommunicationConnectionThread<'a> {
    base: Thread,
    owner: &'a mut InterprocessConnection<'a>,
}

impl<'a> InterprocessCommunicationConnectionThread<'a> {

    pub fn new(c: &mut InterprocessConnection) -> Self {
    
        todo!();
        /*
        : thread("Aloe IPC"),
        : owner(c),
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            owner.runThread();
        */
    }
}

///---------------------------
pub trait InterprocessCommunicationInterface {

    /**
      | Called when the connection is first
      | connected.
      | 
      | If the connection was created with the
      | callbacksOnMessageThread flag set,
      | then this will be called on the message
      | thread; otherwise it will be called
      | on a server thread.
      |
      */
    fn connection_made(&mut self);

    /**
      | Called when the connection is broken.
      | 
      | If the connection was created with the
      | callbacksOnMessageThread flag set,
      | then this will be called on the message
      | thread; otherwise it will be called
      | on a server thread.
      |
      */
    fn connection_lost(&mut self);

    /**
      | Called when a message arrives.
      | 
      | When the object at the other end of this
      | connection sends us a message with sendMessage(),
      | this callback is used to deliver it to
      | us.
      | 
      | If the connection was created with the
      | callbacksOnMessageThread flag set,
      | then this will be called on the message
      | thread; otherwise it will be called
      | on a server thread.
      | 
      | @see sendMessage
      |
      */
    fn message_received(&mut self, message: &MemoryBlock);
}

/**
  | Whether the disconnect call should
  | trigger callbacks.
  |
  */
pub enum InterprocessCommunicationNotify { 
    no, 
    yes 
}

/**
  | Manages a simple two-way messaging
  | connection to another process, using
  | either a socket or a named pipe as the
  | transport medium.
  | 
  | To connect to a waiting socket or an open
  | pipe, use the connectToSocket() or
  | connectToPipe() methods. If this succeeds,
  | messages can be sent to the other end,
  | and incoming messages will result in
  | a callback via the messageReceived()
  | method.
  | 
  | To open a pipe and wait for another client
  | to connect to it, use the createPipe()
  | method.
  | 
  | To act as a socket server and create connections
  | for one or more client, see the InterprocessConnectionServer
  | class.
  | 
  | IMPORTANT NOTE: Your derived Connection
  | class *must* call `disconnect` in its
  | destructor in order to cancel any pending
  | messages before the class is destroyed.
  | 
  | @see InterprocessConnectionServer,
  | Socket, NamedPipe
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[leak_detector]
pub struct InterprocessConnection<'a> {
    pipe_and_socket_lock:         ReadWriteLock,
    socket:                       Box<StreamingSocket>,
    pipe:                         Box<NamedPipe>,
    callback_connection_state:    bool, // default = false
    use_message_thread:           bool,
    magic_message_header:         u32,
    pipe_receive_message_timeout: i32, // default = -1
    thread:                       Box<InterprocessCommunicationConnectionThread<'a>>,
    thread_is_running:            AtomicBool, // default = false 
    safe_action:                  Arc<InterprocessCommunicationSafeAction<'a>>,
}

impl<'a> Drop for InterprocessConnection<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        // You *must* call `disconnect` in the destructor of your derived class to ensure
        // that any pending messages are not delivered. If the messages were delivered after
        // destroying the derived class, we'd end up calling the pure virtual implementations
        // of `messageReceived`, `connectionMade` and `connectionLost` which is definitely
        // not a good idea!
        jassert (! safeAction->isSafe());

        callbackConnectionState = false;
        disconnect (4000, Notify::no);
        thread.reset();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_InterprocessConnection.cpp]
impl<'a> InterprocessConnection<'a> {

    /**
      | Creates a connection.
      | 
      | Connections are created manually,
      | connecting them with the connectToSocket()
      | or connectToPipe() methods, or they
      | are created automatically by a InterprocessConnectionServer
      | when a client wants to connect.
      | 
      | -----------
      | @param callbacksOnMessageThread
      | 
      | if true, callbacks to the connectionMade(),
      | connectionLost() and messageReceived()
      | methods will always be made using the
      | message thread; if false, these will
      | be called immediately on the connection's
      | own thread.
      | ----------
      | @param magicMessageHeaderNumber
      | 
      | a magic number to use in the header to
      | check the validity of the data blocks
      | being sent and received. This can be
      | any number, but the sender and receiver
      | must obviously use matching values
      | or they won't recognise each other.
      |
      */
    pub fn new(
        callbacks_on_message_thread: Option<bool>,
        magic_message_header_number: Option<u32>) -> Self {

        let callbacks_on_message_thread: bool =
            callbacks_on_message_thread.unwrap_or(true);

        let magic_message_header_number: u32 =
            magic_message_header_number.unwrap_or(0xf2b49e2c);

    
        todo!();
        /*


            : useMessageThread (callbacksOnMessageThread),
          magicMessageHeader (magicMessageHeaderNumber),
          safeAction (std::make_shared<SafeAction> (*this))

        thread.reset (new ConnectionThread (*this));
        */
    }
    
    /**
      | Tries to connect this object to a socket.
      | 
      | For this to work, the machine on the other
      | end needs to have a InterprocessConnectionServer
      | object waiting to receive client connections
      | on this port number.
      | 
      | -----------
      | @param hostName
      | 
      | the host computer, either a network
      | address or name
      | ----------
      | @param portNumber
      | 
      | the socket port number to try to connect
      | to
      | ----------
      | @param timeOutMillisecs
      | 
      | how long to keep trying before giving
      | up
      | 
      | -----------
      | @return
      | 
      | true if the connection is established
      | successfully @see Socket
      |
      */
    pub fn connect_to_socket(&mut self, 
        host_name:          &String,
        port_number:        i32,
        time_out_millisecs: i32) -> bool {
        
        todo!();
        /*
            disconnect();

        auto s = std::make_unique<StreamingSocket>();

        if (s->connect (hostName, portNumber, timeOutMillisecs))
        {
            const ScopedWriteLock sl (pipeAndSocketLock);
            initialiseWithSocket (std::move (s));
            return true;
        }

        return false;
        */
    }
    
    /**
      | Tries to connect the object to an existing
      | named pipe.
      | 
      | For this to work, another process on
      | the same computer must already have
      | opened an InterprocessConnection
      | object and used createPipe() to create
      | a pipe for this to connect to.
      | 
      | -----------
      | @param pipeName
      | 
      | the name to use for the pipe - this should
      | be unique to your app
      | ----------
      | @param pipeReceiveMessageTimeoutMs
      | 
      | a timeout length to be used when reading
      | or writing to the pipe, or -1 for an infinite
      | timeout.
      | 
      | -----------
      | @return
      | 
      | true if it connects successfully. @see
      | createPipe, NamedPipe
      |
      */
    pub fn connect_to_pipe(&mut self, 
        pipe_name:  &String,
        timeout_ms: i32) -> bool {
        
        todo!();
        /*
            disconnect();

        auto newPipe = std::make_unique<NamedPipe>();

        if (newPipe->openExisting (pipeName))
        {
            const ScopedWriteLock sl (pipeAndSocketLock);
            pipeReceiveMessageTimeout = timeoutMs;
            initialiseWithPipe (std::move (newPipe));
            return true;
        }

        return false;
        */
    }
    
    /**
      | Tries to create a new pipe for other processes
      | to connect to.
      | 
      | This creates a pipe with the given name,
      | so that other processes can use connectToPipe()
      | to connect to the other end.
      | 
      | -----------
      | @param pipeName
      | 
      | the name to use for the pipe - this should
      | be unique to your app
      | ----------
      | @param pipeReceiveMessageTimeoutMs
      | 
      | a timeout length to be used when reading
      | or writing to the pipe, or -1 for an infinite
      | timeout
      | ----------
      | @param mustNotExist
      | 
      | if set to true, the method will fail if
      | the pipe already exists
      | 
      | -----------
      | @return
      | 
      | true if the pipe was created, or false
      | if it fails (e.g. if another process
      | is already using the pipe)
      |
      */
    pub fn create_pipe(
        &mut self, 
        pipe_name:      &String,
        timeout_ms:     i32,
        must_not_exist: Option<bool>
    ) -> bool {

        let must_not_exist: bool = must_not_exist.unwrap_or(false);
        
        todo!();
        /*
            disconnect();

        auto newPipe = std::make_unique<NamedPipe>();

        if (newPipe->createNewPipe (pipeName, mustNotExist))
        {
            const ScopedWriteLock sl (pipeAndSocketLock);
            pipeReceiveMessageTimeout = timeoutMs;
            initialiseWithPipe (std::move (newPipe));
            return true;
        }

        return false;
        */
    }
    
    /**
      | Disconnects and closes any currently-open
      | sockets or pipes.
      | 
      | Derived classes *must* call this in
      | their destructors in order to avoid
      | undefined behaviour.
      | 
      | -----------
      | @param timeoutMs
      | 
      | the time in ms to wait before killing
      | the thread by force
      | ----------
      | @param notify
      | 
      | whether or not to call `connectionLost`
      |
      */
    pub fn disconnect(
        &mut self, 
        timeout_ms: Option<i32>,
        notify:     Option<InterprocessCommunicationNotify>

    ) {

        let timeout_ms = timeout_ms.unwrap_or(-1);
        let notify     = notify.unwrap_or(InterprocessCommunicationNotify::yes);
        
        todo!();
        /*
            thread->signalThreadShouldExit();

        {
            const ScopedReadLock sl (pipeAndSocketLock);
            if (socket != nullptr)  socket->close();
            if (pipe != nullptr)    pipe->close();
        }

        thread->stopThread (timeoutMs);
        deletePipeAndSocket();

        if (notify == InterprocessCommunicationNotify::yes)
            connectionLostInt();

        callbackConnectionState = false;
        safeAction->setSafe (false);
        */
    }
    
    pub fn delete_pipe_and_socket(&mut self)  {
        
        todo!();
        /*
            const ScopedWriteLock sl (pipeAndSocketLock);
        socket.reset();
        pipe.reset();
        */
    }
    
    /**
      | True if a socket or pipe is currently
      | active.
      |
      */
    pub fn is_connected(&self) -> bool {
        
        todo!();
        /*
            const ScopedReadLock sl (pipeAndSocketLock);

        return ((socket != nullptr && socket->isConnected())
                  || (pipe != nullptr && pipe->isOpen()))
                && threadIsRunning;
        */
    }
    
    /**
      | Returns the name of the machine at the
      | other end of this connection.
      | 
      | This may return an empty string if the
      | name is unknown.
      |
      */
    pub fn get_connected_host_name(&self) -> String {
        
        todo!();
        /*
            {
            const ScopedReadLock sl (pipeAndSocketLock);

            if (pipe == nullptr && socket == nullptr)
                return {};

            if (socket != nullptr && ! socket->isLocal())
                return socket->getHostName();
        }

        return IPAddress::local().toString();
        */
    }
    
    /**
      | Tries to send a message to the other end
      | of this connection.
      | 
      | This will fail if it's not connected,
      | or if there's some kind of write error.
      | If it succeeds, the connection object
      | at the other end will receive the message
      | by a callback to its messageReceived()
      | method.
      | 
      | @see messageReceived
      |
      */
    pub fn send_message(&mut self, message: &MemoryBlock) -> bool {
        
        todo!();
        /*
            uint32 messageHeader[2] = { ByteOrder::swapIfBigEndian (magicMessageHeader),
                                    ByteOrder::swapIfBigEndian ((uint32) message.getSize()) };

        MemoryBlock messageData (sizeof (messageHeader) + message.getSize());
        messageData.copyFrom (messageHeader, 0, sizeof (messageHeader));
        messageData.copyFrom (message.getData(), sizeof (messageHeader), message.getSize());

        return writeData (messageData.getData(), (int) messageData.getSize()) == (int) messageData.getSize();
        */
    }
    
    pub fn write_data(&mut self, 
        data:      *mut c_void,
        data_size: i32) -> i32 {
        
        todo!();
        /*
            const ScopedReadLock sl (pipeAndSocketLock);

        if (socket != nullptr)
            return socket->write (data, dataSize);

        if (pipe != nullptr)
            return pipe->write (data, dataSize, pipeReceiveMessageTimeout);

        return 0;
        */
    }
    
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            safeAction->setSafe (true);
        threadIsRunning = true;
        connectionMadeInt();
        thread->startThread();
        */
    }
    
    pub fn initialise_with_socket(&mut self, new_socket: Box<StreamingSocket>)  {
        
        todo!();
        /*
            jassert (socket == nullptr && pipe == nullptr);
        socket = std::move (newSocket);
        initialise();
        */
    }
    
    pub fn initialise_with_pipe(&mut self, new_pipe: Box<NamedPipe>)  {
        
        todo!();
        /*
            jassert (socket == nullptr && pipe == nullptr);
        pipe = std::move (newPipe);
        initialise();
        */
    }
    
    pub fn connection_made_int(&mut self)  {
        
        todo!();
        /*
            if (! callbackConnectionState)
        {
            callbackConnectionState = true;

            if (useMessageThread)
                (new ConnectionStateMessage (safeAction, true))->post();
            else
                connectionMade();
        }
        */
    }
    
    pub fn connection_lost_int(&mut self)  {
        
        todo!();
        /*
            if (callbackConnectionState)
        {
            callbackConnectionState = false;

            if (useMessageThread)
                (new ConnectionStateMessage (safeAction, false))->post();
            else
                connectionLost();
        }
        */
    }
    
    pub fn deliver_data_int(&mut self, data: &MemoryBlock)  {
        
        todo!();
        /*
            jassert (callbackConnectionState);

        if (useMessageThread)
            (new DataDeliveryMessage (safeAction, data))->post();
        else
            messageReceived (data);
        */
    }
    
    pub fn read_data(&mut self, 
        data: *mut c_void,
        num:  i32) -> i32 {
        
        todo!();
        /*
            const ScopedReadLock sl (pipeAndSocketLock);

        if (socket != nullptr)
            return socket->read (data, num, true);

        if (pipe != nullptr)
            return pipe->read (data, num, pipeReceiveMessageTimeout);

        jassertfalse;
        return -1;
        */
    }
    
    pub fn read_next_message(&mut self) -> bool {
        
        todo!();
        /*
            uint32 messageHeader[2];
        auto bytes = readData (messageHeader, sizeof (messageHeader));

        if (bytes == (int) sizeof (messageHeader)
             && ByteOrder::swapIfBigEndian (messageHeader[0]) == magicMessageHeader)
        {
            auto bytesInMessage = (int) ByteOrder::swapIfBigEndian (messageHeader[1]);

            if (bytesInMessage > 0)
            {
                MemoryBlock messageData ((size_t) bytesInMessage, true);
                int bytesRead = 0;

                while (bytesInMessage > 0)
                {
                    if (thread->threadShouldExit())
                        return false;

                    auto numThisTime = jmin (bytesInMessage, 65536);
                    auto bytesIn = readData (addBytesToPointer (messageData.getData(), bytesRead), numThisTime);

                    if (bytesIn <= 0)
                        break;

                    bytesRead += bytesIn;
                    bytesInMessage -= bytesIn;
                }

                if (bytesRead >= 0)
                    deliverDataInt (messageData);
            }

            return true;
        }

        if (bytes < 0)
        {
            if (socket != nullptr)
                deletePipeAndSocket();

            connectionLostInt();
        }

        return false;
        */
    }
    
    pub fn run_thread(&mut self)  {
        
        todo!();
        /*
            while (! thread->threadShouldExit())
        {
            if (socket != nullptr)
            {
                auto ready = socket->waitUntilReady (true, 100);

                if (ready < 0)
                {
                    deletePipeAndSocket();
                    connectionLostInt();
                    break;
                }

                if (ready == 0)
                {
                    thread->wait (1);
                    continue;
                }
            }
            else if (pipe != nullptr)
            {
                if (! pipe->isOpen())
                {
                    deletePipeAndSocket();
                    connectionLostInt();
                    break;
                }
            }
            else
            {
                break;
            }

            if (thread->threadShouldExit() || ! readNextMessage())
                break;
        }

        threadIsRunning = false;
        */
    }

    /**
      | Returns the socket that this connection
      | is using (or nullptr if it uses a pipe).
      |
      */
    pub fn get_socket(&self) -> *mut StreamingSocket {
        
        todo!();
        /*
            return socket.get();
        */
    }

    /**
      | Returns the pipe that this connection
      | is using (or nullptr if it uses a socket).
      |
      */
    pub fn get_pipe(&self) -> *mut NamedPipe {
        
        todo!();
        /*
            return pipe.get();
        */
    }
}

///---------------------------
pub struct SafeActionImpl<'a> {
    mutex: CriticalSection,
    ref_:  &'a mut InterprocessConnection<'a>,
    safe:  bool, // default = false
}

impl<'a> SafeActionImpl<'a> {

    pub fn new(p: &mut InterprocessConnection) -> Self {
    
        todo!();
        /*
        : ref_(p),
        */
    }
    
    pub fn if_safe<Fn>(&mut self, fn_: Fn)  {
    
        todo!();
        /*
            const ScopedLock lock (mutex);

            if (safe)
                fn (ref);
        */
    }
    
    pub fn set_safe(&mut self, s: bool)  {
        
        todo!();
        /*
            const ScopedLock lock (mutex);
            safe = s;
        */
    }
    
    pub fn is_safe(&mut self) -> bool {
        
        todo!();
        /*
            const ScopedLock lock (mutex);
            return safe;
        */
    }
}


///------------------------------
#[no_copy]
#[leak_detector]
pub struct ConnectionStateMessage<'a> {
    safe_action:     Arc<SafeActionImpl<'a>>,
    connection_made: bool,
}

impl<'a> MessageBaseInterface for ConnectionStateMessage<'a> {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            safeAction->ifSafe ([this] (InterprocessConnection& owner)
            {
                if (connectionMade)
                    owner.connectionMade();
                else
                    owner.connectionLost();
            });
        */
    }
}

impl<'a> ConnectionStateMessage<'a> {

    pub fn new(
        ipc:       Arc<SafeActionImpl>,
        connected: bool) -> Self {
    
        todo!();
        /*


            : safeAction (ipc), connectionMade (connected)
        */
    }
}

///------------------------------
pub struct DataDeliveryMessage<'a> {
    base:        Message,
    safe_action: Arc<SafeActionImpl<'a>>,
    data:        MemoryBlock,
}

impl<'a> DataDeliveryMessage<'a> {

    pub fn new(
        ipc: Arc<SafeActionImpl>,
        d:   &MemoryBlock) -> Self {
    
        todo!();
        /*


            : safeAction (ipc), data (d)
        */
    }
    
    pub fn message_callback(&mut self)  {
        
        todo!();
        /*
            safeAction->ifSafe ([this] (InterprocessConnection& owner)
            {
                owner.messageReceived (data);
            });
        */
    }
}
