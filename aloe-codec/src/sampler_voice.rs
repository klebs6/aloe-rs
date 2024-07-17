crate::ix!();

/**
  | A subclass of SynthesizerVoice that can play
  | a SamplerSound.
  |
  | To use it, create a Synthesizer, add some
  | SamplerVoice objects to it, then give it some
  | SampledSound objects to play.
  |
  | @see SamplerSound, Synthesizer,
  | SynthesizerVoice
  |
  | @tags{Audio}
  */
#[leak_detector]
pub struct SamplerVoice {
    base:                   SynthesizerVoice,
    pitch_ratio:            f64, // default = 0
    source_sample_position: f64, // default = 0
    lgain:                  f32, // default = 0
    rgain:                  f32, // default = 0
    adsr:                   ADSR,
}

impl SamplerVoice {

    pub fn can_play_sound(&mut self, sound: *mut dyn SynthesizerSound) -> bool {
        
        todo!();
        /*
            return dynamic_cast<const SamplerSound*> (sound) != nullptr;
        */
    }
    
    pub fn start_note(&mut self, 
        midi_note_number:             i32,
        velocity:                     f32,
        s:                            *mut dyn SynthesizerSound,
        current_pitch_wheel_position: i32)  {
        
        todo!();
        /*
            if (auto* sound = dynamic_cast<const SamplerSound*> (s))
        {
            pitchRatio = std::pow (2.0, (midiNoteNumber - sound->midiRootNote) / 12.0)
                            * sound->sourceSampleRate / getSampleRate();

            sourceSamplePosition = 0.0;
            lgain = velocity;
            rgain = velocity;

            adsr.setSampleRate (sound->sourceSampleRate);
            adsr.setParameters (sound->params);

            adsr.noteOn();
        }
        else
        {
            jassertfalse; // this object can only play SamplerSounds!
        }
        */
    }
    
    pub fn stop_note(&mut self, 
        velocity:       f32,
        allow_tail_off: bool)  {
        
        todo!();
        /*
            if (allowTailOff)
        {
            adsr.noteOff();
        }
        else
        {
            clearCurrentNote();
            adsr.reset();
        }
        */
    }
    
    pub fn pitch_wheel_moved(&mut self, new_value: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn controller_moved(
        &mut self, 
        controller_number: i32,
        new_value:         i32

    ) {
        
        todo!();
        /*
        
        */
    }
    
    pub fn render_next_block(&mut self, 
        output_buffer: &mut AudioBuffer<f32>,
        start_sample:  i32,
        num_samples:   i32)  {
        
        todo!();
        /*
            if (auto* playingSound = static_cast<SamplerSound*> (getCurrentlyPlayingSound().get()))
        {
            auto& data = *playingSound->data;
            const float* const inL = data.getReadPointer (0);
            const float* const inR = data.getNumChannels() > 1 ? data.getReadPointer (1) : nullptr;

            float* outL = outputBuffer.getWritePointer (0, startSample);
            float* outR = outputBuffer.getNumChannels() > 1 ? outputBuffer.getWritePointer (1, startSample) : nullptr;

            while (--numSamples >= 0)
            {
                auto pos = (int) sourceSamplePosition;
                auto alpha = (float) (sourceSamplePosition - pos);
                auto invAlpha = 1.0f - alpha;

                // just using a very simple linear interpolation here..
                float l = (inL[pos] * invAlpha + inL[pos + 1] * alpha);
                float r = (inR != nullptr) ? (inR[pos] * invAlpha + inR[pos + 1] * alpha)
                                           : l;

                auto envelopeValue = adsr.getNextSample();

                l *= lgain * envelopeValue;
                r *= rgain * envelopeValue;

                if (outR != nullptr)
                {
                    *outL++ += l;
                    *outR++ += r;
                }
                else
                {
                    *outL++ += (l + r) * 0.5f;
                }

                sourceSamplePosition += pitchRatio;

                if (sourceSamplePosition > playingSound->length)
                {
                    stopNote (0.0f, false);
                    break;
                }
            }
        }
        */
    }
}
