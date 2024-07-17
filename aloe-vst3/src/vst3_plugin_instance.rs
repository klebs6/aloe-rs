crate::ix!();

#[no_copy]
#[leak_detector]
pub struct Vst3PluginInstance<'a> {
    base: AudioPluginInstance<'a>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    run_loop: SharedResourcePointer<RunLoop>,

    holder:   Box<Vst3ComponentHolder<'a>>,

    /**
      | Information objects:
      |
      */
    company: String,

    info:    Box<PClassInfo>,
    info2:   Box<PClassInfo2>,
    infow:   Box<PClassInfoW>,

    /**
      | Rudimentary interfaces:
      |
      */
    edit_controller:            VstComSmartPtr<Box<dyn IEditController>>,
    edit_controller2:           VstComSmartPtr<Box<dyn IEditController2>>,
    midi_mapping:               VstComSmartPtr<Box<dyn IMidiMapping>>,
    processor:                  VstComSmartPtr<Box<dyn AudioProcessorInterface>>,
    component_handler:          VstComSmartPtr<Box<dyn IComponentHandler>>,
    component_handler2:         VstComSmartPtr<Box<dyn IComponentHandler2>>,
    unit_info:                  VstComSmartPtr<Box<dyn IUnitInfo>>,
    unit_data:                  VstComSmartPtr<Box<dyn IUnitData>>,
    program_list_data:          VstComSmartPtr<Box<dyn IProgramListData>>,
    component_connection:       VstComSmartPtr<Box<dyn IConnectionPoint>>,
    edit_controller_connection: VstComSmartPtr<Box<dyn IConnectionPoint>>,
    track_info_listener:        VstComSmartPtr<Box<dyn ChannelContextIInfoListener<AttrID = *const u8>>>,

    /**
      | The number of IO buses MUST match that
      | of the plugin, even if there aren't enough
      | channels to process, as very poorly
      | specified by the Steinberg SDK
      |
      */
    input_bus_map:              Vst3FloatAndDoubleBusMapComposite,


    /**
      | The number of IO buses MUST match that
      | of the plugin, even if there aren't enough
      | channels to process, as very poorly
      | specified by the Steinberg SDK
      |
      */
    output_bus_map:             Vst3FloatAndDoubleBusMapComposite,

    input_buses:                Vec<AudioBusBuffers>,
    output_buses:               Vec<AudioBusBuffers>,
    cached_bus_layouts:         AudioProcessorBusesLayout,
    program_names:              StringArray,

    /**
      | = (VstParamID) -1;
      |
      */
    program_parameterid:  ParamID,

    id_to_param_map:      HashMap<ParamID,*mut Vst3Parameter<'a>>,
    parameter_dispatcher: EditControllerParameterDispatcher,
    stored_midi_mapping:  StoredMidiMapping,

    /**
      | The plugin may request a restart during
      | playback, which may in turn attempt
      | to call functions such as setProcessing
      | and setActive. It is an error to call
      | these functions simultaneously with
      | 
      | IAudioProcessor::process, so we use
      | this mutex to ensure that this scenario
      | is impossible.
      |
      */
    process_mutex:        SpinLock,

    cached_param_values:  CachedParamValues,

    /**
      | { new ParameterChanges };
      |
      */
    input_parameter_changes:            VstComSmartPtr<ParameterChanges>,

    /**
      | { new ParameterChanges };
      |
      */
    output_parameter_changes:           VstComSmartPtr<ParameterChanges>,

    midi_inputs:                        VstComSmartPtr<MidiEventList>,
    midi_outputs:                       VstComSmartPtr<MidiEventList>,

    /**
      | < Only use this in processBlock()!
      |
      */
    timing_info:                        ProcessContext,

    is_controller_initialised:          bool, // default = false
    is_active:                          bool, // default = false
    last_process_block_call_was_bypass: bool, // default = false
    bypass_param:                       *mut Vst3Parameter<'a>, // default = nullptr
}

impl<'a> Drop for Vst3PluginInstance<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            struct Vst3Deleter : public CallbackMessage
            {
                Vst3Deleter (Vst3PluginInstance& inInstance, WaitableEvent& inEvent)
                    : vst3Instance (inInstance), completionSignal (inEvent)
                {}

                void messageCallback() override
                {
                    vst3Instance.cleanup();
                    completionSignal.signal();
                }

                Vst3PluginInstance& vst3Instance;
                WaitableEvent& completionSignal;
            };

            if (MessageManager::getInstance()->isThisTheMessageThread())
            {
                cleanup();
            }
            else
            {
                WaitableEvent completionEvent;
                (new Vst3Deleter (*this, completionEvent))->post();
                completionEvent.wait();
            }
        */
    }
}

impl<'a> Vst3PluginInstance<'a> {

    pub fn new(component_holder: *mut Vst3ComponentHolder) -> Self {
    
        todo!();
        /*


            : AudioPluginInstance (getBusProperties (componentHolder->component)),
              holder (componentHolder),
              midiInputs (new MidiEventList()),
              midiOutputs (new MidiEventList())
            holder->host->setPlugin (this);
        */
    }
    
