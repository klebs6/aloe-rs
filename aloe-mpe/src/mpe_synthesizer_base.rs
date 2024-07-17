crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPESynthesizerBase.h]

pub trait MpeSynthesizerBaseInterface {

    /**
      | Implement this method to render your
      | audio inside. @see renderNextBlock
      |
      */
    fn render_next_sub_block<T: Float>(&mut self, 
            output_audio: &mut AudioBuffer<T>,
            start_sample: i32,
            num_samples:  i32);

    /**
      | Implement this method if you want to
      | render 64-bit audio as well; otherwise
      | leave blank.
      |
      */
    fn render_next_sub_block_double(&mut self, 
        output_audio: &mut AudioBuffer<f64>,
        start_sample: i32,
        num_samples:  i32)  { }
}

/**
  | Derive from this class to create a basic
  | audio generator capable of MPE. Implement
  | the callbacks of MPEInstrument::Listener
  | (noteAdded, notePressureChanged
  | etc.) to let your audio generator know
  | that MPE notes were triggered, modulated,
  | or released. What to do inside them,
  | and how that influences your audio generator,
  | is up to you!
  | 
  | This class uses an instance of MPEInstrument
  | internally to handle the MPE note state
  | logic.
  | 
  | This class is a very low-level base class
  | for an MPE instrument. If you need something
  | more sophisticated, have a look at MPESynthesizer.
  | This class extends MPESynthesizerBase
  | by adding the concept of voices that
  | can play notes, a voice stealing algorithm,
  | and much more.
  | 
  | @see MPESynthesizer, MPEInstrument
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MPESynthesizerBase {
    instrument:                      Box<MPEInstrument>,
    note_state_lock:                 CriticalSection,
    sample_rate:                     f64, // default = 0.0
    minimum_sub_block_size:          i32, // default = 32
    sub_block_subdivision_is_strict: bool, // default = false
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/mpe/aloe_MPESynthesizerBase.cpp]
impl MpeInstrumentListener for MPESynthesizerBase {

}

impl Default for MPESynthesizerBase {

    fn default() -> Self {
    
        todo!();
        /*


            : instrument (new MPEInstrument)
        instrument->addListener (this);
        */
    }
}
    
