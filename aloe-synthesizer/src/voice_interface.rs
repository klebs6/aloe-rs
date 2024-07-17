crate::ix!();

pub trait SynthesizerVoiceInterface {

    /**
      | Must return true if this voice object
      | is capable of playing the given sound.
      | 
      | If there are different classes of sound,
      | and different classes of voice, a voice
      | can choose which ones it wants to take
      | on.
      | 
      | A typical implementation of this method
      | may just return true if there's only
      | one type of voice and sound, or it might
      | check the type of the sound object passed-in
      | and see if it's one that it understands.
      |
      */
    fn can_play_sound(&mut self, _0: &mut dyn SynthesizerSound) -> bool;

    /**
      | Called to start a new note. This will
      | be called during the rendering callback,
      | so must be fast and thread-safe.
      |
      */
    fn start_note(
        &mut self, 
        midi_note_number:             i32,
        velocity:                     f32,
        sound:                        &mut dyn SynthesizerSound,
        current_pitch_wheel_position: i32
    );

    /**
      | Called to stop a note.
      | 
      | This will be called during the rendering
      | callback, so must be fast and thread-safe.
      | 
      | The velocity indicates how quickly
      | the note was released - 0 is slowly, 1
      | is quickly.
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
    fn stop_note(&mut self, 
        velocity:       f32,
        allow_tail_off: bool);

    /**
      | Called to let the voice know that the
      | pitch wheel has been moved. This will
      | be called during the rendering callback,
      | so must be fast and thread-safe.
      |
      */
    fn pitch_wheel_moved(&mut self, new_pitch_wheel_value: i32);

    /**
      | Called to let the voice know that a midi
      | controller has been moved. This will
      | be called during the rendering callback,
      | so must be fast and thread-safe.
      |
      */
    fn controller_moved(&mut self, 
        controller_number:    i32,
        new_controller_value: i32);

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
    fn render_next_block<T: Float>(
        &mut self, 
        output_buffer: &mut AudioBuffer<T>,
        start_sample:  i32,
        num_samples:   i32
    ) { }

    /**
      | Returns true if this voice is currently
      | busy playing a sound. By default this
      | just checks the getCurrentlyPlayingNote()
      | value, but can be overridden for more
      | advanced checking.
      |
      */
    fn is_voice_active(&self) -> bool {

        todo!();
        /*

        */
    }

    /**
      | Called to let the voice know that the
      | aftertouch has changed. This will be
      | called during the rendering callback,
      | so must be fast and thread-safe.
      |
      */
    fn aftertouch_changed(&mut self, new_aftertouch_value: i32)  {

        todo!();
        /*

        */
    }

    /**
      | Called to let the voice know that the
      | channel pressure has changed. This
      | will be called during the rendering
      | callback, so must be fast and thread-safe.
      |
      */
    fn channel_pressure_changed(&mut self, new_channel_pressure_value: i32)  {

        todo!();
        /*

        */
    }

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
    fn set_current_playback_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if the voice is currently
      | playing a sound which is mapped to the
      | given midi channel.
      | 
      | If it's not currently playing, this
      | will return false.
      |
      */
    fn is_playing_channel(&self, midi_channel: i32) -> bool {
        
        todo!();
        /*
        
        */
    }
}
