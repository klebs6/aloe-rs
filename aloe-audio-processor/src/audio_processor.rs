crate::ix!();

lazy_static!{
    /*
    static ThreadLocalValue<AudioProcessor::AudioProcessorWrapperType> wrapperTypeBeingCreated;
    */
}

/**
  | magic number to identify memory blocks
  | that we've stored as XML
  |
  */
pub const magic_xml_number: u32 = 0x21324356;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessor.h]

/**
  | Base class for audio processing classes
  | or plugins.
  | 
  | This is intended to act as a base class
  | of audio processor that is general enough
  | to be wrapped as a Vst, AU, RTAS, etc,
  | or used internally.
  | 
  | It is also used by the plugin hosting
  | code as the wrapper around an instance
  | of a loaded plugin.
  | 
  | You should derive your own class from
  | this base class, and if you're building
  | a plugin, you should implement a global
  | function called createPluginFilter()
  | which creates and returns a new instance
  | of your subclass.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessor<'a> {

    /**
      | When loaded by a plugin wrapper, this
      | flag will be set to indicate the type
      | of plugin within which the processor
      | is running.
      |
      */
    wrapper_type:                     AudioProcessorWrapperType,

    play_head:                        Atomic<*mut dyn AudioPlayHeadInterface>, // default = { nullptr  }
    listeners:                        Vec<*mut dyn AudioProcessorListener>,
    active_editor:                    ComponentSafePointer<'a, AudioProcessorEditor<'a>>,
    current_sample_rate:              f64, // default = 0
    block_size:                       i32, // default = 0
    latency_samples:                  i32, // default = 0
    suspended:                        bool, // default = false
    non_realtime:                     AtomicBool, // default = { false  }
    processing_precision:             AudioProcessorProcessingPrecision, // default = singlePrecision
    callback_lock:                    CriticalSection,
    listener_lock:                    CriticalSection,
    active_editor_lock:               CriticalSection,
    input_buses:                      RefCell<Vec<Box<AudioProcessorBus<'a>>>>,
    output_buses:                     RefCell<Vec<Box<AudioProcessorBus<'a>>>>,
    cached_input_speaker_arr_string:  String,
    cached_output_speaker_arr_string: String,
    cached_total_ins:                 i32, // default = 0
    cached_total_outs:                i32, // default = 0
    parameter_tree:                   AudioProcessorParameterGroup,
    flat_parameter_list:              Vec<*mut AudioProcessorParameter>,

    #[cfg(ALOE_DEBUG)]
    #[cfg(not(ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING))]
    changing_params:      BigInteger,

    #[cfg(ALOE_DEBUG)]
    text_recursion_check: bool, // default = false

    #[cfg(ALOE_DEBUG)]
    param_ids:            HashSet<String>,

    #[cfg(ALOE_DEBUG)]
    group_ids:            HashSet<String>,

    #[cfg(ALOE_DEBUG)]
    #[cfg(not(ALOE_DISABLE_CAUTIOUS_PARAMETER_ID_CHECKING))]
    trimmed_param_ids:    HashSet<String>,
}

impl<'a> Default for AudioProcessor<'a> {
    
    /**
      | Constructor.
      | 
      | This constructor will create a main
      | input and output bus which are disabled
      | by default. If you need more fine-grained
      | control then use the other constructors.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*

            : AudioProcessor (AudioProcessorBusesProperties().withInput  ("Input",  AudioChannelSet::stereo(), false)
                                           .withOutput ("Output", AudioChannelSet::stereo(), false))
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessor.cpp]
impl<'a> Drop for AudioProcessor<'a> {

    fn drop(&mut self) {

        todo!();

        /*
        {
            const ScopedLock sl (activeEditorLock);

            // ooh, nasty - the editor should have been deleted before its AudioProcessor.
            jassert (activeEditor == nullptr);
        }

       #if ALOE_DEBUG && ! ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING
        // This will fail if you've called beginParameterChangeGesture() for one
        // or more parameters without having made a corresponding call to endParameterChangeGesture...
        jassert (changingParams.countNumberOfSetBits() == 0);
       #endif
        */
    }
}

impl<'a> GetNextBestLayoutInLayoutList for AudioProcessor<'a> {

    /**
      | Returns the next best layout which is
      | contained in a channel layout map.
      | 
      | You can use this method to help you implement
      | getNextBestLayout. For example:
      | 
      | -----------
      | @code
      | 
      | AudioProcessorBusesLayout getNextBestLayout (const AudioProcessorBusesLayout& layouts) override
      | {
      |     return getNextBestLayoutInLayoutList (layouts, {{1,1},{2,2}});
      | }
      |
      */
    fn get_next_best_layout_in_layout_list(
        &mut self, 
        layouts:             &dyn AudioProcessorBusesLayoutInterface,
        channel_layout_list: &[&[i16]; 2]

    ) -> Box<dyn AudioProcessorBusesLayoutInterface> {
    
        todo!();
        /*
            return getNextBestLayoutInList (layouts, layoutListToArray (channelLayoutList));
        */
    }
}
 
impl<'a> AudioProcessor<'a> {

    /**
      | Constructor for AudioProcessors which
      | use layout maps
      | 
      | If your AudioProcessor uses layout
      | maps then use this constructor.
      |
      */
    pub fn new_from_channel_layout_list(channel_layout_list: &[ [i16; 2] ]) -> Self {
    
        todo!();
        /*
        : audio_processor(busesPropertiesFromLayoutArray (layoutListToArray (channelLayoutList))),
        */
    }
    
    /**
      | Returns the number of buses on the input
      | or output side
      |
      */
    pub fn get_bus_count(&self, is_input: bool) -> i32 {
        
        todo!();
        /*
            return (isInput ? inputBuses : outputBuses).size();
        */
    }

    /**
      | Returns the audio bus with a given index
      | and direction.
      | 
      | If busIndex is invalid then this method
      | will return a nullptr.
      |
      */
    pub fn get_bus_mut(
        &mut self, 
        is_input:  bool,
        bus_index: i32

    ) -> *mut AudioProcessorBus {
        
        todo!();
        /*
            return (isInput ? inputBuses : outputBuses)[busIndex];
        */
    }

    /**
      | Returns the audio bus with a given index
      | and direction.
      | 
      | If busIndex is invalid then this method
      | will return a nullptr.
      |
      */
    pub fn get_bus(
        &self, 
        is_input:  bool,
        bus_index: i32

    ) -> *const AudioProcessorBus {
        
        todo!();
        /*
            return const_cast<AudioProcessor*> (this)->getBus (isInput, busIndex);
        */
    }

