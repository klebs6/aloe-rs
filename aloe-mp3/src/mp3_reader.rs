#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub const MP3_FORMAT_NAME: &'static str = "MP3 file";

pub const MP3_READER_DECODED_DATA_SIZE: usize = 1152;

#[no_copy]
#[leak_detector]
pub struct MP3Reader<'a> {
    base:             AudioFormatReader<'a>,
    stream:           MP3Stream,
    current_position: i64,
    decoded0:         [f32; MP3_READER_DECODED_DATA_SIZE],
    decoded1:         [f32; MP3_READER_DECODED_DATA_SIZE],
    decoded_start:    i32,
    decoded_end:      i32,
}

impl<'a> MP3Reader<'a> {

    pub fn new(in_: *mut dyn Read) -> Self {
    
        todo!();
        /*


            : AudioFormatReader (in, mp3FormatName),
              stream (*in), currentPosition (0),
              decodedStart (0), decodedEnd (0)
            skipID3();
            const int64 streamPos = stream.stream.getPosition();

            if (readNextBlock())
            {
                bitsPerSample = 32;
                usesFloatingPointData = true;
                sampleRate = stream.frame.getFrequency();
                numChannels = (unsigned int) stream.frame.numChannels;
                lengthInSamples = findLength (streamPos);
            }
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
            if (destSamples == nullptr)
            {
                jassertfalse;
                return false;
            }

            if (currentPosition != startSampleInFile)
            {
                if (! stream.seek ((int) (startSampleInFile / 1152 - 1)))
                {
                    currentPosition = -1;
                    createEmptyDecodedData();
                }
                else
                {
                    decodedStart = decodedEnd = 0;
                    const int64 streamPos = stream.currentFrameIndex * 1152;
                    int toSkip = (int) (startSampleInFile - streamPos);
                    jassert (toSkip >= 0);

                    while (toSkip > 0)
                    {
                        if (! readNextBlock())
                        {
                            createEmptyDecodedData();
                            break;
                        }

                        const int numReady = decodedEnd - decodedStart;

                        if (numReady > toSkip)
                        {
                            decodedStart += toSkip;
                            break;
                        }

                        toSkip -= numReady;
                    }

                    currentPosition = startSampleInFile;
                }
            }

            while (numSamples > 0)
            {
                if (decodedEnd <= decodedStart && ! readNextBlock())
                {
                    for (int i = numDestChannels; --i >= 0;)
                        if (destSamples[i] != nullptr)
                            zeromem (destSamples[i] + startOffsetInDestBuffer, (size_t) numSamples * sizeof (float));

                    return false;
                }

                const int numToCopy = jmin (decodedEnd - decodedStart, numSamples);
                float* const* const dst = reinterpret_cast<float**> (destSamples);
                memcpy (dst[0] + startOffsetInDestBuffer, decoded0 + decodedStart, (size_t) numToCopy * sizeof (float));

                if (numDestChannels > 1 && dst[1] != nullptr)
                    memcpy (dst[1] + startOffsetInDestBuffer, (numChannels < 2 ? decoded0 : decoded1) + decodedStart, (size_t) numToCopy * sizeof (float));

                startOffsetInDestBuffer += numToCopy;
                decodedStart += numToCopy;
                currentPosition += numToCopy;
                numSamples -= numToCopy;
            }

            return true;
        */
    }
    
    pub fn create_empty_decoded_data(&mut self)  {
        
        todo!();
        /*
            zeromem (decoded0, sizeof (decoded0));
            zeromem (decoded1, sizeof (decoded1));
            decodedStart = 0;
            decodedEnd = MP3_READER_DECODED_DATA_SIZE;
        */
    }
    
    pub fn read_next_block(&mut self) -> bool {
        
        todo!();
        /*
            for (int attempts = 10; --attempts >= 0;)
            {
                int samplesDone = 0;
                const int result = stream.decodeNextBlock (decoded0, decoded1, samplesDone);

                if (result > 0 && stream.stream.isExhausted())
                {
                    createEmptyDecodedData();
                    return true;
                }

                if (result <= 0)
                {
                    decodedStart = 0;
                    decodedEnd = samplesDone;
                    return result == 0;
                }
            }

            return false;
        */
    }
    
    pub fn skipid3(&mut self)  {
        
        todo!();
        /*
            const int64 originalPosition = stream.stream.getPosition();
            const uint32 firstWord = (uint32) stream.stream.readInt();

            if ((firstWord & 0xffffff) == 0x334449)
            {
                uint8 buffer[6];

                if (stream.stream.read (buffer, 6) == 6
                     && buffer[0] != 0xff
                     && ((buffer[2] | buffer[3] | buffer[4] | buffer[5]) & 0x80) == 0)
                {
                    const uint32 length = (((uint32) buffer[2]) << 21)
                                        | (((uint32) buffer[3]) << 14)
                                        | (((uint32) buffer[4]) << 7)
                                        |  ((uint32) buffer[5]);

                    stream.stream.skipNextBytes (length);
                    return;
                }
            }

            stream.stream.setPosition (originalPosition);
        */
    }
    
    pub fn find_length(&mut self, stream_start_pos: i64) -> i64 {
        
        todo!();
        /*
            int64 numFrames = stream.numFrames;

            if (numFrames <= 0)
            {
                const int64 streamSize = stream.stream.getTotalLength();

                if (streamSize > 0)
                {
                    const int bytesPerFrame = stream.frame.frameSize + 4;

                    if (bytesPerFrame == 417 || bytesPerFrame == 418)
                        numFrames = roundToInt ((double) (streamSize - streamStartPos) / 417.95918); // more accurate for 128k
                    else
                        numFrames = (streamSize - streamStartPos) / bytesPerFrame;
                }
            }

            return numFrames * 1152;
        */
    }
}
