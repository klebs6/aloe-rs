crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPESynthesizer.h]

/**
  | Base class for an MPE-compatible musical
  | device that can play sounds.
  | 
  | This class extends MPESynthesizerBase
  | by adding the concept of voices, each
  | of which can play a sound triggered by
  | a MPENote that can be modulated by MPE
  | dimensions like pressure, pitchbend,
  | and timbre, while the note is sounding.
  | 
  | To create a synthesiser, you'll need
  | to create a subclass of MPESynthesizerVoice
  | which can play back one of these sounds
  | at a time.
  | 
  | Then you can use the addVoice() methods
  | to give the synthesiser a set of voices
  | it can use to play notes. If you only give
  | it one voice it will be monophonic - the
  | more voices it has, the more polyphony
  | it'll have available.
  | 
  | Then repeatedly call the renderNextBlock()
  | method to produce the audio (inherited
  | from MPESynthesizerBase). The voices
  | will be started, stopped, and modulated
  | automatically, based on the MPE/MIDI
  | messages that the synthesiser receives.
  | 
  | Before rendering, be sure to call the
  | setCurrentPlaybackSampleRate()
  | to tell it what the target playback rate
  | is. This value is passed on to the voices
  | so that they can pitch their output correctly.
  | 
  | @see MPESynthesizerBase, MPESynthesizerVoice,
  | MPENote, MPEInstrument
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MPESynthesizer {
    base:                 MPESynthesizerBase,
    voices:               Vec<Box<MPESynthesizerVoice>>,
    voices_lock:          CriticalSection,
    should_steal_voices:  bool, // default = false
    last_note_on_counter: u32, // default = 0
}

impl Default for MPESynthesizer {
    
    /**
      | Constructor. You'll need to add some
      | voices before it'll make any sound.
      | 
      | @see addVoice
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            MPEZoneLayout zoneLayout;
        zoneLayout.setLowerZone (15);
        setZoneLayout (zoneLayout);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPESynthesizer.cpp]
impl MPESynthesizer {

    /**
      | Returns the number of voices that have
      | been added.
      |
      */
    pub fn get_num_voices(&self) -> i32 {
        
        todo!();
        /*
            return voices.size();
        */
    }

    /**
      | If set to true, then the synth will try
      | to take over an existing voice if it runs
      | out and needs to play another note.
      | 
      | The value of this boolean is passed into
      | findFreeVoice(), so the result will
      | depend on the implementation of this
      | method.
      |
      */
    pub fn set_voice_stealing_enabled(&mut self, should_steal: bool)  {
        
        todo!();
        /*
            shouldStealVoices = shouldSteal;
        */
    }

    /**
      | Returns true if note-stealing is enabled.
      |
      */
    pub fn is_voice_stealing_enabled(&self) -> bool {
        
        todo!();
        /*
            return shouldStealVoices;
        */
    }

    /**
      | Callback for MIDI controller messages.
      | The default implementation provided
      | here does nothing; override this method
      | if you need custom MIDI controller handling
      | on top of MPE.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock().
      |
      */
    pub fn handle_controller(&mut self, 
        midi_channel:      i32,
        controller_number: i32,
        controller_value:  i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Callback for MIDI program change messages.
      | The default implementation provided
      | here does nothing; override this method
      | if you need to handle those messages.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock().
      |
      */
    pub fn handle_program_change(&mut self, 
        midi_channel:   i32,
        program_number: i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Constructor to pass to the synthesiser
      | a custom MPEInstrument object to handle
      | the MPE note state, MIDI channel assignment
      | etc. (in case you need custom logic for
      | this that goes beyond MIDI and MPE).
      | The synthesiser will take ownership
      | of this object.
      | 
      | @see MPESynthesizerBase, MPEInstrument
      |
      */
    pub fn new(mpe_instrument: *mut MPEInstrument) -> Self {
    
        todo!();
        /*
        : mpe_synthesiser_base(mpeInstrument),

        
        */
    }
    
    /**
      | Starts a specified voice and tells it
      | to play a particular MPENote. You should
      | never need to call this, it's called
      | internally by MPESynthesizerBase::instrument
      | via the noteStarted callback, but is
      | protected in case it's useful for some
      | custom subclasses.
      |
      */
    pub fn start_voice(&mut self, 
        voice:         *mut MPESynthesizerVoice,
        note_to_start: MPENote)  {
        
        todo!();
        /*
            jassert (voice != nullptr);

        voice->currentlyPlayingNote = noteToStart;
        voice->noteOnTime = lastNoteOnCounter++;
        voice->noteStarted();
        */
    }
    
    /**
      | Stops a given voice and tells it to stop
      | playing a particular MPENote (which
      | should be the same note it is actually
      | playing). You should never need to call
      | this, it's called internally by MPESynthesizerBase::instrument
      | via the noteReleased callback, but
      | is protected in case it's useful for
      | some custom subclasses.
      |
      */
    pub fn stop_voice(&mut self, 
        voice:          *mut MPESynthesizerVoice,
        note_to_stop:   MPENote,
        allow_tail_off: bool)  {
        
        todo!();
        /*
            jassert (voice != nullptr);

        voice->currentlyPlayingNote = noteToStop;
        voice->noteStopped (allowTailOff);
        */
    }
    
    /**
      | Attempts to start playing a new note.
      | 
      | The default method here will find a free
      | voice that is appropriate for playing
      | the given MPENote, and use that voice
      | to start playing the sound. If isNoteStealingEnabled
      | returns true (set this by calling setNoteStealingEnabled),
      | the synthesiser will use the voice stealing
      | algorithm to find a free voice for the
      | note (if no voices are free otherwise).
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(). Do not call it yourself,
      | otherwise the internal MPE note state
      | will become inconsistent.
      |
      */
    pub fn note_added(&mut self, new_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        if (auto* voice = findFreeVoice (newNote, shouldStealVoices))
            startVoice (voice, newNote);
        */
    }
    
    /**
      | Will find any voice that is currently
      | playing changedNote, update its currently
      | playing note, and call its notePressureChanged
      | method.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(). Do not call it yourself.
      |
      */
    pub fn note_pressure_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (voice->isCurrentlyPlayingNote (changedNote))
            {
                voice->currentlyPlayingNote = changedNote;
                voice->notePressureChanged();
            }
        }
        */
    }
    
    /**
      | Will find any voice that is currently
      | playing changedNote, update its currently
      | playing note, and call its notePitchbendChanged
      | method.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(). Do not call it yourself.
      |
      */
    pub fn note_pitchbend_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (voice->isCurrentlyPlayingNote (changedNote))
            {
                voice->currentlyPlayingNote = changedNote;
                voice->notePitchbendChanged();
            }
        }
        */
    }
    
    /**
      | Will find any voice that is currently
      | playing changedNote, update its currently
      | playing note, and call its noteTimbreChanged
      | method.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(). Do not call it yourself.
      |
      */
    pub fn note_timbre_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (voice->isCurrentlyPlayingNote (changedNote))
            {
                voice->currentlyPlayingNote = changedNote;
                voice->noteTimbreChanged();
            }
        }
        */
    }
    
    /**
      | Will find any voice that is currently
      | playing changedNote, update its currently
      | playing note, and call its noteKeyStateChanged
      | method.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(). Do not call it yourself.
      |
      */
    pub fn note_key_state_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (voice->isCurrentlyPlayingNote (changedNote))
            {
                voice->currentlyPlayingNote = changedNote;
                voice->noteKeyStateChanged();
            }
        }
        */
    }
    
    /**
      | Stops playing a note.
      | 
      | This will be called whenever an MPE note
      | is released (either by a note-off message,
      | or by a sustain/sostenuto pedal release
      | for a note that already received a note-off),
      | and should therefore stop playing.
      | 
      | This will find any voice that is currently
      | playing finishedNote, turn its currently
      | playing note off, and call its noteStopped
      | callback.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(). Do not call it yourself,
      | otherwise the internal MPE note state
      | will become inconsistent.
      |
      */
    pub fn note_released(&mut self, finished_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto i = voices.size(); --i >= 0;)
        {
            auto* voice = voices.getUnchecked (i);

            if (voice->isCurrentlyPlayingNote (finishedNote))
                stopVoice (voice, finishedNote, true);
        }
        */
    }
    
    /**
      | Tells the synthesiser what the sample
      | rate is for the audio it's being used
      | to render.
      | 
      | This overrides the implementation
      | in MPESynthesizerBase, to additionally
      | propagate the new value to the voices
      | so that they can use it to render the correct
      | pitches.
      |
      */
    pub fn set_current_playback_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
            MPESynthesizerBase::setCurrentPlaybackSampleRate (newRate);

        const ScopedLock sl (voicesLock);

        turnOffAllVoices (false);

        for (auto i = voices.size(); --i >= 0;)
            voices.getUnchecked (i)->setCurrentSampleRate (newRate);
        */
    }
    
    /**
      | Handle incoming MIDI events.
      | 
      | This method will be called automatically
      | according to the MIDI data passed into
      | renderNextBlock(), but you can also
      | call it yourself to manually inject
      | MIDI events.
      | 
      | This implementation forwards program
      | change messages and non-MPE-related
      | controller messages to handleProgramChange
      | and handleController, respectively,
      | and then simply calls through to MPESynthesizerBase::handleMidiEvent
      | to deal with MPE-related MIDI messages
      | used for MPE notes, zones etc.
      | 
      | This method can be overridden further
      | if you need to do custom MIDI handling
      | on top of what is provided here.
      |
      */
    pub fn handle_midi_event(&mut self, m: &MidiMessage)  {
        
        todo!();
        /*
            if (m.isController())
            handleController (m.getChannel(), m.getControllerNumber(), m.getControllerValue());
        else if (m.isProgramChange())
            handleProgramChange (m.getChannel(), m.getProgramChangeNumber());

        MPESynthesizerBase::handleMidiEvent (m);
        */
    }
    
    /**
      | Searches through the voices to find
      | one that's not currently playing, and
      | which can play the given MPE note.
      | 
      | If all voices are active and stealIfNoneAvailable
      | is false, this returns a nullptr. If
      | all voices are active and stealIfNoneAvailable
      | is true, this will call findVoiceToSteal()
      | to find a voice.
      | 
      | If you need to find a free voice for something
      | else than playing a note (e.g. for deleting
      | it), you can pass an invalid (default-constructed)
      | MPENote.
      |
      */
    pub fn find_free_voice(&self, 
        note_to_find_voice_for:  MPENote,
        steal_if_none_available: bool) -> *mut MPESynthesizerVoice {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (! voice->isActive())
                return voice;
        }

        if (stealIfNoneAvailable)
            return findVoiceToSteal (noteToFindVoiceFor);

        return nullptr;
        */
    }
    
    /**
      | Chooses a voice that is most suitable
      | for being re-used to play a new note,
      | or for being deleted by reduceNumVoices.
      | 
      | The default method will attempt to find
      | the oldest voice that isn't the bottom
      | or top note being played. If that's not
      | suitable for your synth, you can override
      | this method and do something more cunning
      | instead.
      | 
      | If you pass a valid MPENote for the optional
      | argument, then the note number of that
      | note will be taken into account for finding
      | the ideal voice to steal. If you pass
      | an invalid (default-constructed)
      | MPENote instead, this part of the algorithm
      | will be ignored.
      |
      */
    pub fn find_voice_to_steal(&self, note_to_steal_voice_for: Option<MPENote>) -> *mut MPESynthesizerVoice {

        let note_to_steal_voice_for: MPENote = note_to_steal_voice_for.unwrap_or(MPENote::default());
        
        todo!();
        /*
            // This voice-stealing algorithm applies the following heuristics:
        // - Re-use the oldest notes first
        // - Protect the lowest & topmost notes, even if sustained, but not if they've been released.


        // apparently you are trying to render audio without having any voices...
        jassert (voices.size() > 0);

        // These are the voices we want to protect (ie: only steal if unavoidable)
        MPESynthesizerVoice* low = nullptr; // Lowest sounding note, might be sustained, but NOT in release phase
        MPESynthesizerVoice* top = nullptr; // Highest sounding note, might be sustained, but NOT in release phase

        // this is a list of voices we can steal, sorted by how long they've been running
        Vec<MPESynthesizerVoice*> usableVoices;
        usableVoices.ensureStorageAllocated (voices.size());

        for (auto* voice : voices)
        {
            jassert (voice->isActive()); // We wouldn't be here otherwise

            usableVoices.add (voice);

            // NB: Using a functor rather than a lambda here due to scare-stories about
            // compilers generating code containing heap allocations..
            struct Sorter
            {
                bool operator() (const MPESynthesizerVoice* a, const MPESynthesizerVoice* b) const  { return a->noteOnTime < b->noteOnTime; }
            };

            std::sort (usableVoices.begin(), usableVoices.end(), Sorter());

            if (! voice->isPlayingButReleased()) // Don't protect released notes
            {
                auto noteNumber = voice->getCurrentlyPlayingNote().initialNote;

                if (low == nullptr || noteNumber < low->getCurrentlyPlayingNote().initialNote)
                    low = voice;

                if (top == nullptr || noteNumber > top->getCurrentlyPlayingNote().initialNote)
                    top = voice;
            }
        }

        // Eliminate pathological cases (ie: only 1 note playing): we always give precedence to the lowest note(s)
        if (top == low)
            top = nullptr;

        // If we want to re-use the voice to trigger a new note,
        // then The oldest note that's playing the same note number is ideal.
        if (noteToStealVoiceFor.isValid())
            for (auto* voice : usableVoices)
                if (voice->getCurrentlyPlayingNote().initialNote == noteToStealVoiceFor.initialNote)
                    return voice;

        // Oldest voice that has been released (no finger on it and not held by sustain pedal)
        for (auto* voice : usableVoices)
            if (voice != low && voice != top && voice->isPlayingButReleased())
                return voice;

        // Oldest voice that doesn't have a finger on it:
        for (auto* voice : usableVoices)
            if (voice != low && voice != top
                 && voice->getCurrentlyPlayingNote().keyState != MPENote::keyDown
                 && voice->getCurrentlyPlayingNote().keyState != MPENote::keyDownAndSustained)
                return voice;

        // Oldest voice that isn't protected
        for (auto* voice : usableVoices)
            if (voice != low && voice != top)
                return voice;

        // We've only got "protected" voices now: lowest note takes priority
        jassert (low != nullptr);

        // Duophonic synth: give priority to the bass note:
        if (top != nullptr)
            return top;

        return low;
        */
    }
    
    /**
      | Adds a new voice to the synth.
      | 
      | All the voices should be the same class
      | of object and are treated equally.
      | 
      | The object passed in will be managed
      | by the synthesiser, which will delete
      | it later on when no longer needed. The
      | caller should not retain a pointer to
      | the voice.
      |
      */
    pub fn add_voice(&mut self, new_voice: *mut MPESynthesizerVoice)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);
        newVoice->setCurrentSampleRate (getSampleRate());
        voices.add (newVoice);
        */
    }
    
    /**
      | Deletes all voices.
      |
      */
    pub fn clear_voices(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);
        voices.clear();
        */
    }
    
    /**
      | Returns one of the voices that have been
      | added.
      |
      */
    pub fn get_voice(&self, index: i32) -> *mut MPESynthesizerVoice {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);
        return voices [index];
        */
    }
    
    /**
      | Deletes one of the voices.
      |
      */
    pub fn remove_voice(&mut self, index: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);
        voices.remove (index);
        */
    }
    
    /**
      | Reduces the number of voices to newNumVoices.
      | 
      | This will repeatedly call findVoiceToSteal()
      | and remove that voice, until the total
      | number of voices equals newNumVoices.
      | If newNumVoices is greater than or equal
      | to the current number of voices, this
      | method does nothing.
      |
      */
    pub fn reduce_num_voices(&mut self, new_num_voices: i32)  {
        
        todo!();
        /*
            // we can't possibly get to a negative number of voices...
        jassert (newNumVoices >= 0);

        const ScopedLock sl (voicesLock);

        while (voices.size() > newNumVoices)
        {
            if (auto* voice = findFreeVoice ({}, true))
                voices.removeObject (voice);
            else
                voices.remove (0); // if there's no voice to steal, kill the oldest voice
        }
        */
    }
    
    /**
      | Release all MPE notes and turn off all
      | voices.
      | 
      | If allowTailOff is true, the voices
      | will be allowed to fade out the notes
      | gracefully (if they can do). If this
      | is false, the notes will all be cut off
      | immediately.
      | 
      | This method is meant to be called by the
      | user, for example to implement a MIDI
      | panic button in a synth.
      |
      */
    pub fn turn_off_all_voices(&mut self, allow_tail_off: bool)  {
        
        todo!();
        /*
            {
            const ScopedLock sl (voicesLock);

            // first turn off all voices (it's more efficient to do this immediately
            // rather than to go through the MPEInstrument for this).
            for (auto* voice : voices)
            {
                voice->currentlyPlayingNote.noteOffVelocity = MPEValue::from7BitInt (64); // some reasonable number
                voice->currentlyPlayingNote.keyState = MPENote::off;

                voice->noteStopped (allowTailOff);
            }
        }

        // finally make sure the MPE Instrument also doesn't have any notes anymore.
        instrument->releaseAllNotes();
        */
    }
    
    /**
      | This will simply call renderNextBlock
      | for each currently active voice and
      | fill the buffer with the sum. Override
      | this method if you need to do more work
      | to render your audio.
      |
      */
    pub fn render_next_sub_block_f32(&mut self, 
        buffer:       &mut AudioBuffer<f32>,
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (voice->isActive())
                voice->renderNextBlock (buffer, startSample, numSamples);
        }
        */
    }
    
    /**
      | This will simply call renderNextBlock
      | for each currently active voice and
      | fill the buffer with the sum. (double-precision
      | version) Override this method if you
      | need to do more work to render your audio.
      |
      */
    pub fn render_next_sub_block(&mut self, 
        buffer:       &mut AudioBuffer<f64>,
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            const ScopedLock sl (voicesLock);

        for (auto* voice : voices)
        {
            if (voice->isActive())
                voice->renderNextBlock (buffer, startSample, numSamples);
        }
        */
    }
}
