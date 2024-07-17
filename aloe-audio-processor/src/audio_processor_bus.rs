crate::ix!();

/**
  | Describes the layout and properties
  | of an audio bus.
  | 
  | Effectively a bus description is a named
  | set of channel types.
  | 
  | @see AudioChannelSet, AudioProcessor::addBus
  |
  */
#[no_copy]
pub struct AudioProcessorBus<'a> {
    owner:                &'a mut AudioProcessor<'a>,
    name:                 String,
    layout:               AudioChannelSet,
    dflt_layout:          AudioChannelSet,
    last_layout:          AudioChannelSet,
    enabled_by_default:   bool,
    cached_channel_count: i32,
}

impl<'a> AudioProcessorBus<'a> {

    /**
      | Returns true if the current bus is the
      | main input or output bus.
      |
      */
    pub fn is_main(&self) -> bool {
        
        todo!();
        /*
            return getBusIndex() == 0;
        */
    }
        
    /**
      | The bus's name.
      |
      */
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    /**
      | Get the default layout of this bus. @see
      | AudioChannelSet
      |
      */
    pub fn get_default_layout(&self) -> &AudioChannelSet {
        
        todo!();
        /*
            return dfltLayout;
        */
    }
        
    /**
      | The bus's current layout. This will
      | be AudioChannelSet::disabled() if
      | the current layout is disabled. @see
      | AudioChannelSet
      |
      */
    pub fn get_current_layout(&self) -> &AudioChannelSet {
        
        todo!();
        /*
            return layout;
        */
    }

    /**
      | Return the bus's last active channel
      | layout.
      | 
      | If the bus is currently enabled then
      | the result will be identical to getCurrentLayout
      | otherwise it will return the last enabled
      | layout. @see AudioChannelSet
      |
      */
    pub fn get_last_enabled_layout(&self) -> &AudioChannelSet {
        
        todo!();
        /*
            return lastLayout;
        */
    }

    /**
      | Return the number of channels of the
      | current bus.
      |
      */
    #[inline] pub fn get_number_of_channels(&self) -> i32 {
        