    /**
      | Provides the number of channels of the
      | bus with a given index and direction.
      | 
      | If the index, direction combination
      | is invalid then this will return zero.
      |
      */
    #[inline] pub fn get_channel_count_of_bus(&self, 
        is_input:  bool,
        bus_index: i32) -> i32 {
        
        todo!();
        /*
            if (auto* bus = getBus (isInput, busIndex))
                return bus->getNumberOfChannels();

            return 0;
        */
    }

    /**
      | Returns an AudioBuffer containing
      | a set of channel pointers for a specific
      | bus.
      | 
      | This can be called in processBlock to
      | get a buffer containing a sub-group
      | of the master
      | 
      | AudioBuffer which contains all the
      | plugin channels.
      |
      */
    pub fn get_bus_buffer<FloatType>(&self, 
        process_block_buffer: &mut AudioBuffer<FloatType>,
        is_input:             bool,
        bus_index:            i32) -> AudioBuffer<FloatType> {
    
        todo!();
        /*
            auto busNumChannels = getChannelCountOfBus (isInput, busIndex);
            auto channelOffset = getChannelIndexInProcessBlockBuffer (isInput, busIndex, 0);

            return AudioBuffer<FloatType> (processBlockBuffer.getArrayOfWritePointers() + channelOffset,
                                           busNumChannels, processBlockBuffer.getNumSamples());
        */
    }
    
    /**
      | Returns the precision-mode of the processor.
      | 
      | Depending on the result of this method
      | you MUST call the corresponding version
      | of processBlock. The default processing
      | precision is single precision. @see
      | setProcessingPrecision, supportsDoublePrecisionProcessing
      |
      */
    pub fn get_processing_precision(&self) -> AudioProcessorProcessingPrecision {
        
        todo!();
        /*
            return processingPrecision;
        */
    }

    /**
      | Returns true if the current precision
      | is set to doublePrecision.
      |
      */
    pub fn is_using_double_precision(&self) -> bool {
        
        todo!();
        /*
            return processingPrecision == doublePrecision;
        */
    }

    /**
      | Returns the current AudioPlayHead
      | object that should be used to find out
      | the state and position of the playhead.
      | 
      | You can ONLY call this from your processBlock()
      | method! Calling it at other times will
      | produce undefined behaviour, as the
      | host may not have any context in which
      | a time would make sense, and some hosts
      | will almost certainly have multithreading
      | issues if it's not called on the audio
      | thread.
      | 
      | The AudioPlayHead object that is returned
      | can be used to get the details about the
      | time of the start of the block currently
      | being processed. But do not store this
      | pointer or use it outside of the current
      | audio callback, because the host may
      | delete or re-use it.
      | 
      | If the host can't or won't provide any
      | time info, this will return nullptr.
      |
      */
    pub fn get_play_head(&self) -> *mut dyn AudioPlayHeadInterface {
        
        todo!();
        /*
            return playHead;
        */
    }

    /**
      | Returns the total number of input channels.
      | 
      | This method will return the total number
      | of input channels by accumulating the
      | number of channels on each input bus.
      | The number of channels of the buffer
      | passed to your processBlock callback
      | will be equivalent to either getTotalNumInputChannels
      | or getTotalNumOutputChannels - which
      | ever is greater.
      | 
      | -----------
      | @note
      | 
      | getTotalNumInputChannels is equivalent
      | to getMainBusNumInputChannels if
      | your processor does not have any sidechains
      | or aux buses.
      |
      */
    pub fn get_total_num_input_channels(&self) -> i32 {
        
        todo!();
        /*
            return cachedTotalIns;
        */
    }

    /**
      | Returns the total number of output channels.
      | 
      | This method will return the total number
      | of output channels by accumulating
      | the number of channels on each output
      | bus. The number of channels of the buffer
      | passed to your processBlock callback
      | will be equivalent to either getTotalNumInputChannels
      | or getTotalNumOutputChannels - which
      | ever is greater.
      | 
      | -----------
      | @note
      | 
      | getTotalNumOutputChannels is equivalent
      | to getMainBusNumOutputChannels if
      | your processor does not have any sidechains
      | or aux buses.
      |
      */
    pub fn get_total_num_output_channels(&self) -> i32 {
        
        todo!();
        /*
            return cachedTotalOuts;
        */
    }

    /**
      | Returns the number of input channels
      | on the main bus.
      |
      */
    #[inline] pub fn get_main_bus_num_input_channels(&self) -> i32 {
        
        todo!();
        /*
            return getChannelCountOfBus (true,  0);
        */
    }

    /**
      | Returns the number of output channels
      | on the main bus.
      |
      */
    #[inline] pub fn get_main_bus_num_output_channels(&self) -> i32 {
        
        todo!();
        /*
            return getChannelCountOfBus (false, 0);
        */
    }
    
    /**
      | Returns true if the channel layout map
      | contains a certain layout.
      | 
      | You can use this method to help you implement
      | the checkBusesLayoutSupported method.
      | For example
      | 
      | -----------
      | @code
      | 
      | bool checkBusesLayoutSupported (const AudioProcessorBusesLayout& layouts) override
      | {
      |     return containsLayout (layouts, {{1,1},{2,2}});
      | }
      |
      */
    pub fn contains_layout(
        layouts:             &AudioProcessorBusesLayout,
        channel_layout_list: &[ [i16; 2] ]

    ) -> bool {
        
        todo!();
        /*
            return containsLayout (layouts, layoutListToArray (channelLayoutList));
        */
    }
    
    pub fn contains_layout_with_list<const numLayouts: usize>(
        layouts:             &AudioProcessorBusesLayout,
        channel_layout_list: &[[i16; numLayouts]; 2]

    ) -> bool {
    
        todo!();
        /*
            return containsLayout (layouts, layoutListToArray (channelLayoutList));
        */
    }

    pub fn contains_layout_with_io_channel_pair_list(
        &mut self, 
        layouts:         &AudioProcessorBusesLayout,
        channel_layouts: &[AudioProcessorInOutChannelPair]

    ) -> bool {
        
        todo!();
        /*
            if (layouts.inputBuses.size() > 1 || layouts.outputBuses.size() > 1)
            return false;

        const AudioProcessorInOutChannelPair mainLayout (static_cast<int16> (layouts.getNumChannels (true, 0)),
                                           static_cast<int16> (layouts.getNumChannels (false, 0)));

        return channelLayouts.contains (mainLayout);
        */
    }

   
    /**
      | Returns the current sample rate.
      | 
      | This can be called from your processBlock()
      | method - it's not guaranteed to be valid
      | at any other time, and may return 0 if
      | it's unknown.
      |
      */
    pub fn get_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            return currentSampleRate;
        */
    }

    /**
      | Returns the current typical block size
      | that is being used.
      | 
      | This can be called from your processBlock()
      | method - it's not guaranteed to be valid
      | at any other time.
      | 
      | Remember it's not the ONLY block size
      | that may be used when calling processBlock,
      | it's just the normal one. The actual
      | block sizes used may be larger or smaller
      | than this, and will vary between successive
      | calls.
      |
      */
    pub fn get_block_size(&self) -> i32 {
        
        todo!();
        /*
            return blockSize;
        */
    }

    /**
      | This returns the number of samples delay
      | that the processor imposes on the audio
      | passing through it.
      | 
      | The host will call this to find the latency
      | - the processor itself should set this
      | value by calling setLatencySamples()
      | as soon as it can during its initialisation.
      |
      */
    pub fn get_latency_samples(&self) -> i32 {
        
        todo!();
        /*
            return latencySamples;
        */
    }

    /**
      | This returns a critical section that
      | will automatically be locked while
      | the host is calling the processBlock()
      | method.
      | 
      | Use it from your UI or other threads to
      | lock access to variables that are used
      | by the process callback, but obviously
      | be careful not to keep it locked for too
      | long, because that could cause stuttering
      | playback. If you need to do something
      | that'll take a long time and need the
      | processing to stop while it happens,
      | use the suspendProcessing() method
      | instead.
      | 
      | @see suspendProcessing
      |
      */
    pub fn get_callback_lock(&self) -> &CriticalSection {
        
        todo!();
        /*
            return callbackLock;
        */
    }

    /**
      | Returns true if processing is currently
      | suspended. @see suspendProcessing
      |
      */
    pub fn is_suspended(&self) -> bool {
        
        todo!();
        /*
            return suspended;
        */
    }
    
    /**
      | Returns true if the processor is being
      | run in an offline mode for rendering.
      | 
      | If the processor is being run live on
      | realtime signals, this returns false.
      | 
      | If the mode is unknown, this will assume
      | it's realtime and return false.
      | 
      | This value may be unreliable until the
      | prepareToPlay() method has been called,
      | and could change each time prepareToPlay()
      | is called.
      | 
      | @see setNonRealtime()
      |
      */
    pub fn is_non_realtime(&self) -> bool {
        
        todo!();
        /*
            return nonRealtime;
        */
    }

    pub fn layout_list_to_array_with_list<const numLayouts: usize>(configuration: &[[i16; numLayouts]; 2]) -> Vec<AudioProcessorInOutChannelPair> {
    
        todo!();
        /*
            Vec<AudioProcessorInOutChannelPair> layouts;

            for (size_t i = 0; i < numLayouts; ++i)
                layouts.add (AudioProcessorInOutChannelPair (configuration[(int) i]));

            return layouts;
        */
    }
    
    pub fn layout_list_to_array(configuration: &[ [i16; 2] ]) -> Vec<AudioProcessorInOutChannelPair> {
        
        todo!();
        /*
            Vec<AudioProcessorInOutChannelPair> layouts;

            for (auto&& i : configuration)
                layouts.add (AudioProcessorInOutChannelPair (i));

            return layouts;
        */
    }
    
    pub fn set_type_of_next_new_plugin(&mut self, ty: AudioProcessorWrapperType)  {
        
        todo!();
        /*
            wrapperTypeBeingCreated = type;
        */
    }
    
    /**
      | Constructor for multi-bus AudioProcessors
      | 
      | If your AudioProcessor supports multiple
      | buses than use this constructor to initialise
      | the bus layouts and bus names of your
      | plug-in.
      |
      */
    pub fn new(io_config: &AudioProcessorBusesProperties) -> Self {
    
        todo!();
        /*

            : wrapperType (wrapperTypeBeingCreated.get())

        for (auto& layout : ioConfig.inputLayouts)   createBus (true,  layout);
        for (auto& layout : ioConfig.outputLayouts)  createBus (false, layout);

        updateSpeakerFormatStrings();
        */
    }
    
    pub fn get_alternate_display_names(&self) -> Vec<String> {
        
        todo!();
        /*
            return Vec<String> (getName());
        */
    }
    
    /**
      | Dynamically request an additional
      | bus.
      | 
      | Request an additional bus from the audio
      | processor. If the audio processor does
      | not support adding additional buses
      | then this method will return false.
      | 
      | Most audio processors will not allow
      | you to dynamically add/remove audio
      | buses and will return false.
      | 
      | This method will invoke the canApplyBusCountChange
      | callback to probe if a bus can be added
      | and, if yes, will use the supplied bus
      | properties of the canApplyBusCountChange
      | callback to create a new bus.
      | 
      | @see canApplyBusCountChange, removeBus
      |
      */
    pub fn add_bus(&mut self, is_input: bool) -> bool {
        
        todo!();
        /*
            if (! canAddBus (isInput))
            return false;

        AudioProcessorBusProperties busesProps;

        if (! canApplyBusCountChange (isInput, true, busesProps))
            return false;

        createBus (isInput, busesProps);
        return true;
        */
    }
    
    /**
      | Dynamically remove the latest added
      | bus.
      | 
      | Request the removal of the last bus from
      | the audio processor. If the audio processor
      | does not support removing buses then
      | this method will return false.
      | 
      | Most audio processors will not allow
      | you to dynamically add/remove audio
      | buses and will return false.
      | 
      | The default implementation will return
      | false.
      | 
      | This method will invoke the canApplyBusCountChange
      | callback to probe if a bus can currently
      | be removed and, if yes, will go ahead
      | and remove it.
      | 
      | @see addBus, canRemoveBus
      |
      */
    pub fn remove_bus(&mut self, input_bus: bool) -> bool {
        
        todo!();
        /*
            auto numBuses = getBusCount (inputBus);

        if (numBuses == 0)
            return false;

        if (! canRemoveBus (inputBus))
            return false;

        AudioProcessorBusProperties busesProps;

        if (! canApplyBusCountChange (inputBus, false, busesProps))
            return false;

        auto busIndex = numBuses - 1;
        auto numChannels = getChannelCountOfBus (inputBus, busIndex);
        (inputBus ? inputBuses : outputBuses).remove (busIndex);

        audioIOChanged (true, numChannels > 0);
        return true;
        */
    }
    
    /**
      | Set the channel layouts of this audio
      | processor.
      | 
      | If the layout is not supported by this
      | audio processor then this method will
      | return false. You can use the checkBusesLayoutSupported
      | and getNextBestLayout methods to probe
      | which layouts this audio processor
      | supports.
      |
      */
    pub fn set_buses_layout(&mut self, arr: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            jassert (arr.inputBuses. size() == getBusCount (true)
              && arr.outputBuses.size() == getBusCount (false));

        if (arr == getBusesLayout())
            return true;

        auto copy = arr;

        if (! canApplyBusesLayout (copy))
            return false;

        return applyBusLayouts (copy);
        */
    }
    
    /**
      | Set the channel layouts of this audio
      | processor without changing the enablement
      | state of the buses.
      | 
      | If the layout is not supported by this
      | audio processor then this method will
      | return false. You can use the checkBusesLayoutSupported
      | methods to probe which layouts this
      | audio processor supports.
      |
      */
    pub fn set_buses_layout_without_enabling(&mut self, arr: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            auto numIns  = getBusCount (true);
        auto numOuts = getBusCount (false);

        jassert (arr.inputBuses. size() == numIns
              && arr.outputBuses.size() == numOuts);

        auto request = arr;
        auto current = getBusesLayout();

        for (int i = 0; i < numIns; ++i)
            if (request.getNumChannels (true, i) == 0)
                request.getChannelSet (true, i) = current.getChannelSet (true, i);

        for (int i = 0; i < numOuts; ++i)
            if (request.getNumChannels (false, i) == 0)
                request.getChannelSet (false, i) = current.getChannelSet (false, i);

        if (! checkBusesLayoutSupported (request))
            return false;

        for (int dir = 0; dir < 2; ++dir)
        {
            const bool isInput = (dir != 0);

            for (int i = 0; i < (isInput ? numIns : numOuts); ++i)
            {
                auto& bus = *getBus (isInput, i);
                auto& set = request.getChannelSet (isInput, i);

                if (! bus.isEnabled())
                {
                    if (! set.isDisabled())
                        bus.lastLayout = set;

                    set = AudioChannelSet::disabled();
                }
            }
        }

        return setBusesLayout (request);
        */
    }
    
    /**
      | Provides the current channel layouts
      | of this audio processor.
      |
      */
    pub fn get_buses_layout(&self) -> AudioProcessorBusesLayout {
        
        todo!();
        /*
            AudioProcessorBusesLayout layouts;

        for (auto& i : inputBuses)   layouts.inputBuses.add (i->getCurrentLayout());
        for (auto& i : outputBuses)  layouts.outputBuses.add (i->getCurrentLayout());

        return layouts;
        */
    }
    
    /**
      | Provides the channel layout of the bus
      | with a given index and direction.
      | 
      | If the index, direction combination
      | is invalid then this will return an
      | 
      | AudioChannelSet with no channels.
      |
      */
    pub fn get_channel_layout_of_bus(
        &self, 
        is_input:  bool,
        bus_index: i32

    ) -> AudioChannelSet {
        
        todo!();
        /*
            if (auto* bus = (isInput ? inputBuses : outputBuses)[busIndex])
            return bus->getCurrentLayout();

        return {};
        */
    }
    
    /**
      | Set the channel layout of the bus with
      | a given index and direction.
      | 
      | If the index, direction combination
      | is invalid or the layout is not supported
      | by the audio processor then this method
      | will return false.
      |
      */
    pub fn set_channel_layout_of_bus(
        &mut self, 
        is_input_bus: bool,
        bus_index:    i32,
        layout:       &AudioChannelSet

    ) -> bool {
        
        todo!();
        /*
            if (auto* bus = getBus (isInputBus, busIndex))
        {
            auto layouts = bus->getBusesLayoutForLayoutChangeOfBus (layout);

            if (layouts.getChannelSet (isInputBus, busIndex) == layout)
                return applyBusLayouts (layouts);

            return false;
        }

        jassertfalse;  // busIndex parameter is invalid
        return false;
        */
    }
    
    /**
      | Enables all buses
      |
      */
    pub fn enable_all_buses(&mut self) -> bool {
        
        todo!();
        /*
            AudioProcessorBusesLayout layouts;

        for (auto& i : inputBuses)   layouts.inputBuses.add (i->lastLayout);
        for (auto& i : outputBuses)  layouts.outputBuses.add (i->lastLayout);

        return setBusesLayout (layouts);
        */
    }
    
    /**
      | Returns true if the Audio processor
      | is likely to support a given layout.
      | 
      | This can be called regardless if the
      | processor is currently running.
      |
      */
    pub fn check_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            if (layouts.inputBuses.size() == inputBuses.size()
              && layouts.outputBuses.size() == outputBuses.size())
            return isBusesLayoutSupported (layouts);

        return false;
        */
    }
    
    pub fn get_next_best_layout(
        &self, 
        desired_layout: &AudioProcessorBusesLayout,
        actual_layouts: &mut AudioProcessorBusesLayout

    ) {
        
        todo!();
        /*
            // if you are hitting this assertion then you are requesting a next
        // best layout which does not have the same number of buses as the
        // audio processor.
        jassert (desiredLayout.inputBuses.size() == inputBuses.size()
              && desiredLayout.outputBuses.size() == outputBuses.size());

        if (checkBusesLayoutSupported (desiredLayout))
        {
            actualLayouts = desiredLayout;
            return;
        }

        auto originalState = actualLayouts;
        auto currentState = originalState;
        auto bestSupported = currentState;

        for (int dir = 0; dir < 2; ++dir)
        {
            const bool isInput = (dir > 0);

            auto& currentLayouts   = (isInput ? currentState.inputBuses  : currentState.outputBuses);
            auto& bestLayouts      = (isInput ? bestSupported.inputBuses : bestSupported.outputBuses);
            auto& requestedLayouts = (isInput ? desiredLayout.inputBuses : desiredLayout.outputBuses);
            auto& originalLayouts  = (isInput ? originalState.inputBuses : originalState.outputBuses);

            for (int busIndex = 0; busIndex < requestedLayouts.size(); ++busIndex)
            {
                auto& best       = bestLayouts     .getReference (busIndex);
                auto& requested  = requestedLayouts.getReference (busIndex);
                auto& original   = originalLayouts .getReference (busIndex);

                // do we need to do anything
                if (original == requested)
                    continue;

                currentState = bestSupported;
                auto& current = currentLayouts  .getReference (busIndex);

                // already supported?
                current = requested;

                if (checkBusesLayoutSupported (currentState))
                {
                    bestSupported = currentState;
                    continue;
                }

                // try setting the opposite bus to the identical layout
                const bool oppositeDirection = ! isInput;

                if (getBusCount (oppositeDirection) > busIndex)
                {
                    auto& oppositeLayout = (oppositeDirection ? currentState.inputBuses : currentState.outputBuses).getReference (busIndex);
                    oppositeLayout = requested;

                    if (checkBusesLayoutSupported (currentState))
                    {
                        bestSupported = currentState;
                        continue;
                    }

                    // try setting the default layout
                    oppositeLayout = getBus (oppositeDirection, busIndex)->getDefaultLayout();

                    if (checkBusesLayoutSupported (currentState))
                    {
                        bestSupported = currentState;
                        continue;
                    }
                }

                // try setting all other buses to the identical layout
                AudioProcessorBusesLayout allTheSame;
                allTheSame.inputBuses.insertMultiple (-1, requested, getBusCount (true));
                allTheSame.outputBuses.insertMultiple (-1, requested, getBusCount (false));

                if (checkBusesLayoutSupported (allTheSame))
                {
                    bestSupported = allTheSame;
                    continue;
                }

                // what is closer the default or the current layout?
                auto distance = std::abs (best.size() - requested.size());
                auto& defaultLayout = getBus (isInput, busIndex)->getDefaultLayout();

                if (std::abs (defaultLayout.size() - requested.size()) < distance)
                {
                    current = defaultLayout;

                    if (checkBusesLayoutSupported (currentState))
                        bestSupported = currentState;
                }
            }
        }

        actualLayouts = bestSupported;
        */
    }
    
    pub fn set_play_head(&mut self, new_play_head: *mut dyn AudioPlayHeadInterface)  {
        
        todo!();
        /*
            playHead = newPlayHead;
        */
    }
    
    pub fn add_listener(&mut self, new_listener: *mut dyn AudioProcessorListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
        listeners.addIfNotAlreadyThere (newListener);
        */
    }
    
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn AudioProcessorListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
        listeners.removeFirstMatchingValue (listenerToRemove);
        */
    }
    
    /**
      | This is called by the processor to specify
      | its details before being played. Use
      | this version of the function if you are
      | not interested in any sidechain and/or
      | aux buses and do not care about the layout
      | of channels. Otherwise use setRateAndBufferSizeDetails.
      |
      */
    pub fn set_play_config_details(
        &mut self, 
        new_num_ins:     i32,
        new_num_outs:    i32,
        new_sample_rate: f64,
        new_block_size:  i32

    ) {
        
        todo!();
        /*
            bool success = true;

        if (getTotalNumInputChannels() != newNumIns)
            success &= setChannelLayoutOfBus (true,  0, AudioChannelSet::canonicalChannelSet (newNumIns));

        // failed to find a compatible input configuration
        jassert (success);

        if (getTotalNumOutputChannels() != newNumOuts)
            success &= setChannelLayoutOfBus (false, 0, AudioChannelSet::canonicalChannelSet (newNumOuts));

        // failed to find a compatible output configuration
        jassert (success);

        // if the user is using this method then they do not want any side-buses or aux outputs
        success &= disableNonMainBuses();
        jassert (success);

        // the processor may not support this arrangement at all
        jassert (success && newNumIns == getTotalNumInputChannels() && newNumOuts == getTotalNumOutputChannels());

        setRateAndBufferSizeDetails (newSampleRate, newBlockSize);
        ignoreUnused (success);
        */
    }
    
    /**
      | This is called by the processor to specify
      | its details before being played. You
      | should call this function after having
      | informed the processor about the channel
      | and bus layouts via setBusesLayout.
      | 
      | @see setBusesLayout
      |
      */
    pub fn set_rate_and_buffer_size_details(
        &mut self, 
        new_sample_rate: f64,
        new_block_size:  i32

    ) {
        
        todo!();
        /*
            currentSampleRate = newSampleRate;
        blockSize = newBlockSize;
        */
    }
    
    pub fn num_channels_changed(&mut self)  { }
    
    pub fn num_buses_changed(&mut self)  { }
    
    pub fn processor_layouts_changed(&mut self)  { }
    
    /**
      | Returns the position of a bus's channels
      | within the processBlock buffer.
      | 
      | This can be called in processBlock to
      | figure out which channel of the master
      | AudioBuffer maps onto a specific bus's
      | channel.
      |
      */
    pub fn get_channel_index_in_process_block_buffer(
        &self, 
        is_input:      bool,
        bus_index:     i32,
        channel_index: i32

    ) -> i32 {
        
        todo!();
        /*
            auto& ioBus = isInput ? inputBuses : outputBuses;
        jassert (isPositiveAndBelow (busIndex, ioBus.size()));

        for (int i = 0; i < ioBus.size() && i < busIndex; ++i)
            channelIndex += getChannelCountOfBus (isInput, i);

        return channelIndex;
        */
    }
    
    /**
      | Returns the offset in a bus's buffer
      | from an absolute channel index.
      | 
      | This method returns the offset in a bus's
      | buffer given an absolute channel index.
      | 
      | It also provides the bus index. For example,
      | this method would return one for a processor
      | with two stereo buses when given the
      | absolute channel index.
      |
      */
    pub fn get_offset_in_bus_buffer_for_absolute_channel_index(
        &self, 
        is_input:               bool,
        absolute_channel_index: i32,
        bus_index:              &mut i32

    ) -> i32 {
        
        todo!();
        /*
            auto numBuses = getBusCount (isInput);
        int numChannels = 0;

        for (busIndex = 0; busIndex < numBuses && absoluteChannelIndex >= (numChannels = getChannelLayoutOfBus (isInput, busIndex).size()); ++busIndex)
            absoluteChannelIndex -= numChannels;

        return busIndex >= numBuses ? -1 : absoluteChannelIndex;
        */
    }
    
    pub fn set_non_realtime(&mut self, new_non_realtime: bool)  {
        
        todo!();
        /*
            nonRealtime = newNonRealtime;
        */
    }
    
    /**
      | Your processor subclass should call
      | this to set the number of samples delay
      | that it introduces.
      | 
      | The processor should call this as soon
      | as it can during initialisation, and
      | can call it later if the value changes.
      |
      */
    pub fn set_latency_samples(&mut self, new_latency: i32)  {
        
        todo!();
        /*
            if (latencySamples != newLatency)
        {
            latencySamples = newLatency;
            updateHostDisplay (AudioProcessorListener::AudioProcessorChangeDetails().withLatencyChanged (true));
        }
        */
    }
    
    pub fn get_listener_locked(&self, index: i32) -> *mut dyn AudioProcessorListener {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
        return listeners[index];
        */
    }
    
    /**
      | The processor can call this when something
      | (apart from a parameter value) has changed.
      | 
      | It sends a hint to the host that something
      | like the program, number of parameters,
      | etc, has changed, and that it should
      | update itself.
      |
      */
    pub fn update_host_display(&mut self, details: Option<&AudioProcessorListenerChangeDetails>)  {

        let details = details.unwrap_or(&AudioProcessorChangeDetails::get_all_changed());
        
        todo!();
        /*
            for (int i = listeners.size(); --i >= 0;)
            if (auto l = getListenerLocked (i))
                l->audioProcessorChanged (this, details);
        */
    }
    
    pub fn check_for_unsafe_paramid(&mut self, param: *mut AudioProcessorParameter)  {
        
        todo!();
        /*
            checkForDuplicateParamID (param);
        checkForDuplicateTrimmedParamID (param);
        */
    }
    
    pub fn check_for_duplicate_trimmed_paramid(&mut self, param: *mut AudioProcessorParameter)  {
        
        todo!();
        /*
            ignoreUnused (param);

       #if ALOE_DEBUG && ! ALOE_DISABLE_CAUTIOUS_PARAMETER_ID_CHECKING
        if (auto* withID = dynamic_cast<AudioProcessorParameterWithID*> (param))
        {
            constexpr auto maximumSafeAAXParameterIdLength = 31;

            const auto paramID = withID->paramID;

            // If you hit this assertion, a parameter name is too long to be supported
            // by the AAX plugin format.
            // If there's a chance that you'll release this plugin in AAX format, you
            // should consider reducing the length of this paramID.
            // If you need to retain backwards-compatibility and are unable to change
            // the paramID for this reason, you can add ALOE_DISABLE_CAUTIOUS_PARAMETER_ID_CHECKING
            // to your preprocessor definitions to silence this assertion.
            jassertquiet (paramID.length() <= maximumSafeAAXParameterIdLength);

            // If you hit this assertion, two or more parameters have duplicate paramIDs
            // after they have been truncated to support the AAX format.
            // This is a serious issue, and will prevent the duplicated parameters from
            // being automated when running as an AAX plugin.
            // If there's a chance that you'll release this plugin in AAX format, you
            // should reduce the length of this paramID.
            // If you need to retain backwards-compatibility and are unable to change
            // the paramID for this reason, you can add ALOE_DISABLE_CAUTIOUS_PARAMETER_ID_CHECKING
            // to your preprocessor definitions to silence this assertion.
            jassertquiet (trimmedParamIDs.insert (paramID.substring (0, maximumSafeAAXParameterIdLength)).second);
        }
       #endif
        */
    }
    
    pub fn check_for_duplicate_paramid(&mut self, param: *mut AudioProcessorParameter)  {
        
        todo!();
        /*
            ignoreUnused (param);

       #if ALOE_DEBUG
        if (auto* withID = dynamic_cast<AudioProcessorParameterWithID*> (param))
        {
            auto insertResult = paramIDs.insert (withID->paramID);

            // If you hit this assertion then the parameter ID is not unique
            jassert (insertResult.second);
        }
       #endif
        */
    }
    
    pub fn check_for_duplicate_group_ids(&mut self, new_group: &AudioProcessorParameterGroup)  {
        
        todo!();
        /*
            ignoreUnused (newGroup);

       #if ALOE_DEBUG
        auto groups = newGroup.getSubgroups (true);
        groups.add (&newGroup);

        for (auto* group : groups)
        {
            auto insertResult = groupIDs.insert (group->getID());

            // If you hit this assertion then a group ID is not unique
            jassert (insertResult.second);
        }
       #endif
        */
    }
    
    /**
      | Returns a flat list of the parameters
      | in the current tree.
      |
      */
    pub fn get_parameters(&self) -> &[*mut AudioProcessorParameter] {
        
        todo!();
        /*
            return flatParameterList;
        */
    }
    
    /**
      | Returns the group of parameters managed
      | by this AudioProcessor.
      |
      */
    pub fn get_parameter_tree(&self) -> &AudioProcessorParameterGroup {
        
        todo!();
        /*
            return parameterTree;
        */
    }
    
    /**
      | Adds a parameter to the AudioProcessor.
      | 
      | The parameter object will be managed
      | and deleted automatically by the
      | 
      | AudioProcessor when no longer needed.
      |
      */
    pub fn add_parameter(&mut self, param: *mut AudioProcessorParameter)  {
        
        todo!();
        /*
            jassert (param != nullptr);
        parameterTree.addChild (std::unique_ptr<AudioProcessorParameter> (param));

        param->processor = this;
        param->parameterIndex = flatParameterList.size();
        flatParameterList.add (param);

        checkForUnsafeParamID (param);
        */
    }
    
    /**
      | Adds a group of parameters to the AudioProcessor.
      | 
      | All the parameter objects contained
      | within the group will be managed and
      | deleted automatically by the AudioProcessor
      | when no longer needed.
      | 
      | @see addParameter
      |
      */
    pub fn add_parameter_group(&mut self, group: Box<AudioProcessorParameterGroup>)  {
        
        todo!();
        /*
            jassert (group != nullptr);
        checkForDuplicateGroupIDs (*group);

        auto oldSize = flatParameterList.size();
        flatParameterList.addArray (group->getParameters (true));

        for (int i = oldSize; i < flatParameterList.size(); ++i)
        {
            auto p = flatParameterList.getUnchecked (i);
            p->processor = this;
            p->parameterIndex = i;

            checkForUnsafeParamID (p);
        }

        parameterTree.addChild (std::move (group));
        */
    }
    
    /**
      | Sets the group of parameters managed
      | by this AudioProcessor.
      | 
      | Replacing the tree after your AudioProcessor
      | has been constructed will crash many
      | hosts, so don't do it! You may, however,
      | change parameter and group names by
      | iterating the tree returned by getParameterTree().
      | 
      | Afterwards, call updateHostDisplay()
      | to inform the host of the changes.
      | 
      | Not all hosts support dynamic changes
      | to parameters and group names.
      |
      */
    pub fn set_parameter_tree(&mut self, new_tree: AudioProcessorParameterGroup)  {
        
        todo!();
        /*
            #if ALOE_DEBUG
        paramIDs.clear();
        groupIDs.clear();
       #endif

        parameterTree = std::move (newTree);
        checkForDuplicateGroupIDs (parameterTree);

        flatParameterList = parameterTree.getParameters (true);

        for (int i = 0; i < flatParameterList.size(); ++i)
        {
            auto p = flatParameterList.getUnchecked (i);
            p->processor = this;
            p->parameterIndex = i;

            checkForUnsafeParamID (p);
        }
        */
    }
    
    pub fn refresh_parameter_list(&mut self)  { }
    
    /**
      | Returns the default number of steps
      | for a parameter.
      | 
      | NOTE! This method is deprecated! It's
      | recommended that you use
      | 
      | AudioProcessorParameter::getNumSteps()
      | instead.
      | 
      | @see getParameterNumSteps
      |
      */
    pub fn get_default_num_parameter_steps(&mut self) -> i32 {
        
        todo!();
        /*
            return 0x7fffffff;
        */
    }
    
    /**
      | Enables and disables the processing
      | callback.
      | 
      | If you need to do something time-consuming
      | on a thread and would like to make sure
      | the audio processing callback doesn't
      | happen until you've finished, use this
      | to disable the callback and re-enable
      | it again afterwards.
      | 
      | -----------
      | @code
      | 
      | void loadNewPatch()
      | {
      |     suspendProcessing (true);
      | 
      |     ..do something that takes ages..
      | 
      |     suspendProcessing (false);
      | }
      | 
      | If the host tries to make an audio callback
      | while processing is suspended, the
      | processor will return an empty buffer,
      | but won't block the audio thread like
      | it would do if you use the getCallbackLock()
      | critical section to synchronise access.
      | 
      | Any code that calls processBlock()
      | should call isSuspended() before doing
      | so, and if the processor is suspended,
      | it should avoid the call and emit silence
      | or whatever is appropriate.
      | 
      | @see getCallbackLock
      |
      */
    pub fn suspend_processing(&mut self, should_be_suspended: bool)  {
        
        todo!();
        /*
            const ScopedLock sl (callbackLock);
        suspended = shouldBeSuspended;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn process_bypassed<FloatType: num::Float>(
        &mut self, 
        buffer: &mut AudioBuffer<FloatType>,
        _1:     &mut MidiBuffer

    ) {
    
        todo!();
        /*
            // If you hit this assertion then your plug-in is reporting that it introduces
        // some latency, but you haven't overridden processBlockBypassed to produce
        // an identical amount of latency. Without identical latency in
        // processBlockBypassed a host's latency compensation could shift the audio
        // passing through your bypassed plug-in forward in time.
        jassert (getLatencySamples() == 0);

        for (int ch = getMainBusNumInputChannels(); ch < getTotalNumOutputChannels(); ++ch)
            buffer.clear (ch, 0, buffer.getNumSamples());
        */
    }
    
    pub fn process_block_bypassed(
        &mut self, 
        buffer: &mut AudioBuffer<f32>,
        midi:   &mut MidiBuffer

    ) {
        
        todo!();
        /*
            processBypassed (buffer, midi);
        */
    }
    
    pub fn process_block_bypassed_f64(
        &mut self, 
        buffer: &mut AudioBuffer<f64>,
        midi:   &mut MidiBuffer)  {
        
        todo!();
        /*
            processBypassed (buffer, midi);
        */
    }
    
    pub fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            ignoreUnused (buffer, midiMessages);

        // If you hit this assertion then either the caller called the double
        // precision version of processBlock on a processor which does not support it
        // (i.e. supportsDoublePrecisionProcessing() returns false), or the implementation
        // of the AudioProcessor forgot to override the double precision version of this method
        jassertfalse;
        */
    }
    
    pub fn supports_double_precision_processing(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    /**
      | Changes the processing precision of
      | the receiver. A client of the AudioProcessor
      | calls this function to indicate which
      | version of processBlock (single or
      | double precision) it intends to call.
      | The client MUST call this function before
      | calling the prepareToPlay method so
      | that the receiver can do any necessary
      | allocations in the prepareToPlay()
      | method. An implementation of prepareToPlay()
      | should call getProcessingPrecision()
      | to determine with which precision it
      | should allocate it's internal buffers.
      | 
      | -----------
      | @note
      | 
      | setting the processing precision to
      | double floating point precision on
      | a receiver which does not support double
      | precision processing (i.e. supportsDoublePrecisionProcessing()
      | returns false) will result in an assertion.
      | 
      | @see getProcessingPrecision, supportsDoublePrecisionProcessing
      |
      */
    pub fn set_processing_precision(&mut self, precision: AudioProcessorProcessingPrecision)  {
        
        todo!();
        /*
            // If you hit this assertion then you're trying to use double precision
        // processing on a processor which does not support it!
        jassert (precision != doublePrecision || supportsDoublePrecisionProcessing());

        processingPrecision = precision;
        */
    }
    
    pub fn get_input_channel_name(&self, index: i32) -> String {
        
        todo!();
        /*
            return getChannelName (inputBuses,  index);
        */
    }
    
    pub fn get_output_channel_name(&self, index: i32) -> String {
        
        todo!();
        /*
            return getChannelName (outputBuses, index);
        */
    }
    
    pub fn is_input_channel_stereo_pair(&self, index: i32) -> bool {
        
        todo!();
        /*
            return isStereoPair (inputBuses, index);
        */
    }
    
    pub fn is_output_channel_stereo_pair(&self, index: i32) -> bool {
        
        todo!();
        /*
            return isStereoPair (outputBuses, index);
        */
    }
    
    pub fn create_bus(&mut self, 
        input_bus: bool,
        io_config: &AudioProcessorBusProperties)  {
        
        todo!();
        /*
            (inputBus ? inputBuses : outputBuses).add (new AudioProcessorBus (*this, ioConfig.busName, ioConfig.defaultLayout, ioConfig.isActivatedByDefault));

        audioIOChanged (true, ioConfig.isActivatedByDefault);
        */
    }
    
    pub fn buses_properties_from_layout_array(
        &mut self, 
        config: &[AudioProcessorInOutChannelPair]

    ) -> AudioProcessorBusesProperties {
        
        todo!();
        /*
            AudioProcessorBusesProperties ioProps;

        if (config[0].inChannels > 0)
            ioProps.addBus (true, "Input", AudioChannelSet::canonicalChannelSet (config[0].inChannels));

        if (config[0].outChannels > 0)
            ioProps.addBus (false, "Output", AudioChannelSet::canonicalChannelSet (config[0].outChannels));

        return ioProps;
        */
    }
    
    pub fn get_next_best_layout_in_list(
        &self, 
        layouts:        &AudioProcessorBusesLayout,
        legacy_layouts: &[AudioProcessorInOutChannelPair]

    ) -> AudioProcessorBusesLayout {
        
        todo!();
        /*
            auto numChannelConfigs = legacyLayouts.size();
        jassert (numChannelConfigs > 0);

        bool hasInputs = false, hasOutputs = false;

        for (int i = 0; i < numChannelConfigs; ++i)
        {
            if (legacyLayouts[i].inChannels > 0)
            {
                hasInputs = true;
                break;
            }
        }

        for (int i = 0; i < numChannelConfigs; ++i)
        {
            if (legacyLayouts[i].outChannels > 0)
            {
                hasOutputs = true;
                break;
            }
        }

        auto nearest = layouts;
        nearest.inputBuses .resize (hasInputs  ? 1 : 0);
        nearest.outputBuses.resize (hasOutputs ? 1 : 0);

        auto* inBus  = (hasInputs  ? &nearest.inputBuses. getReference (0) : nullptr);
        auto* outBus = (hasOutputs ? &nearest.outputBuses.getReference (0) : nullptr);

        auto inNumChannelsRequested  = static_cast<int16> (inBus  != nullptr ? inBus->size()  : 0);
        auto outNumChannelsRequested = static_cast<int16> (outBus != nullptr ? outBus->size() : 0);

        auto distance = std::numeric_limits<int32>::max();
        int bestConfiguration = 0;

        for (int i = 0; i < numChannelConfigs; ++i)
        {
            auto inChannels  = legacyLayouts.getReference (i).inChannels;
            auto outChannels = legacyLayouts.getReference (i).outChannels;

            auto channelDifference = ((std::abs (inChannels  - inNumChannelsRequested)  & 0xffff) << 16)
                                        | ((std::abs (outChannels - outNumChannelsRequested) & 0xffff) << 0);

            if (channelDifference < distance)
            {
                distance = channelDifference;
                bestConfiguration = i;

                // we can exit if we found a perfect match
                if (distance == 0)
                    return nearest;
            }
        }

        auto inChannels  = legacyLayouts.getReference (bestConfiguration).inChannels;
        auto outChannels = legacyLayouts.getReference (bestConfiguration).outChannels;

        auto currentState = getBusesLayout();
        auto currentInLayout  = (getBusCount (true)  > 0 ? currentState.inputBuses .getReference(0) : AudioChannelSet());
        auto currentOutLayout = (getBusCount (false) > 0 ? currentState.outputBuses.getReference(0) : AudioChannelSet());

        if (inBus != nullptr)
        {
            if      (inChannels == 0)                       *inBus = AudioChannelSet::disabled();
            else if (inChannels == currentInLayout. size()) *inBus = currentInLayout;
            else if (inChannels == currentOutLayout.size()) *inBus = currentOutLayout;
            else                                            *inBus = AudioChannelSet::canonicalChannelSet (inChannels);
        }

        if (outBus != nullptr)
        {
            if      (outChannels == 0)                       *outBus = AudioChannelSet::disabled();
            else if (outChannels == currentOutLayout.size()) *outBus = currentOutLayout;
            else if (outChannels == currentInLayout .size()) *outBus = currentInLayout;
            else                                             *outBus = AudioChannelSet::canonicalChannelSet (outChannels);
        }

        return nearest;
        */
    }
    
    /**
      | Disables all non-main buses (aux and
      | sidechains).
      |
      */
    pub fn disable_non_main_buses(&mut self) -> bool {
        
        todo!();
        /*
            auto layouts = getBusesLayout();

        for (int busIndex = 1; busIndex < layouts.inputBuses.size(); ++busIndex)
            layouts.inputBuses.getReference (busIndex) = AudioChannelSet::disabled();

        for (int busIndex = 1; busIndex < layouts.outputBuses.size(); ++busIndex)
            layouts.outputBuses.getReference (busIndex) = AudioChannelSet::disabled();

        return setBusesLayout (layouts);
        */
    }

    /**
      | Unfortunately the deprecated
      | getInputSpeakerArrangement/getOutputSpeakerArrangement
      | return references to strings. Therefore we need
      | to keep a copy. Once getInputSpeakerArrangement
      | is removed, we can also remove this function
      */
    pub fn update_speaker_format_strings(&mut self)  {
        
        todo!();
        /*
            cachedInputSpeakerArrString.clear();
        cachedOutputSpeakerArrString.clear();

        if (getBusCount (true) > 0)
            cachedInputSpeakerArrString  = getBus (true,  0)->getCurrentLayout().getSpeakerArrangementAsString();

        if (getBusCount (false) > 0)
            cachedOutputSpeakerArrString = getBus (false, 0)->getCurrentLayout().getSpeakerArrangementAsString();
        */
    }
    
    pub fn apply_bus_layouts(&mut self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            if (layouts == getBusesLayout())
            return true;

        auto numInputBuses  = getBusCount (true);
        auto numOutputBuses = getBusCount (false);

        auto oldNumberOfIns  = getTotalNumInputChannels();
        auto oldNumberOfOuts = getTotalNumOutputChannels();

        if (layouts.inputBuses. size() != numInputBuses
         || layouts.outputBuses.size() != numOutputBuses)
            return false;

        int newNumberOfIns = 0, newNumberOfOuts = 0;

        for (int busIndex = 0; busIndex < numInputBuses;  ++busIndex)
        {
            auto& bus = *getBus (true, busIndex);
            const auto& set = layouts.getChannelSet (true, busIndex);
            bus.layout = set;

            if (! set.isDisabled())
                bus.lastLayout = set;

            newNumberOfIns += set.size();
        }

        for (int busIndex = 0; busIndex < numOutputBuses;  ++busIndex)
        {
            auto& bus = *getBus (false, busIndex);
            const auto& set = layouts.getChannelSet (false, busIndex);
            bus.layout = set;

            if (! set.isDisabled())
                bus.lastLayout = set;

            newNumberOfOuts += set.size();
        }

        const bool channelNumChanged = (oldNumberOfIns != newNumberOfIns || oldNumberOfOuts != newNumberOfOuts);
        audioIOChanged (false, channelNumChanged);

        return true;
        */
    }
    
    pub fn audio_io_changed(&mut self, 
        bus_number_changed:  bool,
        channel_num_changed: bool)  {
        
        todo!();
        /*
            auto numInputBuses  = getBusCount (true);
        auto numOutputBuses = getBusCount (false);

        for (int dir = 0; dir < 2; ++dir)
        {
            const bool isInput = (dir == 0);
            auto num = (isInput ? numInputBuses : numOutputBuses);

            for (int i = 0; i < num; ++i)
                if (auto* bus = getBus (isInput, i))
                    bus->updateChannelCount();
        }

        auto countTotalChannels = [] (const Vec<Box<AudioProcessorBus>>& buses) 
        {
            int n = 0;

            for (auto* bus : buses)
                n += bus->getNumberOfChannels();

            return n;
        };

        cachedTotalIns  = countTotalChannels (inputBuses);
        cachedTotalOuts = countTotalChannels (outputBuses);

        updateSpeakerFormatStrings();

        if (busNumberChanged)
            numBusesChanged();

        if (channelNumChanged)
            numChannelsChanged();

        processorLayoutsChanged();
        */
    }
    
    /**
      | Not for public use - this is called before
      | deleting an editor component.
      |
      */
    pub fn editor_being_deleted(&mut self, editor: *mut AudioProcessorEditor)  {
        
        todo!();
        /*
            const ScopedLock sl (activeEditorLock);

        if (activeEditor == editor)
            activeEditor = nullptr;
        */
    }
    
    /**
      | Returns the active editor, if there
      | is one. Bear in mind this can return nullptr
      | even if an editor has previously been
      | opened.
      | 
      | -----------
      | @note
      | 
      | you should only call this method from
      | the message thread as the active editor
      | may be deleted by the message thread,
      | causing a dangling pointer.
      |
      */
    pub fn get_active_editor(&self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            const ScopedLock sl (activeEditorLock);
        return activeEditor;
        */
    }
    
    /**
      | Returns the active editor, or if there
      | isn't one, it will create one.
      | 
      | This may call createEditor() internally
      | to create the component.
      |
      */
    pub fn create_editor_if_needed(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            const ScopedLock sl (activeEditorLock);

        if (activeEditor != nullptr)
            return activeEditor;

        auto* ed = createEditor();

        if (ed != nullptr)
        {
            // you must give your editor comp a size before returning it..
            jassert (ed->getWidth() > 0 && ed->getHeight() > 0);
            activeEditor = ed;
        }

        // You must make your hasEditor() method return a consistent result!
        jassert (hasEditor() == (ed != nullptr));

        return ed;
        */
    }
    
    pub fn get_current_program_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            getStateInformation (destData);
        */
    }
    
    pub fn set_current_program_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            setStateInformation (data, sizeInBytes);
        */
    }
    
    pub fn update_track_properties(&mut self, _0: &AudioProcessorTrackProperties)  { }
    
    /**
      | Helper function that just converts
      | an xml element into a binary blob.
      | 
      | Use this in your processor's getStateInformation()
      | method if you want to store its state
      | as xml.
      | 
      | Then use getXmlFromBinary() to reverse
      | this operation and retrieve the XML
      | from a binary blob.
      |
      */
    pub fn copy_xml_to_binary(&mut self, 
        xml:       &XmlElement,
        dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            {
            MemoryOutputStream out (destData, false);
            out.writeInt (magicXmlNumber);
            out.writeInt (0);
            xml.writeTo (out, XmlElement::TextFormat().singleLine());
            out.writeByte (0);
        }

        // go back and write the string length..
        static_cast<uint32*> (destData.getData())[1]
            = ByteOrder::swapIfBigEndian ((uint32) destData.getSize() - 9);
        */
    }
    
    /**
      | Retrieves an XML element that was stored
      | as binary with the copyXmlToBinary()
      | method.
      | 
      | This might return nullptr if the data's
      | unsuitable or corrupted.
      |
      */
    pub fn get_xml_from_binary(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32) -> Box<XmlElement> {
        
        todo!();
        /*
            if (sizeInBytes > 8 && ByteOrder::littleEndianInt (data) == magicXmlNumber)
        {
            auto stringLength = (int) ByteOrder::littleEndianInt (addBytesToPointer (data, 4));

            if (stringLength > 0)
                return parseXML (String::fromUTF8 (static_cast<const char*> (data) + 8,
                                                    jmin ((sizeInBytes - 8), stringLength)));
        }

        return {};
        */
    }
    
    pub fn can_apply_bus_count_change(&mut self, 
        is_input:       bool,
        is_adding:      bool,
        out_properties: &mut AudioProcessorBusProperties) -> bool {
        
        todo!();
        /*
            if (  isAdding && ! canAddBus    (isInput)) return false;
        if (! isAdding && ! canRemoveBus (isInput)) return false;

        auto num = getBusCount (isInput);

        // No way for me to find out the default layout if there are no other busses!!
        if (num == 0)
            return false;

        if (isAdding)
        {
            outProperties.busName = String (isInput ? "Input #" : "Output #") + String (getBusCount (isInput));
            outProperties.defaultLayout = (num > 0 ? getBus (isInput, num - 1)->getDefaultLayout() : AudioChannelSet());
            outProperties.isActivatedByDefault = true;
        }

        return true;
        */
    }
    
    pub fn get_aax_plugin_id_for_main_bus_config(&self, 
        main_input_layout:  &AudioChannelSet,
        main_output_layout: &AudioChannelSet,
        id_for_audio_suite: bool) -> i32 {
        
        todo!();
        /*
            int uniqueFormatId = 0;

        for (int dir = 0; dir < 2; ++dir)
        {
            const bool isInput = (dir == 0);
            auto& set = (isInput ? mainInputLayout : mainOutputLayout);
            int aaxFormatIndex = 0;

            if      (set == AudioChannelSet::disabled())             aaxFormatIndex = 0;
            else if (set == AudioChannelSet::mono())                 aaxFormatIndex = 1;
            else if (set == AudioChannelSet::stereo())               aaxFormatIndex = 2;
            else if (set == AudioChannelSet::createLCR())            aaxFormatIndex = 3;
            else if (set == AudioChannelSet::createLCRS())           aaxFormatIndex = 4;
            else if (set == AudioChannelSet::quadraphonic())         aaxFormatIndex = 5;
            else if (set == AudioChannelSet::create5point0())        aaxFormatIndex = 6;
            else if (set == AudioChannelSet::create5point1())        aaxFormatIndex = 7;
            else if (set == AudioChannelSet::create6point0())        aaxFormatIndex = 8;
            else if (set == AudioChannelSet::create6point1())        aaxFormatIndex = 9;
            else if (set == AudioChannelSet::create7point0())        aaxFormatIndex = 10;
            else if (set == AudioChannelSet::create7point1())        aaxFormatIndex = 11;
            else if (set == AudioChannelSet::create7point0SDDS())    aaxFormatIndex = 12;
            else if (set == AudioChannelSet::create7point1SDDS())    aaxFormatIndex = 13;
            else if (set == AudioChannelSet::create7point0point2())  aaxFormatIndex = 14;
            else if (set == AudioChannelSet::create7point1point2())  aaxFormatIndex = 15;
            else if (set == AudioChannelSet::ambisonic (1))          aaxFormatIndex = 16;
            else if (set == AudioChannelSet::ambisonic (2))          aaxFormatIndex = 17;
            else if (set == AudioChannelSet::ambisonic (3))          aaxFormatIndex = 18;
            else
            {
                // AAX does not support this format and the wrapper should not have
                // called this method with this layout
                jassertfalse;
            }

            uniqueFormatId = (uniqueFormatId << 8) | aaxFormatIndex;
        }

        return (idForAudioSuite ? 0x6a796161 /* 'jyaa' */ : 0x6a636161 /* 'jcaa' */) + uniqueFormatId;
        */
    }
    
    /**
      | Returns a textual description of a AudioProcessorWrapperType
      | value
      |
      */
    pub fn get_wrapper_type_description(&mut self, ty: AudioProcessorWrapperType) -> *const u8 {
        
        todo!();
        /*
            switch (type)
        {
            case AudioProcessor::wrapperType_Undefined:     return "Undefined";
            case AudioProcessor::wrapperType_Vst:           return "Vst";
            case AudioProcessor::wrapperType_Vst3:          return "Vst3";
            case AudioProcessor::wrapperType_AudioUnit:     return "AU";
            case AudioProcessor::wrapperType_AudioUnitv3:   return "AUv3";
            case AudioProcessor::wrapperType_RTAS:          return "RTAS";
            case AudioProcessor::wrapperType_AAX:           return "AAX";
            case AudioProcessor::wrapperType_Standalone:    return "Standalone";
            case AudioProcessor::wrapperType_Unity:         return "Unity";
            default:                                        jassertfalse; return {};
        }
        */
    }
    
    pub fn set_parameter_notifying_host(&mut self, 
        parameter_index: i32,
        new_value:       f32)  {
        
        todo!();
        /*
            if (auto* param = getParameters()[parameterIndex])
        {
            param->setValueNotifyingHost (newValue);
        }
        else if (isPositiveAndBelow (parameterIndex, getNumParameters()))
        {
            setParameter (parameterIndex, newValue);
            sendParamChangeMessageToListeners (parameterIndex, newValue);
        }
        */
    }
    
    pub fn send_param_change_message_to_listeners(&mut self, 
        parameter_index: i32,
        new_value:       f32)  {
        
        todo!();
        /*
            if (auto* param = getParameters()[parameterIndex])
        {
            param->sendValueChangedMessageToListeners (newValue);
        }
        else
        {
            if (isPositiveAndBelow (parameterIndex, getNumParameters()))
            {
                for (int i = listeners.size(); --i >= 0;)
                    if (auto* l = getListenerLocked (i))
                        l->audioProcessorParameterChanged (this, parameterIndex, newValue);
            }
            else
            {
                jassertfalse; // called with an out-of-range parameter index!
            }
        }
        */
    }
    
    pub fn begin_parameter_change_gesture(&mut self, parameter_index: i32)  {
        
        todo!();
        /*
            if (auto* param = getParameters()[parameterIndex])
        {
            param->beginChangeGesture();
        }
        else
        {
            if (isPositiveAndBelow (parameterIndex, getNumParameters()))
            {
               #if ALOE_DEBUG && ! ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING
                // This means you've called beginParameterChangeGesture twice in succession without a matching
                // call to endParameterChangeGesture. That might be fine in most hosts, but better to avoid doing it.
                jassert (! changingParams[parameterIndex]);
                changingParams.setBit (parameterIndex);
               #endif

                for (int i = listeners.size(); --i >= 0;)
                    if (auto* l = getListenerLocked (i))
                        l->audioProcessorParameterChangeGestureBegin (this, parameterIndex);
            }
            else
            {
                jassertfalse; // called with an out-of-range parameter index!
            }
        }
        */
    }
    
    pub fn end_parameter_change_gesture(&mut self, parameter_index: i32)  {
        
        todo!();
        /*
            if (auto* param = getParameters()[parameterIndex])
        {
            param->endChangeGesture();
        }
        else
        {
            if (isPositiveAndBelow (parameterIndex, getNumParameters()))
            {
               #if ALOE_DEBUG && ! ALOE_DISABLE_AUDIOPROCESSOR_BEGIN_END_GESTURE_CHECKING
                // This means you've called endParameterChangeGesture without having previously called
                // beginParameterChangeGesture. That might be fine in most hosts, but better to keep the
                // calls matched correctly.
                jassert (changingParams[parameterIndex]);
                changingParams.clearBit (parameterIndex);
               #endif

                for (int i = listeners.size(); --i >= 0;)
                    if (auto* l = getListenerLocked (i))
                        l->audioProcessorParameterChangeGestureEnd (this, parameterIndex);
            }
            else
            {
                jassertfalse; // called with an out-of-range parameter index!
            }
        }
        */
    }
    
    pub fn get_parameter_name_with_maxlen(
        &mut self, 
        index:                 i32,
        maximum_string_length: i32

    ) -> String {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->getName (maximumStringLength);

        return isPositiveAndBelow (index, getNumParameters()) ? getParameterName (index).substring (0, maximumStringLength)
                                                              : String();
        */
    }
    
    pub fn get_parameter_text(&mut self, index: i32) -> String {
        
        todo!();
        /*
            #if ALOE_DEBUG
        // if you hit this, then you're probably using the old parameter control methods,
        // but have forgotten to implement either of the getParameterText() methods.
        jassert (! textRecursionCheck);
        ScopedValueSetter<bool> sv (textRecursionCheck, true, false);
       #endif

        return isPositiveAndBelow (index, getNumParameters()) ? getParameterText (index, 1024)
                                                              : String();
        */
    }
    
    pub fn get_parameter_text_with_maxlen(
        &mut self, 
        index:                 i32,
        maximum_string_length: i32

    ) -> String {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->getText (p->getValue(), maximumStringLength);

        return isPositiveAndBelow (index, getNumParameters()) ? getParameterText (index).substring (0, maximumStringLength)
                                                              : String();
        */
    }
    
    pub fn get_num_parameters(&mut self) -> i32 {
        
        todo!();
        /*
            return getParameters().size();
        */
    }
    
    pub fn get_parameter(&mut self, index: i32) -> f32 {
        
        todo!();
        /*
            if (auto* p = getParamChecked (index))
            return p->getValue();

        return 0;
        */
    }
    
    pub fn set_parameter(&mut self, 
        index:     i32,
        new_value: f32)  {
        
        todo!();
        /*
            if (auto* p = getParamChecked (index))
            p->setValue (newValue);
        */
    }
    
    pub fn get_parameter_default_value(&mut self, index: i32) -> f32 {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->getDefaultValue();

        return 0;
        */
    }
    
    pub fn get_parameter_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
            if (auto* p = getParamChecked (index))
            return p->getName (512);

        return {};
        */
    }
    
    pub fn get_parameterid(&mut self, index: i32) -> String {
        
        todo!();
        /*
            // Don't use getParamChecked here, as this must also work for legacy plug-ins
        if (auto* p = dynamic_cast<AudioProcessorParameterWithID*> (getParameters()[index]))
            return p->paramID;

        return String (index);
        */
    }
    
    pub fn get_parameter_num_steps(&mut self, index: i32) -> i32 {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->getNumSteps();

        return AudioProcessor::getDefaultNumParameterSteps();
        */
    }
    
    pub fn is_parameter_discrete(&self, index: i32) -> bool {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->isDiscrete();

        return false;
        */
    }
    
    pub fn get_parameter_label(&self, index: i32) -> String {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->getLabel();

        return {};
        */
    }
    
    pub fn is_parameter_automatable(&self, index: i32) -> bool {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->isAutomatable();

        return true;
        */
    }
    
    pub fn is_parameter_orientation_inverted(&self, index: i32) -> bool {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->isOrientationInverted();

        return false;
        */
    }
    
    pub fn is_meta_parameter(&self, index: i32) -> bool {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->isMetaParameter();

        return false;
        */
    }
    
    pub fn get_parameter_category(&self, index: i32) -> AudioProcessorParameterCategory {
        
        todo!();
        /*
            if (auto* p = getParameters()[index])
            return p->getCategory();

        return AudioProcessorParameter::genericParameter;
        */
    }
    
    pub fn get_param_checked(&self, index: i32) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            auto p = getParameters()[index];

        // If you hit this, then you're either trying to access parameters that are out-of-range,
        // or you're not using addParameter and the managed parameter list, but have failed
        // to override some essential virtual methods and implement them appropriately.
        jassert (p != nullptr);
        return p;
        */
    }
}
