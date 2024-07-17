crate::ix!();

pub struct Missing {}
pub type AEffect           = Missing;
pub type VstTimeInfo       = Missing;
pub type ExtensionsVisitor = Missing;
pub type PlugCategory      = Missing;
pub type BusLayout         = Missing;

#[no_copy]
#[leak_detector]
pub struct VSTPluginInstance<'a> {
    base:                               AudioPluginInstance<'a>,
    base2:                              Timer,
    base3:                              AsyncUpdater<'a>,
    vst_effect:                         *mut AEffect,
    vst_module:                         ModuleHandlePtr,
    extra_functions:                    Box<dyn VstPluginFormatExtraFunctions>,
    uses_cocoa_ns_view:                 bool, // default = false
    name:                               String,
    lock:                               CriticalSection,
    wants_midi_messages:                AtomicBool, // default = { false  }
    initialised:                        bool, // default = false
    is_power_on:                        AtomicBool, // default = { false  }
    last_process_block_call_was_bypass: bool, // default = false
    vst_supports_bypass:                bool, // default = false
    program_names:                      RefCell<StringArray>,
    out_of_place_buffer:                AudioBuffer<f32>,
    midi_in_lock:                       CriticalSection,
    incoming_midi:                      MidiBuffer,
    midi_events_to_send:                VSTMidiEventList,
    vst_host_time:                      VstTimeInfo,
    tmp_buffer_float:                   AudioBuffer<f32>,
    channel_buffer_float:               HeapBlock<*mut f32>,
    tmp_buffer_double:                  AudioBuffer<f64>,
    channel_buffer_double:              HeapBlock<*mut f64>,
    bypass_param:                       Box<VST2BypassParameter<'a>>,
    editor_size:                        Rectangle<i32>,
    xml_info:                           Box<VSTXMLInfo>,
}

impl<'a> Drop for VSTPluginInstance<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (vstEffect != nullptr && vstEffect->magic == 0x56737450 /* 'VstP' */)
            {
                struct VSTDeleter : public CallbackMessage
                {
                    VSTDeleter (VSTPluginInstance& inInstance, WaitableEvent& inEvent)
                        : vstInstance (inInstance), completionSignal (inEvent)
                    {}

                    void messageCallback() override
                    {
                        vstInstance.cleanup();
                        completionSignal.signal();
                    }

                    VSTPluginInstance& vstInstance;
                    WaitableEvent& completionSignal;
                };

                if (MessageManager::getInstance()->isThisTheMessageThread())
                {
                    cleanup();
                }
                else
                {
                    WaitableEvent completionEvent;
                    (new VSTDeleter (*this, completionEvent))->post();
                    completionEvent.wait();
                }
            }
        */
    }
}
 
impl<'a> VSTPluginInstance<'a> {

    pub fn new(
        mh:                 &ModuleHandlePtr,
        io_config:          &AudioProcessorBusProperties,
        effect:             *mut AEffect,
        sample_rate_to_use: f64,
        block_size_to_use:  i32) -> Self {
    
        todo!();
        /*


            : AudioPluginInstance (ioConfig),
              vstEffect (effect),
              vstModule (mh),
              name (mh->pluginName),
              bypassParam (new VST2BypassParameter (*this))
            jassert (vstEffect != nullptr);

            if (auto* xml = vstModule->vstXml.get())
                xmlInfo.reset (VSTXMLInfo::createFor (*xml));

            refreshParameterList();

            vstSupportsBypass = (pluginCanDo ("bypass") > 0);
            setRateAndBufferSizeDetails (sampleRateToUse, blockSizeToUse);
        */
    }
    
    pub fn refresh_parameter_list(&mut self)  {
        
        todo!();
        /*
            AudioProcessorParameterGroup newParameterTree;

            for (int i = 0; i < vstEffect->numParams; ++i)
            {
                String paramName;
                Vec<String> shortParamNames;
                float defaultValue = 0;
                String label;
                bool isAutomatable = dispatch (typename Vst2EffCanBeAutomated, i, 0, nullptr, 0) != 0;
                bool isDiscrete = false;
                int numSteps = AudioProcessor::getDefaultNumParameterSteps();
                bool isBoolSwitch = false;
                StringArray parameterValueStrings;
                const VSTXMLInfo::ValueType* valueType = nullptr;

                if (xmlInfo != nullptr)
                {
                    if (auto* param = xmlInfo->getParamForID (i, nullptr))
                    {
                        paramName = param->name;

                        for (auto& n : param->shortNames)
                            shortParamNames.add (n);

                        struct LengthComparator
                        {
                            static int compareElements (const String& first, const String& second) 
                            {
                                return first.length() - second.length();
                            }
                        };

                        LengthComparator comp;
                        shortParamNames.sort (comp);

                        defaultValue = param->defaultValue;
                        label = param->label;

                        if (param->type == "switch")
                        {
                            isBoolSwitch = true;
                            numSteps = 2;
                            valueType = &xmlInfo->switchValueType;
                        }
                        else
                        {
                            valueType = xmlInfo->getValueType (param->type);
                        }

                        if (param->numberOfStates >= 2)
                        {
                            numSteps = param->numberOfStates;

                            if (valueType != nullptr)
                            {
                                for (auto* entry : valueType->entries)
                                    parameterValueStrings.add (entry->name);

                                parameterValueStrings.removeEmptyStrings();
                            }
                        }

                        isDiscrete = (numSteps != AudioProcessor::getDefaultNumParameterSteps());
                    }
                }

                newParameterTree.addChild (std::make_unique<VSTParameter> (*this, paramName, shortParamNames, defaultValue,
                                                                           label, isAutomatable, isDiscrete, numSteps,
                                                                           isBoolSwitch, parameterValueStrings, valueType));
            }

            setParameterTree (std::move (newParameterTree));
        */
    }
    
    pub fn cleanup(&mut self)  {
        
        todo!();
        /*
            if (vstEffect != nullptr && vstEffect->magic == 0x56737450 /* 'VstP' */)
            {
               #if ALOE_MAC
                if (vstModule->resFileId != 0)
                    UseResFile (vstModule->resFileId);
               #endif

                // Must delete any editors before deleting the plugin instance!
                jassert (getActiveEditor() == nullptr);

                _fpreset(); // some dodgy plug-ins mess around with this

                vstModule->closeEffect (vstEffect);
            }

