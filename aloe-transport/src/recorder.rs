crate::ix!();

/** 
  | A simple class that acts as an
  | AudioIODeviceCallback and writes the incoming
  | audio data to a WAV file.
  */
pub struct AudioRecorder<'a> {

    thumbnail:         &'a mut AudioThumbnail<'a>,

    /**
      | the thread that will write our audio
      | data to disk
      |
      */
    background_thread: TimeSliceThread, // { "Audio Recorder Thread" }; 

    /**
      | the FIFO used to buffer the incoming
      | data
      |
      */
    threaded_writer:   Box<ThreadedWriter<'a, Box<dyn Write>>>,

    sample_rate:       f64, // default = 0.0
    next_sample_num:   i64, // default = 0
    writer_lock:       CriticalSection,
    active_writer:     Atomic<*mut ThreadedWriter<'a, Box<dyn Write>>>, // default = nullptr 
}

impl<'a> AudioIODeviceCallback for AudioRecorder<'a> {

    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            sampleRate = device->getCurrentSampleRate();
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            sampleRate = 0;
        */
    }
    
    fn audio_device_io_callback(&mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32)  {
        
        todo!();
        /*
            const ScopedLock sl (writerLock);

            if (activeWriter.load() != nullptr && numInputChannels >= thumbnail.getNumChannels())
            {
                activeWriter.load()->write (inputChannelData, numSamples);

                // Create an AudioBuffer to wrap our incoming data, note that this does no allocations or copies, it simply references our input data
                AudioBuffer<float> buffer (const_cast<float**> (inputChannelData), thumbnail.getNumChannels(), numSamples);
                thumbnail.addBlock (nextSampleNum, buffer, 0, numSamples);
                nextSampleNum += numSamples;
            }

            // We need to clear the output buffers, in case they're full of junk..
            for (int i = 0; i < numOutputChannels; ++i)
                if (outputChannelData[i] != nullptr)
                    FloatVectorOperations::clear (outputChannelData[i], numSamples);
        */
    }
}

impl<'a> Drop for AudioRecorder<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            stop();
         */
    }
}

impl<'a> AudioRecorder<'a> {

    pub fn new(thumbnail_to_update: &mut AudioThumbnail) -> Self {
    
        todo!();
        /*
        : thumbnail(thumbnailToUpdate),

            backgroundThread.startThread();
        */
    }
    
    pub fn start_recording(&mut self, file: &File)  {
        
        todo!();
        /*
            stop();

            if (sampleRate > 0)
            {
                // Create an OutputStream to write to our destination file...
                file.deleteFile();

                if (auto fileStream = std::unique_ptr<FileOutputStream> (file.createOutputStream()))
                {
                    // Now create a WAV writer object that writes to our output stream...
                    WavAudioFormat wavFormat;

                    if (auto writer = wavFormat.createWriterFor (fileStream.get(), sampleRate, 1, 16, {}, 0))
                    {
                        fileStream.release(); // (passes responsibility for deleting the stream to the writer object that is now using it)

                        // Now we'll create one of these helper objects which will act as a FIFO buffer, and will
                        // write the data to disk on our background thread.
                        threadedWriter.reset (new AudioFormatWriter::ThreadedWriter (writer, backgroundThread, 32768));

                        // Reset our recording thumbnail
                        thumbnail.reset (writer->getNumChannels(), writer->getSampleRate());
                        nextSampleNum = 0;

                        // And now, swap over our active writer pointer so that the audio callback will start using it..
                        const ScopedLock sl (writerLock);
                        activeWriter = threadedWriter.get();
                    }
                }
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            // First, clear this pointer to stop the audio callback from using our writer object..
            {
                const ScopedLock sl (writerLock);
                activeWriter = nullptr;
            }

            // Now we can delete the writer object. It's done in this order because the deletion could
            // take a little time while remaining data gets flushed to disk, so it's best to avoid blocking
            // the audio callback while this happens.
            threadedWriter.reset();
        */
    }
    
    pub fn is_recording(&self) -> bool {
        
        todo!();
        /*
            return activeWriter.load() != nullptr;
        */
    }
    
}
