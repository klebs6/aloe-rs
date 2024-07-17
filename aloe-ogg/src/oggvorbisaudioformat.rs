crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_OggVorbisAudioFormat.h]


/**
  | Reads and writes the Ogg-Vorbis audio
  | format.
  | 
  | To compile this, you'll need to set the
  | ALOE_USE_OGGVORBIS flag.
  | 
  | @see AudioFormat,
  | 
  | @tags{Audio}
  |
  */
#[cfg(ALOE_USE_OGGVORBIS)]
#[no_copy]
#[leak_detector]
pub struct OggVorbisAudioFormat {
    base: AudioFormat,
}

#[cfg(ALOE_USE_OGGVORBIS)]
pub mod ogg_vorbis_audio_format {

    use super::*;

    /**
      | Metadata property name used by the Ogg
      | writer - if you set a string for this value,
      | it will be written into the ogg file as
      | the name of the encoder app.
      | 
      | @see createWriterFor
      |
      */
    lazy_static!{
        /*
        static const char* const encoderName;

            static const char* const id3title;          /**< Metadata key for setting an ID3 title. */
            static const char* const id3artist;         /**< Metadata key for setting an ID3 artist name. */
            static const char* const id3album;          /**< Metadata key for setting an ID3 album. */
            static const char* const id3comment;        /**< Metadata key for setting an ID3 comment. */
            static const char* const id3date;           /**< Metadata key for setting an ID3 date. */
            static const char* const id3genre;          /**< Metadata key for setting an ID3 genre. */
            static const char* const id3trackNumber;    /**< Metadata key for setting an ID3 track number. */

            const char* const OggVorbisAudioFormat::encoderName = "encoder";
            const char* const OggVorbisAudioFormat::id3title = "id3title";
            const char* const OggVorbisAudioFormat::id3artist = "id3artist";
            const char* const OggVorbisAudioFormat::id3album = "id3album";
            const char* const OggVorbisAudioFormat::id3comment = "id3comment";
            const char* const OggVorbisAudioFormat::id3date = "id3date";
            const char* const OggVorbisAudioFormat::id3genre = "id3genre";
            const char* const OggVorbisAudioFormat::id3trackNumber = "id3trackNumber";
        */
    }
}

#[cfg(ALOE_USE_OGGVORBIS)]
impl OggVorbisAudioFormat {
    
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
    
    /**
      | Tries to estimate the quality level
      | of an ogg file based on its size.
      | 
      | If it can't read the file for some reason,
      | this will just return 1 (medium quality),
      | otherwise it will return the approximate
      | quality setting that would have been
      | used to create the file.
      | 
      | @see getQualityOptions
      |
      */
    pub fn estimate_ogg_file_quality(&mut self, source: &File) -> i32 {
        
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

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_OggVorbisAudioFormat.cpp]

#[cfg(ALOE_USE_OGGVORBIS)]
#[cfg(all(target_os="macos",not(__MACOSX__)))]
pub const _MACOSX__: usize = 1;

#[cfg(ALOE_USE_OGGVORBIS)]
pub const OGG_FORMAT_NAME: &'static str = "Ogg-Vorbis file";

///----------------
#[cfg(ALOE_USE_OGGVORBIS)]
#[no_copy]
#[leak_detector]
pub struct OggReader {
    base:                 AudioFormatReader,
    ov_file:              OggVorbisNamespace::OggVorbis_File,
    callbacks:            OggVorbisNamespace::ov_callbacks,
    reservoir:            AudioBuffer<f32>,
    reservoir_start:      i64, // default = 0
    samples_in_reservoir: i64, // default = 0
}

#[cfg(ALOE_USE_OGGVORBIS)]
impl Drop for OggReader {
    fn drop(&mut self) {
        todo!();
        /*
            ov_clear (&ovFile);
        */
    }
}

#[cfg(ALOE_USE_OGGVORBIS)]
impl OggReader {

