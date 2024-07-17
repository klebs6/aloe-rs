crate::ix!();

pub enum AloeAudioProcessorInternalParameters
{
    paramPreset               = 0x70727374, // 'prst'
    paramMidiControllerOffset = 0x6d636d00, // 'mdm*'
    paramBypass               = 0x62797073  // 'byps'
}

#[no_copy]
#[leak_detector]
pub struct AloeAudioProcessor {
    base:                        VstIUnitInfo,

    vst_param_ids:               Vec<VstParamID>,
    cached_param_values:         CachedParamValues,

    bypass_paramid:              VstParamID, // default = 0

    /**
       = static_cast<VstParamID> (paramPreset);
      */
    program_paramid:             VstParamID,

    bypass_is_regular_parameter: bool,        // default = false
    ref_count:                   Atomic<i32>, // default = { 0  }
    audio_processor:             Box<dyn AudioProcessorInterface>,
    aloe_parameters:             LegacyAudioParametersWrapper,
    param_map:                   HashMap<i32,*mut AudioProcessorParameter>,
    owned_bypass_parameter:      Box<AudioProcessorParameter>,
    owned_program_parameter:     Box<AudioProcessorParameter>,
    parameter_groups:            Vec<*const AudioProcessorParameterGroup>,
}

lazy_static!{
    /*
    static const FUID aloe_audio_processor_iid;
    */
}

impl AloeAudioProcessor {

    /*
    ALOE_DECLARE_Vst3_COM_QUERY_METHODS
    ALOE_DECLARE_Vst3_COM_REF_METHODS
    */

    pub fn new(source: *mut dyn AudioProcessorInterface) -> Self {
    
        todo!();
        /*
        : audio_processor(source),

            setupParameters();
        */
    }
    
    pub fn get(&self) -> *mut dyn AudioProcessorInterface {
        
        todo!();
        /*
            return audioProcessor.get();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_count(&mut self) -> i32 {
        
        todo!();
        /*
            return parameterGroups.size() + 1;
        */
    }

    #[PLUGIN_API]
    pub fn get_unit_info(&mut self, 
        unit_index: i32,
        info:       &mut VstUnitInfo) -> tresult {
        
        todo!();
        /*
            if (unitIndex == 0)
            {
                info.id             = VstkRootUnitId;
                info.parentUnitId   = VstkNoParentUnitId;
                info.programListId  = VstkNoProgramListId;

                toString128 (info.name, TRANS("Root Unit"));

                return kResultTrue;
            }

            if (auto* group = parameterGroups[unitIndex - 1])
            {
                info.id             = AloeAudioProcessor::getUnitID (group);
                info.parentUnitId   = AloeAudioProcessor::getUnitID (group->getParent());
                info.programListId  = VstkNoProgramListId;

                toString128 (info.name, group->getName());

                return kResultTrue;
            }

            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_program_list_count(&mut self) -> i32 {
        
        todo!();
        /*
            if (audioProcessor->getNumPrograms() > 0)
                return 1;

            return 0;
        */
    }

    #[PLUGIN_API]
    pub fn get_program_list_info(&mut self, 
        list_index: i32,
        info:       &mut VstProgramListInfo) -> tresult {
        
        todo!();
        /*
            if (listIndex == 0)
            {
                info.id = static_cast<VstProgramListID> (programParamID);
                info.programCount = static_cast<i32> (audioProcessor->getNumPrograms());

                toString128 (info.name, TRANS("Factory Presets"));

                return kResultTrue;
            }

            jassertfalse;
            zerostruct (info);
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_name(&mut self, 
        list_id:       VstProgramListID,
        program_index: i32,
        name:          VstString128) -> tresult {
        
        todo!();
        /*
            if (listId == static_cast<VstProgramListID> (programParamID)
                && isPositiveAndBelow ((int) programIndex, audioProcessor->getNumPrograms()))
            {
                toString128 (name, audioProcessor->getProgramName ((int) programIndex));
                return kResultTrue;
            }

            jassertfalse;
            toString128 (name, String());
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_info(&mut self, 
        _0: VstProgramListID,
        _1: i32,
        _2: VstCString,
        _3: VstString128) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn has_program_pitch_names(&mut self, 
        _0: VstProgramListID,
        _1: i32) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_pitch_name(&mut self, 
        _0: VstProgramListID,
        _1: i32,
        _2: i16,
        _3: VstString128) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn select_unit(&mut self, _0: VstUnitID) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_unit_program_data(
        &mut self, 
        _0: i32,
        _1: i32,
        _2: *mut dyn IBStream

    ) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_selected_unit(&mut self) -> VstUnitID {
        
        todo!();
        /*
            return VstkRootUnitId;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_by_bus(&mut self, 
        _0:      VstMediaType,
        _1:      VstBusDirection,
        _2:      i32,
        _3:      i32,
        unit_id: &mut VstUnitID) -> tresult {
        
        todo!();
        /*
            zerostruct (unitId);
            return kNotImplemented;
        */
    }
    
