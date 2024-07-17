crate::ix!();

/**
  | A wrapper for a datagram (UDP) socket.
  | 
  | This allows low-level use of sockets;
  | for an easier-to-use messaging layer
  | on top of sockets, you could also try
  | the InterprocessConnection class.
  | 
  | @see StreamingSocket, InterprocessConnection,
  | InterprocessConnectionServer
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DatagramSocket {
    handle:              Atomic<i32>, // default = -1 
    is_bound:            bool, // default = false
    last_bind_address:   String,
    last_server_host:    String,
    last_server_port:    i32, // default = -1
    last_server_address: *mut c_void, // default = nullptr
    read_lock:           RefCell<CriticalSection>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_Socket.cpp]
///---------------------------------
impl Drop for DatagramSocket {

    fn drop(&mut self) {
        todo!();
        /* 
        if (lastServerAddress != nullptr)
            freeaddrinfo (static_cast<struct addrinfo*> (lastServerAddress));

        shutdown();
         */
    }
}

impl DatagramSocket {

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

    /**
      | Creates a datagram socket.
      | 
      | You first need to bind this socket to
      | a port with bindToPort if you intend
      | to read from this socket.
      | 
      | If enableBroadcasting is true, the
      | socket will be allowed to send broadcast
      | messages (may require extra privileges
      | on linux)
      |
      */
    pub fn new(can_broadcast: Option<bool>) -> Self {

        let can_broadcast: bool =
            can_broadcast.unwrap_or(false);
    
        todo!();
        /*


            SocketHelpers::initSockets();

        handle = (int) socket (AF_INET, SOCK_DGRAM, 0);

        if (handle >= 0)
        {
            SocketHelpers::resetSocketOptions ((SocketHandle) handle.load(), true, canBroadcast);
            SocketHelpers::makeReusable (handle);
        }
        */
    }

    /**
      | Closes the underlying socket object.
      | 
      | Closes the underlying socket object
      | and aborts any read or write operations.
      | Note that all other methods will return
      | an error after this call and the object
      | cannot be re-used.
      | 
      | This method is useful if another thread
      | is blocking in a read/write call and
      | you would like to abort the read/write
      | thread. Simply deleting the socket
      | object without calling shutdown may
      | cause a race-condition where the read/write
      | returns just before the socket is deleted
      | and the subsequent read/write would
      | try to read from an invalid pointer.
      | By calling shutdown first, the socket
      | object remains valid but all current
      | and subsequent calls to read/write
      | will return immediately.
      |
      */
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            if (handle < 0)
            return;

        std::atomic<int> handleCopy { handle.load() };
        handle = -1;

        std::atomic<bool> connected { false };
        SocketHelpers::closeSocket (handleCopy, readLock, false, 0, connected);

