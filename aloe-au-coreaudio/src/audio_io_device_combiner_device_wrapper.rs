crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AudioIODeviceCombinerDeviceWrapper<'a> {
    owner:                &'a mut AudioIODeviceCombiner<'a>,
    device:               Box<CoreAudioIODevice<'a>>,
    input_index:          i32, // default = 0
    num_input_chans:      i32, // default = 0
    output_index:         i32, // default = 0
    num_output_chans:     i32, // default = 0
    use_inputs:           bool, // default = false
    use_outputs:          bool, // default = false
    is_waiting_for_input: AtomicBool, // default = false 
    input_fifo:           AbstractFifo, // default = 32 
    output_fifo:          AbstractFifo, // default = 32 
    done:                 bool, // default = false
}

impl<'a> AudioIODeviceCallback for AudioIODeviceCombinerDeviceWrapper<'a> {

    fn audio_device_about_to_start(&mut self, d: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            owner.handleAudioDeviceAboutToStart (d);
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            owner.handleAudioDeviceStopped();
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
            if (numInputChannels > 0)
                    {
                        isWaitingForInput = false;

                        int start1, size1, start2, size2;
                        inputFifo.prepareToWrite (numSamples, start1, size1, start2, size2);

                        if (size1 + size2 < numSamples)
                        {
                            inputFifo.reset();
                            inputFifo.prepareToWrite (numSamples, start1, size1, start2, size2);
                        }

                        for (int i = 0; i < numInputChannels; ++i)
                        {
                            auto dest = owner.fifoWritePointers[inputIndex + i];
                            auto src = inputChannelData[i];

                            if (size1 > 0)  FloatVectorOperations::copy (dest + start1, src,         size1);
                            if (size2 > 0)  FloatVectorOperations::copy (dest + start2, src + size1, size2);
                        }

                        auto totalSize = size1 + size2;
                        inputFifo.finishedWrite (totalSize);

                        if (numSamples > totalSize)
                        {
                            auto samplesRemaining = numSamples - totalSize;

                            for (int i = 0; i < numInputChans; ++i)
                                FloatVectorOperations::clear (owner.fifoWritePointers[inputIndex + i] + totalSize, samplesRemaining);

                            owner.underrun();
                        }
                    }

                    if (numOutputChannels > 0)
                    {
                        int start1, size1, start2, size2;
                        outputFifo.prepareToRead (numSamples, start1, size1, start2, size2);

                        if (size1 + size2 < numSamples)
                        {
                            Thread::sleep (1);
                            outputFifo.prepareToRead (numSamples, start1, size1, start2, size2);
                        }

                        for (int i = 0; i < numOutputChannels; ++i)
                        {
                            auto dest = outputChannelData[i];
                            auto src = owner.fifoReadPointers[outputIndex + i];

                            if (size1 > 0)  FloatVectorOperations::copy (dest,         src + start1, size1);
                            if (size2 > 0)  FloatVectorOperations::copy (dest + size1, src + start2, size2);
                        }

                        auto totalSize = size1 + size2;
                        outputFifo.finishedRead (totalSize);

                        if (numSamples > totalSize)
                        {
                            auto samplesRemaining = numSamples - totalSize;

                            for (int i = 0; i < numOutputChannels; ++i)
                                FloatVectorOperations::clear (outputChannelData[i] + totalSize, samplesRemaining);

                            owner.underrun();
                        }
                    }

                    owner.notify();
        */
    }
}

impl<'a> Drop for AudioIODeviceCombinerDeviceWrapper<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                    close();
                 */
    }
}

impl<'a> AudioIODeviceCombinerDeviceWrapper<'a> {

    pub fn new(
        cd:       &mut AudioIODeviceCombiner,
        d:        *mut CoreAudioIODevice,
        use_ins:  bool,
        use_outs: bool) -> Self {
    
        todo!();
        /*


            : owner (cd), device (d),
                      useInputs (useIns), useOutputs (useOuts)

                    d->setDeviceWrapperRestartCallback ([this] { owner.restartAsync(); });
        */
    }
    
