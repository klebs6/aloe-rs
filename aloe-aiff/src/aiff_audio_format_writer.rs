crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AiffAudioFormatWriter<'a, W: Write> {
    base:              AudioFormatWriter<'a, W>,
    temp_block:        MemoryBlock,
    mark_chunk:        MemoryBlock,
    comt_chunk:        MemoryBlock,
    inst_chunk:        MemoryBlock,
    length_in_samples: u64, // default = 0
    bytes_written:     u64, // default = 0
    header_position:   i64, // default = 0
    write_failed:      bool, // default = false
}

impl<'a, W: Write> Drop for AiffAudioFormatWriter<'a, W> {

    fn drop(&mut self) {
        todo!();
        /*
            if ((bytesWritten & 1) != 0)
                output->writeByte (0);

            writeHeader();
        */
    }
}

impl<'a, W: Write> AiffAudioFormatWriter<'a, W> {

    pub fn new(
        out:             *mut dyn Write,
        rate:            f64,
        num_chans:       u32,
        bits:            u32,
        metadata_values: &StringPairArray) -> Self {
    
        todo!();
        /*
        : audio_format_writer(out, aiffFormatName, rate, numChans, bits),

            using namespace AiffFileHelpers;

            if (metadataValues.size() > 0)
            {
                // The meta data should have been sanitised for the AIFF format.
                // If it was originally sourced from a WAV file the MetaDataSource
                // key should be removed (or set to "AIFF") once this has been done
                jassert (metadataValues.getValue ("MetaDataSource", "None") != "WAV");

                MarkChunk::create (markChunk, metadataValues);
                COMTChunk::create (comtChunk, metadataValues);
                InstChunk::create (instChunk, metadataValues);
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
                case 8:     WriteHelper<AudioData::Int8,  AudioData::Int32, AudioData::BigEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                case 16:    WriteHelper<AudioData::Int16, AudioData::Int32, AudioData::BigEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                case 24:    WriteHelper<AudioData::Int24, AudioData::Int32, AudioData::BigEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                case 32:    WriteHelper<AudioData::Int32, AudioData::Int32, AudioData::BigEndian>::write (tempBlock.getData(), (int) numChannels, data, numSamples); break;
                default:    jassertfalse; break;
            }

            if (bytesWritten + bytes >= (size_t) 0xfff00000
                 || ! output->write (tempBlock.getData(), bytes))
            {
                // failed to write to disk, so let's try writing the header.
                // If it's just run out of disk space, then if it does manage
                // to write the header, we'll still have a useable file..
                writeHeader();
                writeFailed = true;
                return false;
            }

            bytesWritten += bytes;
            lengthInSamples += (uint64) numSamples;
            return true;
        */
    }
    
    pub fn write_header(&mut self)  {
        
        todo!();
        /*
            using namespace AiffFileHelpers;

            const bool couldSeekOk = output->setPosition (headerPosition);
            ignoreUnused (couldSeekOk);

            // if this fails, you've given it an output stream that can't seek! It needs
            // to be able to seek back to write the header
            jassert (couldSeekOk);

            auto headerLen = (int) (54 + (markChunk.isEmpty() ? 0 : markChunk.getSize() + 8)
                                       + (comtChunk.isEmpty() ? 0 : comtChunk.getSize() + 8)
                                       + (instChunk.isEmpty() ? 0 : instChunk.getSize() + 8));
            auto audioBytes = (int) (lengthInSamples * ((bitsPerSample * numChannels) / 8));
            audioBytes += (audioBytes & 1);

            output->writeInt (chunkName ("FORM"));
            output->writeIntBigEndian (headerLen + audioBytes - 8);
            output->writeInt (chunkName ("AIFF"));
            output->writeInt (chunkName ("COMM"));
            output->writeIntBigEndian (18);
            output->writeShortBigEndian ((short) numChannels);
            output->writeIntBigEndian ((int) lengthInSamples);
            output->writeShortBigEndian ((short) bitsPerSample);

            uint8 sampleRateBytes[10] = {};

            if (sampleRate <= 1)
            {
                sampleRateBytes[0] = 0x3f;
                sampleRateBytes[1] = 0xff;
                sampleRateBytes[2] = 0x80;
            }
            else
            {
                int mask = 0x40000000;
                sampleRateBytes[0] = 0x40;

                if (sampleRate >= mask)
                {
                    jassertfalse;
                    sampleRateBytes[1] = 0x1d;
                }
                else
                {
                    int n = (int) sampleRate;
                    int i;

                    for (i = 0; i <= 32 ; ++i)
                    {
                        if ((n & mask) != 0)
                            break;

                        mask >>= 1;
                    }

                    n = n << (i + 1);

                    sampleRateBytes[1] = (uint8) (29 - i);
                    sampleRateBytes[2] = (uint8) ((n >> 24) & 0xff);
                    sampleRateBytes[3] = (uint8) ((n >> 16) & 0xff);
                    sampleRateBytes[4] = (uint8) ((n >>  8) & 0xff);
                    sampleRateBytes[5] = (uint8) (n & 0xff);
                }
            }

            output->write (sampleRateBytes, 10);

            if (! markChunk.isEmpty())
            {
                output->writeInt (chunkName ("MARK"));
                output->writeIntBigEndian ((int) markChunk.getSize());
                *output << markChunk;
            }

            if (! comtChunk.isEmpty())
            {
                output->writeInt (chunkName ("COMT"));
                output->writeIntBigEndian ((int) comtChunk.getSize());
                *output << comtChunk;
            }

            if (! instChunk.isEmpty())
            {
                output->writeInt (chunkName ("INST"));
                output->writeIntBigEndian ((int) instChunk.getSize());
                *output << instChunk;
            }

            output->writeInt (chunkName ("SSND"));
            output->writeIntBigEndian (audioBytes + 8);
            output->writeInt (0);
            output->writeInt (0);

            jassert (output->getPosition() == headerLen);
        */
    }
}
