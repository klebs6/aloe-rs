crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/sampler/aloe_Sampler.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/sampler/aloe_Sampler.cpp]
///---------------------

/**
  | A subclass of SynthesizerSound that represents
  | a sampled audio clip.
  |
  | This is a pretty basic sampler, and just
  | attempts to load the whole audio stream into
  | memory.
  |
  | To use it, create a Synthesizer, add some
  | SamplerVoice objects to it, then give it some
  | SampledSound objects to play.
  |
  | @see SamplerVoice, Synthesizer,
  | SynthesizerSound
  |
  | @tags{Audio}
  */
#[leak_detector]
pub struct SamplerSound {
    name:               String,
    data:               Box<AudioBuffer<f32>>,
    source_sample_rate: f64,
    midi_notes:         BigInteger,
    length:             i32, // default = 0
    midi_root_note:     i32, // default = 0
    params:             AdsrParameters,
}

impl SynthesizerSound for SamplerSound {

    fn applies_to_note(&mut self, midi_note_number: i32) -> bool {
        
        todo!();
        /*
            return midiNotes[midiNoteNumber];
        */
    }
    
    fn applies_to_channel(&mut self, midi_channel: i32) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl SamplerSound {

    /**
      | Returns the sample's name
      |
      */
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    /**
      | Returns the audio sample data. This
      | could return nullptr if there was a problem
      | loading the data.
      |
      */
    pub fn get_audio_data(&self) -> *mut AudioBuffer<f32> {
        
        todo!();
        /*
            return data.get();
        */
    }
    
    /**
      | Changes the parameters of the ADSR envelope
      | which will be applied to the sample.
      |
      */
    pub fn set_envelope_parameters(&mut self, parameters_to_use: AdsrParameters)  {
        
        todo!();
        /*
            params
            = parametersToUse;
        */
    }
    
    /**
      | Creates a sampled sound from an audio
      | reader.
      | 
      | This will attempt to load the audio from
      | the source into memory and store it in
      | this object.
      | 
      | -----------
      | @param name
      | 
      | a name for the sample
      | ----------
      | @param source
      | 
      | the audio to load. This object can be
      | safely deleted by the caller after this
      | constructor returns
      | ----------
      | @param midiNotes
      | 
      | the set of midi keys that this sound should
      | be played on. This is used by the SynthesizerSound::appliesToNote()
      | method
      | ----------
      | @param midiNoteForNormalPitch
      | 
      | the midi note at which the sample should
      | be played with its natural rate. All
      | other notes will be pitched up or down
      | relative to this one
      | ----------
      | @param attackTimeSecs
      | 
      | the attack (fade-in) time, in seconds
      | ----------
      | @param releaseTimeSecs
      | 
      | the decay (fade-out) time, in seconds
      | ----------
      | @param maxSampleLengthSeconds
      | 
      | a maximum length of audio to read from
      | the audio source, in seconds
      |
      */
    pub fn new<'a>(
        sound_name:                 &String,
        source:                     &mut AudioFormatReader<'a>,
        notes:                      &BigInteger,
        midi_note_for_normal_pitch: i32,
        attack_time_secs:           f64,
        release_time_secs:          f64,
        max_sample_length_seconds:  f64

    ) -> Self {
    
        todo!();
        /*
        : name(soundName),
        : source_sample_rate(source.sampleRate),
        : midi_notes(notes),
        : midi_root_note(midiNoteForNormalPitch),

            if (sourceSampleRate > 0 && source.lengthInSamples > 0)
        {
            length = jmin ((int) source.lengthInSamples,
                           (int) (maxSampleLengthSeconds * sourceSampleRate));

            data.reset (new AudioBuffer<float> (jmin (2, (int) source.numChannels), length + 4));

            source.read (data.get(), 0, length + 4, 0, true, true);

            params.attack  = static_cast<float> (attackTimeSecs);
            params.release = static_cast<float> (releaseTimeSecs);
        }
        */
    }
    
}