            vstModule = nullptr;
            vstEffect = nullptr;
        */
    }
    
    pub fn create(
        new_module:          &ModuleHandlePtr,
        initial_sample_rate: f64,
        initial_block_size:  i32) -> *mut VSTPluginInstance {
        
        todo!();
        /*
            if (auto* newEffect = constructEffect (newModule))
            {
                newEffect->resvd2 = 0;

                newEffect->dispatcher (newEffect, typename Vst2EffIdentify, 0, 0, nullptr, 0);

                auto blockSize = jmax (32, initialBlockSize);

                newEffect->dispatcher (newEffect, typename Vst2EffSetSampleRate, 0, 0, nullptr, static_cast<float> (initialSampleRate));
                newEffect->dispatcher (newEffect, typename Vst2EffSetBlockSize,  0, blockSize, nullptr, 0);

                newEffect->dispatcher (newEffect, typename Vst2EffOpen, 0, 0, nullptr, 0);
                BusesProperties ioConfig = queryBusIO (newEffect);

                return new VSTPluginInstance (newModule, ioConfig, newEffect, initialSampleRate, blockSize);
            }

            return nullptr;
        */
    }
    
    pub fn fill_in_plugin_description(&self, desc: &mut PluginDescription)  {
        
        todo!();
        /*
            desc.name = name;

            {
                char buffer[512] = { 0 };
                dispatch (typename Vst2EffGetEffectName, 0, 0, buffer, 0);

                desc.descriptiveName = String::createStringFromData (buffer, (int) sizeof (buffer)).trim();

                if (desc.descriptiveName.isEmpty())
                    desc.descriptiveName = name;
            }

            desc.fileOrIdentifier = vstModule->file.getFullPathName();
            desc.uniqueId = desc.deprecatedUid = getUID();
            desc.lastFileModTime = vstModule->file.getLastModificationTime();
            desc.lastInfoUpdateTime = Time::getCurrentTime();
            desc.pluginFormatName = "VST";
            desc.category = getCategory();

            {
                char buffer[512] = { 0 };
                dispatch (typename Vst2EffGetVendorString, 0, 0, buffer, 0);
                desc.manufacturerName = String::createStringFromData (buffer, (int) sizeof (buffer)).trim();
            }

            desc.version = getVersion();
            desc.numInputChannels = getTotalNumInputChannels();
            desc.numOutputChannels = getTotalNumOutputChannels();
            desc.isInstrument = isSynthPlugin();
        */
    }
    
    pub fn initialise_effect(&mut self, 
        initial_sample_rate: f64,
        initial_block_size:  i32) -> bool {
        
        todo!();
        /*
            if (vstEffect != nullptr)
            {
                vstEffect->resvd2 = (pointer_sized_int) (pointer_sized_int) this;
                initialise (initialSampleRate, initialBlockSize);
                return true;
            }

            return false;
        */
    }
    
    pub fn initialise(&mut self, 
        initial_sample_rate: f64,
        initial_block_size:  i32)  {
        
        todo!();
        /*
            if (initialised || vstEffect == nullptr)
                return;

           #if ALOE_WINDOWS
            // On Windows it's highly advisable to create your plugins using the message thread,
            // because many plugins need a chance to create HWNDs that will get their
            // messages delivered by the main message thread, and that's not possible from
            // a background thread.
            ALOE_ASSERT_MESSAGE_THREAD
           #endif

            ALOE_VST_LOG ("Initialising VST: " + vstModule->pluginName + " (" + getVersion() + ")");
            initialised = true;

            setRateAndBufferSizeDetails (initialSampleRate, initialBlockSize);

            dispatch (typename Vst2EffIdentify, 0, 0, nullptr, 0);

            if (getSampleRate() > 0)
                dispatch (typename Vst2EffSetSampleRate, 0, 0, nullptr, (float) getSampleRate());

            if (getBlockSize() > 0)
                dispatch (typename Vst2EffSetBlockSize, 0, jmax (32, getBlockSize()), nullptr, 0);

            dispatch (typename Vst2EffOpen, 0, 0, nullptr, 0);

            setRateAndBufferSizeDetails (getSampleRate(), getBlockSize());

            if (getNumPrograms() > 1)
                setCurrentProgram (0);
            else
                dispatch (typename Vst2EffSetProgram, 0, 0, nullptr, 0);

            for (int i = vstEffect->numInputs;  --i >= 0;)  dispatch (typename Vst2EffConnectInput,  i, 1, nullptr, 0);
            for (int i = vstEffect->numOutputs; --i >= 0;)  dispatch (typename Vst2EffConnectOutput, i, 1, nullptr, 0);

            if (getVstCategory() != typename Vst2kPlugCategShell) // (workaround for Waves 5 plugins which crash during this call)
                updateStoredProgramNames();

            wantsMidiMessages = pluginCanDo ("receiveVstMidiEvent") > 0 || isSynthPlugin();

           #if ALOE_MAC && ALOE_SUPPORT_CARBON
            usesCocoaNSView = ((unsigned int) pluginCanDo ("hasCockosViewAsConfig") & 0xffff0000ul) == 0xbeef0000ul;
           #endif

            setLatencySamples (vstEffect->initialDelay);
        */
    }
    
    pub fn get_extensions(&self, visitor: &mut ExtensionsVisitor)  {
        
        todo!();
        /*
            struct Extensions : public ExtensionsVisitor::VSTClient
            {
                explicit Extensions (const VSTPluginInstance* instanceIn) : instance (instanceIn) {}

                void* getAEffectPtr() const  override   { return instance->vstEffect; }

                const VSTPluginInstance* instance = nullptr;
            };

            visitor.visitVSTClient (Extensions { this });
        */
    }
    
    pub fn get_platform_specific_data(&mut self)  {
        
        todo!();
        /*
            return vstEffect;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            if (vstEffect != nullptr)
            {
                char buffer[512] = { 0 };

                if (dispatch (typename Vst2EffGetProductString, 0, 0, buffer, 0) != 0)
                {
                    String productName = String::createStringFromData (buffer, (int) sizeof (buffer));

                    if (productName.isNotEmpty())
                        return productName;
                }
            }

            return name;
        */
    }
    
    pub fn getuid(&self) -> i32 {
        
        todo!();
        /*
            int uid = vstEffect != nullptr ? vstEffect->uniqueID : 0;

            if (uid == 0)
                uid = vstModule->file.hashCode();

            return uid;
        */
    }
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            if (vstEffect == nullptr)
                return 0.0;

            if ((vstEffect->flags & typename Vst2EffFlagsNoSoundInStop) != 0)
                return 0.0;

            auto tailSize = dispatch (typename Vst2EffGetTailSize, 0, 0, nullptr, 0);
            auto sampleRate = getSampleRate();

            // remain backward compatible with old Aloe plug-ins: anything larger
            // than INT32_MAX is an invalid tail time but old Aloe 64-bit plug-ins
            // would return INT64_MAX for infinite tail time. So treat anything
            // equal or greater than INT32_MAX as infinite tail time.
            if (tailSize >= std::numeric_limits<int32>::max())
                return std::numeric_limits<double>::infinity();

            if (tailSize >= 0 && sampleRate > 0)
                return static_cast<double> (tailSize) / sampleRate;

            return 0.0;
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return wantsMidiMessages;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return pluginCanDo ("sendVstMidiEvent") > 0;
        */
    }
    
    pub fn supportsmpe(&self) -> bool {
        
        todo!();
        /*
            return pluginCanDo ("MPE") > 0;
        */
    }
    
    pub fn get_vst_category(&self) -> PlugCategory {
        
        todo!();
        /*
            return (typename Vst2VstPlugCategory) dispatch (typename Vst2EffGetPlugCategory, 0, 0, nullptr, 0);
        */
    }
    
    pub fn is_synth_plugin(&self) -> bool {
        
        todo!();
        /*
            return (vstEffect != nullptr && (vstEffect->flags & typename Vst2EffFlagsIsSynth) != 0);
        */
    }
    
    pub fn plugin_can_do(&self, text: *const u8) -> i32 {
        
        todo!();
        /*
            return (int) dispatch (typename Vst2EffCanDo, 0, 0, (void*) text,  0);
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        rate:                       f64,
        samples_per_block_expected: i32)  {
        
        todo!();
        /*
            auto numInputBuses  = getBusCount (true);
            auto numOutputBuses = getBusCount (false);

            setRateAndBufferSizeDetails (rate, samplesPerBlockExpected);

            if (numInputBuses <= 1 && numOutputBuses <= 1)
            {
                SpeakerMappings::VstSpeakerConfigurationHolder inArr  (getChannelLayoutOfBus (true,  0));
                SpeakerMappings::VstSpeakerConfigurationHolder outArr (getChannelLayoutOfBus (false, 0));

                dispatch (typename Vst2EffSetSpeakerArrangement, 0, (pointer_sized_int) &inArr.get(), (void*) &outArr.get(), 0.0f);
            }

            vstHostTime.tempo = 120.0;
            vstHostTime.timeSigNumerator = 4;
            vstHostTime.timeSigDenominator = 4;
            vstHostTime.sampleRate = rate;
            vstHostTime.samplePos = 0;
            vstHostTime.flags = typename Vst2kVstNanosValid
                                  | typename Vst2kVstAutomationWriting
                                  | typename Vst2kVstAutomationReading;

            initialise (rate, samplesPerBlockExpected);

            if (initialised)
            {
                wantsMidiMessages = wantsMidiMessages || (pluginCanDo ("receiveVstMidiEvent") > 0) || isSynthPlugin();

                if (wantsMidiMessages)
                    midiEventsToSend.ensureSize (256);
                else
                    midiEventsToSend.freeEvents();

                incomingMidi.clear();

                dispatch (typename Vst2EffSetSampleRate, 0, 0, nullptr, (float) rate);
                dispatch (typename Vst2EffSetBlockSize, 0, jmax (16, samplesPerBlockExpected), nullptr, 0);

                if (supportsDoublePrecisionProcessing())
                {
                    int32 vstPrecision = isUsingDoublePrecision() ? typename Vst2kVstProcessPrecision64
                                                                  : typename Vst2kVstProcessPrecision32;

                    dispatch (typename Vst2EffSetProcessPrecision, 0, (pointer_sized_int) vstPrecision, nullptr, 0);
                }

                auto maxChannels = jmax (1, jmax (vstEffect->numInputs, vstEffect->numOutputs));

                tmpBufferFloat .setSize (maxChannels, samplesPerBlockExpected);
                tmpBufferDouble.setSize (maxChannels, samplesPerBlockExpected);

                channelBufferFloat .calloc (static_cast<size_t> (maxChannels));
                channelBufferDouble.calloc (static_cast<size_t> (maxChannels));

                outOfPlaceBuffer.setSize (jmax (1, vstEffect->numOutputs), samplesPerBlockExpected);

                if (! isPowerOn)
                    setPower (true);

                // dodgy hack to force some plugins to initialise the sample rate..
                if (! hasEditor())
                {
                    if (auto* firstParam = getParameters()[0])
                    {
                        auto old = firstParam->getValue();
                        firstParam->setValue ((old < 0.5f) ? 1.0f : 0.0f);
                        firstParam->setValue (old);
                    }
                }

                dispatch (typename Vst2EffStartProcess, 0, 0, nullptr, 0);

                setLatencySamples (vstEffect->initialDelay);
            }
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            if (initialised)
            {
                dispatch (typename Vst2EffStopProcess, 0, 0, nullptr, 0);
                setPower (false);
            }

            channelBufferFloat.free();
            tmpBufferFloat.setSize (0, 0);

            channelBufferDouble.free();
            tmpBufferDouble.setSize (0, 0);

            outOfPlaceBuffer.setSize (1, 1);
            incomingMidi.clear();

            midiEventsToSend.freeEvents();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            if (isPowerOn)
            {
                setPower (false);
                setPower (true);
            }
        */
    }
    
    pub fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            jassert (! isUsingDoublePrecision());
            processAudio (buffer, midiMessages, tmpBufferFloat, channelBufferFloat, false);
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
            processAudio (buffer, midiMessages, tmpBufferDouble, channelBufferDouble, false);
        */
    }
    
    pub fn process_block_bypassed(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            jassert (! isUsingDoublePrecision());
            processAudio (buffer, midiMessages, tmpBufferFloat, channelBufferFloat, true);
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
            processAudio (buffer, midiMessages, tmpBufferDouble, channelBufferDouble, true);
        */
    }
    
    pub fn supports_double_precision_processing(&self) -> bool {
        
        todo!();
        /*
            return ((vstEffect->flags & typename Vst2EffFlagsCanReplacing) != 0
                 && (vstEffect->flags & typename Vst2EffFlagsCanDoubleReplacing) != 0);
        */
    }
    
    pub fn get_bypass_parameter(&self) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            return vstSupportsBypass ? bypassParam.get() : nullptr;
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
    
    pub fn is_buses_layout_supported(&self, layouts: &BusLayout) -> bool {
        
        todo!();
        /*
            auto numInputBuses  = getBusCount (true);
            auto numOutputBuses = getBusCount (false);

            // it's not possible to change layout if there are sidechains/aux buses
            if (numInputBuses > 1 || numOutputBuses > 1)
                return (layouts == getBusesLayout());

            return (layouts.getNumChannels (true,  0) <= vstEffect->numInputs
                 && layouts.getNumChannels (false, 0) <= vstEffect->numOutputs);
        */
    }
    
    #[cfg(any(target_os="ios",target_os="android"))]
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }

    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return vstEffect != nullptr && (vstEffect->flags & typename Vst2EffFlagsHasEditor) != 0;
        */
    }
    
    pub fn get_input_channel_name(&self, index: i32) -> String {
        
        todo!();
        /*
            if (isValidChannel (index, true))
            {
                typename Vst2VstPinProperties pinProps;
                if (dispatch (typename Vst2EffGetInputProperties, index, 0, &pinProps, 0.0f) != 0)
                    return String (pinProps.label, sizeof (pinProps.label));
            }

            return {};
        */
    }
    
    pub fn is_input_channel_stereo_pair(&self, index: i32) -> bool {
        
        todo!();
        /*
            if (! isValidChannel (index, true))
                return false;

            typename Vst2VstPinProperties pinProps;
            if (dispatch (typename Vst2EffGetInputProperties, index, 0, &pinProps, 0.0f) != 0)
                return (pinProps.flags & typename Vst2kVstPinIsStereo) != 0;

            return true;
        */
    }
    
    pub fn get_output_channel_name(&self, index: i32) -> String {
        
        todo!();
        /*
            if (isValidChannel (index, false))
            {
                typename Vst2VstPinProperties pinProps;
                if (dispatch (typename Vst2EffGetOutputProperties, index, 0, &pinProps, 0.0f) != 0)
                    return String (pinProps.label, sizeof (pinProps.label));
            }

            return {};
        */
    }
    
    pub fn is_output_channel_stereo_pair(&self, index: i32) -> bool {
        
        todo!();
        /*
            if (! isValidChannel (index, false))
                return false;

            typename Vst2VstPinProperties pinProps;
            if (dispatch (typename Vst2EffGetOutputProperties, index, 0, &pinProps, 0.0f) != 0)
                return (pinProps.flags & typename Vst2kVstPinIsStereo) != 0;

            return true;
        */
    }
    
    pub fn is_valid_channel(&self, 
        index:    i32,
        is_input: bool) -> bool {
        
        todo!();
        /*
            return isPositiveAndBelow (index, isInput ? getTotalNumInputChannels()
                                                      : getTotalNumOutputChannels());
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return vstEffect != nullptr ? jmax (0, vstEffect->numPrograms) : 0;
        */
    }

    /**
       NB: some plugs return negative numbers from
       this function.
      */
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) dispatch (typename Vst2EffGetProgram, 0, 0, nullptr, 0);
        */
    }
    
    pub fn set_current_program(&mut self, new_index: i32)  {
        
        todo!();
        /*
            if (getNumPrograms() > 0 && newIndex != getCurrentProgram())
                dispatch (typename Vst2EffSetProgram, 0, jlimit (0, getNumPrograms() - 1, newIndex), nullptr, 0);
        */
    }
    
    pub fn get_program_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
            if (index >= 0)
            {
                if (index == getCurrentProgram())
                    return getCurrentProgramName();

                if (vstEffect != nullptr)
                {
                    char nm[264] = { 0 };

                    if (dispatch (typename Vst2EffGetProgramNameIndexed, jlimit (0, getNumPrograms(), index), -1, nm, 0) != 0)
                        return String::fromUTF8 (nm).trim();
                }
            }

            return {};
        */
    }
    
    pub fn change_program_name(&mut self, 
        index:    i32,
        new_name: &String)  {
        
        todo!();
        /*
            if (index >= 0 && index == getCurrentProgram())
            {
                if (getNumPrograms() > 0 && newName != getCurrentProgramName())
                    dispatch (typename Vst2EffSetProgramName, 0, 0, (void*) newName.substring (0, 24).toRawUTF8(), 0.0f);
            }
            else
            {
                jassertfalse; // xxx not implemented!
            }
        */
    }
    
    pub fn get_state_information(&mut self, mb: &mut MemoryBlock)  {
        
        todo!();
        /*
            saveToFXBFile (mb, true);
        */
    }
    
    pub fn get_current_program_state_information(&mut self, mb: &mut MemoryBlock)  {
        
        todo!();
        /*
            saveToFXBFile (mb, false);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data: *const c_void,
        size: i32)  {
        
        todo!();
        /*
            loadFromFXBFile (data, (size_t) size);
        */
    }
    
    pub fn set_current_program_state_information(&mut self, 
        data: *const c_void,
        size: i32)  {
        
        todo!();
        /*
            loadFromFXBFile (data, (size_t) size);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (dispatch (typename Vst2EffIdle, 0, 0, nullptr, 0) == 0)
                stopTimer();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            updateHostDisplay (AudioProcessorListener::ChangeDetails().withProgramChanged (true)
                                                                      .withParameterInfoChanged (true));
        */
    }
    
    pub fn handle_callback(&mut self, 
        opcode: i32,
        index:  i32,
        value:  PointerSizedInt,
        ptr:    *mut c_void,
        opt:    f32) -> PointerSizedInt {
        
        todo!();
        /*
            switch (opcode)
            {
                case typename Vst2audioMasterAutomate:
                    if (auto* param = getParameters()[index])
                        param->sendValueChangedMessageToListeners (opt);
                    else
                        jassertfalse; // Invalid parameter index!

                    break;

                case typename Vst2audioMasterProcessEvents:            handleMidiFromPlugin ((const typename Vst2VstEvents*) ptr); break;
                case typename Vst2audioMasterGetTime:                  return getVSTTime();
                case typename Vst2audioMasterIdle:                     handleIdle(); break;
                case typename Vst2audioMasterSizeWindow:               setWindowSize (index, (int) value); return 1;
                case typename Vst2audioMasterUpdateDisplay:            triggerAsyncUpdate(); break;
                case typename Vst2audioMasterIOChanged:                setLatencySamples (vstEffect->initialDelay); break;
                case typename Vst2audioMasterNeedIdle:                 startTimer (50); break;

                case typename Vst2audioMasterGetSampleRate:            return (pointer_sized_int) (getSampleRate() > 0 ? getSampleRate() : defaultVSTSampleRateValue);
                case typename Vst2audioMasterGetBlockSize:             return (pointer_sized_int) (getBlockSize() > 0  ? getBlockSize()  : defaultVSTBlockSizeValue);
                case typename Vst2audioMasterWantMidi:                 wantsMidiMessages = true; break;
                case typename Vst2audioMasterGetDirectory:             return getVstDirectory();

                case typename Vst2audioMasterTempoAt:                  return (pointer_sized_int) (extraFunctions != nullptr ? extraFunctions->getTempoAt ((int64) value) : 0);
                case typename Vst2audioMasterGetAutomationState:       return (pointer_sized_int) (extraFunctions != nullptr ? extraFunctions->getAutomationState() : 0);

                case typename Vst2audioMasterBeginEdit:
                    if (auto* param = getParameters()[index])
                        param->beginChangeGesture();
                    else
                        jassertfalse; // Invalid parameter index!

                    break;

                case typename Vst2audioMasterEndEdit:
                    if (auto* param = getParameters()[index])
                        param->endChangeGesture();
                    else
                        jassertfalse; // Invalid parameter index!

                    break;

                case typename Vst2audioMasterPinConnected:             return isValidChannel (index, value == 0) ? 0 : 1; // (yes, 0 = true)
                case typename Vst2audioMasterGetCurrentProcessLevel:   return isNonRealtime() ? 4 : 0;

                // none of these are handled (yet)...
                case typename Vst2audioMasterSetTime:
                case typename Vst2audioMasterGetParameterQuantization:
                case typename Vst2audioMasterGetInputLatency:
                case typename Vst2audioMasterGetOutputLatency:
                case typename Vst2audioMasterGetPreviousPlug:
                case typename Vst2audioMasterGetNextPlug:
                case typename Vst2audioMasterWillReplaceOrAccumulate:
                case typename Vst2audioMasterOfflineStart:
                case typename Vst2audioMasterOfflineRead:
                case typename Vst2audioMasterOfflineWrite:
                case typename Vst2audioMasterOfflineGetCurrentPass:
                case typename Vst2audioMasterOfflineGetCurrentMetaPass:
                case typename Vst2audioMasterGetOutputSpeakerArrangement:
                case typename Vst2audioMasterVendorSpecific:
                case typename Vst2audioMasterSetIcon:
                case typename Vst2audioMasterGetLanguage:
                case typename Vst2audioMasterOpenWindow:
                case typename Vst2audioMasterCloseWindow:
                    break;

                default:
                    return handleGeneralCallback (opcode, index, value, ptr, opt);
            }

            return 0;
        */
    }

    /**
       handles non plugin-specific callbacks..
      */
    pub fn handle_general_callback(
        opcode: i32,
        index:  i32,
        value:  PointerSizedInt,
        ptr:    *mut c_void,
        opt:    f32) -> PointerSizedInt {
        
        todo!();
        /*
            switch (opcode)
            {
                case typename Vst2audioMasterCanDo:                        return handleCanDo ((const char*) ptr);
                case typename Vst2audioMasterVersion:                      return 2400;
                case typename Vst2audioMasterCurrentId:                    return shellUIDToCreate;
                case typename Vst2audioMasterGetNumAutomatableParameters:  return 0;
                case typename Vst2audioMasterGetAutomationState:           return 1;
                case typename Vst2audioMasterGetVendorVersion:             return 0x0101;

                case typename Vst2audioMasterGetVendorString:
                case typename Vst2audioMasterGetProductString:             return getHostName ((char*) ptr);

                case typename Vst2audioMasterGetSampleRate:                return (pointer_sized_int) defaultVSTSampleRateValue;
                case typename Vst2audioMasterGetBlockSize:                 return (pointer_sized_int) defaultVSTBlockSizeValue;
                case typename Vst2audioMasterSetOutputSampleRate:          return 0;

                default:
                    DBG ("*** Unhandled VST Callback: " + String ((int) opcode));
                    break;
            }

            return 0;
        */
    }
    
    pub fn dispatch(&self, 
        opcode: i32,
        index:  i32,
        value:  PointerSizedInt,
        ptr:    *mut c_void,
        opt:    f32) -> PointerSizedInt {
        
        todo!();
        /*
            pointer_sized_int result = 0;

            if (vstEffect != nullptr)
            {
                const ScopedLock sl (lock);
                const IdleCallRecursionPreventer icrp;

                try
                {
                   #if ALOE_MAC
                    auto oldResFile = CurResFile();

                    if (vstModule->resFileId != 0)
                        UseResFile (vstModule->resFileId);
                   #endif

                    result = vstEffect->dispatcher (vstEffect, opcode, index, value, ptr, opt);

                   #if ALOE_MAC
                    auto newResFile = CurResFile();

                    if (newResFile != oldResFile)  // avoid confusing the parent app's resource file with the plug-in's
                    {
                        vstModule->resFileId = newResFile;
                        UseResFile (oldResFile);
                    }
                   #endif
                }
                catch (...)
                {}
            }

            return result;
        */
    }
    
    pub fn load_from_fxb_file(&mut self, 
        data:      *const c_void,
        data_size: usize) -> bool {
        
        todo!();
        /*
            if (dataSize < 28)
                return false;

            auto set = (const fxSet*) data;

            if ((! compareMagic (set->chunkMagic, "CcnK")) || fxbSwap (set->version) > fxbVersionNum)
                return false;

            if (compareMagic (set->fxMagic, "FxBk"))
            {
                // bank of programs
                if (fxbSwap (set->numPrograms) >= 0)
                {
                    auto oldProg = getCurrentProgram();
                    auto numParams = fxbSwap (((const fxProgram*) (set->programs))->numParams);
                    auto progLen = (int) sizeof (fxProgram) + (numParams - 1) * (int) sizeof (float);

                    for (int i = 0; i < fxbSwap (set->numPrograms); ++i)
                    {
                        if (i != oldProg)
                        {
                            auto prog = addBytesToPointer (set->programs, i * progLen);

                            if (getAddressDifference (prog, set) >= (int) dataSize)
                                return false;

                            if (fxbSwap (set->numPrograms) > 0)
                                setCurrentProgram (i);

                            if (! restoreProgramSettings (prog))
                                return false;
                        }
                    }

                    if (fxbSwap (set->numPrograms) > 0)
                        setCurrentProgram (oldProg);

                    auto prog = addBytesToPointer (set->programs, oldProg * progLen);

                    if (getAddressDifference (prog, set) >= (int) dataSize)
                        return false;

                    if (! restoreProgramSettings (prog))
                        return false;
                }
            }
            else if (compareMagic (set->fxMagic, "FxCk"))
            {
                // single program
                auto prog = (const fxProgram*) data;

                if (! compareMagic (prog->chunkMagic, "CcnK"))
                    return false;

                changeProgramName (getCurrentProgram(), prog->prgName);

                for (int i = 0; i < fxbSwap (prog->numParams); ++i)
                    if (auto* param = getParameters()[i])
                        param->setValue (fxbSwapFloat (prog->params[i]));
            }
            else if (compareMagic (set->fxMagic, "FBCh"))
            {
                // non-preset chunk
                auto cset = (const fxChunkSet*) data;

                if ((size_t) fxbSwap (cset->chunkSize) + sizeof (fxChunkSet) - 8 > (size_t) dataSize)
                    return false;

                setChunkData (cset->chunk, fxbSwap (cset->chunkSize), false);
            }
            else if (compareMagic (set->fxMagic, "FPCh"))
            {
                // preset chunk
                auto cset = (const fxProgramSet*) data;

                if ((size_t) fxbSwap (cset->chunkSize) + sizeof (fxProgramSet) - 8 > (size_t) dataSize)
                    return false;

                setChunkData (cset->chunk, fxbSwap (cset->chunkSize), true);

                changeProgramName (getCurrentProgram(), cset->name);
            }
            else
            {
                return false;
            }

            return true;
        */
    }
    
    pub fn save_to_fxb_file(
        &mut self, 
        dest:       &mut MemoryBlock,
        isfxb:      bool,
        max_sizemb: Option<i32>

    ) -> bool {

        let max_sizemb: i32 = max_sizemb.unwrap_or(128);

        todo!();
        /*
            auto numPrograms = getNumPrograms();
            auto numParams = getParameters().size();

            if (usesChunks())
            {
                MemoryBlock chunk;
                getChunkData (chunk, ! isFXB, maxSizeMB);

                if (isFXB)
                {
                    auto totalLen = sizeof (fxChunkSet) + chunk.getSize() - 8;
                    dest.setSize (totalLen, true);

                    auto set = (fxChunkSet*) dest.getData();
                    set->chunkMagic = fxbName ("CcnK");
                    set->byteSize = 0;
                    set->fxMagic = fxbName ("FBCh");
                    set->version = fxbSwap (fxbVersionNum);
                    set->fxID = fxbSwap (getUID());
                    set->fxVersion = fxbSwap (getVersionNumber());
                    set->numPrograms = fxbSwap (numPrograms);
                    set->chunkSize = fxbSwap ((int32) chunk.getSize());

                    chunk.copyTo (set->chunk, 0, chunk.getSize());
                }
                else
                {
                    auto totalLen = sizeof (fxProgramSet) + chunk.getSize() - 8;
                    dest.setSize (totalLen, true);

                    auto set = (fxProgramSet*) dest.getData();
                    set->chunkMagic = fxbName ("CcnK");
                    set->byteSize = 0;
                    set->fxMagic = fxbName ("FPCh");
                    set->version = fxbSwap (fxbVersionNum);
                    set->fxID = fxbSwap (getUID());
                    set->fxVersion = fxbSwap (getVersionNumber());
                    set->numPrograms = fxbSwap (numPrograms);
                    set->chunkSize = fxbSwap ((int32) chunk.getSize());

                    getCurrentProgramName().copyToUTF8 (set->name, sizeof (set->name) - 1);
                    chunk.copyTo (set->chunk, 0, chunk.getSize());
                }
            }
            else
            {
                if (isFXB)
                {
                    auto progLen = (int) sizeof (fxProgram) + (numParams - 1) * (int) sizeof (float);
                    auto len = (size_t) (progLen * jmax (1, numPrograms)) + (sizeof (fxSet) - sizeof (fxProgram));
                    dest.setSize (len, true);

                    auto set = (fxSet*) dest.getData();
                    set->chunkMagic = fxbName ("CcnK");
                    set->byteSize = 0;
                    set->fxMagic = fxbName ("FxBk");
                    set->version = fxbSwap (fxbVersionNum);
                    set->fxID = fxbSwap (getUID());
                    set->fxVersion = fxbSwap (getVersionNumber());
                    set->numPrograms = fxbSwap (numPrograms);

                    MemoryBlock oldSettings;
                    createTempParameterStore (oldSettings);

                    auto oldProgram = getCurrentProgram();

                    if (oldProgram >= 0)
                        setParamsInProgramBlock (addBytesToPointer (set->programs, oldProgram * progLen));

                    for (int i = 0; i < numPrograms; ++i)
                    {
                        if (i != oldProgram)
                        {
                            setCurrentProgram (i);
                            setParamsInProgramBlock (addBytesToPointer (set->programs, i * progLen));
                        }
                    }

                    if (oldProgram >= 0)
                        setCurrentProgram (oldProgram);

                    restoreFromTempParameterStore (oldSettings);
                }
                else
                {
                    dest.setSize ((size_t) ((numParams - 1) * (int) sizeof (float)) + sizeof (fxProgram), true);
                    setParamsInProgramBlock ((fxProgram*) dest.getData());
                }
            }

            return true;
        */
    }
    
    pub fn uses_chunks(&self) -> bool {
        
        todo!();
        /*
            return vstEffect != nullptr && (vstEffect->flags & typename Vst2EffFlagsProgramChunks) != 0;
        */
    }
    
    pub fn get_chunk_data(&self, 
        mb:         &mut MemoryBlock,
        is_preset:  bool,
        max_sizemb: i32) -> bool {
        
        todo!();
        /*
            if (usesChunks())
            {
                void* data = nullptr;
                auto bytes = (size_t) dispatch (typename Vst2EffGetChunk, isPreset ? 1 : 0, 0, &data, 0.0f);

                if (data != nullptr && bytes <= (size_t) maxSizeMB * 1024 * 1024)
                {
                    mb.setSize (bytes);
                    mb.copyFrom (data, 0, bytes);

                    return true;
                }
            }

            return false;
        */
    }
    
    pub fn set_chunk_data(&mut self, 
        data:      *const c_void,
        size:      i32,
        is_preset: bool) -> bool {
        
        todo!();
        /*
            if (size > 0 && usesChunks())
            {
                dispatch (typename Vst2EffSetChunk, isPreset ? 1 : 0, size, (void*) data, 0.0f);

                if (! isPreset)
                    updateStoredProgramNames();

                return true;
            }

            return false;
        */
    }
    
    pub fn update_size_from_editor(&mut self, w: i32, h: i32) -> bool {
        
        todo!();
        /*
            editorSize = { w, h };

            if (auto* editor = getActiveEditor())
            {
                editor->setSize (w, h);
                return true;
            }

            return false;
        */
    }
    
    pub fn get_editor_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return editorSize;
        */
    }
    
    pub fn handle_can_do(name: *const u8) -> PointerSizedInt {
        
        todo!();
        /*
            static const char* canDos[] = { "supplyIdle",
                                            "sendVstEvents",
                                            "sendVstMidiEvent",
                                            "sendVstTimeInfo",
                                            "receiveVstEvents",
                                            "receiveVstMidiEvent",
                                            "supportShell",
                                            "sizeWindow",
                                            "shellCategory" };

            for (int i = 0; i < numElementsInArray (canDos); ++i)
                if (strcmp (canDos[i], name) == 0)
                    return 1;

            return 0;
        */
    }
    
    pub fn get_host_name(name: *mut u8) -> PointerSizedInt {
        
        todo!();
        /*
            String hostName (ALOE_VST_FALLBACK_HOST_NAME);

            if (auto* app = ALOEApplicationBase::getInstance())
                hostName = app->getApplicationName();

            hostName.copyToUTF8 (name, (size_t) jmin (typename Vst2kVstMaxVendorStrLen, typename Vst2kVstMaxProductStrLen) - 1);
            return 1;
        */
    }
    
    pub fn get_vst_time(&mut self) -> PointerSizedInt {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (4311)

            return (pointer_sized_int) &vstHostTime;

            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
    
    pub fn handle_idle(&mut self)  {
        
        todo!();
        /*
            if (insideVSTCallback == 0 && MessageManager::getInstance()->isThisTheMessageThread())
            {
                const IdleCallRecursionPreventer icrp;

               #if ALOE_MAC
                if (getActiveEditor() != nullptr)
                    dispatch (typename Vst2EffEditIdle, 0, 0, nullptr, 0);
               #endif

                Timer::callPendingTimersSynchronously();
                handleUpdateNowIfNeeded();

                for (int i = ComponentPeer::getNumPeers(); --i >= 0;)
                    if (auto* p = ComponentPeer::getPeer(i))
                        p->performAnyPendingRepaintsNow();
            }
        */
    }
    
    pub fn set_window_size(&mut self, 
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            if (auto* ed = getActiveEditor())
            {
               #if ALOE_LINUX || ALOE_BSD
                const MessageManagerLock mmLock;
               #endif

               #if ! ALOE_MAC
                if (auto* peer = ed->getTopLevelComponent()->getPeer())
                {
                    auto scale = peer->getPlatformScaleFactor();
                    updateSizeFromEditor (roundToInt (width / scale), roundToInt (height / scale));

                    return;
                }
               #endif

                updateSizeFromEditor (width, height);
            }
        */
    }
    
    pub fn construct_effect(module: &ModuleHandlePtr) -> *mut AEffect {
        
        todo!();
        /*
            typename Vst2AEffect* effect = nullptr;
            try
            {
                const IdleCallRecursionPreventer icrp;
                _fpreset();

                ALOE_VST_LOG ("Creating VST instance: " + module->pluginName);

               #if ALOE_MAC
                if (module->resFileId != 0)
                    UseResFile (module->resFileId);
               #endif

                {
                    ALOE_VST_WRAPPER_INVOKE_MAIN
                }

                if (effect != nullptr && effect->magic == 0x56737450 /* 'VstP' */)
                {
                    jassert (effect->resvd2 == 0);
                    jassert (effect->object != nullptr);

                    _fpreset(); // some dodgy plugs mess around with this
                }
                else
                {
                    effect = nullptr;
                }
            }
            catch (...)
            {}

            return effect;
        */
    }
    
    pub fn query_busio(effect: *mut AEffect) -> AudioProcessorBusProperties {
        
        todo!();
        /*
            BusesProperties returnValue;

            if (effect->numInputs == 0 && effect->numOutputs == 0)
                return returnValue;

            // Workaround for old broken Aloe plug-ins which would return an invalid
            // speaker arrangement if the host didn't ask for a specific arrangement
            // beforehand.
            // Check if the plug-in reports any default layouts. If it doesn't, then
            // try setting a default layout compatible with the number of pins this
            // plug-in is reporting.
            if (! pluginHasDefaultChannelLayouts (effect))
            {
                SpeakerMappings::VstSpeakerConfigurationHolder canonicalIn  (AudioChannelSet::canonicalChannelSet (effect->numInputs));
                SpeakerMappings::VstSpeakerConfigurationHolder canonicalOut (AudioChannelSet::canonicalChannelSet (effect->numOutputs));

                effect->dispatcher (effect, typename Vst2EffSetSpeakerArrangement, 0,
                                          (pointer_sized_int) &canonicalIn.get(), (void*) &canonicalOut.get(), 0.0f);
            }

            const auto arrangement = getSpeakerArrangementWrapper (effect);

            for (int dir = 0; dir < 2; ++dir)
            {
                const bool isInput = (dir == 0);
                const int opcode = (isInput ? typename Vst2EffGetInputProperties : typename Vst2EffGetOutputProperties);
                const int maxChannels = (isInput ? effect->numInputs : effect->numOutputs);
                const auto* arr = (isInput ? arrangement.in : arrangement.out);
                bool busAdded = false;

                typename Vst2VstPinProperties pinProps;
                AudioChannelSet layout;

                for (int ch = 0; ch < maxChannels; ch += layout.size())
                {
                    if (effect->dispatcher (effect, opcode, ch, 0, &pinProps, 0.0f) == 0)
                        break;

                    if ((pinProps.flags & typename Vst2kVstPinUseSpeaker) != 0)
                    {
                        layout = SpeakerMappings::vstArrangementTypeToChannelSet (pinProps.arrangementType, 0);

                        if (layout.isDisabled())
                            break;
                    }
                    else if (arr == nullptr)
                    {
                        layout = ((pinProps.flags & typename Vst2kVstPinIsStereo) != 0 ? AudioChannelSet::stereo() : AudioChannelSet::mono());
                    }
                    else
                        break;

                    busAdded = true;
                    returnValue.addBus (isInput, pinProps.label, layout, true);
                }

                // no buses?
                if (! busAdded && maxChannels > 0)
                {
                    String busName = (isInput ? "Input" : "Output");

                    if (effect->dispatcher (effect, opcode, 0, 0, &pinProps, 0.0f) != 0)
                        busName = pinProps.label;

                    if (arr != nullptr)
                        layout = SpeakerMappings::vstArrangementTypeToChannelSet (*arr);
                    else
                        layout = AudioChannelSet::canonicalChannelSet (maxChannels);

                    returnValue.addBus (isInput, busName, layout, true);
                }
            }

            return returnValue;
        */
    }
    
    pub fn plugin_has_default_channel_layouts(effect: *mut AEffect) -> bool {
        
        todo!();
        /*
            if (getSpeakerArrangementWrapper (effect).isValid())
                return true;

            for (int dir = 0; dir < 2; ++dir)
            {
                const bool isInput = (dir == 0);
                const int opcode = (isInput ? typename Vst2EffGetInputProperties : typename Vst2EffGetOutputProperties);
                const int maxChannels = (isInput ? effect->numInputs : effect->numOutputs);

                int channels = 1;

                for (int ch = 0; ch < maxChannels; ch += channels)
                {
                    typename Vst2VstPinProperties pinProps;

                    if (effect->dispatcher (effect, opcode, ch, 0, &pinProps, 0.0f) == 0)
                        return false;

                    if ((pinProps.flags & typename Vst2kVstPinUseSpeaker) != 0)
                        return true;

                    channels = (pinProps.flags & typename Vst2kVstPinIsStereo) != 0 ? 2 : 1;
                }
            }

            return false;
        */
    }
    
    pub fn get_speaker_arrangement_wrapper(effect: *mut AEffect) -> VSTPluginInstanceSpeakerArrangements {
        
        todo!();
        /*
            // Workaround: unfortunately old Aloe VST-2 plug-ins had a bug and would crash if
            // you try to get the speaker arrangement when there are no input channels present.
            // Hopefully, one day (when there are no more old Aloe plug-ins around), we can
            // comment out the next two lines.
            if (effect->numInputs == 0)
                return { nullptr, nullptr };

            VSTPluginInstanceSpeakerArrangements result { nullptr, nullptr };
            const auto dispatchResult = effect->dispatcher (effect,
                                                            typename Vst2EffGetSpeakerArrangement,
                                                            0,
                                                            reinterpret_cast<pointer_sized_int> (&result.in),
                                                            &result.out,
                                                            0.0f);

            if (dispatchResult != 0)
                return result;

            return { nullptr, nullptr };
        */
    }
    
    pub fn process_audio<FloatType>(&mut self, 
        buffer:                        &mut AudioBuffer<FloatType>,
        midi_messages:                 &mut MidiBuffer,
        tmp_buffer:                    &mut AudioBuffer<FloatType>,
        channel_buffer:                &mut HeapBlock<*mut FloatType>,
        process_block_bypassed_called: bool)  {
    
        todo!();
        /*
            if (vstSupportsBypass)
            {
                updateBypass (processBlockBypassedCalled);
            }
            else if (processBlockBypassedCalled)
            {
                // if this vst does not support bypass then we will have to do this ourselves
                AudioProcessor::processBlockBypassed (buffer, midiMessages);
                return;
            }

            auto numSamples  = buffer.getNumSamples();
            auto numChannels = buffer.getNumChannels();

            if (initialised)
            {
                if (auto* currentPlayHead = getPlayHead())
                {
                    AudioPlayHeadCurrentPositionInfo position;

                    if (currentPlayHead->getCurrentPosition (position))
                    {

                        vstHostTime.samplePos          = (double) position.timeInSamples;
                        vstHostTime.tempo              = position.bpm;
                        vstHostTime.timeSigNumerator   = position.timeSigNumerator;
                        vstHostTime.timeSigDenominator = position.timeSigDenominator;
                        vstHostTime.ppqPos             = position.ppqPosition;
                        vstHostTime.barStartPos        = position.ppqPositionOfLastBarStart;
                        vstHostTime.flags |= typename Vst2kVstTempoValid
                                               | typename Vst2kVstTimeSigValid
                                               | typename Vst2kVstPpqPosValid
                                               | typename Vst2kVstBarsValid;

                        int32 newTransportFlags = 0;
                        if (position.isPlaying)     newTransportFlags |= typename Vst2kVstTransportPlaying;
                        if (position.isRecording)   newTransportFlags |= typename Vst2kVstTransportRecording;

                        if (newTransportFlags != (vstHostTime.flags & (typename Vst2kVstTransportPlaying
                                                                       | typename Vst2kVstTransportRecording)))
                            vstHostTime.flags = (vstHostTime.flags & ~(typename Vst2kVstTransportPlaying | typename Vst2kVstTransportRecording)) | newTransportFlags | typename Vst2kVstTransportChanged;
                        else
                            vstHostTime.flags &= ~typename Vst2kVstTransportChanged;

                        switch (position.frameRate)
                        {
                            case AudioPlayHead::fps24:       setHostTimeFrameRate (typename Vst2kVstSmpte24fps, 24.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps25:       setHostTimeFrameRate (typename Vst2kVstSmpte25fps, 25.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps30:       setHostTimeFrameRate (typename Vst2kVstSmpte30fps, 30.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps60:       setHostTimeFrameRate (typename Vst2kVstSmpte60fps, 60.0, position.timeInSeconds); break;

                            case AudioPlayHead::fps23976:    setHostTimeFrameRateDrop (typename Vst2kVstSmpte239fps,   24.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps2997:     setHostTimeFrameRateDrop (typename Vst2kVstSmpte2997fps,  30.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps2997drop: setHostTimeFrameRateDrop (typename Vst2kVstSmpte2997dfps, 30.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps30drop:   setHostTimeFrameRateDrop (typename Vst2kVstSmpte30dfps,   30.0, position.timeInSeconds); break;
                            case AudioPlayHead::fps60drop:   setHostTimeFrameRateDrop (typename Vst2kVstSmpte599fps,   60.0, position.timeInSeconds); break;
                            case AudioPlayHead::fpsUnknown:
                            default: break;
                        }

                        if (position.isLooping)
                        {
                            vstHostTime.cycleStartPos = position.ppqLoopStart;
                            vstHostTime.cycleEndPos   = position.ppqLoopEnd;
                            vstHostTime.flags |= (typename Vst2kVstCyclePosValid | typename Vst2kVstTransportCycleActive);
                        }
                        else
                        {
                            vstHostTime.flags &= ~(typename Vst2kVstCyclePosValid | typename Vst2kVstTransportCycleActive);
                        }
                    }
                }

                vstHostTime.nanoSeconds = getVSTHostTimeNanoseconds();

                if (wantsMidiMessages)
                {
                    midiEventsToSend.clear();
                    midiEventsToSend.ensureSize (1);

                    for (const auto metadata : midiMessages)
                        midiEventsToSend.addEvent (metadata.data, metadata.numBytes,
                                                   jlimit (0, numSamples - 1, metadata.samplePosition));

                    vstEffect->dispatcher (vstEffect, typename Vst2EffProcessEvents, 0, 0, midiEventsToSend.events, 0);
                }

                _clearfp();

                // always ensure that the buffer is at least as large as the maximum number of channels
                auto maxChannels = jmax (vstEffect->numInputs, vstEffect->numOutputs);
                auto channels = channelBuffer.get();

                if (numChannels < maxChannels)
                {
                    if (numSamples > tmpBuffer.getNumSamples())
                        tmpBuffer.setSize (tmpBuffer.getNumChannels(), numSamples);

                    tmpBuffer.clear();
                }

                for (int ch = 0; ch < maxChannels; ++ch)
                    channels[ch] = (ch < numChannels ? buffer.getWritePointer (ch) : tmpBuffer.getWritePointer (ch));

                {
                    AudioBuffer<FloatType> processBuffer (channels, maxChannels, numSamples);

                    invokeProcessFunction (processBuffer, numSamples);
                }
            }
            else
            {
                // Not initialised, so just bypass..
                for (int i = getTotalNumOutputChannels(); --i >= 0;)
                    buffer.clear (i, 0, buffer.getNumSamples());
            }

            {
                // copy any incoming midi..
                const ScopedLock sl (midiInLock);

                midiMessages.swapWith (incomingMidi);
                incomingMidi.clear();
            }
        */
    }
    
    #[inline] pub fn invoke_process_function(
        &mut self, 
        buffer:        &mut AudioBuffer<f32>,
        sample_frames: i32
    ) {
        
        todo!();
        /*
            if ((vstEffect->flags & typename Vst2EffFlagsCanReplacing) != 0)
            {
                vstEffect->processReplacing (vstEffect, buffer.getArrayOfWritePointers(),
                                                        buffer.getArrayOfWritePointers(), sampleFrames);
            }
            else
            {
                outOfPlaceBuffer.setSize (vstEffect->numOutputs, sampleFrames);
                outOfPlaceBuffer.clear();

                vstEffect->process (vstEffect, buffer.getArrayOfWritePointers(),
                                               outOfPlaceBuffer.getArrayOfWritePointers(), sampleFrames);

                for (int i = vstEffect->numOutputs; --i >= 0;)
                    buffer.copyFrom (i, 0, outOfPlaceBuffer.getReadPointer (i), sampleFrames);
            }
        */
    }
    
    #[inline] pub fn invoke_process_function_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        sample_frames: i32
    ) {
        
        todo!();
        /*
            vstEffect->processDoubleReplacing (vstEffect, buffer.getArrayOfWritePointers(),
                                                          buffer.getArrayOfWritePointers(), sampleFrames);
        */
    }
    
    pub fn set_host_time_frame_rate(&mut self, 
        frame_rate_index: i64,
        frame_rate:       f64,
        current_time:     f64)  {
        
        todo!();
        /*
            vstHostTime.flags |= typename Vst2kVstSmpteValid;
            vstHostTime.smpteFrameRate   = (int32) frameRateIndex;
            vstHostTime.smpteOffset = (int32) (currentTime * 80.0 * frameRate + 0.5);
        */
    }
    
    pub fn set_host_time_frame_rate_drop(&mut self, 
        frame_rate_index: i64,
        frame_rate:       f64,
        current_time:     f64)  {
        
        todo!();
        /*
            setHostTimeFrameRate (frameRateIndex, frameRate * 1000.0 / 1001.0, currentTime);
        */
    }
    
    pub fn restore_program_settings(&mut self, prog: *const fxProgram) -> bool {
        
        todo!();
        /*
            if (compareMagic (prog->chunkMagic, "CcnK")
                 && compareMagic (prog->fxMagic, "FxCk"))
            {
                changeProgramName (getCurrentProgram(), prog->prgName);

                for (int i = 0; i < fxbSwap (prog->numParams); ++i)
                    if (auto* param = getParameters()[i])
                        param->setValue (fxbSwapFloat (prog->params[i]));

                return true;
            }

            return false;
        */
    }
    
    pub fn get_text_for_opcode(&self, 
        index:  i32,
        opcode: i32) -> String {
        
        todo!();
        /*
            if (vstEffect == nullptr)
                return {};

            jassert (index >= 0 && index < vstEffect->numParams);
            char nm[256] = { 0 };
            dispatch (opcode, index, 0, nm, 0);
            return String::createStringFromData (nm, (int) sizeof (nm)).trim();
        */
    }
    
    pub fn get_current_program_name(&mut self) -> String {
        
        todo!();
        /*
            String progName;

            if (vstEffect != nullptr)
            {
                {
                    char nm[256] = { 0 };
                    dispatch (typename Vst2EffGetProgramName, 0, 0, nm, 0);
                    progName = String::createStringFromData (nm, (int) sizeof (nm)).trim();
                }

                const int index = getCurrentProgram();

                if (index >= 0 && programNames[index].isEmpty())
                {
                    while (programNames.size() < index)
                        programNames.add (String());

                    programNames.set (index, progName);
                }
            }

            return progName;
        */
    }
    
    pub fn set_params_in_program_block(&mut self, prog: *mut fxProgram)  {
        
        todo!();
        /*
            auto numParams = getParameters().size();

            prog->chunkMagic = fxbName ("CcnK");
            prog->byteSize = 0;
            prog->fxMagic = fxbName ("FxCk");
            prog->version = fxbSwap (fxbVersionNum);
            prog->fxID = fxbSwap (getUID());
            prog->fxVersion = fxbSwap (getVersionNumber());
            prog->numParams = fxbSwap (numParams);

            getCurrentProgramName().copyToUTF8 (prog->prgName, sizeof (prog->prgName) - 1);

            for (int i = 0; i < numParams; ++i)
                if (auto* param = getParameters()[i])
                    prog->params[i] = fxbSwapFloat (param->getValue());
        */
    }
    
    pub fn update_stored_program_names(&mut self)  {
        
        todo!();
        /*
            if (vstEffect != nullptr && getNumPrograms() > 0)
            {
                char nm[256] = { 0 };

                // only do this if the plugin can't use indexed names..
                if (dispatch (typename Vst2EffGetProgramNameIndexed, 0, -1, nm, 0) == 0)
                {
                    auto oldProgram = getCurrentProgram();
                    MemoryBlock oldSettings;
                    createTempParameterStore (oldSettings);

                    for (int i = 0; i < getNumPrograms(); ++i)
                    {
                        setCurrentProgram (i);
                        getCurrentProgramName();  // (this updates the list)
                    }

                    setCurrentProgram (oldProgram);
                    restoreFromTempParameterStore (oldSettings);
                }
            }
        */
    }
    
    pub fn handle_midi_from_plugin(&mut self, events: *const VstEvents)  {
        
        todo!();
        /*
            if (events != nullptr)
            {
                const ScopedLock sl (midiInLock);
                VSTMidiEventList::addEventsToMidiBuffer (events, incomingMidi);
            }
        */
    }
    
    pub fn create_temp_parameter_store(&mut self, dest: &mut MemoryBlock)  {
        
        todo!();
        /*
            auto numParameters = getParameters().size();
            dest.setSize (64 + 4 * (size_t) numParameters);
            dest.fillWith (0);

            getCurrentProgramName().copyToUTF8 ((char*) dest.getData(), 63);

            auto p = unalignedPointerCast<float*> (((char*) dest.getData()) + 64);

            for (int i = 0; i < numParameters; ++i)
                if (auto* param = getParameters()[i])
                    p[i] = param->getValue();
        */
    }
    
    pub fn restore_from_temp_parameter_store(&mut self, m: &MemoryBlock)  {
        
        todo!();
        /*
            changeProgramName (getCurrentProgram(), (const char*) m.getData());

            auto p = unalignedPointerCast<float*> (((char*) m.getData()) + 64);
            auto numParameters = getParameters().size();

            for (int i = 0; i < numParameters; ++i)
                if (auto* param = getParameters()[i])
                    param->setValue (p[i]);
        */
    }
    
    pub fn get_vst_directory(&self) -> PointerSizedInt {
        
        todo!();
        /*
            #if ALOE_MAC
            return (pointer_sized_int) (void*) &vstModule->parentDirFSSpec;
           #else
            return (pointer_sized_int) (pointer_sized_uint) vstModule->fullParentDirectoryPathName.toRawUTF8();
           #endif
        */
    }
    
    pub fn get_version_number(&self) -> i32 {
        
        todo!();
        /*
            return vstEffect != nullptr ? vstEffect->version : 0;
        */
    }
    
    pub fn get_version(&self) -> String {
        
        todo!();
        /*
            auto v = (unsigned int) dispatch (typename Vst2EffGetVendorVersion, 0, 0, nullptr, 0);

            String s;

            if (v == 0 || (int) v == -1)
                v = (unsigned int) getVersionNumber();

            if (v != 0)
            {
                // See yfede's post for the rational on this encoding
                // https://forum.aloe.com/t/issues-with-version-integer-reported-by-vst2/23867/6

                unsigned int major = 0, minor = 0, bugfix = 0, build = 0;

                if (v < 10)            // Encoding A
                {
                    major = v;
                }
                else if (v < 10000)    // Encoding B
                {
                    major  = (v / 1000);
                    minor  = (v % 1000) / 100;
                    bugfix = (v % 100)  / 10;
                    build  = (v % 10);
                }
                else if (v < 0x10000)  // Encoding C
                {
                    major  = (v / 10000);
                    minor  = (v % 10000) / 1000;
                    bugfix = (v % 1000)  / 100;
                    build  = (v % 100)   / 10;
                }
                else if (v < 0x650000) // Encoding D
                {
                    major  = (v >> 16) & 0xff;
                    minor  = (v >> 8)  & 0xff;
                    bugfix = (v >> 0)  & 0xff;
                }
                else                  // Encoding E
                {
                    major  = (v / 10000000);
                    minor  = (v % 10000000) / 100000;
                    bugfix = (v % 100000)   / 1000;
                    build  = (v % 1000);
                }

                s << (int) major << '.' << (int) minor << '.' << (int) bugfix << '.' << (int) build;
            }

            return s;
        */
    }
    
    pub fn get_category(&self) -> *const u8 {
        
        todo!();
        /*
            switch (getVstCategory())
            {
                case typename Vst2kPlugCategEffect:          return "Effect";
                case typename Vst2kPlugCategSynth:           return "Synth";
                case typename Vst2kPlugCategAnalysis:        return "Analysis";
                case typename Vst2kPlugCategMastering:       return "Mastering";
                case typename Vst2kPlugCategSpacializer:     return "Spacial";
                case typename Vst2kPlugCategRoomFx:          return "Reverb";
                case typename Vst2kPlugSurroundFx:           return "Surround";
                case typename Vst2kPlugCategRestoration:     return "Restoration";
                case typename Vst2kPlugCategGenerator:       return "Tone generation";
                case typename Vst2kPlugCategOfflineProcess:  return "Offline Process";
                case typename Vst2kPlugCategShell:           return "Shell";
                case typename Vst2kPlugCategUnknown:         return "Unknown";
                case typename Vst2kPlugCategMaxCount:
                default:                              break;
            }

            return nullptr;
        */
    }
    
    pub fn set_power(&mut self, on: bool)  {
        
        todo!();
        /*
            dispatch (typename Vst2EffMainsChanged, 0, on ? 1 : 0, nullptr, 0);
            isPowerOn = on;
        */
    }
    
    pub fn update_bypass(&mut self, process_block_bypassed_called: bool)  {
        
        todo!();
        /*
            if (processBlockBypassedCalled)
            {
                if (bypassParam->getValue() == 0.0f || ! lastProcessBlockCallWasBypass)
                    bypassParam->setValue (1.0f);
            }
            else
            {
                if (lastProcessBlockCallWasBypass)
                    bypassParam->setValue (0.0f);
            }

            lastProcessBlockCallWasBypass = processBlockBypassedCalled;
        */
    }

    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            #if ALOE_IOS || ALOE_ANDROID
        return nullptr;
       #else
        return hasEditor() ? new VSTPluginWindow (*this)
                           : nullptr;
       #endif
        */
    }
}