    pub fn new(inp: *mut InputStream) -> Self {
    
        todo!();
        /*
        : audio_format_reader(inp, oggFormatName),

            sampleRate = 0;
            usesFloatingPointData = true;

            callbacks.read_func  = &oggReadCallback;
            callbacks.seek_func  = &oggSeekCallback;
            callbacks.close_func = &oggCloseCallback;
            callbacks.tell_func  = &oggTellCallback;

            auto err = ov_open_callbacks (input, &ovFile, nullptr, 0, callbacks);

            if (err == 0)
            {
                auto* info = ov_info (&ovFile, -1);

                auto* comment = ov_comment (&ovFile, -1);
                addMetadataItem (comment, "ENCODER",     OggVorbisAudioFormat::encoderName);
                addMetadataItem (comment, "TITLE",       OggVorbisAudioFormat::id3title);
                addMetadataItem (comment, "ARTIST",      OggVorbisAudioFormat::id3artist);
                addMetadataItem (comment, "ALBUM",       OggVorbisAudioFormat::id3album);
                addMetadataItem (comment, "COMMENT",     OggVorbisAudioFormat::id3comment);
                addMetadataItem (comment, "DATE",        OggVorbisAudioFormat::id3date);
                addMetadataItem (comment, "GENRE",       OggVorbisAudioFormat::id3genre);
                addMetadataItem (comment, "TRACKNUMBER", OggVorbisAudioFormat::id3trackNumber);

                lengthInSamples = (uint32) ov_pcm_total (&ovFile, -1);
                numChannels = (unsigned int) info->channels;
                bitsPerSample = 16;
                sampleRate = (double) info->rate;

                reservoir.setSize ((int) numChannels, (int) jmin (lengthInSamples, (int64) 4096));
            }
        */
    }
    
    pub fn add_metadata_item(&mut self, 
        comment:       *mut OggVorbisNamespace::vorbis_comment,
        name:          *const u8,
        metadata_name: *const u8)  {
        
        todo!();
        /*
            if (auto* value = vorbis_comment_query (comment, name, 0))
                metadataValues.set (metadataName, value);
        */
    }
    
    pub fn read_samples(&mut self, 
        dest_samples:                *mut *mut i32,
        num_dest_channels:           i32,
        start_offset_in_dest_buffer: i32,
        start_sample_in_file:        i64,
        num_samples:                 i32) -> bool {
        
        todo!();
        /*
            while (numSamples > 0)
            {
                auto numAvailable = (reservoirStart + samplesInReservoir - startSampleInFile);

                if (startSampleInFile >= reservoirStart && numAvailable > 0)
                {
                    // got a few samples overlapping, so use them before seeking..

                    auto numToUse = jmin ((int64) numSamples, numAvailable);

                    for (int i = jmin (numDestChannels, reservoir.getNumChannels()); --i >= 0;)
                        if (destSamples[i] != nullptr)
                            memcpy (destSamples[i] + startOffsetInDestBuffer,
                                    reservoir.getReadPointer (i, (int) (startSampleInFile - reservoirStart)),
                                    (size_t) numToUse * sizeof (float));

                    startSampleInFile += numToUse;
                    numSamples -= (int) numToUse;
                    startOffsetInDestBuffer += (int) numToUse;

                    if (numSamples == 0)
                        break;
                }

                if (startSampleInFile < reservoirStart
                    || startSampleInFile + numSamples > reservoirStart + samplesInReservoir)
                {
                    // buffer miss, so refill the reservoir
                    reservoirStart = jmax (0, (int) startSampleInFile);
                    samplesInReservoir = reservoir.getNumSamples();

                    if (reservoirStart != (int) ov_pcm_tell (&ovFile))
                        ov_pcm_seek (&ovFile, reservoirStart);

                    int bitStream = 0;
                    int offset = 0;
                    int numToRead = (int) samplesInReservoir;

                    while (numToRead > 0)
                    {
                        float** dataIn = nullptr;
                        auto samps = static_cast<int> (ov_read_float (&ovFile, &dataIn, numToRead, &bitStream));

                        if (samps <= 0)
                            break;

                        jassert (samps <= numToRead);

                        for (int i = jmin ((int) numChannels, reservoir.getNumChannels()); --i >= 0;)
                            memcpy (reservoir.getWritePointer (i, offset), dataIn[i], (size_t) samps * sizeof (float));

                        numToRead -= samps;
                        offset += samps;
                    }

                    if (numToRead > 0)
                        reservoir.clear (offset, numToRead);
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
    
    pub fn ogg_read_callback(
        ptr:        *mut c_void,
        size:       usize,
        nmemb:      usize,
        datasource: *mut c_void) -> usize {
        
        todo!();
        /*
            return (size_t) (static_cast<InputStream*> (datasource)->read (ptr, (int) (size * nmemb))) / size;
        */
    }
    
