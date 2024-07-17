crate::ix!();

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub struct AlsaClient {
    base:             ReferenceCountedObject,
    handle:           *mut SndSeq, // default = nullptr
    client_id:        i32, // default = 0
    ports:            Vec<Box<AlsaClientPort>>,
    active_callbacks: Atomic<i32>,
    callback_lock:    CriticalSection,
    input_thread:     Box<MidiInputThread>,
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Default for AlsaClient {
    
    fn default() -> Self {
        todo!();
        /*


            jassert (instance == nullptr);

            snd_seq_open (&handle, "default", SND_SEQ_OPEN_DUPLEX, 0);

            if (handle != nullptr)
            {
                snd_seq_nonblock (handle, SND_SEQ_NONBLOCK);
                snd_seq_set_client_name (handle, getAlsaMidiName().toRawUTF8());
                clientId = snd_seq_client_id (handle);

                // It's good idea to pre-allocate a good number of elements
                ports.ensureStorageAllocated (32);
            
        */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Drop for AlsaClient {
    fn drop(&mut self) {
        todo!();
        /* 
            jassert (instance != nullptr);
            instance = nullptr;

            jassert (activeCallbacks.get() == 0);

            if (inputThread)
                inputThread->stopThread (3000);

            if (handle != nullptr)
                snd_seq_close (handle);
         */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl AlsaClient {
    
    pub fn get_alsa_midi_name() -> String {
        
        todo!();
        /*
            #ifdef ALOE_ALSA_MIDI_NAME
             return ALOE_ALSA_MIDI_NAME;
            #else
             if (auto* app = ALOEApplicationBase::getInstance())
                 return app->getApplicationName();

             return "Aloe";
            #endif
        */
    }
    
    pub fn get_instance() -> AlsaClientPtr {
        
        todo!();
        /*
            if (instance == nullptr)
                instance = new AlsaClient();

            return instance;
        */
    }
    
    pub fn register_callback(&mut self)  {
        
        todo!();
        /*
            if (inputThread == nullptr)
                inputThread.reset (new MidiInputThread (*this));

            if (++activeCallbacks == 1)
                inputThread->startThread();
        */
    }
    
    pub fn unregister_callback(&mut self)  {
        
        todo!();
        /*
            jassert (activeCallbacks.get() > 0);

            if (--activeCallbacks == 0 && inputThread->isThreadRunning())
                inputThread->signalThreadShouldExit();
        */
    }
    
    pub fn handle_incoming_midi_message(&mut self, 
        event:   *mut SndSeqEvent,
        message: &MidiMessage)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

            if (auto* port = ports[event->dest.port])
                port->handleIncomingMidiMessage (message);
        */
    }
    
    pub fn handle_partial_sysex_message(&mut self, 
        event:            *mut SndSeqEvent,
        message_data:     *const u8,
        num_bytes_so_far: i32,
        time_stamp:       f64)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

            if (auto* port = ports[event->dest.port])
                port->handlePartialSysexMessage (messageData, numBytesSoFar, timeStamp);
        */
    }
    
    pub fn get(&self) -> *mut SndSeq {
        
        todo!();
        /*
            return handle;
        */
    }
    
    pub fn get_id(&self) -> i32 {
        
        todo!();
        /*
            return clientId;
        */
    }
    
    pub fn create_port(&mut self, 
        name:                &String,
        for_input:           bool,
        enable_subscription: bool) -> *mut AlsaClientPort {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

            auto port = new AlsaClientPort (*this, forInput);
            port->createPort (name, enableSubscription);
            ports.set (port->getPortId(), port);
            incReferenceCount();
            return port;
        */
    }
    
    pub fn delete_port(&mut self, port: *mut AlsaClientPort)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);

            ports.set (port->getPortId(), nullptr);
            decReferenceCount();
        */
    }
}
