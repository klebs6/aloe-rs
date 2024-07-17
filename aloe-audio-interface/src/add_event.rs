crate::ix!();

pub trait AddEventWithMessageAndSampleNumber {

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
    fn add_event_with_message_and_sample_number(
        &mut self, 
        m:             &dyn MidiMessageInterface,
        sample_number: i32

    ) -> bool;
}

pub trait AddEvent {

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
    fn add_event(
        &mut self, 
        new_data:      *const c_void,
        max_bytes:     i32,
        sample_number: i32
    ) -> bool;
}

pub trait AddEvents {

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
    fn add_events(
        &mut self, 
        other_buffer:        &dyn MidiBufferInterface,
        start_sample:        i32,
        num_samples:         i32,
        sample_delta_to_add: i32
    );
}
