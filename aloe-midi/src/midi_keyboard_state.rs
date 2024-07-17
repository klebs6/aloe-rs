crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiKeyboardState.h]

/**
  | Represents a piano keyboard, keeping track of
  | which keys are currently pressed.
  |
  | This object can parse a stream of midi events,
  | using them to update its idea of which keys
  | are pressed for each individual midi channel.
  |
  | When keys go up or down, it can broadcast
  | these events to listener objects.
  |
  | It also allows key up/down events to be
  | triggered with its noteOn() and noteOff()
  | methods, and midi messages for these events
  | will be merged into the midi stream that gets
  | processed by processNextMidiBuffer().
  |
  | @tags{Audio}
  */
#[no_copy]
#[leak_detector]
pub struct MidiKeyboardState {
    lock:          CriticalSection,
    note_states:   [Atomic<u16>; 128],
    events_to_add: MidiBuffer,
    listeners:     ListenerList<Rc<RefCell<dyn MidiKeyboardStateListener>>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/aloe_MidiKeyboardState.cpp]
impl Default for MidiKeyboardState {

    fn default() -> Self {
    
        todo!();
        /*


            zerostruct (noteStates);
        */
    }
}
    
impl MidiKeyboardState {

    /** 
      | Resets the state of the object.
      |
      | All internal data for all the channels is
      | reset, but no events are sent as a result.
      |
      | If you want to release any keys that are
      | currently down, and to send out note-up
      | midi messages for this, use the
      | allNotesOff() method instead.
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        zerostruct (noteStates);
        eventsToAdd.clear();
        */
    }
    
    /** 
      | Returns true if the given midi key is
      | currently held down for the given midi
      | channel.
      |
      | The channel number must be between 1 and
      | 16. If you want to see if any notes are on
      | for a range of channels, use the
      | isNoteOnForChannels() method.
      */
    pub fn is_note_on(&self, 
        midi_channel: i32,
        n:            i32) -> bool {
        
        todo!();
        /*
            jassert (midiChannel > 0 && midiChannel <= 16);

        return isPositiveAndBelow (n, 128)
                && (noteStates[n] & (1 << (midiChannel - 1))) != 0;
        */
    }
    
    /** 
      | Returns true if the given midi key is
      | currently held down on any of a set of midi
      | channels.
      |
      | The channel mask has a bit set for each midi
      | channel you want to test for - bit
      | 0 = midi channel 1, bit 1 = midi channel 2,
      | etc.
      |
      | If a note is on for at least one of the
      | specified channels, this returns true.
      */
    pub fn is_note_on_for_channels(&self, 
        midi_channel_mask: i32,
        n:                 i32) -> bool {
        
        todo!();
        /*
            return isPositiveAndBelow (n, 128)
                && (noteStates[n] & midiChannelMask) != 0;
        */
    }
    
    /** 
      | Turns a specified note on.
      |
      | This will cause a suitable midi note-on
      | event to be injected into the midi buffer
      | during the next call to
      | processNextMidiBuffer().
      |
      | It will also trigger a synchronous
      | callback to the listeners to tell them
      | that the key has gone down.
      */
    pub fn note_on(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            jassert (midiChannel > 0 && midiChannel <= 16);
        jassert (isPositiveAndBelow (midiNoteNumber, 128));

        const ScopedLock sl (lock);

        if (isPositiveAndBelow (midiNoteNumber, 128))
        {
            const int timeNow = (int) Time::getMillisecondCounter();
            eventsToAdd.addEvent (MidiMessage::noteOn (midiChannel, midiNoteNumber, velocity), timeNow);
            eventsToAdd.clear (0, timeNow - 500);

            noteOnInternal (midiChannel, midiNoteNumber, velocity);
        }
        */
    }
    
    pub fn note_on_internal(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (midiNoteNumber, 128))
        {
            noteStates[midiNoteNumber] = static_cast<uint16> (noteStates[midiNoteNumber] | (1 << (midiChannel - 1)));
            listeners.call ([&] (MidiKeyboardStateListener& l) { l.handleNoteOn (this, midiChannel, midiNoteNumber, velocity); });
        }
        */
    }
    
    /** 
      | Turns a specified note off.
      |
      | This will cause a suitable midi note-off
      | event to be injected into the midi buffer
      | during the next call to
      | processNextMidiBuffer().
      |
      | It will also trigger a synchronous
      | callback to the listeners to tell them
      | that the key has gone up.
      |
      | But if the note isn't actually down for
      | the given channel, this method will in fact do
      | nothing.
      */
    pub fn note_off(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (isNoteOn (midiChannel, midiNoteNumber))
        {
            const int timeNow = (int) Time::getMillisecondCounter();
            eventsToAdd.addEvent (MidiMessage::noteOff (midiChannel, midiNoteNumber), timeNow);
            eventsToAdd.clear (0, timeNow - 500);

            noteOffInternal (midiChannel, midiNoteNumber, velocity);
        }
        */
    }
    
    pub fn note_off_internal(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            if (isNoteOn (midiChannel, midiNoteNumber))
        {
            noteStates[midiNoteNumber] = static_cast<uint16> (noteStates[midiNoteNumber] & ~(1 << (midiChannel - 1)));
            listeners.call ([&] (MidiKeyboardStateListener& l) { l.handleNoteOff (this, midiChannel, midiNoteNumber, velocity); });
        }
        */
    }
    
    /** 
      | This will turn off any currently-down notes
      | for the given midi channel.
      |
      | If you pass 0 for the midi channel, it
      | will in fact turn off all notes on all
      | channels.
      |
      | Calling this method will make calls to
      | noteOff(), so can trigger synchronous
      | callbacks and events being added to the
      | midi stream.
      */
    pub fn all_notes_off(&mut self, midi_channel: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (midiChannel <= 0)
        {
            for (int i = 1; i <= 16; ++i)
                allNotesOff (i);
        }
        else
        {
            for (int i = 0; i < 128; ++i)
                noteOff (midiChannel, i, 0.0f);
        }
        */
    }
    
    /** 
      | Looks at a key-up/down event and uses it to
      | update the state of this object.
      |
      | To process a buffer full of midi messages,
      | use the processNextMidiBuffer() method
      | instead.
      */
    pub fn process_next_midi_event(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            if (message.isNoteOn())
        {
            noteOnInternal (message.getChannel(), message.getNoteNumber(), message.getFloatVelocity());
        }
        else if (message.isNoteOff())
        {
            noteOffInternal (message.getChannel(), message.getNoteNumber(), message.getFloatVelocity());
        }
        else if (message.isAllNotesOff())
        {
            for (int i = 0; i < 128; ++i)
                noteOffInternal (message.getChannel(), i, 0.0f);
        }
        */
    }
    
    /** 
      | Scans a midi stream for up/down events and
      | adds its own events to it.
      |
      | This will look for any up/down events and
      | use them to update the internal state,
      | synchronously making suitable callbacks to
      | the listeners.
      |
      | If injectIndirectEvents is true, then midi
      | events to produce the recent noteOn() and
      | noteOff() calls will be added into the
      | buffer.
      |
      | Only the section of the buffer whose
      | timestamps are between startSample and
      | (startSample + numSamples) will be
      | affected, and any events added will be
      | placed between these times.
      |
      | If you're going to use this method, you'll
      | need to keep calling it regularly for it
      | to work satisfactorily.
      |
      | To process a single midi event at a time,
      | use the processNextMidiEvent() method
      | instead.
      */
    pub fn process_next_midi_buffer(&mut self, 
        buffer:                 &mut MidiBuffer,
        start_sample:           i32,
        num_samples:            i32,
        inject_indirect_events: bool)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (const auto metadata : buffer)
            processNextMidiEvent (metadata.getMessage());

        if (injectIndirectEvents)
        {
            const int firstEventToAdd = eventsToAdd.getFirstEventTime();
            const double scaleFactor = numSamples / (double) (eventsToAdd.getLastEventTime() + 1 - firstEventToAdd);

            for (const auto metadata : eventsToAdd)
            {
                const auto pos = jlimit (0, numSamples - 1, roundToInt ((metadata.samplePosition - firstEventToAdd) * scaleFactor));
                buffer.addEvent (metadata.getMessage(), startSample + pos);
            }
        }

        eventsToAdd.clear();
        */
    }
    
    /**
      | Registers a listener for callbacks
      | when keys go up or down. @see removeListener
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn MidiKeyboardStateListener)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        listeners.add (listener);
        */
    }
    
    /**
      | Deregisters a listener. @see addListener
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn MidiKeyboardStateListener)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        listeners.remove (listener);
        */
    }
}
