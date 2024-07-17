crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/midi_io/aloe_MidiMessageCollector.h]

/**
  | Collects incoming realtime MIDI messages
  | and turns them into blocks suitable
  | for processing by a block-based audio
  | callback.
  | 
  | The class can also be used as either a
  | MidiKeyboardState::Listener or a
  | MidiInputCallback so it can easily
  | use a midi input or keyboard component
  | as its source.
  | 
  | @see MidiMessage, MidiInput
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MidiMessageCollector {
    last_callback_time: f64, // default = 0
    midi_callback_lock: CriticalSection,
    incoming_messages:  MidiBuffer,
    sample_rate:        f64, // default = 44100.0

    #[cfg(ALOE_DEBUG)]
    has_called_reset: bool, // default = false
}

impl MidiKeyboardStateListener for MidiMessageCollector {

    fn handle_note_on(&mut self, 
        _0:               *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            MidiMessage m (MidiMessage::noteOn (midiChannel, midiNoteNumber, velocity));
        m.setTimeStamp (Time::getMillisecondCounterHiRes() * 0.001);

        addMessageToQueue (m);
        */
    }
    
    fn handle_note_off(&mut self, 
        _0:               *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            MidiMessage m (MidiMessage::noteOff (midiChannel, midiNoteNumber, velocity));
        m.setTimeStamp (Time::getMillisecondCounterHiRes() * 0.001);

        addMessageToQueue (m);
        */
    }
}

impl MidiInputCallback for MidiMessageCollector {

    fn handle_incoming_midi_message(&mut self, 
        _0:      *mut MidiInput,
        message: &MidiMessage)  {
        
        todo!();
        /*
            addMessageToQueue (message);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/midi_io/aloe_MidiMessageCollector.cpp]
impl MidiMessageCollector {

    /**
      | Clears any messages from the queue.
      | 
      | You need to call this method before starting
      | to use the collector, so that it knows
      | the correct sample rate to use.
      |
      */
    pub fn reset(&mut self, new_sample_rate: f64)  {
        
        todo!();
        /*
            const ScopedLock sl (midiCallbackLock);

        jassert (newSampleRate > 0);

       #if ALOE_DEBUG
        hasCalledReset = true;
       #endif
        sampleRate = newSampleRate;
        incomingMessages.clear();
        lastCallbackTime = Time::getMillisecondCounterHiRes();
        */
    }
    
    /**
      | Takes an incoming real-time message
      | and adds it to the queue.
      | 
      | The message's timestamp is taken, and
      | it will be ready for retrieval as part
      | of the block returned by the next call
      | to removeNextBlockOfMessages().
      | 
      | This method is fully thread-safe when
      | overlapping calls are made with removeNextBlockOfMessages().
      |
      */
    pub fn add_message_to_queue(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            const ScopedLock sl (midiCallbackLock);

       #if ALOE_DEBUG
        jassert (hasCalledReset); // you need to call reset() to set the correct sample rate before using this object
       #endif

        // the messages that come in here need to be time-stamped correctly - see MidiInput
        // for details of what the number should be.
        jassert (message.getTimeStamp() != 0);

        auto sampleNumber = (int) ((message.getTimeStamp() - 0.001 * lastCallbackTime) * sampleRate);

        incomingMessages.addEvent (message, sampleNumber);

        // if the messages don't get used for over a second, we'd better
        // get rid of any old ones to avoid the queue getting too big
        if (sampleNumber > sampleRate)
            incomingMessages.clear (0, sampleNumber - (int) sampleRate);
        */
    }
    
    /**
      | Removes all the pending messages from
      | the queue as a buffer.
      | 
      | This will also correct the messages'
      | timestamps to make sure they're in the
      | range 0 to numSamples - 1.
      | 
      | This call should be made regularly by
      | something like an audio processing
      | callback, because the time that it happens
      | is used in calculating the midi event
      | positions.
      | 
      | This method is fully thread-safe when
      | overlapping calls are made with addMessageToQueue().
      | 
      | Precondition: numSamples must be greater
      | than 0.
      |
      */
    pub fn remove_next_block_of_messages(&mut self, 
        dest_buffer: &mut MidiBuffer,
        num_samples: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (midiCallbackLock);

       #if ALOE_DEBUG
        jassert (hasCalledReset); // you need to call reset() to set the correct sample rate before using this object
       #endif

        jassert (numSamples > 0);

        auto timeNow = Time::getMillisecondCounterHiRes();
        auto msElapsed = timeNow - lastCallbackTime;

        lastCallbackTime = timeNow;

        if (! incomingMessages.isEmpty())
        {
            int numSourceSamples = jmax (1, roundToInt (msElapsed * 0.001 * sampleRate));
            int startSample = 0;
            int scale = 1 << 16;

            if (numSourceSamples > numSamples)
            {
                // if our list of events is longer than the buffer we're being
                // asked for, scale them down to squeeze them all in..
                const int maxBlockLengthToUse = numSamples << 5;

                auto iter = incomingMessages.cbegin();

                if (numSourceSamples > maxBlockLengthToUse)
                {
                    startSample = numSourceSamples - maxBlockLengthToUse;
                    numSourceSamples = maxBlockLengthToUse;
                    iter = incomingMessages.findNextSamplePosition (startSample);
                }

                scale = (numSamples << 10) / numSourceSamples;

                std::for_each (iter, incomingMessages.cend(), [&] (const MidiMessageMetadata& meta)
                {
                    const auto pos = ((meta.samplePosition - startSample) * scale) >> 10;
                    destBuffer.addEvent (meta.data, meta.numBytes, jlimit (0, numSamples - 1, pos));
                });
            }
            else
            {
                // if our event list is shorter than the number we need, put them
                // towards the end of the buffer
                startSample = numSamples - numSourceSamples;

                for (const auto metadata : incomingMessages)
                    destBuffer.addEvent (metadata.data, metadata.numBytes,
                                         jlimit (0, numSamples - 1, metadata.samplePosition + startSample));
            }

            incomingMessages.clear();
        }
        */
    }
    
    /**
      | Preallocates storage for collected
      | messages.
      | 
      | This can be called before audio processing
      | begins to ensure that there is sufficient
      | space for the expected MIDI messages,
      | in order to avoid allocations within
      | the audio callback.
      |
      */
    pub fn ensure_storage_allocated(&mut self, bytes: usize)  {
        
        todo!();
        /*
            incomingMessages.ensureSize (bytes);
        */
    }
    
}