    pub fn open(&mut self, 
        input_channels:  &BigInteger,
        output_channels: &BigInteger,
        sample_rate:     f64,
        buffer_size:     i32,
        channel_index:   i32,
        fifo_size:       i32) -> String {
        
        todo!();
        /*
            inputFifo.setTotalSize (fifoSize);
                    outputFifo.setTotalSize (fifoSize);
                    inputFifo.reset();
                    outputFifo.reset();

                    auto err = device->open (useInputs  ? inputChannels  : BigInteger(),
                                             useOutputs ? outputChannels : BigInteger(),
                                             sampleRate, bufferSize);

                    numInputChans  = useInputs  ? device->getActiveInputChannels().countNumberOfSetBits()  : 0;
                    numOutputChans = useOutputs ? device->getActiveOutputChannels().countNumberOfSetBits() : 0;

                    isWaitingForInput = numInputChans > 0;

                    inputIndex = channelIndex;
                    outputIndex = channelIndex + numInputChans;

                    return err;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            device->close();
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            reset();
                    device->start (this);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            inputFifo.reset();
                    outputFifo.reset();
        */
    }
    
    pub fn get_output_channel_names(&self) -> StringArray {
        
        todo!();
        /*
            return useOutputs ? device->getOutputChannelNames() : StringArray();
        */
    }
    
    pub fn get_input_channel_names(&self) -> StringArray {
        
        todo!();
        /*
            return useInputs  ? device->getInputChannelNames()  : StringArray();
        */
    }
    
    pub fn is_input_ready(&self, num_samples: i32) -> bool {
        
        todo!();
        /*
            return numInputChans == 0 || inputFifo.getNumReady() >= numSamples;
        */
    }
    
    pub fn read_input(&mut self, 
        dest_buffer: &mut AudioBuffer<f32>,
        num_samples: i32)  {
        
        todo!();
        /*
            if (numInputChans == 0)
                        return;

                    int start1, size1, start2, size2;
                    inputFifo.prepareToRead (numSamples, start1, size1, start2, size2);

                    for (int i = 0; i < numInputChans; ++i)
                    {
                        auto index = inputIndex + i;
                        auto dest = destBuffer.getWritePointer (index);
                        auto src = owner.fifoReadPointers[index];

                        if (size1 > 0)  FloatVectorOperations::copy (dest,         src + start1, size1);
                        if (size2 > 0)  FloatVectorOperations::copy (dest + size1, src + start2, size2);
                    }

                    inputFifo.finishedRead (size1 + size2);
        */
    }
    
    pub fn is_output_ready(&self, num_samples: i32) -> bool {
        
        todo!();
        /*
            return numOutputChans == 0 || outputFifo.getFreeSpace() >= numSamples;
        */
    }
    
    pub fn push_output_data(&mut self, 
        src_buffer:  &mut AudioBuffer<f32>,
        num_samples: i32)  {
        
        todo!();
        /*
            if (numOutputChans == 0)
                        return;

                    int start1, size1, start2, size2;
                    outputFifo.prepareToWrite (numSamples, start1, size1, start2, size2);

                    for (int i = 0; i < numOutputChans; ++i)
                    {
                        auto index = outputIndex + i;
                        auto dest = owner.fifoWritePointers[index];
                        auto src = srcBuffer.getReadPointer (index);

                        if (size1 > 0)  FloatVectorOperations::copy (dest + start1, src,         size1);
                        if (size2 > 0)  FloatVectorOperations::copy (dest + start2, src + size1, size2);
                    }

                    outputFifo.finishedWrite (size1 + size2);
        */
    }
    
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return device->getCurrentSampleRate();
        */
    }
    
    pub fn set_current_sample_rate(&mut self, new_sample_rate: f64) -> bool {
        
        todo!();
        /*
            return device->setCurrentSampleRate (newSampleRate);
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return device->getCurrentBufferSizeSamples();
        */
    }
    
    pub fn audio_device_error(&mut self, error_message: &String)  {
        
        todo!();
        /*
            owner.handleAudioDeviceError (errorMessage);
        */
    }
}
