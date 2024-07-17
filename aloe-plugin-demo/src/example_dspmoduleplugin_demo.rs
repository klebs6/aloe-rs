crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/DSPModulePluginDemo.h]

lazy_static!{
    /*
    template <typename Func, typename... Items>
    constexpr void forEach (Func&& func, Items&&... items)
         ( (std::initializer_list<int> { (func (std::forward<Items> (items)), 0)... }))
    {
        (void) std::initializer_list<int> { ((void) func (std::forward<Items> (items)), 0)... };
    }
    */
}

pub fn add_all_and_make_visible<Components>(
        target:   &mut Component,
        children: &mut Components)  {

    todo!();
    /*
        forEach ([&] (Component& child) { target.addAndMakeVisible (child); }, children...);
    */
}

pub fn prepare_all<Processors>(
        spec:       &ProcessSpec,
        processors: &mut Processors)  {

    todo!();
    /*
        forEach ([&] (auto& proc) { proc.prepare (spec); }, processors...);
    */
}

pub fn reset_all<Processors>(processors: &mut Processors)  {

    todo!();
    /*
        forEach ([] (auto& proc) { proc.reset(); }, processors...);
    */
}

pub enum EffectsTabs
{
    TabDistortion = 1,
    TabConvolution,
    TabMultiBand,
    TabCompressor,
    TabNoiseGate,
    TabLimiter,
    TabDelayLineDirect,
    TabDelayLineEffect,
    TabPhaser,
    TabChorus,
    TabLadder
}

///-----------------------

/**
  | We use this enum to index into the chain
  | above
  |
  */
pub enum ProcessorIndices
{
    noiseGateIndex,
    inputGainIndex,
    directDelayIndex,
    multiBandIndex,
    compressorIndex,
    phaserIndex,
    chorusIndex,
    distortionIndex,
    ladderIndex,
    delayEffectIndex,
    convolutionIndex,
    limiterIndex,
    outputGainIndex,
    pannerIndex
}

pub type Chain<'a> = ProcessorChain/*<
    NoiseGate<f32>,
    Gain<f32>,
    DirectDelayProcessor,
    MultiBandProcessor,
    Compressor<f32>,
    Phaser<f32>,
    Chorus<f32>,
    DistortionProcessor,
    LadderFilter<f32>,
    DelayEffectProcessor,
    ConvolutionProcessor<'a>,
    Limiter<f32>,
    Gain<f32>,
    Panner<f32>
>*/;

#[no_copy]
#[leak_detector]
pub struct DspModulePluginDemo<'a> {

    /**
      | We store this here so that the editor
      | retains its state if it is closed and
      | reopened
      */
    index_tab:       i32, // default = 0
    parameters:      DspModulePluginDemoParameterReferences<'a>,
    apvts:           AudioProcessorValueTreeState<'a>,
    chain:           Chain<'a>,
    requires_update: AtomicBool, // default = { true  }
    ir_size:         Atomic<i32>, // default = { 0  }
}

impl<'a> ValueTreeListener for DspModulePluginDemo<'a> {

}

impl<'a> AudioProcessorInterface for DspModulePluginDemo<'a> {

}

impl<'a> Reset for DspModulePluginDemo<'a> {

    fn reset(&mut self)  {

        todo!();
        /*
           chain.reset();
           update();
           */
    }
}

impl<'a> SetCurrentProgram for DspModulePluginDemo<'a> {

    fn set_current_program(&mut self, _0: i32)  { }
}

impl<'a> GetProgramName for DspModulePluginDemo<'a> {

    fn get_program_name(&mut self, _0: i32) -> String {
        "None".to_string()
    }
}

impl<'a> GetProcessingPrecision for DspModulePluginDemo<'a> {

    fn get_processing_precision(&self) -> AudioProcessorProcessingPrecision { todo!() }
}

impl<'a> GetPlayHead for DspModulePluginDemo<'a> {

    fn get_play_head(&self) -> *mut (dyn AudioPlayHeadInterface + 'static) { todo!() }
}

impl<'a> GetParameters for DspModulePluginDemo<'a> {
    fn get_parameters(&self) -> &[*mut dyn AudioProcessorParameterInterface] { todo!() }
}

impl<'a> GetParameterTree for DspModulePluginDemo<'a> {

    fn get_parameter_tree(&self) -> &dyn AudioProcessorParameterGroupInterface { todo!() }
}

impl<'a> GetOffsetInBusBufferForAbsoluteChannelIndex for DspModulePluginDemo<'a> {

    fn get_offset_in_bus_buffer_for_absolute_channel_index(&self, _: bool, _: i32, _: &mut i32) -> i32 { todo!() }
}

impl<'a> GetNumPrograms for DspModulePluginDemo<'a> {

}

impl<'a> GetNextBestLayoutInLayoutList for DspModulePluginDemo<'a> {

    fn get_next_best_layout_in_layout_list(&mut self, _: &dyn AudioProcessorBusesLayoutInterface, _: &[&[i16]; 2]) 
        -> Box<dyn AudioProcessorBusesLayoutInterface> { todo!() }
}

impl<'a> GetName for DspModulePluginDemo<'a> {

    fn get_name(&self) -> String { todo!() }
}

impl<'a> GetMainBusNumOutputChannels for DspModulePluginDemo<'a> {
    fn get_main_bus_num_output_channels(&self) -> i32 { todo!() }
}

impl<'a> GetMainBusNumInputChannels for DspModulePluginDemo<'a> {
    fn get_main_bus_num_input_channels(&self) -> i32 { todo!() }
}

impl<'a> GetLatencySamples for DspModulePluginDemo<'a> {
    fn get_latency_samples(&self) -> i32 { todo!() }
}

impl<'a> GetDefaultNumParameterSteps for DspModulePluginDemo<'a> {
    fn get_default_num_parameter_steps() -> i32 { todo!() }
}

impl<'a> GetCurrentProgramStateInformation for DspModulePluginDemo<'a> {

