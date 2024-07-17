crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MemoryMappedAiffReader<'a, R: Read> {
    base:          MemoryMappedAudioFormatReader<'a,R>,
    little_endian: bool,
}

impl<'a, R: Read> MemoryMappedAiffReader<'a, R> {

    pub fn new(
        f:      &File,
        reader: &AiffAudioFormatReader) -> Self {
    
        todo!();
        /*


            : MemoryMappedAudioFormatReader (f, reader, reader.dataChunkStart,
                                             reader.bytesPerFrame * reader.lengthInSamples, reader.bytesPerFrame),
              littleEndian (reader.littleEndian)
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

            if (map == nullptr || ! mappedSection.contains (Range<int64> (startSampleInFile, startSampleInFile + numSamples)))
            {
                jassertfalse; // you must make sure that the window contains all the samples you're going to attempt to read.
                return false;
            }

            if (littleEndian)
                AiffAudioFormatReader::copySampleData<AudioData::LittleEndian>
                        (bitsPerSample, usesFloatingPointData, destSamples, startOffsetInDestBuffer,
                         numDestChannels, sampleToPointer (startSampleInFile), (int) numChannels, numSamples);
            else
                AiffAudioFormatReader::copySampleData<AudioData::BigEndian>
                        (bitsPerSample, usesFloatingPointData, destSamples, startOffsetInDestBuffer,
                         numDestChannels, sampleToPointer (startSampleInFile), (int) numChannels, numSamples);

            return true;
        */
    }
    
    pub fn get_sample(&self, 
        sample: i64,
        result: *mut f32)  {
        
        todo!();
        /*
            auto num = (int) numChannels;

            if (map == nullptr || ! mappedSection.contains (sample))
            {
                jassertfalse; // you must make sure that the window contains all the samples you're going to attempt to read.

                zeromem (result, (size_t) num * sizeof (float));
                return;
            }

            float** dest = &result;
            const void* source = sampleToPointer (sample);

            if (littleEndian)
            {
                switch (bitsPerSample)
                {
                    case 8:     ReadHelper<AudioData::Float32, AudioData::UInt8, AudioData::LittleEndian>::read (dest, 0, 1, source, 1, num); break;
                    case 16:    ReadHelper<AudioData::Float32, AudioData::Int16, AudioData::LittleEndian>::read (dest, 0, 1, source, 1, num); break;
                    case 24:    ReadHelper<AudioData::Float32, AudioData::Int24, AudioData::LittleEndian>::read (dest, 0, 1, source, 1, num); break;
                    case 32:    if (usesFloatingPointData) ReadHelper<AudioData::Float32, AudioData::Float32, AudioData::LittleEndian>::read (dest, 0, 1, source, 1, num);
                                else                       ReadHelper<AudioData::Float32, AudioData::Int32,   AudioData::LittleEndian>::read (dest, 0, 1, source, 1, num);
                                break;
                    default:    jassertfalse; break;
                }
            }
            else
            {
                switch (bitsPerSample)
                {
                    case 8:     ReadHelper<AudioData::Float32, AudioData::UInt8, AudioData::BigEndian>::read (dest, 0, 1, source, 1, num); break;
                    case 16:    ReadHelper<AudioData::Float32, AudioData::Int16, AudioData::BigEndian>::read (dest, 0, 1, source, 1, num); break;
                    case 24:    ReadHelper<AudioData::Float32, AudioData::Int24, AudioData::BigEndian>::read (dest, 0, 1, source, 1, num); break;
                    case 32:    if (usesFloatingPointData) ReadHelper<AudioData::Float32, AudioData::Float32, AudioData::BigEndian>::read (dest, 0, 1, source, 1, num);
                                else                       ReadHelper<AudioData::Float32, AudioData::Int32,   AudioData::BigEndian>::read (dest, 0, 1, source, 1, num);
                                break;
                    default:    jassertfalse; break;
                }
            }
        */
    }
    
    pub fn read_max_levels(&mut self, 
        start_sample_in_file: i64,
        num_samples:          i64,
        results:              *mut Range<f32>,
        num_channels_to_read: i32)  {
        
        todo!();
        /*
            numSamples = jmin (numSamples, lengthInSamples - startSampleInFile);

            if (map == nullptr || numSamples <= 0 || ! mappedSection.contains (Range<int64> (startSampleInFile, startSampleInFile + numSamples)))
            {
                jassert (numSamples <= 0); // you must make sure that the window contains all the samples you're going to attempt to read.

                for (int i = 0; i < numChannelsToRead; ++i)
                    results[i] = Range<float>();

                return;
            }

            switch (bitsPerSample)
            {
                case 8:     scanMinAndMax<AudioData::UInt8> (startSampleInFile, numSamples, results, numChannelsToRead); break;
                case 16:    scanMinAndMax<AudioData::Int16> (startSampleInFile, numSamples, results, numChannelsToRead); break;
                case 24:    scanMinAndMax<AudioData::Int24> (startSampleInFile, numSamples, results, numChannelsToRead); break;
                case 32:    if (usesFloatingPointData) scanMinAndMax<AudioData::Float32> (startSampleInFile, numSamples, results, numChannelsToRead);
                            else                       scanMinAndMax<AudioData::Int32>   (startSampleInFile, numSamples, results, numChannelsToRead);
                            break;
                default:    jassertfalse; break;
            }
        */
    }
    
    pub fn scan_min_and_max<SampleType>(&self, 
        start_sample_in_file: i64,
        num_samples:          i64,
        results:              *mut Range<f32>,
        num_channels_to_read: i32)  {
    
        todo!();
        /*
            for (int i = 0; i < numChannelsToRead; ++i)
                results[i] = scanMinAndMaxForChannel<SampleType> (i, startSampleInFile, numSamples);
        */
    }
    
    pub fn scan_min_and_max_for_channel<SampleType>(&self, 
        channel:              i32,
        start_sample_in_file: i64,
        num_samples:          i64) -> Range<f32> {
    
        todo!();
        /*
            return littleEndian ? scanMinAndMaxInterleaved<SampleType, AudioData::LittleEndian> (channel, startSampleInFile, numSamples)
                                : scanMinAndMaxInterleaved<SampleType, AudioData::BigEndian>    (channel, startSampleInFile, numSamples);
        */
    }
}
