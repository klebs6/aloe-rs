crate::ix!();

#[no_copy]
#[leak_detector]
pub struct WavAudioFormatWriter<'a, W: Write> {
    base:              AudioFormatWriter<'a,W>,
    temp_block:        MemoryBlock,
    bwav_chunk:        MemoryBlock,
    axml_chunk:        MemoryBlock,
    smpl_chunk:        MemoryBlock,
    inst_chunk:        MemoryBlock,
    cue_chunk:         MemoryBlock,
    list_chunk:        MemoryBlock,
    list_info_chunk:   MemoryBlock,
    acid_chunk:        MemoryBlock,
    trck_chunk:        MemoryBlock,
    length_in_samples: u64, // default = 0
    bytes_written:     u64, // default = 0
    header_position:   i64, // default = 0
    write_failed:      bool, // default = false
}

impl<'a, W: Write> Drop for WavAudioFormatWriter<'a, W> {

    fn drop(&mut self) {
        todo!();
        /*
            writeHeader();
        */
    }
}

impl<'a, W: Write> WavAudioFormatWriter<'a, W> {

    pub fn new(
        out:                   *mut OutputStream,
        rate:                  f64,
        channel_layout_to_use: &AudioChannelSet,
        bits:                  u32,
        metadata_values:       &StringPairArray) -> Self {
    
        todo!();
        /*
        : audio_format_writer(out, wavFormatName, rate, channelLayoutToUse, bits),

            using namespace WavFileHelpers;

            if (metadataValues.size() > 0)
            {
                // The meta data should have been sanitised for the WAV format.
                // If it was originally sourced from an AIFF file the MetaDataSource
                // key should be removed (or set to "WAV") once this has been done
                jassert (metadataValues.getValue ("MetaDataSource", "None") != "AIFF");

                bwavChunk     = BWAVChunk::createFrom (metadataValues);
                axmlChunk     = AXMLChunk::createFrom (metadataValues);
                smplChunk     = SMPLChunk::createFrom (metadataValues);
                instChunk     = InstChunk::createFrom (metadataValues);
                cueChunk      = CueChunk ::createFrom (metadataValues);
                listChunk     = ListChunk::createFrom (metadataValues);
                listInfoChunk = ListInfoChunk::createFrom (metadataValues);
                acidChunk     = AcidChunk::createFrom (metadataValues);
                trckChunk     = TracktionChunk::createFrom (metadataValues);
            }

            headerPosition = out->getPosition();
            writeHeader();
        */
    }
    
    pub fn write(&mut self, 
        data:        *const *const i32,
        num_samples: i32) -> bool {
        
        todo!();
        /*
            jassert (numSamples >= 0);
            jassert (data != nullptr && *data != nullptr); // the input must contain at least one channel!

            if (writeFailed)
                return false;

            auto bytes = numChannels * (size_t) numSamples * bitsPerSample / 8;
            tempBlock.ensureSize (bytes, false);

            switch (bitsPerSample)
            {
                case 8:     WriteHelper<AudioData::UInt8, AudioData::Int32, AudioData::LittleEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                case 16:    WriteHelper<AudioData::Int16, AudioData::Int32, AudioData::LittleEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                case 24:    WriteHelper<AudioData::Int24, AudioData::Int32, AudioData::LittleEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                case 32:    WriteHelper<AudioData::Int32, AudioData::Int32, AudioData::LittleEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                default:    jassertfalse; break;
            }

            if (! output->write (tempBlock.getData(), bytes))
            {
                // failed to write to disk, so let's try writing the header.
                // If it's just run out of disk space, then if it does manage
                // to write the header, we'll still have a usable file..
                writeHeader();
                writeFailed = true;
                return false;
            }

            bytesWritten += bytes;
            lengthInSamples += (uint64) numSamples;
            return true;
        */
    }
    
    pub fn flush(&mut self) -> bool {
        
        todo!();
        /*
            auto lastWritePos = output->getPosition();
            writeHeader();

            if (output->setPosition (lastWritePos))
                return true;

            // if this fails, you've given it an output stream that can't seek! It needs
            // to be able to seek back to write the header
            jassertfalse;
            return false;
        */
    }
    
    pub fn write_header(&mut self)  {
        
        todo!();
        /*
            if ((bytesWritten & 1) != 0) // pad to an even length
                output->writeByte (0);

            using namespace WavFileHelpers;

            if (headerPosition != output->getPosition() && ! output->setPosition (headerPosition))
            {
                // if this fails, you've given it an output stream that can't seek! It needs to be
                // able to seek back to go back and write the header after the data has been written.
                jassertfalse;
                return;
            }

            const size_t bytesPerFrame = numChannels * bitsPerSample / 8;
            uint64 audioDataSize = bytesPerFrame * lengthInSamples;
            auto channelMask = getChannelMaskFromChannelLayout (channelLayout);

            const bool isRF64 = (bytesWritten >= 0x100000000LL);
            const bool isWaveFmtEx = isRF64 || (channelMask != 0);

            int64 riffChunkSize = (int64) (4 /* 'RIFF' */ + 8 + 40 /* WAVEFORMATEX */
                                           + 8 + audioDataSize + (audioDataSize & 1)
                                           + chunkSize (bwavChunk)
                                           + chunkSize (axmlChunk)
                                           + chunkSize (smplChunk)
                                           + chunkSize (instChunk)
                                           + chunkSize (cueChunk)
                                           + chunkSize (listChunk)
                                           + chunkSize (listInfoChunk)
                                           + chunkSize (acidChunk)
                                           + chunkSize (trckChunk)
                                           + (8 + 28)); // (ds64 chunk)

