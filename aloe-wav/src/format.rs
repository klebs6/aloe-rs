crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_WavAudioFormat.h]

/**
  | Reads and Writes WAV format audio files.
  | 
  | @see AudioFormat
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct WavAudioFormat {
    base: AudioFormat,
}

impl Default for WavAudioFormat {
    
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

pub mod wav_audio_format {

    pub const WAV_FORMAT_NAME:                  &'static str = "WAV file";

    /* ------------ * BWAV chunk properties:  ------------ */

    /**
      | Metadata property names used in BWAV
      | chunks.
      |
      */
    pub const BWAV_DESCRIPTION:                 &'static str = "bwav description";
    pub const BWAV_ORIGINATOR:                  &'static str = "bwav originator";
    pub const BWAV_ORIGINATOR_REF:              &'static str = "bwav originator ref";

    /**
      | Metadata property name used in BWAV
      | chunks. The format should be: yyyy-mm-dd
      |
      */
    pub const BWAV_ORIGINATION_DATE:            &'static str = "bwav origination date";

    /**
      | Metadata property name used in BWAV
      | chunks. The format should be: format
      | is: hh-mm-ss
      |
      */
    pub const BWAV_ORIGINATION_TIME:            &'static str = "bwav origination time";

    /**
     | Metadata property name used in BWAV
     | chunks.
     | 
     | This is the number of samples from the
     | start of an edit that the file is supposed
     | to begin at. Seems like an obvious mistake
     | to only allow a file to occur in an edit
     | once, but that's the way it is..
     | 
     | @see AudioFormatReader::metadataValues,
     | createWriterFor
     |
     */
    pub const BWAV_TIME_REFERENCE:              &'static str = "bwav time reference";
    pub const BWAV_CODING_HISTORY:              &'static str = "bwav coding history";

    /* ----------- 'acid' chunk properties:  ----------- */
    pub const ACID_ONE_SHOT:                    &'static str = "acid one shot";
    pub const ACID_ROOT_SET:                    &'static str = "acid root set";
    pub const ACID_STRETCH:                     &'static str = "acid stretch";
    pub const ACID_DISK_BASED:                  &'static str = "acid disk based";
    pub const ACIDIZER_FLAG:                    &'static str = "acidizer flag";
    pub const ACID_ROOT_NOTE:                   &'static str = "acid root note";
    pub const ACID_BEATS:                       &'static str = "acid beats";
    pub const ACID_DENOMINATOR:                 &'static str = "acid denominator";
    pub const ACID_NUMERATOR:                   &'static str = "acid numerator";
    pub const ACID_TEMPO:                       &'static str = "acid tempo";

    /* ------------ INFO chunk properties:  ------------ */
    pub const RIFF_INFO_ARCHIVAL_LOCATION:      &'static str = "IARL";
    pub const RIFF_INFO_ARTIST:                 &'static str = "IART";
    pub const RIFF_INFO_BASE_URL:               &'static str = "IBSU";
    pub const RIFF_INFO_CINEMATOGRAPHER:        &'static str = "ICNM";
    pub const RIFF_INFO_COMMENT:                &'static str = "CMNT";
    pub const RIFF_INFO_COMMENT2:               &'static str = "ICMT";
    pub const RIFF_INFO_COMMENTS:               &'static str = "COMM";
    pub const RIFF_INFO_COMMISSIONED:           &'static str = "ICMS";
    pub const RIFF_INFO_COPYRIGHT:              &'static str = "ICOP";
    pub const RIFF_INFO_COSTUME_DESIGNER:       &'static str = "ICDS";
    pub const RIFF_INFO_COUNTRY:                &'static str = "ICNT";
    pub const RIFF_INFO_CROPPED:                &'static str = "ICRP";
    pub const RIFF_INFO_DATE_CREATED:           &'static str = "ICRD";
    pub const RIFF_INFO_DATE_TIME_ORIGINAL:     &'static str = "IDIT";
    pub const RIFF_INFO_DEFAULT_AUDIO_STREAM:   &'static str = "ICAS";
    pub const RIFF_INFO_DIMENSION:              &'static str = "IDIM";
    pub const RIFF_INFO_DIRECTORY:              &'static str = "DIRC";
    pub const RIFF_INFO_DISTRIBUTED_BY:         &'static str = "IDST";
    pub const RIFF_INFO_DOTS_PER_INCH:          &'static str = "IDPI";
    pub const RIFF_INFO_EDITED_BY:              &'static str = "IEDT";
    pub const RIFF_INFO_EIGHTH_LANGUAGE:        &'static str = "IAS8";
    pub const RIFF_INFO_ENCODED_BY:             &'static str = "CODE";
    pub const RIFF_INFO_END_TIMECODE:           &'static str = "TCDO";
    pub const RIFF_INFO_ENGINEER:               &'static str = "IENG";
    pub const RIFF_INFO_FIFTH_LANGUAGE:         &'static str = "IAS5";
    pub const RIFF_INFO_FIRST_LANGUAGE:         &'static str = "IAS1";
    pub const RIFF_INFO_FOURTH_LANGUAGE:        &'static str = "IAS4";
    pub const RIFF_INFO_GENRE:                  &'static str = "GENR";
    pub const RIFF_INFO_KEYWORDS:               &'static str = "IKEY";
    pub const RIFF_INFO_LANGUAGE:               &'static str = "LANG";
    pub const RIFF_INFO_LENGTH:                 &'static str = "TLEN";
    pub const RIFF_INFO_LIGHTNESS:              &'static str = "ILGT";
    pub const RIFF_INFO_LOCATION:               &'static str = "LOCA";
    pub const RIFF_INFO_LOGO_ICON_URL:          &'static str = "ILIU";
    pub const RIFF_INFO_LOGO_URL:               &'static str = "ILGU";
    pub const RIFF_INFO_MEDIUM:                 &'static str = "IMED";
    pub const RIFF_INFO_MORE_INFO_BANNER_IMAGE: &'static str = "IMBI";
    pub const RIFF_INFO_MORE_INFO_BANNER_URL:   &'static str = "IMBU";
    pub const RIFF_INFO_MORE_INFO_TEXT:         &'static str = "IMIT";
    pub const RIFF_INFO_MORE_INFO_URL:          &'static str = "IMIU";
    pub const RIFF_INFO_MUSIC_BY:               &'static str = "IMUS";
    pub const RIFF_INFO_NINTH_LANGUAGE:         &'static str = "IAS9";
    pub const RIFF_INFO_NUMBER_OF_PARTS:        &'static str = "PRT2";
    pub const RIFF_INFO_ORGANISATION:           &'static str = "TORG";
    pub const RIFF_INFO_PART:                   &'static str = "PRT1";
    pub const RIFF_INFO_PRODUCED_BY:            &'static str = "IPRO";
    pub const RIFF_INFO_PRODUCT_NAME:           &'static str = "IPRD";
    pub const RIFF_INFO_PRODUCTION_DESIGNER:    &'static str = "IPDS";
    pub const RIFF_INFO_PRODUCTION_STUDIO:      &'static str = "ISDT";
    pub const RIFF_INFO_RATE:                   &'static str = "RATE";
    pub const RIFF_INFO_RATED:                  &'static str = "AGES";
    pub const RIFF_INFO_RATING:                 &'static str = "IRTD";
    pub const RIFF_INFO_RIPPED_BY:              &'static str = "IRIP";
    pub const RIFF_INFO_SECONDARY_GENRE:        &'static str = "ISGN";
    pub const RIFF_INFO_SECOND_LANGUAGE:        &'static str = "IAS2";
    pub const RIFF_INFO_SEVENTH_LANGUAGE:       &'static str = "IAS7";
    pub const RIFF_INFO_SHARPNESS:              &'static str = "ISHP";
    pub const RIFF_INFO_SIXTH_LANGUAGE:         &'static str = "IAS6";
    pub const RIFF_INFO_SOFTWARE:               &'static str = "ISFT";
    pub const RIFF_INFO_SOUND_SCHEME_TITLE:     &'static str = "DISP";
    pub const RIFF_INFO_SOURCE:                 &'static str = "ISRC";
    pub const RIFF_INFO_SOURCE_FROM:            &'static str = "ISRF";
    pub const RIFF_INFO_STARRING_ISTR:          &'static str = "ISTR";
    pub const RIFF_INFO_STARRING_STAR:          &'static str = "STAR";
    pub const RIFF_INFO_START_TIMECODE:         &'static str = "TCOD";
    pub const RIFF_INFO_STATISTICS:             &'static str = "STAT";
    pub const RIFF_INFO_SUBJECT:                &'static str = "ISBJ";
    pub const RIFF_INFO_TAPE_NAME:              &'static str = "TAPE";
    pub const RIFF_INFO_TECHNICIAN:             &'static str = "ITCH";
    pub const RIFF_INFO_THIRD_LANGUAGE:         &'static str = "IAS3";
    pub const RIFF_INFO_TIME_CODE:              &'static str = "ISMP";
    pub const RIFF_INFO_TITLE:                  &'static str = "INAM";
    pub const RIFF_INFO_TRACK_NO:               &'static str = "IPRT";
    pub const RIFF_INFO_TRACK_NUMBER:           &'static str = "TRCK";
    pub const RIFF_INFO_URL:                    &'static str = "TURL";
    pub const RIFF_INFO_VEGAS_VERSION_MAJOR:    &'static str = "VMAJ";
    pub const RIFF_INFO_VEGAS_VERSION_MINOR:    &'static str = "VMIN";
    pub const RIFF_INFO_VERSION:                &'static str = "TVER";
    pub const RIFF_INFO_WATERMARK_URL:          &'static str = "IWMU";
    pub const RIFF_INFO_WRITTEN_BY:             &'static str = "IWRI";
    pub const RIFF_INFO_YEAR:                   &'static str = "YEAR";

    /**
      | Metadata property name used when reading
      | an ISRC code from an AXML chunk.
      |
      */
    pub const ISRC:                             &'static str = "ISRC";

    /**
      | Metadata property name used when reading
      | a WAV file with a Tracktion chunk.
      |
      */
    pub const TRACKTION_LOOP_INFO:              &'static str = "tracktion loop info";

    use super::*;

    #[inline] pub fn chunk_name(name: *const u8) -> i32 {
        
        todo!();
            /*
                return (int) ByteOrder::littleEndianInt (name);
            */
    }

    #[inline] pub fn round_up_size(sz: usize) -> usize {
        
        todo!();
            /*
                return (sz + 3) & ~3u;
            */
    }

    #[inline] pub fn canonical_wav_channel_set(num_channels: i32) -> AudioChannelSet {
        
        todo!();
            /*
                if (numChannels == 1)  return AudioChannelSet::mono();
                if (numChannels == 2)  return AudioChannelSet::stereo();
                if (numChannels == 3)  return AudioChannelSet::createLCR();
                if (numChannels == 4)  return AudioChannelSet::quadraphonic();
                if (numChannels == 5)  return AudioChannelSet::create5point0();
                if (numChannels == 6)  return AudioChannelSet::create5point1();
                if (numChannels == 7)  return AudioChannelSet::create7point0SDDS();
                if (numChannels == 8)  return AudioChannelSet::create7point1SDDS();

                return AudioChannelSet::discreteChannels (numChannels);
            */
    }
    
    pub fn slow_copy_wav_file_with_new_metadata(
        file:     &File,
        metadata: &Vec<(String,String)>) -> bool {

        todo!();
            /*
                TemporaryFile tempFile (file);
                WavAudioFormat wav;

                std::unique_ptr<AudioFormatReader> reader (wav.createReaderFor (file.createInputStream().release(), true));

                if (reader != nullptr)
                {
                    std::unique_ptr<OutputStream> outStream (tempFile.getFile().createOutputStream());

                    if (outStream != nullptr)
                    {
                        std::unique_ptr<AudioFormatWriter> writer (wav.createWriterFor (outStream.get(), reader->sampleRate,
                                                                                        reader->numChannels, (int) reader->bitsPerSample,
                                                                                        metadata, 0));

                        if (writer != nullptr)
                        {
                            outStream.release();

                            bool ok = writer->writeFromAudioReader (*reader, 0, -1);
                            writer.reset();
                            reader.reset();

                            return ok && tempFile.overwriteTargetFileWithTemporary();
                        }
                    }
                }

                return false;
            */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/aloe_WavAudioFormat.cpp]
