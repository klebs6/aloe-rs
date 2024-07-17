crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiDataConcatenator.h]

/**
  | Helper class that takes chunks of incoming
  | midi bytes, packages them into messages, and
  | dispatches them to a midi callback.
  |
  | @tags{Audio}
  */
#[no_copy]
pub struct MidiDataConcatenator {
    current_message:     [u8; 3],
    current_message_len: i32, // default = 0
    pending_sysex_data:  MemoryBlock,
    pending_sysex_time:  f64, // default = 0
    pending_sysex_size:  i32, // default = 0
}

impl MidiDataConcatenator {

    pub fn new(initial_buffer_size: i32) -> Self {
    
        todo!();
        /*
            : pendingSysexData ((size_t) initialBufferSize)
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            currentMessageLen = 0;
            pendingSysexSize = 0;
            pendingSysexTime = 0;
        */
    }
    
    pub fn push_midi_data<UserDataType, CallbackType>(&mut self, 
        input_data: *const c_void,
        num_bytes:  i32,
        time:       f64,
        input:      *mut UserDataType,
        callback:   &mut CallbackType)  {
    
        todo!();
        /*
            auto d = static_cast<const uint8*> (inputData);

            while (numBytes > 0)
            {
                auto nextByte = *d;

                if (pendingSysexSize != 0 || nextByte == 0xf0)
                {
                    processSysex (d, numBytes, time, input, callback);
                    currentMessageLen = 0;
                    continue;
                }

                ++d;
                --numBytes;

                if (isRealtimeMessage (nextByte))
                {
                    callback.handleIncomingMidiMessage (input, MidiMessage (nextByte, time));
                    // These can be embedded in the middle of a normal message, so we won't
                    // reset the currentMessageLen here.
                    continue;
                }

                if (isInitialByte (nextByte))
                {
                    currentMessage[0] = nextByte;
                    currentMessageLen = 1;
                }
                else if (currentMessageLen > 0 && currentMessageLen < 3)
                {
                    currentMessage[currentMessageLen++] = nextByte;
                }
                else
                {
                    // message is too long or invalid MIDI - abandon it and start again with the next byte
                    currentMessageLen = 0;
                    continue;
                }

                auto expectedLength = MidiMessage::getMessageLengthFromFirstByte (currentMessage[0]);

                if (expectedLength == currentMessageLen)
                {
                    callback.handleIncomingMidiMessage (input, MidiMessage (currentMessage, expectedLength, time));
                    currentMessageLen = 1; // reset, but leave the first byte to use as the running status byte
                }
            }
        */
    }
    
    pub fn process_sysex<UserDataType, CallbackType>(&mut self, 
        d:         &mut *const u8,
        num_bytes: &mut i32,
        time:      f64,
        input:     *mut UserDataType,
        callback:  &mut CallbackType)  {
    
        todo!();
        /*
            if (*d == 0xf0)
            {
                pendingSysexSize = 0;
                pendingSysexTime = time;
            }

            pendingSysexData.ensureSize ((size_t) (pendingSysexSize + numBytes), false);
            auto totalMessage = static_cast<uint8*> (pendingSysexData.getData());
            auto dest = totalMessage + pendingSysexSize;

            do
            {
                if (pendingSysexSize > 0 && isStatusByte (*d))
                {
                    if (*d == 0xf7)
                    {
                        *dest++ = *d++;
                        ++pendingSysexSize;
                        --numBytes;
                        break;
                    }

                    if (*d >= 0xfa || *d == 0xf8)
                    {
                        callback.handleIncomingMidiMessage (input, MidiMessage (*d, time));
                        ++d;
                        --numBytes;
                    }
                    else
                    {
                        pendingSysexSize = 0;
                        int used = 0;
                        const MidiMessage m (d, numBytes, used, 0, time);

                        if (used > 0)
                        {
                            callback.handleIncomingMidiMessage (input, m);
                            numBytes -= used;
                            d += used;
                        }

                        break;
                    }
                }
                else
                {
                    *dest++ = *d++;
                    ++pendingSysexSize;
                    --numBytes;
                }
            }
            while (numBytes > 0);

            if (pendingSysexSize > 0)
            {
                if (totalMessage [pendingSysexSize - 1] == 0xf7)
                {
                    callback.handleIncomingMidiMessage (input, MidiMessage (totalMessage, pendingSysexSize, pendingSysexTime));
                    pendingSysexSize = 0;
                }
                else
                {
                    callback.handlePartialSysexMessage (input, totalMessage, pendingSysexSize, pendingSysexTime);
                }
            }
        */
    }
    
    pub fn is_realtime_message(byte: u8) -> bool {
        
        todo!();
        /*
            return byte >= 0xf8 && byte <= 0xfe;
        */
    }
    
    pub fn is_status_byte(byte: u8) -> bool {
        
        todo!();
        /*
            return byte >= 0x80;
        */
    }
    
    pub fn is_initial_byte(byte: u8) -> bool {
        
        todo!();
        /*
            return isStatusByte (byte) && byte != 0xf7;
        */
    }
}
