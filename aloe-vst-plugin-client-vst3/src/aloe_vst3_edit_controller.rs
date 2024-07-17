crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AloeVst3EditController<'a> {
    base:                                VstEditController,
    base2:                               VstIMidiMapping,
    base3:                               VstIUnitInfo,
    base4:                               VstChannelContextIInfoListener,
 
    audio_processor:                     VstComSmartPtr<AloeAudioProcessor>,
    component_restarter:                 ComponentRestarter<'a>, //{ *this };

    parameter_to_midi_controller_offset: VstParamID,
    parameter_to_midi_controller:        [AloeVst3EditControllerMidiController; NUM_MIDI_CHANNELS * ControllerNumbers::count_ctrl_number()],
    midi_controller_to_parameter:        [[VstParamID; NUM_MIDI_CHANNELS]; ControllerNumbers::count_ctrl_number()],
    owned_parameter_listeners:           Vec<Box<AloeVst3EditControllerOwnedParameterListener<'a>>>,
    vst_3is_playing:                     AtomicBool, // default = { false  }
    in_setup_processing:                 AtomicBool, // default = { false  }
    last_latency_samples:                i32, // default = 0

    #[cfg(not(target_os="macos"))]
    last_scale_factor_received:          f32, // default = 1.0f
}

lazy_static!{
    /*
    static const FUID aloe_vst3_edit_controller_iid;
    */
}

impl<'a> ComponentRestarterListener for AloeVst3EditController<'a> {
    fn restart_component_on_message_thread(&mut self, _: i32) { todo!() }
}

impl<'a> AudioProcessorListener for AloeVst3EditController<'a> {

}