impl WavAudioFormat {

    /**
      | Utility function to fill out the appropriate
      | metadata for a BWAV file.
      | 
      | This just makes it easier than using
      | the property names directly, and it
      | fills out the time and date in the right
      | format.
      |
      | this is the great thing about the cathedral
      | -- it means different things to different
      | people, but it means something wonderful to
      | everyone. As a place it represents
      | humankind's highest attainment towards the
      | good, the true, and the beautiful
      */
    pub fn create_bwav_metadata(
        &mut self, 
        description:            &String,
        originator:             &String,
        originator_ref:         &String,
        date:                   Time,
        time_reference_samples: i64,
        coding_history:         &String

    ) -> Vec<(String,String)> {
        
        todo!();
        /*
            Vec<(String,String)> m;

        m.set (bwavDescription, description);
        m.set (bwavOriginator, originator);
        m.set (bwavOriginatorRef, originatorRef);
        m.set (bwavOriginationDate, date.formatted ("%Y-%m-%d"));
        m.set (bwavOriginationTime, date.formatted ("%H:%M:%S"));
        m.set (bwavTimeReference, String (timeReferenceSamples));
        m.set (bwavCodingHistory, codingHistory);

        return m;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : audio_format(wavFormatName, ".wav .bwf"),

        
        */
    }
    