        isBound = false;
        */
    }

    /**
      | Binds the socket to the specified local
      | port.
      | 
      | The localPortNumber is the port on which
      | to bind this socket. If this value is
      | 0, the port number is assigned by the
      | operating system.
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

        if (handle < 0)
            return false;

        if (SocketHelpers::bindSocket ((SocketHandle) handle.load(), port, addr))
        {
            isBound = true;
            lastBindAddress = addr;
            return true;
        }

        return false;
        */
    }

    /**
      | Returns the local port number to which
      | this socket is currently bound.
      | 
      | This is useful if you need to know to which
      | port the OS has actually bound your socket
      | when bindToPort was called with zero.
      | 
      | -----------
      | @return
      | 
      | -1 if the socket didn't bind to any port
      | yet or an error occurred
      |
      */
    pub fn get_bound_port(&self) -> i32 {
        
        todo!();
        /*
            return (handle >= 0 && isBound) ? SocketHelpers::getBoundPort ((SocketHandle) handle.load()) : -1;
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
            if (handle < 0)
            return -1;

        return SocketHelpers::waitForReadiness (handle, readLock, readyForReading, timeoutMsecs);
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
            if (handle < 0 || ! isBound)
            return -1;

        std::atomic<bool> connected { true };
        return SocketHelpers::readSocket ((SocketHandle) handle.load(), destBuffer, maxBytesToRead,
                                          connected, shouldBlock, readLock);
        */
    }

    /**
      | Reads bytes from the socket and return
      | the IP address of the sender.
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
      | was an error. On a successful result,
      | the senderIPAddress value will be set
      | to the IP of the sender @see waitUntilReady
      |
      */
    pub fn read_with_sender_info(
        &mut self, 
        dest_buffer:       *mut c_void,
        max_bytes_to_read: i32,
        should_block:      bool,
        sender_ip_address: &mut String,
        sender_port:       &mut i32) -> i32 {
        
        todo!();
        /*
            if (handle < 0 || ! isBound)
            return -1;

        std::atomic<bool> connected { true };
        return SocketHelpers::readSocket ((SocketHandle) handle.load(), destBuffer, maxBytesToRead, connected,
                                          shouldBlock, readLock, &senderIPAddress, &senderPort);
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
        remote_hostname:    &String,
        remote_port_number: i32,
        source_buffer:      *const c_void,
        num_bytes_to_write: i32) -> i32 {
        
        todo!();
        /*
            jassert (SocketHelpers::isValidPortNumber (remotePortNumber));

        if (handle < 0)
            return -1;

        struct addrinfo*& info = reinterpret_cast<struct addrinfo*&> (lastServerAddress);

        // getaddrinfo can be quite slow so cache the result of the address lookup
        if (info == nullptr || remoteHostname != lastServerHost || remotePortNumber != lastServerPort)
        {
            if (info != nullptr)
                freeaddrinfo (info);

            if ((info = SocketHelpers::getAddressInfo (true, remoteHostname, remotePortNumber)) == nullptr)
                return -1;

            lastServerHost = remoteHostname;
            lastServerPort = remotePortNumber;
        }

        return (int) ::sendto ((SocketHandle) handle.load(), (const char*) sourceBuffer,
                               (aloe_recvsend_size_t) numBytesToWrite, 0,
                               info->ai_addr, (socklen_t) info->ai_addrlen);
        */
    }

    /**
      | Join a multicast group.
      | 
      | -----------
      | @return
      | 
      | true if it succeeds
      |
      */
    pub fn join_multicast(&mut self, multicast_ip_address: &String) -> bool {
        
        todo!();
        /*
            if (handle < 0 || ! isBound)
            return false;

        return SocketHelpers::multicast (handle, multicastIPAddress, lastBindAddress, true);
        */
    }

    /**
      | Leave a multicast group.
      | 
      | -----------
      | @return
      | 
      | true if it succeeds
      |
      */
    pub fn leave_multicast(&mut self, multicast_ip_address: &String) -> bool {
        
        todo!();
        /*
            if (handle < 0 || ! isBound)
            return false;

        return SocketHelpers::multicast (handle, multicastIPAddress, lastBindAddress, false);
        */
    }

    /**
      | Enables or disables multicast loopback.
      | 
      | -----------
      | @return
      | 
      | true if it succeeds
      |
      */
    pub fn set_multicast_loopback_enabled(&mut self, enable: bool) -> bool {
        
        todo!();
        /*
            if (handle < 0 || ! isBound)
            return false;

        return SocketHelpers::setOption<bool> ((SocketHandle) handle.load(), IPPROTO_IP, IP_MULTICAST_LOOP, enable);
        */
    }

    /**
      | Allow other applications to re-use
      | the port.
      | 
      | Allow any other application currently
      | running to bind to the same port. Do not
      | use this if your socket handles sensitive
      | data as it could be read by any, possibly
      | malicious, third-party apps.
      | 
      | -----------
      | @return
      | 
      | true on success
      |
      */
    pub fn set_enable_port_reuse(&mut self, enabled: bool) -> bool {
        
        todo!();
        /*
            #if ALOE_ANDROID
        ignoreUnused (enabled);
       #else
        if (handle >= 0)
            return SocketHelpers::setOption ((SocketHandle) handle.load(),
                                            #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
                                             SO_REUSEADDR,  // port re-use is implied by addr re-use on these platforms
                                            #else
                                             SO_REUSEPORT,
                                            #endif
                                             (int) (enabled ? 1 : 0));
       #endif

        return false;
        */
    }
}