impl<'a> AudioProcessorParameterChanged for AloeVst3EditController<'a> {
    fn audio_processor_parameter_changed(&mut self, _: *mut (dyn aloe_audio_interface::AudioProcessorInterface + 'static), _: i32, _: f32) { todo!() }
}

impl<'a> AudioProcessorChanged for AloeVst3EditController<'a> {
    fn audio_processor_changed(&mut self, _: *mut (dyn aloe_audio_interface::AudioProcessorInterface + 'static), _: &AudioProcessorListenerChangeDetails) { todo!() }
}

impl<'a> AudioProcessorParameterChangeGestureEnd for AloeVst3EditController<'a> {

}

impl<'a> AudioProcessorParameterChangeGestureBegin for AloeVst3EditController<'a> {

}

impl<'a> AloeVst3EditController<'a> {

    pub fn new(host: *mut VstIHostApplication) -> Self {
    
        todo!();
        /*

            if (host != nullptr)
                host->queryInterface (FUnknown::iid, (void**) &hostContext);
        */
    }

    refcount_methods!{ ComponentBase }

    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        targetiid: TUID,
        obj:       *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            const auto userProvidedInterface = queryAdditionalInterfaces (getPluginInstance(),
                                                                          targetIID,
                                                                          &Vst3ClientExtensions::queryIEditController);

            const auto aloeProvidedInterface = queryInterfaceInternal (targetIID);

            return extractResult (userProvidedInterface, aloeProvidedInterface, obj);
        */
    }
    
    #[PLUGIN_API]
    pub fn initialize(&mut self, context: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            if (hostContext != context)
                hostContext = context;

            return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn terminate(&mut self) -> tresult {
        
        todo!();
        /*
            if (auto* pluginInstance = getPluginInstance())
                pluginInstance->removeListener (this);

            audioProcessor = nullptr;

            return EditController::terminate();
        */
    }
    
    #[PLUGIN_API]
    pub fn set_channel_context_infos(&mut self, list: *mut VstIAttributeList) -> tresult {
        
        todo!();
        /*
            if (auto* instance = getPluginInstance())
            {
                if (list != nullptr)
                {
                    AudioProcessor::TrackProperties trackProperties;

                    {
                        VstString128 channelName;
                        if (list->getString (VstChannelContext::kChannelNameKey, channelName, sizeof (channelName)) == kResultTrue)
                            trackProperties.name = toString (channelName);
                    }

                    {
                        int64 colour;
                        if (list->getInt (VstChannelContext::kChannelColorKey, colour) == kResultTrue)
                            trackProperties.colour = Colour (VstChannelContext::GetRed ((uint32) colour),  VstChannelContext::GetGreen ((uint32) colour),
                                                             VstChannelContext::GetBlue ((uint32) colour), VstChannelContext::GetAlpha ((uint32) colour));
                    }

                    if (MessageManager::getInstance()->isThisTheMessageThread())
                        instance->updateTrackProperties (trackProperties);
                    else
                        MessageManager::callAsync ([trackProperties, instance]
                                                   { instance->updateTrackProperties (trackProperties); });
                }
            }

            return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_component_state(&mut self, stream: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            if (! MessageManager::existsAndIsCurrentThread())
           #if ALOE_LINUX || ALOE_BSD
            {
                tresult result = kResultOk;
                WaitableEvent finishedEvent;

                MessageManager::callAsync ([&]
                {
                    result = setComponentState (stream);
                    finishedEvent.signal();
                });

                finishedEvent.wait();
                return result;
            }
           #else
            // As an IEditController member, the host should only call this from the message thread.
            jassertfalse;
           #endif

            if (auto* pluginInstance = getPluginInstance())
            {
                for (auto vstParamId : audioProcessor->getParamIDs())
                {
                    auto paramValue = [&]
                    {
                        if (vstParamId == audioProcessor->getProgramParamID())
                            return EditController::plainParamToNormalized (audioProcessor->getProgramParamID(),
                                                                           pluginInstance->getCurrentProgram());

                        return (double) audioProcessor->getParamForVstParamID (vstParamId)->getValue();
                    }();

                    setParamNormalized (vstParamId, paramValue);
                }
            }

            if (auto* handler = getComponentHandler())
                handler->restartComponent (VstkParamValuesChanged);

            return VstEditController::setComponentState (stream);
        */
    }
    
    pub fn set_audio_processor(&mut self, audio_proc: *mut AloeAudioProcessor)  {
        
        todo!();
        /*
            if (audioProcessor != audioProc)
                installAudioProcessor (audioProc);
        */
    }

    #[PLUGIN_API]
    pub fn connect(&mut self, other: *mut dyn IConnectionPoint) -> tresult {
        
        todo!();
        /*
            if (other != nullptr && audioProcessor == nullptr)
            {
                auto result = ComponentBase::connect (other);

                if (! audioProcessor.loadFrom (other))
                    sendIntMessage ("AloeVst3EditController", (int64) (pointer_sized_int) this);
                else
                    installAudioProcessor (audioProcessor);

                return result;
            }

            jassertfalse;
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_midi_controller_assignment(&mut self, 
        bus_index:              i32,
        channel:                i16,
        midi_controller_number: VstCtrlNumber,
        resultid:               &mut VstParamID) -> tresult {
        
        todo!();
        /*
            #if ALOE_Vst3_EMULATE_MIDI_CC_WITH_PARAMETERS
            resultID = midiControllerToParameter[channel][midiControllerNumber];
            return kResultTrue; // Returning false makes some hosts stop asking for further MIDI Controller Assignments
           #else
            ignoreUnused (channel, midiControllerNumber, resultID);
            return kResultFalse;
           #endif
        */
    }

    /**
       Converts an incoming parameter index to
       a MIDI controller:
      */
    pub fn get_midi_controller_for_parameter(&mut self, 
        index:       VstParamID,
        channel:     &mut i32,
        ctrl_number: &mut i32) -> bool {
        
        todo!();
        /*
            auto mappedIndex = static_cast<int> (index - parameterToMidiControllerOffset);

            if (isPositiveAndBelow (mappedIndex, numElementsInArray (parameterToMidiController)))
            {
                auto& mc = parameterToMidiController[mappedIndex];

                if (mc.channel != -1 && mc.ctrlNumber != -1)
                {
                    channel = jlimit (1, 16, mc.channel + 1);
                    ctrlNumber = mc.ctrlNumber;
                    return true;
                }
            }

            return false;
        */
    }
    
    #[inline] pub fn is_midi_controller_paramid(&self, paramid: VstParamID) -> bool {
        
        todo!();
        /*
            return (paramID >= parameterToMidiControllerOffset
                        && isPositiveAndBelow (paramID - parameterToMidiControllerOffset,
                                               static_cast<VstParamID> (numElementsInArray (parameterToMidiController))));
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_count(&mut self) -> i32 {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getUnitCount();

            jassertfalse;
            return 1;
        */
    }

    #[PLUGIN_API]
    pub fn get_unit_info(&mut self, 
        unit_index: i32,
        info:       &mut VstUnitInfo) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getUnitInfo (unitIndex, info);

            if (unitIndex == 0)
            {
                info.id             = VstkRootUnitId;
                info.parentUnitId   = VstkNoParentUnitId;
                info.programListId  = VstkNoProgramListId;

                toString128 (info.name, TRANS("Root Unit"));

                return kResultTrue;
            }

            jassertfalse;
            zerostruct (info);
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_program_list_count(&mut self) -> i32 {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getProgramListCount();

            jassertfalse;
            return 0;
        */
    }

    #[PLUGIN_API]
    pub fn get_program_list_info(&mut self, 
        list_index: i32,
        info:       &mut VstProgramListInfo) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getProgramListInfo (listIndex, info);

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
            if (audioProcessor != nullptr)
                return audioProcessor->getProgramName (listId, programIndex, name);

            jassertfalse;
            toString128 (name, String());
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_program_info(&mut self, 
        list_id:         VstProgramListID,
        program_index:   i32,
        attribute_id:    &str,
        attribute_value: VstString128) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getProgramInfo (listId, programIndex, attributeId, attributeValue);

            jassertfalse;
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn has_program_pitch_names(&mut self, 
        list_id:       VstProgramListID,
        program_index: i32) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->hasProgramPitchNames (listId, programIndex);

            jassertfalse;
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_program_pitch_name(&mut self, 
        list_id:       VstProgramListID,
        program_index: i32,
        midi_pitch:    i16,
        name:          VstString128) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getProgramPitchName (listId, programIndex, midiPitch, name);

            jassertfalse;
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn select_unit(&mut self, unit_id: VstUnitID) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->selectUnit (unitId);

            jassertfalse;
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn set_unit_program_data(&mut self, 
        list_or_unit_id: i32,
        program_index:   i32,
        data:            *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->setUnitProgramData (listOrUnitId, programIndex, data);

            jassertfalse;
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_selected_unit(&mut self) -> VstUnitID {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getSelectedUnit();

            jassertfalse;
            return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_unit_by_bus(&mut self, 
        ty:        VstMediaType,
        dir:       VstBusDirection,
        bus_index: i32,
        channel:   i32,
        unit_id:   &mut VstUnitID) -> tresult {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->getUnitByBus (type, dir, busIndex, channel, unitId);

            jassertfalse;
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn create_view(&mut self, name: *const u8) -> *mut dyn IPlugView {
        
        todo!();
        /*
            if (auto* pluginInstance = getPluginInstance())
            {
                const auto mayCreateEditor = pluginInstance->hasEditor()
                                          && name != nullptr
                                          && std::strcmp (name, VstViewType::kEditor) == 0
                                          && (pluginInstance->getActiveEditor() == nullptr
                                              || getHostType().isAdobeAudition()
                                              || getHostType().isPremiere());

                if (mayCreateEditor)
                    return new AloeVst3Editor (*this, *audioProcessor);
            }

            return nullptr;
        */
    }
    
    pub fn begin_gesture(&mut self, vst_param_id: VstParamID)  {
        
        todo!();
        /*
            if (MessageManager::getInstance()->isThisTheMessageThread())
                beginEdit (vstParamId);
        */
    }
    
    pub fn end_gesture(&mut self, vst_param_id: VstParamID)  {
        
        todo!();
        /*
            if (MessageManager::getInstance()->isThisTheMessageThread())
                endEdit (vstParamId);
        */
    }
    
    pub fn param_changed(&mut self, 
        parameter_index: i32,
        vst_param_id:    VstParamID,
        new_value:       f64)  {
        
        todo!();
        /*
            if (inParameterChangedCallback)
                return;

            if (MessageManager::getInstance()->isThisTheMessageThread())
            {
                // NB: Cubase has problems if performEdit is called without setParamNormalized
                EditController::setParamNormalized (vstParamId, newValue);
                performEdit (vstParamId, newValue);
            }
            else
            {
                audioProcessor->setParameterValue (parameterIndex, (float) newValue);
            }
        */
    }
    
    pub fn audio_processor_parameter_change_gesture_begin(
        &mut self, 
        _0:    *mut dyn AudioProcessorInterface,
        index: i32

    ) {
        
        todo!();
        /*
            beginGesture (audioProcessor->getVstParamIDForIndex (index));
        */
    }
    
    pub fn audio_processor_parameter_change_gesture_end(&mut self, 
        _0:    *mut dyn AudioProcessorInterface,
        index: i32)  {
        
        todo!();
        /*
            endGesture (audioProcessor->getVstParamIDForIndex (index));
        */
    }
    
    pub fn audio_processor_parameter_changed(&mut self, 
        _0:        *mut dyn AudioProcessorInterface,
        index:     i32,
        new_value: f32)  {
        
        todo!();
        /*
            paramChanged (index, audioProcessor->getVstParamIDForIndex (index), newValue);
        */
    }
    
    pub fn audio_processor_changed(&mut self, 
        _0:      *mut dyn AudioProcessorInterface,
        details: &AudioProcessorChangeDetails)  {
        
        todo!();
        /*
            i32 flags = 0;

            if (details.parameterInfoChanged)
            {
                for (i32 i = 0; i < parameters.getParameterCount(); ++i)
                    if (auto* param = dynamic_cast<AloeVst3EditControllerParam*> (parameters.getParameterByIndex (i)))
                        if (param->updateParameterInfo() && (flags & VstkParamTitlesChanged) == 0)
                            flags |= VstkParamTitlesChanged;
            }

            if (auto* pluginInstance = getPluginInstance())
            {
                if (details.programChanged)
                {
                    const auto programParameterId = audioProcessor->getProgramParamID();

                    if (audioProcessor->getParamForVstParamID (programParameterId) != nullptr)
                    {
                        const auto currentProgram = pluginInstance->getCurrentProgram();
                        const auto paramValue = roundToInt (EditController::normalizedParamToPlain (programParameterId,
                                                                                                    EditController::getParamNormalized (programParameterId)));

                        if (currentProgram != paramValue)
                        {
                            beginGesture (programParameterId);
                            paramChanged (audioProcessor->findCacheIndexForParamID (programParameterId),
                                          programParameterId,
                                          EditController::plainParamToNormalized (programParameterId, currentProgram));
                            endGesture (programParameterId);

                            flags |= VstkParamValuesChanged;
                        }
                    }
                }

                auto latencySamples = pluginInstance->getLatencySamples();

                if (details.latencyChanged && latencySamples != lastLatencySamples)
                {
                    flags |= VstkLatencyChanged;
                    lastLatencySamples = latencySamples;
                }
            }

            if (! inSetupProcessing)
                componentRestarter.restart (flags);
        */
    }
    
    pub fn get_plugin_instance(&self) -> *mut dyn AudioProcessorInterface {
        
        todo!();
        /*
            if (audioProcessor != nullptr)
                return audioProcessor->get();

            return nullptr;
        */
    }
    
    pub fn restart_component_on_message_thread(&mut self, flags: i32)  {
        
        todo!();
        /*
            if (auto* handler = componentHandler)
                handler->restartComponent (flags);
        */
    }
    
    pub fn query_interface_internal(&mut self, targetiid: TUID) -> InterfaceResultWithDeferredAddRef {
        
        todo!();
        /*
            const auto result = testForMultiple (*this,
                                                 targetIID,
                                                 UniqueBase<FObject>{},
                                                 UniqueBase<AloeVst3EditController>{},
                                                 UniqueBase<VstIEditController>{},
                                                 UniqueBase<VstIEditController2>{},
                                                 UniqueBase<VstIConnectionPoint>{},
                                                 UniqueBase<VstIMidiMapping>{},
                                                 UniqueBase<VstIUnitInfo>{},
                                                 UniqueBase<VstChannelContext::IInfoListener>{},
                                                 SharedBase<IPluginBase, VstIEditController>{},
                                                 UniqueBase<IDependent>{},
                                                 SharedBase<FUnknown, VstIEditController>{});

            if (result.isOk())
                return result;

            if (doUIDsMatch (targetIID, AloeAudioProcessor::iid))
                return { kResultOk, audioProcessor.get() };

            return {};
        */
    }
    
    pub fn install_audio_processor(&mut self, new_audio_processor: &VstComSmartPtr<AloeAudioProcessor>)  {
        
        todo!();
        /*
            audioProcessor = newAudioProcessor;

            if (auto* extensions = dynamic_cast<Vst3ClientExtensions*> (audioProcessor->get()))
            {
                extensions->setIComponentHandler (componentHandler);
                extensions->setIHostApplication (hostContext.get());
            }

            if (auto* pluginInstance = getPluginInstance())
            {
                lastLatencySamples = pluginInstance->getLatencySamples();

                pluginInstance->addListener (this);

                // as the bypass is not part of the regular parameters we need to listen for it explicitly
                if (! audioProcessor->isBypassRegularParameter())
                {
                    const auto paramID = audioProcessor->getBypassParamID();
                    ownedParameterListeners.push_back (std::make_unique<AloeVst3EditControllerOwnedParameterListener> (*this,
                                                                                                 *audioProcessor->getParamForVstParamID (paramID),
                                                                                                 paramID,
                                                                                                 audioProcessor->findCacheIndexForParamID (paramID)));
                }

                if (parameters.getParameterCount() <= 0)
                {
                    auto n = audioProcessor->getParamIDs().size();

                    for (int i = 0; i < n; ++i)
                    {
                        auto vstParamID = audioProcessor->getVstParamIDForIndex (i);

                        if (vstParamID == audioProcessor->getProgramParamID())
                            continue;

                        auto* aloeParam = audioProcessor->getParamForVstParamID (vstParamID);
                        auto* parameterGroup = pluginInstance->getParameterTree().getGroupsForParameter (aloeParam).getLast();
                        auto unitID = AloeAudioProcessor::getUnitID (parameterGroup);

                        parameters.addParameter (new AloeVst3EditControllerParam (*this, *aloeParam, vstParamID, unitID,
                                                            (vstParamID == audioProcessor->getBypassParamID())));
                    }

                    const auto programParamId = audioProcessor->getProgramParamID();

                    if (auto* programParam = audioProcessor->getParamForVstParamID (programParamId))
                    {
                        ownedParameterListeners.push_back (std::make_unique<AloeVst3EditControllerOwnedParameterListener> (*this,
                                                                                                     *programParam,
                                                                                                     programParamId,
                                                                                                     audioProcessor->findCacheIndexForParamID (programParamId)));

                        parameters.addParameter (new AloeVst3EditControllerProgramChangeParameter (*pluginInstance, audioProcessor->getProgramParamID()));
                    }
                }

               #if ALOE_Vst3_EMULATE_MIDI_CC_WITH_PARAMETERS
                parameterToMidiControllerOffset = static_cast<VstParamID> (audioProcessor->isUsingManagedParameters() ? AloeAudioProcessor::paramMidiControllerOffset
                                                                                                                        : parameters.getParameterCount());

                initialiseMidiControllerMappings();
               #endif

                audioProcessorChanged (pluginInstance, ChangeDetails().withParameterInfoChanged (true));
            }
        */
    }
    
    pub fn initialise_midi_controller_mappings(&mut self)  {
        
        todo!();
        /*
            for (int c = 0, p = 0; c < numMIDIChannels; ++c)
            {
                for (int i = 0; i < VstkCountCtrlNumber; ++i, ++p)
                {
                    midiControllerToParameter[c][i] = static_cast<VstParamID> (p) + parameterToMidiControllerOffset;
                    parameterToMidiController[p].channel = c;
                    parameterToMidiController[p].ctrlNumber = i;

                    parameters.addParameter (new VstParameter (toString ("MIDI CC " + String (c) + "|" + String (i)),
                                             static_cast<VstParamID> (p) + parameterToMidiControllerOffset, nullptr, 0, 0,
                                             0, VstkRootUnitId));
                }
            }
        */
    }
    
    pub fn send_int_message(
        &mut self, 
        id_tag: *const u8,
        value:  i64
    ) {
        
        todo!();
        /*
            jassert (hostContext != nullptr);

            if (auto* message = allocateMessage())
            {
                const FReleaser releaser (message);
                message->setMessageID (idTag);
                message->getAttributes()->setInt (idTag, value);
                sendMessage (message);
            }
        */
    }
}
