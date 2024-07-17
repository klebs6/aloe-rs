crate::ix!();

pub type Vst3BufferExchangeBus<FloatType: num::Float>    = Vec<*mut FloatType>;
pub type Vst3BufferExchangeBusMap<FloatType: num::Float> = Vec<Vst3BufferExchangeBus<FloatType>>;

pub struct Vst3BufferExchange<FloatType: num::Float> {
    _p0: PhantomData<FloatType>,
}

impl<FloatType: num::Float> Vst3BufferExchange<FloatType> {

    pub fn assign_raw_pointer(
        vst_buffers: &mut AudioBusBuffers,
        raw:         *mut *mut f32
    ) {
        
        todo!();
        /*
            vstBuffers.channelBuffers32 = raw;
        */
    }
    
    pub fn assign_raw_pointer_f64(
        vst_buffers: &mut AudioBusBuffers,
        raw:         *mut *mut f64

    ) {
        
        todo!();
        /*
            vstBuffers.channelBuffers64 = raw;
        */
    }

    /**
      | Assigns a series of AudioBuffer's channels
      | to an AudioBusBuffers'
      | 
      | -----------
      | @warning
      | 
      | For speed, does not check the channel
      | count and offsets according to the AudioBuffer
      |
      */
    pub fn associate_buffer_to(
        vst_buffers:          &mut AudioBusBuffers,
        bus:                  &mut Vst3BufferExchangeBus<FloatType>,
        buffer:               &mut AudioBuffer<FloatType>,
        num_channels:         i32,
        channel_start_offset: i32,
        sample_offset:        Option<i32>

    )  {

        let sample_offset: i32 = sample_offset.unwrap_or(0);

        todo!();
        /*
            const int channelEnd = numChannels + channelStartOffset;
            jassert (channelEnd >= 0 && channelEnd <= buffer.getNumChannels());

            bus.clearQuick();

            for (int i = channelStartOffset; i < channelEnd; ++i)
                bus.add (buffer.getWritePointer (i, sampleOffset));

            assignRawPointer (vstBuffers, (numChannels > 0 ? bus.getRawDataPointer() : nullptr));
            vstBuffers.numChannels      = numChannels;
            vstBuffers.silenceFlags     = 0;
        */
    }
    
    pub fn map_arrangement_to_buses(
        channel_index_offset: &mut i32,
        index:                i32,
        result:               &mut Vec<AudioBusBuffers>,
        bus_map_to_use:       &mut Vst3BufferExchangeBusMap<FloatType>,
        arrangement:          &AudioChannelSet,
        source:               &mut AudioBuffer<FloatType>)  {
        
        todo!();
        /*
            const int numChansForBus = arrangement.size();

            if (index >= result.size())
                result.add (typename VstAudioBusBuffers());

            if (index >= busMapToUse.size())
                busMapToUse.add (Vst3BufferExchangeBus());

            associateBufferTo (result.getReference (index),
                               busMapToUse.getReference (index),
                               source, numChansForBus, channelIndexOffset);

            channelIndexOffset += numChansForBus;
        */
    }
    
    pub fn map_buffer_to_buses_with_arrangements(
        result:         &mut Vec<AudioBusBuffers>,
        bus_map_to_use: &mut Vst3BufferExchangeBusMap<FloatType>,
        arrangements:   &[AudioChannelSet],
        source:         &mut AudioBuffer<FloatType>
    ) {
        
        todo!();
        /*
            int channelIndexOffset = 0;

            for (int i = 0; i < arrangements.size(); ++i)
                mapArrangementToBuses (channelIndexOffset, i, result, busMapToUse,
                                        arrangements.getUnchecked (i), source);
        */
    }
    
    pub fn map_buffer_to_buses(
        result:         &mut Vec<AudioBusBuffers>,
        processor:      &mut dyn AudioProcessorInterface,
        bus_map_to_use: &mut Vst3BufferExchangeBusMap<FloatType>,
        is_input:       bool,
        num_buses:      i32,
        source:         &mut AudioBuffer<FloatType>)  {
        
        todo!();
        /*
            int channelIndexOffset = 0;

            for (int i = 0; i < numBuses; ++i)
                mapArrangementToBuses (channelIndexOffset, i,
                                        result, busMapToUse,
                                        getArrangementForBus (&processor, isInput, i),
                                        source);
        */
    }
}
