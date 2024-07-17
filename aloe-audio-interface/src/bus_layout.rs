crate::ix!();

pub trait GetDefaultLayout {

    /**
      | Get the default layout of this bus. @see
      | AudioChannelSet
      |
      */
    fn get_default_layout(&self) -> &dyn AudioChannelSetInterface;
}

pub trait GetCurrentLayout {

    /**
      | The bus's current layout. This will
      | be AudioChannelSet::disabled() if
      | the current layout is disabled. @see
      | AudioChannelSet
      |
      */
    fn get_current_layout(&self) -> &dyn AudioChannelSetInterface;
}

pub trait GetLastEnabledLayout {

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
    fn get_last_enabled_layout(&self) -> &dyn AudioChannelSetInterface;
}

pub trait SetCurrentLayout {

    /**
      | Sets the bus's current layout.
      | 
      | If the AudioProcessor does not support
      | this layout then this will return false.
      | @see AudioChannelSet
      |
      */
    fn set_current_layout(&mut self, layout: &dyn AudioChannelSetInterface) -> bool;
}

pub trait SetCurrentLayoutWithoutEnabling {

    /**
      | Sets the bus's current layout without
      | changing the enabled state.
      | 
      | If the AudioProcessor does not support
      | this layout then this will return false.
      | @see AudioChannelSet
      |
      */
    fn set_current_layout_without_enabling(&mut self, layout: &dyn AudioChannelSetInterface) -> bool;
}

pub trait IsLayoutSupported {

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
    fn is_layout_supported(
        &self, 
        set:            &dyn AudioChannelSetInterface,
        current_layout: *mut dyn AudioProcessorBusesLayoutInterface
    ) -> bool;
}

pub trait SupportedLayoutWithChannels {

    /**
      | Returns a ChannelSet that the bus supports
      | with a given number of channels.
      |
      */
    fn supported_layout_with_channels(&self, channels: i32) -> dyn AudioChannelSetInterface;
}

pub trait GetBusesLayoutForLayoutChangeOfBus {

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
    fn get_buses_layout_for_layout_change_of_bus(&self, set: &dyn AudioChannelSetInterface) -> dyn AudioProcessorBusesLayoutInterface;
}
