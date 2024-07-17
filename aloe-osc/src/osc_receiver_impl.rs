crate::ix!();

#[no_copy]
#[leak_detector]
pub struct OSCReceiverPimpl {
    base:                            Thread,
    base2:                           MessageListener,
    listeners:                       ListenerList<Box<dyn OSCReceiverListener<OSCReceiverMessageLoopCallback>>>,
    realtime_listeners:              ListenerList<Box<dyn OSCReceiverListener<OSCReceiverRealtimeCallback>>>,
    listeners_with_address:          Vec<(OSCAddress,*mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverMessageLoopCallback>)>,
    realtime_listeners_with_address: Vec<(OSCAddress,*mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverRealtimeCallback>)>,
    socket:                          OptionalScopedPointer<DatagramSocket>,
    format_error_handler:            OSCReceiverFormatErrorHandler, // default = { nullptr  }
}

impl Drop for OSCReceiverPimpl {

    fn drop(&mut self) {
        todo!();
        /*
            disconnect();
        */
    }
}

impl OSCReceiverPimpl {

    pub fn new(osc_thread_name: &String) -> Self {
    
        todo!();
        /*
        : thread(oscThreadName),

        
        */
    }
    
    pub fn connect_to_port(&mut self, port_number: i32) -> bool {
        
        todo!();
        /*
            if (! disconnect())
                return false;

            socket.setOwned (new DatagramSocket (false));

            if (! socket->bindToPort (portNumber))
                return false;

            startThread();
            return true;
        */
    }
    
    pub fn connect_to_socket(&mut self, new_socket: &mut DatagramSocket) -> bool {
        
        todo!();
        /*
            if (! disconnect())
                return false;

            socket.setNonOwned (&newSocket);
            startThread();
            return true;
        */
    }
    
    pub fn disconnect(&mut self) -> bool {
        
        todo!();
        /*
            if (socket != nullptr)
            {
                signalThreadShouldExit();

                if (socket.willDeleteObject())
                    socket->shutdown();

                waitForThreadToExit (10000);
                socket.reset();
            }

            return true;
        */
    }
    
    pub fn add_listener_message_loop_callback(&mut self, listener_to_add: *mut dyn OSCReceiverListener<OSCReceiverMessageLoopCallback>)  {
        
        todo!();
        /*
            listeners.add (listenerToAdd);
        */
    }
    
    pub fn add_listener_realtime_callback(&mut self, listener_to_add: *mut dyn OSCReceiverListener<OSCReceiverRealtimeCallback>)  {
        
        todo!();
        /*
            realtimeListeners.add (listenerToAdd);
        */
    }
    
    pub fn add_listener_message_loop_callback_with_address(&mut self, 
        listener_to_add:  *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverMessageLoopCallback>,
        address_to_match: OSCAddress)  {
        
        todo!();
        /*
            addListenerWithAddress (listenerToAdd, addressToMatch, listenersWithAddress);
        */
    }
    
    pub fn add_listener_realtime_callback_with_address(&mut self, 
        listener_to_add:  *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverRealtimeCallback>,
        address_to_match: OSCAddress)  {
        
        todo!();
        /*
            addListenerWithAddress (listenerToAdd, addressToMatch, realtimeListenersWithAddress);
        */
    }
    
    pub fn remove_message_loop_callback_listener(&mut self, listener_to_remove: *mut dyn OSCReceiverListener<OSCReceiverMessageLoopCallback>)  {
        
        todo!();
        /*
            listeners.remove (listenerToRemove);
        */
    }
    
    pub fn remove_realtime_callback_listener(&mut self, listener_to_remove: *mut dyn OSCReceiverListener<OSCReceiverRealtimeCallback>)  {
        
        todo!();
        /*
            realtimeListeners.remove (listenerToRemove);
        */
    }
    
    pub fn remove_message_loop_callback_listener_with_address(&mut self, listener_to_remove: *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverMessageLoopCallback>)  {
        
        todo!();
        /*
            removeListenerWithAddress (listenerToRemove, listenersWithAddress);
        */
    }
    
    pub fn remove_realtime_callback_listener_with_address(&mut self, listener_to_remove: *mut dyn OSCReceiverListenerWithOSCAddress<OSCReceiverRealtimeCallback>)  {
        
        todo!();
        /*
            removeListenerWithAddress (listenerToRemove, realtimeListenersWithAddress);
        */
    }
    
