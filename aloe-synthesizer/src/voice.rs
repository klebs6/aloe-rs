crate::ix!();

/**
  | Represents a voice that a Synthesizer
  | can use to play a SynthesizerSound.
  | 
  | A voice plays a single sound at a time,
  | and a synthesiser holds an array of voices
  | so that it can play polyphonically.
  | 
  | @see Synthesizer, SynthesizerSound
  | 
  | @tags{Audio}
  |
  */
#[leak_detector]
pub struct SynthesizerVoice {
    current_sample_rate:          f64,
    currently_playing_note:       i32,
    current_playing_midi_channel: i32,
    note_on_time:                 u32,
    currently_playing_sound:      Option<SynthesizerSoundPtr>,
    key_is_down:                  bool,
    sustain_pedal_down:           bool,
    sostenuto_pedal_down:         bool,
    temp_buffer:                  AudioBuffer<f32>,
}

impl Default for SynthesizerVoice {

    fn default() -> Self {
        Self {
            current_sample_rate:          44100.0,
            currently_playing_note:       -1,
            current_playing_midi_channel: 0,
            note_on_time:                 0,
            currently_playing_sound:      None,
            key_is_down:                  false,
            sustain_pedal_down:           false,
            sostenuto_pedal_down:         false,
            temp_buffer:                  AudioBuffer::<f32>::default(),
        }
    }
}

impl SynthesizerVoice {

    /**
      | Returns the midi note that this voice
      | is currently playing. Returns a value
      | less than 0 if no note is playing.
      |
      */
    pub fn get_currently_playing_note(&self) -> i32 {
        
        todo!();
        /*
            return currentlyPlayingNote;
        */
    }

    /**
      | Returns the sound that this voice is
      | currently playing. Returns nullptr
      | if it's not playing.
      |
      */
    pub fn get_currently_playing_sound(&self) -> SynthesizerSoundPtr {
        
        todo!();
        /*
            return currentlyPlayingSound;
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
      | Returns true if the key that triggered
      | this voice is still held down. Note that
      | the voice may still be playing after
      | the key was released (e.g because the
      | sostenuto pedal is down).
      |
      */
    pub fn is_key_down(&self) -> bool {
        
        todo!();
        /*
            return keyIsDown;
        */
    }

    /**
      | Allows you to modify the flag indicating
      | that the key that triggered this voice
      | is still held down. @see isKeyDown
      |
      */
    pub fn set_key_down(&mut self, is_now_down: bool)  {
        
        todo!();
        /*
            keyIsDown = isNowDown;
        */
    }

    /**
      | Returns true if the sustain pedal is
      | currently active for this voice.
      |
      */
    pub fn is_sustain_pedal_down(&self) -> bool {
        
        todo!();
        /*
            return sustainPedalDown;
        */
    }

    /**
      | Modifies the sustain pedal flag.
      |
      */
    pub fn set_sustain_pedal_down(&mut self, is_now_down: bool)  {
        
        todo!();
        /*
            sustainPedalDown = isNowDown;
        */
    }

    /**
      | Returns true if the sostenuto pedal
      | is currently active for this voice.
      |
      */
    pub fn is_sostenuto_pedal_down(&self) -> bool {
        
        todo!();
        /*
            return sostenutoPedalDown;
        */
    }

    /**
      | Modifies the sostenuto pedal flag.
      |
      */
    pub fn set_sostenuto_pedal_down(&mut self, is_now_down: bool)  {
        
        todo!();
        /*
            sostenutoPedalDown = isNowDown;
        */
    }

    /**
      | Returns true if a voice is sounding in
      | its release phase *
      |
      */
    pub fn is_playing_but_released(&self) -> bool {
        
        todo!();
        /*
            return isVoiceActive() && ! (isKeyDown() || isSostenutoPedalDown() || isSustainPedalDown());
        */
    }

    pub fn is_playing_channel(&self, midi_channel: i32) -> bool {
        
        todo!();
        /*
            return currentPlayingMidiChannel == midiChannel;
        */
    }
    
    pub fn set_current_playback_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
            currentSampleRate = newRate;
        */
    }
    
    pub fn is_voice_active(&self) -> bool {
        
        todo!();
        /*
            return getCurrentlyPlayingNote() >= 0;
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
            currentlyPlayingNote = -1;
        currentlyPlayingSound = nullptr;
        currentPlayingMidiChannel = 0;
        */
    }
    
    pub fn aftertouch_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn channel_pressure_changed(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns true if this voice started playing
      | its current note before the other voice
      | did.
      |
      */
    pub fn was_started_before(&self, other: &SynthesizerVoice) -> bool {
        
        todo!();
        /*
            return noteOnTime < other.noteOnTime;
        */
    }
    
    pub fn render_next_block<T: Float>(&mut self, 
        output_buffer: &mut AudioBuffer<T>,
        start_sample:  i32,
        num_samples:   i32)  {
        
        todo!();
        /*
            AudioBuffer<double> subBuffer (outputBuffer.getArrayOfWritePointers(),
                                       outputBuffer.getNumChannels(),
                                       startSample, numSamples);

        tempBuffer.makeCopyOf (subBuffer, true);
        renderNextBlock (tempBuffer, 0, numSamples);
        subBuffer.makeCopyOf (tempBuffer, true);
        */
    }
}
