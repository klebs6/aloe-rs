crate::ix!();

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub fn get_formatted_port_identifier(
        client_id: i32,
        port_id:   i32) -> String {
    
    todo!();
    /*
        return String (clientId) + "-" + String (portId);
    */
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub type AlsaClientPtr = ReferenceCountedObjectPtr<AlsaClient>;

/**
  | represents an input or output port of
  | the supplied AlsaClient
  |
  */
#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub struct AlsaClientPort {
    client:           &mut AlsaClient,
    callback:         *mut MidiInputCallback, // default = nullptr
    midi_parser:      *mut SndMidiEvent, // default = nullptr
    midi_input:       *mut MidiInput, // default = nullptr
    port_name:        String,
    max_event_size:   i32, // default = 4096
    port_id:          i32, // default = -1
    callback_enabled: AtomicBool, // default = false 
    is_input:         bool, // default = false
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Drop for AlsaClientPort {

    fn drop(&mut self) {
        todo!();
        /* 
                if (isValid())
                {
                    if (isInput)
                        enableCallback (false);
                    else
                        snd_midi_event_free (midiParser);

                    snd_seq_delete_simple_port (client.get(), portId);
                }
             */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl PartialEq<AlsaClientPort> for AlsaClientPort {
    
    #[inline] fn eq(&self, other: &AlsaClientPort) -> bool {
        todo!();
        /*
            return portId != -1 && portId == lhs.portId;
        */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Eq for AlsaClientPort {}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl AlsaClientPort {

    pub fn new(
        c:         &mut AlsaClient,
        for_input: bool) -> Self {
    
        todo!();
        /*
        : client(c),
        : is_input(forInput),

        
        */
    }
    
    pub fn connect_with(&self, 
        source_client: i32,
        source_port:   i32)  {
        
        todo!();
        /*
            if (isInput)
                    snd_seq_connect_from (client.get(), portId, sourceClient, sourcePort);
                else
                    snd_seq_connect_to (client.get(), portId, sourceClient, sourcePort);
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return client.get() != nullptr && portId >= 0;
        */
    }
    
    pub fn setup_input(&mut self, 
        input: *mut MidiInput,
        cb:    *mut MidiInputCallback)  {
        
        todo!();
        /*
            jassert (cb != nullptr && input != nullptr);
                callback = cb;
                midiInput = input;
        */
    }
    
    pub fn setup_output(&mut self)  {
        
        todo!();
        /*
            jassert (! isInput);
                snd_midi_event_new ((size_t) maxEventSize, &midiParser);
        */
    }
    
    pub fn enable_callback(&mut self, enable: bool)  {
        
        todo!();
        /*
            const auto oldValue = callbackEnabled.exchange (enable);

                if (oldValue != enable)
                {
                    if (enable)
                        client.registerCallback();
                    else
                        client.unregisterCallback();
                }
        */
    }
    
    pub fn send_message_now(&mut self, message: &MidiMessage) -> bool {
        
        todo!();
        /*
            if (message.getRawDataSize() > maxEventSize)
                {
                    maxEventSize = message.getRawDataSize();
                    snd_midi_event_free (midiParser);
                    snd_midi_event_new ((size_t) maxEventSize, &midiParser);
                }

                snd_seq_event_t event;
                snd_seq_ev_clear (&event);

                auto numBytes = (long) message.getRawDataSize();
                auto* data = message.getRawData();

                auto seqHandle = client.get();
                bool success = true;

                while (numBytes > 0)
                {
                    auto numSent = snd_midi_event_encode (midiParser, data, numBytes, &event);

                    if (numSent <= 0)
                    {
                        success = numSent == 0;
                        break;
                    }

                    numBytes -= numSent;
                    data += numSent;

                    snd_seq_ev_set_source (&event, (unsigned char) portId);
                    snd_seq_ev_set_subs (&event);
                    snd_seq_ev_set_direct (&event);

                    if (snd_seq_event_output_direct (seqHandle, &event) < 0)
                    {
                        success = false;
                        break;
                    }
                }

                snd_midi_event_reset_encode (midiParser);
                return success;
        */
    }
    
    pub fn create_port(&mut self, 
        name:                &String,
        enable_subscription: bool)  {
        
        todo!();
        /*
            if (auto seqHandle = client.get())
                {
                    const unsigned int caps =
                        isInput ? (SND_SEQ_PORT_CAP_WRITE | (enableSubscription ? SND_SEQ_PORT_CAP_SUBS_WRITE : 0))
                                : (SND_SEQ_PORT_CAP_READ  | (enableSubscription ? SND_SEQ_PORT_CAP_SUBS_READ : 0));

                    portName = name;
                    portId = snd_seq_create_simple_port (seqHandle, portName.toUTF8(), caps,
                                                         SND_SEQ_PORT_TYPE_MIDI_GENERIC |
                                                         SND_SEQ_PORT_TYPE_APPLICATION);
                }
        */
    }
    
    pub fn handle_incoming_midi_message(&self, message: &MidiMessage)  {
        
        todo!();
        /*
            if (callbackEnabled)
                    callback->handleIncomingMidiMessage (midiInput, message);
        */
    }
    
    pub fn handle_partial_sysex_message(&mut self, 
        message_data:     *const u8,
        num_bytes_so_far: i32,
        time_stamp:       f64)  {
        
        todo!();
        /*
            if (callbackEnabled)
                    callback->handlePartialSysexMessage (midiInput, messageData, numBytesSoFar, timeStamp);
        */
    }
    
    pub fn get_port_id(&self) -> i32 {
        
        todo!();
        /*
            return portId;
        */
    }
    
    pub fn get_port_name(&self) -> &String {
        
        todo!();
        /*
            return portName;
        */
    }
}

///---------------------
#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub struct AlsaPortPtr {
    ptr: *mut AlsaClient::AlsaClientPort, // default = nullptr
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl Drop for AlsaPortPtr {

    fn drop(&mut self) {
        todo!();
        /*      AlsaClient::getInstance()->deletePort (ptr);  */
    }
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl AlsaPortPtr {
    
    pub fn new(p: *mut AlsaClient::AlsaClientPort) -> Self {
    
        todo!();
        /*
        : ptr(p),

        
        */
    }
}
