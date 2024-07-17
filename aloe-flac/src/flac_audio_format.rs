crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_FlacAudioFormat.h]

/**
  | Reads and writes the lossless-compression
  | FLAC audio format.
  | 
  | To compile this, you'll need to set the
  | ALOE_USE_FLAC flag.
  | 
  | @see AudioFormat
  | 
  | @tags{Audio}
  |
  */
#[cfg(ALOE_USE_FLAC)]
#[no_copy]
#[leak_detector]
pub struct FlacAudioFormat {
    base: AudioFormat,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_FlacAudioFormat.cpp]
#[cfg(ALOE_USE_FLAC)]
impl FlacAudioFormat {
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
        
        */
    }
    
    pub fn can_do_stereo(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn can_do_mono(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_compressed(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_quality_options(&mut self) -> StringArray {
        
        todo!();
        /*
        
        */
    }
    
    pub fn create_reader_for(&mut self, 
        source_stream:                  *mut InputStream,
        delete_stream_if_opening_fails: bool) -> *mut AudioFormatReader {
        
        todo!();
        /*
        
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
        
        */
    }
}

#[cfg(ALOE_USE_FLAC)]
pub mod namespace {

    use super::*;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    pub const VERSION: &'static str = "1.3.1";

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    pub const NO_DLL: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    #[cfg(not(ALOE_MSVC))]
    pub const HAVE_LROUND: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    #[cfg(target_os="macos")]
    pub const SYS_DARWIN: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    #[cfg(not(SIZE_MAX))]
    pub const SIZE_MAX: usize = 0xffffffff;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    #[cfg(ALOE_INTEL)]
    #[cfg(ALOE_32BIT)]
    pub const CPU_IA32: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    #[cfg(ALOE_INTEL)]
    #[cfg(ALOE_64BIT)]
    pub const CPU_X86_64: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    #[cfg(ALOE_INTEL)]
    pub const HAS_X86INTRIN: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    pub const __STDC_LIMIT_MACROS: usize = 1;

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    macro_rules! max {
        () => {
            /*
                    jmax
            */
        }
    }

    #[cfg(any(ALOE_INCLUDE_FLAC_CODE,not(ALOE_INCLUDE_FLAC_CODE)))]
    macro_rules! min {
        () => {
            /*
                    jmin
            */
        }
    }
}

#[cfg(ALOE_USE_FLAC)]
pub const format_name: &'static str = "FLAC file";

///-------------------------
#[cfg(ALOE_USE_FLAC)]
#[no_copy]
#[leak_detector]
pub struct FlacReader {
    base:                 AudioFormatReader,
    decoder:              FlacNamespace::StreamDecoder,
    reservoir:            AudioBuffer<f32>,
    reservoir_start:      i64, // default = 0
    samples_in_reservoir: i64, // default = 0
    ok:                   bool, // default = false
    scanning_for_length:  bool, // default = false
}

#[cfg(ALOE_USE_FLAC)]
impl Drop for FlacReader {
    fn drop(&mut self) {
        todo!();
        /*
            FlacNamespace::stream_decoder_delete (decoder);
        */
    }
}

#[cfg(ALOE_USE_FLAC)]
impl FlacReader {

    pub fn new(in_: *mut InputStream) -> Self {
    
        todo!();
        /*
        : audio_format_reader(in, flacFormatName),

            lengthInSamples = 0;
            decoder = FlacNamespace::stream_decoder_new();

            ok = stream_decoder_init_stream (decoder,
                                                   readCallback_, seekCallback_, tellCallback_, lengthCallback_,
                                                   eofCallback_, writeCallback_, metadataCallback_, errorCallback_,
                                                   this) == FlacNamespace::STREAM_DECODER_INIT_STATUS_OK;

            if (ok)
            {
                stream_decoder_process_until_end_of_metadata (decoder);

                if (lengthInSamples == 0 && sampleRate > 0)
                {
                    // the length hasn't been stored in the metadata, so we'll need to
                    // work it out the length the hard way, by scanning the whole file..
                    scanningForLength = true;
                    stream_decoder_process_until_end_of_stream (decoder);
                    scanningForLength = false;
                    auto tempLength = lengthInSamples;

                    stream_decoder_reset (decoder);
                    stream_decoder_process_until_end_of_metadata (decoder);
                    lengthInSamples = tempLength;
                }
            }
        */
    }
    
    pub fn use_metadata(&mut self, info: &FlacNamespace::StreamMetadata_StreamInfo)  {
        
        todo!();
        /*
            sampleRate = info.sample_rate;
            bitsPerSample = info.bits_per_sample;
            lengthInSamples = (unsigned int) info.total_samples;
            numChannels = info.channels;

            reservoir.setSize ((int) numChannels, 2 * (int) info.max_blocksize, false, false, true);
        */
    }

    /**
       returns the number of samples read
      */
    pub fn read_samples(&mut self, 
        dest_samples:                *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32) -> bool {
        
        todo!();
        /*
            if (! ok)
                return false;

            while (numSamples > 0)
            {
                if (startSampleInFile >= reservoirStart
                     && startSampleInFile < reservoirStart + samplesInReservoir)
                {
                    auto num = (int) jmin ((i64) numSamples,
                                           reservoirStart + samplesInReservoir - startSampleInFile);

                    jassert (num > 0);

                    for (int i = jmin (numDestChannels, reservoir.getNumChannels()); --i >= 0;)
                        if (destSamples[i] != nullptr)
                            memcpy (destSamples[i] + startOffsetInDestBuffer,
                                    reservoir.getReadPointer (i, (int) (startSampleInFile - reservoirStart)),
                                    (size_t) num * sizeof (int));

                    startOffsetInDestBuffer += num;
                    startSampleInFile += num;
                    numSamples -= num;
                }
                else
                {
                    if (startSampleInFile >= lengthInSamples)
                    {
                        samplesInReservoir = 0;
                    }
                    else if (startSampleInFile < reservoirStart
                              || startSampleInFile > reservoirStart + jmax (samplesInReservoir, (i64) 511))
                    {
                        // had some problems with flac crashing if the read pos is aligned more
                        // accurately than this. Probably fixed in newer versions of the library, though.
                        reservoirStart = (int) (startSampleInFile & ~511);
                        samplesInReservoir = 0;
                        stream_decoder_seek_absolute (decoder, (FlacNamespace::u64) reservoirStart);
                    }
                    else
                    {
                        reservoirStart += samplesInReservoir;
                        samplesInReservoir = 0;
                        stream_decoder_process_single (decoder);
                    }

                    if (samplesInReservoir == 0)
                        break;
                }
            }

            if (numSamples > 0)
            {
                for (int i = numDestChannels; --i >= 0;)
                    if (destSamples[i] != nullptr)
                        zeromem (destSamples[i] + startOffsetInDestBuffer, (size_t) numSamples * sizeof (int));
            }

            return true;
        */
    }
    
    pub fn use_samples(&mut self, 
        buffer:      *const &[FlacNamespace::i32],
        num_samples: i32)  {
        
        todo!();
        /*
            if (scanningForLength)
            {
                lengthInSamples += numSamples;
            }
            else
            {
                if (numSamples > reservoir.getNumSamples())
                    reservoir.setSize ((int) numChannels, numSamples, false, false, true);

                auto bitsToShift = 32 - bitsPerSample;

                for (int i = 0; i < (int) numChannels; ++i)
                {
                    auto* src = buffer[i];
                    int n = i;

                    while (src == nullptr && n > 0)
                        src = buffer [--n];

                    if (src != nullptr)
                    {
                        auto* dest = reinterpret_cast<int*> (reservoir.getWritePointer(i));

                        for (int j = 0; j < numSamples; ++j)
                            dest[j] = src[j] << bitsToShift;
                    }
                }

                samplesInReservoir = numSamples;
            }
        */
    }
    
    pub fn read_callback(
        _0:          *const FlacNamespace::StreamDecoder,
        buffer:      &[FlacNamespace::byte],
        bytes:       *mut usize,
        client_data: *mut c_void) -> FlacNamespace::StreamDecoderReadStatus {
        
        todo!();
        /*
            *bytes = (size_t) static_cast<const FlacReader*> (client_data)->input->read (buffer, (int) *bytes);
            return FlacNamespace::STREAM_DECODER_READ_STATUS_CONTINUE;
        */
    }
    
    pub fn seek_callback(
        _0:                   *const FlacNamespace::StreamDecoder,
        absolute_byte_offset: FlacNamespace::u64,
        client_data:          *mut c_void) -> FlacNamespace::StreamDecoderSeekStatus {
        
        todo!();
        /*
            static_cast<const FlacReader*> (client_data)->input->setPosition ((int) absolute_byte_offset);
            return FlacNamespace::STREAM_DECODER_SEEK_STATUS_OK;
        */
    }
    
    pub fn tell_callback(
        _0:                   *const FlacNamespace::StreamDecoder,
        absolute_byte_offset: *mut FlacNamespace::u64,
        client_data:          *mut c_void) -> FlacNamespace::StreamDecoderTellStatus {
        
        todo!();
        /*
            *absolute_byte_offset = (u64) static_cast<const FlacReader*> (client_data)->input->getPosition();
            return FlacNamespace::STREAM_DECODER_TELL_STATUS_OK;
        */
    }
    
    pub fn length_callback(
        _0:            *const FlacNamespace::StreamDecoder,
        stream_length: *mut FlacNamespace::u64,
        client_data:   *mut c_void) -> FlacNamespace::StreamDecoderLengthStatus {
        
        todo!();
        /*
            *stream_length = (u64) static_cast<const FlacReader*> (client_data)->input->getTotalLength();
            return FlacNamespace::STREAM_DECODER_LENGTH_STATUS_OK;
        */
    }
    
    pub fn eof_callback(
        _0:          *const FlacNamespace::StreamDecoder,
        client_data: *mut c_void) -> FlacNamespace::bool {
        
        todo!();
        /*
            return static_cast<const FlacReader*> (client_data)->input->isExhausted();
        */
    }
    
    pub fn write_callback(
        _0:          *const FlacNamespace::StreamDecoder,
        frame:       *const FlacNamespace::Frame,
        buffer:      *const &[FlacNamespace::i32],
        client_data: *mut c_void) -> FlacNamespace::StreamDecoderWriteStatus {
        
        todo!();
        /*
            static_cast<FlacReader*> (client_data)->useSamples (buffer, (int) frame->header.blocksize);
            return FlacNamespace::STREAM_DECODER_WRITE_STATUS_CONTINUE;
        */
    }
    
    pub fn metadata_callback(
        _0:          *const FlacNamespace::StreamDecoder,
        metadata:    *const FlacNamespace::StreamMetadata,
        client_data: *mut c_void)  {
        
        todo!();
        /*
            static_cast<FlacReader*> (client_data)->useMetadata (metadata->data.stream_info);
        */
    }
    
    pub fn error_callback(
        _0: *const FlacNamespace::StreamDecoder,
        _1: FlacNamespace::StreamDecoderErrorStatus,
        _2: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }
}

///---------------------------
#[cfg(ALOE_USE_FLAC)]
#[no_copy]
#[leak_detector]
pub struct FlacWriter {
    base:             AudioFormatWriter,
    ok:               bool, // default = false
    encoder:          FlacNamespace::StreamEncoder,
    stream_start_pos: i64,
}

#[cfg(ALOE_USE_FLAC)]
impl Drop for FlacWriter {
    fn drop(&mut self) {
        todo!();
        /*
            if (ok)
            {
                FlacNamespace::stream_encoder_finish (encoder);
                output->flush();
            }
            else
            {
                output = nullptr; // to stop the base class deleting this, as it needs to be returned
                                  // to the caller of createWriter()
            }

            FlacNamespace::stream_encoder_delete (encoder);
        */
    }
}

#[cfg(ALOE_USE_FLAC)]
impl FlacWriter {

    pub fn new(
        out:                  *mut OutputStream,
        rate:                 f64,
        num_chans:            u32,
        bits:                 u32,
        quality_option_index: i32) -> Self {
    
        todo!();
        /*


            : AudioFormatWriter (out, flacFormatName, rate, numChans, bits),
              streamStartPos (output != nullptr ? jmax (output->getPosition(), 0ll) : 0ll)

            encoder = FlacNamespace::stream_encoder_new();

            if (qualityOptionIndex > 0)
                stream_encoder_set_compression_level (encoder, (u32) jmin (8, qualityOptionIndex));

            stream_encoder_set_do_mid_side_stereo (encoder, numChannels == 2);
            stream_encoder_set_loose_mid_side_stereo (encoder, numChannels == 2);
            stream_encoder_set_channels (encoder, numChannels);
            stream_encoder_set_bits_per_sample (encoder, jmin ((unsigned int) 24, bitsPerSample));
            stream_encoder_set_sample_rate (encoder, (unsigned int) sampleRate);
            stream_encoder_set_blocksize (encoder, 0);
            stream_encoder_set_do_escape_coding (encoder, true);

            ok = stream_encoder_init_stream (encoder,
                                                   encodeWriteCallback, encodeSeekCallback,
                                                   encodeTellCallback, encodeMetadataCallback,
                                                   this) == FlacNamespace::STREAM_ENCODER_INIT_STATUS_OK;
        */
    }
    
    pub fn write(&mut self, 
        samples_to_write: *const *const i32,
        num_samples:      i32) -> bool {
        
        todo!();
        /*
            if (! ok)
                return false;

            HeapBlock<int*> channels;
            HeapBlock<int> temp;
            auto bitsToShift = 32 - (int) bitsPerSample;

            if (bitsToShift > 0)
            {
                temp.malloc (numChannels * (size_t) numSamples);
                channels.calloc (numChannels + 1);

                for (unsigned int i = 0; i < numChannels; ++i)
                {
                    if (samplesToWrite[i] == nullptr)
                        break;

                    auto* destData = temp.get() + i * (size_t) numSamples;
                    channels[i] = destData;

                    for (int j = 0; j < numSamples; ++j)
                        destData[j] = (samplesToWrite[i][j] >> bitsToShift);
                }

                samplesToWrite = const_cast<const int**> (channels.get());
            }

            return stream_encoder_process (encoder, (const FlacNamespace::i32**) samplesToWrite, (unsigned) numSamples) != 0;
        */
    }
    
    pub fn write_data(&self, 
        data: *const c_void,
        size: i32) -> bool {
        
        todo!();
        /*
            return output->write (data, (size_t) size);
        */
    }
    
    pub fn pack_uint32(
        val:   FlacNamespace::u32,
        b:     *mut FlacNamespace::byte,
        bytes: i32)  {
        
        todo!();
        /*
            b += bytes;

            for (int i = 0; i < bytes; ++i)
            {
                *(--b) = (FlacNamespace::byte) (val & 0xff);
                val >>= 8;
            }
        */
    }
    
    pub fn write_meta_data(&mut self, metadata: *const FlacNamespace::StreamMetadata)  {
        
        todo!();
        /*
            using namespace FlacNamespace;
            auto& info = metadata->data.stream_info;

            unsigned char buffer[STREAM_METADATA_STREAMINFO_LENGTH];
            const unsigned int channelsMinus1 = info.channels - 1;
            const unsigned int bitsMinus1 = info.bits_per_sample - 1;

            packUint32 (info.min_blocksize, buffer, 2);
            packUint32 (info.max_blocksize, buffer + 2, 2);
            packUint32 (info.min_framesize, buffer + 4, 3);
            packUint32 (info.max_framesize, buffer + 7, 3);
            buffer[10] = (uint8) ((info.sample_rate >> 12) & 0xff);
            buffer[11] = (uint8) ((info.sample_rate >> 4) & 0xff);
            buffer[12] = (uint8) (((info.sample_rate & 0x0f) << 4) | (channelsMinus1 << 1) | (bitsMinus1 >> 4));
            buffer[13] = (byte) (((bitsMinus1 & 0x0f) << 4) | (unsigned int) ((info.total_samples >> 32) & 0x0f));
            packUint32 ((u32) info.total_samples, buffer + 14, 4);
            memcpy (buffer + 18, info.md5sum, 16);

            const bool seekOk = output->setPosition (streamStartPos + 4);
            ignoreUnused (seekOk);

            // if this fails, you've given it an output stream that can't seek! It needs
            // to be able to seek back to write the header
            jassert (seekOk);

            output->writeIntBigEndian (STREAM_METADATA_STREAMINFO_LENGTH);
            output->write (buffer, STREAM_METADATA_STREAMINFO_LENGTH);
        */
    }
    
    pub fn encode_write_callback(
        _0:            *const FlacNamespace::StreamEncoder,
        buffer:        &[FlacNamespace::byte],
        bytes:         usize,
        samples:       u32,
        current_frame: u32,
        client_data:   *mut c_void) -> FlacNamespace::StreamEncoderWriteStatus {
        
        todo!();
        /*
            return static_cast<FlacWriter*> (client_data)->writeData (buffer, (int) bytes)
                    ? FlacNamespace::STREAM_ENCODER_WRITE_STATUS_OK
                    : FlacNamespace::STREAM_ENCODER_WRITE_STATUS_FATAL_ERROR;
        */
    }
    
    pub fn encode_seek_callback(
        _0: *const FlacNamespace::StreamEncoder,
        _1: FlacNamespace::u64,
        _2: *mut c_void) -> FlacNamespace::StreamEncoderSeekStatus {
        
        todo!();
        /*
            return FlacNamespace::STREAM_ENCODER_SEEK_STATUS_UNSUPPORTED;
        */
    }
    
    pub fn encode_tell_callback(
        _0:                   *const FlacNamespace::StreamEncoder,
        absolute_byte_offset: *mut FlacNamespace::u64,
        client_data:          *mut c_void) -> FlacNamespace::StreamEncoderTellStatus {
        
        todo!();
        /*
            if (client_data == nullptr)
                return FlacNamespace::STREAM_ENCODER_TELL_STATUS_UNSUPPORTED;

            *absolute_byte_offset = (FlacNamespace::u64) static_cast<FlacWriter*> (client_data)->output->getPosition();
            return FlacNamespace::STREAM_ENCODER_TELL_STATUS_OK;
        */
    }
    
    pub fn encode_metadata_callback(
        _0:          *const FlacNamespace::StreamEncoder,
        metadata:    *const FlacNamespace::StreamMetadata,
        client_data: *mut c_void)  {
        
        todo!();
        /*
            static_cast<FlacWriter*> (client_data)->writeMetaData (metadata);
        */
    }
}

#[cfg(ALOE_USE_FLAC)]
impl FlacAudioFormat {

    pub fn new() -> Self {
    
        todo!();
        /*
        : audio_format(flacFormatName, ".flac"),

        
        */
    }
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 8000, 11025, 12000, 16000, 22050, 32000, 44100, 48000,
                 88200, 96000, 176400, 192000, 352800, 384000 };
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 16, 24 };
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
    
    pub fn create_reader_for(&mut self, 
        in_:                            *mut InputStream,
        delete_stream_if_opening_fails: bool) -> *mut AudioFormatReader {
        
        todo!();
        /*
            std::unique_ptr<FlacReader> r (new FlacReader (in));

        if (r->sampleRate > 0)
            return r.release();

        if (! deleteStreamIfOpeningFails)
            r->input = nullptr;

        return nullptr;
        */
    }
    
    pub fn create_writer_for(&mut self, 
        out:                  *mut OutputStream,
        sample_rate:          f64,
        number_of_channels:   u32,
        bits_per_sample:      i32,
        metadata_values:      &StringPairArray,
        quality_option_index: i32) -> *mut AudioFormatWriter {
        
        todo!();
        /*
            if (out != nullptr && getPossibleBitDepths().contains (bitsPerSample))
        {
            std::unique_ptr<FlacWriter> w (new FlacWriter (out, sampleRate, numberOfChannels,
                                                         (u32) bitsPerSample, qualityOptionIndex));
            if (w->ok)
                return w.release();
        }

        return nullptr;
        */
    }
    
    pub fn get_quality_options(&mut self) -> StringArray {
        
        todo!();
        /*
            return { "0 (Fastest)", "1", "2", "3", "4", "5 (Default)","6", "7", "8 (Highest quality)" };
        */
    }
}
