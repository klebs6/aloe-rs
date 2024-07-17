crate::ix!();

pub const AIFF_FORMAT_NAME: &'static str = "AIFF file";

/**
  | Metadata property names used when reading
  | a aiff file with a basc chunk.
  |
  */
pub const AIFF_AUDIO_FORMAT_APPLE_ONE_SHOT:    &'static str = "apple one shot";
pub const AIFF_AUDIO_FORMAT_APPLE_ROOT_SET:    &'static str = "apple root set";
pub const AIFF_AUDIO_FORMAT_APPLE_ROOT_NOTE:   &'static str = "apple root note";
pub const AIFF_AUDIO_FORMAT_APPLE_BEATS:       &'static str = "apple beats";
pub const AIFF_AUDIO_FORMAT_APPLE_DENOMINATOR: &'static str = "apple denominator";
pub const AIFF_AUDIO_FORMAT_APPLE_NUMERATOR:   &'static str = "apple numerator";
pub const AIFF_AUDIO_FORMAT_APPLE_TAG:         &'static str = "apple tag";
pub const AIFF_AUDIO_FORMAT_APPLE_KEY:         &'static str = "apple key";

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_AiffAudioFormat.h]

/**
  | Reads and Writes AIFF format audio files.
  | 
  | @see AudioFormat
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AiffAudioFormat {
    base: AudioFormat,
}

impl Default for AiffAudioFormat {
    
    /**
      | Creates an format object.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_AiffAudioFormat.cpp]
impl AiffAudioFormat {
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : audio_format(aiffFormatName, ".aiff .aif"),

        
        */
    }
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000 };
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 8, 16, 24 };
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

    #[cfg(target_os="macos")]
    pub fn can_handle_file(&mut self, f: &File) -> bool {
        
        todo!();
        /*
            if (AudioFormat::canHandleFile (f))
            return true;

        auto type = f.getMacOSType();

        // (NB: written as hex to avoid four-char-constant warnings)
        return type == 0x41494646 /* AIFF */ || type == 0x41494643 /* AIFC */
            || type == 0x61696666 /* aiff */ || type == 0x61696663 /* aifc */;
        */
    }
    
    pub fn create_reader_for(
        &mut self, 
        source_stream:                  *mut dyn Read,
        delete_stream_if_opening_fails: bool

    ) -> *mut AudioFormatReader {
        
        todo!();
        /*
            std::unique_ptr<AiffAudioFormatReader> w (new AiffAudioFormatReader (sourceStream));

        if (w->sampleRate > 0 && w->numChannels > 0)
            return w.release();

        if (! deleteStreamIfOpeningFails)
            w->input = nullptr;

        return nullptr;
        */
    }
    
    pub fn create_memory_mapped_reader_from_file<R: Read>(&mut self, file: &File) -> *mut MemoryMappedAudioFormatReader<R> {
        
        todo!();
        /*
            return createMemoryMappedReader (file.createInputStream().release());
        */
    }
    
    pub fn create_memory_mapped_reader<R: Read>(&mut self, fin: *mut FileInputStream) -> *mut MemoryMappedAudioFormatReader<R> {
        
        todo!();
        /*
            if (fin != nullptr)
        {
            AiffAudioFormatReader reader (fin);

            if (reader.lengthInSamples > 0)
                return new MemoryMappedAiffReader (fin->getFile(), reader);
        }

        return nullptr;
        */
    }
    
    pub fn create_writer_for<W: Write>(
        &mut self, 
        out:                  *mut W,
        sample_rate:          f64,
        number_of_channels:   u32,
        bits_per_sample:      i32,
        metadata_values:      &StringPairArray,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<W> {
        
        todo!();
        /*
            if (out != nullptr && getPossibleBitDepths().contains (bitsPerSample))
            return new AiffAudioFormatWriter (out, sampleRate, numberOfChannels,
                                              (unsigned int) bitsPerSample, metadataValues);

        return nullptr;
        */
    }
}
