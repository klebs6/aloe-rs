crate::ix!();

/**
  | A simple demo synth voice that just plays
  | a sine wave..
  |
  */
#[derive(Default)]
pub struct SineWaveVoice {
    base:          SynthesizerVoice,
    current_angle: f64, // default = 0.0
    angle_delta:   f64, // default = 0.0
    level:         f64, // default = 0.0
    tail_off:      f64, // default = 0.0
}

impl SineWaveVoice {
    
    pub fn can_play_sound(&mut self, sound: *mut dyn SynthesizerSound) -> bool {
        
        todo!();
        /*
            return dynamic_cast<SineWaveSound*> (sound) != nullptr;
        */
    }
    
    pub fn start_note(&mut self, 
        midi_note_number:             i32,
        velocity:                     f32,
        sound:                        *mut dyn SynthesizerSound,
        current_pitch_wheel_position: i32)  {
        
        todo!();
        /*
            currentAngle = 0.0;
            level = velocity * 0.15;
            tailOff = 0.0;

            auto cyclesPerSecond = MidiMessage::getMidiNoteInHertz (midiNoteNumber);
            auto cyclesPerSample = cyclesPerSecond / getSampleRate();

            angleDelta = cyclesPerSample * MathConstants<double>::twoPi;
        */
    }
    
    pub fn stop_note(&mut self, 
        velocity:       f32,
        allow_tail_off: bool)  {
        
        todo!();
        /*
            if (allowTailOff)
            {
                // start a tail-off by setting this flag. The render callback will pick up on
                // this and do a fade out, calling clearCurrentNote() when it's finished.

                if (tailOff == 0.0) // we only need to begin a tail-off if it's not already doing so - the
                                    // stopNote method could be called more than once.
                    tailOff = 1.0;
            }
            else
            {
                // we're being told to stop playing immediately, so reset everything..

                clearCurrentNote();
                angleDelta = 0.0;
            }
        */
    }
    
    pub fn pitch_wheel_moved(&mut self, new_value: i32)  {
        
        todo!();
        /*
            // not implemented for the purposes of this demo!
        */
    }
    
    pub fn controller_moved(&mut self, 
        controller_number: i32,
        new_value:         i32)  {
        
        todo!();
        /*
            // not implemented for the purposes of this demo!
        */
    }
    
    pub fn render_next_block(&mut self, 
        output_buffer: &mut AudioBuffer<f32>,
        start_sample:  i32,
        num_samples:   i32)  {
        
        todo!();
        /*
            if (angleDelta != 0.0)
            {
                if (tailOff > 0.0)
                {
                    while (--numSamples >= 0)
                    {
                        auto currentSample = (float) (sin (currentAngle) * level * tailOff);

                        for (auto i = outputBuffer.getNumChannels(); --i >= 0;)
                            outputBuffer.addSample (i, startSample, currentSample);

                        currentAngle += angleDelta;
                        ++startSample;

                        tailOff *= 0.99;

                        if (tailOff <= 0.005)
                        {
                            // tells the synth that this voice has stopped
                            clearCurrentNote();

                            angleDelta = 0.0;
                            break;
                        }
                    }
                }
                else
                {
                    while (--numSamples >= 0)
                    {
                        auto currentSample = (float) (sin (currentAngle) * level);

                        for (auto i = outputBuffer.getNumChannels(); --i >= 0;)
                            outputBuffer.addSample (i, startSample, currentSample);

                        currentAngle += angleDelta;
                        ++startSample;
                    }
                }
            }
        */
    }
}