    pub fn ogg_seek_callback(
        datasource: *mut c_void,
        offset:     OggVorbisNamespace::ogg_int64_t,
        whence:     i32) -> i32 {
        
        todo!();
        /*
            auto* in = static_cast<InputStream*> (datasource);

            if (whence == SEEK_CUR)
                offset += in->getPosition();
            else if (whence == SEEK_END)
                offset += in->getTotalLength();

            in->setPosition (offset);
            return 0;
        */
    }
    
    pub fn ogg_close_callback(_0: *mut c_void) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn ogg_tell_callback(datasource: *mut c_void) -> i64 {
        
        todo!();
        /*
            return (long) static_cast<InputStream*> (datasource)->getPosition();
        */
    }
}

///----------------
#[cfg(ALOE_USE_OGGVORBIS)]
#[no_copy]
#[leak_detector]
pub struct OggWriter {
    base: AudioFormatWriter,
}

#[cfg(ALOE_USE_OGGVORBIS)]
pub mod ogg_writer {

    use super::*;

    lazy_static!{
        /*
        bool ok = false;

            OggVorbisNamespace::ogg_stream_state os;
            OggVorbisNamespace::ogg_page og;
            OggVorbisNamespace::ogg_packet op;
            OggVorbisNamespace::vorbis_info vi;
            OggVorbisNamespace::vorbis_comment vc;
            OggVorbisNamespace::vorbis_dsp_state vd;
            OggVorbisNamespace::vorbis_block vb;
        */
    }
}

#[cfg(ALOE_USE_OGGVORBIS)]
impl Drop for OggWriter {
    fn drop(&mut self) {
        todo!();
        /*
            if (ok)
            {
                // write a zero-length packet to show ogg that we're finished..
                writeSamples (0);

                ogg_stream_clear (&os);
                vorbis_block_clear (&vb);
                vorbis_dsp_clear (&vd);
                vorbis_comment_clear (&vc);

                vorbis_info_clear (&vi);
                output->flush();
            }
            else
            {
                vorbis_info_clear (&vi);
                output = nullptr; // to stop the base class deleting this, as it needs to be returned
                                  // to the caller of createWriter()
            }
        */
    }
}

#[cfg(ALOE_USE_OGGVORBIS)]
impl OggWriter {
    
    pub fn new(
        out:           *mut OutputStream,
        rate:          f64,
        num_chans:     u32,
        bits_per_samp: u32,
        quality_index: i32,
        metadata:      &StringPairArray) -> Self {
    
        todo!();
        /*
        : audio_format_writer(out, oggFormatName, rate, numChans, bitsPerSamp),

            vorbis_info_init (&vi);

            if (vorbis_encode_init_vbr (&vi, (int) numChans, (int) rate,
                                        jlimit (0.0f, 1.0f, (float) qualityIndex * 0.1f)) == 0)
            {
                vorbis_comment_init (&vc);

                addMetadata (metadata, OggVorbisAudioFormat::encoderName,    "ENCODER");
                addMetadata (metadata, OggVorbisAudioFormat::id3title,       "TITLE");
                addMetadata (metadata, OggVorbisAudioFormat::id3artist,      "ARTIST");
                addMetadata (metadata, OggVorbisAudioFormat::id3album,       "ALBUM");
                addMetadata (metadata, OggVorbisAudioFormat::id3comment,     "COMMENT");
                addMetadata (metadata, OggVorbisAudioFormat::id3date,        "DATE");
                addMetadata (metadata, OggVorbisAudioFormat::id3genre,       "GENRE");
                addMetadata (metadata, OggVorbisAudioFormat::id3trackNumber, "TRACKNUMBER");

                vorbis_analysis_init (&vd, &vi);
                vorbis_block_init (&vd, &vb);

                ogg_stream_init (&os, Random::getSystemRandom().nextInt());

                OggVorbisNamespace::ogg_packet header, header_comm, header_code;
                vorbis_analysis_headerout (&vd, &vc, &header, &header_comm, &header_code);

                ogg_stream_packetin (&os, &header);
                ogg_stream_packetin (&os, &header_comm);
                ogg_stream_packetin (&os, &header_code);

                for (;;)
                {
                    if (ogg_stream_flush (&os, &og) == 0)
                        break;

                    output->write (og.header, (size_t) og.header_len);
                    output->write (og.body,   (size_t) og.body_len);
                }

                ok = true;
            }
        */
    }
    
