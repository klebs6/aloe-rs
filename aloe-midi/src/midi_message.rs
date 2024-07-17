crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiMessage.h]

/**
  | Encapsulates a MIDI message.
  |
  | @see MidiMessageSequence, MidiOutput,
  | MidiInput
  |
  | @tags{Audio}
  */
pub struct MidiMessage {
    packed_data: MidiMessagePackedData,
    time_stamp:  f64, // default = 0
    size:        i32,
}

impl Default for MidiMessage {
    
    /**
      | Creates an active-sense message. Since
      | the MidiMessage has to contain a valid
      | message, this default constructor
      | just initialises it with an empty sysex
      | message.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : size(2),

            packedData.asBytes[0] = 0xf0;
        packedData.asBytes[1] = 0xf7;
        */
    }
}

impl Drop for MidiMessage {

    fn drop(&mut self) {
        todo!();
        /* 
        if (isHeapAllocated())
            std::free (packedData.allocatedData);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiMessage.cpp]
impl MidiMessage {

    /**
      | Creates a midi message from a list of
      | bytes.
      |
      */
    pub fn new<Data>(
        byte1:       i32,
        byte2:       i32,
        byte3:       i32,
        other_bytes: Data) -> Self {
    
        todo!();
        /*


            : size (3 + sizeof... (otherBytes))
            // this checks that the length matches the data..
            jassert (size > 3 || byte1 >= 0xf0 || getMessageLengthFromFirstByte ((uint8) byte1) == size);

            const uint8 data[] = { (uint8) byte1, (uint8) byte2, (uint8) byte3, static_cast<uint8> (otherBytes)... };
            memcpy (allocateSpace (size), data, (size_t) size);
        */
    }

    /**
      | Returns a pointer to the raw midi data.
      | @see getRawDataSize
      |
      */
    pub fn get_raw_data(&self) -> *const u8 {
        
        todo!();
        /*
            return getData();
        */
    }

    /**
      | Returns the number of bytes of data in
      | the message. @see getRawData
      |
      */
    pub fn get_raw_data_size(&self) -> i32 {
        
        todo!();
        /*
            return size;
        */
    }

    /** 
      | Returns the timestamp associated with this
      | message.
      |
      | The exact meaning of this time and its units
      | will vary, as messages are used in a variety
      | of different contexts.
      |
      | If you're getting the message from a midi
      | file, this could be a time in seconds, or
      | a number of ticks - see
      | MidiFile::convertTimestampTicksToSeconds().
      |
      | If the message is being used in a MidiBuffer,
      | it might indicate the number of audio samples
      | from the start of the buffer.
      |
      | If the message was created by a MidiInput,
      | see
      | MidiInputCallback::handleIncomingMidiMessage()
      | for details of the way that it initialises
      | this value.
      |
      | @see setTimeStamp, addToTimeStamp
      */
    pub fn get_time_stamp(&self) -> f64 {
        
        todo!();
        /*
            return timeStamp;
        */
    }

    /** 
      | Changes the message's associated timestamp.
      |
      | The units for the timestamp will be
      | application-specific - see the notes for
      | getTimeStamp().  @see addToTimeStamp,
      | getTimeStamp
      */
    pub fn set_time_stamp(&mut self, new_timestamp: f64)  {

        todo!();
        /*
            timeStamp = newTimestamp;
        */
    }

    /** 
      | Adds a value to the message's timestamp.
      |
      | The units for the timestamp will be
      | application-specific.
      */
    pub fn add_to_time_stamp(&mut self, delta: f64)  {

        todo!();
        /*
            timeStamp += delta;
        */
    }

    #[inline] pub fn is_heap_allocated(&self) -> bool {
        
        todo!();
        /*
            return size > (int) sizeof (packedData);
        */
    }
    
    #[inline] pub fn get_data(&self) -> *mut u8 {
        