            riffChunkSize += (riffChunkSize & 1);

            if (isRF64)
                writeChunkHeader (chunkName ("RF64"), -1);
            else
                writeChunkHeader (chunkName ("RIFF"), (int) riffChunkSize);

            output->writeInt (chunkName ("WAVE"));

            if (! isRF64)
            {
               #if ! ALOE_WAV_DO_NOT_PAD_HEADER_SIZE
                /* NB: This junk chunk is added for padding, so that the header is a fixed size
                   regardless of whether it's RF64 or not. That way, we can begin recording a file,
                   and when it's finished, can go back and write either a RIFF or RF64 header,
                   depending on whether more than 2^32 samples were written.

                   The ALOE_WAV_DO_NOT_PAD_HEADER_SIZE macro allows you to disable this feature in case
                   you need to create files for crappy WAV players with bugs that stop them skipping chunks
                   which they don't recognise. But DO NOT USE THIS option unless you really have no choice,
                   because it means that if you write more than 2^32 samples to the file, you'll corrupt it.
                */
                writeChunkHeader (chunkName ("JUNK"), 28 + (isWaveFmtEx? 0 : 24));
                output->writeRepeatedByte (0, 28 /* ds64 */ + (isWaveFmtEx? 0 : 24));
               #endif
            }
            else
            {
               #if ALOE_WAV_DO_NOT_PAD_HEADER_SIZE
                // If you disable padding, then you MUST NOT write more than 2^32 samples to a file.
                jassertfalse;
               #endif

                writeChunkHeader (chunkName ("ds64"), 28);  // chunk size for uncompressed data (no table)
                output->writeInt64 (riffChunkSize);
                output->writeInt64 ((int64) audioDataSize);
                output->writeRepeatedByte (0, 12);
            }

            if (isWaveFmtEx)
            {
                writeChunkHeader (chunkName ("fmt "), 40);
                output->writeShort ((short) (uint16) 0xfffe); // WAVE_FORMAT_EXTENSIBLE
            }
            else
            {
                writeChunkHeader (chunkName ("fmt "), 16);
                output->writeShort (bitsPerSample < 32 ? (short) 1 /*WAVE_FORMAT_PCM*/
                                                       : (short) 3 /*WAVE_FORMAT_IEEE_FLOAT*/);
            }

            output->writeShort ((short) numChannels);
            output->writeInt ((int) sampleRate);
            output->writeInt ((int) ((double) bytesPerFrame * sampleRate)); // nAvgBytesPerSec
            output->writeShort ((short) bytesPerFrame); // nBlockAlign
            output->writeShort ((short) bitsPerSample); // wBitsPerSample

            if (isWaveFmtEx)
            {
                output->writeShort (22); // cbSize (size of the extension)
                output->writeShort ((short) bitsPerSample); // wValidBitsPerSample
                output->writeInt (channelMask);

                const ExtensibleWavSubFormat& subFormat = bitsPerSample < 32 ? pcmFormat : IEEEFloatFormat;

                output->writeInt ((int) subFormat.data1);
                output->writeShort ((short) subFormat.data2);
                output->writeShort ((short) subFormat.data3);
                output->write (subFormat.data4, sizeof (subFormat.data4));
            }

            writeChunk (bwavChunk,     chunkName ("bext"));
            writeChunk (axmlChunk,     chunkName ("axml"));
            writeChunk (smplChunk,     chunkName ("smpl"));
            writeChunk (instChunk,     chunkName ("inst"), 7);
            writeChunk (cueChunk,      chunkName ("cue "));
            writeChunk (listChunk,     chunkName ("LIST"));
            writeChunk (listInfoChunk, chunkName ("LIST"));
            writeChunk (acidChunk,     chunkName ("acid"));
            writeChunk (trckChunk,     chunkName ("Trkn"));

            writeChunkHeader (chunkName ("data"), isRF64 ? -1 : (int) (lengthInSamples * bytesPerFrame));

            usesFloatingPointData = (bitsPerSample == 32);
        */
    }
    
    pub fn chunk_size(data: &MemoryBlock) -> usize {
        
        todo!();
        /*
            return data.isEmpty() ? 0 : (8 + data.getSize());
        */
    }
    
    pub fn write_chunk_header(&self, 
        chunk_type: i32,
        size:       i32)  {
        
        todo!();
        /*
            output->writeInt (chunkType);
            output->writeInt (size);
        */
    }
    
    pub fn write_chunk(
        &self, 
        data:       &MemoryBlock,
        chunk_type: i32,
        size:       Option<i32>

    ) {

        let size: i32 = size.unwrap_or(0);

        todo!();
        /*
            if (! data.isEmpty())
            {
                writeChunkHeader (chunkType, size != 0 ? size : (int) data.getSize());
                *output << data;
            }
        */
    }
    
    pub fn get_channel_mask_from_channel_layout(layout: &AudioChannelSet) -> i32 {
        
        todo!();
        /*
            if (layout.isDiscreteLayout())
                return 0;

            // Don't add an extended format chunk for mono and stereo. Basically, all wav players
            // interpret a wav file with only one or two channels to be mono or stereo anyway.
            if (layout == AudioChannelSet::mono() || layout == AudioChannelSet::stereo())
                return 0;

            auto channels = layout.getChannelTypes();
            auto wavChannelMask = 0;

            for (auto channel : channels)
            {
                int wavChannelBit = static_cast<int> (channel) - 1;
                jassert (wavChannelBit >= 0 && wavChannelBit <= 31);

                wavChannelMask |= (1 << wavChannelBit);
            }

            return wavChannelMask;
        */
    }
}
