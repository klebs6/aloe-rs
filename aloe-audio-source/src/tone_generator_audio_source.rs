crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ToneGeneratorAudioSource.h]

/**
  | A simple AudioSource that generates
  | a sine wave.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ToneGeneratorAudioSource {
    frequency:        f64,
    sample_rate:      f64,
    current_phase:    f64,
    phase_per_sample: f64,
    amplitude:        f32,
}

impl AudioSource for ToneGeneratorAudioSource {}

impl PrepareToPlayAudioSource for ToneGeneratorAudioSource {

    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn prepare_to_play(
        &mut self, 
        samples_per_block_expected: i32,
        rate:                       f64
    ) {
        
        todo!();
        /*
            currentPhase = 0.0;
        phasePerSample = 0.0;
        sampleRate = rate;
        */
    }
}

impl ReleaseResources for ToneGeneratorAudioSource {
    
    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetNextAudioBlock for ToneGeneratorAudioSource {
    
    /**
      | Implementation of the AudioSource
      | method.
      |
      */
    fn get_next_audio_block(&mut self, info: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            if (phasePerSample == 0.0)
            phasePerSample = MathConstants<double>::twoPi / (sampleRate / frequency);

        for (int i = 0; i < info.numSamples; ++i)
        {
            const float sample = amplitude * (float) std::sin (currentPhase);
            currentPhase += phasePerSample;

            for (int j = info.buffer->getNumChannels(); --j >= 0;)
                info.buffer->setSample (j, info.startSample + i, sample);
        }
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ToneGeneratorAudioSource.cpp]
impl Default for ToneGeneratorAudioSource {

    fn default() -> Self {
    
        todo!();
        /*
        : frequency(1000.0),
        : sample_rate(44100.0),
        : current_phase(0.0),
        : phase_per_sample(0.0),
        : amplitude(0.5f),
        */
    }
}
    
impl ToneGeneratorAudioSource {

    /**
      | Sets the signal's amplitude.
      |
      */
    pub fn set_amplitude(&mut self, new_amplitude: f32)  {
        
        todo!();
        /*
            amplitude = newAmplitude;
        */
    }
    
    /**
      | Sets the signal's frequency.
      |
      */
    pub fn set_frequency(&mut self, new_frequency_hz: f64)  {
        
        todo!();
        /*
            frequency = newFrequencyHz;
        phasePerSample = 0.0;
        */
    }
}