    fn get_current_program_state_information(&mut self, _: &mut MemoryBlock) { todo!() }
}

impl<'a> GetCurrentProgram for DspModulePluginDemo<'a> {

}

impl<'a> GetChannelLayoutOfBus for DspModulePluginDemo<'a> {

    fn get_channel_layout_of_bus(&self, _: bool, _: i32) -> AudioChannelSet { todo!() }
}

impl<'a> GetBusChannelIndexInProcessBlockBuffer for DspModulePluginDemo<'a> {

    fn get_channel_index_in_process_block_buffer(&self, _: bool, _: i32, _: i32) -> i32 { todo!() }
}

impl<'a> GetChannelCountOfBus for DspModulePluginDemo<'a> {

    fn get_channel_count_of_bus(&self, _: bool, _: i32) -> i32 { todo!() }
}

impl<'a> GetCallbackLock for DspModulePluginDemo<'a> {

    fn get_callback_lock(&self) -> &CriticalSection { todo!() }
}

impl<'a> GetBypassParameter for DspModulePluginDemo<'a> {

}

impl<'a> GetBusesLayout for DspModulePluginDemo<'a> {
    fn get_buses_layout(&self) -> Box<(dyn AudioProcessorBusesLayoutInterface + 'static)> { todo!() }


}

impl<'a> GetBusCount for DspModulePluginDemo<'a> {
    fn get_bus_count(&self, _: bool) -> i32 { todo!() }
}

impl<'a> GetBus for DspModulePluginDemo<'a> {
    fn get_bus_mut(&mut self, _: bool, _: i32) -> *mut (dyn AudioProcessorBusInterface + 'static) { todo!() }
    fn get_bus(&self, _: bool, _: i32) -> *const (dyn AudioProcessorBusInterface + 'static) { todo!() }
}

impl<'a> GetBlockSize for DspModulePluginDemo<'a> {
    fn get_block_size(&self) -> i32 { todo!() }
}

impl<'a> GetAlternateDisplayNames for DspModulePluginDemo<'a> {
    fn get_alternate_display_names(&self) -> Vec<String> { todo!() }
}

impl<'a> GetActiveEditor for DspModulePluginDemo<'a> {
    fn get_active_editor(&self) -> *mut (dyn AudioProcessorEditorInterface + 'static) { todo!() }
}

impl<'a> GetAAXPluginIDForMainBusConfig for DspModulePluginDemo<'a> {
    fn get_aax_plugin_id_for_main_bus_config(&self, _: &AudioChannelSet, _: &AudioChannelSet, _: bool) -> i32 { todo!() }
}

impl<'a> EnableAllBuses for DspModulePluginDemo<'a> {
    fn enable_all_buses(&mut self) -> bool { todo!() }
}

impl<'a> EditorBeingDeleted for DspModulePluginDemo<'a> {
    fn editor_being_deleted(&mut self, _: *mut (dyn AudioProcessorEditorInterface + 'static)) { todo!() }
}

impl<'a> DisableNonMainBuses for DspModulePluginDemo<'a> {
    fn disable_non_main_buses(&mut self) -> bool { todo!() }
}

impl<'a> CreateEditorIfNeeded for DspModulePluginDemo<'a> {

    fn create_editor_if_needed(&mut self) -> *mut (dyn AudioProcessorEditorInterface + 'static) { todo!() }
}

impl<'a> CreateEditor for DspModulePluginDemo<'a> {

    fn create_editor(&mut self) -> *mut dyn AudioProcessorEditorInterface {

        todo!();
        /*
           return nullptr;
           */
    }
}

impl<'a> CopyXMLToBinary for DspModulePluginDemo<'a> {
    fn copy_xml_to_binary(_: &XmlElement, _: &mut MemoryBlock) { todo!() }


}

impl<'a> ContainsLayout for DspModulePluginDemo<'a> {
    fn contains_layout(_: &dyn AudioProcessorBusesLayoutInterface, _: &[[i16; 2]]) -> bool { todo!() }
    fn contains_layouts<const NUM_LAYOUTS: usize>(_: &dyn AudioProcessorBusesLayoutInterface, _: &[[i16; NUM_LAYOUTS]; 2]) -> bool { todo!() }
}

impl<'a> CheckSupportsMpe for DspModulePluginDemo<'a> {

}

impl<'a> CheckSupportsDoublePrecisionProcessing for DspModulePluginDemo<'a> {
    fn supports_double_precision_processing(&self) -> bool { todo!() }
}

impl<'a> CheckProducesMidi for DspModulePluginDemo<'a> {
    fn produces_midi(&self) -> bool { todo!() }
}

impl<'a> CheckIsMidiEffect for DspModulePluginDemo<'a> {

}

impl<'a> CheckIsBusesLayoutSupported for DspModulePluginDemo<'a> {

}

impl<'a> CheckHasEditor for DspModulePluginDemo<'a> {
    fn has_editor(&self) -> bool { todo!() }
}

impl<'a> CheckCanRemoveBus for DspModulePluginDemo<'a> {

}

impl<'a> CheckCanApplyBusesLayout for DspModulePluginDemo<'a> {

}

impl<'a> CheckCanAddBus for DspModulePluginDemo<'a> {

}

impl<'a> CheckBusesLayoutSupported for DspModulePluginDemo<'a> {
    fn check_buses_layout_supported(&self, _: &dyn AudioProcessorBusesLayoutInterface) -> bool { todo!() }
}

impl<'a> CheckAcceptsMidi for DspModulePluginDemo<'a> {
    fn accepts_midi(&self) -> bool { todo!() }
}

impl<'a> ChangeProgramName for DspModulePluginDemo<'a> {
    fn change_program_name(&mut self, _: i32, _: &str) { todo!() }
}

impl<'a> CanApplyBusCountChange for DspModulePluginDemo<'a> {
    fn can_apply_bus_count_change(&mut self, _: bool, _: bool, _: &mut AudioProcessorBusProperties) -> bool { todo!() }
}

impl<'a> BusesPropertiesFromLayoutArray for DspModulePluginDemo<'a> {
    fn buses_properties_from_layout_array(_: &[AudioProcessorInOutChannelPair]) -> Box<(dyn AudioProcessorBusesPropertiesInterface + 'static)> { todo!() }
}

impl<'a> ApplyBusLayouts for DspModulePluginDemo<'a> {
    fn apply_bus_layouts(&mut self, _: &dyn AudioProcessorBusesLayoutInterface) -> bool { todo!() }
}

impl<'a> AddParameterGroup for DspModulePluginDemo<'a> {
    fn add_parameter_group(&mut self, _: Box<(dyn AudioProcessorParameterGroupInterface + 'static)>) { todo!() }
}

impl<'a> AddParameter for DspModulePluginDemo<'a> {
    fn add_parameter(&mut self, _: *mut (dyn AudioProcessorParameterInterface + 'static)) { todo!() }
}

impl<'a> AddAudioProcessorListener for DspModulePluginDemo<'a> {
    fn add_listener(&mut self, _: *mut (dyn AudioProcessorListener + 'static)) { todo!() }
}

impl<'a> AudioProcessorAddBus for DspModulePluginDemo<'a> {
    fn add_bus(&mut self, _: bool) -> bool { todo!() }
}

impl<'a> SetStateInformation for DspModulePluginDemo<'a> {
    fn set_state_information(&mut self, _: *const aloe_deps::c_void, _: i32) { todo!() }
}

impl<'a> SetRateAndBufferSizeDetails for DspModulePluginDemo<'a> {
    fn set_rate_and_buffer_size_details(&mut self, _: f64, _: i32) { todo!() }
}

impl<'a> SetProcessingPrecision for DspModulePluginDemo<'a> {
    fn set_processing_precision(&mut self, _: AudioProcessorProcessingPrecision) { todo!() }
}

impl<'a> SetPlayHead for DspModulePluginDemo<'a> {
    fn set_play_head(&mut self, _: *mut (dyn AudioPlayHeadInterface + 'static)) { todo!() }
}

impl<'a> SetPlayConfigDetails for DspModulePluginDemo<'a> {
    fn set_play_config_details(&mut self, _: i32, _: i32, _: f64, _: i32) { todo!() }
}

impl<'a> SetParameterTree for DspModulePluginDemo<'a> {
    fn set_parameter_tree(&mut self, _: &dyn AudioProcessorParameterGroupInterface) { todo!() }
}

impl<'a> SetNonRealtime for DspModulePluginDemo<'a> {
    fn set_non_realtime(&mut self, _: bool) { todo!() }
}

impl<'a> SetLatencySamples for DspModulePluginDemo<'a> {
    fn set_latency_samples(&mut self, _: i32) { todo!() }
}

impl<'a> SetCurrentProgramStateInformation for DspModulePluginDemo<'a> {
    fn set_current_program_state_information(&mut self, _: *const aloe_deps::c_void, _: i32) { todo!() }
}

impl<'a> SetChannelLayoutOfBus for DspModulePluginDemo<'a> {
    fn set_channel_layout_of_bus(&mut self, _: bool, _: i32, _: &AudioChannelSet) -> bool { todo!() }
}

impl<'a> SetBusesLayoutWithoutEnabling for DspModulePluginDemo<'a> {
    fn set_buses_layout_without_enabling(&mut self, _: &dyn AudioProcessorBusesLayoutInterface) -> bool { todo!() }
}

impl<'a> SetBusesLayout for DspModulePluginDemo<'a> {
    fn set_buses_layout(&mut self, _: &dyn AudioProcessorBusesLayoutInterface) -> bool { todo!() }
}

impl<'a> SendParamChangeMessageToListeners for DspModulePluginDemo<'a> {
    fn send_param_change_message_to_listeners(&mut self, _: i32, _: f32) { todo!() }
}

impl<'a> RemoveAudioProcessorListener for DspModulePluginDemo<'a> {
    fn remove_listener(&mut self, _: *mut (dyn AudioProcessorListener + 'static)) { todo!() }
}

impl<'a> RemoveBus for DspModulePluginDemo<'a> {
    fn remove_bus(&mut self, _: bool) -> bool { todo!() }
}

impl<'a> ReleaseResources for DspModulePluginDemo<'a> {

    fn release_resources(&mut self)  { }
}

impl<'a> RefreshParameterList for DspModulePluginDemo<'a> {

    fn refresh_parameter_list(&mut self) { todo!() }
}

impl<'a> ProcessorLayoutsChanged for DspModulePluginDemo<'a> {

    fn processor_layouts_changed(&mut self) { todo!() }
}

impl<'a> ProcessBlockF64 for DspModulePluginDemo<'a> {

    fn process_block(&mut self, _: &mut aloe_buffers::AudioBuffer<f64>, _: &mut dyn MidiBufferInterface) { todo!() }
}

impl<'a> ProcessBlockBypassed for DspModulePluginDemo<'a> {

    fn process_block_bypassed(&mut self, _: &mut aloe_buffers::AudioBuffer<f32>, _: &mut dyn MidiBufferInterface) { todo!() }
}

impl<'a> ProcessBlock for DspModulePluginDemo<'a> {

    fn process_block(
        &mut self, 
        buffer: &mut AudioBuffer<f32>,
        _1:     &mut dyn MidiBufferInterface
    ) {
        
        todo!();
        /*
            if (jmax (getTotalNumInputChannels(), getTotalNumOutputChannels()) == 0)
                return;

            ScopedNoDenormals noDenormals;

            if (requiresUpdate.load())
                update();

            irSize = get<convolutionIndex> (chain).reverb.getCurrentIRSize();

            const auto totalNumInputChannels  = getTotalNumInputChannels();
            const auto totalNumOutputChannels = getTotalNumOutputChannels();

            setLatencySamples (get<convolutionIndex> (chain).getLatency()
                               + (isBypassed<distortionIndex> (chain) ? 0 : roundToInt (get<distortionIndex> (chain).getLatency())));

            const auto numChannels = jmax (totalNumInputChannels, totalNumOutputChannels);

            auto inoutBlock = AudioBlock<float> (buffer).getSubsetChannelBlock (0, (size_t) numChannels);
            chain.process (ProcessContextReplacing<float> (inoutBlock));
        */
    }
}

impl<'a> PrepareToPlay for DspModulePluginDemo<'a> {

    fn prepare_to_play(&mut self, _: f64, _: i32) { todo!() }
}

impl<'a> NumChannelsChanged for DspModulePluginDemo<'a> {

    fn num_channels_changed(&mut self) { todo!() }
}

impl<'a> NumBusesChanged for DspModulePluginDemo<'a> {

    fn num_buses_changed(&mut self) { todo!() }
}

impl<'a> MemoryWarningReceived for DspModulePluginDemo<'a> {

}

impl<'a> LayoutListToArray for DspModulePluginDemo<'a> {
    fn layout_lists_to_array<const NUM_LAYOUTS: usize>(_: &[[i16; NUM_LAYOUTS]; 2]) -> Vec<AudioProcessorInOutChannelPair> { todo!() }
    fn layout_list_to_array(_: &[[i16; 2]]) -> Vec<AudioProcessorInOutChannelPair> { todo!() }
}

impl<'a> IsUsingDoublePrecision for DspModulePluginDemo<'a> {

    fn is_using_double_precision(&self) -> bool { todo!() }
}

impl<'a> IsSuspended for DspModulePluginDemo<'a> {

    fn is_suspended(&self) -> bool { todo!() }
}

impl<'a> IsNonRealtime for DspModulePluginDemo<'a> {

    fn is_non_realtime(&self) -> bool { todo!() }
}

impl<'a> GetXMLFromBinary for DspModulePluginDemo<'a> {

    fn get_xml_from_binary(_: *const c_void, _: i32) -> Box<XmlElement> { todo!() }
}

impl<'a> GetWrapperTypeDescription for DspModulePluginDemo<'a> {

    fn get_wrapper_type_description(_: AudioProcessorWrapperType) -> *const u8 { todo!() }
}

impl<'a> GetTotalNumOutputChannels for DspModulePluginDemo<'a> {

    fn get_total_num_output_channels(&self) -> i32 { todo!() }
}

impl<'a> GetTotalNumInputChannels for DspModulePluginDemo<'a> {

    fn get_total_num_input_channels(&self) -> i32 { todo!() }
}

impl<'a> GetTailLengthSeconds for DspModulePluginDemo<'a> {

    fn get_tail_length_seconds(&self) -> f64 { todo!() }
}

impl<'a> GetStateInformation for DspModulePluginDemo<'a> {

    fn get_state_information(&mut self, _: &mut MemoryBlock) { todo!() }
}

impl<'a> GetSampleRate for DspModulePluginDemo<'a> {

    fn get_sample_rate(&self) -> f64 { todo!() }
}

impl<'a> GetResponseCurve for DspModulePluginDemo<'a> {

}

impl<'a> UpdateTrackProperties for DspModulePluginDemo<'a> {

    fn update_track_properties(&mut self, _: &AudioProcessorTrackProperties) { todo!() }
}

impl<'a> UpdateHostDisplay for DspModulePluginDemo<'a> {

    fn update_host_display(&mut self, _: &AudioProcessorListenerChangeDetails) { todo!() }
}

impl<'a> SuspendProcessing for DspModulePluginDemo<'a> {

    fn suspend_processing(&mut self, _: bool) { todo!() }
}

impl<'a> SetTypeOfNextNewPlugin for DspModulePluginDemo<'a> {

    fn set_type_of_next_new_plugin(_: AudioProcessorWrapperType) { todo!() }
}

pub struct DspModulePluginDemoLayoutAndReferences<'a>
{
    layout:     AudioProcessorValueTreeStateParameterLayout,
    references: DspModulePluginDemoParameterReferences<'a>,
}

pub type DspModulePluginDemoParameter = AudioProcessorValueTreeStateParameter;

/**
  | This struct holds references to the raw
  | parameters, so that we don't have to search
  | the APVTS (involving string comparisons and
  | map lookups!) every time a parameter
  | changes.
  */
pub struct DspModulePluginDemoParameterReferences<'a> {
    input_gain:                 &'a mut DspModulePluginDemoParameter,
    output_gain:                &'a mut DspModulePluginDemoParameter,
    pan:                        &'a mut DspModulePluginDemoParameter,
    distortion_enabled:         &'a mut AudioParameterBool,
    distortion_type:            &'a mut AudioParameterChoice,
    distortion_in_gain:         &'a mut DspModulePluginDemoParameter,
    distortion_lowpass:         &'a mut DspModulePluginDemoParameter,
    distortion_highpass:        &'a mut DspModulePluginDemoParameter,
    distortion_comp_gain:       &'a mut DspModulePluginDemoParameter,
    distortion_mix:             &'a mut DspModulePluginDemoParameter,
    distortion_oversampler:     &'a mut AudioParameterChoice,
    multi_band_enabled:         &'a mut AudioParameterBool,
    multi_band_freq:            &'a mut DspModulePluginDemoParameter,
    multi_band_low_volume:      &'a mut DspModulePluginDemoParameter,
    multi_band_high_volume:     &'a mut DspModulePluginDemoParameter,
    convolution_cab_enabled:    &'a mut AudioParameterBool,
    convolution_reverb_enabled: &'a mut AudioParameterBool,
    convolution_reverb_mix:     &'a mut DspModulePluginDemoParameter,
    compressor_enabled:         &'a mut AudioParameterBool,
    compressor_threshold:       &'a mut DspModulePluginDemoParameter,
    compressor_ratio:           &'a mut DspModulePluginDemoParameter,
    compressor_attack:          &'a mut DspModulePluginDemoParameter,
    compressor_release:         &'a mut DspModulePluginDemoParameter,
    noise_gate_enabled:         &'a mut AudioParameterBool,
    noise_gate_threshold:       &'a mut DspModulePluginDemoParameter,
    noise_gate_ratio:           &'a mut DspModulePluginDemoParameter,
    noise_gate_attack:          &'a mut DspModulePluginDemoParameter,
    noise_gate_release:         &'a mut DspModulePluginDemoParameter,
    limiter_enabled:            &'a mut AudioParameterBool,
    limiter_threshold:          &'a mut DspModulePluginDemoParameter,
    limiter_release:            &'a mut DspModulePluginDemoParameter,
    direct_delay_enabled:       &'a mut AudioParameterBool,
    direct_delay_type:          &'a mut AudioParameterChoice,
    direct_delay_value:         &'a mut DspModulePluginDemoParameter,
    direct_delay_smoothing:     &'a mut DspModulePluginDemoParameter,
    direct_delay_mix:           &'a mut DspModulePluginDemoParameter,
    delay_effect_enabled:       &'a mut AudioParameterBool,
    delay_effect_type:          &'a mut AudioParameterChoice,
    delay_effect_value:         &'a mut DspModulePluginDemoParameter,
    delay_effect_smoothing:     &'a mut DspModulePluginDemoParameter,
    delay_effect_lowpass:       &'a mut DspModulePluginDemoParameter,
    delay_effect_mix:           &'a mut DspModulePluginDemoParameter,
    delay_effect_feedback:      &'a mut DspModulePluginDemoParameter,
    phaser_enabled:             &'a mut AudioParameterBool,
    phaser_rate:                &'a mut DspModulePluginDemoParameter,
    phaser_depth:               &'a mut DspModulePluginDemoParameter,
    phaser_centre_frequency:    &'a mut DspModulePluginDemoParameter,
    phaser_feedback:            &'a mut DspModulePluginDemoParameter,
    phaser_mix:                 &'a mut DspModulePluginDemoParameter,
    chorus_enabled:             &'a mut AudioParameterBool,
    chorus_rate:                &'a mut DspModulePluginDemoParameter,
    chorus_depth:               &'a mut DspModulePluginDemoParameter,
    chorus_centre_delay:        &'a mut DspModulePluginDemoParameter,
    chorus_feedback:            &'a mut DspModulePluginDemoParameter,
    chorus_mix:                 &'a mut DspModulePluginDemoParameter,
    ladder_enabled:             &'a mut AudioParameterBool,
    ladder_mode:                &'a mut AudioParameterChoice,
    ladder_cutoff:              &'a mut DspModulePluginDemoParameter,
    ladder_resonance:           &'a mut DspModulePluginDemoParameter,
    ladder_drive:               &'a mut DspModulePluginDemoParameter,
}

impl<'a> DspModulePluginDemoParameterReferences<'a> {

    pub fn add_to_layout<Param>(
        layout: &mut AudioProcessorValueTreeStateParameterLayout,
        param:  Box<Param>) -> &mut Param {
    
        todo!();
        /*
            auto& ref = *param;
                layout.add (std::move (param));
                return ref;
        */
    }
    
    pub fn value_to_text_function(x: f32) -> String {
        
        todo!();
        /*
            return String (x, 2);
        */
    }
    
    pub fn text_to_value_function(str_: &String) -> f32 {
        
        todo!();
        /*
            return str.getFloatValue();
        */
    }
    
    pub fn value_to_text_pan_function(x: f32) -> String {
        
        todo!();
        /*
            return getPanningTextForValue ((x + 100.0f) / 200.0f);
        */
    }
    
    pub fn text_to_value_pan_function(str_: &String) -> f32 {
        
        todo!();
        /*
            return getPanningValueForText (str) * 200.0f - 100.0f;
        */
    }

    /**
      | Creates parameters, adds them to the
      | layout, and stores references to the
      | parameters in this struct.
      */
    pub fn new(layout: &mut AudioProcessorValueTreeStateParameterLayout) -> Self {
    
        todo!();
        /*


            : inputGain (addToLayout (layout,
                                          std::make_unique<DspModulePluginDemoParameter> (ID::inputGain,
                                                                       "Input",
                                                                       "dB",
                                                                       NormalisableRange<float> (-40.0f, 40.0f),
                                                                       0.0f,
                                                                       valueToTextFunction,
                                                                       textToValueFunction))),
                  outputGain (addToLayout (layout,
                                           std::make_unique<DspModulePluginDemoParameter> (ID::outputGain,
                                                                        "Output",
                                                                        "dB",
                                                                        NormalisableRange<float> (-40.0f, 40.0f),
                                                                        0.0f,
                                                                        valueToTextFunction,
                                                                        textToValueFunction))),
                  pan (addToLayout (layout,
                                    std::make_unique<DspModulePluginDemoParameter> (ID::pan,
                                                                 "Panning",
                                                                 "",
                                                                 NormalisableRange<float> (-100.0f, 100.0f),
                                                                 0.0f,
                                                                 valueToTextPanFunction,
                                                                 textToValuePanFunction))),
                  distortionEnabled (addToLayout (layout,
                                                  std::make_unique<AudioParameterBool> (ID::distortionEnabled,
                                                                                        "Distortion",
                                                                                        true,
                                                                                        ""))),
                  distortionType (addToLayout (layout,
                                               std::make_unique<AudioParameterChoice> (ID::distortionType,
                                                                                       "Waveshaper",
                                                                                       StringArray { "std::tanh", "Approx. tanh" },
                                                                                       0))),
                  distortionInGain (addToLayout (layout,
                                                 std::make_unique<DspModulePluginDemoParameter> (ID::distortionInGain,
                                                                              "Gain",
                                                                              "dB",
                                                                              NormalisableRange<float> (-40.0f, 40.0f),
                                                                              0.0f,
                                                                              valueToTextFunction,
                                                                              textToValueFunction))),
                  distortionLowpass (addToLayout (layout,
                                                  std::make_unique<DspModulePluginDemoParameter> (ID::distortionLowpass,
                                                                               "Post Low-pass",
                                                                               "Hz",
                                                                               NormalisableRange<float> (20.0f, 22000.0f, 0.0f, 0.25f),
                                                                               22000.0f,
                                                                               valueToTextFunction,
                                                                               textToValueFunction))),
                  distortionHighpass (addToLayout (layout,
                                                   std::make_unique<DspModulePluginDemoParameter> (ID::distortionHighpass,
                                                                                "Pre High-pass",
                                                                                "Hz",
                                                                                NormalisableRange<float> (20.0f, 22000.0f, 0.0f, 0.25f),
                                                                                20.0f,
                                                                                valueToTextFunction,
                                                                                textToValueFunction))),
                  distortionCompGain (addToLayout (layout,
                                                   std::make_unique<DspModulePluginDemoParameter> (ID::distortionCompGain,
                                                                                "Compensat.",
                                                                                "dB",
                                                                                NormalisableRange<float> (-40.0f, 40.0f),
                                                                                0.0f,
                                                                                valueToTextFunction,
                                                                                textToValueFunction))),
                  distortionMix (addToLayout (layout,
                                              std::make_unique<DspModulePluginDemoParameter> (ID::distortionMix,
                                                                           "Mix",
                                                                           "%",
                                                                           NormalisableRange<float> (0.0f, 100.0f),
                                                                           100.0f,
                                                                           valueToTextFunction,
                                                                           textToValueFunction))),
                  distortionOversampler (addToLayout (layout,
                                                      std::make_unique<AudioParameterChoice> (ID::distortionOversampler,
                                                                                              "Oversampling",
                                                                                              StringArray { "2X", "4X", "8X", "2X compensated", "4X compensated", "8X compensated" },
                                                                                              1))),
                  multiBandEnabled (addToLayout (layout,
                                                 std::make_unique<AudioParameterBool> (ID::multiBandEnabled,
                                                                                       "Multi-band",
                                                                                       false,
                                                                                       ""))),
                  multiBandFreq (addToLayout (layout,
                                              std::make_unique<DspModulePluginDemoParameter> (ID::multiBandFreq,
                                                                           "Sep. Freq.",
                                                                           "Hz",
                                                                           NormalisableRange<float> (20.0f, 22000.0f, 0.0f, 0.25f),
                                                                           2000.0f,
                                                                           valueToTextFunction,
                                                                           textToValueFunction))),
                  multiBandLowVolume (addToLayout (layout,
                                                   std::make_unique<DspModulePluginDemoParameter> (ID::multiBandLowVolume,
                                                                                "Low volume",
                                                                                "dB",
                                                                                NormalisableRange<float> (-40.0f, 40.0f),
                                                                                0.0f,
                                                                                valueToTextFunction,
                                                                                textToValueFunction))),
                  multiBandHighVolume (addToLayout (layout,
                                                    std::make_unique<DspModulePluginDemoParameter> (ID::multiBandHighVolume,
                                                                                 "High volume",
                                                                                 "dB",
                                                                                 NormalisableRange<float> (-40.0f, 40.0f),
                                                                                 0.0f,
                                                                                 valueToTextFunction,
                                                                                 textToValueFunction))),
                  convolutionCabEnabled (addToLayout (layout,
                                                      std::make_unique<AudioParameterBool> (ID::convolutionCabEnabled,
                                                                                            "Cabinet",
                                                                                            false,
                                                                                            ""))),
                  convolutionReverbEnabled (addToLayout (layout,
                                                         std::make_unique<AudioParameterBool> (ID::convolutionReverbEnabled,
                                                                                               "Reverb",
                                                                                               false,
                                                                                               ""))),
                  convolutionReverbMix (addToLayout (layout,
                                                     std::make_unique<DspModulePluginDemoParameter> (ID::convolutionReverbMix,
                                                                                  "Reverb Mix",
                                                                                  "%",
                                                                                  NormalisableRange<float> (0.0f, 100.0f),
                                                                                  50.0f,
                                                                                  valueToTextFunction,
                                                                                  textToValueFunction))),
                  compressorEnabled (addToLayout (layout,
                                                  std::make_unique<AudioParameterBool> (ID::compressorEnabled,
                                                                                        "Comp.",
                                                                                        false,
                                                                                        ""))),
                  compressorThreshold (addToLayout (layout,
                                                    std::make_unique<DspModulePluginDemoParameter> (ID::compressorThreshold,
                                                                                 "Threshold",
                                                                                 "dB",
                                                                                 NormalisableRange<float> (-100.0f, 0.0f),
                                                                                 0.0f,
                                                                                 valueToTextFunction,
                                                                                 textToValueFunction))),
                  compressorRatio (addToLayout (layout,
                                                std::make_unique<DspModulePluginDemoParameter> (ID::compressorRatio,
                                                                             "Ratio",
                                                                             ":1",
                                                                             NormalisableRange<float> (1.0f, 100.0f, 0.0f, 0.25f),
                                                                             1.0f,
                                                                             valueToTextFunction,
                                                                             textToValueFunction))),
                  compressorAttack (addToLayout (layout,
                                                 std::make_unique<DspModulePluginDemoParameter> (ID::compressorAttack,
                                                                              "Attack",
                                                                              "ms",
                                                                              NormalisableRange<float> (0.01f, 1000.0f, 0.0f, 0.25f),
                                                                              1.0f,
                                                                              valueToTextFunction,
                                                                              textToValueFunction))),
                  compressorRelease (addToLayout (layout,
                                                  std::make_unique<DspModulePluginDemoParameter> (ID::compressorRelease,
                                                                               "Release",
                                                                               "ms",
                                                                               NormalisableRange<float> (10.0f, 10000.0f, 0.0f, 0.25f),
                                                                               100.0f,
                                                                               valueToTextFunction,
                                                                               textToValueFunction))),
                  noiseGateEnabled (addToLayout (layout,
                                                 std::make_unique<AudioParameterBool> (ID::noiseGateEnabled,
                                                                                       "Gate",
                                                                                       false,
                                                                                       ""))),
                  noiseGateThreshold (addToLayout (layout,
                                                   std::make_unique<DspModulePluginDemoParameter> (ID::noiseGateThreshold,
                                                                                "Threshold",
                                                                                "dB",
                                                                                NormalisableRange<float> (-100.0f, 0.0f),
                                                                                -100.0f,
                                                                                valueToTextFunction,
                                                                                textToValueFunction))),
                  noiseGateRatio (addToLayout (layout,
                                               std::make_unique<DspModulePluginDemoParameter> (ID::noiseGateRatio,
                                                                            "Ratio",
                                                                            ":1",
                                                                            NormalisableRange<float> (1.0f, 100.0f, 0.0f, 0.25f),
                                                                            10.0f,
                                                                            valueToTextFunction,
                                                                            textToValueFunction))),
                  noiseGateAttack (addToLayout (layout,
                                                std::make_unique<DspModulePluginDemoParameter> (ID::noiseGateAttack,
                                                                             "Attack",
                                                                             "ms",
                                                                             NormalisableRange<float> (0.01f, 1000.0f, 0.0f, 0.25f),
                                                                             1.0f,
                                                                             valueToTextFunction,
                                                                             textToValueFunction))),
                  noiseGateRelease (addToLayout (layout,
                                                 std::make_unique<DspModulePluginDemoParameter> (ID::noiseGateRelease,
                                                                              "Release",
                                                                              "ms",
                                                                              NormalisableRange<float> (10.0f, 10000.0f, 0.0f, 0.25f),
                                                                              100.0f,
                                                                              valueToTextFunction,
                                                                              textToValueFunction))),
                  limiterEnabled (addToLayout (layout,
                                               std::make_unique<AudioParameterBool> (ID::limiterEnabled,
                                                                                     "Limiter",
                                                                                     false,
                                                                                     ""))),
                  limiterThreshold (addToLayout (layout,
                                                 std::make_unique<DspModulePluginDemoParameter> (ID::limiterThreshold,
                                                                              "Threshold",
                                                                              "dB",
                                                                              NormalisableRange<float> (-40.0f, 0.0f),
                                                                              0.0f,
                                                                              valueToTextFunction,
                                                                              textToValueFunction))),
                  limiterRelease (addToLayout (layout,
                                               std::make_unique<DspModulePluginDemoParameter> (ID::limiterRelease,
                                                                            "Release",
                                                                            "ms",
                                                                            NormalisableRange<float> (10.0f, 10000.0f, 0.0f, 0.25f),
                                                                            100.0f,
                                                                            valueToTextFunction,
                                                                            textToValueFunction))),
                  directDelayEnabled (addToLayout (layout,
                                                   std::make_unique<AudioParameterBool> (ID::directDelayEnabled,
                                                                                         "DL Dir.",
                                                                                         false,
                                                                                         ""))),
                  directDelayType (addToLayout (layout,
                                                std::make_unique<AudioParameterChoice> (ID::directDelayType,
                                                                                        "DL Type",
                                                                                        StringArray { "None", "Linear", "Lagrange", "Thiran" },
                                                                                        1))),
                  directDelayValue (addToLayout (layout,
                                                 std::make_unique<DspModulePluginDemoParameter> (ID::directDelayValue,
                                                                              "Delay",
                                                                              "smps",
                                                                              NormalisableRange<float> (0.0f, 44100.0f),
                                                                              0.0f,
                                                                              valueToTextFunction,
                                                                              textToValueFunction))),
                  directDelaySmoothing (addToLayout (layout,
                                                     std::make_unique<DspModulePluginDemoParameter> (ID::directDelaySmoothing,
                                                                                  "Smooth",
                                                                                  "ms",
                                                                                  NormalisableRange<float> (20.0f, 10000.0f, 0.0f, 0.25f),
                                                                                  200.0f,
                                                                                  valueToTextFunction,
                                                                                  textToValueFunction))),
                  directDelayMix (addToLayout (layout,
                                               std::make_unique<DspModulePluginDemoParameter> (ID::directDelayMix,
                                                                            "Delay Mix",
                                                                            "%",
                                                                            NormalisableRange<float> (0.0f, 100.0f),
                                                                            50.0f,
                                                                            valueToTextFunction,
                                                                            textToValueFunction))),
                  delayEffectEnabled (addToLayout (layout,
                                                   std::make_unique<AudioParameterBool> (ID::delayEffectEnabled,
                                                                                         "DL Effect",
                                                                                         false,
                                                                                         ""))),
                  delayEffectType (addToLayout (layout,
                                                std::make_unique<AudioParameterChoice> (ID::delayEffectType,
                                                                                        "DL Type",
                                                                                        StringArray { "None", "Linear", "Lagrange", "Thiran" },
                                                                                        1))),
                  delayEffectValue (addToLayout (layout,
                                                 std::make_unique<DspModulePluginDemoParameter> (ID::delayEffectValue,
                                                                              "Delay",
                                                                              "ms",
                                                                              NormalisableRange<float> (0.01f, 1000.0f),
                                                                              100.0f,
                                                                              valueToTextFunction,
                                                                              textToValueFunction))),
                  delayEffectSmoothing (addToLayout (layout,
                                                     std::make_unique<DspModulePluginDemoParameter> (ID::delayEffectSmoothing,
                                                                                  "Smooth",
                                                                                  "ms",
                                                                                  NormalisableRange<float> (20.0f, 10000.0f, 0.0f, 0.25f),
                                                                                  400.0f,
                                                                                  valueToTextFunction,
                                                                                  textToValueFunction))),
                  delayEffectLowpass (addToLayout (layout,
                                                   std::make_unique<DspModulePluginDemoParameter> (ID::delayEffectLowpass,
                                                                                "Low-pass",
                                                                                "Hz",
                                                                                NormalisableRange<float> (20.0f, 22000.0f, 0.0f, 0.25f),
                                                                                22000.0f,
                                                                                valueToTextFunction,
                                                                                textToValueFunction))),
                  delayEffectMix (addToLayout (layout,
                                               std::make_unique<DspModulePluginDemoParameter> (ID::delayEffectMix,
                                                                            "Delay Mix",
                                                                            "%",
                                                                            NormalisableRange<float> (0.0f, 100.0f),
                                                                            50.0f,
                                                                            valueToTextFunction,
                                                                            textToValueFunction))),
                  delayEffectFeedback (addToLayout (layout,
                                                    std::make_unique<DspModulePluginDemoParameter> (ID::delayEffectFeedback,
                                                                                 "Feedback",
                                                                                 "dB",
                                                                                 NormalisableRange<float> (-100.0f, 0.0f),
                                                                                 -100.0f,
                                                                                 valueToTextFunction,
                                                                                 textToValueFunction))),
                  phaserEnabled (addToLayout (layout,
                                              std::make_unique<AudioParameterBool> (ID::phaserEnabled,
                                                                                    "Phaser",
                                                                                    false,
                                                                                    ""))),
                  phaserRate (addToLayout (layout,
                                           std::make_unique<DspModulePluginDemoParameter> (ID::phaserRate,
                                                                        "Rate",
                                                                        "Hz",
                                                                        NormalisableRange<float> (0.05f, 20.0f, 0.0f, 0.25f),
                                                                        1.0f,
                                                                        valueToTextFunction,
                                                                        textToValueFunction))),
                  phaserDepth (addToLayout (layout,
                                            std::make_unique<DspModulePluginDemoParameter> (ID::phaserDepth,
                                                                         "Depth",
                                                                         "%",
                                                                         NormalisableRange<float> (0.0f, 100.0f),
                                                                         50.0f,
                                                                         valueToTextFunction,
                                                                         textToValueFunction))),
                  phaserCentreFrequency (addToLayout (layout,
                                                      std::make_unique<DspModulePluginDemoParameter> (ID::phaserCentreFrequency,
                                                                                   "Center",
                                                                                   "Hz",
                                                                                   NormalisableRange<float> (20.0f, 20000.0f, 0.0f, 0.25f),
                                                                                   600.0f,
                                                                                   valueToTextFunction,
                                                                                   textToValueFunction))),
                  phaserFeedback (addToLayout (layout,
                                               std::make_unique<DspModulePluginDemoParameter> (ID::phaserFeedback,
                                                                            "Feedback",
                                                                            "%",
                                                                            NormalisableRange<float> (0.0f, 100.0f),
                                                                            50.0f,
                                                                            valueToTextFunction,
                                                                            textToValueFunction))),
                  phaserMix (addToLayout (layout,
                                          std::make_unique<DspModulePluginDemoParameter> (ID::phaserMix,
                                                                       "Mix",
                                                                       "%",
                                                                       NormalisableRange<float> (0.0f, 100.0f),
                                                                       50.0f,
                                                                       valueToTextFunction,
                                                                       textToValueFunction))),
                  chorusEnabled (addToLayout (layout,
                                              std::make_unique<AudioParameterBool> (ID::chorusEnabled,
                                                                                    "Chorus",
                                                                                    false,
                                                                                    ""))),
                  chorusRate (addToLayout (layout,
                                           std::make_unique<DspModulePluginDemoParameter> (ID::chorusRate,
                                                                        "Rate",
                                                                        "Hz",
                                                                        NormalisableRange<float> (0.05f, 20.0f, 0.0f, 0.25f),
                                                                        1.0f,
                                                                        valueToTextFunction,
                                                                        textToValueFunction))),
                  chorusDepth (addToLayout (layout,
                                            std::make_unique<DspModulePluginDemoParameter> (ID::chorusDepth,
                                                                         "Depth",
                                                                         "%",
                                                                         NormalisableRange<float> (0.0f, 100.0f),
                                                                         50.0f,
                                                                         valueToTextFunction,
                                                                         textToValueFunction))),
                  chorusCentreDelay (addToLayout (layout,
                                                  std::make_unique<DspModulePluginDemoParameter> (ID::chorusCentreDelay,
                                                                               "Center",
                                                                               "ms",
                                                                               NormalisableRange<float> (1.0f, 100.0f, 0.0f, 0.25f),
                                                                               7.0f,
                                                                               valueToTextFunction,
                                                                               textToValueFunction))),
                  chorusFeedback (addToLayout (layout,
                                               std::make_unique<DspModulePluginDemoParameter> (ID::chorusFeedback,
                                                                            "Feedback",
                                                                            "%",
                                                                            NormalisableRange<float> (0.0f, 100.0f),
                                                                            50.0f,
                                                                            valueToTextFunction,
                                                                            textToValueFunction))),
                  chorusMix (addToLayout (layout,
                                          std::make_unique<DspModulePluginDemoParameter> (ID::chorusMix,
                                                                       "Mix",
                                                                       "%",
                                                                       NormalisableRange<float> (0.0f, 100.0f),
                                                                       50.0f,
                                                                       valueToTextFunction,
                                                                       textToValueFunction))),
                  ladderEnabled (addToLayout (layout,
                                              std::make_unique<AudioParameterBool> (ID::ladderEnabled,
                                                                                    "Ladder",
                                                                                    false,
                                                                                    ""))),
                  ladderMode (addToLayout (layout,
                                           std::make_unique<AudioParameterChoice> (ID::ladderMode,
                                                                                   "Mode",
                                                                                   StringArray { "LP12", "LP24", "HP12", "HP24", "BP12", "BP24" },
                                                                                   1))),
                  ladderCutoff (addToLayout (layout,
                                             std::make_unique<DspModulePluginDemoParameter> (ID::ladderCutoff,
                                                                          "Frequency",
                                                                          "Hz",
                                                                          NormalisableRange<float> (10.0f, 22000.0f, 0.0f, 0.25f),
                                                                          1000.0f,
                                                                          valueToTextFunction,
                                                                          textToValueFunction))),
                  ladderResonance (addToLayout (layout,
                                                std::make_unique<DspModulePluginDemoParameter> (ID::ladderResonance,
                                                                             "Resonance",
                                                                             "%",
                                                                             NormalisableRange<float> (0.0f, 100.0f),
                                                                             0.0f,
                                                                             valueToTextFunction,
                                                                             textToValueFunction))),
                  ladderDrive (addToLayout (layout,
                                            std::make_unique<DspModulePluginDemoParameter> (ID::ladderDrive,
                                                                         "Drive",
                                                                         "dB",
                                                                         NormalisableRange<float> (0.0f, 40.0f),
                                                                         0.0f,
                                                                         valueToTextFunction,
                                                                         textToValueFunction)))
        */
    }
}

