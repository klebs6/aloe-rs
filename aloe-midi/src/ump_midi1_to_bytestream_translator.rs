crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPMidi1ToBytestreamTranslator.h]

/**
  | Parses a raw stream of uint32_t holding
  | a series of Universal MIDI Packets using
  | the MIDI 1.0 Protocol, converting to
  | plain (non-UMP) MidiMessages.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsMidi1ToBytestreamTranslator {
    pending_sys_ex_data: Vec<u8>,
    pending_sys_ex_time: f64, // default = 0.0
}

impl UniversalMidiPacketsMidi1ToBytestreamTranslator {

    /**
      | Ensures that there is room in the internal
      | buffer for a sysex message of at least
      | `initialBufferSize` bytes.
      |
      */
    pub fn new(initial_buffer_size: i32) -> Self {
    
        todo!();
        /*


            pendingSysExData.reserve (size_t (initialBufferSize));
        */
    }

    /**
      | Clears the concatenator.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            pendingSysExData.clear();
            pendingSysExTime = 0.0;
        */
    }

    /**
      | Converts a Universal MIDI Packet using
      | the MIDI 1.0 Protocol to an equivalent
      | MidiMessage. Accumulates SysEx packets
      | into a single MidiMessage, as appropriate.
      | 
      | -----------
      | @param packet
      | 
      | a packet which is using the MIDI 1.0 Protocol.
      | ----------
      | @param time
      | 
      | the timestamp to be applied to these
      | messages.
      | ----------
      | @param callback
      | 
      | a callback which will be called with
      | each converted MidiMessage.
      |
      */
    pub fn dispatch<MessageCallback>(&mut self, 
        packet:   &UniversalMidiPacketsView,
        time:     f64,
        callback: MessageCallback)  {
    
        todo!();
        /*
            const auto firstWord = *packet.data();

            if (! pendingSysExData.empty() && shouldPacketTerminateSysExEarly (firstWord))
                pendingSysExData.clear();

            switch (packet.size())
            {
                case 1:
                {
                    // Utility messages don't translate to bytestream format
                    if (Utils::getMessageType (firstWord) != 0x00)
                        callback (fromUmp (UniversalMidiPacketX1 { firstWord }, time));

                    break;
                }

                case 2:
                {
                    if (Utils::getMessageType (firstWord) == 0x3)
                        processSysEx (UniversalMidiPacketX2 { packet[0], packet[1] }, time, callback);

                    break;
                }

                case 3:  // no 3-word packets in the current spec
                case 4:  // no 4-word packets translate to bytestream format
                default:
                    break;
            }
        */
    }

    /**
      | Converts from a Universal MIDI Packet
      | to MIDI 1 bytestream format.
      | 
      | This is only capable of converting a
      | single Universal MIDI Packet to an equivalent
      | bytestream MIDI message. This function
      | cannot understand multi-packet messages,
      | like UniversalMidiPacketsSysEx7 messages.
      | 
      | To convert multi-packet messages,
      | use `UniversalMidiPacketsMidi1ToBytestreamTranslator`
      | to convert from a UMP MIDI 1.0 stream,
      | or `UniversalMidiPacketsToBytestreamDispatcher` to convert
      | from both MIDI 2.0 and MIDI 1.0.
      |
      */
    pub fn from_ump(
        m:    &UniversalMidiPacketX1,
        time: Option<f64>

    ) -> MidiMessage {

        let time: f64 = time.unwrap_or(0.0);

        todo!();
        /*
            const auto word = m.front();
            jassert (Utils::getNumWordsForMessageType (word) == 1);

            const std::array<uint8_t, 3> bytes { { uint8_t ((word >> 0x10) & 0xff),
                                                   uint8_t ((word >> 0x08) & 0xff),
                                                   uint8_t ((word >> 0x00) & 0xff) } };
            const auto numBytes = MidiMessage::getMessageLengthFromFirstByte (bytes.front());
            return MidiMessage (bytes.data(), numBytes, time);
        */
    }
    
    pub fn process_sys_ex<MessageCallback>(
        &mut self, 
        packet:   &UniversalMidiPacketX2,
        time:     f64,
        callback: MessageCallback

    ) {
    
        todo!();

        /*
            switch (getSysEx7Kind (packet[0]))
            {
                case UniversalMidiPacketsSysEx7::Kind::complete:
                    startSysExMessage (time);
                    pushBytes (packet);
                    terminateSysExMessage (callback);
                    break;

                case UniversalMidiPacketsSysEx7::Kind::begin:
                    startSysExMessage (time);
                    pushBytes (packet);
                    break;

                case UniversalMidiPacketsSysEx7::Kind::continuation:
                    if (pendingSysExData.empty())
                        break;

                    pushBytes (packet);
                    break;

                case UniversalMidiPacketsSysEx7::Kind::end:
                    if (pendingSysExData.empty())
                        break;

                    pushBytes (packet);
                    terminateSysExMessage (callback);
                    break;
            }
        */
    }
    
    pub fn push_bytes(&mut self, packet: &UniversalMidiPacketX2)  {
        
        todo!();
        /*
            const auto bytes = UniversalMidiPacketsSysEx7::getDataBytes (packet);
            pendingSysExData.insert (pendingSysExData.end(),
                                     bytes.data.begin(),
                                     bytes.data.begin() + bytes.size);
        */
    }
    
    pub fn start_sys_ex_message(&mut self, time: f64)  {
        
        todo!();
        /*
            pendingSysExTime = time;
            pendingSysExData.push_back (0xf0);
        */
    }
    
    pub fn terminate_sys_ex_message<MessageCallback>(&mut self, callback: MessageCallback)  {
    
        todo!();
        /*
            pendingSysExData.push_back (0xf7);
            callback (MidiMessage (pendingSysExData.data(),
                                   int (pendingSysExData.size()),
                                   pendingSysExTime));
            pendingSysExData.clear();
        */
    }
    
    pub fn should_packet_terminate_sys_ex_early(first_word: u32) -> bool {
        
        todo!();
        /*
            return ! (isSysExContinuation (firstWord)
                      || isSystemRealTime (firstWord)
                      || isJROrNOP (firstWord));
        */
    }
    
    pub fn get_sys_ex_7kind(word: u32) -> UniversalMidiPacketsSysEx7Kind {
        
        todo!();
        /*
            return UniversalMidiPacketsSysEx7::Kind ((word >> 0x14) & 0xf);
        */
    }
    
    pub fn is_jr_ornop(word: u32) -> bool {
        
        todo!();
        /*
            return Utils::getMessageType (word) == 0x0;
        */
    }
    
    pub fn is_sys_ex_continuation(word: u32) -> bool {
        
        todo!();
        /*
            if (Utils::getMessageType (word) != 0x3)
                return false;

            const auto kind = getSysEx7Kind (word);
            return kind == UniversalMidiPacketsSysEx7::Kind::continuation || kind == UniversalMidiPacketsSysEx7::Kind::end;
        */
    }
    
    pub fn is_system_real_time(word: u32) -> bool {
        
        todo!();
        /*
            return Utils::getMessageType (word) == 0x1 && ((word >> 0x10) & 0xff) >= 0xf8;
        */
    }
}
