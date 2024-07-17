crate::ix!();

/**
  | Sets up `channels` so that it contains
  | channel pointers suitable for passing
  | to an AudioProcessor's processBlock.
  | 
  | On return, `channels` will hold `max
  | (processorIns, processorOuts)` entries.
  | The first `processorIns` entries will
  | point to buffers holding input data.
  | Any entries after the first `processorIns`
  | entries will point to zeroed buffers.
  | 
  | In the case that the system only provides
  | a single input channel, but the processor
  | has been initialised with multiple
  | input channels, the system input will
  | be copied to all processor inputs.
  | 
  | In the case that the system provides
  | no input channels, but the processor
  | has been initialise with multiple input
  | channels, the processor's input channels
  | will all be zeroed.
  | 
  | -----------
  | @param ins
  | 
  | the system inputs.
  | ----------
  | @param outs
  | 
  | the system outputs.
  | ----------
  | @param numSamples
  | 
  | the number of samples in the system buffers.
  | ----------
  | @param processorIns
  | 
  | the number of input channels requested
  | by the processor.
  | ----------
  | @param processorOuts
  | 
  | the number of output channels requested
  | by the processor.
  | ----------
  | @param tempBuffer
  | 
  | temporary storage for inputs that don't
  | have a corresponding output.
  | ----------
  | @param channels
  | 
  | holds pointers to each of the processor's
  | audio channels.
  |
  */
pub fn initialise_io_buffers(
        ins:            ChannelInfo<f32>,
        outs:           ChannelInfo<f32>,
        num_samples:    i32,
        processor_ins:  i32,
        processor_outs: i32,
        temp_buffer:    &mut AudioBuffer<f32>,
        channels:       &mut Vec<*mut f32>)  {
    
    todo!();
    /*
        jassert ((int) channels.size() >= jmax (processorIns, processorOuts));

        size_t totalNumChans = 0;
        const auto numBytes = (size_t) numSamples * sizeof (float);

        const auto prepareInputChannel = [&] (int index)
        {
            if (ins.numChannels == 0)
                zeromem (channels[totalNumChans], numBytes);
            else
                memcpy (channels[totalNumChans], ins.data[index % ins.numChannels], numBytes);
        };

        if (processorIns > processorOuts)
        {
            // If there aren't enough output channels for the number of
            // inputs, we need to use some temporary extra ones (can't
            // use the input data in case it gets written to).
            jassert (tempBuffer.getNumChannels() >= processorIns - processorOuts);
            jassert (tempBuffer.getNumSamples() >= numSamples);

            for (int i = 0; i < processorOuts; ++i)
            {
                channels[totalNumChans] = outs.data[i];
                prepareInputChannel (i);
                ++totalNumChans;
            }

            for (auto i = processorOuts; i < processorIns; ++i)
            {
                channels[totalNumChans] = tempBuffer.getWritePointer (i - outs.numChannels);
                prepareInputChannel (i);
                ++totalNumChans;
            }
        }
        else
        {
            for (int i = 0; i < processorIns; ++i)
            {
                channels[totalNumChans] = outs.data[i];
                prepareInputChannel (i);
                ++totalNumChans;
            }

            for (auto i = processorIns; i < processorOuts; ++i)
            {
                channels[totalNumChans] = outs.data[i];
                zeromem (channels[totalNumChans], (size_t) numSamples * sizeof (float));
                ++totalNumChans;
            }
        }
    */
}
