crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCSender.h]

/**
  | An OSC message sender.
  | 
  | An OSCSender object can connect to a
  | network port. It then can send OSC messages
  | and bundles to a specified host over
  | an UDP socket.
  | 
  | @tags{OSC}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OSCSender {
    impl_: Box<OSCSenderImpl>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCSender.cpp]
impl Default for OSCSender {
    
    fn default() -> Self {
        todo!();
        /*


            : impl (new OSCSenderImpl())
        */
    }
}

impl Drop for OSCSender {

    fn drop(&mut self) {
        todo!();
        /*
            impl->disconnect();
        impl.reset();
        */
    }
}

impl OSCSender {
    
    /**
      | Creates a new OSC message with the specified
      | address pattern and list of arguments,
      | and sends it to the target.
      | 
      | -----------
      | @param address
      | 
      | The OSC address pattern of the message
      | (you can use a string literal here).
      | ----------
      | @param args
      | 
      | The list of arguments for the message.
      |
      */
    pub fn send<Args>(&mut self, 
        address: &OSCAddressPattern,
        args:    Args) -> bool {
    
        todo!();
        /*
            return send (OSCMessage (address, std::forward<Args> (args)...));
        */
    }
    
    /**
      | Creates a new OSC message with the specified
      | address pattern and list of arguments,
      | and sends it to the target.
      | 
      | -----------
      | @param targetIPAddress
      | 
      | The IP address to send to
      | ----------
      | @param targetPortNumber
      | 
      | The target port number
      | ----------
      | @param address
      | 
      | The OSC address pattern of the message
      | (you can use a string literal here).
      | ----------
      | @param args
      | 
      | The list of arguments for the message.
      |
      */
    pub fn send_to_ip_address_with_pattern<Args>(
        &mut self, 
        target_ip_address:  &String,
        target_port_number: i32,
        address:            &OSCAddressPattern,
        args:               Args

    ) -> bool {
    
        todo!();
        /*
            return sendToIPAddress (targetIPAddress, targetPortNumber, OSCMessage (address, std::forward<Args> (args)...));
        */
    }
    
    /**
      | Connects to a datagram socket and prepares
      | the socket for sending OSC packets to
      | the specified target.
      | 
      | -----------
      | @note
      | 
      | The operating system will choose which
      | specific network adapter(s) to bind
      | your socket to, and which local port
      | to use for the sender.
      | 
      | -----------
      | @param targetHostName
      | 
      | The remote host to which messages will
      | be send.
      | ----------
      | @param targetPortNumber
      | 
      | The remote UDP port number on which the
      | host will receive the messages.
      | 
      | -----------
      | @return
      | 
      | true if the connection was successful;
      | false otherwise. @see send, disconnect.
      |
      */
    pub fn connect(&mut self, 
        target_host_name:   &String,
        target_port_number: i32) -> bool {
        
        todo!();
        /*
            return impl->connect (targetHostName, targetPortNumber);
        */
    }
    
    /**
      | Uses an existing datagram socket for
      | sending OSC packets to the specified
      | target.
      | 
      | -----------
      | @param socket
      | 
      | An existing datagram socket. Make sure
      | this doesn't get deleted while this
      | class is still using it!
      | ----------
      | @param targetHostName
      | 
      | The remote host to which messages will
      | be send.
      | ----------
      | @param targetPortNumber
      | 
      | The remote UDP port number on which the
      | host will receive the messages.
      | 
      | -----------
      | @return
      | 
      | true if the connection was successful;
      | false otherwise. @see connect, send,
      | disconnect.
      |
      */
    pub fn connect_to_socket(&mut self, 
        socket:             &mut DatagramSocket,
        target_host_name:   &String,
        target_port_number: i32) -> bool {
        
        todo!();
        /*
            return impl->connectToSocket (socket, targetHostName, targetPortNumber);
        */
    }
    
    /**
      | Disconnects from the currently used
      | UDP port.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the disconnection was successful;
      | false otherwise. @see connect.
      |
      */
    pub fn disconnect(&mut self) -> bool {
        
        todo!();
        /*
            return impl->disconnect();
        */
    }
    
    /**
      | Sends an OSC message to the target.
      | 
      | -----------
      | @param message
      | 
      | The OSC message to send.
      | 
      | -----------
      | @return
      | 
      | true if the operation was successful.
      |
      */
    pub fn send_osc_message(&mut self, message: &OSCMessage) -> bool {
        
        todo!();
        /*
            return impl->send (message);
        */
    }
    
    /**
      | Send an OSC bundle to the target.
      | 
      | -----------
      | @param bundle
      | 
      | The OSC bundle to send.
      | 
      | -----------
      | @return
      | 
      | true if the operation was successful.
      |
      */
    pub fn send_bundle(&mut self, bundle: &OSCBundle) -> bool {
        
        todo!();
        /*
            return impl->send (bundle);
        */
    }
    
    /**
      | Sends an OSC message to a specific IP
      | address and port.
      | 
      | This overrides the address and port
      | that was originally set for this sender.
      | 
      | -----------
      | @param targetIPAddress
      | 
      | The IP address to send to
      | ----------
      | @param targetPortNumber
      | 
      | The target port number
      | ----------
      | @param message
      | 
      | The OSC message to send.
      | 
      | -----------
      | @return
      | 
      | true if the operation was successful.
      |
      */
    pub fn send_to_ip_address(&mut self, 
        host:    &String,
        port:    i32,
        message: &OSCMessage) -> bool {
        
        todo!();
        /*
            return impl->send (message, host, port);
        */
    }
    
    /**
      | Sends an OSC bundle to a specific IP address
      | and port.
      | 
      | This overrides the address and port
      | that was originally set for this sender.
      | 
      | -----------
      | @param targetIPAddress
      | 
      | The IP address to send to
      | ----------
      | @param targetPortNumber
      | 
      | The target port number
      | ----------
      | @param bundle
      | 
      | The OSC bundle to send.
      | 
      | -----------
      | @return
      | 
      | true if the operation was successful.
      |
      */
    pub fn send_bundle_to_ip_address(&mut self, 
        host:   &String,
        port:   i32,
        bundle: &OSCBundle) -> bool {
        
        todo!();
        /*
            return impl->send (bundle,  host, port);
        */
    }
}
