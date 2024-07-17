crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_BufferingAudioFormatReader.h]

/**
  | An AudioFormatReader that uses a background
  | thread to pre-read data from another
  | reader.
  | 
  | @see AudioFormatReader
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct BufferingAudioReader<'a, R: Read> {
    base:               AudioFormatReader<'a>,
    base2:              TimeSliceClient,
    source:             Box<AudioFormatReader<'a>>,
    thread:             &'a mut TimeSliceThread,
    next_read_position: Atomic<i64>, // default = { 0  }
    num_blocks:         i32,
    timeout_ms:         i32, // default = 0
    lock:               CriticalSection,
    blocks:             Vec<Box<BufferedBlock>>,
    _0: PhantomData<R>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/format/aloe_BufferingAudioFormatReader.cpp]
impl<'a, R: Read> Drop for BufferingAudioReader<'a, R> {

    fn drop(&mut self) {
        todo!();
        /*
            thread.removeTimeSliceClient (this);
        */
    }
}

impl<'a, R: Read> BufferingAudioReader<'a, R> {

    /**
      | Creates a reader.
      | 
      | -----------
      | @param sourceReader
      | 
      | the source reader to wrap. This BufferingAudioReader
      | takes ownership of this object and will
      | delete it later when no longer needed
      | ----------
      | @param timeSliceThread
      | 
      | the thread that should be used to do the
      | background reading.
      | 
      | Make sure that the thread you supply
      | is running, and won't be deleted while
      | the reader object still exists.
      | ----------
      | @param samplesToBuffer
      | 
      | the total number of samples to buffer
      | ahead.
      |
      */
    pub fn new(
        source_reader:     *mut AudioFormatReader<'a>,
        time_slice_thread: &mut TimeSliceThread,
        samples_to_buffer: i32

    ) -> Self {
    
        todo!();
        /*

            : AudioFormatReader (nullptr, sourceReader->getFormatName()),
          source (sourceReader), thread (timeSliceThread),
          numBlocks (1 + (samplesToBuffer / samplesPerBlock))

        sampleRate            = source->sampleRate;
        lengthInSamples       = source->lengthInSamples;
        numChannels           = source->numChannels;
        metadataValues        = source->metadataValues;
        bitsPerSample         = 32;
        usesFloatingPointData = true;

        timeSliceThread.addTimeSliceClient (this);
        */
    }
    
    /**
      | Sets a number of milliseconds that the
      | reader can block for in its readSamples()
      | method before giving up and returning
      | silence.
      | 
      | A value of less that 0 means "wait forever".
      | The default timeout is 0.
      |
      */
    pub fn set_read_timeout(&mut self, timeout_milliseconds: i32)  {
        
        todo!();
        /*
            timeoutMs = timeoutMilliseconds;
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
            auto startTime = Time::getMillisecondCounter();
        clearSamplesBeyondAvailableLength (destSamples, numDestChannels, startOffsetInDestBuffer,
                                           startSampleInFile, numSamples, lengthInSamples);

        const ScopedLock sl (lock);
        nextReadPosition = startSampleInFile;

        while (numSamples > 0)
        {
            if (auto block = getBlockContaining (startSampleInFile))
            {
                auto offset = (int) (startSampleInFile - block->range.getStart());
                auto numToDo = jmin (numSamples, (int) (block->range.getEnd() - startSampleInFile));

                for (int j = 0; j < numDestChannels; ++j)
                {
                    if (auto* dest = (float*) destSamples[j])
                    {
                        dest += startOffsetInDestBuffer;

                        if (j < (int) numChannels)
                            FloatVectorOperations::copy (dest, block->buffer.getReadPointer (j, offset), numToDo);
                        else
                            FloatVectorOperations::clear (dest, numToDo);
                    }
                }

                startOffsetInDestBuffer += numToDo;
                startSampleInFile += numToDo;
                numSamples -= numToDo;
            }
            else
            {
                if (timeoutMs >= 0 && Time::getMillisecondCounter() >= startTime + (uint32) timeoutMs)
                {
                    for (int j = 0; j < numDestChannels; ++j)
                        if (auto* dest = (float*) destSamples[j])
                            FloatVectorOperations::clear (dest + startOffsetInDestBuffer, numSamples);

                    break;
                }
                else
                {
                    ScopedUnlock ul (lock);
                    Thread::yield();
                }
            }
        }

        return true;
        */
    }
    
    pub fn get_block_containing(&self, pos: i64) -> *mut BufferedBlock {
        
        todo!();
        /*
            for (auto* b : blocks)
            if (b->range.contains (pos))
                return b;

        return nullptr;
        */
    }
    
    pub fn use_time_slice(&mut self) -> i32 {
        
        todo!();
        /*
            return readNextBufferChunk() ? 1 : 100;
        */
    }
    
    pub fn read_next_buffer_chunk(&mut self) -> bool {
        
        todo!();
        /*
            auto pos = (nextReadPosition.load() / samplesPerBlock) * samplesPerBlock;
        auto endPos = jmin (lengthInSamples, pos + numBlocks * samplesPerBlock);

        Vec<Box<BufferedBlock>> newBlocks;

        for (int i = blocks.size(); --i >= 0;)
            if (blocks.getUnchecked (i)->range.intersects (Range<int64> (pos, endPos)))
                newBlocks.add (blocks.getUnchecked (i));

        if (newBlocks.size() == numBlocks)
        {
            newBlocks.clear (false);
            return false;
        }

        for (auto p = pos; p < endPos; p += samplesPerBlock)
        {
            if (getBlockContaining (p) == nullptr)
            {
                newBlocks.add (new BufferedBlock (*source, p, samplesPerBlock));
                break; // just do one block
            }
        }

        {
            const ScopedLock sl (lock);
            newBlocks.swapWith (blocks);
        }

        for (int i = blocks.size(); --i >= 0;)
            newBlocks.removeObject (blocks.getUnchecked (i), false);

        return true;
        */
    }
}