    #[inline] pub fn get_vst_param_id_for_index(&self, param_index: i32) -> VstParamID {
        
        todo!();
        /*
            #if ALOE_FORCE_USE_LEGACY_PARAM_IDS
            return static_cast<VstParamID> (paramIndex);
           #else
            return vstParamIDs.getReference (paramIndex);
           #endif
        */
    }
    
    pub fn get_param_for_vst_paramid(&self, paramid: VstParamID) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            return paramMap[static_cast<i32> (paramID)];
        */
    }
    
    pub fn get_bypass_parameter(&self) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            return getParamForVstParamID (bypassParamID);
        */
    }
    
    pub fn get_program_parameter(&self) -> *mut AudioProcessorParameter {
        
        todo!();
        /*
            return getParamForVstParamID (programParamID);
        */
    }
    
    pub fn get_unitid(group: *const AudioProcessorParameterGroup) -> VstUnitID {
        
        todo!();
        /*
            if (group == nullptr || group->getParent() == nullptr)
                return VstkRootUnitId;

            // From the Vst3 docs (also applicable to unit IDs!):
            // Up to 2^31 parameters can be exported with id range [0, 2147483648]
            // (the range [2147483649, 429496729] is reserved for host application).
            auto unitID = group->getID().hashCode() & 0x7fffffff;

            // If you hit this assertion then your group ID is hashing to a value
            // reserved by the Vst3 SDK. Please use a different group ID.
            jassert (unitID != VstkRootUnitId);

            return unitID;
        */
    }
    
    pub fn get_param_ids(&self) -> &[VstParamID] {
        
        todo!();
        /*
            return vstParamIDs;
        */
    }
    
    pub fn get_bypass_paramid(&self) -> VstParamID {
        
        todo!();
        /*
            return bypassParamID;
        */
    }
    
    pub fn get_program_paramid(&self) -> VstParamID {
        
        todo!();
        /*
            return programParamID;
        */
    }
    
    pub fn is_bypass_regular_parameter(&self) -> bool {
        
        todo!();
        /*
            return bypassIsRegularParameter;
        */
    }
    
    pub fn find_cache_index_for_paramid(&self, paramid: VstParamID) -> i32 {
        
        todo!();
        /*
            return vstParamIDs.indexOf (paramID);
        */
    }
    
    pub fn set_parameter_value(&mut self, 
        param_index: i32,
        value:       f32)  {
        
        todo!();
        /*
            cachedParamValues.set (paramIndex, value);
        */
    }
    
    pub fn for_all_changed_parameters<Callback>(&mut self, callback: Callback)  {
    
        todo!();
        /*
            cachedParamValues.ifSet ([&] (i32 index, float value)
            {
                callback (cachedParamValues.getParamID (index), value);
            });
        */
    }
    
    pub fn is_using_managed_parameters(&self) -> bool {
        
        todo!();
        /*
            return aloeParameters.isUsingManagedParameters();
        */
    }
    
    pub fn setup_parameters(&mut self)  {
        
        todo!();
        /*
            parameterGroups = audioProcessor->getParameterTree().getSubgroups (true);

           #if ALOE_DEBUG
            auto allGroups = parameterGroups;
            allGroups.add (&audioProcessor->getParameterTree());
            std::unordered_set<VstUnitID> unitIDs;

            for (auto* group : allGroups)
            {
                auto insertResult = unitIDs.insert (getUnitID (group));

                // If you hit this assertion then either a group ID is not unique or
                // you are very unlucky and a hashed group ID is not unique
                jassert (insertResult.second);
            }
           #endif

           #if ALOE_FORCE_USE_LEGACY_PARAM_IDS
            const bool forceLegacyParamIDs = true;
           #else
            const bool forceLegacyParamIDs = false;
           #endif

            aloeParameters.update (*audioProcessor, forceLegacyParamIDs);
            auto numParameters = aloeParameters.getNumParameters();

            bool vst3WrapperProvidedBypassParam = false;
            auto* bypassParameter = audioProcessor->getBypassParameter();

            if (bypassParameter == nullptr)
            {
                vst3WrapperProvidedBypassParam = true;
                ownedBypassParameter.reset (new AudioParameterBool ("byps", "Bypass", false, {}, {}, {}));
                bypassParameter = ownedBypassParameter.get();
            }

            // if the bypass parameter is not part of the exported parameters that the plug-in supports
            // then add it to the end of the list as Vst3 requires the bypass parameter to be exported!
            bypassIsRegularParameter = aloeParameters.params.contains (audioProcessor->getBypassParameter());

            if (! bypassIsRegularParameter)
                aloeParameters.params.add (bypassParameter);

            int i = 0;
            for (auto* aloeParam : aloeParameters.params)
            {
                bool isBypassParameter = (aloeParam == bypassParameter);

                VstParamID vstParamID = forceLegacyParamIDs ? static_cast<VstParamID> (i++)
                                                              : generateVstParamIDForParam (aloeParam);

                if (isBypassParameter)
                {
                    // we need to remain backward compatible with the old bypass id
                    if (vst3WrapperProvidedBypassParam)
                    {
                        ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6240)
                        vstParamID = static_cast<VstParamID> ((isUsingManagedParameters() && ! forceLegacyParamIDs) ? paramBypass : numParameters);
                        ALOE_END_IGNORE_WARNINGS_MSVC
                    }

                    bypassParamID = vstParamID;
                }

                vstParamIDs.add (vstParamID);
                paramMap.set (static_cast<i32> (vstParamID), aloeParam);
            }

            auto numPrograms = audioProcessor->getNumPrograms();

            if (numPrograms > 1)
            {
                ownedProgramParameter = std::make_unique<AudioParameterInt> ("aloeProgramParameter", "Program",
                                                                             0, numPrograms - 1,
                                                                             audioProcessor->getCurrentProgram());

                aloeParameters.params.add (ownedProgramParameter.get());

                if (forceLegacyParamIDs)
                    programParamID = static_cast<VstParamID> (i++);

                vstParamIDs.add (programParamID);
                paramMap.set (static_cast<i32> (programParamID), ownedProgramParameter.get());
            }

            cachedParamValues = CachedParamValues { { vstParamIDs.begin(), vstParamIDs.end() } };
        */
    }
    
    pub fn generate_vst_param_id_for_param(&mut self, param: *mut AudioProcessorParameter) -> VstParamID {
        
        todo!();
        /*
            auto aloeParamID = LegacyAudioParameter::getParamID (param, false);

          #if ALOE_FORCE_USE_LEGACY_PARAM_IDS
            return static_cast<VstParamID> (aloeParamID.getIntValue());
          #else
            auto paramHash = static_cast<VstParamID> (aloeParamID.hashCode());

           #if ALOE_USE_STUDIO_ONE_COMPATIBLE_PARAMETERS
            // studio one doesn't like negative parameters
            paramHash &= ~(((VstParamID) 1) << (sizeof (VstParamID) * 8 - 1));
           #endif

            return paramHash;
          #endif
        */
    }
}
