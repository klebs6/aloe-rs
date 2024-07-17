crate::ix!();

/**
  | Holds a sequence of time-stamped midi events.
  |
  | Analogous to the AudioBuffer, this holds a set
  | of midi events with integer time-stamps. The
  | buffer is kept sorted in order of the
  | time-stamps.
  |
  | If you're working with a sequence of midi
  | events that may need to be manipulated or
  | read/written to a midi file, then
  | MidiMessageSequence is probably a more
  | appropriate container. MidiBuffer is designed
  | for lower-level streams of raw midi data.
  |
  | @see MidiMessage
  |
  | @tags{Audio}
  */
#[derive(Default)] // Creates an empty MidiBuffer
#[leak_detector]
pub struct MidiBuffer {

    /** 
      | The raw data holding this buffer.
      |
      | Obviously access to this data is provided at
      | your own risk. Its internal format could
      | change in future, so don't write code that
      | relies on it!
      */
    data: Vec<u8>,
}

impl MidiBuffer {

    /**
      | Get a read-only iterator pointing to
      | the beginning of this buffer.
      |
      */
    pub fn begin(&self) -> MidiBufferIterator {
        
        todo!();
        /*
            return
            cbegin();
        */
    }

    /**
      | Get a read-only iterator pointing one
      | past the end of this buffer.
      |
      */
    pub fn end(&self) -> MidiBufferIterator {
        
        todo!();
        /*
            return cend();
        */
    }

    /**
      | Get a read-only iterator pointing to
      | the beginning of this buffer.
      |
      */
    pub fn cbegin(&self) -> MidiBufferIterator {
        
        todo!();
        /*
            return MidiBufferIterator (data.begin());
        */
    }

    /**
      | Get a read-only iterator pointing one
      | past the end of this buffer.
      |
      */
    pub fn cend(&self) -> MidiBufferIterator {
        
        todo!();
        /*
            return MidiBufferIterator (data.end());
        */
    }

    /**
      | Creates a MidiBuffer containing a single
      | midi message.
      |
      */
    pub fn new(message: &MidiMessage) -> Self {
    
        todo!();
        /*
            addEvent (message, 0);
        */
    }
    
    /** 
      | Exchanges the contents of this buffer with
      | another one.
      |
      | This is a quick operation, because no
      | memory allocating or copying is done, it
      | just swaps the internal state of the two
      | buffers.
      */
    pub fn swap_with(&mut self, other: &mut MidiBuffer)  {
        
        todo!();
        /*
            data.swapWith (other.data);
        */
    }
    
