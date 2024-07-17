crate::ix!();

///------------------------------
#[no_copy]
#[leak_detector]
pub struct WavAudioFormatReader<'a> {
    base:                    AudioFormatReader<'a>,
    bwav_chunk_start:        i64, // default = 0
    bwav_size:               i64, // default = 0
    data_chunk_start:        i64, // default = 0
    data_length:             i64, // default = 0
    bytes_per_frame:         i32, // default = 0
    isrf64:                  bool, // default = false
    is_subformat_ogg_vorbis: bool, // default = false
    channel_layout:          AudioChannelSet,
}

impl<'a> WavAudioFormatReader<'a> {

    pub fn new(in_: *mut dyn Read) -> Self {
    
        todo!();
        /*
        : audio_format_reader(in, wavFormatName),

            using namespace WavFileHelpers;
            uint64 len = 0, end = 0;
            int cueNoteIndex = 0;
            int cueLabelIndex = 0;
            int cueRegionIndex = 0;

            auto streamStartPos = input->getPosition();
            auto firstChunkType = input->readInt();

            if (firstChunkType == chunkName ("RF64"))
            {
                input->skipNextBytes (4); // size is -1 for RF64
                isRF64 = true;
            }
            else if (firstChunkType == chunkName ("RIFF"))
            {
                len = (uint64) (uint32) input->readInt();
                end = len + (uint64) input->getPosition();
            }
            else
            {
                return;
            }

            auto startOfRIFFChunk = input->getPosition();

            if (input->readInt() == chunkName ("WAVE"))
            {
                if (isRF64 && input->readInt() == chunkName ("ds64"))
                {
                    auto length = (uint32) input->readInt();

                    if (length < 28)
                        return;

                    auto chunkEnd = input->getPosition() + length + (length & 1);
                    len = (uint64) input->readInt64();
                    end = len + (uint64) startOfRIFFChunk;
                    dataLength = input->readInt64();
                    input->setPosition (chunkEnd);
                }

                while ((uint64) input->getPosition() < end && ! input->isExhausted())
                {
                    auto chunkType = input->readInt();
                    auto length = (uint32) input->readInt();
                    auto chunkEnd = input->getPosition() + length + (length & 1);

                    if (chunkType == chunkName ("fmt "))
                    {
                        // read the format chunk
                        auto format = (unsigned short) input->readShort();
                        numChannels = (unsigned int) input->readShort();
                        sampleRate = input->readInt();
                        auto bytesPerSec = input->readInt();
                        input->skipNextBytes (2);
                        bitsPerSample = (unsigned int) (int) input->readShort();

                        if (bitsPerSample > 64 && (int) sampleRate != 0)
                        {
                            bytesPerFrame = bytesPerSec / (int) sampleRate;

                            if (numChannels != 0)
                                bitsPerSample = 8 * (unsigned int) bytesPerFrame / numChannels;
                        }
                        else
                        {
                            bytesPerFrame = (int) (numChannels * bitsPerSample / 8);
                        }

                        if (format == 3)
                        {
                            usesFloatingPointData = true;
                        }
                        else if (format == 0xfffe) // WAVE_FORMAT_EXTENSIBLE
                        {
                            if (length < 40) // too short
                            {
                                bytesPerFrame = 0;
                            }
                            else
                            {
                                input->skipNextBytes (4); // skip over size and bitsPerSample
                                auto channelMask = input->readInt();
                                metadataValues.set ("ChannelMask", String (channelMask));
                                channelLayout = getChannelLayoutFromMask (channelMask, numChannels);

                                ExtensibleWavSubFormat subFormat;
                                subFormat.data1 = (uint32) input->readInt();
                                subFormat.data2 = (uint16) input->readShort();
                                subFormat.data3 = (uint16) input->readShort();
                                input->read (subFormat.data4, sizeof (subFormat.data4));

                                if (subFormat == IEEEFloatFormat)
                                    usesFloatingPointData = true;
                                else if (subFormat != pcmFormat && subFormat != ambisonicFormat)
                                    bytesPerFrame = 0;
                            }
                        }
                        else if (format == 0x674f  // WAVE_FORMAT_OGG_VORBIS_MODE_1
                              || format == 0x6750  // WAVE_FORMAT_OGG_VORBIS_MODE_2
                              || format == 0x6751  // WAVE_FORMAT_OGG_VORBIS_MODE_3
                              || format == 0x676f  // WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS
                              || format == 0x6770  // WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS
                              || format == 0x6771) // WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS
                        {
                            isSubformatOggVorbis = true;
                            sampleRate = 0; // to mark the wav reader as failed
                            input->setPosition (streamStartPos);
                            return;
                        }
                        else if (format != 1)
                        {
                            bytesPerFrame = 0;
                        }
                    }
                    else if (chunkType == chunkName ("data"))
                    {
                        if (isRF64)
                        {
                            if (dataLength > 0)
                                chunkEnd = input->getPosition() + dataLength + (dataLength & 1);
                        }
                        else
                        {
                            dataLength = length;
                        }

                        dataChunkStart = input->getPosition();
                        lengthInSamples = (bytesPerFrame > 0) ? (dataLength / bytesPerFrame) : 0;
                    }
                    else if (chunkType == chunkName ("bext"))
                    {
                        bwavChunkStart = input->getPosition();
                        bwavSize = length;

                        HeapBlock<BWAVChunk> bwav;
                        bwav.calloc (jmax ((size_t) length + 1, sizeof (BWAVChunk)), 1);
                        input->read (bwav, (int) length);
                        bwav->copyTo (metadataValues, (int) length);
                    }
                    else if (chunkType == chunkName ("smpl"))
                    {
                        HeapBlock<SMPLChunk> smpl;
                        smpl.calloc (jmax ((size_t) length + 1, sizeof (SMPLChunk)), 1);
                        input->read (smpl, (int) length);
                        smpl->copyTo (metadataValues, (int) length);
                    }
                    else if (chunkType == chunkName ("inst") || chunkType == chunkName ("INST")) // need to check which...
                    {
                        HeapBlock<InstChunk> inst;
                        inst.calloc (jmax ((size_t) length + 1, sizeof (InstChunk)), 1);
                        input->read (inst, (int) length);
                        inst->copyTo (metadataValues);
                    }
                    else if (chunkType == chunkName ("cue "))
                    {
                        HeapBlock<CueChunk> cue;
                        cue.calloc (jmax ((size_t) length + 1, sizeof (CueChunk)), 1);
                        input->read (cue, (int) length);
                        cue->copyTo (metadataValues, (int) length);
                    }
                    else if (chunkType == chunkName ("axml"))
                    {
                        MemoryBlock axml;
                        input->readIntoMemoryBlock (axml, (ssize_t) length);
                        AXMLChunk::addToMetadata (metadataValues, axml.toString());
                    }
                    else if (chunkType == chunkName ("LIST"))
                    {
                        auto subChunkType = input->readInt();

                        if (subChunkType == chunkName ("info") || subChunkType == chunkName ("INFO"))
                        {
                            ListInfoChunk::addToMetadata (metadataValues, *input, chunkEnd);
                        }
                        else if (subChunkType == chunkName ("adtl"))
                        {
                            while (input->getPosition() < chunkEnd)
                            {
                                auto adtlChunkType = input->readInt();
                                auto adtlLength = (uint32) input->readInt();
                                auto adtlChunkEnd = input->getPosition() + (adtlLength + (adtlLength & 1));

                                if (adtlChunkType == chunkName ("labl") || adtlChunkType == chunkName ("note"))
                                {
                                    String prefix;

                                    if (adtlChunkType == chunkName ("labl"))
                                        prefix << "CueLabel" << cueLabelIndex++;
                                    else if (adtlChunkType == chunkName ("note"))
                                        prefix << "CueNote" << cueNoteIndex++;

                                    auto identifier = (uint32) input->readInt();
                                    auto stringLength = (int) adtlLength - 4;

                                    MemoryBlock textBlock;
                                    input->readIntoMemoryBlock (textBlock, stringLength);

                                    metadataValues.set (prefix + "Identifier", String (identifier));
                                    metadataValues.set (prefix + "Text", textBlock.toString());
                                }
                                else if (adtlChunkType == chunkName ("ltxt"))
                                {
                                    auto prefix = "CueRegion" + String (cueRegionIndex++);
                                    auto identifier     = (uint32) input->readInt();
                                    auto sampleLength   = (uint32) input->readInt();
                                    auto purpose        = (uint32) input->readInt();
                                    auto country        = (uint16) input->readShort();
                                    auto language       = (uint16) input->readShort();
                                    auto dialect        = (uint16) input->readShort();
                                    auto codePage       = (uint16) input->readShort();
                                    auto stringLength   = adtlLength - 20;

                                    MemoryBlock textBlock;
                                    input->readIntoMemoryBlock (textBlock, (int) stringLength);

                                    metadataValues.set (prefix + "Identifier",   String (identifier));
                                    metadataValues.set (prefix + "SampleLength", String (sampleLength));
                                    metadataValues.set (prefix + "Purpose",      String (purpose));
                                    metadataValues.set (prefix + "Country",      String (country));
                                    metadataValues.set (prefix + "Language",     String (language));
                                    metadataValues.set (prefix + "Dialect",      String (dialect));
                                    metadataValues.set (prefix + "CodePage",     String (codePage));
                                    metadataValues.set (prefix + "Text",         textBlock.toString());
                                }

                                input->setPosition (adtlChunkEnd);
                            }
                        }
                    }
                    else if (chunkType == chunkName ("acid"))
                    {
                        AcidChunk (*input, length).addToMetadata (metadataValues);
                    }
                    else if (chunkType == chunkName ("Trkn"))
                    {
                        MemoryBlock tracktion;
                        input->readIntoMemoryBlock (tracktion, (ssize_t) length);
                        metadataValues.set (WavAudioFormat::tracktionLoopInfo, tracktion.toString());
                    }
                    else if (chunkEnd <= input->getPosition())
                    {
                        break;
                    }

                    input->setPosition (chunkEnd);
                }
            }

            if (cueLabelIndex > 0)          metadataValues.set ("NumCueLabels",    String (cueLabelIndex));
            if (cueNoteIndex > 0)           metadataValues.set ("NumCueNotes",     String (cueNoteIndex));
            if (cueRegionIndex > 0)         metadataValues.set ("NumCueRegions",   String (cueRegionIndex));
            if (metadataValues.size() > 0)  metadataValues.set ("MetaDataSource",  "WAV");
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
            clearSamplesBeyondAvailableLength (destSamples, numDestChannels, startOffsetInDestBuffer,
                                               startSampleInFile, numSamples, lengthInSamples);

            if (numSamples <= 0)
                return true;

            input->setPosition (dataChunkStart + startSampleInFile * bytesPerFrame);

            while (numSamples > 0)
            {
                const int tempBufSize = 480 * 3 * 4; // (keep this a multiple of 3)
                char tempBuffer[tempBufSize];

                auto numThisTime = jmin (tempBufSize / bytesPerFrame, numSamples);
                auto bytesRead = input->read (tempBuffer, numThisTime * bytesPerFrame);

                if (bytesRead < numThisTime * bytesPerFrame)
                {
                    jassert (bytesRead >= 0);
                    zeromem (tempBuffer + bytesRead, (size_t) (numThisTime * bytesPerFrame - bytesRead));
                }

                copySampleData (bitsPerSample, usesFloatingPointData,
                                destSamples, startOffsetInDestBuffer, numDestChannels,
                                tempBuffer, (int) numChannels, numThisTime);

                startOffsetInDestBuffer += numThisTime;
                numSamples -= numThisTime;
            }

            return true;
        */
    }
    
