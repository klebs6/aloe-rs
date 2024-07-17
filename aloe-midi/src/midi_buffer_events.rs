crate::ix!();

#[inline] pub fn midi_buffer_get_event_time(d: *const c_void) -> i32 {
    
    todo!();
    /*
        return readUnaligned<int32> (d);
    */
}

#[inline] pub fn midi_buffer_get_event_data_size(d: *const c_void) -> u16 {
    
    todo!();
    /*
        return readUnaligned<uint16> (static_cast<const char*> (d) + sizeof (int32));
    */
}

#[inline] pub fn midi_buffer_get_event_total_size(d: *const c_void) -> u16 {
    
    todo!();
    /*
        return (uint16) (getEventDataSize (d) + sizeof (int32) + sizeof (uint16));
    */
}

pub fn midi_buffer_find_actual_event_length(
        data:      *const u8,
        max_bytes: i32) -> i32 {
    
    todo!();
    /*
        auto byte = (unsigned int) *data;

            if (byte == 0xf0 || byte == 0xf7)
            {
                int i = 1;

                while (i < maxBytes)
                    if (data[i++] == 0xf7)
                        break;

                return i;
            }

            if (byte == 0xff)
            {
                if (maxBytes == 1)
                    return 1;

                const auto var = MidiMessage::readVariableLengthValue (data + 1, maxBytes - 1);
                return jmin (maxBytes, var.value + 2 + var.bytesUsed);
            }

            if (byte >= 0x80)
                return jmin (maxBytes, MidiMessage::getMessageLengthFromFirstByte ((uint8) byte));

            return 0;
    */
}

pub fn midi_buffer_find_event_after(
        d:               *mut u8,
        end_data:        *mut u8,
        sample_position: i32) -> *mut u8 {
    
    todo!();
    /*
        while (d < endData && getEventTime (d) <= samplePosition)
                d += getEventTotalSize (d);

            return d;
    */
}
