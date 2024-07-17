crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_Socket.h]

/**
  | A wrapper for a streaming (TCP) socket.
  | 
  | This allows low-level use of sockets;
  | for an easier-to-use messaging layer
  | on top of sockets, you could also try
  | the InterprocessConnection class.
  | 
  | @see DatagramSocket, InterprocessConnection,
  | InterprocessConnectionServer
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct StreamingSocket {
    host_name:   String,
    port_number: Atomic<i32>, // default = 0 
    handle:      Atomic<i32>, // default = -1 
    connected:   AtomicBool, // default = false 
    is_listener: AtomicBool, // default = false 
    read_lock:   RefCell<CriticalSection>,
}

impl StreamingSocket {

    /**
      | True if the socket is currently connected.
      |
      */
    pub fn is_connected(&self) -> bool {
        
        todo!();
        /*
            return connected;
        */
    }

    /**
      | Returns the name of the currently connected
      | host.
      |
      */
    pub fn get_host_name(&self) -> &String {
        
        todo!();
        /*
            return hostName;
        */
    }

    /**
      | Returns the port number that's currently
      | open.
      |
      */
    pub fn get_port(&self) -> i32 {
        
        todo!();
        /*
            return portNumber;
        */
    }

    /**
      | Returns the OS's socket handle that's
      | currently open.
      |
      */
    pub fn get_raw_socket_handle(&self) -> i32 {
        
        todo!();
        /*
            return handle;
        */
    }
}

///-------------------------
impl Drop for StreamingSocket {
    fn drop(&mut self) {
        todo!();
        /* 
        close();
 */
    }
}

impl Default for StreamingSocket {
    
    /**
      | Creates an uninitialised socket.
      | 
      | To connect it, use the connect() method,
      | after which you can read() or write()
      | to it.
      | 
      | To wait for other sockets to connect
      | to this one, the createListener() method
      | enters "listener" mode, and can be used
      | to spawn new sockets for each connection
      | that comes along.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            SocketHelpers::initSockets();
        */
    }
}

impl StreamingSocket {
    
    pub fn new(
        host:     &String,
        port_num: i32,
        h:        i32) -> Self {
    
        todo!();
        /*


            : hostName (host),
          portNumber (portNum),
          handle (h),
          connected (true)
        jassert (SocketHelpers::isValidPortNumber (portNum));

        SocketHelpers::initSockets();
        SocketHelpers::resetSocketOptions ((SocketHandle) h, false, false);
        */
    }

    /**
      | Reads bytes from the socket.
      | 
      | If blockUntilSpecifiedAmountHasArrived
      | is true, the method will block until
      | maxBytesToRead bytes have been read,
      | (or until an error occurs). If this flag
      | is false, the method will return as much
      | data as is currently available without
      | blocking.
      | 
      | -----------
      | @return
      | 
      | the number of bytes read, or -1 if there
      | was an error @see waitUntilReady
      |
      */
    pub fn read(&mut self, 
        dest_buffer:       *mut c_void,
        max_bytes_to_read: i32,
        should_block:      bool) -> i32 {
        
        todo!();
        /*
            return (connected && ! isListener) ? SocketHelpers::readSocket ((SocketHandle) handle.load(), destBuffer,maxBytesToRead,
                                                                        connected, shouldBlock, readLock)
                                           : -1;
        */
    }

    /**
      | Writes bytes to the socket from a buffer.
      | 
      | Note that this method will block unless
      | you have checked the socket is ready
      | for writing before calling it (see the
      | waitUntilReady() method).
      | 
      | -----------
      | @return
      | 
      | the number of bytes written, or -1 if
      | there was an error
      |
      */
    pub fn write(&mut self, 
        source_buffer:      *const c_void,
        num_bytes_to_write: i32) -> i32 {
        
        todo!();
        /*
            if (isListener || ! connected)
            return -1;

        return (int) ::send ((SocketHandle) handle.load(), (const char*) sourceBuffer, (aloe_recvsend_size_t) numBytesToWrite, 0);
        */
    }

    /**
      | Waits until the socket is ready for reading
      | or writing.
      | 
      | If readyForReading is true, it will
      | wait until the socket is ready for reading;
      | if false, it will wait until it's ready
      | for writing.
      | 
      | If the timeout is < 0, it will wait forever,
      | or else will give up after the specified
      | time.
      | 
      | -----------
      | @return
      | 
      | 1 if the socket is ready on return, 0 if
      | it times-out before the socket becomes
      | ready, or -1 if an error occurs
      |
      */
    pub fn wait_until_ready(&mut self, 
        ready_for_reading: bool,
        timeout_msecs:     i32) -> i32 {
        
        todo!();
        /*
            return connected ? SocketHelpers::waitForReadiness (handle, readLock, readyForReading, timeoutMsecs)
                         : -1;
        */
    }

    /**
      | Binds the socket to the specified local
      | port.
      | 
      | -----------
      | @return
      | 
      | true on success; false may indicate
      | that another socket is already bound
      | on the same port
      |
      */
    pub fn bind_to_port(&mut self, port: i32) -> bool {
        
        todo!();
        /*
            return bindToPort (port, String());
        */
    }

