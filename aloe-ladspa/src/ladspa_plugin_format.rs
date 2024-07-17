crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_LADSPAPluginFormat.h]

/**
  | Implements a plugin format manager
  | for LADSPA plugins.
  | 
  | @tags{Audio}
  |
  */
#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
#[no_copy]
#[leak_detector]
pub struct LADSPAPluginFormat {
    base: AudioPluginFormat,
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl LADSPAPluginFormat {

    pub fn get_format_name() -> String {
        
        todo!();
        /*
            return "LADSPA";
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return getFormatName();
        */
    }
    
    pub fn can_scan_for_plugins(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_trivial_to_scan(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn find_all_types_for_file(&mut self, 
        _0:                 &mut Vec<Box<PluginDescription>>,
        file_or_identifier: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn file_might_contain_this_plugin_type(&mut self, file_or_identifier: &String) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_name_of_plugin_from_identifier(&mut self, file_or_identifier: &String) -> String {
        
        todo!();
        /*
        
        */
    }
    
    pub fn plugin_needs_rescanning(&mut self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn search_paths_for_plugins(&mut self, 
        _0:        &FileSearchPath,
        recursive: bool,
        _2:        bool) -> StringArray {
        
        todo!();
        /*
        
        */
    }
    
    pub fn does_plugin_still_exist(&mut self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_default_locations_to_search(&mut self) -> FileSearchPath {
        
        todo!();
        /*
        
        */
    }
    
    pub fn create_plugin_instance(&mut self, 
        _0:                  &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32,
        _3:                  PluginCreationCallback)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn requires_unblocked_message_thread_during_creation(&self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn recursive_file_search(&mut self, 
        _0:        &mut StringArray,
        _1:        &File,
        recursive: bool)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_LADSPAPluginFormat.cpp]

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
lazy_static!{
    /*
    static int shellLADSPAUIDToCreate = 0;
    static int insideLADSPACallback = 0;
    */
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
pub const ALOE_LADSPA_LOGGING: usize = 1;

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
#[cfg(ALOE_LADSPA_LOGGING)]
macro_rules! aloe_ladspa_log {
    ($x:ident) => {
        /*
                Logger::writeToLog (x);
        */
    }
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
#[cfg(not(ALOE_LADSPA_LOGGING))]
macro_rules! aloe_ladspa_log { ($x:ident) => { } }

///-------------------
#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
#[no_copy]
#[leak_detector]
pub struct LADSPAModuleHandle {
    base:        ReferenceCountedObject,
    file:        File,
    module_main: LADSPA_Descriptor_Function, // default = nullptr
    module:      DynamicLibrary,
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl HasPtr for LADSPAModuleHandle {
    type Ptr = ReferenceCountedObjectPtr<LADSPAModuleHandle>;
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl Drop for LADSPAModuleHandle {
    fn drop(&mut self) {
        todo!();
        /*
            getActiveModules().removeFirstMatchingValue (this);
            close();
        */
    }
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl LADSPAModuleHandle {

    pub fn new(f: &File) -> Self {
    
        todo!();
        /*
        : file(f),

            getActiveModules().add (this);
        */
    }
    
    pub fn get_active_modules() -> &mut Vec<*mut LADSPAModuleHandle> {
        
        todo!();
        /*
            static Vec<LADSPAModuleHandle*> activeModules;
            return activeModules;
        */
    }
    
    pub fn find_or_create_module(file: &File) -> *mut LADSPAModuleHandle {
        
        todo!();
        /*
            for (auto i = getActiveModules().size(); --i >= 0;)
            {
                auto* module = getActiveModules().getUnchecked(i);

                if (module->file == file)
                    return module;
            }

            ++insideLADSPACallback;
            shellLADSPAUIDToCreate = 0;

            ALOE_LADSPA_LOG ("Loading LADSPA module: " + file.getFullPathName());

            std::unique_ptr<LADSPAModuleHandle> m (new LADSPAModuleHandle (file));

            if (! m->open())
                m = nullptr;

            --insideLADSPACallback;

            return m.release();
        */
    }
    
    pub fn open(&mut self) -> bool {
        
        todo!();
        /*
            module.open (file.getFullPathName());
            moduleMain = (LADSPA_Descriptor_Function) module.getFunction ("ladspa_descriptor");

            return (moduleMain != nullptr);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            module.close();
        */
    }
}

///--------------------
#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
#[no_copy]
#[leak_detector]
pub struct LADSPAPluginInstance {
    base:        AudioPluginInstance,
    module:      LADSPAModuleHandle::Ptr,
    plugin:      *const LADSPA_Descriptor, // default = nullptr
    handle:      LADSPA_Handle, // default = nullptr
    name:        String,
    lock:        CriticalSection,
    initialised: bool, // default = false
    temp_buffer: AudioBuffer<f32>, //{ 1, 1 };
    inputs:      Vec<i32>,
    outputs:     Vec<i32>,
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
pub mod ladspa_plugin_instance {
    use super::*;

    pub struct LADSPAParameter {
        base:            Parameter,
        plugin_instance: &mut LADSPAPluginInstance,
        paramid:         i32,
        name:            String,
        automatable:     bool,
        param_value:     ParameterValue,
        default_value:   f32, // default = 0.0f
    }

    pub mod ladspa_parameter {
        use super::*;

        #[derive(Default)]
        pub struct ParameterValue
        {
            scaled:   f32, // default = 0
            unscaled: f32, // default = 0
        }

        impl ParameterValue {
            
            pub fn new(s: f32, u: f32) -> Self {
            
                todo!();
                /*
                : scaled(s),
                : unscaled(u),

                
                */
            }
        }
    }

    impl LADSPAParameter {

        pub fn new(
            parent:                   &mut LADSPAPluginInstance,
            parameterid:              i32,
            parameter_name:           &String,
            parameter_is_automatable: bool) -> Self {
        
            todo!();
            /*


                : pluginInstance (parent),
                      paramID (parameterID),
                      name (parameterName),
                      automatable (parameterIsAutomatable)
                    reset();
            */
        }
        
        pub fn get_value(&self) -> f32 {
            
            todo!();
            /*
                if (pluginInstance.plugin != nullptr)
                    {
                        const ScopedLock sl (pluginInstance.lock);

                        return paramValue.unscaled;
                    }

                    return 0.0f;
            */
        }
        
        pub fn get_current_value_as_text(&self) -> String {
            
            todo!();
            /*
                if (auto* interface = pluginInstance.plugin)
                    {
                        const auto& hint = interface->PortRangeHints[paramID];

                        if (LADSPA_IS_HINT_INTEGER (hint.HintDescriptor))
                            return String ((int) paramValue.scaled);

                        return String (paramValue.scaled, 4);
                    }

                    return {};
            */
        }
        
        pub fn set_value(&mut self, new_value: f32)  {
            
            todo!();
            /*
                if (auto* interface = pluginInstance.plugin)
                    {
                        const ScopedLock sl (pluginInstance.lock);

                        if (paramValue.unscaled != newValue)
                            paramValue = ParameterValue (getNewParamScaled (interface->PortRangeHints [paramID], newValue), newValue);
                    }
            */
        }
        
        pub fn get_default_value(&self) -> f32 {
            
            todo!();
            /*
                return defaultValue;
            */
        }
        
        pub fn get_default_param_value(&self) -> ParameterValue {
            
            todo!();
            /*
                if (auto* interface = pluginInstance.plugin)
                    {
                        const auto& hint = interface->PortRangeHints[paramID];
                        const auto& desc = hint.HintDescriptor;

                        if (LADSPA_IS_HINT_HAS_DEFAULT (desc))
                        {
                            if (LADSPA_IS_HINT_DEFAULT_0 (desc))    return {};
                            if (LADSPA_IS_HINT_DEFAULT_1 (desc))    return { 1.0f, 1.0f };
                            if (LADSPA_IS_HINT_DEFAULT_100 (desc))  return { 100.0f, 0.5f };
                            if (LADSPA_IS_HINT_DEFAULT_440 (desc))  return { 440.0f, 0.5f };

                            const auto scale = LADSPA_IS_HINT_SAMPLE_RATE (desc) ? (float) pluginInstance.getSampleRate()
                            : 1.0f;
                            const auto lower = hint.LowerBound * scale;
                            const auto upper = hint.UpperBound * scale;

                            if (LADSPA_IS_HINT_BOUNDED_BELOW (desc) && LADSPA_IS_HINT_DEFAULT_MINIMUM (desc))   return { lower, 0.0f };
                            if (LADSPA_IS_HINT_BOUNDED_ABOVE (desc) && LADSPA_IS_HINT_DEFAULT_MAXIMUM (desc))   return { upper, 1.0f };

                            if (LADSPA_IS_HINT_BOUNDED_BELOW (desc))
                            {
                                auto useLog = LADSPA_IS_HINT_LOGARITHMIC (desc);

                                if (LADSPA_IS_HINT_DEFAULT_LOW    (desc))  return { scaledValue (lower, upper, 0.25f, useLog), 0.25f };
                                if (LADSPA_IS_HINT_DEFAULT_MIDDLE (desc))  return { scaledValue (lower, upper, 0.50f, useLog), 0.50f };
                                if (LADSPA_IS_HINT_DEFAULT_HIGH   (desc))  return { scaledValue (lower, upper, 0.75f, useLog), 0.75f };
                            }
                        }
                    }

                    return {};
            */
        }
        
        pub fn reset(&mut self)  {
            
            todo!();
            /*
                paramValue = getDefaultParamValue();
                    defaultValue = paramValue.unscaled;
            */
        }
        
        pub fn get_name(&self, maximum_string_length: i32) -> String {
            
            todo!();
            /*
                return name;
            */
        }
        
        pub fn get_label(&self) -> String {
            
            todo!();
            /*
                return {};
            */
        }
        
        pub fn is_automatable(&self) -> bool {
            
            todo!();
            /*
                return automatable;
            */
        }
        
        pub fn scaled_value(
            low:     f32,
            high:    f32,
            alpha:   f32,
            use_log: bool) -> f32 {
            
            todo!();
            /*
                if (useLog && low > 0 && high > 0)
                        return expf (logf (low) * (1.0f - alpha) + logf (high) * alpha);

                    return low + (high - low) * alpha;
            */
        }
        
        pub fn to_int_if_necessary(
            desc:  &LADSPA_PortRangeHintDescriptor,
            value: f32) -> f32 {
            
            todo!();
            /*
                return LADSPA_IS_HINT_INTEGER (desc) ? ((float) (int) value) : value;
            */
        }
        
        pub fn get_new_param_scaled(&self, 
            hint:      &LADSPA_PortRangeHint,
            new_value: f32) -> f32 {
            
            todo!();
            /*
                const auto& desc = hint.HintDescriptor;

                    if (LADSPA_IS_HINT_TOGGLED (desc))
                        return (newValue < 0.5f) ? 0.0f : 1.0f;

                    const auto scale = LADSPA_IS_HINT_SAMPLE_RATE (desc) ? (float) pluginInstance.getSampleRate()
                    : 1.0f;
                    const auto lower = hint.LowerBound * scale;
                    const auto upper = hint.UpperBound * scale;

                    if (LADSPA_IS_HINT_BOUNDED_BELOW (desc) && LADSPA_IS_HINT_BOUNDED_ABOVE (desc))
                        return toIntIfNecessary (desc, scaledValue (lower, upper, newValue, LADSPA_IS_HINT_LOGARITHMIC (desc)));

                    if (LADSPA_IS_HINT_BOUNDED_BELOW (desc))   return toIntIfNecessary (desc, newValue);
                    if (LADSPA_IS_HINT_BOUNDED_ABOVE (desc))   return toIntIfNecessary (desc, newValue * upper);

                    return 0.0f;
            */
        }
    }
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl Drop for LADSPAPluginInstance {
    fn drop(&mut self) {
        todo!();
        /*
            const ScopedLock sl (lock);

            jassert (insideLADSPACallback == 0);

            if (handle != nullptr && plugin != nullptr && plugin->cleanup != nullptr)
                plugin->cleanup (handle);

            initialised = false;
            module = nullptr;
            plugin = nullptr;
            handle = nullptr;
        */
    }
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl LADSPAPluginInstance {

    pub fn new(m: &LADSPAModuleHandle::Ptr) -> Self {
    
        todo!();
        /*
        : module(m),

            ++insideLADSPACallback;

            name = module->file.getFileNameWithoutExtension();

            ALOE_LADSPA_LOG ("Creating LADSPA instance: " + name);

            if (module->moduleMain != nullptr)
            {
                plugin = module->moduleMain ((size_t) shellLADSPAUIDToCreate);

                if (plugin == nullptr)
                {
                    ALOE_LADSPA_LOG ("Cannot find any valid descriptor in shared library");
                    --insideLADSPACallback;
                    return;
                }
            }
            else
            {
                ALOE_LADSPA_LOG ("Cannot find any valid plugin in shared library");
                --insideLADSPACallback;
                return;
            }

            const auto sampleRate = getSampleRate() > 0 ? getSampleRate()
                                                        : 44100.0;

            handle = plugin->instantiate (plugin, (uint32) sampleRate);

            --insideLADSPACallback;
        */
    }
    
    pub fn initialise(&mut self, 
        initial_sample_rate: f64,
        initial_block_size:  i32)  {
        
        todo!();
        /*
            setPlayConfigDetails (inputs.size(), outputs.size(), initialSampleRate, initialBlockSize);

            if (initialised || plugin == nullptr || handle == nullptr)
                return;

            ALOE_LADSPA_LOG ("Initialising LADSPA: " + name);

            initialised = true;

            inputs.clear();
            outputs.clear();
            AudioProcessorParameterGroup newTree;

            for (unsigned int i = 0; i < plugin->PortCount; ++i)
            {
                const auto portDesc = plugin->PortDescriptors[i];

                if ((portDesc & LADSPA_PORT_CONTROL) != 0)
                    newTree.addChild (std::make_unique<LADSPAParameter> (*this, (int) i,
                                                                         String (plugin->PortNames[i]).trim(),
                                                                         (portDesc & LADSPA_PORT_INPUT) != 0));

                if ((portDesc & LADSPA_PORT_AUDIO) != 0)
                {
                    if ((portDesc & LADSPA_PORT_INPUT) != 0)    inputs.add ((int) i);
                    if ((portDesc & LADSPA_PORT_OUTPUT) != 0)   outputs.add ((int) i);
                }
            }

            setParameterTree (std::move (newTree));

            for (auto* param : getParameters())
                if (auto* ladspaParam = dynamic_cast<LADSPAParameter*> (param))
                    plugin->connect_port (handle, (size_t) ladspaParam->paramID, &(ladspaParam->paramValue.scaled));

            setPlayConfigDetails (inputs.size(), outputs.size(), initialSampleRate, initialBlockSize);

            setCurrentProgram (0);
            setLatencySamples (0);

            // Some plugins crash if this doesn't happen:
            if (plugin->activate   != nullptr)   plugin->activate (handle);
            if (plugin->deactivate != nullptr)   plugin->deactivate (handle);
        */
    }

    /* --------- AudioPluginInstance methods:  --------- */
    
    pub fn fill_in_plugin_description(&self, desc: &mut PluginDescription)  {
        
        todo!();
        /*
            desc.name = getName();
            desc.fileOrIdentifier = module->file.getFullPathName();
            desc.uniqueId = desc.deprecatedUid = getUID();
            desc.lastFileModTime = module->file.getLastModificationTime();
            desc.lastInfoUpdateTime = Time::getCurrentTime();
            desc.pluginFormatName = "LADSPA";
            desc.category = getCategory();
            desc.manufacturerName = plugin != nullptr ? String (plugin->Maker) : String();
            desc.version = getVersion();
            desc.numInputChannels  = getTotalNumInputChannels();
            desc.numOutputChannels = getTotalNumOutputChannels();
            desc.isInstrument = false;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            if (plugin != nullptr && plugin->Label != nullptr)
                return plugin->Label;

            return name;
        */
    }
    
    pub fn getuid(&self) -> i32 {
        
        todo!();
        /*
            if (plugin != nullptr && plugin->UniqueID != 0)
                return (int) plugin->UniqueID;

            return module->file.hashCode();
        */
    }
    
    pub fn get_version(&self) -> String {
        
        todo!();
        /*
            return LADSPA_VERSION;
        */
    }
    
    pub fn get_category(&self) -> String {
        
        todo!();
        /*
            return "Effect";
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        new_sample_rate:            f64,
        samples_per_block_expected: i32)  {
        
        todo!();
        /*
            setLatencySamples (0);

            initialise (newSampleRate, samplesPerBlockExpected);

            if (initialised)
            {
                tempBuffer.setSize (jmax (1, outputs.size()), samplesPerBlockExpected);

                // dodgy hack to force some plugins to initialise the sample rate..
                if (auto* firstParam = getParameters()[0])
                {
                    const auto old = firstParam->getValue();
                    firstParam->setValue ((old < 0.5f) ? 1.0f : 0.0f);
                    firstParam->setValue (old);
                }

                if (plugin->activate != nullptr)
                    plugin->activate (handle);
            }
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            if (handle != nullptr && plugin->deactivate != nullptr)
                plugin->deactivate (handle);

            tempBuffer.setSize (1, 1);
        */
    }
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        _1:     &mut MidiBuffer)  {
        
        todo!();
        /*
            auto numSamples = buffer.getNumSamples();

            if (initialised && plugin != nullptr && handle != nullptr)
            {
                for (int i = 0; i < inputs.size(); ++i)
                    plugin->connect_port (handle, (size_t) inputs[i],
                                          i < buffer.getNumChannels() ? buffer.getWritePointer (i) : nullptr);

                if (plugin->run != nullptr)
                {
                    for (int i = 0; i < outputs.size(); ++i)
                        plugin->connect_port (handle, (size_t) outputs.getUnchecked(i),
                                              i < buffer.getNumChannels() ? buffer.getWritePointer (i) : nullptr);

                    plugin->run (handle, (size_t) numSamples);
                    return;
                }

                if (plugin->run_adding != nullptr)
                {
                    tempBuffer.setSize (outputs.size(), numSamples);
                    tempBuffer.clear();

                    for (int i = 0; i < outputs.size(); ++i)
                        plugin->connect_port (handle, (size_t) outputs.getUnchecked(i), tempBuffer.getWritePointer (i));

                    plugin->run_adding (handle, (size_t) numSamples);

                    for (int i = 0; i < outputs.size(); ++i)
                        if (i < buffer.getNumChannels())
                            buffer.copyFrom (i, 0, tempBuffer, i, 0, numSamples);

                    return;
                }

                jassertfalse; // no callback to use?
            }

            for (auto i = getTotalNumInputChannels(), e = getTotalNumOutputChannels(); i < e; ++i)
                buffer.clear (i, 0, numSamples);
        */
    }
    
    pub fn is_input_channel_stereo_pair(&self, index: i32) -> bool {
        
        todo!();
        /*
            return isPositiveAndBelow (index, getTotalNumInputChannels());
        */
    }
    
    pub fn is_output_channel_stereo_pair(&self, index: i32) -> bool {
        
        todo!();
        /*
            return isPositiveAndBelow (index, getTotalNumOutputChannels());
        */
    }
    
    pub fn get_input_channel_name(&self, index: i32) -> String {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, getTotalNumInputChannels()))
                return String (plugin->PortNames [inputs [index]]).trim();

            return {};
        */
    }
    
    pub fn get_output_channel_name(&self, index: i32) -> String {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, getTotalNumInputChannels()))
                return String (plugin->PortNames [outputs [index]]).trim();

            return {};
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn set_current_program(&mut self, _0: i32)  {
        
        todo!();
        /*
            for (auto* param : getParameters())
                if (auto* ladspaParam = dynamic_cast<LADSPAParameter*> (param))
                    ladspaParam->reset();
        */
    }
    
    pub fn get_program_name(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            auto numParameters = getParameters().size();
            destData.setSize ((size_t) numParameters * sizeof (float));
            destData.fillWith (0);

            auto* p = unalignedPointerCast<float*> (destData.getData());

            for (int i = 0; i < numParameters; ++i)
                if (auto* param = getParameters()[i])
                    p[i] = param->getValue();
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
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            ignoreUnused (sizeInBytes);

            auto* p = static_cast<const float*> (data);

            for (int i = 0; i < getParameters().size(); ++i)
                if (auto* param = getParameters()[i])
                    param->setValue (p[i]);
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return handle != nullptr;
        */
    }
}

#[cfg(all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd")))]
impl LADSPAPluginFormat {

    pub fn find_all_types_for_file(&mut self, 
        results:            &mut Vec<Box<PluginDescription>>,
        file_or_identifier: &String)  {
        
        todo!();
        /*
            if (! fileMightContainThisPluginType (fileOrIdentifier))
            return;

        PluginDescription desc;
        desc.fileOrIdentifier = fileOrIdentifier;
        desc.uniqueId = desc.deprecatedUid = 0;

        auto createdInstance = createInstanceFromDescription (desc, 44100.0, 512);
        auto instance = dynamic_cast<LADSPAPluginInstance*> (createdInstance.get());

        if (instance == nullptr || ! instance->isValid())
            return;

        instance->initialise (44100.0, 512);
        instance->fillInPluginDescription (desc);

        if (instance->module->moduleMain != nullptr)
        {
            for (int uid = 0;; ++uid)
            {
                if (auto* plugin = instance->module->moduleMain ((size_t) uid))
                {
                    desc.uniqueId = desc.deprecatedUid = uid;
                    desc.name = plugin->Name != nullptr ? plugin->Name : "Unknown";

                    if (! arrayContainsPlugin (results, desc))
                        results.add (new PluginDescription (desc));
                }
                else
                {
                    break;
                }
            }
        }
        */
    }
    
    pub fn create_plugin_instance(&mut self, 
        desc:        &PluginDescription,
        sample_rate: f64,
        block_size:  i32,
        callback:    PluginCreationCallback)  {
        
        todo!();
        /*
            std::unique_ptr<LADSPAPluginInstance> result;

        if (fileMightContainThisPluginType (desc.fileOrIdentifier))
        {
            auto file = File (desc.fileOrIdentifier);

            auto previousWorkingDirectory = File::getCurrentWorkingDirectory();
            file.getParentDirectory().setAsCurrentWorkingDirectory();

            const LADSPAModuleHandle::Ptr module (LADSPAModuleHandle::findOrCreateModule (file));

            if (module != nullptr)
            {
                shellLADSPAUIDToCreate = desc.uniqueId != 0 ? desc.uniqueId : desc.deprecatedUid;

                result.reset (new LADSPAPluginInstance (module));

                if (result->plugin != nullptr && result->isValid())
                    result->initialise (sampleRate, blockSize);
                else
                    result = nullptr;
            }

            previousWorkingDirectory.setAsCurrentWorkingDirectory();
        }

        String errorMsg;

        if (result == nullptr)
            errorMsg = TRANS ("Unable to load XXX plug-in file").replace ("XXX", "LADSPA");

        callback (std::move (result), errorMsg);
        */
    }
    
    pub fn requires_unblocked_message_thread_during_creation(&self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn file_might_contain_this_plugin_type(&mut self, file_or_identifier: &String) -> bool {
        
        todo!();
        /*
            auto f = File::createFileWithoutCheckingPath (fileOrIdentifier);
        return f.existsAsFile() && f.hasFileExtension (".so");
        */
    }
    
    pub fn get_name_of_plugin_from_identifier(&mut self, file_or_identifier: &String) -> String {
        
        todo!();
        /*
            return fileOrIdentifier;
        */
    }
    
    pub fn plugin_needs_rescanning(&mut self, desc: &PluginDescription) -> bool {
        
        todo!();
        /*
            return File (desc.fileOrIdentifier).getLastModificationTime() != desc.lastFileModTime;
        */
    }
    
    pub fn does_plugin_still_exist(&mut self, desc: &PluginDescription) -> bool {
        
        todo!();
        /*
            return File::createFileWithoutCheckingPath (desc.fileOrIdentifier).exists();
        */
    }
    
    pub fn search_paths_for_plugins(&mut self, 
        directories_to_search: &FileSearchPath,
        recursive:             bool,
        _2:                    bool) -> StringArray {
        
        todo!();
        /*
            StringArray results;

        for (int j = 0; j < directoriesToSearch.getNumPaths(); ++j)
            recursiveFileSearch (results, directoriesToSearch[j], recursive);

        return results;
        */
    }
    
    pub fn recursive_file_search(&mut self, 
        results:   &mut StringArray,
        dir:       &File,
        recursive: bool)  {
        
        todo!();
        /*
            for (const auto& iter : RangedDirectoryIterator (dir, false, "*", File::findFilesAndDirectories))
        {
            auto f = iter.getFile();
            bool isPlugin = false;

            if (fileMightContainThisPluginType (f.getFullPathName()))
            {
                isPlugin = true;
                results.add (f.getFullPathName());
            }

            if (recursive && (! isPlugin) && f.isDirectory())
                recursiveFileSearch (results, f, true);
        }
        */
    }
    
    pub fn get_default_locations_to_search(&mut self) -> FileSearchPath {
        
        todo!();
        /*
            return  { SystemStats::getEnvironmentVariable ("LADSPA_PATH", "/usr/lib/ladspa;/usr/local/lib/ladspa;~/.ladspa").replace (":", ";") };
        */
    }
}
