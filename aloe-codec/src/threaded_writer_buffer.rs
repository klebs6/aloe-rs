crate::ix!();

///--------------------------
#[no_copy]
pub struct ThreadedWriterBuffer<'a, W: Write> {
    base:                 TimeSliceClient,
    fifo:                 AbstractFifo,
    buffer:               AudioBuffer<f32>,
    time_slice_thread:    &'a mut TimeSliceThread,
    writer:               Box<AudioFormatWriter<'a, W>>,
    thumbnail_lock:       CriticalSection,
    receiver:             *mut dyn IncomingDataReceiver, // default = {}
    samples_written:      i64, // default = 0
    samples_per_flush:    i32, // default = 0
    flush_sample_counter: i32, // default = 0
    is_running:           AtomicBool, // default = { true  }
}

impl<'a, W: Write> Drop for ThreadedWriterBuffer<'a, W> {

    fn drop(&mut self) {
        todo!();
        /*
            isRunning = false;
            timeSliceThread.removeTimeSliceClient (this);

            while (writePendingData() == 0)
            {}
        */
    }
}

impl<'a, W: Write> ThreadedWriterBuffer<'a, W> {

    pub fn new(
        tst:         &mut TimeSliceThread,
        w:           *mut AudioFormatWriter<'a, W>,
        channels:    i32,
        num_samples: i32

    ) -> Self {
    
        todo!();
        /*
        : fifo(numSamples),
        : buffer(channels, numSamples),
        : time_slice_thread(tst),
        : writer(w),

            timeSliceThread.addTimeSliceClient (this);
        */
    }
    
    pub fn write(&mut self, 
        data:        *const *const f32,
        num_samples: i32) -> bool {
        
        todo!();
        /*
            if (numSamples <= 0 || ! isRunning)
                return true;

            jassert (timeSliceThread.isThreadRunning());  // you need to get your thread running before pumping data into this!

            int start1, size1, start2, size2;
            fifo.prepareToWrite (numSamples, start1, size1, start2, size2);

            if (size1 + size2 < numSamples)
                return false;

            for (int i = buffer.getNumChannels(); --i >= 0;)
            {
                buffer.copyFrom (i, start1, data[i], size1);
                buffer.copyFrom (i, start2, data[i] + size1, size2);
            }

            fifo.finishedWrite (size1 + size2);
            timeSliceThread.notify();
            return true;
        */
    }
    
    pub fn use_time_slice(&mut self) -> i32 {
        
        todo!();
        /*
            return writePendingData();
        */
    }
    
    pub fn write_pending_data(&mut self) -> i32 {
        
        todo!();
        /*
            auto numToDo = fifo.getTotalSize() / 4;

            int start1, size1, start2, size2;
            fifo.prepareToRead (numToDo, start1, size1, start2, size2);

            if (size1 <= 0)
                return 10;

            writer->writeFromAudioSampleBuffer (buffer, start1, size1);

            const ScopedLock sl (thumbnailLock);

            if (receiver != nullptr)
                receiver->addBlock (samplesWritten, buffer, start1, size1);

            samplesWritten += size1;

            if (size2 > 0)
            {
                writer->writeFromAudioSampleBuffer (buffer, start2, size2);

                if (receiver != nullptr)
                    receiver->addBlock (samplesWritten, buffer, start2, size2);

                samplesWritten += size2;
            }

            fifo.finishedRead (size1 + size2);

            if (samplesPerFlush > 0)
            {
                flushSampleCounter -= size1 + size2;

                if (flushSampleCounter <= 0)
                {
                    flushSampleCounter = samplesPerFlush;
                    writer->flush();
                }
            }

            return 0;
        */
    }
    
    pub fn set_data_receiver(&mut self, new_receiver: *mut dyn IncomingDataReceiver)  {
        
        todo!();
        /*
            if (newReceiver != nullptr)
                newReceiver->reset (buffer.getNumChannels(), writer->getSampleRate(), 0);

            const ScopedLock sl (thumbnailLock);
            receiver = newReceiver;
            samplesWritten = 0;
        */
    }
    
    pub fn set_flush_interval(&mut self, num_samples: i32)  {
        
        todo!();
        /*
            samplesPerFlush = numSamples;
        */
    }
}