    /**
      | Removes all events from the buffer.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            data.clearQuick();
        */
    }
    
    /** 
      | Preallocates some memory for the buffer to
      | use.
      |
      | This helps to avoid needing to reallocate
      | space when the buffer has messages added
      | to it.
      */
    pub fn ensure_size(&mut self, minimum_num_bytes: usize)  {
        
        todo!();
        /*
            data.ensureStorageAllocated ((int) minimumNumBytes);
        */
    }
    
    /** 
      | Returns true if the buffer is empty.
      |
      | To actually retrieve the events, use
      | a MidiBufferIterator object
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return data.size() == 0;
        */
    }
    
    /** 
      | Removes all events between two times from the
      | buffer.
      |
      | All events for which (start <= event
      | position < start + numSamples) will be
      | removed.
      */
    pub fn clear_from_start_with_len(
        &mut self, 
        start_sample: i32,
        num_samples:  i32

    ) {
        
        todo!();
        /*
            auto start = MidiBufferHelpers::findEventAfter (data.begin(), data.end(), startSample - 1);
        auto end   = MidiBufferHelpers::findEventAfter (start,        data.end(), startSample + numSamples - 1);

        data.removeRange ((int) (start - data.begin()), (int) (end - start));
        */
    }
    
    /** 
      | Adds an event to the buffer.
      |
      | The sample number will be used to determine
      | the position of the event in the buffer,
      | which is always kept sorted. The
      | MidiMessage's timestamp is ignored.
      |
      | If an event is added whose sample position
      | is the same as one or more events already in
      | the buffer, the new event will be placed
      | after the existing ones.
      |
      | To retrieve events, use a MidiBufferIterator
      | object.
      |
      | Returns true on success, or false on
      | failure.
      */
    pub fn add_event_with_message_and_sample_number(
        &mut self, 
        m:             &MidiMessage,
        sample_number: i32

    ) -> bool {
        
        todo!();
        /*
            return addEvent (m.getRawData(), m.getRawDataSize(), sampleNumber);
        */
    }
    
    /** 
      | Adds an event to the buffer from raw midi
      | data.
      |
      | The sample number will be used to
      | determine the position of the event in the
      | buffer, which is always kept sorted.
      |
      | If an event is added whose sample position
      | is the same as one or more events already
      | in the buffer, the new event will be
      | placed after the existing ones.
      |
      | The event data will be inspected to
      | calculate the number of bytes in length
      | that the midi event really takes up, so
      | maxBytesOfMidiData may be longer than the
      | data that actually gets stored. E.g. if
      | you pass in a note-on and a length of
      | 4 bytes, it'll actually only store
      | 3 bytes. If the midi data is invalid, it
      | might not add an event at all.
      |
      | To retrieve events, use
      | a MidiBufferIterator object.
      |
      | Returns true on success, or false on
      | failure.
      */
    pub fn add_event(&mut self, 
        new_data:      *const c_void,
        max_bytes:     i32,
        sample_number: i32) -> bool {
        
        todo!();
        /*
            auto numBytes = MidiBufferHelpers::findActualEventLength (static_cast<const uint8*> (newData), maxBytes);

        if (numBytes <= 0)
            return true;

        if (std::numeric_limits<uint16>::max() < numBytes)
        {
            // This method only supports messages smaller than (1 << 16) bytes
            return false;
        }

        auto newItemSize = (size_t) numBytes + sizeof (int32) + sizeof (uint16);
        auto offset = (int) (MidiBufferHelpers::findEventAfter (data.begin(), data.end(), sampleNumber) - data.begin());

        data.insertMultiple (offset, 0, (int) newItemSize);

        auto* d = data.begin() + offset;
        writeUnaligned<int32>  (d, sampleNumber);
        d += sizeof (int32);
        writeUnaligned<uint16> (d, static_cast<uint16> (numBytes));
        d += sizeof (uint16);
        memcpy (d, newData, (size_t) numBytes);

        return true;
        */
    }
    
    /**
      | Adds some events from another buffer
      | to this one.
      | 
      | -----------
      | @param otherBuffer
      | 
      | the buffer containing the events you
      | want to add
      | ----------
      | @param startSample
      | 
      | the lowest sample number in the source
      | buffer for which events should be added.
      | Any source events whose timestamp is
      | less than this will be ignored
      | ----------
      | @param numSamples
      | 
      | the valid range of samples from the source
      | buffer for which events should be added
      | - i.e. events in the source buffer whose
      | timestamp is greater than or equal to
      | (startSample + numSamples) will be
      | ignored. If this value is less than 0,
      | all events after startSample will be
      | taken.
      | ----------
      | @param sampleDeltaToAdd
      | 
      | a value which will be added to the source
      | timestamps of the events that are added
      | to this buffer
      |
      */
    pub fn add_events(&mut self, 
        other_buffer:        &MidiBuffer,
        start_sample:        i32,
        num_samples:         i32,
        sample_delta_to_add: i32)  {
        
        todo!();
        /*
            for (auto i = otherBuffer.findNextSamplePosition (startSample); i != otherBuffer.cend(); ++i)
        {
            const auto metadata = *i;

            if (metadata.samplePosition >= startSample + numSamples && numSamples >= 0)
                break;

            addEvent (metadata.data, metadata.numBytes, metadata.samplePosition + sampleDeltaToAdd);
        }
        */
    }
    
    /** 
      | Counts the number of events in the buffer.
      |
      | This is actually quite a slow operation,
      | as it has to iterate through all the
      | events, so you might prefer to call
      | isEmpty() if that's all you need to know.
      */
    pub fn get_num_events(&self) -> i32 {
        
        todo!();
        /*
            int n = 0;
        auto end = data.end();

        for (auto d = data.begin(); d < end; ++n)
            d += MidiBufferHelpers::getEventTotalSize (d);

        return n;
        */
    }
    
    /** 
      | Returns the sample number of the first event
      | in the buffer.
      |
      | If the buffer's empty, this will just
      | return 0.
      */
    pub fn get_first_event_time(&self) -> i32 {
        
        todo!();
        /*
            return data.size() > 0 ? MidiBufferHelpers::getEventTime (data.begin()) : 0;
        */
    }
    
    /** 
      | Returns the sample number of the last event
      | in the buffer.
      |
      | If the buffer's empty, this will just return 0.
      */
    pub fn get_last_event_time(&self) -> i32 {
        
        todo!();
        /*
            if (data.size() == 0)
            return 0;

        auto endData = data.end();

        for (auto d = data.begin();;)
        {
            auto nextOne = d + MidiBufferHelpers::getEventTotalSize (d);

            if (nextOne >= endData)
                return MidiBufferHelpers::getEventTime (d);

            d = nextOne;
        }
        */
    }
    
    /**
      | Get an iterator pointing to the first
      | event with a timestamp greater-than
      | or equal-to `samplePosition`.
      |
      */
    pub fn find_next_sample_position(&self, sample_position: i32) -> MidiBufferIterator {
        
        todo!();
        /*
            return std::find_if (cbegin(), cend(), [&] (const MidiMessageMetadata& metadata) 
        {
            return metadata.samplePosition >= samplePosition;
        });
        */
    }
}