    pub fn handle_buffer(&mut self, 
        data:      *const u8,
        data_size: usize)  {
        
        todo!();
        /*
            OSCInputStream inStream (data, dataSize);

            try
            {
                auto content = inStream.readElementWithKnownSize (dataSize);

                // realtime listeners should receive the OSC content first - and immediately
                // on this thread:
                callRealtimeListeners (content);

                if (content.isMessage())
                    callRealtimeListenersWithAddress (content.getMessage());

                // now post the message that will trigger the handleMessage callback
                // dealing with the non-realtime listeners.
                if (listeners.size() > 0 || listenersWithAddress.size() > 0)
                    postMessage (new OSCReceiverPimplCallbackMessage (content));
            }
            catch (const OSCFormatError&)
            {
                if (formatErrorHandler != nullptr)
                    formatErrorHandler (data, (int) dataSize);
            }
        */
    }
    
    pub fn register_format_error_handler(&mut self, handler: OSCReceiverFormatErrorHandler)  {
        
        todo!();
        /*
            formatErrorHandler = handler;
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            int bufferSize = 65535;
            HeapBlock<char> oscBuffer (bufferSize);

            while (! threadShouldExit())
            {
                jassert (socket != nullptr);
                auto ready = socket->waitUntilReady (true, 100);

                if (ready < 0 || threadShouldExit())
                    return;

                if (ready == 0)
                    continue;

                auto bytesRead = (size_t) socket->read (oscBuffer.getData(), bufferSize, false);

                if (bytesRead >= 4)
                    handleBuffer (oscBuffer.getData(), bytesRead);
            }
        */
    }
    
    
    pub fn add_listener_with_address<ListenerType>(&mut self, 
        listener_to_add: *mut ListenerType,
        address:         OSCAddress,
        array:           &mut Vec<(OSCAddress,*mut ListenerType)>)  {
    
        todo!();
        /*
            for (auto& i : array)
                if (address == i.first && listenerToAdd == i.second)
                    return;

            array.add (std::make_pair (address, listenerToAdd));
        */
    }
    
    
    pub fn remove_listener_with_address<ListenerType>(&mut self, 
        listener_to_remove: *mut ListenerType,
        array:              &mut Vec<(OSCAddress,*mut ListenerType)>)  {
    
        todo!();
        /*
            for (int i = 0; i < array.size(); ++i)
            {
                if (listenerToRemove == array.getReference (i).second)
                {
                    // aarrgh... can't simply call array.remove (i) because this
                    // requires a default c'tor to be present for OSCAddress...
                    // luckily, we don't care about methods preserving element order:
                    array.swap (i, array.size() - 1);
                    array.removeLast();
                    break;
                }
            }
        */
    }
    
    pub fn handle_message(&mut self, msg: &Message)  {
        
        todo!();
        /*
            if (auto* callbackMessage = dynamic_cast<const OSCReceiverPimplCallbackMessage*> (&msg))
            {
                auto& content = callbackMessage->content;

                callListeners (content);

                if (content.isMessage())
                    callListenersWithAddress (content.getMessage());
            }
        */
    }
    
    pub fn call_listeners(&mut self, content: &OSCBundleElement)  {
        
        todo!();
        /*
            using OSCListener = OSCReceiver::OSCReceiverListener<OSCReceiver::OSCReceiverMessageLoopCallback>;

            if (content.isMessage())
            {
                auto&& message = content.getMessage();
                listeners.call ([&] (OSCListener& l) { l.oscMessageReceived (message); });
            }
            else if (content.isBundle())
            {
                auto&& bundle = content.getBundle();
                listeners.call ([&] (OSCListener& l) { l.oscBundleReceived (bundle); });
            }
        */
    }
    
    pub fn call_realtime_listeners(&mut self, content: &OSCBundleElement)  {
        
        todo!();
        /*
            using OSCListener = OSCReceiver::OSCReceiverListener<OSCReceiver::OSCReceiverRealtimeCallback>;

            if (content.isMessage())
            {
                auto&& message = content.getMessage();
                realtimeListeners.call ([&] (OSCListener& l) { l.oscMessageReceived (message); });
            }
            else if (content.isBundle())
            {
                auto&& bundle = content.getBundle();
                realtimeListeners.call ([&] (OSCListener& l) { l.oscBundleReceived (bundle); });
            }
        */
    }
    
    pub fn call_listeners_with_address(&mut self, message: &OSCMessage)  {
        
        todo!();
        /*
            for (auto& entry : listenersWithAddress)
                if (auto* listener = entry.second)
                    if (message.getAddressPattern().matches (entry.first))
                        listener->oscMessageReceived (message);
        */
    }
    
    pub fn call_realtime_listeners_with_address(&mut self, message: &OSCMessage)  {
        
        todo!();
        /*
            for (auto& entry : realtimeListenersWithAddress)
                if (auto* listener = entry.second)
                    if (message.getAddressPattern().matches (entry.first))
                        listener->oscMessageReceived (message);
        */
    }
}
