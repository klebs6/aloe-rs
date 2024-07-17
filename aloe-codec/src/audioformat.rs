crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormat.h]

/**
  | Subclasses of AudioFormat are used
  | to read and write different audio file
  | formats.
  | 
  | @see AudioFormatReader, AudioFormatWriter,
  | WavAudioFormat, AiffAudioFormat
  | 
  | @tags{Audio}
  |
  */
#[derive(Clone)]
pub struct AudioFormat {
    format_name:     String,
    file_extensions: Vec<String>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormat.cpp]
impl AudioFormat {

    /**
      | Creates an AudioFormat object.
      | 
      | -----------
      | @param formatName
      | 
      | this sets the value that will be returned
      | by getFormatName()
      | ----------
      | @param fileExtensions
      | 
      | an array of file extensions - these will
      | be returned by getFileExtensions()
      |
      */
    pub fn new_with_extensions_vec(
        name:       String,
        extensions: Vec<String>) -> Self {
    
        todo!();
        /*
        : format_name(name),
        : file_extensions(extensions),

        
        */
    }
    
    /**
      | Creates an AudioFormat object.
      | 
      | -----------
      | @param formatName
      | 
      | this sets the value that will be returned
      | by getFormatName()
      | ----------
      | @param fileExtensions
      | 
      | a whitespace-separated list of file
      | extensions - these will be returned
      | by getFileExtensions()
      |
      */
    pub fn new(
        name:       &str,
        extensions: &str) -> Self {
    
        todo!();
        /*


            : formatName (name.text), fileExtensions (StringArray::fromTokens (extensions, false))
        */
    }
}

impl AudioFormatInterface for AudioFormat {}

impl CanHandleFile for AudioFormat {
    
    fn can_handle_file(&mut self, f: &File) -> bool {
        
        todo!();
        /*
            for (auto& e : getFileExtensions())
            if (f.hasFileExtension (e))
                return true;

        return false;
        */
    }
}
    
impl GetFormatName for AudioFormat {

    /**
      | Returns the name of this format. e.g.
      | "WAV file" or "AIFF file"
      |
      */
    fn get_format_name(&self) -> &String {
        
        todo!();
        /*
            return formatName;
        */
    }
}
    
impl GetFileExtensions for AudioFormat {

    fn get_file_extensions(&self) -> Vec<String> {
        
        todo!();
        /*
            return fileExtensions;
        */
    }
}
    
impl IsCompressed for AudioFormat {

    fn is_compressed(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
impl GetQualityOptions for AudioFormat {

    fn get_quality_options(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return {};
        */
    }
}
    
impl CreateMemoryMappedReader for AudioFormat {

    fn create_memory_mapped_reader(
        &mut self, 
        _0: &File

    ) -> Box<dyn MemoryMappedAudioFormatReaderInterface> {
        
        todo!();
        /*
            return nullptr;
        */
    }

    fn create_memory_mapped_reader_with_file_input_stream(
        &mut self, 
        fin: *mut FileInputStream

    ) -> Box<dyn MemoryMappedAudioFormatReaderInterface> {
        
        todo!();
        /*
            delete fin;
        return nullptr;
        */
    }
}
    
impl IsChannelLayoutSupported for AudioFormat {

    fn is_channel_layout_supported(&mut self, channel_set: &AudioChannelSet) -> bool {
        
        todo!();
        /*
            if (channelSet == AudioChannelSet::mono())      return canDoMono();
        if (channelSet == AudioChannelSet::stereo())    return canDoStereo();

        return false;
        */
    }
}
    
impl CreateWriterFor for AudioFormat {

    fn create_writer_for(
        &mut self, 
        stream_to_write_to:   &mut dyn Write,
        sample_rate_to_use:   f64,
        channel_layout:       &AudioChannelSet,
        bits_per_sample:      i32,
        metadata_values:      &Vec<(String,String)>,
        quality_option_index: i32

    ) -> Box<dyn AudioFormatWriterInterface> {
        
        todo!();
        /*
            if (isChannelLayoutSupported (channelLayout))
            return createWriterFor (streamToWriteTo, sampleRateToUse,
                                    static_cast<unsigned int> (channelLayout.size()),
                                    bitsPerSample, metadataValues, qualityOptionIndex);

        return nullptr;
        */
    }
}
