crate::ix!();

pub struct MultichannelEngine {
    head:          Vec<Box<ConvolutionEngine>>,
    tail:          Vec<Box<ConvolutionEngine>>,
    tail_buffer:   AudioBuffer<f32>,
    latency:       i32,
    ir_size:       i32,
    block_size:    i32,
    is_zero_delay: bool,
}

impl MultichannelEngine {

    pub fn new(
        buf:              &AudioBuffer<f32>,
        max_block_size:   i32,
        max_buffer_size:  i32,
        head_size_in:     ConvolutionNonUniform,
        is_zero_delay_in: bool

    ) -> Self {
    
        todo!();
        /*


            : tailBuffer (1, maxBlockSize),
              latency (isZeroDelayIn ? 0 : maxBufferSize),
              irSize (buf.getNumSamples()),
              blockSize (maxBlockSize),
              isZeroDelay (isZeroDelayIn)

            constexpr auto numChannels = 2;

            const auto makeEngine = [&] (int channel, int offset, int length, uint32 thisBlockSize)
            {
                return std::make_unique<ConvolutionEngine> (buf.getReadPointer (jmin (buf.getNumChannels() - 1, channel), offset),
                                                            length,
                                                            static_cast<size_t> (thisBlockSize));
            };

            if (headSizeIn.headSizeInSamples == 0)
            {
                for (int i = 0; i < numChannels; ++i)
                    head.emplace_back (makeEngine (i, 0, buf.getNumSamples(), static_cast<uint32> (maxBufferSize)));
            }
            else
            {
                const auto size = jmin (buf.getNumSamples(), headSizeIn.headSizeInSamples);

                for (int i = 0; i < numChannels; ++i)
                    head.emplace_back (makeEngine (i, 0, size, static_cast<uint32> (maxBufferSize)));

                const auto tailBufferSize = static_cast<uint32> (headSizeIn.headSizeInSamples + (isZeroDelay ? 0 : maxBufferSize));

                if (size != buf.getNumSamples())
                    for (int i = 0; i < numChannels; ++i)
                        tail.emplace_back (makeEngine (i, size, buf.getNumSamples() - size, tailBufferSize));
            }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (const auto& e : head)
                e->reset();

            for (const auto& e : tail)
                e->reset();
        */
    }
    
    pub fn process_samples(&mut self, 
        input:  &AudioBlock<f32>,
        output: &mut AudioBlock<f32>)  {
        
        todo!();
        /*
            const auto numChannels = jmin (head.size(), input.getNumChannels(), output.getNumChannels());
            const auto numSamples  = jmin (input.getNumSamples(), output.getNumSamples());

            const AudioBlock<float> fullTailBlock (tailBuffer);
            const auto tailBlock = fullTailBlock.getSubBlock (0, (size_t) numSamples);

            const auto isUniform = tail.empty();

            for (size_t channel = 0; channel < numChannels; ++channel)
            {
                if (! isUniform)
                    tail[channel]->processSamplesWithAddedLatency (input.getChannelPointer (channel),
                                                                   tailBlock.getChannelPointer (0),
                                                                   numSamples);

                if (isZeroDelay)
                    head[channel]->processSamples (input.getChannelPointer (channel),
                                                   output.getChannelPointer (channel),
                                                   numSamples);
                else
                    head[channel]->processSamplesWithAddedLatency (input.getChannelPointer (channel),
                                                                   output.getChannelPointer (channel),
                                                                   numSamples);

                if (! isUniform)
                    output.getSingleChannelBlock (channel) += tailBlock;
            }

            const auto numOutputChannels = output.getNumChannels();

            for (auto i = numChannels; i < numOutputChannels; ++i)
                output.getSingleChannelBlock (i).copyFrom (output.getSingleChannelBlock (0));
        */
    }
    
    pub fn get_ir_size(&self) -> i32 {
        
        todo!();
        /*
            return irSize;
        */
    }
    
    pub fn get_latency(&self) -> i32 {
        
        todo!();
        /*
            return latency;
        */
    }
    
    pub fn get_block_size(&self) -> i32 {
        
        todo!();
        /*
            return blockSize;
        */
    }
}
