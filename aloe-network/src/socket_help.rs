crate::ix!();

#[cfg(not(ALOE_WASM))]
#[cfg(not(AI_NUMERICSERV))]
pub const AI_NUMERICSERV: usize = 0x1000;

#[cfg(not(ALOE_WASM))] #[cfg(target_os="windows")] pub type aloe_socklen_t       = i32;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="windows")] pub type aloe_recvsend_size_t = i32;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="windows")] pub type SocketHandle         = SOCKET;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="windows")] pub const invalidSocket: SocketHandle = INVALID_SOCKET;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="android")] pub type aloe_socklen_t       = libc::socklen_t;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="android")] pub type aloe_recvsend_size_t = usize;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="android")] pub type SocketHandle         = i32;
#[cfg(not(ALOE_WASM))] #[cfg(target_os="android")] pub const invalidSocket: SocketHandle = -1;

#[cfg(not(ALOE_WASM))] #[cfg(not(any(target_os="windows",target_os="android")))] pub type aloe_socklen_t       = libc::socklen_t;
#[cfg(not(ALOE_WASM))] #[cfg(not(any(target_os="windows",target_os="android")))] pub type aloe_recvsend_size_t = libc::socklen_t;
#[cfg(not(ALOE_WASM))] #[cfg(not(any(target_os="windows",target_os="android")))] pub type SocketHandle         = i32;
#[cfg(not(ALOE_WASM))] #[cfg(not(any(target_os="windows",target_os="android")))] pub const invalidSocket: SocketHandle = -1;

pub mod socket_helpers {
    use super::*;

    #[cfg(not(ALOE_WASM))]
    pub fn init_sockets()  {
        
        todo!();
        /*
            #if ALOE_WINDOWS
                static bool socketsStarted = false;

                if (! socketsStarted)
                {
                    WSADATA wsaData;
                    const WORD wVersionRequested = MAKEWORD (1, 1);
                    socketsStarted = WSAStartup (wVersionRequested, &wsaData) == 0;
                }
               #endif
        */
    }

    #[cfg(not(ALOE_WASM))]
    #[inline] pub fn is_valid_port_number(port: i32) -> bool {
        
