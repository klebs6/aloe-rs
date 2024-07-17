crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCReceiver.h]

/**
  | A class for receiving OSC data.
  | 
  | An OSCReceiver object allows you to
  | receive OSC bundles and messages.
  | 
  | It can connect to a network port, receive
  | incoming OSC packets from the network
  | via UDP, parse them, and forward the
  | included OSCMessage and OSCBundle
  | objects to its listeners.
  | 
  | @tags{OSC}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OSCReceiver {
    pimpl: Box<OSCReceiverPimpl>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCReceiver.cpp]
impl Default for OSCReceiver {
    
    fn default() -> Self {
    
        todo!();
        /*
        : osc_receiver("Aloe OSC server"),

        
        */
    }
}

impl Drop for OSCReceiver {

    fn drop(&mut self) {
        todo!();
        /*
            pimpl.reset();
        */
    }
}

impl From<&str> for OSCReceiver {

    /**
      | Creates an OSCReceiver with a specific
      | name for its thread.
      |
      */
    fn from(thread_name: &str) -> Self {
    
        todo!();
        /*


            : pimpl (new OSCReceiverPimpl (threadName))
        */
    }
}

impl OSCReceiver {
    
    /**
      | Connects to the specified UDP port using
      | a datagram socket, and starts listening
      | to OSC packets arriving on this port.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the connection was successful;
      | false otherwise.
      |
      */
    pub fn connect(&mut self, port_number: i32) -> bool {
        
        todo!();
        /*
            return pimpl->connectToPort (portNumber);
        */
    }
    
    /**
      | Connects to a UDP datagram socket that
      | is already set up, and starts listening
      | to OSC packets arriving on this port.
      | 
      | Make sure that the object you give it
      | doesn't get deleted while this object
      | is still using it!
      | 
      | 
      | -----------
      | @return
      | 
      | true if the connection was successful;
      | false otherwise.
      |
      */
    pub fn connect_to_socket(&mut self, socket: &mut DatagramSocket) -> bool {
        
        todo!();
        /*
            return pimpl->connectToSocket (socket);
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
      | false otherwise.
      |
      */
    pub fn disconnect(&mut self) -> bool {
        
        todo!();
        /*
            return pimpl->disconnect();
        */
    }
    
    /**
      | Adds a listener that listens to OSC messages
      | and bundles.
      | 
      | This listener will be called on the application's
      | message loop.
      |
      */
    pub fn add_listener_message_loop_callback(&mut self, listener_to_add: *mut dyn OSCReceiverListener<OSCReceiverMessageLoopCallback>)  {
        
        todo!();
        /*
            pimpl->addListener (listenerToAdd);
        */
    }
    
    /**
      | Adds a listener that listens to OSC messages
      | and bundles.
      | 
      | This listener will be called in real-time
      | directly on the network thread that
      | receives OSC data.
      |
      */
    pub fn add_listener_realtime_callback(&mut self, listener_to_add: *mut dyn OSCReceiverListener<OSCReceiverRealtimeCallback>)  {
        
        todo!();
        /*
            pimpl->addListener (listenerToAdd);
        */
    }
    
    /**
      | Adds a filtered listener that listens
      | to OSC messages matching the address
      | used to register the listener here.
      | 
      | The listener will be called on the application's
      | message loop.
      |
      */
    pub fn add_listener_message_loop_callback_with_address(&mut self, 
        listener_to_add:  *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverMessageLoopCallback>,
        address_to_match: OSCAddress)  {
        
        todo!();
        /*
            pimpl->addListener (listenerToAdd, addressToMatch);
        */
    }
    
    /**
      | Adds a filtered listener that listens
      | to OSC messages matching the address
      | used to register the listener here.
      | 
      | The listener will be called on the application's
      | message loop.
      |
      */
    pub fn add_listener_realtime_callback_with_address(&mut self, 
        listener_to_add:  *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverRealtimeCallback>,
        address_to_match: OSCAddress)  {
        
        todo!();
        /*
            pimpl->addListener (listenerToAdd, addressToMatch);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_message_loop_callback_listener(&mut self, listener_to_remove: *mut dyn OSCReceiverListener<OSCReceiverMessageLoopCallback>)  {
        
        todo!();
        /*
            pimpl->removeListener (listenerToRemove);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_realtime_callback_listener(&mut self, listener_to_remove: *mut dyn OSCReceiverListener<OSCReceiverRealtimeCallback>)  {
        
        todo!();
        /*
            pimpl->removeListener (listenerToRemove);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_message_loop_callback_listener_with_address(&mut self, listener_to_remove: *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverMessageLoopCallback>)  {
        
        todo!();
        /*
            pimpl->removeListener (listenerToRemove);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_realtime_callback_listener_with_address(&mut self, listener_to_remove: *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverRealtimeCallback>)  {
        
        todo!();
        /*
            pimpl->removeListener (listenerToRemove);
        */
    }
    
    /**
      | Installs a custom error handler which
      | is called in case the receiver encounters
      | a stream it cannot parse as an OSC bundle
      | or OSC message.
      | 
      | By default (i.e. if you never use this
      | method), in case of a parsing error nothing
      | happens and the invalid packet is simply
      | discarded.
      |
      */
    pub fn register_format_error_handler(&mut self, handler: OSCReceiverFormatErrorHandler)  {
        
        todo!();
        /*
            pimpl->registerFormatErrorHandler (handler);
        */
    }
}
