crate::ix!();

pub struct ChannelRemapper<'a> {
    processor:                     &'a mut AudioProcessor<'a>,
    input_layout_map_ptr_storage:  HeapBlock<*mut i32>,
    output_layout_map_ptr_storage: HeapBlock<*mut i32>,
    input_layout_map_storage:      HeapBlock<i32>,
    output_layout_map_storage:     HeapBlock<i32>,
    input_layout_map:              *mut *mut i32,
    output_layout_map:             *mut *mut i32,
}

impl<'a> ChannelRemapper<'a> {

    pub fn new(p: &mut AudioProcessor) -> Self {
    
        todo!();
        /*
        : processor(p),
        : input_layout_map(nullptr),
        : output_layout_map(nullptr),

        
        */
    }
    
    pub fn alloc(&mut self)  {
        
        todo!();
        /*
            const int numInputBuses  = AudioUnitHelpers::getBusCount (processor, true);
                const int numOutputBuses = AudioUnitHelpers::getBusCount (processor, false);

                initializeChannelMapArray (true, numInputBuses);
                initializeChannelMapArray (false, numOutputBuses);

                for (int busIdx = 0; busIdx < numInputBuses; ++busIdx)
                    fillLayoutChannelMaps (true, busIdx);

                for (int busIdx = 0; busIdx < numOutputBuses; ++busIdx)
                    fillLayoutChannelMaps (false, busIdx);
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            inputLayoutMap = outputLayoutMap = nullptr;
                inputLayoutMapPtrStorage.free();
                outputLayoutMapPtrStorage.free();
                inputLayoutMapStorage.free();
                outputLayoutMapStorage.free();
        */
    }
    
    #[inline] pub fn get(&self, 
        input: bool,
        bus:   i32) -> *const i32 {
        
        todo!();
        /*
            return (input ? inputLayoutMap : outputLayoutMap) [bus];
        */
    }
    
    pub fn initialize_channel_map_array(&mut self, 
        is_input:  bool,
        num_buses: i32)  {
        
        todo!();
        /*
            HeapBlock<int*>& layoutMapPtrStorage = isInput ? inputLayoutMapPtrStorage : outputLayoutMapPtrStorage;
                HeapBlock<int>& layoutMapStorage = isInput ? inputLayoutMapStorage : outputLayoutMapStorage;
                int**& layoutMap = isInput ? inputLayoutMap : outputLayoutMap;

                const int totalInChannels  = processor.getTotalNumInputChannels();
                const int totalOutChannels = processor.getTotalNumOutputChannels();

                layoutMapPtrStorage.calloc (static_cast<size_t> (numBuses));
                layoutMapStorage.calloc (static_cast<size_t> (isInput ? totalInChannels : totalOutChannels));

                layoutMap  = layoutMapPtrStorage. get();

                int ch = 0;
                for (int busIdx = 0; busIdx < numBuses; ++busIdx)
                {
                    layoutMap[busIdx] = layoutMapStorage.get() + ch;
                    ch += processor.getChannelCountOfBus (isInput, busIdx);
                }
        */
    }
    
    pub fn fill_layout_channel_maps(&mut self, 
        is_input: bool,
        bus_nr:   i32)  {
        
        todo!();
        /*
            int* layoutMap = (isInput ? inputLayoutMap : outputLayoutMap)[busNr];
                auto channelFormat = processor.getChannelLayoutOfBus (isInput, busNr);
                AudioChannelLayout coreAudioLayout;

                zerostruct (coreAudioLayout);
                coreAudioLayout.mChannelLayoutTag = CoreAudioLayouts::toCoreAudio (channelFormat);

                const int numChannels = channelFormat.size();
                auto coreAudioChannels = CoreAudioLayouts::getCoreAudioLayoutChannels (coreAudioLayout);

                for (int i = 0; i < numChannels; ++i)
                    layoutMap[i] = coreAudioChannels.indexOf (channelFormat.getTypeOfChannel (i));
        */
    }
}