    /**
      | Binds the socket to the specified local
      | port and local address.
      | 
      | If localAddress is not an empty string
      | then the socket will be bound to localAddress
      | as well. This is useful if you would like
      | to bind your socket to a specific network
      | adapter. Note that localAddress must
      | be an IP address assigned to one of your
      | network address otherwise this function
      | will fail.
      | 
      | 
      | -----------
      | @return
      | 
      | true on success; false may indicate
      | that another socket is already bound
      | on the same port @see bindToPort(int
      | localPortNumber), IPAddress::getAllAddresses
      |
      */
    pub fn bind_to_port_with_addr(
        &mut self, 
        port: i32,
        addr: &String) -> bool {
        
        todo!();
        /*
            jassert (SocketHelpers::isValidPortNumber (port));

        return SocketHelpers::bindSocket ((SocketHandle) handle.load(), port, addr);
        */
    }

    /**
      | Returns the local port number to which
      | this socket is currently bound.
      | 
      | This is useful if you need to know to which
      | port the OS has actually bound your socket
      | when calling the constructor or bindToPort
      | with zero as the localPortNumber argument.
      | 
      | 
      | -----------
      | @return
      | 
      | -1 if the function fails
      |
      */
    pub fn get_bound_port(&self) -> i32 {
        
        todo!();
        /*
            return SocketHelpers::getBoundPort ((SocketHandle) handle.load());
        */
    }

    /**
      | Tries to connect the socket to hostname:port.
      | 
      | If timeOutMillisecs is 0, then this
      | method will block until the operating
      | system rejects the connection (which
      | could take a long time).
      | 
      | 
      | -----------
      | @return
      | 
      | true if it succeeds, false if otherwise
      | @see isConnected
      |
      */
    pub fn connect(
        &mut self, 
        remote_host_name:   &String,
        remote_port_number: i32,
        time_out_millisecs: Option<i32>

    ) -> bool {

        let time_out_millisecs = time_out_millisecs.unwrap_or(3000);
        
        todo!();
        /*
            jassert (SocketHelpers::isValidPortNumber (remotePortNumber));

        if (isListener)
        {
            // a listener socket can't connect to another one!
            jassertfalse;
            return false;
        }

        if (connected)
            close();

        hostName = remoteHostName;
        portNumber = remotePortNumber;
        isListener = false;

        connected = SocketHelpers::connectSocket (handle, readLock, remoteHostName,
                                                  remotePortNumber, timeOutMillisecs);

        if (! connected)
            return false;

        if (! SocketHelpers::resetSocketOptions ((SocketHandle) handle.load(), false, false))
        {
            close();
            return false;
        }

        return true;
        */
    }

    /**
      | Closes the connection.
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (handle >= 0)
            SocketHelpers::closeSocket (handle, readLock, isListener, portNumber, connected);

        hostName.clear();
        portNumber = 0;
        handle = -1;
        isListener = false;
        */
    }

    /**
      | Puts this socket into "listener" mode.
      | 
      | When in this mode, your thread can call
      | waitForNextConnection() repeatedly,
      | which will spawn new sockets for each
      | new connection, so that these can be
      | handled in parallel by other threads.
      | 
      | -----------
      | @param portNumber
      | 
      | the port number to listen on
      | ----------
      | @param localHostName
      | 
      | the interface address to listen on -
      | pass an empty string to listen on all
      | addresses
      | 
      | -----------
      | @return
      | 
      | true if it manages to open the socket
      | successfully @see waitForNextConnection
      |
      */
    pub fn create_listener(
        &mut self, 
        new_port_number: i32,
        local_host_name: Option<&String>

    ) -> bool {

        let local_host_name: &String = local_host_name.unwrap_or(&String::from(""));
        
        todo!();
        /*
            jassert (SocketHelpers::isValidPortNumber (newPortNumber));

        if (connected)
            close();

        hostName = "listener";
        portNumber = newPortNumber;
        isListener = true;

        handle = (int) socket (AF_INET, SOCK_STREAM, 0);

        if (handle < 0)
            return false;

       #if ! ALOE_WINDOWS // on windows, adding this option produces behaviour different to posix
        SocketHelpers::makeReusable (handle);
       #endif

        if (SocketHelpers::bindSocket ((SocketHandle) handle.load(), portNumber, localHostName)
             && listen ((SocketHandle) handle.load(), SOMAXCONN) >= 0)
        {
            connected = true;
            return true;
        }

        close();
        return false;
        */
    }

    /**
      | When in "listener" mode, this waits
      | for a connection and spawns it as a new
      | socket.
      | 
      | The object that gets returned will be
      | owned by the caller.
      | 
      | This method can only be called after
      | using createListener().
      | 
      | @see createListener
      |
      */
    pub fn wait_for_next_connection(&self) -> *mut StreamingSocket {
        
        todo!();
        /*
            // To call this method, you first have to use createListener() to
        // prepare this socket as a listener.
        jassert (isListener || ! connected);

        if (connected && isListener)
        {
            struct sockaddr_storage address;
            aloe_socklen_t len = sizeof (address);
            auto newSocket = (int) accept ((SocketHandle) handle.load(), (struct sockaddr*) &address, &len);

            if (newSocket >= 0 && connected)
                return new StreamingSocket (inet_ntoa (((struct sockaddr_in*) &address)->sin_addr),
                                            portNumber, newSocket);
        }

        return nullptr;
        */
    }

    /**
      | True if the socket is connected to this
      | machine rather than over the network.
      |
      */
    pub fn is_local(&self) -> bool {
        
        todo!();
        /*
            if (! isConnected())
            return false;

        IPAddress currentIP (SocketHelpers::getConnectedAddress ((SocketHandle) handle.load()));

        for (auto& a : IPAddress::getAllAddresses())
            if (a == currentIP)
                return true;

        return hostName == "127.0.0.1";
        */
    }
}