    pub fn cleanup(&mut self)  {
        
        todo!();
        /*
            jassert (getActiveEditor() == nullptr); // You must delete any editors before deleting the plugin instance!

            releaseResources();

            if (editControllerConnection != nullptr && componentConnection != nullptr)
            {
                editControllerConnection->disconnect (componentConnection);
                componentConnection->disconnect (editControllerConnection);
            }

            editController->setComponentHandler (nullptr);

            if (isControllerInitialised)
                editController->terminate();

            holder->terminate();

            componentConnection = nullptr;
            editControllerConnection = nullptr;
            unitData = nullptr;
            unitInfo = nullptr;
            programListData = nullptr;
            componentHandler2 = nullptr;
            componentHandler = nullptr;
            processor = nullptr;
            midiMapping = nullptr;
            editController2 = nullptr;
            editController = nullptr;
        */
    }
    
    pub fn initialise(&mut self) -> bool {
        
        todo!();
        /*
            // It's highly advisable to create your plugins using the message thread.
            // The Vst3 spec requires that many of the functions called during
            // initialisation are only called from the message thread.
            ALOE_ASSERT_MESSAGE_THREAD

            if (! holder->initialise())
                return false;

            if (! (isControllerInitialised || holder->fetchController (editController)))
                return false;

            // (May return an error if the plugin combines the IComponent and IEditController implementations)
            editController->initialize (holder->host->getFUnknown());

            isControllerInitialised = true;
            editController->setComponentHandler (holder->host);
            grabInformationObjects();
            interconnectComponentAndController();

            auto configureParameters = [this]
            {
                refreshParameterList();
                synchroniseStates();
                syncProgramNames();
            };

            configureParameters();
            setupIO();

            // Some plug-ins don't present their parameters until after the IO has been
            // configured, so we need to jump though all these hoops again
            if (getParameters().isEmpty() && editController->getParameterCount() > 0)
                configureParameters();

            updateMidiMappings();

            parameterDispatcher.start (*editController);

            return true;
        */
    }
    
    pub fn get_extensions(&self, visitor: &mut dyn ExtensionsVisitorInterface)  {
        
        todo!();
        /*
            struct Extensions : public ExtensionsVisitor::Vst3Client
            {
                explicit Extensions (const Vst3PluginInstance* instanceIn) : instance (instanceIn) {}

                void* getIComponentPtr() const  override   { return instance->holder->component; }

                MemoryBlock getPreset() const override             { return instance->getStateForPresetFile(); }

                bool setPreset (const MemoryBlock& rawData) const override
                {
                    return instance->setStateFromPresetFile (rawData);
                }

                const Vst3PluginInstance* instance = nullptr;
            };

            visitor.visitVst3Client (Extensions { this });
        */
    }
    
    pub fn get_platform_specific_data(&mut self)  {
        
        todo!();
        /*
            return holder->component;
        */
    }
    
    pub fn update_midi_mappings(&mut self)  {
        
        todo!();
        /*
            // MIDI mappings will always be updated on the main thread, but we need to ensure
            // that we're not simultaneously reading them on the audio thread.
            const SpinLock::ScopedLockType processLock (processMutex);

            if (midiMapping != nullptr)
                storedMidiMapping.storeMappings (*midiMapping);
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            auto& module = holder->module;
            return module != nullptr ? module->getName() : String();
        */
    }
    
    pub fn repopulate_arrangements(&self, 
        input_arrangements:  &mut Vec<SpeakerArrangement>,
        output_arrangements: &mut Vec<SpeakerArrangement>)  {
        
        todo!();
        /*
            inputArrangements.clearQuick();
            outputArrangements.clearQuick();

            auto numInputAudioBuses  = getBusCount (true);
            auto numOutputAudioBuses = getBusCount (false);

            for (int i = 0; i < numInputAudioBuses; ++i)
                inputArrangements.add (getArrangementForBus (processor, true, i));

            for (int i = 0; i < numOutputAudioBuses; ++i)
                outputArrangements.add (getArrangementForBus (processor, false, i));
        */
    }
    
