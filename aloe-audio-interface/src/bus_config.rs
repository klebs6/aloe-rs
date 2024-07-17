crate::ix!();

pub trait GetAAXPluginIDForMainBusConfig {

    /**
      | AAX plug-ins need to report a unique
      | "plug-in id" for every audio layout
      | configuration that your AudioProcessor
      | supports on the main bus. Override this
      | function if you want your AudioProcessor
      | to use a custom "plug-in id" (for example
      | to stay backward compatible with older
      | versions of Aloe).
      | 
      | The default implementation will compute
      | a unique integer from the input and output
      | layout and add this value to the 4 character
      | code 'jcaa' (for native AAX) or 'jyaa'
      | (for AudioSuite plug-ins).
      |
      */
    fn get_aax_plugin_id_for_main_bus_config(&self, 
            main_input_layout:  &AudioChannelSet,
            main_output_layout: &AudioChannelSet,
            id_for_audio_suite: bool) -> i32;
}

pub trait CheckIsBusesLayoutSupported {

    /**
      | Callback to query if the AudioProcessor
      | supports a specific layout.
      | 
      | This callback is called when the host
      | probes the supported bus layouts via
      | the checkBusesLayoutSupported method.
      | You should override this callback if
      | you would like to limit the layouts that
      | your AudioProcessor supports. The
      | default implementation will accept
      | any layout. Aloe does basic sanity checks
      | so that the provided layouts parameter
      | will have the same number of buses as
      | your
      | 
      | AudioProcessor.
      | 
      | @see checkBusesLayoutSupported
      |
      */
    fn is_buses_layout_supported(&self, _0: &dyn AudioProcessorBusesLayoutInterface) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

pub trait CheckCanApplyBusesLayout {

    /**
      | Callback to check if a certain bus layout
      | can now be applied.
      | 
      | Most subclasses will not need to override
      | this method and should instead override
      | the isBusesLayoutSupported callback
      | to reject certain layout changes.
      | 
      | This callback is called when the user
      | requests a layout change. It will only
      | be called if processing of the AudioProcessor
      | has been stopped by a previous call to
      | releaseResources or after the construction
      | of the AudioProcessor. It will be called
      | just before the actual layout change.
      | By returning false you will abort the
      | layout change and setBusesLayout will
      | return false indicating that the layout
      | change was not successful.
      | 
      | The default implementation will simply
      | call isBusesLayoutSupported.
      | 
      | You only need to override this method
      | if there is a chance that your AudioProcessor
      | may not accept a layout although you
      | have previously claimed to support
      | it via the isBusesLayoutSupported
      | callback. This can occur if your AudioProcessor's
      | supported layouts depend on other plug-in
      | parameters which may have changed since
      | the last call to isBusesLayoutSupported,
      | such as the format of an audio file which
      | can be selected by the user in the AudioProcessor's
      | editor. This callback gives the
      | 
      | AudioProcessor a last chance to reject
      | a layout if conditions have changed
      | as it is always called just before the
      | actual layout change.
      | 
      | As it is never called while the AudioProcessor
      | is processing audio, it can also be used
      | for AudioProcessors which wrap other
      | plug-in formats to apply the current
      | layout to the underlying plug-in. This
      | callback gives such AudioProcessors
      | a chance to reject the layout change
      | should an error occur with the underlying
      | plug-in during the layout change.
      | 
      | @see isBusesLayoutSupported, setBusesLayout
      |
      */
    fn can_apply_buses_layout(&self, layouts: &dyn AudioProcessorBusesLayoutInterface) -> bool {
        
        todo!();
        /*
            return isBusesLayoutSupported (layouts);
        */
    }
}

pub trait ApplyBusLayouts {

    /**
      | This method will be called when a new
      | bus layout needs to be applied.
      | 
      | Most subclasses will not need to override
      | this method and should just use the default
      | implementation.
      |
      */
    fn apply_bus_layouts(&mut self, layouts: &dyn AudioProcessorBusesLayoutInterface) -> bool;
}

pub trait CanApplyBusCountChange {

    /**
      | Callback to query if adding/removing
      | buses currently possible.
      | 
      | This callback is called when the host
      | calls addBus or removeBus.
      | 
      | Similar to canApplyBusesLayout, this
      | callback is only called while the AudioProcessor
      | is stopped and gives the processor a
      | last chance to reject a requested bus
      | change. It can also be used to apply the
      | bus count change to an underlying wrapped
      | plug-in.
      | 
      | When adding a bus, isAddingBuses will
      | be true and the plug-in is expected to
      | fill out outNewBusProperties with
      | the properties of the bus which will
      | be created just after the successful
      | return of this callback.
      | 
      | Implementations of AudioProcessor
      | will rarely need to override this method.
      | Only override this method if your processor
      | supports adding and removing buses
      | and if it needs more fine grain control
      | over the naming of new buses or may reject
      | bus number changes although canAddBus
      | or canRemoveBus returned true.
      | 
      | The default implementation will return
      | false if canAddBus/canRemoveBus returns
      | false (the default behavior). Otherwise,
      | this method returns "Input #busIndex"
      | for input buses and "Output #busIndex"
      | for output buses where busIndex is the
      | index for newly created buses. The default
      | layout in this case will be the layout
      | of the previous bus of the same direction.
      |
      */
    fn can_apply_bus_count_change(
        &mut self, 
        is_input:               bool,
        is_adding_buses:        bool,
        out_new_bus_properties: &mut AudioProcessorBusProperties
    ) -> bool;
}

pub trait GetBusCount {

    /**
      | Returns the number of buses on the input
      | or output side
      |
      */
    fn get_bus_count(&self, is_input: bool) -> i32;
}

pub trait SetBusesLayout {

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
    fn set_buses_layout(&mut self, _0: &dyn AudioProcessorBusesLayoutInterface) -> bool;
}

pub trait SetBusesLayoutWithoutEnabling {

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
    fn set_buses_layout_without_enabling(&mut self, _0: &dyn AudioProcessorBusesLayoutInterface) -> bool;
}

pub trait GetBusesLayout {

    /**
      | Provides the current channel layouts
      | of this audio processor.
      |
      */
    fn get_buses_layout(&self) -> Box<dyn AudioProcessorBusesLayoutInterface>;
}

pub trait GetChannelLayoutOfBus {

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
    fn get_channel_layout_of_bus(
        &self, 
        is_input:  bool,
        bus_index: i32
    ) -> AudioChannelSet;
}

pub trait SetChannelLayoutOfBus {

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
    fn set_channel_layout_of_bus(
        &mut self, 
        is_input:  bool,
        bus_index: i32,
        layout:    &AudioChannelSet
    ) -> bool;
}

pub trait GetChannelCountOfBus {

    /**
      | Provides the number of channels of the
      | bus with a given index and direction.
      | 
      | If the index, direction combination
      | is invalid then this will return zero.
      |
      */
    fn get_channel_count_of_bus(
        &self, 
        is_input:  bool,
        bus_index: i32
    ) -> i32;
}