    pub fn copy_sample_data(
        num_bits_per_sample:         u32,
        floating_point_data:         bool,
        dest_samples:                *mut *mut i32,
        start_offset_in_dest_buffer: i32,
        num_dest_channels:           i32,
        source_data:                 *const c_void,
        number_of_channels:          i32,
        num_samples:                 i32)  {
        
        todo!();
        /*
            switch (numBitsPerSample)
            {
                case 8:     ReadHelper<AudioData::Int32, AudioData::UInt8, AudioData::LittleEndian>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples); break;
                case 16:    ReadHelper<AudioData::Int32, AudioData::Int16, AudioData::LittleEndian>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples); break;
                case 24:    ReadHelper<AudioData::Int32, AudioData::Int24, AudioData::LittleEndian>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples); break;
                case 32:    if (floatingPointData) ReadHelper<AudioData::Float32, AudioData::Float32, AudioData::LittleEndian>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples);
                            else                   ReadHelper<AudioData::Int32,   AudioData::Int32,   AudioData::LittleEndian>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples);
                            break;
                default:    jassertfalse; break;
            }
        */
    }
    
    pub fn get_channel_layout(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            if (channelLayout.size() == static_cast<int> (numChannels))
                return channelLayout;

            return WavFileHelpers::canonicalWavChannelSet (static_cast<int> (numChannels));
        */
    }
    
