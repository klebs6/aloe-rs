crate::ix!();

/**
  | Metadata property name used when reading
  | a caf file with a MIDI chunk.
  |
  */
#[cfg(any(target_os="macos", target_os="ios"))]
pub const CORE_AUDIO_FORMAT_MIDI_DATA_BASE64: &'static str = "midiDataBase64";

/**
  | Metadata property name used when reading
  | a caf file with tempo information.
  |
  */
#[cfg(any(target_os="macos", target_os="ios"))]
pub const CORE_AUDIO_FORMAT_TEMPO:            &'static str = "tempo";

/**
  | Metadata property name used when reading
  | a caf file time signature information.
  |
  */
#[cfg(any(target_os="macos", target_os="ios"))]
pub const CORE_AUDIO_FORMAT_TIME_SIG:         &'static str = "time signature";

/**
  | Metadata property name used when reading
  | a caf file time signature information.
  |
  */
#[cfg(any(target_os="macos", target_os="ios"))]
pub const CORE_AUDIO_FORMAT_KEY_SIG:          &'static str = "key signature";

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_CoreAudioFormat.h]

/**
  | OSX and iOS only - This uses the AudioToolbox
  | framework to read any audio format that
  | the system has a codec for.
  | 
  | This should be able to understand formats
  | such as mp3, m4a, etc.
  | 
  | @see AudioFormat
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
#[cfg(any(target_os="macos", target_os="ios"))]
pub struct CoreAudioFormat {
    base: AudioFormat,
}

#[cfg(any(target_os="macos", target_os="ios"))]
impl Default for CoreAudioFormat {
    
    /**
      | Creates a format object.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_CoreAudioFormat.cpp]
#[cfg(any(target_os="macos", target_os="ios"))]
impl CoreAudioFormat {

    pub fn new() -> Self {
    
        todo!();
        /*
        : audio_format(coreAudioFormatName, findFileExtensionsForCoreAudioCodecs()),

        
        */
    }
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn can_do_stereo(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn can_do_mono(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn create_reader_for<R: Read>(
        &mut self, 
        source_stream:                  *mut R,
        delete_stream_if_opening_fails: bool

    ) -> *mut AudioFormatReader {
        
        todo!();
        /*
            std::unique_ptr<CoreAudioReader> r (new CoreAudioReader (sourceStream));

        if (r->ok)
            return r.release();

        if (! deleteStreamIfOpeningFails)
            r->input = nullptr;

        return nullptr;
        */
    }
    
    pub fn create_writer_for<W: Write>(
        &mut self, 
        _0:                   *mut OutputStream,
        sample_rate_to_use:   f64,
        number_of_channels:   u32,
        bits_per_sample:      i32,
        metadata_values:      &StringPairArray,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<W> {
        
        todo!();
        /*
            jassertfalse; // not yet implemented!
        return nullptr;
        */
    }
}