        todo!();
        /*
            return isPositiveAndBelow (port, 65536);
        */
    }

    #[cfg(not(ALOE_WASM))]
    pub fn set_option_with_value<Type>(
            handle:   SocketHandle,
            mode:     i32,
            property: i32,
            value:    Type) -> bool {

        todo!();
        /*
            return setsockopt (handle, mode, property, reinterpret_cast<const char*> (&value), sizeof (value)) == 0;
        */
    }


    #[cfg(not(ALOE_WASM))]
    pub fn set_option<Type>(
            handle:   SocketHandle,
            property: i32,
            value:    Type) -> bool {

        todo!();
        /*
            return setOption (handle, SOL_SOCKET, property, value);
        */
    }


    #[cfg(not(ALOE_WASM))]
    pub fn reset_socket_options(
            handle:          SocketHandle,
            is_datagram:     bool,
            allow_broadcast: bool) -> bool {
        
        todo!();
        /*
            return handle != invalidSocket
                        && setOption (handle, SO_RCVBUF, (int) 65536)
                        && setOption (handle, SO_SNDBUF, (int) 65536)
                        && (isDatagram ? ((! allowBroadcast) || setOption (handle, SO_BROADCAST, (int) 1))
                                       : setOption (handle, IPPROTO_TCP, TCP_NODELAY, (int) 1));
        */
    }

    #[cfg(not(ALOE_WASM))]
    pub fn close_socket(
            handle:      &mut Atomic<i32>,
            read_lock:   &mut CriticalSection,
            is_listener: bool,
            port_number: i32,
            connected:   &mut AtomicBool)  {
        
        todo!();
        /*
            const auto h = (SocketHandle) handle.load();
                handle = -1;

               #if ALOE_WINDOWS
                ignoreUnused (portNumber, isListener, readLock);

                if (h != invalidSocket || connected)
                    closesocket (h);

                // make sure any read process finishes before we delete the socket
                CriticalSection::ScopedLockType lock (readLock);
                connected = false;
               #else
                if (connected)
                {
                    connected = false;

                    if (isListener)
                    {
                        // need to do this to interrupt the accept() function..
                        StreamingSocket temp;
                        temp.connect (IPAddress::local().toString(), portNumber, 1000);
                    }
                }

                if (h >= 0)
                {
                    // unblock any pending read requests
                    ::shutdown (h, SHUT_RDWR);

                    {
                        // see man-page of recv on linux about a race condition where the
                        // shutdown command is lost if the receiving thread does not have
                        // a chance to process before close is called. On Mac OS X shutdown
                        // does not unblock a select call, so using a lock here will dead-lock
                        // both threads.
                       #if ALOE_LINUX || ALOE_BSD || ALOE_ANDROID
                        CriticalSection::ScopedLockType lock (readLock);
                        ::close (h);
                       #else
                        ::close (h);
                        CriticalSection::ScopedLockType lock (readLock);
                      #endif
                    }
                }
               #endif
        */
    }

    #[cfg(not(ALOE_WASM))]
    pub fn bind_socket(
            handle:  SocketHandle,
            port:    i32,
            address: &String) -> bool {
        
        todo!();
        /*
            if (handle == invalidSocket || ! isValidPortNumber (port))
                    return false;

                struct sockaddr_in addr;
                zerostruct (addr); // (can't use "= { 0 }" on this object because it's typedef'ed as a C struct)

                addr.sin_family = PF_INET;
                addr.sin_port = htons ((uint16) port);
                addr.sin_addr.s_addr = address.isNotEmpty() ? ::inet_addr (address.toRawUTF8())
                                                            : htonl (INADDR_ANY);

                return ::bind (handle, (struct sockaddr*) &addr, sizeof (addr)) >= 0;
        */
    }

    #[cfg(not(ALOE_WASM))]
    pub fn get_bound_port(handle: SocketHandle) -> i32 {
        
        todo!();
        /*
            if (handle != invalidSocket)
                {
                    struct sockaddr_in addr;
                    socklen_t len = sizeof (addr);

                    if (getsockname (handle, (struct sockaddr*) &addr, &len) == 0)
                        return ntohs (addr.sin_port);
                }

                return -1;
        */
    }

    #[cfg(not(ALOE_WASM))]
    pub fn get_connected_address(handle: SocketHandle) -> String {
        
        todo!();
        /*
            struct sockaddr_in addr;
                socklen_t len = sizeof (addr);

                if (getpeername (handle, (struct sockaddr*) &addr, &len) >= 0)
                    return inet_ntoa (addr.sin_addr);

                return "0.0.0.0";
        */
    }

    #[cfg(not(ALOE_WASM))]
    pub fn set_socket_blocking_state(
            handle:       SocketHandle,
            should_block: bool) -> bool {
        
        todo!();
        /*
            #if ALOE_WINDOWS
                u_long nonBlocking = shouldBlock ? 0 : (u_long) 1;
                return ioctlsocket (handle, (long) FIONBIO, &nonBlocking) == 0;
               #else
                int socketFlags = fcntl (handle, F_GETFL, 0);

                if (socketFlags == -1)
                    return false;

                if (shouldBlock)
                    socketFlags &= ~O_NONBLOCK;
                else
                    socketFlags |= O_NONBLOCK;

                return fcntl (handle, F_SETFL, socketFlags) == 0;
               #endif
        */
    }

    #[cfg(not(ALOE_WASM))]
    #[cfg(not(target_os="windows"))]
    pub fn get_socket_blocking_state(handle: SocketHandle) -> bool {
        
        todo!();
        /*
            return (fcntl (handle, F_GETFL, 0) & O_NONBLOCK) == 0;
        */
    }

    pub fn read_socket(
        handle:                                   SocketHandle,
        dest_buffer:                              *mut c_void,
        max_bytes_to_read:                        i32,
        connected:                                &mut AtomicBool,
        block_until_specified_amount_has_arrived: bool,
        read_lock:                                &mut CriticalSection,
        senderip:                                 *mut String,
        sender_port:                              *mut i32

    ) -> i32 {

        todo!();
        /*
            #if ! ALOE_WINDOWS
                if (blockUntilSpecifiedAmountHasArrived != getSocketBlockingState (handle))
               #endif
                    setSocketBlockingState (handle, blockUntilSpecifiedAmountHasArrived);

                int bytesRead = 0;

                while (bytesRead < maxBytesToRead)
                {
                    long bytesThisTime = -1;
                    auto buffer = static_cast<char*> (destBuffer) + bytesRead;
                    auto numToRead = (aloe_recvsend_size_t) (maxBytesToRead - bytesRead);

                    {
                        // avoid race-condition
                        CriticalSection::ScopedTryLockType lock (readLock);

                        if (lock.isLocked())
                        {
                            if (senderIP == nullptr || senderPort == nullptr)
                            {
                                bytesThisTime = ::recv (handle, buffer, numToRead, 0);
                            }
                            else
                            {
                                sockaddr_in client;
                                socklen_t clientLen = sizeof (sockaddr);

                                bytesThisTime = ::recvfrom (handle, buffer, numToRead, 0, (sockaddr*) &client, &clientLen);

                                *senderIP = String::fromUTF8 (inet_ntoa (client.sin_addr), 16);
                                *senderPort = ntohs (client.sin_port);
                            }
                        }
                    }

                    if (bytesThisTime <= 0 || ! connected)
                    {
                        if (bytesRead == 0 && blockUntilSpecifiedAmountHasArrived)
                            bytesRead = -1;

                        break;
                    }

                    bytesRead = static_cast<int> (bytesRead + bytesThisTime);

                    if (! blockUntilSpecifiedAmountHasArrived)
                        break;
                }

                return (int) bytesRead;
        */
    }

    pub fn wait_for_readiness(
            handle:        &mut Atomic<i32>,
            read_lock:     &mut CriticalSection,
            for_reading:   bool,
            timeout_msecs: i32) -> i32 {
        
        todo!();
        /*
            // avoid race-condition
                CriticalSection::ScopedTryLockType lock (readLock);

                if (! lock.isLocked())
                    return -1;

                auto hasErrorOccurred = [&handle]() -> bool
                {
                    auto h = (SocketHandle) handle.load();

                    if (h == invalidSocket)
                        return true;

                    int opt;
                    aloe_socklen_t len = sizeof (opt);

                    if (getsockopt (h, SOL_SOCKET, SO_ERROR, (char*) &opt, &len) < 0  || opt != 0)
                        return true;

                    return false;
                };

                auto h = handle.load();

               #if ALOE_WINDOWS || ALOE_MINGW
                struct timeval timeout;
                struct timeval* timeoutp;

                if (timeoutMsecs >= 0)
                {
                    timeout.tv_sec = timeoutMsecs / 1000;
                    timeout.tv_usec = (timeoutMsecs % 1000) * 1000;
                    timeoutp = &timeout;
                }
                else
                {
                    timeoutp = nullptr;
                }

                fd_set rset, wset;
                FD_ZERO (&rset);
                FD_SET ((SOCKET) h, &rset);
                FD_ZERO (&wset);
                FD_SET ((SOCKET) h, &wset);

                fd_set* prset = forReading ? &rset : nullptr;
                fd_set* pwset = forReading ? nullptr : &wset;

                // NB - need to use select() here as WSAPoll is broken on Windows
                if (select ((int) h + 1, prset, pwset, nullptr, timeoutp) < 0 || hasErrorOccurred())
                    return -1;

                return FD_ISSET (h, forReading ? &rset : &wset) ? 1 : 0;
              #else
                short eventsFlag = (forReading ? POLLIN : POLLOUT);
                pollfd pfd { (SocketHandle) h, eventsFlag, 0 };

                int result = 0;

                for (;;)
                {
                    result = poll (&pfd, 1, timeoutMsecs);

                    if (result >= 0 || errno != EINTR)
                        break;
                }

                if (result < 0 || hasErrorOccurred())
                    return -1;

                return (pfd.revents & eventsFlag) != 0;
              #endif
        */
    }

    pub fn get_address_info(
            is_datagram: bool,
            host_name:   &String,
            port_number: i32) -> *mut addrinfo {
        
        todo!();
        /*
            struct addrinfo hints;
                zerostruct (hints);

                hints.ai_family = AF_UNSPEC;
                hints.ai_socktype = isDatagram ? SOCK_DGRAM : SOCK_STREAM;
                hints.ai_flags = AI_NUMERICSERV;

                struct addrinfo* info = nullptr;

                if (getaddrinfo (hostName.toRawUTF8(), String (portNumber).toRawUTF8(), &hints, &info) == 0)
                    return info;

                return nullptr;
        */
    }

    pub fn connect_socket(
            handle:             &mut Atomic<i32>,
            read_lock:          &mut CriticalSection,
            host_name:          &String,
            port_number:        i32,
            time_out_millisecs: i32) -> bool {
        
        todo!();
        /*
            bool success = false;

                if (auto* info = getAddressInfo (false, hostName, portNumber))
                {
                    for (auto* i = info; i != nullptr; i = i->ai_next)
                    {
                        auto newHandle = socket (i->ai_family, i->ai_socktype, 0);

                        if (newHandle != invalidSocket)
                        {
                            setSocketBlockingState (newHandle, false);
                            auto result = ::connect (newHandle, i->ai_addr, (socklen_t) i->ai_addrlen);
                            success = (result >= 0);

                            if (! success)
                            {
                               #if ALOE_WINDOWS
                                if (result == SOCKET_ERROR && WSAGetLastError() == WSAEWOULDBLOCK)
                               #else
                                if (errno == EINPROGRESS)
                               #endif
                                {
                                    std::atomic<int> cvHandle { (int) newHandle };

                                    if (waitForReadiness (cvHandle, readLock, false, timeOutMillisecs) == 1)
                                        success = true;
                                }
                            }

                            if (success)
                            {
                                handle = (int) newHandle;
                                break;
                            }

                           #if ALOE_WINDOWS
                            closesocket (newHandle);
                           #else
                            ::close (newHandle);
                           #endif
                        }
                    }

                    freeaddrinfo (info);

                    if (success)
                    {
                        auto h = (SocketHandle) handle.load();
                        setSocketBlockingState (h, true);
                        resetSocketOptions (h, false, false);
                    }
                }

                return success;
        */
    }

    pub fn make_reusable(handle: i32)  {
        
        todo!();
        /*
            setOption ((SocketHandle) handle, SO_REUSEADDR, (int) 1);
        */
    }

    pub fn multicast(
            handle:               i32,
            multicast_ip_address: &String,
            interface_ip_address: &String,
            join:                 bool) -> bool {
        
        todo!();
        /*
            struct ip_mreq mreq;

                zerostruct (mreq);
                mreq.imr_multiaddr.s_addr = inet_addr (multicastIPAddress.toRawUTF8());
                mreq.imr_interface.s_addr = INADDR_ANY;

                if (interfaceIPAddress.isNotEmpty())
                    mreq.imr_interface.s_addr = inet_addr (interfaceIPAddress.toRawUTF8());

                return setsockopt ((SocketHandle) handle, IPPROTO_IP,
                                   join ? IP_ADD_MEMBERSHIP
                                        : IP_DROP_MEMBERSHIP,
                                   (const char*) &mreq, sizeof (mreq)) == 0;
        */
    }
}