impl MPESynthesizerBase {

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
            return sampleRate;
        */
    }

    /**
      | Constructor.
      | 
      | If you use this constructor, the synthesiser
      | will take ownership of the provided
      | instrument object, and will use it internally
      | to handle the MPE note state logic. This
      | is useful if you want to use an instance
      | of your own class derived from MPEInstrument
      | for the MPE logic.
      |
      */
    pub fn new(inst: *mut MPEInstrument) -> Self {
    
        todo!();
        /*
        : instrument(inst),

            jassert (instrument != nullptr);
        instrument->addListener (this);
        */
    }
    
    /**
      | Returns the synthesiser's internal
      | MPE zone layout. This happens by value,
      | to enforce thread-safety and class
      | invariants.
      |
      */
    pub fn get_zone_layout(&self) -> MPEZoneLayout {
        
        todo!();
        /*
            return instrument->getZoneLayout();
        */
    }
    
    /**
      | Re-sets the synthesiser's internal
      | MPE zone layout to the one passed in.
      | As a side effect, this will discard all
      | currently playing notes, call noteReleased
      | for all of them, and disable legacy mode
      | (if previously enabled).
      |
      */
    pub fn set_zone_layout(&mut self, new_layout: MPEZoneLayout)  {
        
        todo!();
        /*
            instrument->setZoneLayout (newLayout);
        */
    }
    
    /**
      | Puts the synthesiser into legacy mode.
      | 
      | -----------
      | @param pitchbendRange
      | 
      | The note pitchbend range in semitones
      | to use when in legacy mode. Must be between
      | 0 and 96, otherwise behaviour is undefined.
      | The default pitchbend range in legacy
      | mode is +/- 2 semitones.
      | ----------
      | @param channelRange
      | 
      | The range of MIDI channels to use for
      | notes when in legacy mode. The default
      | is to use all MIDI channels (1-16).
      | 
      | To get out of legacy mode, set a new MPE
      | zone layout using setZoneLayout.
      |
      */
    pub fn enable_legacy_mode(
        &mut self, 
        pitchbend_range: Option<i32>,
        channel_range:   Option<Range<i32>>
    ) {

        let pitchbend_range: i32 = pitchbend_range.unwrap_or(2);
        let channel_range = channel_range.unwrap_or(1.. 17);
        
        todo!();
        /*
            instrument->enableLegacyMode (pitchbendRange, channelRange);
        */
    }
    
    /**
      | Returns true if the instrument is in
      | legacy mode, false otherwise.
      |
      */
    pub fn is_legacy_mode_enabled(&self) -> bool {
        
        todo!();
        /*
            return instrument->isLegacyModeEnabled();
        */
    }
    
    /**
      | Returns the range of MIDI channels (1-16)
      | to be used for notes when in legacy mode.
      |
      */
    pub fn get_legacy_mode_channel_range(&self) -> Range<i32> {
        
        todo!();
        /*
            return instrument->getLegacyModeChannelRange();
        */
    }
    
    /**
      | Re-sets the range of MIDI channels (1-16)
      | to be used for notes when in legacy mode.
      |
      */
    pub fn set_legacy_mode_channel_range(&mut self, channel_range: Range<i32>)  {
        
        todo!();
        /*
            instrument->setLegacyModeChannelRange (channelRange);
        */
    }
    
    /**
      | Returns the pitchbend range in semitones
      | (0-96) to be used for notes when in legacy
      | mode.
      |
      */
    pub fn get_legacy_mode_pitchbend_range(&self) -> i32 {
        
        todo!();
        /*
            return instrument->getLegacyModePitchbendRange();
        */
    }
    
    /**
      | Re-sets the pitchbend range in semitones
      | (0-96) to be used for notes when in legacy
      | mode.
      |
      */
    pub fn set_legacy_mode_pitchbend_range(&mut self, pitchbend_range: i32)  {
        
        todo!();
        /*
            instrument->setLegacyModePitchbendRange (pitchbendRange);
        */
    }
    
    /**
      | Set the MPE tracking mode for the pressure
      | dimension.
      |
      */
    pub fn set_pressure_tracking_mode(&mut self, mode_to_use: MpeInstrumentTrackingMode)  {
        
        todo!();
        /*
            instrument->setPressureTrackingMode (modeToUse);
        */
    }
    
    /**
      | Set the MPE tracking mode for the pitchbend
      | dimension.
      |
      */
    pub fn set_pitchbend_tracking_mode(&mut self, mode_to_use: MpeInstrumentTrackingMode)  {
        
        todo!();
        /*
            instrument->setPitchbendTrackingMode (modeToUse);
        */
    }
    
    /**
      | Set the MPE tracking mode for the timbre
      | dimension.
      |
      */
    pub fn set_timbre_tracking_mode(&mut self, mode_to_use: MpeInstrumentTrackingMode)  {
        
        todo!();
        /*
            instrument->setTimbreTrackingMode (modeToUse);
        */
    }
    
    /**
      | Handle incoming MIDI events (called
      | from renderNextBlock).
      | 
      | The default implementation provided
      | here simply forwards everything to
      | MPEInstrument::processNextMidiEvent,
      | where it is used to update the MPE notes,
      | zones etc. MIDI messages not relevant
      | for MPE are ignored.
      | 
      | This method can be overridden if you
      | need to do custom MIDI handling on top
      | of MPE. The MPESynthesizer class overrides
      | this to implement callbacks for MIDI
      | program changes and non-MPE-related
      | MIDI controller messages.
      |
      */
    pub fn handle_midi_event(&mut self, m: &MidiMessage)  {
        
        todo!();
        /*
            instrument->processNextMidiEvent (m);
        */
    }
    
    /**
      | Creates the next block of audio output.
      | 
      | Call this to make sound. This will chop
      | up the AudioBuffer into subBlock pieces
      | separated by events in the MIDI buffer,
      | and then call renderNextSubBlock on
      | each one of them. In between you will
      | get calls to noteAdded/Changed/Finished,
      | where you can update parameters that
      | depend on those notes to use for your
      | audio rendering.
      | 
      | -----------
      | @param outputAudio
      | 
      | Buffer into which audio will be rendered
      | ----------
      | @param inputMidi
      | 
      | MIDI events to process
      | ----------
      | @param startSample
      | 
      | The first sample to process in both buffers
      | ----------
      | @param numSamples
      | 
      | The number of samples to process
      |
      */
    pub fn render_next_block<FloatType: num::Float>(&mut self, 
        output_audio: &mut AudioBuffer<FloatType>,
        input_midi:   &MidiBuffer,
        start_sample: i32,
        num_samples:  i32)  {
    
        todo!();
        /*
            // you must set the sample rate before using this!
        jassert (sampleRate != 0);

        const ScopedLock sl (noteStateLock);

        auto prevSample = startSample;
        const auto endSample = startSample + numSamples;

        for (auto it = inputMidi.findNextSamplePosition (startSample); it != inputMidi.cend(); ++it)
        {
            const auto metadata = *it;

            if (metadata.samplePosition >= endSample)
                break;

            const auto smallBlockAllowed = (prevSample == startSample && ! subBlockSubdivisionIsStrict);
            const auto thisBlockSize = smallBlockAllowed ? 1 : minimumSubBlockSize;

            if (metadata.samplePosition >= prevSample + thisBlockSize)
            {
                renderNextSubBlock (outputAudio, prevSample, metadata.samplePosition - prevSample);
                prevSample = metadata.samplePosition;
            }

            handleMidiEvent (metadata.getMessage());
        }

        if (prevSample < endSample)
            renderNextSubBlock (outputAudio, prevSample, endSample - prevSample);
        */
    }
    
    /**
      | Tells the synthesiser what the sample
      | rate is for the audio it's being used
      | to render.
      |
      */
    pub fn set_current_playback_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
            if (sampleRate != newRate)
        {
            const ScopedLock sl (noteStateLock);
            instrument->releaseAllNotes();
            sampleRate = newRate;
        }
        */
    }
    
    /**
      | Sets a minimum limit on the size to which
      | audio sub-blocks will be divided when
      | rendering.
      | 
      | When rendering, the audio blocks that
      | are passed into renderNextBlock()
      | will be split up into smaller blocks
      | that lie between all the incoming midi
      | messages, and it is these smaller sub-blocks
      | that are rendered with multiple calls
      | to renderVoices().
      | 
      | Obviously in a pathological case where
      | there are midi messages on every sample,
      | then renderVoices() could be called
      | once per sample and lead to poor performance,
      | so this setting allows you to set a lower
      | limit on the block size.
      | 
      | The default setting is 32, which means
      | that midi messages are accurate to about
      | < 1ms accuracy, which is probably fine
      | for most purposes, but you may want to
      | increase or decrease this value for
      | your synth.
      | 
      | If shouldBeStrict is true, the audio
      | sub-blocks will strictly never be smaller
      | than numSamples.
      | 
      | If shouldBeStrict is false (default),
      | the first audio sub-block in the buffer
      | is allowed to be smaller, to make sure
      | that the first MIDI event in a buffer
      | will always be sample-accurate (this
      | can sometimes help to avoid quantisation
      | or phasing issues).
      |
      */
    pub fn set_minimum_rendering_subdivision_size(
        &mut self, 
        num_samples:      i32,
        should_be_strict: Option<bool>
    ) {

        let should_be_strict: bool = should_be_strict.unwrap_or(false);
        
        todo!();
        /*
            jassert (numSamples > 0); // it wouldn't make much sense for this to be less than 1
        minimumSubBlockSize = numSamples;
        subBlockSubdivisionIsStrict = shouldBeStrict;
        */
    }
}
