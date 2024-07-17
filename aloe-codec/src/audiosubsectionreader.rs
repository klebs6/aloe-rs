crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioSubsectionReader.h]

/**
  | This class is used to wrap an AudioFormatReader
  | and only read from a subsection of the
  | file.
  | 
  | So if you have a reader which can read
  | a 1000 sample file, you could wrap it
  | in one of these to only access, e.g. samples
  | 100 to 200, and any samples outside that
  | will come back as 0. Accessing sample
  | 0 from this reader will actually read
  | the first sample from the other's subsection,
  | which might be at a non-zero position.
  | 
  | @see AudioFormatReader
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioSubsectionReader<'a> {
    base:                       AudioFormatReader<'a>,
    source:                     *const AudioFormatReader<'a>,
    start_sample:               i64,
    length:                     i64,
    delete_source_when_deleted: bool,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_AudioSubsectionReader.cpp]
impl<'a> Drop for AudioSubsectionReader<'a> {

    fn drop(&mut self) {

        todo!();

        /*
            if (deleteSourceWhenDeleted)
            delete source;
        */
    }
}

impl<'a> AudioSubsectionReader<'a> {
    
    /**
      | Creates an AudioSubsectionReader
      | for a given data source.
      | 
      | -----------
      | @param sourceReader
      | 
      | the source reader from which we'll be
      | taking data
      | ----------
      | @param subsectionStartSample
      | 
      | the sample within the source reader
      | which will be mapped onto sample 0 for
      | this reader.
      | ----------
      | @param subsectionLength
      | 
      | the number of samples from the source
      | that will make up the subsection. If
      | this reader is asked for any samples
      | beyond this region, it will return zero.
      | ----------
      | @param deleteSourceWhenDeleted
      | 
      | if true, the sourceReader object will
      | be deleted when this object is deleted.
      |
      */
    pub fn new(
        source_to_use:       *mut AudioFormatReader<'a>,
        start_sample_to_use: i64,
        length_to_use:       i64,
        delete_source:       bool) -> Self {
    
        todo!();
        /*


            : AudioFormatReader (nullptr, sourceToUse->getFormatName()),
         source (sourceToUse),
         startSample (startSampleToUse),
         deleteSourceWhenDeleted (deleteSource)

        length = jmin (jmax ((int64) 0, source->lengthInSamples - startSample), lengthToUse);

        sampleRate = source->sampleRate;
        bitsPerSample = source->bitsPerSample;
        lengthInSamples = length;
        numChannels = source->numChannels;
        usesFloatingPointData = source->usesFloatingPointData;
        */
    }
    
    pub fn read_samples(
        &mut self, 
        dest_samples:                *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32
    ) -> bool {
        
        todo!();
        /*
            clearSamplesBeyondAvailableLength (destSamples, numDestChannels, startOffsetInDestBuffer,
                                           startSampleInFile, numSamples, length);

        return source->readSamples (destSamples, numDestChannels, startOffsetInDestBuffer,
                                    startSampleInFile + startSample, numSamples);
        */
    }
    
    pub fn read_max_levels(
        &mut self, 
        start_sample_in_file: i64,
        num_samples:          i64,
        results:              *mut Range<f32>,
        num_channels_to_read: i32
    )  {
        
        todo!();
        /*
            startSampleInFile = jmax ((int64) 0, startSampleInFile);
        numSamples = jmax ((int64) 0, jmin (numSamples, length - startSampleInFile));

        source->readMaxLevels (startSampleInFile + startSample, numSamples, results, numChannelsToRead);
        */
    }
}