    pub fn processor_layouts_to_arrangements(&mut self, 
        input_arrangements:  &mut Vec<SpeakerArrangement>,
        output_arrangements: &mut Vec<SpeakerArrangement>)  {
        
        todo!();
        /*
            inputArrangements.clearQuick();
            outputArrangements.clearQuick();

            auto numInputBuses  = getBusCount (true);
            auto numOutputBuses = getBusCount (false);

            for (int i = 0; i < numInputBuses; ++i)
                inputArrangements.add (getVst3SpeakerArrangement (getBus (true, i)->getLastEnabledLayout()));

            for (int i = 0; i < numOutputBuses; ++i)
                outputArrangements.add (getVst3SpeakerArrangement (getBus (false, i)->getLastEnabledLayout()));
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        new_sample_rate:             f64,
        estimated_samples_per_block: i32)  {
        
        todo!();
        /*
            // The Vst3 spec requires that IComponent::setupProcessing() is called on the message
            // thread. If you call it from a different thread, some plugins may break.
            ALOE_ASSERT_MESSAGE_THREAD
            MessageManagerLock lock;

            const SpinLock::ScopedLockType processLock (processMutex);

            // Avoid redundantly calling things like setActive, which can be a heavy-duty call for some plugins:
            if (isActive
                  && getSampleRate() == newSampleRate
                  && getBlockSize() == estimatedSamplesPerBlock)
                return;

            using namespace Vst;

            ProcessSetup setup;
            setup.symbolicSampleSize    = isUsingDoublePrecision() ? kSample64 : kSample32;
            setup.maxSamplesPerBlock    = estimatedSamplesPerBlock;
            setup.sampleRate            = newSampleRate;
            setup.processMode           = isNonRealtime() ? kOffline : kRealtime;

            warnOnFailure (processor->setupProcessing (setup));

            holder->initialise();

            Vec<VstSpeakerArrangement> inputArrangements, outputArrangements;
            processorLayoutsToArrangements (inputArrangements, outputArrangements);

            // Some plug-ins will crash if you pass a nullptr to setBusArrangements!
            SpeakerArrangement nullArrangement = {};
            auto* inputArrangementData  = inputArrangements.isEmpty()  ? &nullArrangement : inputArrangements.getRawDataPointer();
            auto* outputArrangementData = outputArrangements.isEmpty() ? &nullArrangement : outputArrangements.getRawDataPointer();

            warnOnFailure (processor->setBusArrangements (inputArrangementData,  inputArrangements.size(),
                                                          outputArrangementData, outputArrangements.size()));

            Vec<VstSpeakerArrangement> actualInArr, actualOutArr;
            repopulateArrangements (actualInArr, actualOutArr);

            jassert (actualInArr == inputArrangements && actualOutArr == outputArrangements);

            // Needed for having the same sample rate in processBlock(); some plugins need this!
            setRateAndBufferSizeDetails (newSampleRate, estimatedSamplesPerBlock);

            auto numInputBuses  = getBusCount (true);
            auto numOutputBuses = getBusCount (false);

            for (int i = 0; i < numInputBuses; ++i)
                warnOnFailure (holder->component->activateBus (VstkAudio, VstkInput,  i, getBus (true,  i)->isEnabled() ? 1 : 0));

            for (int i = 0; i < numOutputBuses; ++i)
                warnOnFailure (holder->component->activateBus (VstkAudio, VstkOutput, i, getBus (false, i)->isEnabled() ? 1 : 0));

            setLatencySamples (jmax (0, (int) processor->getLatencySamples()));
            cachedBusLayouts = getBusesLayout();

            setStateForAllMidiBuses (true);

            warnOnFailure (holder->component->setActive (true));
            warnOnFailureIfImplemented (processor->setProcessing (true));

            isActive = true;
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType lock (processMutex);

            if (! isActive)
                return; // Avoids redundantly calling things like setActive

            isActive = false;

            setStateForAllMidiBuses (false);

            if (processor != nullptr)
                warnOnFailureIfImplemented (processor->setProcessing (false));

            if (holder->component != nullptr)
                warnOnFailure (holder->component->setActive (false));
        */
    }
    
    pub fn supports_double_precision_processing(&self) -> bool {
        
        todo!();
        /*
            return (processor->canProcessSampleSize (VstkSample64) == kResultTrue);
        */
    }

    /**
      | Important: It is strongly recommended
      | to use this function if you need to find
      | the Aloe parameter corresponding to
      | a particular IEditController parameter.
      | 
      | -----------
      | @note
      | 
      | a parameter at a given index in the IEditController
      | does not necessarily correspond to
      | the parameter at the same index in
      | 
      | AudioProcessor::getParameters().
      |
      */
    pub fn get_parameter_forid(&self, paramid: ParamID) -> *mut Vst3Parameter {
        
        todo!();
        /*
            const auto it = idToParamMap.find (paramID);
            return it != idToParamMap.end() ? it->second : nullptr;
        */
    }
    
    pub fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            jassert (! isUsingDoublePrecision());

            const SpinLock::ScopedLockType processLock (processMutex);