        todo!();
        /*
            return cachedChannelCount;
        */
    }

    /**
      | Returns true if the current bus is enabled.
      |
      */
    pub fn is_enabled(&self) -> bool {
        
        todo!();
        /*
            return ! layout.isDisabled();
        */
    }

    /**
      | Returns if this bus is enabled by default.
      |
      */
    pub fn is_enabled_by_default(&self) -> bool {
        
        todo!();
        /*
            return enabledByDefault;
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
    pub fn get_bus_buffer<FloatType>(&self, process_block_buffer: &mut AudioBuffer<FloatType>) -> AudioBuffer<FloatType> {
    
        todo!();
        /*
            auto di = getDirectionAndIndex();
                return owner.getBusBuffer (processBlockBuffer, di.isInput, di.index);
        */
    }
    
    pub fn new(
        processor:       &mut AudioProcessor,
        bus_name:        &String,
        default_layout:  &AudioChannelSet,
        is_dflt_enabled: bool

    ) -> Self {
    
        todo!();
        /*

            : owner (processor), name (busName),
          layout (isDfltEnabled ? defaultLayout : AudioChannelSet()),
          dfltLayout (defaultLayout), lastLayout (defaultLayout),
          enabledByDefault (isDfltEnabled)

        // Your default layout cannot be disabled
        jassert (! dfltLayout.isDisabled());
        */
    }
    
    /**
      | Returns true if this bus is an input bus.
      |
      */
    pub fn is_input(&self) -> bool {
        
        todo!();
        /*
            return owner.inputBuses.contains (this);
        */
    }
    
    /**
      | Returns the index of this bus.
      |
      */
    pub fn get_bus_index(&self) -> i32 {
        
        todo!();
        /*
            return getDirectionAndIndex().index;
        */
    }
    
    pub fn get_direction_and_index(&self) -> AudioProcessorBusDirectionAndIndex {
        
        todo!();
        /*
            BusDirectionAndIndex di;
        di.index = owner.inputBuses.indexOf (this);
        di.isInput = (di.index >= 0);

        if (! di.isInput)
            di.index = owner.outputBuses.indexOf (this);

        return di;
        */
    }
    
    /**
      | Sets the bus's current layout.
      | 
      | If the AudioProcessor does not support
      | this layout then this will return false.
      | @see AudioChannelSet
      |
      */
    pub fn set_current_layout(&mut self, bus_layout: &AudioChannelSet) -> bool {
        
        todo!();
        /*
            auto di = getDirectionAndIndex();
        return owner.setChannelLayoutOfBus (di.isInput, di.index, busLayout);
        */
    }
    
    /**
      | Sets the bus's current layout without
      | changing the enabled state.
      | 
      | If the AudioProcessor does not support
      | this layout then this will return false.
      | @see AudioChannelSet
      |
      */
    pub fn set_current_layout_without_enabling(&mut self, set: &AudioChannelSet) -> bool {
        
        todo!();
        /*
            if (! set.isDisabled())
        {
            if (isEnabled())
                return setCurrentLayout (set);

            if (isLayoutSupported (set))
            {
                lastLayout = set;
                return true;
            }

            return false;
        }

        return isLayoutSupported (set);
        */
    }
    
    /**
      | Set the number of channels of this bus.
      | This will return false if the AudioProcessor
      | does not support this layout.
      |
      */
    pub fn set_number_of_channels(&mut self, channels: i32) -> bool {
        
        todo!();
        /*
            auto di = getDirectionAndIndex();

        if (owner.setChannelLayoutOfBus (di.isInput, di.index, AudioChannelSet::canonicalChannelSet (channels)))
            return true;

        if (channels == 0)
            return false;

        auto namedSet = AudioChannelSet::namedChannelSet (channels);

        if (! namedSet.isDisabled() && owner.setChannelLayoutOfBus (di.isInput, di.index, namedSet))
            return true;

        return owner.setChannelLayoutOfBus (di.isInput, di.index, AudioChannelSet::discreteChannels (channels));
        */
    }
    
    /**
      | Enable or disable this bus. This will
      | return false if the AudioProcessor
      | does not support disabling this bus.
      |
      */
    pub fn enable(&mut self, should_enable: Option<bool>) -> bool {

        let should_enable: bool = should_enable.unwrap_or(true);
        
        todo!();
        /*
            if (isEnabled() == shouldEnable)
            return true;

        return setCurrentLayout (shouldEnable ? lastLayout : AudioChannelSet::disabled());
        */
    }
    
    /**
      | Returns the maximum number of channels
      | that this bus can support.
      | 
      | -----------
      | @param limit
      | 
      | The maximum value to return.
      |
      */
    pub fn get_max_supported_channels(&self, limit: Option<usize>) -> i32 {

        let limit = limit.unwrap_or(AUDIO_CHANNEL_SET_MAX_CHANNELS_OF_NAMED_LAYOUT);
        
        todo!();
        /*
            for (int ch = limit; ch > 0; --ch)
            if (isNumberOfChannelsSupported (ch))
                return ch;

        return (isMain() && isLayoutSupported (AudioChannelSet::disabled())) ? 0 : -1;
        */
    }
    
    /**
      | Checks if a particular layout is supported.
      | 
      | -----------
      | @param set
      | 
      | The AudioChannelSet which is to be probed.
      | ----------
      | @param currentLayout
      | 
      | If non-null, pretend that the current
      | layout of the AudioProcessor is currentLayout.
      | On exit, currentLayout will be modified
      | to to represent the buses layouts of
      | the AudioProcessor as if the layout
      | of the receiver had been successfully
      | changed. This is useful as changing
      | the layout of the receiver may change
      | the bus layout of other buses.
      | 
      | @see AudioChannelSet
      |
      */
    pub fn is_layout_supported(
        &self, 
        set:       &AudioChannelSet,
        io_layout: *mut AudioProcessorBusesLayout

    ) -> bool {
        
        todo!();
        /*
            auto di = getDirectionAndIndex();

        // check that supplied ioLayout is actually valid
        if (ioLayout != nullptr)
        {
            if (! owner.checkBusesLayoutSupported (*ioLayout))
            {
                *ioLayout = owner.getBusesLayout();

                // the current layout you supplied is not a valid layout
                jassertfalse;
            }
        }

        auto currentLayout = (ioLayout != nullptr ? *ioLayout : owner.getBusesLayout());
        auto& actualBuses = (di.isInput ? currentLayout.inputBuses : currentLayout.outputBuses);

        if (actualBuses.getReference (di.index) == set)
            return true;

        auto desiredLayout = currentLayout;

        (di.isInput ? desiredLayout.inputBuses
                    : desiredLayout.outputBuses).getReference (di.index) = set;

        owner.getNextBestLayout (desiredLayout, currentLayout);

        if (ioLayout != nullptr)
            *ioLayout = currentLayout;

        // Nearest layout has a different number of buses. Aloe plug-ins MUST
        // have fixed number of buses.
        jassert (currentLayout.inputBuses. size() == owner.getBusCount (true)
              && currentLayout.outputBuses.size() == owner.getBusCount (false));

        return actualBuses.getReference (di.index) == set;
        */
    }
    
    /**
      | Checks if this bus can support a given
      | number of channels.
      |
      */
    pub fn is_number_of_channels_supported(&self, channels: i32) -> bool {
        
        todo!();
        /*
            if (channels == 0)
            return isLayoutSupported(AudioChannelSet::disabled());

        auto set = supportedLayoutWithChannels (channels);
        return (! set.isDisabled()) && isLayoutSupported (set);
        */
    }
    
    /**
      | Returns a ChannelSet that the bus supports
      | with a given number of channels.
      |
      */
    pub fn supported_layout_with_channels(&self, channels: i32) -> AudioChannelSet {
        
        todo!();
        /*
            if (channels == 0)
            return AudioChannelSet::disabled();

        {
            AudioChannelSet set;

            if (! (set = AudioChannelSet::namedChannelSet  (channels)).isDisabled() && isLayoutSupported (set))
                return set;

            if (! (set = AudioChannelSet::discreteChannels (channels)).isDisabled() && isLayoutSupported (set))
                return set;
        }

        for (auto& set : AudioChannelSet::channelSetsWithNumberOfChannels (channels))
            if (isLayoutSupported (set))
                return set;

        return AudioChannelSet::disabled();
        */
    }
    
    /**
      | Returns the resulting layouts of all
      | buses after changing the layout of this
      | bus.
      | 
      | Changing an individual layout of a bus
      | may also change the layout of all the
      | other buses. This method returns what
      | the layouts of all the buses of the audio
      | processor would be, if you were to change
      | the layout of this bus to the given layout.
      | If there is no way to support the given
      | layout then this method will return
      | the next best layout.
      |
      */
    pub fn get_buses_layout_for_layout_change_of_bus(&self, set: &AudioChannelSet) -> AudioProcessorBusesLayout {
        
        todo!();
        /*
            auto layouts = owner.getBusesLayout();
        isLayoutSupported (set, &layouts);
        return layouts;
        */
    }
    
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
    pub fn get_channel_index_in_process_block_buffer(&self, channel_index: i32) -> i32 {
        
        todo!();
        /*
            auto di = getDirectionAndIndex();
        return owner.getChannelIndexInProcessBlockBuffer (di.isInput, di.index, channelIndex);
        */
    }
    
    pub fn update_channel_count(&mut self)  {
        
        todo!();
        /*
            cachedChannelCount = layout.size();
        */
    }
}