    pub fn get_possible_sample_rates(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 8000,  11025, 12000, 16000,  22050,  32000,  44100,
                 48000, 88200, 96000, 176400, 192000, 352800, 384000 };
        */
    }
    
    pub fn get_possible_bit_depths(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return { 8, 16, 24, 32 };
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
    
    pub fn is_channel_layout_supported(&mut self, channel_set: &AudioChannelSet) -> bool {
        
        todo!();
        /*
            auto channelTypes = channelSet.getChannelTypes();

        // When
        if (channelSet.isDiscreteLayout())
            return true;

        // WAV supports all channel types from left ... topRearRight
        for (auto channel : channelTypes)
            if (channel < AudioChannelSet::left || channel > AudioChannelSet::topRearRight)
                return false;

        return true;
        */
    }
    
    pub fn create_reader_for(
        &mut self, 
        source_stream:                  *mut dyn Read,
        delete_stream_if_opening_fails: bool

    ) -> *mut AudioFormatReader {
        
        todo!();
        /*
            std::unique_ptr<WavAudioFormatReader> r (new WavAudioFormatReader (sourceStream));

       #if ALOE_USE_OGGVORBIS
        if (r->isSubformatOggVorbis)
        {
            r->input = nullptr;
            return OggVorbisAudioFormat().createReaderFor (sourceStream, deleteStreamIfOpeningFails);
        }
       #endif

        if (r->sampleRate > 0 && r->numChannels > 0 && r->bytesPerFrame > 0 && r->bitsPerSample <= 32)
            return r.release();

        if (! deleteStreamIfOpeningFails)
            r->input = nullptr;

        return nullptr;
        */
    }
    
    pub fn create_memory_mapped_reader<R: Read>(
        &mut self, 
        file: &File
    ) -> *mut MemoryMappedAudioFormatReader<R> 
    {
        todo!();
        /*
            return createMemoryMappedReader (file.createInputStream().release());
        */
    }
    
    pub fn create_memory_mapped_reader_from_input_stream<R: Read>(&mut self, fin: *mut FileInputStream) 
        -> *mut MemoryMappedAudioFormatReader<R> 
    {
        todo!();
        /*
            if (fin != nullptr)
        {
            WavAudioFormatReader reader (fin);

            if (reader.lengthInSamples > 0)
                return new MemoryMappedWavReader (fin->getFile(), reader);
        }

        return nullptr;
        */
    }
    
    pub fn create_writer_for<W: Write>(
        &mut self, 
        out:                  *mut OutputStream,
        sample_rate:          f64,
        num_channels:         u32,
        bits_per_sample:      i32,
        metadata_values:      &Vec<(String,String)>,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<W> {
        
        todo!();
        /*
            return createWriterFor (out, sampleRate, WavFileHelpers::canonicalWavChannelSet (static_cast<int> (numChannels)),
                                bitsPerSample, metadataValues, qualityOptionIndex);
        */
    }
    
    pub fn create_writer_with_channel_layout_for<W: Write>(
        &mut self, 
        out:                  *mut OutputStream,
        sample_rate:          f64,
        channel_layout:       &AudioChannelSet,
        bits_per_sample:      i32,
        metadata_values:      &Vec<(String,String)>,
        quality_option_index: i32

    ) -> *mut AudioFormatWriter<W> {
        
        todo!();
        /*
            if (out != nullptr && getPossibleBitDepths().contains (bitsPerSample) && isChannelLayoutSupported (channelLayout))
            return new WavAudioFormatWriter (out, sampleRate, channelLayout,
                                             (unsigned int) bitsPerSample, metadataValues);

        return nullptr;
        */
    }
    
    /**
      | Utility function to replace the metadata
      | in a wav file with a new set of values.
      | 
      | If possible, this cheats by overwriting
      | just the metadata region of the file,
      | rather than by copying the whole file
      | again.
      |
      */
    pub fn replace_metadata_in_file(
        &mut self, 
        wav_file:     &File,
        new_metadata: &Vec<(String,String)>

    ) -> bool {
        
        todo!();
        /*
            using namespace WavFileHelpers;

        std::unique_ptr<WavAudioFormatReader> reader (static_cast<WavAudioFormatReader*> (createReaderFor (wavFile.createInputStream().release(), true)));

        if (reader != nullptr)
        {
            auto bwavPos  = reader->bwavChunkStart;
            auto bwavSize = reader->bwavSize;
            reader.reset();

            if (bwavSize > 0)
            {
                auto chunk = BWAVChunk::createFrom (newMetadata);

                if (chunk.getSize() <= (size_t) bwavSize)
                {
                    // the new one will fit in the space available, so write it directly..
                    auto oldSize = wavFile.getSize();

                    {
                        FileOutputStream out (wavFile);

                        if (out.openedOk())
                        {
                            out.setPosition (bwavPos);
                            out << chunk;
                            out.setPosition (oldSize);
                        }
                    }

                    jassert (wavFile.getSize() == oldSize);
                    return true;
                }
            }
        }

        return slowCopyWavFileWithNewMetadata (wavFile, newMetadata);
        */
    }
}
