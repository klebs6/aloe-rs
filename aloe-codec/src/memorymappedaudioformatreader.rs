crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_MemoryMappedAudioFormatReader.h]

pub trait MemoryMappedAudioFormatReaderInterface {

    /**
      | Attempts to map a section of the file
      | into memory.
      |
      */
    fn map_section_of_file(&mut self, samples_to_map: Range<i64>) -> bool;

    /**
      | Returns the samples for all channels
      | at a given sample position.
      | 
      | The result array must be large enough
      | to hold a value for each channel that
      | this reader contains.
      |
      */
    fn get_sample(&self, 
        sample_index: i64,
        result:       *mut f32);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatReader.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioFormatReader.cpp]

/**
  | used to force the compiler not to optimise-away
  | the read operation
  |
  */
lazy_static!{
    /*
    static int memoryReadDummyVariable;
    */
}

/**
  | A specialised type of AudioFormatReader
  | that uses a MemoryMappedFile to read
  | directly from an audio file.
  | 
  | This allows for incredibly fast random-access
  | to sample data in the mapped region of
  | the file, but not all audio formats support
  | it - see
  | AudioFormat::createMemoryMappedReader().
  | 
  | -----------
  | @note
  | 
  | before reading samples from a MemoryMappedAudioFormatReader,
  | you must first call mapEntireFile()
  | or mapSectionOfFile() to ensure that
  | the region you want to read has been mapped.
  | 
  | @see AudioFormat::createMemoryMappedReader,
  | AudioFormatReader
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MemoryMappedAudioFormatReader<'a, R: Read> {
    base:             AudioFormatReader<'a>,
    file:             File,
    mapped_section:   Range<i64>,
    map:              Box<MemoryMappedFile>,
    data_chunk_start: i64,
    data_length:      i64,
    bytes_per_frame:  i32,
    _0: PhantomData<R>,
}

impl<'a, R: Read> MemoryMappedAudioFormatReader<'a, R> {

    /**
      | Returns the file that is being mapped
      |
      */
    pub fn get_file(&self) -> &File {
        
        todo!();
        /*
            return file;
        */
    }

    /**
      | Returns the sample range that's currently
      | memory-mapped and available for reading.
      |
      */
    pub fn get_mapped_section(&self) -> Range<i64> {
        
        todo!();
        /*
            return mappedSection;
        */
    }

    /**
      | Returns the number of bytes currently
      | being mapped
      |
      */
    pub fn get_num_bytes_used(&self) -> usize {
        
        todo!();
        /*
            return map != nullptr ? map->getSize() : 0;
        */
    }

    /**
      | Converts a sample index to a byte position
      | in the file.
      |
      */
    #[inline] pub fn sample_to_file_pos(&self, sample: i64) -> i64 {
        
        todo!();
        /*
            return dataChunkStart + sample * bytesPerFrame;
        */
    }

    /**
      | Converts a byte position in the file
      | to a sample index.
      |
      */
    #[inline] pub fn file_pos_to_sample(&self, file_pos: i64) -> i64 {
        
        todo!();
        /*
            return (filePos - dataChunkStart) / bytesPerFrame;
        */
    }

    /**
      | Converts a sample index to a pointer
      | to the mapped file memory.
      |
      */
    #[inline] pub fn sample_to_pointer(&self, sample: i64)  {
        
        todo!();
        /*
            return addBytesToPointer (map->getData(), sampleToFilePos (sample) - map->getRange().getStart());
        */
    }

    /**
      | Used by AudioFormatReader subclasses
      | to scan for min/max ranges in interleaved
      | data.
      |
      */
    pub fn scan_min_and_max_interleaved<SampleType, Endianness>(&self, 
        channel:              i32,
        start_sample_in_file: i64,
        num_samples:          i64) -> Range<f32> {
    
        todo!();
        /*
            using SourceType = AudioData::Pointer <SampleType, Endianness, AudioData::Interleaved, AudioData::Const>;

            return SourceType (addBytesToPointer (sampleToPointer (startSampleInFile), ((int) bitsPerSample / 8) * channel), (int) numChannels)
                    .findMinAndMax ((size_t) numSamples);
        */
    }

    /**
      | Creates an MemoryMappedAudioFormatReader
      | object.
      | 
      | -----------
      | @note
      | 
      | before attempting to read any data,
      | you must call mapEntireFile() or mapSectionOfFile()
      | to ensure that the region you want to
      | read has been mapped.
      |
      */
    pub fn new(
        f:          &File,
        reader:     &AudioFormatReader<'a>,
        start:      i64,
        length:     i64,
        frame_size: i32

    ) -> Self {
    
        todo!();
        /*


            : AudioFormatReader (nullptr, reader.getFormatName()), file (f),
          dataChunkStart (start), dataLength (length), bytesPerFrame (frameSize)

        sampleRate      = reader.sampleRate;
        bitsPerSample   = reader.bitsPerSample;
        lengthInSamples = reader.lengthInSamples;
        numChannels     = reader.numChannels;
        metadataValues  = reader.metadataValues;
        usesFloatingPointData = reader.usesFloatingPointData;
        */
    }
    
    /**
      | Attempts to map the entire file into
      | memory.
      |
      */
    pub fn map_entire_file(&mut self) -> bool {
        
        todo!();
        /*
            return mapSectionOfFile (Range<int64> (0, lengthInSamples));
        */
    }
    
    pub fn map_section_of_file(&mut self, samples_to_map: Range<i64>) -> bool {
        
        todo!();
        /*
            if (map == nullptr || samplesToMap != mappedSection)
        {
            map.reset();

            const Range<int64> fileRange (sampleToFilePos (samplesToMap.getStart()),
                                          sampleToFilePos (samplesToMap.getEnd()));

            map.reset (new MemoryMappedFile (file, fileRange, MemoryMappedFile::readOnly));

            if (map->getData() == nullptr)
                map.reset();
            else
                mappedSection = Range<int64> (jmax ((int64) 0, filePosToSample (map->getRange().getStart() + (bytesPerFrame - 1))),
                                              jmin (lengthInSamples, filePosToSample (map->getRange().getEnd())));
        }

        return map != nullptr;
        */
    }
    
    /**
      | Touches the memory for the given sample,
      | to force it to be loaded into active memory.
      |
      */
    pub fn touch_sample(&self, sample: i64)  {
        
        todo!();
        /*
            if (map != nullptr && mappedSection.contains (sample))
            memoryReadDummyVariable += *(char*) sampleToPointer (sample);
        else
            jassertfalse; // you must make sure that the window contains all the samples you're going to attempt to read.
        */
    }
}
