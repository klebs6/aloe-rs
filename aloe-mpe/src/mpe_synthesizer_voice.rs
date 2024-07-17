crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPESynthesizerVoice.h]
pub trait MpeSynthesizerVoiceInterface {

    /**
      | Called by the MPESynthesizer to let
      | the voice know that a new note has started
      | on it. This will be called during the
      | rendering callback, so must be fast
      | and thread-safe.
      |
      */
    fn note_started(&mut self);

    /**
      | Called by the MPESynthesizer to let
      | the voice know that its currently playing
      | note has stopped. This will be called
      | during the rendering callback, so must
      | be fast and thread-safe.
      | 
      | If allowTailOff is false or the voice
      | doesn't want to tail-off, then it must
      | stop all sound immediately, and must
      | call clearCurrentNote() to reset the
      | state of this voice and allow the synth
      | to reassign it another sound.
      | 
      | If allowTailOff is true and the voice
      | decides to do a tail-off, then it's allowed
      | to begin fading out its sound, and it
      | can stop playing until it's finished.
      | As soon as it finishes playing (during
      | the rendering callback), it must make
      | sure that it calls clearCurrentNote().
      |
      */
    fn note_stopped(&mut self, allow_tail_off: bool);

    /**
      | Called by the MPESynthesizer to let
      | the voice know that its currently playing
      | note has changed its pressure value.
      | This will be called during the rendering
      | callback, so must be fast and thread-safe.
      |
      */
    fn note_pressure_changed(&mut self);

    /**
      | Called by the MPESynthesizer to let
      | the voice know that its currently playing
      | note has changed its pitchbend value.
      | This will be called during the rendering
      | callback, so must be fast and thread-safe.
      | 
      | Note: You can call currentlyPlayingNote.getFrequencyInHertz()
      | to find out the effective frequency
      | of the note, as a sum of the initial note
      | number, the per-note pitchbend and
      | the master pitchbend.
      |
      */
    fn note_pitchbend_changed(&mut self);

    /**
      | Called by the MPESynthesizer to let
      | the voice know that its currently playing
      | note has changed its timbre value. This
      | will be called during the rendering
      | callback, so must be fast and thread-safe.
      |
      */
    fn note_timbre_changed(&mut self);

    /**
      | Called by the MPESynthesizer to let
      | the voice know that its currently playing
      | note has changed its key state. This
      | typically happens when a sustain or
      | sostenuto pedal is pressed or released
      | (on an MPE channel relevant for this
      | note), or if the note key is lifted while
      | the sustained or sostenuto pedal is
      | still held down. This will be called
      | during the rendering callback, so must
      | be fast and thread-safe.
      |
      */
    fn note_key_state_changed(&mut self);

    /**
      | Renders the next block of data for this
      | voice.
      | 
      | The output audio data must be added to
      | the current contents of the buffer provided.
      | Only the region of the buffer between
      | startSample and (startSample + numSamples)
      | should be altered by this method.
      | 
      | If the voice is currently silent, it
      | should just return without doing anything.
      | 
      | If the sound that the voice is playing
      | finishes during the course of this rendered
      | block, it must call clearCurrentNote(),
      | to tell the synthesiser that it has finished.
      | 
      | The size of the blocks that are rendered
      | can change each time it is called, and
      | may involve rendering as little as 1
      | sample at a time. In between rendering
      | callbacks, the voice's methods will
      | be called to tell it about note and controller
      | events.
      |
      */
    fn render_next_block<T: Float>(&mut self, 
            output_buffer: &mut AudioBuffer<T>,
            start_sample:  i32,
            num_samples:   i32);

    /**
      | Renders the next block of 64-bit data
      | for this voice.
      | 
      | Support for 64-bit audio is optional.
      | You can choose to not override this method
      | if you don't need it (the default implementation
      | simply does nothing).
      |
      */
    fn render_next_block_64_bit(&mut self, 
            output_buffer: &mut AudioBuffer<f64>,
            start_sample:  i32,
            num_samples:   i32) {}

    /**
      | Changes the voice's reference sample
      | rate.
      | 
      | The rate is set so that subclasses know
      | the output rate and can set their pitch
      | accordingly.
      | 
      | This method is called by the synth, and
      | subclasses can access the current rate
      | with the currentSampleRate member.
      |
      */
    fn set_current_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
            currentSampleRate = newRate;
        */
    }
}

/**
  | Represents an MPE voice that an MPESynthesizer
  | can use to play a sound.
  | 
  | A voice plays a single sound at a time,
  | and a synthesiser holds an array of voices
  | so that it can play polyphonically.
  | 
  | @see MPESynthesizer, MPENote
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
#[derive(Default)]
pub struct MPESynthesizerVoice {

    /**
      | This will be set to an incrementing counter value
      | in MPESynthesizer::startVoice() and can be used
      | to determine the order in which voices started.
      */
    note_on_time:           u32, // default = 0

    current_sample_rate:    f64, // default = 0.0
    currently_playing_note: MPENote,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPESynthesizerVoice.cpp]
impl MPESynthesizerVoice {

    /**
      | Returns the MPENote that this voice
      | is currently playing.
      | 
      | Returns an invalid MPENote if no note
      | is playing (you can check this using
      | MPENote::isValid() or MPEVoice::isActive()).
      |
      */
    pub fn get_currently_playing_note(&self) -> MPENote {
        
        todo!();
        /*
            return currentlyPlayingNote;
        */
    }

    /**
      | Returns true if this voice is currently
      | busy playing a sound.
      | 
      | By default this just checks whether
      | getCurrentlyPlayingNote() returns
      | a valid MPE note, but can be overridden
      | for more advanced checking.
      |
      */
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return currentlyPlayingNote.isValid();
        */
    }

    /**
      | Returns the current target sample rate
      | at which rendering is being done. Subclasses
      | may need to know this so that they can
      | pitch things correctly.
      |
      */
    pub fn get_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            return currentSampleRate;
        */
    }

    /**
      | Returns true if the voice is currently
      | playing the given MPENote (as identified
      | by the note's initial note number and
      | MIDI channel).
      |
      */
    pub fn is_currently_playing_note(&self, note: MPENote) -> bool {
        
        todo!();
        /*
            return isActive() && currentlyPlayingNote.noteID == note.noteID;
        */
    }
    
    /**
      | Returns true if a voice is sounding in
      | its release phase. *
      |
      */
    pub fn is_playing_but_released(&self) -> bool {
        
        todo!();
        /*
            return isActive() && currentlyPlayingNote.keyState == MPENote::off;
        */
    }
    
    /**
      | Resets the state of this voice after
      | a sound has finished playing.
      | 
      | The subclass must call this when it finishes
      | playing a note and becomes available
      | to play new ones.
      | 
      | It must either call it in the stopNote()
      | method, or if the voice is tailing off,
      | then it should call it later during the
      | renderNextBlock method, as soon as
      | it finishes its tail-off.
      | 
      | It can also be called at any time during
      | the render callback if the sound happens
      | to have finished, e.g. if it's playing
      | a sample and the sample finishes.
      |
      */
    pub fn clear_current_note(&mut self)  {
        
        todo!();
        /*
            currentlyPlayingNote = MPENote();
        */
    }
}