            if (isActive && processor != nullptr)
                processAudio (buffer, midiMessages, VstkSample32, false);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer
    ) {
        
        todo!();
        /*
            jassert (isUsingDoublePrecision());

            const SpinLock::ScopedLockType processLock (processMutex);

            if (isActive && processor != nullptr)
                processAudio (buffer, midiMessages, VstkSample64, false);
        */
    }
    
    pub fn process_block_bypassed(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            jassert (! isUsingDoublePrecision());

            const SpinLock::ScopedLockType processLock (processMutex);

            if (bypassParam != nullptr)
            {
                if (isActive && processor != nullptr)
                    processAudio (buffer, midiMessages, VstkSample32, true);
            }
            else
            {
                AudioProcessor::processBlockBypassed (buffer, midiMessages);
            }
        */
    }
    
    pub fn process_block_bypassed_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer
    ) {
        
        todo!();
        /*
            jassert (isUsingDoublePrecision());

            const SpinLock::ScopedLockType processLock (processMutex);

            if (bypassParam != nullptr)
            {
                if (isActive && processor != nullptr)
                    processAudio (buffer, midiMessages, VstkSample64, true);
            }
            else
            {
                AudioProcessor::processBlockBypassed (buffer, midiMessages);
            }
        */
    }
    
    
    pub fn process_audio<FloatType>(
        &mut self, 
        buffer:                         &mut AudioBuffer<FloatType>,
        midi_messages:                  &mut MidiBuffer,
        sample_size:                    SymbolicSampleSizes,
        is_process_block_bypassed_call: bool
    ) {
    
        todo!();
        /*
            using namespace Vst;
            auto numSamples = buffer.getNumSamples();

            auto numInputAudioBuses  = getBusCount (true);
            auto numOutputAudioBuses = getBusCount (false);

            updateBypass (isProcessBlockBypassedCall);

            ProcessData data;
            data.processMode            = isNonRealtime() ? kOffline : kRealtime;
            data.symbolicSampleSize     = sampleSize;
            data.numInputs              = numInputAudioBuses;
            data.numOutputs             = numOutputAudioBuses;
            data.inputParameterChanges  = inputParameterChanges;
            data.outputParameterChanges = outputParameterChanges;
            data.numSamples             = (i32) numSamples;

            updateTimingInformation (data, getSampleRate());

            for (int i = getTotalNumInputChannels(); i < buffer.getNumChannels(); ++i)
                buffer.clear (i, 0, numSamples);

            inputParameterChanges->clear();
            outputParameterChanges->clear();

            associateWith (data, buffer);
            associateWith (data, midiMessages);

            cachedParamValues.ifSet ([&] (i32 index, float value)
            {
                inputParameterChanges->set (cachedParamValues.getParamID (index), value);
            });

            processor->process (data);

            outputParameterChanges->forEach ([&] (i32 index, float value)
            {
                parameterDispatcher.push (index, value);
            });

            midiMessages.clear();
            MidiEventList::toMidiBuffer (midiMessages, *midiOutputs);
        */
    }
    
    pub fn can_add_bus(&self, _0: bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn can_remove_bus(&self, _0: bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            const SpinLock::ScopedLockType processLock (processMutex);

            // if the processor is not active, we ask the underlying plug-in if the
            // layout is actually supported
            if (! isActive)
                return canApplyBusesLayout (layouts);

            // not much we can do to check the layout while the audio processor is running
            // Let's at least check if it is a Vst3 compatible layout
            for (int dir = 0; dir < 2; ++dir)
            {
                bool isInput = (dir == 0);
                auto n = getBusCount (isInput);

                for (int i = 0; i < n; ++i)
                    if (getChannelLayoutOfBus (isInput, i).isDiscreteLayout())
                        return false;
            }

            return true;
        */
    }
    
    pub fn sync_bus_layouts(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            for (int dir = 0; dir < 2; ++dir)
            {
                bool isInput = (dir == 0);
                auto n = getBusCount (isInput);
                const VstBusDirection vstDir = (isInput ? VstkInput : VstkOutput);

                for (int busIdx = 0; busIdx < n; ++busIdx)
                {
                    const bool isEnabled = (! layouts.getChannelSet (isInput, busIdx).isDisabled());

                    if (holder->component->activateBus (VstkAudio, vstDir, busIdx, (isEnabled ? 1 : 0)) != kResultOk)
                        return false;
                }
            }

            Vec<VstSpeakerArrangement> inputArrangements, outputArrangements;

            for (int i = 0; i < layouts.inputBuses.size(); ++i)
            {
                const auto& requested = layouts.getChannelSet (true, i);
                inputArrangements.add (getVst3SpeakerArrangement (requested.isDisabled() ? getBus (true, i)->getLastEnabledLayout() : requested));
            }

            for (int i = 0; i < layouts.outputBuses.size(); ++i)
            {
                const auto& requested = layouts.getChannelSet (false, i);
                outputArrangements.add (getVst3SpeakerArrangement (requested.isDisabled() ? getBus (false, i)->getLastEnabledLayout() : requested));
            }

            // Some plug-ins will crash if you pass a nullptr to setBusArrangements!
            VstSpeakerArrangement nullArrangement = {};
            auto* inputArrangementData  = inputArrangements.isEmpty()  ? &nullArrangement : inputArrangements.getRawDataPointer();
            auto* outputArrangementData = outputArrangements.isEmpty() ? &nullArrangement : outputArrangements.getRawDataPointer();

            if (processor->setBusArrangements (inputArrangementData, inputArrangements.size(),
                                               outputArrangementData, outputArrangements.size()) != kResultTrue)
                return false;

            // check if the layout matches the request
            Vec<VstSpeakerArrangement> actualIn, actualOut;
            repopulateArrangements (actualIn, actualOut);

            return (actualIn == inputArrangements && actualOut == outputArrangements);
        */
    }
    
    pub fn can_apply_buses_layout(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            // someone tried to change the layout while the AudioProcessor is running
            // call releaseResources first!
            jassert (! isActive);

            bool result = syncBusLayouts (layouts);

            // didn't succeed? Make sure it's back in it's original state
            if (! result)
                syncBusLayouts (getBusesLayout());

            return result;
        */
    }
    
    pub fn update_track_properties(&mut self, properties: &AudioProcessorTrackProperties)  {
        
        todo!();
        /*
            if (trackInfoListener != nullptr)
            {
                VstComSmartPtr<VstIAttributeList> l (new AudioProcessorTrackPropertiesAttributeList (properties));
                trackInfoListener->setChannelContextInfos (l);
            }
        */
    }
    
    pub fn get_channel_name(&self, 
        channel_index:     i32,
        for_input:         bool,
        for_audio_channel: bool) -> String {
        
        todo!();
        /*
            auto numBuses = getNumSingleDirectionBusesFor (holder->component, forInput, forAudioChannel);
            int numCountedChannels = 0;

            for (int i = 0; i < numBuses; ++i)
            {
                auto busInfo = getBusInfo (forInput, forAudioChannel, i);

                numCountedChannels += busInfo.channelCount;

                if (channelIndex < numCountedChannels)
                    return toString (busInfo.name);
            }

            return {};
        */
    }
    
    pub fn get_input_channel_name(&self, channel_index: i32) -> String {
        
        todo!();
        /*
            return getChannelName (channelIndex, true, true);
        */
    }
    
    pub fn get_output_channel_name(&self, channel_index: i32) -> String {
        
        todo!();
        /*
            return getChannelName (channelIndex, false, true);
        */
    }
    
    pub fn is_input_channel_stereo_pair(&self, channel_index: i32) -> bool {
        
        todo!();
        /*
            int busIdx;
            return getOffsetInBusBufferForAbsoluteChannelIndex (true, channelIndex, busIdx) >= 0
                     && getBusInfo (true, true, busIdx).channelCount == 2;
        */
    }
    
    pub fn is_output_channel_stereo_pair(&self, channel_index: i32) -> bool {
        
        todo!();
        /*
            int busIdx;
            return getOffsetInBusBufferForAbsoluteChannelIndex (false, channelIndex, busIdx) >= 0
                     && getBusInfo (false, true, busIdx).channelCount == 2;
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return getNumSingleDirectionBusesFor (holder->component, true,  false) > 0;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return getNumSingleDirectionBusesFor (holder->component, false, false) > 0;
        */
    }
    
    pub fn get_bypass_parameter(&self) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            return bypassParam;
        */
    }

    /**
      | May return a negative value as a means
      | of informing us that the plugin has "infinite
      | tail," or 0 for "no tail."
      |
      */
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            if (processor != nullptr)
            {
                auto sampleRate = getSampleRate();

                if (sampleRate > 0.0)
                {
                    auto tailSamples = processor->getTailSamples();

                    if (tailSamples == VstkInfiniteTail)
                        return std::numeric_limits<double>::infinity();

                    return jlimit (0, 0x7fffffff, (int) processor->getTailSamples()) / sampleRate;
                }
            }

            return 0.0;
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            if (auto* view = tryCreatingView())
                return new Vst3PluginWindow (this, view);

            return nullptr;
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            // (if possible, avoid creating a second instance of the editor, because that crashes some plugins)
            if (getActiveEditor() != nullptr)
                return true;

            VstComSmartPtr<IPlugView> view (tryCreatingView(), false);
            return view != nullptr;
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return programNames.size();
        */
    }
    
    pub fn get_program_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
            return index >= 0 ? programNames[index] : String();
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            if (programNames.size() > 0 && editController != nullptr)
                if (auto* param = getParameterForID (programParameterID))
                    return jmax (0, roundToInt (param->getValue() * (float) (programNames.size() - 1)));

            return 0;
        */
    }
    
    pub fn set_current_program(&mut self, program: i32)  {
        
        todo!();
        /*
            if (programNames.size() > 0 && editController != nullptr)
            {
                auto value = static_cast<VstParamValue> (program) / static_cast<VstParamValue> (jmax (1, programNames.size() - 1));

                if (auto* param = getParameterForID (programParameterID))
                    param->setValueNotifyingHost ((float) value);
            }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType lock (processMutex);

            if (holder->component != nullptr && processor != nullptr)
            {
                processor->setProcessing (false);
                holder->component->setActive (false);

                holder->component->setActive (true);
                processor->setProcessing (true);
            }
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            // The Vst3 plugin format requires that get/set state calls are made
            // from the message thread.
            // We'll lock the message manager here as a safety precaution, but some
            // plugins may still misbehave!

            ALOE_ASSERT_MESSAGE_THREAD
            MessageManagerLock lock;

            parameterDispatcher.flush();

            XmlElement state ("Vst3PluginState");

            appendStateFrom (state, holder->component, "IComponent");
            appendStateFrom (state, editController, "IEditController");

            AudioProcessor::copyXmlToBinary (state, destData);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            // The Vst3 plugin format requires that get/set state calls are made
            // from the message thread.
            // We'll lock the message manager here as a safety precaution, but some
            // plugins may still misbehave!

            ALOE_ASSERT_MESSAGE_THREAD
            MessageManagerLock lock;

            parameterDispatcher.flush();

            if (auto head = AudioProcessor::getXmlFromBinary (data, sizeInBytes))
            {
                auto componentStream (createMemoryStreamForState (*head, "IComponent"));

                if (componentStream != nullptr && holder->component != nullptr)
                    holder->component->setState (componentStream);

                if (editController != nullptr)
                {
                    if (componentStream != nullptr)
                    {
                        int64 result;
                        componentStream->seek (0, IBStream::kIBSeekSet, &result);
                        setComponentStateAndResetParameters (*componentStream);
                    }

                    auto controllerStream (createMemoryStreamForState (*head, "IEditController"));

                    if (controllerStream != nullptr)
                        editController->setState (controllerStream);
                }
            }
        */
    }
    
    pub fn set_component_state_and_reset_parameters(&mut self, stream: &mut MemoryStream)  {
        
        todo!();
        /*
            jassert (editController != nullptr);

            warnOnFailureIfImplemented (editController->setComponentState (&stream));
            resetParameters();
        */
    }
    
    pub fn reset_parameters(&mut self)  {
        
        todo!();
        /*
            for (auto* parameter : getParameters())
            {
                auto* vst3Param = static_cast<Vst3Parameter*> (parameter);
                const auto value = (float) editController->getParamNormalized (vst3Param->getParamID());
                vst3Param->setValueWithoutUpdatingProcessor (value);
            }
        */
    }
    
    pub fn get_state_for_preset_file(&self) -> MemoryBlock {
        
        todo!();
        /*
            VstComSmartPtr<MemoryStream> memoryStream = new MemoryStream();

            if (memoryStream == nullptr || holder->component == nullptr)
                return {};

            const auto saved = VstPresetFile::savePreset (memoryStream,
                                                                       holder->cidOfComponent,
                                                                       holder->component,
                                                                       editController);

            if (saved)
                return { memoryStream->getData(), static_cast<size_t> (memoryStream->getSize()) };

            return {};
        */
    }
    
    pub fn set_state_from_preset_file(&self, raw_data: &MemoryBlock) -> bool {
        
        todo!();
        /*
            MemoryBlock rawDataCopy (rawData);
            VstComSmartPtr<MemoryStream> memoryStream = new MemoryStream (rawDataCopy.getData(), (int) rawDataCopy.getSize());

            if (memoryStream == nullptr || holder->component == nullptr)
                return false;

            return VstPresetFile::loadPreset (memoryStream, holder->cidOfComponent,
                                                           holder->component, editController, nullptr);
        */
    }
    
    pub fn fill_in_plugin_description(&self, description: &mut PluginDescription)  {
        
        todo!();
        /*
            holder->fillInPluginDescription (description);
        */
    }

    /**
      | @note
      | 
      | Not applicable to Vst3
      |
      */
    pub fn get_current_program_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            destData.setSize (0, true);
        */
    }

    /**
      | @note
      | 
      | Not applicable to Vst3
      |
      */
    pub fn set_current_program_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            ignoreUnused (data, sizeInBytes);
        */
    }
    
    
    pub fn append_state_from<Type>(
        head:       &mut XmlElement,
        object:     &mut VstComSmartPtr<Type>,
        identifier: &String)  {
    
        todo!();
        /*
            if (object != nullptr)
            {
                MemoryStream stream;

                if (object->getState (&stream) == kResultTrue)
                {
                    MemoryBlock info (stream.getData(), (size_t) stream.getSize());
                    head.createNewChildElement (identifier)->addTextElement (info.toBase64Encoding());
                }
            }
        */
    }
    
    pub fn create_memory_stream_for_state(
        head:       &mut XmlElement,
        identifier: &str) -> VstComSmartPtr<MemoryStream> {
        
        todo!();
        /*
            if (auto* state = head.getChildByName (identifier))
            {
                MemoryBlock mem;

                if (mem.fromBase64Encoding (state->getAllSubText()))
                {
                    VstComSmartPtr<MemoryStream> stream (new MemoryStream(), false);
                    stream->setSize ((TSize) mem.getSize());
                    mem.copyTo (stream->getData(), 0, mem.getSize());
                    return stream;
                }
            }

            return nullptr;
        */
    }
    
    /**
      | Some plugins need to be "connected"
      | to intercommunicate between their
      | implemented classes
      |
      */
    pub fn interconnect_component_and_controller(&mut self)  {
        
        todo!();
        /*
            componentConnection.loadFrom (holder->component);
            editControllerConnection.loadFrom (editController);

            if (componentConnection != nullptr && editControllerConnection != nullptr)
            {
                warnOnFailure (componentConnection->connect (editControllerConnection));
                warnOnFailure (editControllerConnection->connect (componentConnection));
            }
        */
    }
    
    pub fn refresh_parameter_list(&mut self)  {
        
        todo!();
        /*
            AudioProcessorParameterGroup newParameterTree;

            // We're going to add parameter groups to the tree recursively in the same order as the
            // first parameters contained within them.
            std::map<VstUnitID, VstUnitInfo> infoMap;
            std::map<VstUnitID, AudioProcessorParameterGroup*> groupMap;
            groupMap[VstkRootUnitId] = &newParameterTree;

            if (unitInfo != nullptr)
            {
                const auto numUnits = unitInfo->getUnitCount();

                for (int i = 1; i < numUnits; ++i)
                {
                    VstUnitInfo ui{};
                    unitInfo->getUnitInfo (i, ui);
                    infoMap[ui.id] = std::move (ui);
                }
            }

            {
                auto allIds = getAllParamIDs (*editController);
                inputParameterChanges ->initialise (allIds);
                outputParameterChanges->initialise (allIds);
                cachedParamValues = CachedParamValues { std::move (allIds) };
            }

            for (int i = 0; i < editController->getParameterCount(); ++i)
            {
                auto paramInfo = getParameterInfoForIndex (i);
                auto* param = new Vst3Parameter (*this,
                                                 i,
                                                 paramInfo.id,
                                                 (paramInfo.flags & VstParameterInfo::kCanAutomate) != 0);

                if ((paramInfo.flags & VstParameterInfo::kIsBypass) != 0)
                    bypassParam = param;

                std::function<AudioProcessorParameterGroup*(VstUnitID)> findOrCreateGroup;
                findOrCreateGroup = [&groupMap, &infoMap, &findOrCreateGroup] (VstUnitID groupID)
                {
                    auto existingGroup = groupMap.find (groupID);

                    if (existingGroup != groupMap.end())
                        return existingGroup->second;

                    auto groupInfo = infoMap.find (groupID);

                    if (groupInfo == infoMap.end())
                        return groupMap[VstkRootUnitId];

                    auto* group = new AudioProcessorParameterGroup (String (groupInfo->first),
                                                                    toString (groupInfo->second.name),
                                                                    {});
                    groupMap[groupInfo->first] = group;

                    auto* parentGroup = findOrCreateGroup (groupInfo->second.parentUnitId);
                    parentGroup->addChild (std::unique_ptr<AudioProcessorParameterGroup> (group));

                    return group;
                };

                auto* group = findOrCreateGroup (paramInfo.unitId);
                group->addChild (std::unique_ptr<AudioProcessorParameter> (param));
            }

            setParameterTree (std::move (newParameterTree));

            idToParamMap = [this]
            {
                std::map<VstParamID, Vst3Parameter*> result;

                for (auto* parameter : getParameters())
                {
                    auto* vst3Param = static_cast<Vst3Parameter*> (parameter);
                    result.emplace (vst3Param->getParamID(), vst3Param);
                }

                return result;
            }();
        */
    }
    
    pub fn synchronise_states(&mut self)  {
        
        todo!();
        /*
            MemoryStream stream;

            if (holder->component->getState (&stream) == kResultTrue)
                if (stream.seek (0, IBStream::kIBSeekSet, nullptr) == kResultTrue)
                    setComponentStateAndResetParameters (stream);
        */
    }
    
    pub fn grab_information_objects(&mut self)  {
        
        todo!();
        /*
            processor.loadFrom (holder->component);
            unitInfo.loadFrom (holder->component);
            programListData.loadFrom (holder->component);
            unitData.loadFrom (holder->component);
            editController2.loadFrom (holder->component);
            midiMapping.loadFrom (holder->component);
            componentHandler.loadFrom (holder->component);
            componentHandler2.loadFrom (holder->component);
            trackInfoListener.loadFrom (holder->component);

            if (processor == nullptr)           processor.loadFrom (editController);
            if (unitInfo == nullptr)            unitInfo.loadFrom (editController);
            if (programListData == nullptr)     programListData.loadFrom (editController);
            if (unitData == nullptr)            unitData.loadFrom (editController);
            if (editController2 == nullptr)     editController2.loadFrom (editController);
            if (midiMapping == nullptr)         midiMapping.loadFrom (editController);
            if (componentHandler == nullptr)    componentHandler.loadFrom (editController);
            if (componentHandler2 == nullptr)   componentHandler2.loadFrom (editController);
            if (trackInfoListener == nullptr)   trackInfoListener.loadFrom (editController);
        */
    }
    
    pub fn set_state_for_all_midi_buses(&mut self, new_state: bool)  {
        
        todo!();
        /*
            setStateForAllBusesOfType (holder->component, newState, true, false);   // Activate/deactivate MIDI inputs
            setStateForAllBusesOfType (holder->component, newState, false, false);  // Activate/deactivate MIDI outputs
        */
    }
    
    pub fn setupio(&mut self)  {
        
        todo!();
        /*
            setStateForAllMidiBuses (true);

            VstProcessSetup setup;
            setup.symbolicSampleSize   = VstkSample32;
            setup.maxSamplesPerBlock   = 1024;
            setup.sampleRate           = 44100.0;
            setup.processMode          = VstkRealtime;

            warnOnFailure (processor->setupProcessing (setup));

            cachedBusLayouts = getBusesLayout();
            setRateAndBufferSizeDetails (setup.sampleRate, (int) setup.maxSamplesPerBlock);
        */
    }
    
    pub fn get_bus_properties(component: &mut VstComSmartPtr<Box<dyn VstIComponent>>) 
        -> AudioProcessorBusesProperties 
    {
        todo!();
        /*
            AudioProcessor::BusesProperties busProperties;
            VstComSmartPtr<VstIAudioProcessor> processor;
            processor.loadFrom (component.get());

            for (int dirIdx = 0; dirIdx < 2; ++dirIdx)
            {
                const bool isInput = (dirIdx == 0);
                const VstBusDirection dir = (isInput ? VstkInput : VstkOutput);
                const int numBuses = component->getBusCount (VstkAudio, dir);

                for (int i = 0; i < numBuses; ++i)
                {
                    VstBusInfo info;

                    if (component->getBusInfo (VstkAudio, dir, (i32) i, info) != kResultOk)
                        continue;

                    AudioChannelSet layout = (info.channelCount == 0 ? AudioChannelSet::disabled()
                                                                     : AudioChannelSet::discreteChannels (info.channelCount));

                    VstSpeakerArrangement arr;
                    if (processor != nullptr && processor->getBusArrangement (dir, i, arr) == kResultOk)
                        layout = getChannelSetForSpeakerArrangement (arr);

                    busProperties.addBus (isInput, toString (info.name), layout,
                                          (info.flags & VstBusInfo::kDefaultActive) != 0);
                }
            }

            return busProperties;
        */
    }
    
    pub fn get_bus_info(
        &self, 
        for_input: bool,
        for_audio: bool,
        index:     Option<i32>

    ) -> VstBusInfo {

        let index: i32 = index.unwrap_or(0);

        todo!();
        /*
            VstBusInfo busInfo;
            busInfo.mediaType = forAudio ? VstkAudio : VstkEvent;
            busInfo.direction = forInput ? VstkInput : VstkOutput;
            busInfo.channelCount = 0;

            holder->component->getBusInfo (busInfo.mediaType, busInfo.direction,
                                           (i32) index, busInfo);
            return busInfo;
        */
    }
    
    pub fn update_bypass(&mut self, process_block_bypassed_called: bool)  {
        
        todo!();
        /*
            // to remain backward compatible, the logic needs to be the following:
            // - if processBlockBypassed was called then definitely bypass the Vst3
            // - if processBlock was called then only un-bypass the Vst3 if the previous
            //   call was processBlockBypassed, otherwise do nothing
            if (processBlockBypassedCalled)
            {
                if (bypassParam != nullptr && (bypassParam->getValue() == 0.0f || ! lastProcessBlockCallWasBypass))
                    bypassParam->setValue (1.0f);
            }
            else
            {
                if (lastProcessBlockCallWasBypass && bypassParam != nullptr)
                    bypassParam->setValue (0.0f);

            }

            lastProcessBlockCallWasBypass = processBlockBypassedCalled;
        */
    }

    /**
      | -----------
      | @note
      | 
      | An IPlugView, when first created, should
      | start with a ref-count of 1!
      |
      */
    pub fn try_creating_view(&self) -> *mut dyn IPlugView {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_THREAD

            IPlugView* v = editController->createView (VstViewType::kEditor);

            if (v == nullptr) v = editController->createView (nullptr);
            if (v == nullptr) editController->queryInterface (IPlugView::iid, (void**) &v);

            return v;
        */
    }
    
    
    pub fn associate_with<FloatType>(
        &mut self, 
        destination: &mut ProcessData,
        buffer:      &mut AudioBuffer<FloatType>
    ) {
    
        todo!();
        /*
            Vst3BufferExchange<FloatType>::mapBufferToBuses (inputBuses,  inputBusMap.get<FloatType>(),  cachedBusLayouts.inputBuses,  buffer);
            Vst3BufferExchange<FloatType>::mapBufferToBuses (outputBuses, outputBusMap.get<FloatType>(), cachedBusLayouts.outputBuses, buffer);

            destination.inputs  = inputBuses.getRawDataPointer();
            destination.outputs = outputBuses.getRawDataPointer();
        */
    }
    
    pub fn associate_with_midi_buffer(
        &mut self, 
        destination: &mut ProcessData,
        midi_buffer: &mut MidiBuffer
    ) {
        
        todo!();
        /*
            midiInputs->clear();
            midiOutputs->clear();

            if (acceptsMidi())
            {
                MidiEventList::hostToPluginEventList (*midiInputs,
                                                      midiBuffer,
                                                      destination.inputParameterChanges,
                                                      storedMidiMapping);
            }

            destination.inputEvents = midiInputs;
            destination.outputEvents = midiOutputs;
        */
    }
    
    pub fn update_timing_information(
        &mut self, 
        destination:         &mut ProcessData,
        process_sample_rate: f64
    ) {
        
        todo!();
        /*
            toProcessContext (timingInfo, getPlayHead(), processSampleRate);
            destination.processContext = &timingInfo;
        */
    }
    
    pub fn get_parameter_info_for_index(&self, index: i32) -> ParameterInfo {
        
        todo!();
        /*
            VstParameterInfo paramInfo{};

            if (editController != nullptr)
                editController->getParameterInfo ((i32) index, paramInfo);

            return paramInfo;
        */
    }
    
    pub fn get_program_list_info(&self, index: i32) -> ProgramListInfo {
        
        todo!();
        /*
            VstProgramListInfo paramInfo{};

            if (unitInfo != nullptr)
                unitInfo->getProgramListInfo (index, paramInfo);

            return paramInfo;
        */
    }
    
    pub fn sync_program_names(&mut self)  {
        
        todo!();
        /*
            programNames.clear();

            if (processor == nullptr || editController == nullptr)
                return;

            VstUnitID programUnitID;
            VstParameterInfo paramInfo{};

            {
                int idx, num = editController->getParameterCount();

                for (idx = 0; idx < num; ++idx)
                    if (editController->getParameterInfo (idx, paramInfo) == kResultOk
                         && (paramInfo.flags & VstParameterInfo::kIsProgramChange) != 0)
                        break;

                if (idx >= num)
                    return;

                programParameterID = paramInfo.id;
                programUnitID = paramInfo.unitId;
            }

            if (unitInfo != nullptr)
            {
                VstUnitInfo uInfo{};
                const int unitCount = unitInfo->getUnitCount();

                for (int idx = 0; idx < unitCount; ++idx)
                {
                    if (unitInfo->getUnitInfo(idx, uInfo) == kResultOk
                          && uInfo.id == programUnitID)
                    {
                        const int programListCount = unitInfo->getProgramListCount();

                        for (int j = 0; j < programListCount; ++j)
                        {
                            VstProgramListInfo programListInfo{};

                            if (unitInfo->getProgramListInfo (j, programListInfo) == kResultOk
                                  && programListInfo.id == uInfo.programListId)
                            {
                                VstString128 name;

                                for (int k = 0; k < programListInfo.programCount; ++k)
                                    if (unitInfo->getProgramName (programListInfo.id, k, name) == kResultOk)
                                        programNames.add (toString (name));

                                return;
                            }
                        }

                        break;
                    }
                }
            }

            if (editController != nullptr && paramInfo.stepCount > 0)
            {
                auto numPrograms = paramInfo.stepCount + 1;

                for (int i = 0; i < numPrograms; ++i)
                {
                    auto valueNormalized = static_cast<VstParamValue> (i) / static_cast<VstParamValue> (paramInfo.stepCount);

                    VstString128 programName;
                    if (editController->getParamStringByValue (paramInfo.id, valueNormalized, programName) == kResultOk)
                        programNames.add (toString (programName));
                }
            }
        */
    }
}