    pub fn get_channel_layout_from_mask(
        dw_channel_mask:    i32,
        total_num_channels: usize) -> AudioChannelSet {
        
        todo!();
        /*
            AudioChannelSet wavFileChannelLayout;

            // AudioChannelSet and wav's dwChannelMask are compatible
            BigInteger channelBits (dwChannelMask);

            for (auto bit = channelBits.findNextSetBit (0); bit >= 0; bit = channelBits.findNextSetBit (bit + 1))
                wavFileChannelLayout.addChannel (static_cast<AudioChannelSet::ChannelType> (bit + 1));

            // channel layout and number of channels do not match
            if (wavFileChannelLayout.size() != static_cast<int> (totalNumChannels))
            {
                // for backward compatibility with old wav files, assume 1 or 2
                // channel wav files are mono/stereo respectively
                if (totalNumChannels <= 2 && dwChannelMask == 0)
                    wavFileChannelLayout = AudioChannelSet::canonicalChannelSet (static_cast<int> (totalNumChannels));
                else
                {
                    auto discreteSpeaker = static_cast<int> (AudioChannelSet::discreteChannel0);

                    while (wavFileChannelLayout.size() < static_cast<int> (totalNumChannels))
                        wavFileChannelLayout.addChannel (static_cast<AudioChannelSet::ChannelType> (discreteSpeaker++));
                }
            }

            return wavFileChannelLayout;
        */
    }
}

