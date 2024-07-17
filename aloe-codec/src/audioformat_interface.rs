crate::ix!();

pub trait AudioFormatInterface {

    /**
      | Returns all the file extensions that
      | might apply to a file of this format.
      | 
      | The first item will be the one that's
      | preferred when creating a new file.
      | 
      | So for a wav file this might just return
      | ".wav"; for an AIFF file it might return
      | two items, ".aif" and ".aiff"
      |
      */
    fn get_file_extensions(&self) -> Vec<String>;

    /**
      | Returns true if this the given file can
      | be read by this format.
      | 
      | Subclasses shouldn't do too much work
      | here, just check the extension or file
      | type. The base class implementation
      | just checks the file's extension against
      | one of the ones that was registered in
      | the constructor.
      |
      */
    fn can_handle_file(&mut self, file_to_test: &File) -> bool;

    /**
      | Returns a set of sample rates that the
      | format can read and write.
      |
      */
    fn get_possible_sample_rates(&mut self) -> Vec<i32>;

    /**
      | Returns a set of bit depths that the format
      | can read and write.
      |
      */
    fn get_possible_bit_depths(&mut self) -> Vec<i32>;

    /**
      | Returns true if the format can do 2-channel
      | audio.
      |
      */
    fn can_do_stereo(&mut self) -> bool;

    /**
      | Returns true if the format can do 1-channel
      | audio.
      |
      */
    fn can_do_mono(&mut self) -> bool;

    /**
      | Returns true if the format uses compressed
      | data.
      |
      */
    fn is_compressed(&mut self) -> bool;

    /**
      | Returns true if the channel layout is
      | supported by this format.
      |
      */
    fn is_channel_layout_supported(&mut self, channel_set: &AudioChannelSet) -> bool;

    /**
      | Returns a list of different qualities
      | that can be used when writing.
      | 
      | Non-compressed formats will just return
      | an empty array, but for something like
      | Ogg-Vorbis or MP3, it might return a
      | list of bit-rates, etc.
      | 
      | When calling createWriterFor(), an
      | index from this array is passed in to
      | tell the format which option is required.
      |
      */
    fn get_quality_options(&mut self) -> Vec<String>;

    /**
      | Tries to create an object that can read
      | from a stream containing audio data
      | in this format.
      | 
      | The reader object that is returned can
      | be used to read from the stream, and should
      | then be deleted by the caller.
      | 
      | -----------
      | @param sourceStream
      | 
      | the stream to read from - the AudioFormatReader
      | object that is returned will delete
      | this stream when it no longer needs it.
      | ----------
      | @param deleteStreamIfOpeningFails
      | 
      | if no reader can be created, this determines
      | whether this method should delete the
      | stream object that was passed-in. (If
      | a valid reader is returned, it will always
      | be in charge of deleting the stream,
      | so this parameter is ignored) @see AudioFormatReader
      |
      */
    fn create_reader_for<'a>(
        &mut self, 
        source_stream:                  *mut dyn Read,
        delete_stream_if_opening_fails: bool

    ) -> *mut AudioFormatReader<'a>;

    /**
      | Attempts to create a MemoryMappedAudioFormatReader,
      | if possible for this format.
      | 
      | If the format does not support this,
      | the method will return nullptr;
      |
      */
    fn create_memory_mapped_reader<'a, R: Read>(&mut self, file: &File) -> *mut MemoryMappedAudioFormatReader<'a, R>;

    fn create_memory_mapped_reader_from_stream<'a, R: Read>(
        &mut self, 
        fin: *mut FileInputStream
    ) -> *mut MemoryMappedAudioFormatReader<'a, R>;

    /**
      | Tries to create an object that can write
      | to a stream with this audio format.
      | 
      | The writer object that is returned can
      | be used to write to the stream, and should
      | then be deleted by the caller.
      | 
      | If the stream can't be created for some
      | reason (e.g. the parameters passed
      | in here aren't suitable), this will
      | return nullptr.
      | 
      | -----------
      | @param streamToWriteTo
      | 
      | the stream that the data will go to - this
      | will be deleted by the AudioFormatWriter
      | object when it's no longer needed. If
      | no AudioFormatWriter can be created
      | by this method, the stream will NOT be
      | deleted, so that the caller can re-use
      | it to try to open a different format,
      | etc
      | ----------
      | @param sampleRateToUse
      | 
      | the sample rate for the file, which must
      | be one of the ones returned by getPossibleSampleRates()
      | ----------
      | @param numberOfChannels
      | 
      | the number of channels
      | ----------
      | @param bitsPerSample
      | 
      | the bits per sample to use - this must
      | be one of the values returned by getPossibleBitDepths()
      | ----------
      | @param metadataValues
      | 
      | a set of metadata values that the writer
      | should try to write to the stream. Exactly
      | what these are depends on the format,
      | and the subclass doesn't actually have
      | to do anything with them if it doesn't
      | want to. Have a look at the specific format
      | implementation classes to see possible
      | values that can be used
      | ----------
      | @param qualityOptionIndex
      | 
      | the index of one of compression qualities
      | returned by the getQualityOptions()
      | method. If there aren't any quality
      | options for this format, just pass 0
      | in this parameter, as it'll be ignored
      | @see AudioFormatWriter
      |
      */
    fn create_writer_for<'a, W: Write>(
        &mut self, 
        stream_to_write_to:   *mut W,
        sample_rate_to_use:   f64,
        number_of_channels:   u32,
        bits_per_sample:      i32,
        metadata_values:      &Vec<(String,String)>,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<'a, W>;

    /**
      | Tries to create an object that can write
      | to a stream with this audio format.
      | 
      | The writer object that is returned can
      | be used to write to the stream, and should
      | then be deleted by the caller.
      | 
      | If the stream can't be created for some
      | reason (e.g. the parameters passed
      | in here aren't suitable), this will
      | return nullptr.
      | 
      | -----------
      | @param streamToWriteTo
      | 
      | the stream that the data will go to - this
      | will be deleted by the AudioFormatWriter
      | object when it's no longer needed. If
      | no AudioFormatWriter can be created
      | by this method, the stream will NOT be
      | deleted, so that the caller can re-use
      | it to try to open a different format,
      | etc
      | ----------
      | @param sampleRateToUse
      | 
      | the sample rate for the file, which must
      | be one of the ones returned by getPossibleSampleRates()
      | ----------
      | @param channelLayout
      | 
      | the channel layout for the file. Use
      | isChannelLayoutSupported to check
      | if the writer supports this layout.
      | ----------
      | @param bitsPerSample
      | 
      | the bits per sample to use - this must
      | be one of the values returned by getPossibleBitDepths()
      | ----------
      | @param metadataValues
      | 
      | a set of metadata values that the writer
      | should try to write to the stream. Exactly
      | what these are depends on the format,
      | and the subclass doesn't actually have
      | to do anything with them if it doesn't
      | want to. Have a look at the specific format
      | implementation classes to see possible
      | values that can be used
      | ----------
      | @param qualityOptionIndex
      | 
      | the index of one of compression qualities
      | returned by the getQualityOptions()
      | method. If there aren't any quality
      | options for this format, just pass 0
      | in this parameter, as it'll be ignored
      | @see AudioFormatWriter
      |
      */
    fn create_writer_with_channel_layout_for<'a, W: Write>(
        &mut self, 
        stream_to_write_to:   *mut W,
        sample_rate_to_use:   f64,
        channel_layout:       &AudioChannelSet,
        bits_per_sample:      i32,
        metadata_values:      &Vec<(String,String)>,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<'a, W>;
}