        todo!();
        /*
            return isHeapAllocated() ? packedData.allocatedData : (uint8*) packedData.asBytes;
        */
    }
    
    /**
      | Converts a floating-point value between
      | 0 and 1 to a MIDI 7-bit value between 0
      | and 127.
      |
      */
    pub fn float_value_to_midi_byte(&mut self, v: f32) -> u8 {
        
        todo!();
        /*
            jassert (v >= 0 && v <= 1.0f);  // if your value is > 1, maybe you're passing an
                                        // integer value to a float method by mistake?

        return MidiHelpers::validVelocity (roundToInt (v * 127.0f));
        */
    }
    
    /**
      | Converts a pitchbend value in semitones
      | to a MIDI 14-bit pitchwheel position
      | value.
      |
      */
    pub fn pitchbend_to_pitchwheel_pos(&mut self, 
        pitchbend:       f32,
        pitchbend_range: f32) -> u16 {
        
        todo!();
        /*
            // can't translate a pitchbend value that is outside of the given range!
        jassert (std::abs (pitchbend) <= pitchbendRange);

        return static_cast<uint16> (pitchbend > 0.0f
                                      ? jmap (pitchbend, 0.0f, pitchbendRange, 8192.0f, 16383.0f)
                                      : jmap (pitchbend, -pitchbendRange, 0.0f, 0.0f, 8192.0f));
        */
    }
    
    /**
      | Reads a midi variable-length integer,
      | with protection against buffer overflow.
      | 
      | -----------
      | @param data
      | 
      | the data to read the number from
      | ----------
      | @param maxBytesToUse
      | 
      | the number of bytes in the region following
      | `data`
      | 
      | -----------
      | @return
      | 
      | a struct containing the parsed value,
      | and the number of bytes that were read.
      | If parsing fails, both the `value` and
      | `bytesUsed` fields will be set to 0 and
      | `isValid()` will return false
      |
      */
    pub fn read_variable_length_value(&mut self, 
        data:             *const u8,
        max_bytes_to_use: i32) -> MidiMessageVariableLengthValue {
        
        todo!();
        /*
            uint32 v = 0;

        // The largest allowable variable-length value is 0x0f'ff'ff'ff which is
        // represented by the 4-byte stream 0xff 0xff 0xff 0x7f.
        // Longer bytestreams risk overflowing a 32-bit signed int.
        const auto limit = jmin (maxBytesToUse, 4);

        for (int numBytesUsed = 0; numBytesUsed < limit; ++numBytesUsed)
        {
            const auto i = data[numBytesUsed];
            v = (v << 7) + (i & 0x7f);

            if (! (i & 0x80))
                return { (int) v, numBytesUsed + 1 };
        }

        // If this is hit, the input was malformed. Either there were not enough
        // bytes of input to construct a full value, or no terminating byte was
        // found. This implementation only supports variable-length values of up
        // to four bytes.
        return {};
        */
    }
    
    /**
      | Reads a midi variable-length integer.
      | 
      | -----------
      | @note
      | 
      | This signature has been deprecated
      | in favour of the safer
      | 
      | readVariableLengthValue.
      | ----------
      | @note
      | 
      | The `data` argument indicates the data
      | to read the number from, and `numBytesUsed` 
      | is used as an out-parameter to indicate the 
      | number of bytes that were read.
      |
      */
    #[deprecated]
    pub fn read_variable_length_val(&mut self, 
        data:           *const u8,
        num_bytes_used: &mut i32) -> i32 {
        
        todo!();
        /*
            numBytesUsed = 0;
        int v = 0, i;

        do
        {
            i = (int) *data++;

            if (++numBytesUsed > 6)
                break;

            v = (v << 7) + (i & 0x7f);

        } while (i & 0x80);

        return v;
        */
    }
    
    /**
      | Based on the first byte of a short midi
      | message, this uses a lookup table to
      | return the message length (either 1,
      | 2, or 3 bytes).
      | 
      | -----------
      | @note
      | 
      | The value passed in must be 0x80 or higher.
      |
      */
    pub fn get_message_length_from_first_byte(&mut self, first_byte: u8) -> i32 {
        
        todo!();
        /*
            // this method only works for valid starting bytes of a short midi message
        jassert (firstByte >= 0x80 && firstByte != 0xf0 && firstByte != 0xf7);

        static const char messageLengths[] =
        {
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            1, 2, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
        };

        return messageLengths[firstByte & 0x7f];
        */
    }
    
    /**
      | Creates a midi message from a block of
      | data.
      |
      */
    pub fn new_from_block_of_data(
        d:          *const c_void,
        data_size:  i32,
        time_stamp: Option<f64>

    ) -> Self {

        let time_stamp: f64 = time_stamp.unwrap_or(0.0);
    
        todo!();
        /*
        : time_stamp(time_stamp),
        : size(dataSize),

            jassert (dataSize > 0);
        // this checks that the length matches the data..
        jassert (dataSize > 3 || *(uint8*)d >= 0xf0 || getMessageLengthFromFirstByte (*(uint8*)d) == size);

        memcpy (allocateSpace (dataSize), d, (size_t) dataSize);
        */
    }
    
    /**
      | Creates a 1-byte short midi message.
      | 
      | -----------
      | @param byte1
      | 
      | message byte 1
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      |
      */
    pub fn new_1byte_short_message(
        byte1:      i32,
        time_stamp: Option<f64>

    ) -> Self {

        let time_stamp: f64 = time_stamp.unwrap_or(0.0);
    
        todo!();
        /*
        : time_stamp(time_stamp),
        : size(1),

            packedData.asBytes[0] = (uint8) byte1;

        // check that the length matches the data..
        jassert (byte1 >= 0xf0 || getMessageLengthFromFirstByte ((uint8) byte1) == 1);
        */
    }
    
    /**
      | Creates a 2-byte short midi message.
      | 
      | -----------
      | @param byte1
      | 
      | message byte 1
      | ----------
      | @param byte2
      | 
      | message byte 2
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      |
      */
    pub fn new_2byte_short_message(
        byte1:      i32,
        byte2:      i32,
        time_stamp: Option<f64>

    ) -> Self {

        let time_stamp: f64 = time_stamp.unwrap_or(0.0);
    
        todo!();
        /*
        : time_stamp(time_stamp),
        : size(2),

            packedData.asBytes[0] = (uint8) byte1;
        packedData.asBytes[1] = (uint8) byte2;

        // check that the length matches the data..
        jassert (byte1 >= 0xf0 || getMessageLengthFromFirstByte ((uint8) byte1) == 2);
        */
    }
    
    /**
      | Creates a 3-byte short midi message.
      | 
      | -----------
      | @param byte1
      | 
      | message byte 1
      | ----------
      | @param byte2
      | 
      | message byte 2
      | ----------
      | @param byte3
      | 
      | message byte 3
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      |
      */
    pub fn new_3byte_short_message(
        byte1:      i32,
        byte2:      i32,
        byte3:      i32,
        time_stamp: Option<f64>

    ) -> Self {

        let time_stamp: f64 = time_stamp.unwrap_or(0.0);
    
        todo!();
        /*
        : time_stamp(time_stamp),
        : size(3),

            packedData.asBytes[0] = (uint8) byte1;
        packedData.asBytes[1] = (uint8) byte2;
        packedData.asBytes[2] = (uint8) byte3;

        // check that the length matches the data..
        jassert (byte1 >= 0xf0 || getMessageLengthFromFirstByte ((uint8) byte1) == 3);
        */
    }
    
    /**
      | Creates a copy of another midi message.
      |
      */
    pub fn new_from_other_ref(other: &MidiMessage) -> Self {
    
        todo!();
        /*
        : time_stamp(other.timeStamp),
        : size(other.size),

            if (isHeapAllocated())
            memcpy (allocateSpace (size), other.getData(), (size_t) size);
        else
            packedData.allocatedData = other.packedData.allocatedData;
        */
    }
    
    /**
      | Creates a copy of another midi message,
      | with a different timestamp.
      |
      */
    pub fn new_from_other_with_different_timestamp(
        other:          &MidiMessage,
        new_time_stamp: f64) -> Self {
    
        todo!();
        /*
        : time_stamp(newTimeStamp),
        : size(other.size),

            if (isHeapAllocated())
            memcpy (allocateSpace (size), other.getData(), (size_t) size);
        else
            packedData.allocatedData = other.packedData.allocatedData;
        */
    }
    
    /**
      | Reads the next midi message from some
      | data.
      | 
      | -----------
      | @note
      | 
      | This will read as many bytes from a data
      | stream as it needs to make a
      | 
      | complete message, and will return the
      | number of bytes it used. This lets
      | 
      | you read a sequence of midi messages
      | from a file or stream.
      | 
      | -----------
      | @param data
      | 
      | the data to read from
      | ----------
      | @param maxBytesToUse
      | 
      | the maximum number of bytes it's allowed
      | to read
      | ----------
      | @param numBytesUsed
      | 
      | returns the number of bytes that were
      | actually needed
      | ----------
      | @param lastStatusByte
      | 
      | in a sequence of midi messages, the initial
      | byte can be dropped from a message if
      | it's the same as the first byte of the
      | previous message, so this lets you supply
      | the byte to use if the first byte of the
      | message has in fact been dropped.
      | ----------
      | @param timeStamp
      | 
      | the time to give the midi message - this
      | value doesn't use any particular units,
      | so will be application-specific
      | ----------
      | @param sysexHasEmbeddedLength
      | 
      | when reading sysexes, this flag indicates
      | whether to expect the data to begin with
      | a variable-length field indicating
      | its size
      |
      */
    pub fn new_from_midi_stream(
        src_data:                  *const c_void,
        sz:                        i32,
        num_bytes_used:            &mut i32,
        last_status_byte:          u8,
        time_stamp:                Option<f64>,
        sysex_has_embedded_length: Option<bool>

    ) -> Self {

        let time_stamp:                 f64 = time_stamp.unwrap_or(0.0);
        let sysex_has_embedded_length: bool = sysex_has_embedded_length.unwrap_or(true);
    
        todo!();
        /*
        : time_stamp(time_stamp),

            auto src = static_cast<const uint8*> (srcData);
        auto byte = (unsigned int) *src;

        if (byte < 0x80)
        {
            byte = (unsigned int) lastStatusByte;
            numBytesUsed = -1;
        }
        else
        {
            numBytesUsed = 0;
            --sz;
            ++src;
        }

        if (byte >= 0x80)
        {
            if (byte == 0xf0)
            {
                auto d = src;
                bool haveReadAllLengthBytes = ! sysexHasEmbeddedLength;
                int numVariableLengthSysexBytes = 0;

                while (d < src + sz)
                {
                    if (*d >= 0x80)
                    {
                        if (*d == 0xf7)
                        {
                            ++d;  // include the trailing 0xf7 when we hit it
                            break;
                        }

                        if (haveReadAllLengthBytes) // if we see a 0x80 bit set after the initial data length
                            break;                  // bytes, assume it's the end of the sysex

                        ++numVariableLengthSysexBytes;
                    }
                    else if (! haveReadAllLengthBytes)
                    {
                        haveReadAllLengthBytes = true;
                        ++numVariableLengthSysexBytes;
                    }

                    ++d;
                }

                src += numVariableLengthSysexBytes;
                size = 1 + (int) (d - src);

                auto dest = allocateSpace (size);
                *dest = (uint8) byte;
                memcpy (dest + 1, src, (size_t) (size - 1));

                numBytesUsed += (numVariableLengthSysexBytes + size);  // (these aren't counted in the size)
            }
            else if (byte == 0xff)
            {
                const auto bytesLeft = readVariableLengthValue (src + 1, sz - 1);
                size = jmin (sz + 1, bytesLeft.bytesUsed + 2 + bytesLeft.value);

                auto dest = allocateSpace (size);
                *dest = (uint8) byte;
                memcpy (dest + 1, src, (size_t) size - 1);

                numBytesUsed += size;
            }
            else
            {
                size = getMessageLengthFromFirstByte ((uint8) byte);
                packedData.asBytes[0] = (uint8) byte;

                if (size > 1)
                {
                    packedData.asBytes[1] = (sz > 0 ? src[0] : 0);

                    if (size > 2)
                        packedData.asBytes[2] = (sz > 1 ? src[1] : 0);
                }

                numBytesUsed += jmin (size, sz + 1);
            }
        }
        else
        {
            packedData.allocatedData = nullptr;
            size = 0;
        }
        */
    }
    
    /**
      | Copies this message from another one.
      |
      */
    pub fn assign_from_other_ref(&mut self, other: &MidiMessage) -> &mut MidiMessage {
        
        todo!();
        /*
            if (this != &other)
        {
            if (other.isHeapAllocated())
            {
                auto* newStorage = static_cast<uint8*> (isHeapAllocated()
                  ? std::realloc (packedData.allocatedData, (size_t) other.size)
                  : std::malloc ((size_t) other.size));

                if (newStorage == nullptr)
                    throw std::bad_alloc{}; // The midi message has not been adjusted at this point

                packedData.allocatedData = newStorage;
                memcpy (packedData.allocatedData, other.packedData.allocatedData, (size_t) other.size);
            }
            else
            {
                if (isHeapAllocated())
                    std::free (packedData.allocatedData);

                packedData.allocatedData = other.packedData.allocatedData;
            }

            timeStamp = other.timeStamp;
            size = other.size;
        }

        return *this;
        */
    }
    
    pub fn new_from_other(other: MidiMessage) -> Self {
    
        todo!();
        /*
        : time_stamp(other.timeStamp),
        : size(other.size),

            packedData.allocatedData = other.packedData.allocatedData;
        other.size = 0;
        */
    }
    
    pub fn assign_from(&mut self, other: MidiMessage) -> &mut MidiMessage {
        
        todo!();
        /*
            packedData.allocatedData = other.packedData.allocatedData;
        timeStamp = other.timeStamp;
        size = other.size;
        other.size = 0;
        return *this;
        */
    }
    
    pub fn allocate_space(&mut self, bytes: i32) -> *mut u8 {
        
        todo!();
        /*
            if (bytes > (int) sizeof (packedData))
        {
            auto d = static_cast<uint8*> (std::malloc ((size_t) bytes));
            packedData.allocatedData = d;
            return d;
        }

        return packedData.asBytes;
        */
    }
    
    /**
      | Returns a human-readable description
      | of the midi message as a string, for example
      | "Note On C#3 Velocity 120 Channel 1".
      |
      */
    pub fn get_description(&self) -> String {
        
        todo!();
        /*
            if (isNoteOn())           return "Note on "  + MidiMessage::getMidiNoteName (getNoteNumber(), true, true, 3) + " Velocity " + String (getVelocity()) + " Channel " + String (getChannel());
        if (isNoteOff())          return "Note off " + MidiMessage::getMidiNoteName (getNoteNumber(), true, true, 3) + " Velocity " + String (getVelocity()) + " Channel " + String (getChannel());
        if (isProgramChange())    return "Program change " + String (getProgramChangeNumber()) + " Channel " + String (getChannel());
        if (isPitchWheel())       return "Pitch wheel " + String (getPitchWheelValue()) + " Channel " + String (getChannel());
        if (isAftertouch())       return "Aftertouch " + MidiMessage::getMidiNoteName (getNoteNumber(), true, true, 3) +  ": " + String (getAfterTouchValue()) + " Channel " + String (getChannel());
        if (isChannelPressure())  return "Channel pressure " + String (getChannelPressureValue()) + " Channel " + String (getChannel());
        if (isAllNotesOff())      return "All notes off Channel " + String (getChannel());
        if (isAllSoundOff())      return "All sound off Channel " + String (getChannel());
        if (isMetaEvent())        return "Meta event";

        if (isController())
        {
            String name (MidiMessage::getControllerName (getControllerNumber()));

            if (name.isEmpty())
                name = String (getControllerNumber());

            return "Controller " + name + ": " + String (getControllerValue()) + " Channel " + String (getChannel());
        }

        return String::toHexString (getRawData(), getRawDataSize());
        */
    }
    
    /** 
      | Return a copy of this message with a new
      | timestamp.
      |
      | The units for the timestamp will be
      | application-specific - see the notes for
      | getTimeStamp().
      */
    pub fn with_time_stamp(&self, new_timestamp: f64) -> MidiMessage {
        
        todo!();
        /*
            return { *this, newTimestamp };
        */
    }
    
    /**
      | Returns the midi channel associated
      | with the message. @see isForChannel,
      | setChannel
      | 
      | -----------
      | @return
      | 
      | a value 1 to 16 if the message has a channel,
      | or 0 if it hasn't (e.g. if it's a sysex)
      |
      */
    pub fn get_channel(&self) -> i32 {
        
        todo!();
        /*
            auto data = getRawData();

        if ((data[0] & 0xf0) != 0xf0)
            return (data[0] & 0xf) + 1;

        return 0;
        */
    }
    
    /**
      | Returns true if the message applies
      | to the given midi channel. @see getChannel,
      | setChannel
      | 
      | -----------
      | @param channelNumber
      | 
      | the channel number to look for, in the
      | range 1 to 16
      |
      */
    pub fn is_for_channel(&self, channel: i32) -> bool {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16); // valid channels are numbered 1 to 16

        auto data = getRawData();

        return ((data[0] & 0xf) == channel - 1)
                 && ((data[0] & 0xf0) != 0xf0);
        */
    }
    
    /**
      | Changes the message's midi channel.
      | This won't do anything for non-channel
      | messages like sysexes.
      | 
      | -----------
      | @param newChannelNumber
      | 
      | the channel number to change it to, in
      | the range 1 to 16
      |
      */
    pub fn set_channel(&mut self, channel: i32)  {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16); // valid channels are numbered 1 to 16

        auto data = getData();

        if ((data[0] & 0xf0) != (uint8) 0xf0)
            data[0] = (uint8) ((data[0] & (uint8) 0xf0)
                                | (uint8)(channel - 1));
        */
    }
    
    /**
      | Returns true if this message is a 'key-down'
      | event. 
      |
      | @see isNoteOff, getNoteNumber, getVelocity, noteOn
      | 
      | -----------
      | @param returnTrueForVelocity0
      | 
      | if true, then if this event is a note-on
      | with velocity 0, it will still be considered
      | to be a note-on and the method will return
      | true. If returnTrueForVelocity0 is
      | false, then if this is a note-on event
      | with velocity 0, it'll be regarded as
      | a note-off, and the method will return
      | false
      |
      */
    pub fn is_note_on(&self, return_true_for_velocity0: Option<bool>) -> bool {

        let return_true_for_velocity0: bool = return_true_for_velocity0.unwrap_or(false);
        
        todo!();
        /*
            auto data = getRawData();

        return ((data[0] & 0xf0) == 0x90)
                 && (returnTrueForVelocity0 || data[2] != 0);
        */
    }
    
    /**
      | Returns true if this message is a 'key-up'
      | event.
      | 
      | -----------
      | @note
      | 
      | If returnTrueForNoteOnVelocity0
      | is true, then his will also return true
      | 
      | for a note-on event with a velocity of 0.
      | 
      | @see isNoteOn, getNoteNumber, getVelocity,
      | noteOff
      |
      */
    pub fn is_note_off(&self, return_true_for_note_on_velocity0: Option<bool>) -> bool {

        let return_true_for_note_on_velocity0: bool = return_true_for_note_on_velocity0.unwrap_or(true);
        
        todo!();
        /*
            auto data = getRawData();

        return ((data[0] & 0xf0) == 0x80)
                || (returnTrueForNoteOnVelocity0 && (data[2] == 0) && ((data[0] & 0xf0) == 0x90));
        */
    }
    
    /**
      | Returns true if this message is a 'key-down'
      | or 'key-up' event. @see isNoteOn, isNoteOff
      |
      */
    pub fn is_note_on_or_off(&self) -> bool {
        
        todo!();
        /*
            auto d = getRawData()[0] & 0xf0;
        return (d == 0x90) || (d == 0x80);
        */
    }
    
    /**
      | Returns the midi note number for note-on
      | and note-off messages. If the message
      | isn't a note-on or off, the value returned
      | is undefined. @see isNoteOff, getMidiNoteName,
      | getMidiNoteInHertz, setNoteNumber
      |
      */
    pub fn get_note_number(&self) -> i32 {
        
        todo!();
        /*
            return getRawData()[1];
        */
    }
    
    /**
      | Changes the midi note number of a note-on
      | or note-off message. If the message
      | isn't a note on or off, this will do nothing.
      |
      */
    pub fn set_note_number(&mut self, new_note_number: i32)  {
        
        todo!();
        /*
            if (isNoteOnOrOff() || isAftertouch())
            getData()[1] = (uint8) (newNoteNumber & 127);
        */
    }
    
    /**
      | Returns the velocity of a note-on or
      | note-off message.
      | 
      | -----------
      | @note
      | 
      | The value returned will be in the range
      | 0 to 127.
      | 
      | If the message isn't a note-on or off
      | event, it will return 0.
      | 
      | @see getFloatVelocity
      |
      */
    pub fn get_velocity(&self) -> u8 {
        
        todo!();
        /*
            if (isNoteOnOrOff())
            return getRawData()[2];

        return 0;
        */
    }
    
    /**
      | Returns the velocity of a note-on or
      | note-off message.
      | 
      | -----------
      | @note
      | 
      | The value returned will be in the range
      | 0 to 1.0
      | 
      | If the message isn't a note-on or off
      | event, it will return 0.
      | 
      | @see getVelocity, setVelocity
      |
      */
    pub fn get_float_velocity(&self) -> f32 {
        
        todo!();
        /*
            return getVelocity() * (1.0f / 127.0f);
        */
    }
    
    /**
      | Changes the velocity of a note-on or
      | note-off message.
      | 
      | -----------
      | @note
      | 
      | If the message isn't a note on or off,
      | this will do nothing.
      | 
      | -----------
      | @param newVelocity
      | 
      | the new velocity, in the range 0 to 1.0
      | 
      | @see getFloatVelocity, multiplyVelocity
      |
      */
    pub fn set_velocity(&mut self, new_velocity: f32)  {
        
        todo!();
        /*
            if (isNoteOnOrOff())
            getData()[2] = floatValueToMidiByte (newVelocity);
        */
    }
    
    /**
      | Multiplies the velocity of a note-on
      | or note-off message by a given amount.
      | 
      | -----------
      | @note
      | 
      | If the message isn't a note on or off,
      | this will do nothing.
      | 
      | -----------
      | @param scaleFactor
      | 
      | the value by which to multiply the velocity
      | 
      | @see setVelocity
      |
      */
    pub fn multiply_velocity(&mut self, scale_factor: f32)  {
        
        todo!();
        /*
            if (isNoteOnOrOff())
        {
            auto data = getData();
            data[2] = MidiHelpers::validVelocity (roundToInt (scaleFactor * data[2]));
        }
        */
    }
    
    /**
      | Returns true if the message is an aftertouch
      | event.
      | 
      | -----------
      | @note
      | 
      | For aftertouch events, use the getNoteNumber()
      | method to find out the key
      | 
      | that it applies to, and getAfterTouchValue()
      | to find out the amount. Use
      | 
      | getChannel() to find out the channel.
      | 
      | @see getAftertouchValue, getNoteNumber
      |
      */
    pub fn is_aftertouch(&self) -> bool {
        
        todo!();
        /*
            return (getRawData()[0] & 0xf0) == 0xa0;
        */
    }
    
    /**
      | Returns the amount of aftertouch from
      | an aftertouch messages.
      | 
      | -----------
      | @note
      | 
      | The value returned is in the range 0 to
      | 127, and will be nonsense for messages
      | other than aftertouch messages.
      | 
      | @see isAftertouch
      |
      */
    pub fn get_after_touch_value(&self) -> i32 {
        
        todo!();
        /*
            jassert (isAftertouch());
        return getRawData()[2];
        */
    }
    
    /**
      | Creates an aftertouch message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param aftertouchAmount
      | 
      | the amount of aftertouch, 0 to 127
      | 
      | @see isAftertouch
      |
      */
    pub fn aftertouch_change(&mut self, 
        channel:          i32,
        note_num:         i32,
        aftertouch_value: i32) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16); // valid channels are numbered 1 to 16
        jassert (isPositiveAndBelow (noteNum, 128));
        jassert (isPositiveAndBelow (aftertouchValue, 128));

        return MidiMessage (MidiHelpers::initialByte (0xa0, channel),
                            noteNum & 0x7f,
                            aftertouchValue & 0x7f);
        */
    }
    
    /**
      | Returns true if the message is a channel-pressure
      | change event.
      | 
      | -----------
      | @note
      | 
      | This is like aftertouch, but common
      | to the whole channel rather than a specific
      | 
      | note. Use getChannelPressureValue()
      | to find out the pressure, and getChannel()
      | 
      | to find out the channel.
      | 
      | @see channelPressureChange
      |
      */
    pub fn is_channel_pressure(&self) -> bool {
        
        todo!();
        /*
            return (getRawData()[0] & 0xf0) == 0xd0;
        */
    }
    
    /**
      | Returns the pressure from a channel
      | pressure change message.
      | 
      | -----------
      | @return
      | 
      | the pressure, in the range 0 to 127
      | 
      | @see isChannelPressure, channelPressureChange
      |
      */
    pub fn get_channel_pressure_value(&self) -> i32 {
        
        todo!();
        /*
            jassert (isChannelPressure());
        return getRawData()[1];
        */
    }
    
    /**
      | Creates a channel-pressure change
      | event.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel: 1 to 16
      | ----------
      | @param pressure
      | 
      | the pressure, 0 to 127
      | 
      | @see isChannelPressure
      |
      */
    pub fn channel_pressure_change(&mut self, 
        channel:  i32,
        pressure: i32) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16); // valid channels are numbered 1 to 16
        jassert (isPositiveAndBelow (pressure, 128));

        return MidiMessage (MidiHelpers::initialByte (0xd0, channel), pressure & 0x7f);
        */
    }
    
    /**
      | Returns true if this message is a 'sustain
      | pedal down' controller message.
      |
      */
    pub fn is_sustain_pedal_on(&self) -> bool {
        
        todo!();
        /*
            return isControllerOfType (0x40) && getRawData()[2] >= 64;
        */
    }
    
    /**
      | Returns true if this message is a 'sustain
      | pedal up' controller message.
      |
      */
    pub fn is_sustain_pedal_off(&self) -> bool {
        
        todo!();
        /*
            return isControllerOfType (0x40) && getRawData()[2] <  64;
        */
    }
    
    /**
      | Returns true if this message is a 'sostenuto
      | pedal down' controller message.
      |
      */
    pub fn is_sostenuto_pedal_on(&self) -> bool {
        
        todo!();
        /*
            return isControllerOfType (0x42) && getRawData()[2] >= 64;
        */
    }
    
    /**
      | Returns true if this message is a 'sostenuto
      | pedal up' controller message.
      |
      */
    pub fn is_sostenuto_pedal_off(&self) -> bool {
        
        todo!();
        /*
            return isControllerOfType (0x42) && getRawData()[2] <  64;
        */
    }
    
    /**
      | Returns true if this message is a 'soft
      | pedal down' controller message.
      |
      */
    pub fn is_soft_pedal_on(&self) -> bool {
        
        todo!();
        /*
            return isControllerOfType (0x43) && getRawData()[2] >= 64;
        */
    }
    
    /**
      | Returns true if this message is a 'soft
      | pedal up' controller message.
      |
      */
    pub fn is_soft_pedal_off(&self) -> bool {
        
        todo!();
        /*
            return isControllerOfType (0x43) && getRawData()[2] <  64;
        */
    }
    
    /** 
      | Returns true if the message is a program
      | (patch) change message.
      | @see getProgramChangeNumber,
      | getGMInstrumentName
      */
    pub fn is_program_change(&self) -> bool {
        
        todo!();
        /*
            return (getRawData()[0] & 0xf0) == 0xc0;
        */
    }
    
    /** 
     | Returns the new program number of a program
     | change message.  If the message isn't
     | a program change, the value returned is
     | undefined.  @see isProgramChange,
     | getGMInstrumentName
    */
    pub fn get_program_change_number(&self) -> i32 {
        
        todo!();
        /*
            jassert (isProgramChange());
        return getRawData()[1];
        */
    }
    
    /**
      | Creates a program-change message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param programNumber
      | 
      | the midi program number, 0 to 127
      | 
      | @see isProgramChange, getGMInstrumentName
      |
      */
    pub fn program_change(&mut self, 
        channel:        i32,
        program_number: i32) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16); // valid channels are numbered 1 to 16

        return MidiMessage (MidiHelpers::initialByte (0xc0, channel), programNumber & 0x7f);
        */
    }
    
    /**
      | Returns true if the message is a pitch-wheel
      | move. @see getPitchWheelValue, pitchWheel
      |
      */
    pub fn is_pitch_wheel(&self) -> bool {
        
        todo!();
        /*
            return (getRawData()[0] & 0xf0) == 0xe0;
        */
    }
    
    /**
      | Returns the pitch wheel position from
      | a pitch-wheel move message.
      | 
      | -----------
      | @note
      | 
      | The value returned is a 14-bit number
      | from 0 to 0x3fff, indicating the wheel
      | position.
      | 
      | If called for messages which aren't
      | pitch wheel events, the number returned
      | will be
      | 
      | nonsense.
      | 
      | @see isPitchWheel
      |
      */
    pub fn get_pitch_wheel_value(&self) -> i32 {
        
        todo!();
        /*
            jassert (isPitchWheel());
        auto data = getRawData();
        return data[1] | (data[2] << 7);
        */
    }
    
    /**
      | Creates a pitch-wheel move message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param position
      | 
      | the wheel position, in the range 0 to
      | 16383
      | 
      | @see isPitchWheel
      |
      */
    pub fn pitch_wheel(&mut self, 
        channel:  i32,
        position: i32) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16); // valid channels are numbered 1 to 16
        jassert (isPositiveAndBelow (position, 0x4000));

        return MidiMessage (MidiHelpers::initialByte (0xe0, channel),
                            position & 127, (position >> 7) & 127);
        */
    }
    
    /**
      | Returns true if this is a midi controller
      | message. @see getControllerNumber,
      | getControllerValue, controllerEvent
      |
      */
    pub fn is_controller(&self) -> bool {
        
        todo!();
        /*
            return (getRawData()[0] & 0xf0) == 0xb0;
        */
    }
    
    /** 
      | Returns true if this message is
      | a controller message and if it has the
      | specified
      | controller type.
      */
    pub fn is_controller_of_type(&self, controller_type: i32) -> bool {
        
        todo!();
        /*
            auto data = getRawData();
        return (data[0] & 0xf0) == 0xb0 && data[1] == controllerType;
        */
    }
    
    /**
      | Returns the controller number of a controller
      | message.
      | 
      | -----------
      | @note
      | 
      | The name of the controller can be looked
      | up using the getControllerName() method.
      | 
      | Note that the value returned is invalid
      | for messages that aren't controller
      | changes.
      | 
      | @see isController, getControllerName,
      | getControllerValue
      |
      */
    pub fn get_controller_number(&self) -> i32 {
        
        todo!();
        /*
            jassert (isController());
        return getRawData()[1];
        */
    }
    
    /**
      | Returns the controller value from a
      | controller message.
      | 
      | -----------
      | @note
      | 
      | A value 0 to 127 is returned to indicate
      | the new controller position.
      | 
      | Note that the value returned is invalid
      | for messages that aren't controller
      | changes.
      | 
      | @see isController, getControllerNumber
      |
      */
    pub fn get_controller_value(&self) -> i32 {
        
        todo!();
        /*
            jassert (isController());
        return getRawData()[2];
        */
    }
    
    /**
      | Creates a controller message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param controllerType
      | 
      | the type of controller
      | ----------
      | @param value
      | 
      | the controller value
      | 
      | @see isController
      |
      */
    pub fn controller_event(&mut self, 
        channel:         i32,
        controller_type: i32,
        value:           i32) -> MidiMessage {
        
        todo!();
        /*
            // the channel must be between 1 and 16 inclusive
        jassert (channel > 0 && channel <= 16);

        return MidiMessage (MidiHelpers::initialByte (0xb0, channel),
                            controllerType & 127, value & 127);
        */
    }
    
    /**
      | Creates a key-down message (using an
      | integer velocity).
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 127
      | 
      | @see isNoteOn
      |
      */
    pub fn note_on(&mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    u8) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16);
        jassert (isPositiveAndBelow (noteNumber, 128));

        return MidiMessage (MidiHelpers::initialByte (0x90, channel),
                            noteNumber & 127, MidiHelpers::validVelocity (velocity));
        */
    }
    
    /**
      | Creates a key-down message (using a
      | floating-point velocity).
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 1.0
      | 
      | @see isNoteOn
      |
      */
    pub fn note_on_with_floating_point_velocity(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    f32

    ) -> MidiMessage {
        
        todo!();
        /*
            return noteOn (channel, noteNumber, floatValueToMidiByte (velocity));
        */
    }
    
    /**
      | Creates a key-up message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 127
      | 
      | @see isNoteOff
      |
      */
    pub fn note_off_with_u8_velocity(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    u8
    ) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16);
        jassert (isPositiveAndBelow (noteNumber, 128));

        return MidiMessage (MidiHelpers::initialByte (0x80, channel),
                            noteNumber & 127, MidiHelpers::validVelocity (velocity));
        */
    }
    
    /**
      | Creates a key-up message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | ----------
      | @param velocity
      | 
      | in the range 0 to 1.0
      | 
      | @see isNoteOff
      |
      */
    pub fn note_off_with_f32_velocity(
        &mut self, 
        channel:     i32,
        note_number: i32,
        velocity:    f32

    ) -> MidiMessage {
        
        todo!();
        /*
            return noteOff (channel, noteNumber, floatValueToMidiByte (velocity));
        */
    }
    
    /**
      | Creates a key-up message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param noteNumber
      | 
      | the key number, 0 to 127
      | 
      | @see isNoteOff
      |
      */
    pub fn note_off(&mut self, 
        channel:     i32,
        note_number: i32) -> MidiMessage {
        
        todo!();
        /*
            jassert (channel > 0 && channel <= 16);
        jassert (isPositiveAndBelow (noteNumber, 128));

        return MidiMessage (MidiHelpers::initialByte (0x80, channel), noteNumber & 127, 0);
        */
    }
    
    /**
      | Creates an all-notes-off message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | 
      | @see isAllNotesOff
      |
      */
    pub fn all_notes_off(&mut self, channel: i32) -> MidiMessage {
        
        todo!();
        /*
            return controllerEvent (channel, 123, 0);
        */
    }
    
    /**
      | Checks whether this message is an all-notes-off
      | message. @see allNotesOff
      |
      */
    pub fn is_all_notes_off(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData();
        return (data[0] & 0xf0) == 0xb0 && data[1] == 123;
        */
    }
    
    /**
      | Creates an all-sound-off message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | 
      | @see isAllSoundOff
      |
      */
    pub fn all_sound_off(&mut self, channel: i32) -> MidiMessage {
        
        todo!();
        /*
            return controllerEvent (channel, 120, 0);
        */
    }
    
    /** 
      | Checks whether this message is an
      | all-sound-off message.
      | @see allSoundOff
      */
    pub fn is_all_sound_off(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData();
        return data[1] == 120 && (data[0] & 0xf0) == 0xb0;
        */
    }
    
    /**
      | Checks whether this message is a reset
      | all controllers message. @see allControllerOff
      |
      */
    pub fn is_reset_all_controllers(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData();
        return (data[0] & 0xf0) == 0xb0 && data[1] == 121;
        */
    }
    
    /**
      | Creates an all-controllers-off message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      |
      */
    pub fn all_controllers_off(&mut self, channel: i32) -> MidiMessage {
        
        todo!();
        /*
            return controllerEvent (channel, 121, 0);
        */
    }
    
    /**
      | Creates a master-volume change message.
      | 
      | -----------
      | @param volume
      | 
      | the volume, 0 to 1.0
      |
      */
    pub fn master_volume(&mut self, volume: f32) -> MidiMessage {
        
        todo!();
        /*
            auto vol = jlimit (0, 0x3fff, roundToInt (volume * 0x4000));

        return { 0xf0, 0x7f, 0x7f, 0x04, 0x01, vol & 0x7f, vol >> 7, 0xf7 };
        */
    }
    
    /**
      | Returns true if this is a system-exclusive
      | message.
      |
      */
    pub fn is_sys_ex(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xf0;
        */
    }
    
    /**
      | Creates a system-exclusive message.
      | The data passed in is wrapped with header
      | and tail bytes of 0xf0 and 0xf7.
      |
      */
    pub fn create_sys_ex_message(&mut self, 
        sysex_data: *const c_void,
        data_size:  i32) -> MidiMessage {
        
        todo!();
        /*
            HeapBlock<uint8> m (dataSize + 2);

        m[0] = 0xf0;
        memcpy (m + 1, sysexData, (size_t) dataSize);
        m[dataSize + 1] = 0xf7;

        return MidiMessage (m, dataSize + 2);
        */
    }
    
    /**
      | Returns a pointer to the sysex data inside
      | the message. If this event isn't a sysex
      | event, it'll return 0. @see getSysExDataSize
      |
      */
    pub fn get_sys_ex_data(&self) -> *const u8 {
        
        todo!();
        /*
            return isSysEx() ? getRawData() + 1 : nullptr;
        */
    }
    
    /**
      | Returns the size of the sysex data. This
      | value excludes the 0xf0 header byte
      | and the 0xf7 at the end. @see getSysExData
      |
      */
    pub fn get_sys_ex_data_size(&self) -> i32 {
        
        todo!();
        /*
            return isSysEx() ? size - 2 : 0;
        */
    }
    
    /**
      | Returns true if this event is a meta-event.
      | 
      | -----------
      | @note
      | 
      | Meta-events are things like tempo changes,
      | track names, etc.
      | 
      | @see getMetaEventType, isTrackMetaEvent,
      | isEndOfTrackMetaEvent, isTextMetaEvent, 
      | isTrackNameEvent, isTempoMetaEvent, 
      | isTimeSignatureMetaEvent,
      | 
      | isKeySignatureMetaEvent, isMidiChannelMetaEvent
      |
      */
    pub fn is_meta_event(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xff;
        */
    }
    
    /**
      | Returns true if this is an active-sense
      | message.
      |
      */
    pub fn is_active_sense(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xfe;
        */
    }
    
    /** 
      | Returns a meta-event's type number.
      |
      | If the message isn't a meta-event, this
      | will return -1.
      |
      | @see isMetaEvent, isTrackMetaEvent,
      | isEndOfTrackMetaEvent,
      | isTextMetaEvent, isTrackNameEvent,
      | isTempoMetaEvent,
      | isTimeSignatureMetaEvent,
      | isKeySignatureMetaEvent,
      | isMidiChannelMetaEvent
      */
    pub fn get_meta_event_type(&self) -> i32 {
        
        todo!();
        /*
            auto data = getRawData();
        return (size < 2 || *data != 0xff) ? -1 : data[1];
        */
    }
    
    /**
      | Returns the length of the data for a meta-event.
      | @see isMetaEvent, getMetaEventData
      |
      */
    pub fn get_meta_event_length(&self) -> i32 {
        
        todo!();
        /*
            auto data = getRawData();

        if (*data == 0xff)
        {
            const auto var = readVariableLengthValue (data + 2, size - 2);
            return jmax (0, jmin (size - 2 - var.bytesUsed, var.value));
        }

        return 0;
        */
    }
    
    /**
      | Returns a pointer to the data in a meta-event.
      | @see isMetaEvent, getMetaEventLength
      */
    pub fn get_meta_event_data(&self) -> *const u8 {
        
        todo!();
        /*
            jassert (isMetaEvent());

        auto d = getRawData() + 2;
        const auto var = readVariableLengthValue (d, size - 2);
        return d + var.bytesUsed;
        */
    }
    
    /**
      | Returns true if this is a 'track' meta-event.
      |
      */
    pub fn is_track_meta_event(&self) -> bool {
        
        todo!();
        /*
            return getMetaEventType() == 0;
        */
    }
    
    /**
      | Returns true if this is an 'end-of-track'
      | meta-event.
      |
      */
    pub fn is_end_of_track_meta_event(&self) -> bool {
        
        todo!();
        /*
            return getMetaEventType() == 47;
        */
    }
    
    /**
      | Returns true if this is a 'text' meta-event.
      | @see getTextFromTextMetaEvent
      |
      */
    pub fn is_text_meta_event(&self) -> bool {
        
        todo!();
        /*
            auto t = getMetaEventType();
        return t > 0 && t < 16;
        */
    }
    
    /**
      | Returns the text from a text meta-event.
      | @see isTextMetaEvent
      |
      */
    pub fn get_text_from_text_meta_event(&self) -> String {
        
        todo!();
        /*
            auto textData = reinterpret_cast<const char*> (getMetaEventData());

        return String (CharPointer_UTF8 (textData),
                       CharPointer_UTF8 (textData + getMetaEventLength()));
        */
    }
    
    /**
      | Creates a text meta-event.
      |
      */
    pub fn text_meta_event(&mut self, 
        ty:   i32,
        text: &str) -> MidiMessage {
        
        todo!();
        /*
            jassert (type > 0 && type < 16);

        MidiMessage result;

        const size_t textSize = text.text.sizeInBytes() - 1;

        uint8 header[8];
        size_t n = sizeof (header);

        header[--n] = (uint8) (textSize & 0x7f);

        for (size_t i = textSize; (i >>= 7) != 0;)
            header[--n] = (uint8) ((i & 0x7f) | 0x80);

        header[--n] = (uint8) type;
        header[--n] = 0xff;

        const size_t headerLen = sizeof (header) - n;
        const int totalSize = (int) (headerLen + textSize);

        auto dest = result.allocateSpace (totalSize);
        result.size = totalSize;

        memcpy (dest, header + n, headerLen);
        memcpy (dest + headerLen, text.text.getAddress(), textSize);

        return result;
        */
    }
    
    /**
      | Returns true if this is an 'track name'
      | meta-event. You can use the getTextFromTextMetaEvent()
      | method to get the track's name.
      |
      */
    pub fn is_track_name_event(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData(); return (data[1] == 3)    && (*data == 0xff);
        */
    }
    
    /**
      | Returns true if this is a 'tempo' meta-event.
      | @see getTempoMetaEventTickLength,
      | getTempoSecondsPerQuarterNote
      |
      */
    pub fn is_tempo_meta_event(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData(); return (data[1] == 81)   && (*data == 0xff);
        */
    }
    
    /**
      | Returns true if this is a 'channel' meta-event.
      | 
      | -----------
      | @note
      | 
      | A channel meta-event specifies the
      | midi channel that should be used
      | 
      | for subsequent meta-events.
      | 
      | @see getMidiChannelMetaEventChannel
      |
      */
    pub fn is_midi_channel_meta_event(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData(); return (data[1] == 0x20) && (*data == 0xff) && (data[2] == 1);
        */
    }
    
    /**
      | Returns the channel number from a channel
      | meta-event.
      | 
      | -----------
      | @return
      | 
      | the channel, in the range 1 to 16.
      | 
      | @see isMidiChannelMetaEvent
      |
      */
    pub fn get_midi_channel_meta_event_channel(&self) -> i32 {
        
        todo!();
        /*
            jassert (isMidiChannelMetaEvent());
        return getRawData()[3] + 1;
        */
    }
    
    /**
      | Calculates the seconds-per-quarter-note
      | from a tempo meta-event. @see isTempoMetaEvent,
      | getTempoMetaEventTickLength
      |
      */
    pub fn get_tempo_seconds_per_quarter_note(&self) -> f64 {
        
        todo!();
        /*
            if (! isTempoMetaEvent())
            return 0.0;

        auto d = getMetaEventData();

        return (((unsigned int) d[0] << 16)
                 | ((unsigned int) d[1] << 8)
                 | d[2])
                / 1000000.0;
        */
    }
    
    /**
      | Returns the tick length from a tempo
      | meta-event.
      | 
      | -----------
      | @param timeFormat
      | 
      | the 16-bit time format value from the
      | midi file's header.
      | 
      | -----------
      | @return
      | 
      | the tick length (in seconds).
      | 
      | @see isTempoMetaEvent
      |
      */
    pub fn get_tempo_meta_event_tick_length(&self, time_format: i16) -> f64 {
        
        todo!();
        /*
            if (timeFormat > 0)
        {
            if (! isTempoMetaEvent())
                return 0.5 / timeFormat;

            return getTempoSecondsPerQuarterNote() / timeFormat;
        }

        const int frameCode = (-timeFormat) >> 8;
        double framesPerSecond;

        switch (frameCode)
        {
            case 24: framesPerSecond = 24.0;   break;
            case 25: framesPerSecond = 25.0;   break;
            case 29: framesPerSecond = 30.0 * 1000.0 / 1001.0;  break;
            case 30: framesPerSecond = 30.0;   break;
            default: framesPerSecond = 30.0;   break;
        }

        return (1.0 / framesPerSecond) / (timeFormat & 0xff);
        */
    }
    
    /**
      | Creates a tempo meta-event. @see isTempoMetaEvent
      |
      */
    pub fn tempo_meta_event(&mut self, microseconds_per_quarter_note: i32) -> MidiMessage {
        
        todo!();
        /*
            return { 0xff, 81, 3,
                 (uint8) (microsecondsPerQuarterNote >> 16),
                 (uint8) (microsecondsPerQuarterNote >> 8),
                 (uint8) microsecondsPerQuarterNote };
        */
    }
    
    /**
      | Returns true if this is a 'time-signature'
      | meta-event. @see getTimeSignatureInfo
      |
      */
    pub fn is_time_signature_meta_event(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData();
        return (data[1] == 0x58) && (*data == (uint8) 0xff);
        */
    }
    
    /**
      | Returns the time-signature values
      | from a time-signature meta-event.
      | @see isTimeSignatureMetaEvent
      |
      */
    pub fn get_time_signature_info(&self, 
        numerator:   &mut i32,
        denominator: &mut i32)  {
        
        todo!();
        /*
            if (isTimeSignatureMetaEvent())
        {
            auto d = getMetaEventData();
            numerator = d[0];
            denominator = 1 << d[1];
        }
        else
        {
            numerator = 4;
            denominator = 4;
        }
        */
    }
    
    /**
      | Creates a time-signature meta-event.
      | @see isTimeSignatureMetaEvent
      |
      */
    pub fn time_signature_meta_event(&mut self, 
        numerator:   i32,
        denominator: i32) -> MidiMessage {
        
        todo!();
        /*
            int n = 1;
        int powerOfTwo = 0;

        while (n < denominator)
        {
            n <<= 1;
            ++powerOfTwo;
        }

        return { 0xff, 0x58, 0x04, numerator, powerOfTwo, 1, 96 };
        */
    }
    
    /**
      | Creates a midi channel meta-event.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | 
      | @see isMidiChannelMetaEvent
      |
      */
    pub fn midi_channel_meta_event(&mut self, channel: i32) -> MidiMessage {
        
        todo!();
        /*
            return { 0xff, 0x20, 0x01, jlimit (0, 0xff, channel - 1) };
        */
    }
    
    /**
      | Returns true if this is a 'key-signature'
      | meta-event. @see getKeySignatureNumberOfSharpsOrFlats,
      | isKeySignatureMajorKey
      |
      */
    pub fn is_key_signature_meta_event(&self) -> bool {
        
        todo!();
        /*
            return getMetaEventType() == 0x59;
        */
    }
    
    /** 
      | Returns the key from a key-signature
      | meta-event.
      |
      | This method must only be called if
      | isKeySignatureMetaEvent() is true.
      |
      | A positive number here indicates the
      | number of sharps in the key signature, and
      | a negative number indicates a number of
      | flats. So e.g. 3 = F# + C# + G#, -2 = Bb
      | + Eb
      |
      | @see isKeySignatureMetaEvent,
      | isKeySignatureMajorKey
    */
    pub fn get_key_signature_number_of_sharps_or_flats(&self) -> i32 {
        
        todo!();
        /*
            return (int) (int8) getMetaEventData()[0];
        */
    }
    
    /**
      | Returns true if this key-signature
      | event is major, or false if it's minor.
      | This method must only be called if isKeySignatureMetaEvent()
      | is true.
      |
      */
    pub fn is_key_signature_major_key(&self) -> bool {
        
        todo!();
        /*
            return getMetaEventData()[1] == 0;
        */
    }
    
    /**
      | Creates a key-signature meta-event.
      | 
      | -----------
      | @param numberOfSharpsOrFlats
      | 
      | if positive, this indicates the number
      | of sharps in the key; if negative, the
      | number of flats
      | ----------
      | @param isMinorKey
      | 
      | if true, the key is minor; if false, it
      | is major
      | 
      | @see isKeySignatureMetaEvent
      |
      */
    pub fn key_signature_meta_event(&mut self, 
        number_of_sharps_or_flats: i32,
        is_minor_key:              bool) -> MidiMessage {
        
        todo!();
        /*
            jassert (numberOfSharpsOrFlats >= -7 && numberOfSharpsOrFlats <= 7);

        return { 0xff, 0x59, 0x02, numberOfSharpsOrFlats, isMinorKey ? 1 : 0 };
        */
    }
    
    /**
      | Creates an end-of-track meta-event.
      | @see isEndOfTrackMetaEvent
      |
      */
    pub fn end_of_track(&mut self) -> MidiMessage {
        
        todo!();
        /*
            return { 0xff, 0x2f, 0x00 };
        */
    }
    
    /**
      | Returns true if this is a song-position-pointer
      | message. @see getSongPositionPointerMidiBeat,
      | songPositionPointer
      */
    pub fn is_song_position_pointer(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xf2;
        */
    }
    
    /**
      | Returns the midi beat-number of a song-position-pointer
      | message. @see isSongPositionPointer,
      | songPositionPointer
      |
      */
    pub fn get_song_position_pointer_midi_beat(&self) -> i32 {
        
        todo!();
        /*
            auto data = getRawData(); return data[1] | (data[2] << 7);
        */
    }
    
    /**
      | Creates a song-position-pointer message.
      | 
      | -----------
      | @note
      | 
      | The position is a number of midi beats
      | from the start of the song, where 1 midi
      | beat is 6 midi clocks, and there are 24
      | midi clocks in a quarter-note. So there
      | are 4 midi beats in a quarter-note.
      | 
      | @see isSongPositionPointer, getSongPositionPointerMidiBeat
      |
      */
    pub fn song_position_pointer(&mut self, position_in_midi_beats: i32) -> MidiMessage {
        
        todo!();
        /*
            return { 0xf2,
                 positionInMidiBeats & 127,
                 (positionInMidiBeats >> 7) & 127 };
        */
    }
    
    /**
      | Returns true if this is a midi start event.
      | @see midiStart
      |
      */
    pub fn is_midi_start(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xfa;
        */
    }
    
    /**
      | Creates a midi start event.
      |
      */
    pub fn midi_start(&mut self) -> MidiMessage {
        
        todo!();
        /*
            return MidiMessage (0xfa);
        */
    }
    
    /**
      | Returns true if this is a midi continue
      | event. @see midiContinue
      |
      */
    pub fn is_midi_continue(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xfb;
        */
    }
    
    /**
      | Creates a midi continue event.
      |
      */
    pub fn midi_continue(&mut self) -> MidiMessage {
        
        todo!();
        /*
            return MidiMessage (0xfb);
        */
    }
    
    /**
      | Returns true if this is a midi stop event.
      | @see midiStop
      |
      */
    pub fn is_midi_stop(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xfc;
        */
    }
    
    /**
      | Creates a midi stop event.
      |
      */
    pub fn midi_stop(&mut self) -> MidiMessage {
        
        todo!();
        /*
            return MidiMessage (0xfc);
        */
    }
    
    /**
      | Returns true if this is a midi clock event.
      | @see midiClock, songPositionPointer
      |
      */
    pub fn is_midi_clock(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xf8;
        */
    }
    
    /**
      | Creates a midi clock event.
      |
      */
    pub fn midi_clock(&mut self) -> MidiMessage {
        
        todo!();
        /*
            return MidiMessage (0xf8);
        */
    }
    
    /**
      | Returns true if this is a quarter-frame
      | midi timecode message. @see quarterFrame,
      | getQuarterFrameSequenceNumber,
      | getQuarterFrameValue
      |
      */
    pub fn is_quarter_frame(&self) -> bool {
        
        todo!();
        /*
            return *getRawData() == 0xf1;
        */
    }
    
    /**
      | Returns the sequence number of a quarter-frame
      | midi timecode message. This will be
      | a value between 0 and 7. @see isQuarterFrame,
      | getQuarterFrameValue, quarterFrame
      |
      */
    pub fn get_quarter_frame_sequence_number(&self) -> i32 {
        
        todo!();
        /*
            return ((int) getRawData()[1]) >> 4;
        */
    }
    
    /**
      | Returns the value from a quarter-frame
      | message. This will be the lower nybble
      | of the message's data-byte, a value
      | between 0 and 15
      |
      */
    pub fn get_quarter_frame_value(&self) -> i32 {
        
        todo!();
        /*
            return ((int) getRawData()[1]) & 0x0f;
        */
    }
    
    /**
      | Creates a quarter-frame MTC message.
      | 
      | -----------
      | @param sequenceNumber
      | 
      | a value 0 to 7 for the upper nybble of the
      | message's data byte
      | ----------
      | @param value
      | 
      | a value 0 to 15 for the lower nybble of
      | the message's data byte
      |
      */
    pub fn quarter_frame(&mut self, 
        sequence_number: i32,
        value:           i32) -> MidiMessage {
        
        todo!();
        /*
            return MidiMessage (0xf1, (sequenceNumber << 4) | value);
        */
    }
    
    /**
      | Returns true if this is a full-frame
      | midi timecode message.
      |
      */
    pub fn is_full_frame(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData();

        return data[0] == 0xf0
                && data[1] == 0x7f
                && size >= 10
                && data[3] == 0x01
                && data[4] == 0x01;
        */
    }
    
    /** 
      | Extracts the timecode information from
      | a full-frame midi timecode message.
      |
      | You should only call this on messages
      | where you've used isFullFrame() to check
      | that they're the right kind.
      */
    pub fn get_full_frame_parameters(
        &self, 
        hours:         &mut i32,
        minutes:       &mut i32,
        seconds:       &mut i32,
        frames:        &mut i32,
        timecode_type: &mut MidiMessageSmpteTimecodeType

    ) {
        
        todo!();
        /*
            jassert (isFullFrame());

        auto data = getRawData();
        timecodeType = (MidiMessageSmpteTimecodeType) (data[5] >> 5);
        hours   = data[5] & 0x1f;
        minutes = data[6];
        seconds = data[7];
        frames  = data[8];
        */
    }
    
    /**
      | Creates a full-frame MTC message.
      |
      */
    pub fn full_frame(
        &mut self, 
        hours:         i32,
        minutes:       i32,
        seconds:       i32,
        frames:        i32,
        timecode_type: MidiMessageSmpteTimecodeType

    ) -> MidiMessage {
        
        todo!();
        /*
            return { 0xf0, 0x7f, 0x7f, 0x01, 0x01,
                 (hours & 0x01f) | (timecodeType << 5),
                 minutes, seconds, frames,
                 0xf7 };
        */
    }
    
    /**
      | Checks whether this is an MMC message.
      | If it is, you can use the getMidiMachineControlCommand()
      | to find out its type.
      |
      */
    pub fn is_midi_machine_control_message(&self) -> bool {
        
        todo!();
        /*
            auto data = getRawData();

        return data[0] == 0xf0
            && data[1] == 0x7f
            && data[3] == 0x06
            && size > 5;
        */
    }
    
    /**
      | For an MMC message, this returns its
      | type. Make sure it's actually an MMC
      | message with isMidiMachineControlMessage()
      | before calling this method.
      |
      */
    pub fn get_midi_machine_control_command(&self) -> Box<dyn MidiMachineControlCommand> {
        
        todo!();
        /*
            jassert (isMidiMachineControlMessage());

        return (MidiMachineControlCommand) getRawData()[4];
        */
    }
    
    /**
      | Creates an MMC message.
      |
      */
    pub fn midi_machine_control_command(&mut self, command: &dyn MidiMachineControlCommand) -> MidiMessage {
        
        todo!();
        /*
            return { 0xf0, 0x7f, 0, 6, command, 0xf7 };
        */
    }
    
    /**
      | Checks whether this is an MMC "goto"
      | message. If it is, the parameters passed-in
      | are set to the time that the message contains.
      | @see midiMachineControlGoto
      |
      */
    pub fn is_midi_machine_control_goto(
        &self, 
        hours:   &mut i32,
        minutes: &mut i32,
        seconds: &mut i32,
        frames:  &mut i32

    ) -> bool {
        
        todo!();
        /*
            auto data = getRawData();

        if (size >= 12
             && data[0] == 0xf0
             && data[1] == 0x7f
             && data[3] == 0x06
             && data[4] == 0x44
             && data[5] == 0x06
             && data[6] == 0x01)
        {
            hours = data[7] % 24;   // (that some machines send out hours > 24)
            minutes = data[8];
            seconds = data[9];
            frames  = data[10];

            return true;
        }

        return false;
        */
    }
    
    /**
      | Creates an MMC "goto" message. This
      | messages tells the device to go to a specific
      | frame. @see isMidiMachineControlGoto
      |
      */
    pub fn midi_machine_control_goto(
        &mut self, 
        hours:   i32,
        minutes: i32,
        seconds: i32,
        frames:  i32

    ) -> MidiMessage {
        
        todo!();
        /*
            return { 0xf0, 0x7f, 0, 6, 0x44, 6, 1, hours, minutes, seconds, frames, 0xf7 };
        */
    }
    
    /**
      | Returns the name of a midi note number.
      | E.g "C", "D#", etc.
      | 
      | -----------
      | @param noteNumber
      | 
      | the midi note number, 0 to 127
      | ----------
      | @param useSharps
      | 
      | if true, sharpened notes are used, e.g.
      | "C#", otherwise they'll be flattened,
      | e.g. "Db"
      | ----------
      | @param includeOctaveNumber
      | 
      | if true, the octave number will be appended
      | to the string, e.g. "C#4"
      | ----------
      | @param octaveNumForMiddleC
      | 
      | if an octave number is being appended,
      | this indicates the number that will
      | be used for middle C's octave
      | 
      | @see getMidiNoteInHertz
      |
      */
    pub fn get_midi_note_name(
        &mut self, 
        note:                   i32,
        use_sharps:             bool,
        include_octave_number:  bool,
        octave_num_for_middlec: i32

    ) -> String {
        
        todo!();
        /*
            static const char* const sharpNoteNames[] = { "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" };
        static const char* const flatNoteNames[]  = { "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B" };

        if (isPositiveAndBelow (note, 128))
        {
            String s (useSharps ? sharpNoteNames[note % 12]
                                : flatNoteNames [note % 12]);

            if (includeOctaveNumber)
                s << (note / 12 + (octaveNumForMiddleC - 5));

            return s;
        }

        return {};
        */
    }
    
    /**
      | Returns the frequency of a midi note
      | number.
      | 
      | -----------
      | @note
      | 
      | The frequencyOfA parameter is an optional
      | frequency for 'A', normally 440-444Hz
      | for concert pitch.
      | 
      | @see getMidiNoteName
      |
      */
    pub fn get_midi_note_in_hertz(
        &mut self, 
        note_number:   i32,
        frequency_ofa: Option<f64>

    ) -> f64 {

        let frequency_ofa: f64 = frequency_ofa.unwrap_or(440.0);
        
        todo!();
        /*
            return frequencyOfA * std::pow (2.0, (noteNumber - 69) / 12.0);
        */
    }
    
    /**
      | Returns true if the given midi note number
      | is a black key.
      |
      */
    pub fn is_midi_note_black(&mut self, note_number: i32) -> bool {
        
        todo!();
        /*
            return ((1 << (noteNumber % 12)) & 0x054a) != 0;
        */
    }
    
    /**
      | Returns the standard name of a GM instrument,
      | or nullptr if unknown for this index.
      | 
      | -----------
      | @param midiInstrumentNumber
      | 
      | the program number 0 to 127
      | 
      | @see getProgramChangeNumber
      |
      */
    pub fn get_gm_instrument_name(&mut self, n: i32) -> *const u8 {
        
        todo!();
        /*
            static const char* names[] =
        {
            NEEDS_TRANS("Acoustic Grand Piano"),    NEEDS_TRANS("Bright Acoustic Piano"),   NEEDS_TRANS("Electric Grand Piano"),    NEEDS_TRANS("Honky-tonk Piano"),
            NEEDS_TRANS("Electric Piano 1"),        NEEDS_TRANS("Electric Piano 2"),        NEEDS_TRANS("Harpsichord"),             NEEDS_TRANS("Clavinet"),
            NEEDS_TRANS("Celesta"),                 NEEDS_TRANS("Glockenspiel"),            NEEDS_TRANS("Music Box"),               NEEDS_TRANS("Vibraphone"),
            NEEDS_TRANS("Marimba"),                 NEEDS_TRANS("Xylophone"),               NEEDS_TRANS("Tubular Bells"),           NEEDS_TRANS("Dulcimer"),
            NEEDS_TRANS("Drawbar Organ"),           NEEDS_TRANS("Percussive Organ"),        NEEDS_TRANS("Rock Organ"),              NEEDS_TRANS("Church Organ"),
            NEEDS_TRANS("Reed Organ"),              NEEDS_TRANS("Accordion"),               NEEDS_TRANS("Harmonica"),               NEEDS_TRANS("Tango Accordion"),
            NEEDS_TRANS("Acoustic Guitar (nylon)"), NEEDS_TRANS("Acoustic Guitar (steel)"), NEEDS_TRANS("Electric Guitar (jazz)"),  NEEDS_TRANS("Electric Guitar (clean)"),
            NEEDS_TRANS("Electric Guitar (mute)"),  NEEDS_TRANS("Overdriven Guitar"),       NEEDS_TRANS("Distortion Guitar"),       NEEDS_TRANS("Guitar Harmonics"),
            NEEDS_TRANS("Acoustic Bass"),           NEEDS_TRANS("Electric Bass (finger)"),  NEEDS_TRANS("Electric Bass (pick)"),    NEEDS_TRANS("Fretless Bass"),
            NEEDS_TRANS("Slap Bass 1"),             NEEDS_TRANS("Slap Bass 2"),             NEEDS_TRANS("Synth Bass 1"),            NEEDS_TRANS("Synth Bass 2"),
            NEEDS_TRANS("Violin"),                  NEEDS_TRANS("Viola"),                   NEEDS_TRANS("Cello"),                   NEEDS_TRANS("Contrabass"),
            NEEDS_TRANS("Tremolo Strings"),         NEEDS_TRANS("Pizzicato Strings"),       NEEDS_TRANS("Orchestral Harp"),         NEEDS_TRANS("Timpani"),
            NEEDS_TRANS("String Ensemble 1"),       NEEDS_TRANS("String Ensemble 2"),       NEEDS_TRANS("SynthStrings 1"),          NEEDS_TRANS("SynthStrings 2"),
            NEEDS_TRANS("Choir Aahs"),              NEEDS_TRANS("Voice Oohs"),              NEEDS_TRANS("Synth Voice"),             NEEDS_TRANS("Orchestra Hit"),
            NEEDS_TRANS("Trumpet"),                 NEEDS_TRANS("Trombone"),                NEEDS_TRANS("Tuba"),                    NEEDS_TRANS("Muted Trumpet"),
            NEEDS_TRANS("French Horn"),             NEEDS_TRANS("Brass Section"),           NEEDS_TRANS("SynthBrass 1"),            NEEDS_TRANS("SynthBrass 2"),
            NEEDS_TRANS("Soprano Sax"),             NEEDS_TRANS("Alto Sax"),                NEEDS_TRANS("Tenor Sax"),               NEEDS_TRANS("Baritone Sax"),
            NEEDS_TRANS("Oboe"),                    NEEDS_TRANS("English Horn"),            NEEDS_TRANS("Bassoon"),                 NEEDS_TRANS("Clarinet"),
            NEEDS_TRANS("Piccolo"),                 NEEDS_TRANS("Flute"),                   NEEDS_TRANS("Recorder"),                NEEDS_TRANS("Pan Flute"),
            NEEDS_TRANS("Blown Bottle"),            NEEDS_TRANS("Shakuhachi"),              NEEDS_TRANS("Whistle"),                 NEEDS_TRANS("Ocarina"),
            NEEDS_TRANS("Lead 1 (square)"),         NEEDS_TRANS("Lead 2 (sawtooth)"),       NEEDS_TRANS("Lead 3 (calliope)"),       NEEDS_TRANS("Lead 4 (chiff)"),
            NEEDS_TRANS("Lead 5 (charang)"),        NEEDS_TRANS("Lead 6 (voice)"),          NEEDS_TRANS("Lead 7 (fifths)"),         NEEDS_TRANS("Lead 8 (bass+lead)"),
            NEEDS_TRANS("Pad 1 (new age)"),         NEEDS_TRANS("Pad 2 (warm)"),            NEEDS_TRANS("Pad 3 (polysynth)"),       NEEDS_TRANS("Pad 4 (choir)"),
            NEEDS_TRANS("Pad 5 (bowed)"),           NEEDS_TRANS("Pad 6 (metallic)"),        NEEDS_TRANS("Pad 7 (halo)"),            NEEDS_TRANS("Pad 8 (sweep)"),
            NEEDS_TRANS("FX 1 (rain)"),             NEEDS_TRANS("FX 2 (soundtrack)"),       NEEDS_TRANS("FX 3 (crystal)"),          NEEDS_TRANS("FX 4 (atmosphere)"),
            NEEDS_TRANS("FX 5 (brightness)"),       NEEDS_TRANS("FX 6 (goblins)"),          NEEDS_TRANS("FX 7 (echoes)"),           NEEDS_TRANS("FX 8 (sci-fi)"),
            NEEDS_TRANS("Sitar"),                   NEEDS_TRANS("Banjo"),                   NEEDS_TRANS("Shamisen"),                NEEDS_TRANS("Koto"),
            NEEDS_TRANS("Kalimba"),                 NEEDS_TRANS("Bag pipe"),                NEEDS_TRANS("Fiddle"),                  NEEDS_TRANS("Shanai"),
            NEEDS_TRANS("Tinkle Bell"),             NEEDS_TRANS("Agogo"),                   NEEDS_TRANS("Steel Drums"),             NEEDS_TRANS("Woodblock"),
            NEEDS_TRANS("Taiko Drum"),              NEEDS_TRANS("Melodic Tom"),             NEEDS_TRANS("Synth Drum"),              NEEDS_TRANS("Reverse Cymbal"),
            NEEDS_TRANS("Guitar Fret Noise"),       NEEDS_TRANS("Breath Noise"),            NEEDS_TRANS("Seashore"),                NEEDS_TRANS("Bird Tweet"),
            NEEDS_TRANS("Telephone Ring"),          NEEDS_TRANS("Helicopter"),              NEEDS_TRANS("Applause"),                NEEDS_TRANS("Gunshot")
        };

        return isPositiveAndBelow (n, numElementsInArray (names)) ? names[n] : nullptr;
        */
    }
    
    /**
      | Returns the name of a bank of GM instruments,
      | or nullptr if unknown for this bank number.
      | 
      | -----------
      | @param midiBankNumber
      | 
      | the bank, 0 to 15
      |
      */
    pub fn get_gm_instrument_bank_name(&mut self, n: i32) -> *const u8 {
        
        todo!();
        /*
            static const char* names[] =
        {
            NEEDS_TRANS("Piano"),           NEEDS_TRANS("Chromatic Percussion"),    NEEDS_TRANS("Organ"),       NEEDS_TRANS("Guitar"),
            NEEDS_TRANS("Bass"),            NEEDS_TRANS("Strings"),                 NEEDS_TRANS("Ensemble"),    NEEDS_TRANS("Brass"),
            NEEDS_TRANS("Reed"),            NEEDS_TRANS("Pipe"),                    NEEDS_TRANS("Synth Lead"),  NEEDS_TRANS("Synth Pad"),
            NEEDS_TRANS("Synth Effects"),   NEEDS_TRANS("Ethnic"),                  NEEDS_TRANS("Percussive"),  NEEDS_TRANS("Sound Effects")
        };

        return isPositiveAndBelow (n, numElementsInArray (names)) ? names[n] : nullptr;
        */
    }
    
    /**
      | Returns the standard name of a channel
      | 10 percussion sound, or nullptr if unknown
      | for this note number.
      | 
      | -----------
      | @param midiNoteNumber
      | 
      | the key number, 35 to 81
      |
      */
    pub fn get_rhythm_instrument_name(&mut self, n: i32) -> *const u8 {
        
        todo!();
        /*
            static const char* names[] =
        {
            NEEDS_TRANS("Acoustic Bass Drum"),  NEEDS_TRANS("Bass Drum 1"),     NEEDS_TRANS("Side Stick"),      NEEDS_TRANS("Acoustic Snare"),
            NEEDS_TRANS("Hand Clap"),           NEEDS_TRANS("Electric Snare"),  NEEDS_TRANS("Low Floor Tom"),   NEEDS_TRANS("Closed Hi-Hat"),
            NEEDS_TRANS("High Floor Tom"),      NEEDS_TRANS("Pedal Hi-Hat"),    NEEDS_TRANS("Low Tom"),         NEEDS_TRANS("Open Hi-Hat"),
            NEEDS_TRANS("Low-Mid Tom"),         NEEDS_TRANS("Hi-Mid Tom"),      NEEDS_TRANS("Crash Cymbal 1"),  NEEDS_TRANS("High Tom"),
            NEEDS_TRANS("Ride Cymbal 1"),       NEEDS_TRANS("Chinese Cymbal"),  NEEDS_TRANS("Ride Bell"),       NEEDS_TRANS("Tambourine"),
            NEEDS_TRANS("Splash Cymbal"),       NEEDS_TRANS("Cowbell"),         NEEDS_TRANS("Crash Cymbal 2"),  NEEDS_TRANS("Vibraslap"),
            NEEDS_TRANS("Ride Cymbal 2"),       NEEDS_TRANS("Hi Bongo"),        NEEDS_TRANS("Low Bongo"),       NEEDS_TRANS("Mute Hi Conga"),
            NEEDS_TRANS("Open Hi Conga"),       NEEDS_TRANS("Low Conga"),       NEEDS_TRANS("High Timbale"),    NEEDS_TRANS("Low Timbale"),
            NEEDS_TRANS("High Agogo"),          NEEDS_TRANS("Low Agogo"),       NEEDS_TRANS("Cabasa"),          NEEDS_TRANS("Maracas"),
            NEEDS_TRANS("Short Whistle"),       NEEDS_TRANS("Long Whistle"),    NEEDS_TRANS("Short Guiro"),     NEEDS_TRANS("Long Guiro"),
            NEEDS_TRANS("Claves"),              NEEDS_TRANS("Hi Wood Block"),   NEEDS_TRANS("Low Wood Block"),  NEEDS_TRANS("Mute Cuica"),
            NEEDS_TRANS("Open Cuica"),          NEEDS_TRANS("Mute Triangle"),   NEEDS_TRANS("Open Triangle")
        };

        return (n >= 35 && n <= 81) ? names[n - 35] : nullptr;
        */
    }
    
    /**
      | Returns the name of a controller type
      | number, or nullptr if unknown for this
      | controller number. @see getControllerNumber
      |
      */
    pub fn get_controller_name(&mut self, n: i32) -> *const u8 {
        
        todo!();
        /*
            static const char* names[] =
        {
            NEEDS_TRANS("Bank Select"), NEEDS_TRANS("Modulation Wheel (coarse)"), NEEDS_TRANS("Breath controller (coarse)"),
            nullptr,
            NEEDS_TRANS("Foot Pedal (coarse)"), NEEDS_TRANS("Portamento Time (coarse)"), NEEDS_TRANS("Data Entry (coarse)"),
            NEEDS_TRANS("Volume (coarse)"), NEEDS_TRANS("Balance (coarse)"),
            nullptr,
            NEEDS_TRANS("Pan position (coarse)"), NEEDS_TRANS("Expression (coarse)"), NEEDS_TRANS("Effect Control 1 (coarse)"),
            NEEDS_TRANS("Effect Control 2 (coarse)"),
            nullptr, nullptr,
            NEEDS_TRANS("General Purpose Slider 1"), NEEDS_TRANS("General Purpose Slider 2"),
            NEEDS_TRANS("General Purpose Slider 3"), NEEDS_TRANS("General Purpose Slider 4"),
            nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
            NEEDS_TRANS("Bank Select (fine)"), NEEDS_TRANS("Modulation Wheel (fine)"), NEEDS_TRANS("Breath controller (fine)"),
            nullptr,
            NEEDS_TRANS("Foot Pedal (fine)"), NEEDS_TRANS("Portamento Time (fine)"), NEEDS_TRANS("Data Entry (fine)"), NEEDS_TRANS("Volume (fine)"),
            NEEDS_TRANS("Balance (fine)"), nullptr, NEEDS_TRANS("Pan position (fine)"), NEEDS_TRANS("Expression (fine)"),
            NEEDS_TRANS("Effect Control 1 (fine)"), NEEDS_TRANS("Effect Control 2 (fine)"),
            nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
            nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
            NEEDS_TRANS("Hold Pedal (on/off)"), NEEDS_TRANS("Portamento (on/off)"), NEEDS_TRANS("Sustenuto Pedal (on/off)"), NEEDS_TRANS("Soft Pedal (on/off)"),
            NEEDS_TRANS("Legato Pedal (on/off)"), NEEDS_TRANS("Hold 2 Pedal (on/off)"), NEEDS_TRANS("Sound Variation"), NEEDS_TRANS("Sound Timbre"),
            NEEDS_TRANS("Sound Release Time"), NEEDS_TRANS("Sound Attack Time"), NEEDS_TRANS("Sound Brightness"), NEEDS_TRANS("Sound Control 6"),
            NEEDS_TRANS("Sound Control 7"), NEEDS_TRANS("Sound Control 8"), NEEDS_TRANS("Sound Control 9"), NEEDS_TRANS("Sound Control 10"),
            NEEDS_TRANS("General Purpose Button 1 (on/off)"), NEEDS_TRANS("General Purpose Button 2 (on/off)"),
            NEEDS_TRANS("General Purpose Button 3 (on/off)"), NEEDS_TRANS("General Purpose Button 4 (on/off)"),
            nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
            NEEDS_TRANS("Reverb Level"), NEEDS_TRANS("Tremolo Level"), NEEDS_TRANS("Chorus Level"), NEEDS_TRANS("Celeste Level"),
            NEEDS_TRANS("Phaser Level"), NEEDS_TRANS("Data Button increment"), NEEDS_TRANS("Data Button decrement"), NEEDS_TRANS("Non-registered Parameter (fine)"),
            NEEDS_TRANS("Non-registered Parameter (coarse)"), NEEDS_TRANS("Registered Parameter (fine)"), NEEDS_TRANS("Registered Parameter (coarse)"),
            nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
            nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
            NEEDS_TRANS("All Sound Off"), NEEDS_TRANS("All Controllers Off"), NEEDS_TRANS("Local Keyboard (on/off)"), NEEDS_TRANS("All Notes Off"),
            NEEDS_TRANS("Omni Mode Off"), NEEDS_TRANS("Omni Mode On"), NEEDS_TRANS("Mono Operation"), NEEDS_TRANS("Poly Operation")
        };

        return isPositiveAndBelow (n, numElementsInArray (names)) ? names[n] : nullptr;
        */
    }
}
