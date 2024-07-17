crate::ix!();

#[derive(Default)]
pub struct MPEDemoSynthVoice {
    base:                        MPESynthesizerVoice,
    level:                       SmoothedValue<f64>,
    timbre:                      SmoothedValue<f64>,
    frequency:                   SmoothedValue<f64>,
    phase:                       f64, // default = 0.0
    phase_delta:                 f64, // default = 0.0
    tail_off:                    f64, // default = 0.0
    max_level:                   f64, // default = 0.05
    max_level_db:                f64, // default = 31.0
    smoothing_length_in_seconds: f64, // default = 0.01
}

impl MPEDemoSynthVoice {

    pub fn note_started(&mut self)  {
        
        todo!();
        /*
            jassert (currentlyPlayingNote.isValid());
            jassert (currentlyPlayingNote.keyState == MPENote::keyDown
                     || currentlyPlayingNote.keyState == MPENote::keyDownAndSustained);

            level    .setTargetValue (currentlyPlayingNote.pressure.asUnsignedFloat());
            frequency.setTargetValue (currentlyPlayingNote.getFrequencyInHertz());
            timbre   .setTargetValue (currentlyPlayingNote.timbre.asUnsignedFloat());

            phase = 0.0;
            auto cyclesPerSample = frequency.getNextValue() / currentSampleRate;
            phaseDelta = MathConstants<double>::twoPi * cyclesPerSample;

            tailOff = 0.0;
        */
    }
    
    pub fn note_stopped(&mut self, allow_tail_off: bool)  {
        
        todo!();
        /*
            jassert (currentlyPlayingNote.keyState == MPENote::off);

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
                phaseDelta = 0.0;
            }
        */
    }
    
    pub fn note_pressure_changed(&mut self)  {
        
        todo!();
        /*
            level.setTargetValue (currentlyPlayingNote.pressure.asUnsignedFloat());
        */
    }
    
    pub fn note_pitchbend_changed(&mut self)  {
        
        todo!();
        /*
            frequency.setTargetValue (currentlyPlayingNote.getFrequencyInHertz());
        */
    }
    
    pub fn note_timbre_changed(&mut self)  {
        
        todo!();
        /*
            timbre.setTargetValue (currentlyPlayingNote.timbre.asUnsignedFloat());
        */
    }
    
    pub fn note_key_state_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_current_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
            if (currentSampleRate != newRate)
            {
                noteStopped (false);
                currentSampleRate = newRate;

                level    .reset (currentSampleRate, smoothingLengthInSeconds);
                timbre   .reset (currentSampleRate, smoothingLengthInSeconds);
                frequency.reset (currentSampleRate, smoothingLengthInSeconds);
            }
        */
    }
    
    pub fn render_next_block(&mut self, 
        output_buffer: &mut AudioBuffer<f32>,
        start_sample:  i32,
        num_samples:   i32)  {
        
        todo!();
        /*
            if (phaseDelta != 0.0)
            {
                if (tailOff > 0.0)
                {
                    while (--numSamples >= 0)
                    {
                        auto currentSample = getNextSample() * (float) tailOff;

                        for (auto i = outputBuffer.getNumChannels(); --i >= 0;)
                            outputBuffer.addSample (i, startSample, currentSample);

                        ++startSample;

                        tailOff *= 0.99;

                        if (tailOff <= 0.005)
                        {
                            clearCurrentNote();

                            phaseDelta = 0.0;
                            break;
                        }
                    }
                }
                else
                {
                    while (--numSamples >= 0)
                    {
                        auto currentSample = getNextSample();

                        for (auto i = outputBuffer.getNumChannels(); --i >= 0;)
                            outputBuffer.addSample (i, startSample, currentSample);

                        ++startSample;
                    }
                }
            }
        */
    }
    
    pub fn get_next_sample(&mut self) -> f32 {
        
        todo!();
        /*
            auto levelDb = (level.getNextValue() - 1.0) * maxLevelDb;
            auto amplitude = pow (10.0f, 0.05f * levelDb) * maxLevel;

            // timbre is used to blend between a sine and a square.
            auto f1 = std::sin (phase);
            auto f2 = copysign (1.0, f1);
            auto a2 = timbre.getNextValue();
            auto a1 = 1.0 - a2;

            auto nextSample = float (amplitude * ((a1 * f1) + (a2 * f2)));

            auto cyclesPerSample = frequency.getNextValue() / currentSampleRate;
            phaseDelta = MathConstants<double>::twoPi * cyclesPerSample;
            phase = std::fmod (phase + phaseDelta, MathConstants<double>::twoPi);

            return nextSample;
        */
    }
}

