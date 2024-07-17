#[cfg(feature = "ALOE_USE_MP3AUDIOFORMAT")] //see lib.rs for disclaimer
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_LAMEEncoderAudioFormat.h]

/**
  | An AudioFormat class which can use an
  | installed version of the LAME mp3 encoder
  | to encode a file.
  | 
  | This format can't read MP3s, it just
  | writes them. Internally, the
  | AudioFormatWriter object that is returned
  | writes the incoming audio data to a temporary
  | WAV file, and then when the writer is
  | deleted, it invokes the LAME executable
  | to convert the data to an MP3, whose data
  | is then piped into the original OutputStream
  | that was used when first creating the
  | writer.
  | 
  | @see AudioFormat
  | 
  | @tags{Audio}
  |
  */
#[cfg(feature = "lame-audio-format")]
#[no_copy]
#[leak_detector]
pub struct LAMEEncoderAudioFormat {
    base:     AudioFormat,
    lame_app: File,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_LAMEEncoderAudioFormat.cpp]
#[cfg(feature = "lame-audio-format")]
impl LAMEEncoderAudioFormat {

    /**
      | Creates a LAMEEncoderAudioFormat
      | that expects to find a working LAME executable
      | at the location given.
      |
      */
    pub fn new(lame_application: &File) -> Self {
    
        todo!();
        /*
          : AudioFormat ("MP3 file", ".mp3"),
          lameApp (lameApplication)
        */
    }
    
    pub fn can_handle_file(&mut self, _0: &File) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 32000, 44100, 48000 };
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 16 };
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
    
    pub fn is_compressed(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_quality_options(&mut self) -> StringArray {
        
        todo!();
        /*
            static const char* vbrOptions[] = { "VBR quality 0 (best)", "VBR quality 1", "VBR quality 2", "VBR quality 3",
                                            "VBR quality 4 (normal)", "VBR quality 5", "VBR quality 6", "VBR quality 7",
                                            "VBR quality 8", "VBR quality 9 (smallest)", nullptr };
        StringArray opts (vbrOptions);

        const int cbrRates[] = { 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320 };

        for (int i = 0; i < numElementsInArray (cbrRates); ++i)
            opts.add (String (cbrRates[i]) + " Kb/s CBR");

        return opts;
        */
    }
    
    pub fn create_reader_for(&mut self, 
        _0: *mut InputStream,
        _1: bool) -> *mut AudioFormatReader {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn create_writer_for(&mut self, 
        stream_to_write_to:   *mut OutputStream,
        sample_rate_to_use:   f64,
        number_of_channels:   u32,
        bits_per_sample:      i32,
        metadata_values:      &StringPairArray,
        quality_option_index: i32) -> *mut AudioFormatWriter {
        
        todo!();
        /*
            if (streamToWriteTo == nullptr)
            return nullptr;

        int vbr = 4;
        int cbr = 0;

        const String qual (getQualityOptions() [qualityOptionIndex]);

        if (qual.contains ("VBR"))
            vbr = qual.retainCharacters ("0123456789").getIntValue();
        else
            cbr = qual.getIntValue();

        return new LameEncoderAudioFormatWriter (streamToWriteTo, getFormatName(), lameApp, vbr, cbr,
                           sampleRateToUse, numberOfChannels, bitsPerSample, metadataValues);
        */
    }
}
