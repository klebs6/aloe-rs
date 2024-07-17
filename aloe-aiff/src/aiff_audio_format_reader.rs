crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AiffAudioFormatReader<'a> {
    base:             AudioFormatReader<'a>,
    bytes_per_frame:  i32,
    data_chunk_start: i64,
    little_endian:    bool,
}

impl<'a> AiffAudioFormatReader<'a> {

    pub fn new(in_: *mut dyn Read) -> Self {
    
        todo!();
        /*
        : audio_format_reader(in, aiffFormatName),

            using namespace AiffFileHelpers;

            std::map<String, String> metadataValuesMap;

            for (int i = 0; i != metadataValues.size(); ++i)
            {
                metadataValuesMap.emplace (metadataValues.getAllKeys().getReference (i),
                                           metadataValues.getAllValues().getReference (i));
            }

            // If this fails, there were duplicate keys in the metadata
            jassert ((size_t) metadataValuesMap.size() == (size_t) metadataValues.size());

            if (input->readInt() == chunkName ("FORM"))
            {
                auto len = input->readIntBigEndian();
                auto end = input->getPosition() + len;
                auto nextType = input->readInt();

                if (nextType == chunkName ("AIFF") || nextType == chunkName ("AIFC"))
                {
                    bool hasGotVer = false;
                    bool hasGotData = false;
                    bool hasGotType = false;

                    while (input->getPosition() < end)
                    {
                        auto type = input->readInt();
                        auto length = (uint32) input->readIntBigEndian();
                        auto chunkEnd = input->getPosition() + length;

                        if (type == chunkName ("FVER"))
                        {
                            hasGotVer = true;
                            auto ver = input->readIntBigEndian();

                            if (ver != 0 && ver != (int) 0xa2805140)
                                break;
                        }
                        else if (type == chunkName ("COMM"))
                        {
                            hasGotType = true;

                            numChannels = (unsigned int) input->readShortBigEndian();
                            lengthInSamples = input->readIntBigEndian();
                            bitsPerSample = (unsigned int) input->readShortBigEndian();
                            bytesPerFrame = (int) ((numChannels * bitsPerSample) >> 3);

                            unsigned char sampleRateBytes[10];
                            input->read (sampleRateBytes, 10);
                            const int byte0 = sampleRateBytes[0];

                            if ((byte0 & 0x80) != 0
                                 || byte0 <= 0x3F || byte0 > 0x40
                                 || (byte0 == 0x40 && sampleRateBytes[1] > 0x1C))
                                break;

                            auto sampRate = ByteOrder::bigEndianInt (sampleRateBytes + 2);
                            sampRate >>= (16414 - ByteOrder::bigEndianShort (sampleRateBytes));
                            sampleRate = (int) sampRate;

                            if (length <= 18)
                            {
                                // some types don't have a chunk large enough to include a compression
                                // type, so assume it's just big-endian pcm
                                littleEndian = false;
                            }
                            else
                            {
                                auto compType = input->readInt();

                                if (compType == chunkName ("NONE") || compType == chunkName ("twos"))
                                {
                                    littleEndian = false;
                                }
                                else if (compType == chunkName ("sowt"))
                                {
                                    littleEndian = true;
                                }
                                else if (compType == chunkName ("fl32") || compType == chunkName ("FL32"))
                                {
                                    littleEndian = false;
                                    usesFloatingPointData = true;
                                }
                                else
                                {
                                    sampleRate = 0;
                                    break;
                                }
                            }
                        }
                        else if (type == chunkName ("SSND"))
                        {
                            hasGotData = true;

                            auto offset = input->readIntBigEndian();
                            dataChunkStart = input->getPosition() + 4 + offset;
                            lengthInSamples = (bytesPerFrame > 0) ? jmin (lengthInSamples, ((int64) length) / (int64) bytesPerFrame) : 0;
                        }
                        else if (type == chunkName ("MARK"))
                        {
                            auto numCues = (uint16) input->readShortBigEndian();

                            // these two are always the same for AIFF-read files
                            metadataValuesMap.emplace ("NumCuePoints", String (numCues));
                            metadataValuesMap.emplace ("NumCueLabels", String (numCues));

                            for (uint16 i = 0; i < numCues; ++i)
                            {
                                auto identifier = (uint16) input->readShortBigEndian();
                                auto offset = (uint32) input->readIntBigEndian();
                                auto stringLength = (uint8) input->readByte();
                                MemoryBlock textBlock;
                                input->readIntoMemoryBlock (textBlock, stringLength);

                                // if the stringLength is even then read one more byte as the
                                // string needs to be an even number of bytes INCLUDING the
                                // leading length character in the pascal string
                                if ((stringLength & 1) == 0)
                                    input->readByte();

                                auto prefixCue = "Cue" + String (i);
                                metadataValuesMap.emplace (prefixCue + "Identifier", String (identifier));
                                metadataValuesMap.emplace (prefixCue + "Offset", String (offset));

                                auto prefixLabel = "CueLabel" + String (i);
                                metadataValuesMap.emplace (prefixLabel + "Identifier", String (identifier));
                                metadataValuesMap.emplace (prefixLabel + "Text", textBlock.toString());
                            }
                        }
                        else if (type == chunkName ("COMT"))
                        {
                            auto numNotes = (uint16) input->readShortBigEndian();
                            metadataValuesMap.emplace ("NumCueNotes", String (numNotes));

                            for (uint16 i = 0; i < numNotes; ++i)
                            {
                                auto timestamp = (uint32) input->readIntBigEndian();
                                auto identifier = (uint16) input->readShortBigEndian(); // may be zero in this case
                                auto stringLength = (uint16) input->readShortBigEndian();

                                MemoryBlock textBlock;
                                input->readIntoMemoryBlock (textBlock, stringLength + (stringLength & 1));

                                auto prefix = "CueNote" + String (i);
                                metadataValuesMap.emplace (prefix + "TimeStamp", String (timestamp));
                                metadataValuesMap.emplace (prefix + "Identifier", String (identifier));
                                metadataValuesMap.emplace (prefix + "Text", textBlock.toString());
                            }
                        }
                        else if (type == chunkName ("INST"))
                        {
                            HeapBlock<InstChunk> inst;
                            inst.calloc (jmax ((size_t) length + 1, sizeof (InstChunk)), 1);
                            input->read (inst, (int) length);
                            inst->copyTo (metadataValuesMap);
                        }
                        else if (type == chunkName ("basc"))
                        {
                            AiffFileHelpers::BASCChunk (*input).addToMetadata (metadataValuesMap);
                        }
                        else if (type == chunkName ("cate"))
                        {
                            metadataValuesMap.emplace (AiffAudioFormat::appleTag,
                                                      AiffFileHelpers::CATEChunk::read (*input, length));
                        }
                        else if ((hasGotVer && hasGotData && hasGotType)
                                  || chunkEnd < input->getPosition()
                                  || input->isExhausted())
                        {
                            break;
                        }

                        input->setPosition (chunkEnd + (chunkEnd & 1)); // (chunks should be aligned to an even byte address)
                    }
                }
            }

            if (metadataValuesMap.size() > 0)
                metadataValuesMap.emplace ("MetaDataSource", "AIFF");

            metadataValues.addMap (metadataValuesMap);
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
                char tempBuffer [tempBufSize];

                const int numThisTime = jmin (tempBufSize / bytesPerFrame, numSamples);
                const int bytesRead = input->read (tempBuffer, numThisTime * bytesPerFrame);

                if (bytesRead < numThisTime * bytesPerFrame)
                {
                    jassert (bytesRead >= 0);
                    zeromem (tempBuffer + bytesRead, (size_t) (numThisTime * bytesPerFrame - bytesRead));
                }

                if (littleEndian)
                    copySampleData<AudioData::LittleEndian> (bitsPerSample, usesFloatingPointData,
                                                             destSamples, startOffsetInDestBuffer, numDestChannels,
                                                             tempBuffer, (int) numChannels, numThisTime);
                else
                    copySampleData<AudioData::BigEndian> (bitsPerSample, usesFloatingPointData,
                                                          destSamples, startOffsetInDestBuffer, numDestChannels,
                                                          tempBuffer, (int) numChannels, numThisTime);

                startOffsetInDestBuffer += numThisTime;
                numSamples -= numThisTime;
            }

            return true;
        */
    }
    
    pub fn copy_sample_data<Endianness>(
        num_bits_per_sample:         u32,
        floating_point_data:         bool,
        dest_samples:                *const *const i32,
        start_offset_in_dest_buffer: i32,
        num_dest_channels:           i32,
        source_data:                 *const c_void,
        number_of_channels:          i32,
        num_samples:                 i32)  {
    
        todo!();
        /*
            switch (numBitsPerSample)
            {
                case 8:     ReadHelper<AudioData::Int32, AudioData::Int8,  Endianness>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples); break;
                case 16:    ReadHelper<AudioData::Int32, AudioData::Int16, Endianness>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples); break;
                case 24:    ReadHelper<AudioData::Int32, AudioData::Int24, Endianness>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples); break;
                case 32:    if (floatingPointData) ReadHelper<AudioData::Float32, AudioData::Float32, Endianness>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples);
                            else                   ReadHelper<AudioData::Int32,   AudioData::Int32,   Endianness>::read (destSamples, startOffsetInDestBuffer, numDestChannels, sourceData, numberOfChannels, numSamples);
                            break;
                default:    jassertfalse; break;
            }
        */
    }
}