    pub fn write(&mut self, 
        samples_to_write: *const *const i32,
        num_samples:      i32) -> bool {
        
        todo!();
        /*
            if (ok)
            {
                if (numSamples > 0)
                {
                    const double gain = 1.0 / 0x80000000u;
                    float** const vorbisBuffer = vorbis_analysis_buffer (&vd, numSamples);

                    for (int i = (int) numChannels; --i >= 0;)
                    {
                        if (auto* dst = vorbisBuffer[i])
                        {
                            if (const int* src = samplesToWrite [i])
                            {
                                for (int j = 0; j < numSamples; ++j)
                                    dst[j] = (float) (src[j] * gain);
                            }
                        }
                    }
                }

                writeSamples (numSamples);
            }

            return ok;
        */
    }
    
    pub fn write_samples(&mut self, num_samples: i32)  {
        
        todo!();
        /*
            vorbis_analysis_wrote (&vd, numSamples);

            while (vorbis_analysis_blockout (&vd, &vb) == 1)
            {
                vorbis_analysis (&vb, nullptr);
                vorbis_bitrate_addblock (&vb);

                while (vorbis_bitrate_flushpacket (&vd, &op))
                {
                    ogg_stream_packetin (&os, &op);

                    for (;;)
                    {
                        if (ogg_stream_pageout (&os, &og) == 0)
                            break;

                        output->write (og.header, (size_t) og.header_len);
                        output->write (og.body,   (size_t) og.body_len);

                        if (ogg_page_eos (&og))
                            break;
                    }
                }
            }
        */
    }
    
    pub fn add_metadata(&mut self, 
        metadata:    &StringPairArray,
        name:        *const u8,
        vorbis_name: *const u8)  {
        
        todo!();
        /*
            auto s = metadata [name];

            if (s.isNotEmpty())
                vorbis_comment_add_tag (&vc, vorbisName, const_cast<char*> (s.toRawUTF8()));
        */
    }
}

///------------------
#[cfg(ALOE_USE_OGGVORBIS)]
impl OggVorbisAudioFormat {

    pub fn new() -> Self {
    
        todo!();
        /*
        : audio_format(oggFormatName, ".ogg"),

        
        */
    }
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 8000, 11025, 12000, 16000, 22050, 32000,
                 44100, 48000, 88200, 96000, 176400, 192000 };
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 32 };
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
            std::unique_ptr<OggReader> r (new OggReader (in));

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
        num_channels:         u32,
        bits_per_sample:      i32,
        metadata_values:      &StringPairArray,
        quality_option_index: i32) -> *mut AudioFormatWriter {
        
        todo!();
        /*
            if (out == nullptr)
            return nullptr;

        std::unique_ptr<OggWriter> w (new OggWriter (out, sampleRate, numChannels,
                                                     (unsigned int) bitsPerSample,
                                                     qualityOptionIndex, metadataValues));

        return w->ok ? w.release() : nullptr;
        */
    }
    
    pub fn get_quality_options(&mut self) -> StringArray {
        
        todo!();
        /*
            return { "64 kbps", "80 kbps", "96 kbps", "112 kbps", "128 kbps", "160 kbps",
                 "192 kbps", "224 kbps", "256 kbps", "320 kbps", "500 kbps" };
        */
    }
    
    pub fn estimate_ogg_file_quality(&mut self, source: &File) -> i32 {
        
        todo!();
        /*
            if (auto in = source.createInputStream())
        {
            if (auto r = std::unique_ptr<AudioFormatReader> (createReaderFor (in.release(), true)))
            {
                auto lengthSecs = (double) r->lengthInSamples / r->sampleRate;
                auto approxBitsPerSecond = (int) ((double) source.getSize() * 8 / lengthSecs);

                auto qualities = getQualityOptions();
                int bestIndex = 0;
                int bestDiff = 10000;

                for (int i = qualities.size(); --i >= 0;)
                {
                    auto diff = std::abs (qualities[i].getIntValue() - approxBitsPerSecond);

                    if (diff < bestDiff)
                    {
                        bestDiff = diff;
                        bestIndex = i;
                    }
                }

                return bestIndex;
            }
        }

        return 0;
        */
    }
}
