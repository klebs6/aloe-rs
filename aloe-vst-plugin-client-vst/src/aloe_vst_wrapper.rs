crate::ix!();

/**
  | This is an AudioEffectX object that
  | holds and wraps our AudioProcessor...
  |
  */
#[no_copy]
#[leak_detector]
pub struct AloeVSTWrapper<'a> {
    base3:                         Timer,

    library_initialiser:           ScopedAloeInitialiser_GUI,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    message_thread:                SharedResourcePointer<MessageThread>,

    host_callback:                 Vst2AudioMasterCallback,
    processor:                     Box<AudioProcessor<'a>>,
    sample_rate:                   f64, // default = 44100.0
    block_size:                    i32, // default = 1024
    vst_effect:                    Vst2AEffect,
    state_information_lock:        CriticalSection,
    chunk_memory:                  MemoryBlock,
    chunk_memory_time:             u32, // default = 0
    editor_comp:                   Box<AloeVSTWrapperEditorCompWrapper<'a>>,
    editor_rect:                   Vst2ERect,
    midi_events:                   MidiBuffer,
    outgoing_events:               VSTMidiEventList,
    aloe_parameters:               LegacyAudioParametersWrapper,
    is_processing:                 bool, // default = false
    is_bypassed:                   bool, // default = false
    has_shutdown:                  bool, // default = false
    first_process_callback:        bool, // default = true
    should_delete_editor:          bool, // default = false

    #[cfg(target_os="macos")]
    #[cfg(ALOE_64BIT)]
    use_ns_view: bool, // default = true

    #[cfg(target_os="macos")]
    #[cfg(not(ALOE_64BIT))]
    use_ns_view: bool, // default = false

    float_temp_buffers:            VstTempBuffers<f32>,
    double_temp_buffers:           VstTempBuffers<f64>,
    max_num_in_channels:           i32, // default = 0
    max_num_out_channels:          i32, // default = 0
    cached_in_arrangement:         HeapBlock<Vst2VstSpeakerArrangement>,
    cached_out_arrangement:        HeapBlock<Vst2VstSpeakerArrangement>,
    in_parameter_changed_callback: ThreadLocalValue<bool>,
    host_change_updater:           AloeVSTWrapperHostChangeUpdater<'a>, //{ *this };
}

impl<'a> AudioProcessorListener for AloeVSTWrapper<'a> {

}

impl<'a> AudioProcessorChanged for AloeVSTWrapper<'a> {

    fn audio_processor_changed(
        &mut self, 
        _0:      *mut dyn AudioProcessorInterface,
        details: &AudioProcessorChangeDetails
    ) {
        
        todo!();
        /*
            hostChangeUpdater.update (details);
        */
    }
}

impl<'a> AudioProcessorParameterChanged for AloeVSTWrapper<'a> {

    fn audio_processor_parameter_changed(
        &mut self, 
        _0:        *mut dyn AudioProcessorInterface,
        index:     i32,
        new_value: f32
    ) {
        
        todo!();
        /*
            if (inParameterChangedCallback.get())
            {
                inParameterChangedCallback = false;
                return;
            }

            if (hostCallback != nullptr)
                hostCallback (&vstEffect, Vst2::audioMasterAutomate, index, 0, nullptr, newValue);
        */
    }
}

impl<'a> AudioProcessorParameterChangeGestureBegin for AloeVSTWrapper<'a> {

}

impl<'a> AudioProcessorParameterChangeGestureEnd for AloeVSTWrapper<'a> {

}

impl<'a> AudioPlayHeadInterface for AloeVSTWrapper<'a> {
    
    fn get_current_position(&mut self, info: &mut AudioPlayHeadCurrentPositionInfo) -> bool {
        
        todo!();
        /*
            const Vst2::VstTimeInfo* ti = nullptr;

            if (hostCallback != nullptr)
            {
                int32 flags = Vst2::kVstPpqPosValid  | Vst2::kVstTempoValid
                            | Vst2::kVstBarsValid    | Vst2::kVstCyclePosValid
                            | Vst2::kVstTimeSigValid | Vst2::kVstSmpteValid
                            | Vst2::kVstClockValid;

                auto result = hostCallback (&vstEffect, Vst2::audioMasterGetTime, 0, flags, nullptr, 0);
                ti = reinterpret_cast<Vst2::VstTimeInfo*> (result);
            }

            if (ti == nullptr || ti->sampleRate <= 0)
                return false;

            info.bpm = (ti->flags & Vst2::kVstTempoValid) != 0 ? ti->tempo : 0.0;

            if ((ti->flags & Vst2::kVstTimeSigValid) != 0)
            {
                info.timeSigNumerator   = ti->timeSigNumerator;
                info.timeSigDenominator = ti->timeSigDenominator;
            }
            else
            {
                info.timeSigNumerator   = 4;
                info.timeSigDenominator = 4;
            }

            info.timeInSamples = (int64) (ti->samplePos + 0.5);
            info.timeInSeconds = ti->samplePos / ti->sampleRate;
            info.ppqPosition = (ti->flags & Vst2::kVstPpqPosValid) != 0 ? ti->ppqPos : 0.0;
            info.ppqPositionOfLastBarStart = (ti->flags & Vst2::kVstBarsValid) != 0 ? ti->barStartPos : 0.0;

            if ((ti->flags & Vst2::kVstSmpteValid) != 0)
            {
                AudioPlayHead::FrameRateType rate = AudioPlayHead::fpsUnknown;
                double fps = 1.0;

                switch (ti->smpteFrameRate)
                {
                    case Vst2::kVstSmpte239fps:       rate = AudioPlayHead::fps23976;    fps = 24.0 * 1000.0 / 1001.0; break;
                    case Vst2::kVstSmpte24fps:        rate = AudioPlayHead::fps24;       fps = 24.0;  break;
                    case Vst2::kVstSmpte25fps:        rate = AudioPlayHead::fps25;       fps = 25.0;  break;
                    case Vst2::kVstSmpte2997fps:      rate = AudioPlayHead::fps2997;     fps = 30.0 * 1000.0 / 1001.0; break;
                    case Vst2::kVstSmpte30fps:        rate = AudioPlayHead::fps30;       fps = 30.0;  break;
                    case Vst2::kVstSmpte2997dfps:     rate = AudioPlayHead::fps2997drop; fps = 30.0 * 1000.0 / 1001.0; break;
                    case Vst2::kVstSmpte30dfps:       rate = AudioPlayHead::fps30drop;   fps = 30.0;  break;

                    case Vst2::kVstSmpteFilm16mm:
                    case Vst2::kVstSmpteFilm35mm:     fps = 24.0; break;

                    case Vst2::kVstSmpte249fps:       fps = 25.0 * 1000.0 / 1001.0; break;
                    case Vst2::kVstSmpte599fps:       fps = 60.0 * 1000.0 / 1001.0; break;
                    case Vst2::kVstSmpte60fps:        fps = 60; break;

                    default:                          jassertfalse; // unknown frame-rate..
                }

                info.frameRate = rate;
                info.editOriginTime = ti->smpteOffset / (80.0 * fps);
            }
            else
            {
                info.frameRate = AudioPlayHead::fpsUnknown;
                info.editOriginTime = 0;
            }

            info.isRecording = (ti->flags & Vst2::kVstTransportRecording) != 0;
            info.isPlaying   = (ti->flags & (Vst2::kVstTransportRecording | Vst2::kVstTransportPlaying)) != 0;
            info.isLooping   = (ti->flags & Vst2::kVstTransportCycleActive) != 0;

            if ((ti->flags & Vst2::kVstCyclePosValid) != 0)
            {
                info.ppqLoopStart = ti->cycleStartPos;
                info.ppqLoopEnd   = ti->cycleEndPos;
            }
            else
            {
                info.ppqLoopStart = 0;
                info.ppqLoopEnd = 0;
            }

            return true;
        */
    }

}

impl<'a> AudioProcessorParameterListener for AloeVSTWrapper<'a> {

    fn parameter_value_changed(&mut self, 
        _0:        i32,
        new_value: f32)  {
        
        todo!();
        /*
            // this can only come from the bypass parameter
            isBypassed = (newValue != 0.0f);
        */
    }
    
    fn parameter_gesture_changed(&mut self, 
        _0: i32,
        _1: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for AloeVSTWrapper<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            ALOE_AUTORELEASEPOOL
            {
               #if ALOE_LINUX || ALOE_BSD
                MessageManagerLock mmLock;
               #endif

                stopTimer();
                deleteEditor (false);

                hasShutdown = true;

                processor = nullptr;

                jassert (editorComp == nullptr);

                deleteTempChannels();

               #if ALOE_WINDOWS
                if (--numActivePlugins == 0)
                    messageThreadIsDefinitelyCorrect = false;
               #endif
            }
        */
    }
}

impl<'a> AloeVSTWrapper<'a> {

    pub fn new(
        cb: Vst2AudioMasterCallback,
        af: Box<AudioProcessor>) -> Self {
    
        todo!();
        /*


            : hostCallback (cb),
             processor (std::move (af))

            inParameterChangedCallback = false;

            // VST-2 does not support disabling buses: so always enable all of them
            processor->enableAllBuses();

            findMaxTotalChannels (maxNumInChannels, maxNumOutChannels);

            // You must at least have some channels
            jassert (processor->isMidiEffect() || (maxNumInChannels > 0 || maxNumOutChannels > 0));

            if (processor->isMidiEffect())
                maxNumInChannels = maxNumOutChannels = 2;

           #ifdef AloePlugin_PreferredChannelConfigurations
            processor->setPlayConfigDetails (maxNumInChannels, maxNumOutChannels, 44100.0, 1024);
           #endif

            processor->setRateAndBufferSizeDetails (0, 0);
            processor->setPlayHead (this);
            processor->addListener (this);

            if (auto* aloeParam = processor->getBypassParameter())
                aloeParam->addListener (this);

            aloeParameters.update (*processor, false);

            memset (&vstEffect, 0, sizeof (vstEffect));
            vstEffect.magic = 0x56737450 /* 'VstP' */;
            vstEffect.dispatcher = (Vst2::AEffectDispatcherProc) dispatcherCB;
            vstEffect.process = nullptr;
            vstEffect.setParameter = (Vst2::AEffectSetParameterProc) setParameterCB;
            vstEffect.getParameter = (Vst2::AEffectGetParameterProc) getParameterCB;
            vstEffect.numPrograms = jmax (1, processor->getNumPrograms());
            vstEffect.numParams = aloeParameters.getNumParameters();
            vstEffect.numInputs = maxNumInChannels;
            vstEffect.numOutputs = maxNumOutChannels;
            vstEffect.initialDelay = processor->getLatencySamples();
            vstEffect.object = this;
            vstEffect.uniqueID = AloePlugin_VSTUniqueID;

           #ifdef AloePlugin_VSTChunkStructureVersion
            vstEffect.version = AloePlugin_VSTChunkStructureVersion;
           #else
            vstEffect.version = AloePlugin_VersionCode;
           #endif

            vstEffect.processReplacing = (Vst2::AEffectProcessProc) processReplacingCB;
            vstEffect.processDoubleReplacing = (Vst2::AEffectProcessDoubleProc) processDoubleReplacingCB;

            vstEffect.flags |= Vst2::effFlagsHasEditor;

            vstEffect.flags |= Vst2::effFlagsCanReplacing;
            if (processor->supportsDoublePrecisionProcessing())
                vstEffect.flags |= Vst2::effFlagsCanDoubleReplacing;

            vstEffect.flags |= Vst2::effFlagsProgramChunks;

           #if AloePlugin_IsSynth
            vstEffect.flags |= Vst2::effFlagsIsSynth;
           #else
            if (processor->getTailLengthSeconds() == 0.0)
                vstEffect.flags |= Vst2::effFlagsNoSoundInStop;
           #endif

           #if ALOE_WINDOWS
            ++numActivePlugins;
           #endif
        */
    }
    
    pub fn get_aeffect(&mut self) -> *mut Vst2AEffect {
        
        todo!();
        /*
            return &vstEffect;
        */
    }
    
    pub fn internal_process_replacing<FloatType>(&mut self, 
        inputs:      *mut *mut FloatType,
        outputs:     *mut *mut FloatType,
        num_samples: i32,
        tmp_buffers: &mut VstTempBuffers<FloatType>)  {
    
        todo!();
        /*
            const bool isMidiEffect = processor->isMidiEffect();

            if (firstProcessCallback)
            {
                firstProcessCallback = false;

                // if this fails, the host hasn't called resume() before processing
                jassert (isProcessing);

                // (tragically, some hosts actually need this, although it's stupid to have
                //  to do it here..)
                if (! isProcessing)
                    resume();

                processor->setNonRealtime (isProcessLevelOffline());

               #if ALOE_WINDOWS
                if (getHostType().isWavelab())
                {
                    int priority = GetThreadPriority (GetCurrentThread());

                    if (priority <= THREAD_PRIORITY_NORMAL && priority >= THREAD_PRIORITY_LOWEST)
                        processor->setNonRealtime (true);
                }
               #endif
            }

           #if ALOE_DEBUG && ! (AloePlugin_ProducesMidiOutput || AloePlugin_IsMidiEffect)
            const int numMidiEventsComingIn = midiEvents.getNumEvents();
           #endif

            {
                const int numIn  = processor->getTotalNumInputChannels();
                const int numOut = processor->getTotalNumOutputChannels();

                const ScopedLock sl (processor->getCallbackLock());

                if (processor->isSuspended())
                {
                    for (int i = 0; i < numOut; ++i)
                        if (outputs[i] != nullptr)
                            FloatVectorOperations::clear (outputs[i], numSamples);
                }
                else
                {
                    int i;
                    for (i = 0; i < numOut; ++i)
                    {
                        auto* chan = tmpBuffers.tempChannels.getUnchecked(i);

                        if (chan == nullptr)
                        {
                            chan = outputs[i];

                            bool bufferPointerReusedForOtherChannels = false;

                            for (int j = i; --j >= 0;)
                            {
                                if (outputs[j] == chan)
                                {
                                    bufferPointerReusedForOtherChannels = true;
                                    break;
                                }
                            }

                            // if some output channels are disabled, some hosts supply the same buffer
                            // for multiple channels or supply a nullptr - this buggers up our method
                            // of copying the inputs over the outputs, so we need to create unique temp
                            // buffers in this case..
                            if (bufferPointerReusedForOtherChannels || chan == nullptr)
                            {
                                chan = new FloatType [(size_t) blockSize * 2];
                                tmpBuffers.tempChannels.set (i, chan);
                            }
                        }

                        if (i < numIn)
                        {
                            if (chan != inputs[i])
                                memcpy (chan, inputs[i], (size_t) numSamples * sizeof (FloatType));
                        }
                        else
                        {
                            FloatVectorOperations::clear (chan, numSamples);
                        }

                        tmpBuffers.channels[i] = chan;
                    }

                    for (; i < numIn; ++i)
                        tmpBuffers.channels[i] = inputs[i];

                    {
                        const int numChannels = jmax (numIn, numOut);
                        AudioBuffer<FloatType> chans (tmpBuffers.channels, isMidiEffect ? 0 : numChannels, numSamples);

                        if (isBypassed)
                            processor->processBlockBypassed (chans, midiEvents);
                        else
                            processor->processBlock (chans, midiEvents);
                    }

                    // copy back any temp channels that may have been used..
                    for (i = 0; i < numOut; ++i)
                        if (auto* chan = tmpBuffers.tempChannels.getUnchecked(i))
                            if (auto* dest = outputs[i])
                                memcpy (dest, chan, (size_t) numSamples * sizeof (FloatType));
                }
            }

            if (! midiEvents.isEmpty())
            {
               #if AloePlugin_ProducesMidiOutput || AloePlugin_IsMidiEffect
                auto numEvents = midiEvents.getNumEvents();

                outgoingEvents.ensureSize (numEvents);
                outgoingEvents.clear();

                for (const auto metadata : midiEvents)
                {
                    jassert (metadata.samplePosition >= 0 && metadata.samplePosition < numSamples);

                    outgoingEvents.addEvent (metadata.data, metadata.numBytes, metadata.samplePosition);
                }

                // Send VST events to the host.
                if (hostCallback != nullptr)
                    hostCallback (&vstEffect, Vst2::audioMasterProcessEvents, 0, 0, outgoingEvents.events, 0);
               #elif ALOE_DEBUG
                /*  This assertion is caused when you've added some events to the
                    midiMessages array in your processBlock() method, which usually means
                    that you're trying to send them somewhere. But in this case they're
                    getting thrown away.

                    If your plugin does want to send midi messages, you'll need to set
                    the AloePlugin_ProducesMidiOutput macro to 1 in your
                    AloePluginCharacteristics.h file.

                    If you don't want to produce any midi output, then you should clear the
                    midiMessages array at the end of your processBlock() method, to
                    indicate that you don't want any of the events to be passed through
                    to the output.
                */
                jassert (midiEvents.getNumEvents() <= numMidiEventsComingIn);
               #endif

                midiEvents.clear();
            }
        */
    }
    
    pub fn process_replacing(&mut self, 
        inputs:        *mut *mut f32,
        outputs:       *mut *mut f32,
        sample_frames: i32)  {
        
        todo!();
        /*
            jassert (! processor->isUsingDoublePrecision());
            internalProcessReplacing (inputs, outputs, sampleFrames, floatTempBuffers);
        */
    }
    
    pub fn process_replacingcb(
        vst_interface: *mut Vst2AEffect,
        inputs:        *mut *mut f32,
        outputs:       *mut *mut f32,
        sample_frames: i32)  {
        
        todo!();
        /*
            getWrapper (vstInterface)->processReplacing (inputs, outputs, sampleFrames);
        */
    }
    
    pub fn process_double_replacing(&mut self, 
        inputs:        *mut *mut f64,
        outputs:       *mut *mut f64,
        sample_frames: i32)  {
        
        todo!();
        /*
            jassert (processor->isUsingDoublePrecision());
            internalProcessReplacing (inputs, outputs, sampleFrames, doubleTempBuffers);
        */
    }
    
    pub fn process_double_replacingcb(
        vst_interface: *mut Vst2AEffect,
        inputs:        *mut *mut f64,
        outputs:       *mut *mut f64,
        sample_frames: i32)  {
        
        todo!();
        /*
            getWrapper (vstInterface)->processDoubleReplacing (inputs, outputs, sampleFrames);
        */
    }
    
    pub fn resume(&mut self)  {
        
        todo!();
        /*
            if (processor != nullptr)
            {
                isProcessing = true;

                auto numInAndOutChannels = static_cast<size_t> (vstEffect.numInputs + vstEffect.numOutputs);
                floatTempBuffers .channels.calloc (numInAndOutChannels);
                doubleTempBuffers.channels.calloc (numInAndOutChannels);

                auto currentRate = sampleRate;
                auto currentBlockSize = blockSize;

                firstProcessCallback = true;

                processor->setNonRealtime (isProcessLevelOffline());
                processor->setRateAndBufferSizeDetails (currentRate, currentBlockSize);

                deleteTempChannels();

                processor->prepareToPlay (currentRate, currentBlockSize);

                midiEvents.ensureSize (2048);
                midiEvents.clear();

                vstEffect.initialDelay = processor->getLatencySamples();

                /** If this plug-in is a synth or it can receive midi events we need to tell the
                    host that we want midi. In the SDK this method is marked as deprecated, but
                    some hosts rely on this behaviour.
                */
                if (vstEffect.flags & Vst2::effFlagsIsSynth || AloePlugin_WantsMidiInput || AloePlugin_IsMidiEffect)
                {
                    if (hostCallback != nullptr)
                        hostCallback (&vstEffect, Vst2::audioMasterWantMidi, 0, 1, nullptr, 0);
                }

                if (getHostType().isAbletonLive()
                     && hostCallback != nullptr
                     && processor->getTailLengthSeconds() == std::numeric_limits<double>::infinity())
                {
                    AbletonLiveHostSpecific hostCmd;

                    hostCmd.magic = 0x41624c69; // 'AbLi'
                    hostCmd.cmd = 5;
                    hostCmd.commandSize = sizeof (int);
                    hostCmd.flags = AbletonLiveHostSpecific::KCantBeSuspended;

                    hostCallback (&vstEffect, Vst2::audioMasterVendorSpecific, 0, 0, &hostCmd, 0.0f);
                }

               #if AloePlugin_ProducesMidiOutput || AloePlugin_IsMidiEffect
                outgoingEvents.ensureSize (512);
               #endif
            }
        */
    }
    
    pub fn suspend(&mut self)  {
        
        todo!();
        /*
            if (processor != nullptr)
            {
                processor->releaseResources();
                outgoingEvents.freeEvents();

                isProcessing = false;
                floatTempBuffers.channels.free();
                doubleTempBuffers.channels.free();

                deleteTempChannels();
            }
        */
    }
    
    pub fn get_parameter(&self, index: i32) -> f32 {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (index))
                return param->getValue();

            return 0.0f;
        */
    }
    
    pub fn get_parametercb(
        vst_interface: *mut Vst2AEffect,
        index:         i32) -> f32 {
        
        todo!();
        /*
            return getWrapper (vstInterface)->getParameter (index);
        */
    }
    
    pub fn set_parameter(&mut self, 
        index: i32,
        value: f32)  {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (index))
            {
                param->setValue (value);

                inParameterChangedCallback = true;
                param->sendValueChangedMessageToListeners (value);
            }
        */
    }
    
    pub fn set_parametercb(
        vst_interface: *mut Vst2AEffect,
        index:         i32,
        value:         f32)  {
        
        todo!();
        /*
            getWrapper (vstInterface)->setParameter (index, value);
        */
    }
    
    pub fn audio_processor_parameter_change_gesture_begin(&mut self, 
        _0:    *mut AudioProcessor,
        index: i32)  {
        
        todo!();
        /*
            if (hostCallback != nullptr)
                hostCallback (&vstEffect, Vst2::audioMasterBeginEdit, index, 0, nullptr, 0);
        */
    }
    
    pub fn audio_processor_parameter_change_gesture_end(&mut self, 
        _0:    *mut AudioProcessor,
        index: i32)  {
        
        todo!();
        /*
            if (hostCallback != nullptr)
                hostCallback (&vstEffect, Vst2::audioMasterEndEdit, index, 0, nullptr, 0);
        */
    }
    
    
    pub fn get_pin_properties(&self, 
        properties: &mut Vst2VstPinProperties,
        direction:  bool,
        index:      i32) -> bool {
        
        todo!();
        /*
            if (processor->isMidiEffect())
                return false;

            int channelIdx, busIdx;

            // fill with default
            properties.flags = 0;
            properties.label[0] = 0;
            properties.shortLabel[0] = 0;
            properties.arrangementType = Vst2::kSpeakerArrEmpty;

            if ((channelIdx = processor->getOffsetInBusBufferForAbsoluteChannelIndex (direction, index, busIdx)) >= 0)
            {
                auto& bus = *processor->getBus (direction, busIdx);
                auto& channelSet = bus.getCurrentLayout();
                auto channelType = channelSet.getTypeOfChannel (channelIdx);

                properties.flags = Vst2::kVstPinIsActive | Vst2::kVstPinUseSpeaker;
                properties.arrangementType = SpeakerMappings::channelSetToVstArrangementType (channelSet);
                String label = bus.getName();

               #ifdef AloePlugin_PreferredChannelConfigurations
                label += " " + String (channelIdx);
               #else
                if (channelSet.size() > 1)
                    label += " " + AudioChannelSet::getAbbreviatedChannelTypeName (channelType);
               #endif

                label.copyToUTF8 (properties.label, (size_t) (Vst2::kVstMaxLabelLen + 1));
                label.copyToUTF8 (properties.shortLabel, (size_t) (Vst2::kVstMaxShortLabelLen + 1));

                if (channelType == AudioChannelSet::left
                    || channelType == AudioChannelSet::leftSurround
                    || channelType == AudioChannelSet::leftCentre
                    || channelType == AudioChannelSet::leftSurroundSide
                    || channelType == AudioChannelSet::topFrontLeft
                    || channelType == AudioChannelSet::topRearLeft
                    || channelType == AudioChannelSet::leftSurroundRear
                    || channelType == AudioChannelSet::wideLeft)
                    properties.flags |= Vst2::kVstPinIsStereo;

                return true;
            }

            return false;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (shouldDeleteEditor)
            {
                shouldDeleteEditor = false;
                deleteEditor (true);
            }

            {
                ScopedLock lock (stateInformationLock);

                if (chunkMemoryTime > 0
                     && chunkMemoryTime < Time::getApproximateMillisecondCounter() - 2000
                     && ! recursionCheck)
                {
                    chunkMemory.reset();
                    chunkMemoryTime = 0;
                }
            }

            if (editorComp != nullptr)
                editorComp->checkVisibility();
        */
    }
    
    pub fn set_has_editor_flag(&mut self, should_set_has_editor: bool)  {
        
        todo!();
        /*
            auto hasEditor = (vstEffect.flags & Vst2::effFlagsHasEditor) != 0;

            if (shouldSetHasEditor == hasEditor)
                return;

            if (shouldSetHasEditor)
                vstEffect.flags |= Vst2::effFlagsHasEditor;
            else
                vstEffect.flags &= ~Vst2::effFlagsHasEditor;
        */
    }
    
    pub fn create_editor_comp(&mut self)  {
        
        todo!();
        /*
            if (hasShutdown || processor == nullptr)
                return;

            if (editorComp == nullptr)
            {
                if (auto* ed = processor->createEditorIfNeeded())
                {
                    setHasEditorFlag (true);
                    editorComp.reset (new AloeVSTWrapperEditorCompWrapper (*this, *ed));
                }
                else
                {
                    setHasEditorFlag (false);
                }
            }

            shouldDeleteEditor = false;
        */
    }
    
    pub fn delete_editor(&mut self, can_delete_later_if_modal: bool)  {
        
        todo!();
        /*
            ALOE_AUTORELEASEPOOL
            {
                typename PopupMenu::dismissAllActiveMenus();

                jassert (! recursionCheck);
                ScopedValueSetter<bool> svs (recursionCheck, true, false);

                if (editorComp != nullptr)
                {
                    if (auto* modalComponent = Component::getCurrentlyModalComponent())
                    {
                        modalComponent->exitModalState (0);

                        if (canDeleteLaterIfModal)
                        {
                            shouldDeleteEditor = true;
                            return;
                        }
                    }

                    editorComp->detachHostWindow();

                    if (auto* ed = editorComp->getEditorComp())
                        processor->editorBeingDeleted (ed);

                    editorComp = nullptr;

                    // there's some kind of component currently modal, but the host
                    // is trying to delete our plugin. You should try to avoid this happening..
                    jassert (Component::getCurrentlyModalComponent() == nullptr);
                }
            }
        */
    }
    
    pub fn dispatcher(&mut self, 
        op_code: i32,
        args:    VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (hasShutdown)
                return 0;

            switch (opCode)
            {
                case Vst2::effOpen:                     return handleOpen (args);
                case Vst2::effClose:                    return handleClose (args);
                case Vst2::effSetProgram:               return handleSetCurrentProgram (args);
                case Vst2::effGetProgram:               return handleGetCurrentProgram (args);
                case Vst2::effSetProgramName:           return handleSetCurrentProgramName (args);
                case Vst2::effGetProgramName:           return handleGetCurrentProgramName (args);
                case Vst2::effGetParamLabel:            return handleGetParameterLabel (args);
                case Vst2::effGetParamDisplay:          return handleGetParameterText (args);
                case Vst2::effGetParamName:             return handleGetParameterName (args);
                case Vst2::effSetSampleRate:            return handleSetSampleRate (args);
                case Vst2::effSetBlockSize:             return handleSetBlockSize (args);
                case Vst2::effMainsChanged:             return handleResumeSuspend (args);
                case Vst2::effEditGetRect:              return handleGetEditorBounds (args);
                case Vst2::effEditOpen:                 return handleOpenEditor (args);
                case Vst2::effEditClose:                return handleCloseEditor (args);
                case Vst2::effIdentify:                 return (pointer_sized_int) ByteOrder::bigEndianInt ("NvEf");
                case Vst2::effGetChunk:                 return handleGetData (args);
                case Vst2::effSetChunk:                 return handleSetData (args);
                case Vst2::effProcessEvents:            return handlePreAudioProcessingEvents (args);
                case Vst2::effCanBeAutomated:           return handleIsParameterAutomatable (args);
                case Vst2::effString2Parameter:         return handleParameterValueForText (args);
                case Vst2::effGetProgramNameIndexed:    return handleGetProgramName (args);
                case Vst2::effGetInputProperties:       return handleGetInputPinProperties (args);
                case Vst2::effGetOutputProperties:      return handleGetOutputPinProperties (args);
                case Vst2::effGetPlugCategory:          return handleGetPlugInCategory (args);
                case Vst2::effSetSpeakerArrangement:    return handleSetSpeakerConfiguration (args);
                case Vst2::effSetBypass:                return handleSetBypass (args);
                case Vst2::effGetEffectName:            return handleGetPlugInName (args);
                case Vst2::effGetProductString:         return handleGetPlugInName (args);
                case Vst2::effGetVendorString:          return handleGetManufacturerName (args);
                case Vst2::effGetVendorVersion:         return handleGetManufacturerVersion (args);
                case Vst2::effVendorSpecific:           return handleManufacturerSpecific (args);
                case Vst2::effCanDo:                    return handleCanPlugInDo (args);
                case Vst2::effGetTailSize:              return handleGetTailSize (args);
                case Vst2::effKeysRequired:             return handleKeyboardFocusRequired (args);
                case Vst2::effGetVstVersion:            return handleGetVstInterfaceVersion (args);
                case Vst2::effGetCurrentMidiProgram:    return handleGetCurrentMidiProgram (args);
                case Vst2::effGetSpeakerArrangement:    return handleGetSpeakerConfiguration (args);
                case Vst2::effSetTotalSampleToProcess:  return handleSetNumberOfSamplesToProcess (args);
                case Vst2::effSetProcessPrecision:      return handleSetSampleFloatType (args);
                case Vst2::effGetNumMidiInputChannels:  return handleGetNumMidiInputChannels();
                case Vst2::effGetNumMidiOutputChannels: return handleGetNumMidiOutputChannels();
                default:                                return 0;
            }
        */
    }
    
    pub fn dispatchercb(
        vst_interface: *mut Vst2AEffect,
        op_code:       i32,
        index:         i32,
        value:         PointerSizedInt,
        ptr:           *mut c_void,
        opt:           f32) -> PointerSizedInt {
        
        todo!();
        /*
            auto* wrapper = getWrapper (vstInterface);
            VstOpCodeArguments args = { index, value, ptr, opt };

            if (opCode == Vst2::effClose)
            {
                wrapper->dispatcher (opCode, args);
                delete wrapper;
                return 1;
            }

            return wrapper->dispatcher (opCode, args);
        */
    }
    
    pub fn get_wrapper(v: *mut Vst2AEffect) -> *mut AloeVSTWrapper<'a> {
        
        todo!();
        /*
            return static_cast<AloeVSTWrapper*> (v->object);
        */
    }
    
    pub fn is_process_level_offline(&mut self) -> bool {
        
        todo!();
        /*
            return hostCallback != nullptr
                    && (int32) hostCallback (&vstEffect, Vst2::audioMasterGetCurrentProcessLevel, 0, 0, nullptr, 0) == 4;
        */
    }
    
    pub fn convert_hex_version_to_decimal(hex_version: u32) -> i32 {
        
        todo!();
        /*
            #if ALOE_VST_RETURN_HEX_VERSION_NUMBER_DIRECTLY
            return (int32) hexVersion;
           #else
            // Currently, only Cubase displays the version number to the user
            // We are hoping here that when other DAWs start to display the version
            // number, that they do so according to yfede's encoding table in the link
            // below. If not, then this code will need an if (isSteinberg()) in the
            // future.
            int major = (hexVersion >> 16) & 0xff;
            int minor = (hexVersion >> 8) & 0xff;
            int bugfix = hexVersion & 0xff;

            // for details, see: https://forum.aloe.com/t/issues-with-version-integer-reported-by-vst2/23867

            // Encoding B
            if (major < 1)
                return major * 1000 + minor * 100 + bugfix * 10;

            // Encoding E
            if (major > 100)
                return major * 10000000 + minor * 100000 + bugfix * 1000;

            // Encoding D
            return static_cast<int32> (hexVersion);
           #endif
        */
    }

    /**
       Workarounds for hosts which attempt to open
       editor windows on a non-GUI
       thread.. (Grrrr...)
      */
    #[cfg(target_os="windows")]
    pub fn check_whether_message_thread_is_correct()  {
        
        todo!();
        /*
            auto host = getHostType();

            if (host.isWavelab() || host.isCubaseBridged() || host.isPremiere())
            {
                if (! messageThreadIsDefinitelyCorrect)
                {
                    MessageManager::getInstance()->setCurrentThreadAsMessageThread();

                    struct MessageThreadCallback  : public CallbackMessage
                    {
                        MessageThreadCallback (bool& tr) : triggered (tr) {}
                        void messageCallback() override     { triggered = true; }

                        bool& triggered;
                    };

                    (new MessageThreadCallback (messageThreadIsDefinitelyCorrect))->post();
                }
            }
        */
    }

    #[cfg(not(target_os="windows"))]
    pub fn check_whether_message_thread_is_correct()  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn delete_temp_channels_with_buffers<FloatType>(&mut self, tmp_buffers: &mut VstTempBuffers<FloatType>)  {
    
        todo!();
        /*
            tmpBuffers.release();

            if (processor != nullptr)
                tmpBuffers.tempChannels.insertMultiple (0, nullptr, vstEffect.numInputs
                                                                     + vstEffect.numOutputs);
        */
    }
    
    pub fn delete_temp_channels(&mut self)  {
        
        todo!();
        /*
            deleteTempChannels (floatTempBuffers);
            deleteTempChannels (doubleTempBuffers);
        */
    }
    
    pub fn find_max_total_channels(&mut self, 
        max_total_ins:  &mut i32,
        max_total_outs: &mut i32)  {
        
        todo!();
        /*
            #ifdef AloePlugin_PreferredChannelConfigurations
            int configs[][2] = { AloePlugin_PreferredChannelConfigurations };
            maxTotalIns = maxTotalOuts = 0;

            for (auto& config : configs)
            {
                maxTotalIns =  jmax (maxTotalIns,  config[0]);
                maxTotalOuts = jmax (maxTotalOuts, config[1]);
            }
           #else
            auto numInputBuses  = processor->getBusCount (true);
            auto numOutputBuses = processor->getBusCount (false);

            if (numInputBuses > 1 || numOutputBuses > 1)
            {
                maxTotalIns = maxTotalOuts = 0;

                for (int i = 0; i < numInputBuses; ++i)
                    maxTotalIns  += processor->getChannelCountOfBus (true, i);

                for (int i = 0; i < numOutputBuses; ++i)
                    maxTotalOuts += processor->getChannelCountOfBus (false, i);
            }
            else
            {
                maxTotalIns  = numInputBuses  > 0 ? processor->getBus (true,  0)->getMaxSupportedChannels (64) : 0;
                maxTotalOuts = numOutputBuses > 0 ? processor->getBus (false, 0)->getMaxSupportedChannels (64) : 0;
            }
           #endif
        */
    }
    
    pub fn plugin_has_sidechains_or_auxs(&self) -> bool {
        
        todo!();
        /*
            return (processor->getBusCount (true) > 1 || processor->getBusCount (false) > 1);
        */
    }

    /**
      | Host to plug-in calls.
      |
      */
    pub fn handle_open(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            // Note: most hosts call this on the UI thread, but wavelab doesn't, so be careful in here.
            setHasEditorFlag (processor->hasEditor());

            return 0;
        */
    }
    
    pub fn handle_close(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            // Note: most hosts call this on the UI thread, but wavelab doesn't, so be careful in here.
            stopTimer();

            if (MessageManager::getInstance()->isThisTheMessageThread())
                deleteEditor (false);

            return 0;
        */
    }
    
    pub fn handle_set_current_program(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr && isPositiveAndBelow ((int) args.value, processor->getNumPrograms()))
                processor->setCurrentProgram ((int) args.value);

            return 0;
        */
    }
    
    pub fn handle_get_current_program(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return (processor != nullptr && processor->getNumPrograms() > 0 ? processor->getCurrentProgram() : 0);
        */
    }
    
    pub fn handle_set_current_program_name(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr && processor->getNumPrograms() > 0)
                processor->changeProgramName (processor->getCurrentProgram(), (char*) args.ptr);

            return 0;
        */
    }
    
    pub fn handle_get_current_program_name(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr && processor->getNumPrograms() > 0)
                processor->getProgramName (processor->getCurrentProgram()).copyToUTF8 ((char*) args.ptr, 24 + 1);

            return 0;
        */
    }
    
    pub fn handle_get_parameter_label(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (args.index))
            {
                // length should technically be kVstMaxParamStrLen, which is 8, but hosts will normally allow a bit more.
                param->getLabel().copyToUTF8 ((char*) args.ptr, 24 + 1);
            }

            return 0;
        */
    }
    
    pub fn handle_get_parameter_text(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (args.index))
            {
                // length should technically be kVstMaxParamStrLen, which is 8, but hosts will normally allow a bit more.
                param->getCurrentValueAsText().copyToUTF8 ((char*) args.ptr, 24 + 1);
            }

            return 0;
        */
    }
    
    pub fn handle_get_parameter_name(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (args.index))
            {
                // length should technically be kVstMaxParamStrLen, which is 8, but hosts will normally allow a bit more.
                param->getName (32).copyToUTF8 ((char*) args.ptr, 32 + 1);
            }

            return 0;
        */
    }
    
    pub fn handle_set_sample_rate(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            sampleRate = args.opt;
            return 0;
        */
    }
    
    pub fn handle_set_block_size(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            blockSize = (int32) args.value;
            return 0;
        */
    }
    
    pub fn handle_resume_suspend(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (args.value)
                resume();
            else
                suspend();

            return 0;
        */
    }
    
    pub fn handle_get_editor_bounds(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            checkWhetherMessageThreadIsCorrect();
            const MessageManagerLock mmLock;
            createEditorComp();

            if (editorComp != nullptr)
            {
                editorComp->getEditorBounds (editorRect);
                *((Vst2::ERect**) args.ptr) = &editorRect;
                return (pointer_sized_int) &editorRect;
            }

            return 0;
        */
    }
    
    pub fn handle_open_editor(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            checkWhetherMessageThreadIsCorrect();
            const MessageManagerLock mmLock;
            jassert (! recursionCheck);

            startTimerHz (4); // performs misc housekeeping chores

            deleteEditor (true);
            createEditorComp();

            if (editorComp != nullptr)
            {
                editorComp->attachToHost (args);
                return 1;
            }

            return 0;
        */
    }
    
    pub fn handle_close_editor(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            checkWhetherMessageThreadIsCorrect();
            const MessageManagerLock mmLock;
            deleteEditor (true);
            return 0;
        */
    }
    
    pub fn handle_get_data(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor == nullptr)
                return 0;

            auto data = (void**) args.ptr;
            bool onlyStoreCurrentProgramData = (args.index != 0);

            ScopedLock lock (stateInformationLock);
            chunkMemory.reset();

            if (onlyStoreCurrentProgramData)
                processor->getCurrentProgramStateInformation (chunkMemory);
            else
                processor->getStateInformation (chunkMemory);

            *data = (void*) chunkMemory.getData();

            // because the chunk is only needed temporarily by the host (or at least you'd
            // hope so) we'll give it a while and then free it in the timer callback.
            chunkMemoryTime = Time::getApproximateMillisecondCounter();

            return (int32) chunkMemory.getSize();
        */
    }
    
    pub fn handle_set_data(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr)
            {
                void* data = args.ptr;
                int32 byteSize = (int32) args.value;
                bool onlyRestoreCurrentProgramData = (args.index != 0);

                {
                    ScopedLock lock (stateInformationLock);

                    chunkMemory.reset();
                    chunkMemoryTime = 0;

                    if (byteSize > 0 && data != nullptr)
                    {
                        if (onlyRestoreCurrentProgramData)
                            processor->setCurrentProgramStateInformation (data, byteSize);
                        else
                            processor->setStateInformation (data, byteSize);
                    }
                }
            }

            return 0;
        */
    }
    
    pub fn handle_pre_audio_processing_events(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            #if AloePlugin_WantsMidiInput || AloePlugin_IsMidiEffect
            VSTMidiEventList::addEventsToMidiBuffer ((Vst2::VstEvents*) args.ptr, midiEvents);
            return 1;
           #else
            ignoreUnused (args);
            return 0;
           #endif
        */
    }
    
    pub fn handle_is_parameter_automatable(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (args.index))
            {
                const bool isMeter = ((((unsigned int) param->getCategory() & 0xffff0000) >> 16) == 2);
                return (param->isAutomatable() && (! isMeter) ? 1 : 0);
            }

            return 0;
        */
    }
    
    pub fn handle_parameter_value_for_text(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* param = aloeParameters.getParamForIndex (args.index))
            {
                if (! LegacyAudioParameter::isLegacy (param))
                {
                    auto value = param->getValueForText (String::fromUTF8 ((char*) args.ptr));
                    param->setValue (value);

                    inParameterChangedCallback = true;
                    param->sendValueChangedMessageToListeners (value);

                    return 1;
                }
            }

            return 0;
        */
    }
    
    pub fn handle_get_program_name(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr && isPositiveAndBelow (args.index, processor->getNumPrograms()))
            {
                processor->getProgramName (args.index).copyToUTF8 ((char*) args.ptr, 24 + 1);
                return 1;
            }

            return 0;
        */
    }
    
    pub fn handle_get_input_pin_properties(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return (processor != nullptr && getPinProperties (*(Vst2::VstPinProperties*) args.ptr, true, args.index)) ? 1 : 0;
        */
    }
    
    pub fn handle_get_output_pin_properties(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return (processor != nullptr && getPinProperties (*(Vst2::VstPinProperties*) args.ptr, false, args.index)) ? 1 : 0;
        */
    }
    
    pub fn handle_get_plug_in_category(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return Vst2::AloePlugin_VSTCategory;
        */
    }
    
    pub fn handle_set_speaker_configuration(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            auto* pluginInput  = reinterpret_cast<Vst2::VstSpeakerArrangement*> (args.value);
            auto* pluginOutput = reinterpret_cast<Vst2::VstSpeakerArrangement*> (args.ptr);

            if (processor->isMidiEffect())
                return 0;

            auto numIns  = processor->getBusCount (true);
            auto numOuts = processor->getBusCount (false);

            if (pluginInput != nullptr && pluginInput->type >= 0)
            {
                // inconsistent request?
                if (SpeakerMappings::vstArrangementTypeToChannelSet (*pluginInput).size() != pluginInput->numChannels)
                    return 0;
            }

            if (pluginOutput != nullptr && pluginOutput->type >= 0)
            {
                // inconsistent request?
                if (SpeakerMappings::vstArrangementTypeToChannelSet (*pluginOutput).size() != pluginOutput->numChannels)
                    return 0;
            }

            if (pluginInput != nullptr  && pluginInput->numChannels  > 0 && numIns  == 0)
                return 0;

            if (pluginOutput != nullptr && pluginOutput->numChannels > 0 && numOuts == 0)
                return 0;

            auto layouts = processor->getBusesLayout();

            if (pluginInput != nullptr && pluginInput-> numChannels >= 0 && numIns  > 0)
                layouts.getChannelSet (true,  0) = SpeakerMappings::vstArrangementTypeToChannelSet (*pluginInput);

            if (pluginOutput != nullptr && pluginOutput->numChannels >= 0 && numOuts > 0)
                layouts.getChannelSet (false, 0) = SpeakerMappings::vstArrangementTypeToChannelSet (*pluginOutput);

           #ifdef AloePlugin_PreferredChannelConfigurations
            short configs[][2] = { AloePlugin_PreferredChannelConfigurations };
            if (! AudioProcessor::containsLayout (layouts, configs))
                return 0;
           #endif

            return processor->setBusesLayout (layouts) ? 1 : 0;
        */
    }
    
    pub fn handle_set_bypass(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            isBypassed = (args.value != 0);

            if (auto* bypass = processor->getBypassParameter())
                bypass->setValueNotifyingHost (isBypassed ? 1.0f : 0.0f);

            return 1;
        */
    }
    
    pub fn handle_get_plug_in_name(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            String (AloePlugin_Name).copyToUTF8 ((char*) args.ptr, 64 + 1);
            return 1;
        */
    }
    
    pub fn handle_get_manufacturer_name(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            String (AloePlugin_Manufacturer).copyToUTF8 ((char*) args.ptr, 64 + 1);
            return 1;
        */
    }
    
    pub fn handle_get_manufacturer_version(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return convertHexVersionToDecimal (AloePlugin_VersionCode);
        */
    }
    
    pub fn handle_manufacturer_specific(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (handleManufacturerSpecificVST2Opcode (args.index, args.value, args.ptr, args.opt))
                return 1;

            if (args.index == (int32) ByteOrder::bigEndianInt ("PreS")
                 && args.value == (int32) ByteOrder::bigEndianInt ("AeCs"))
                return handleSetContentScaleFactor (args.opt);

            if (args.index == Vst2::effGetParamDisplay)
                return handleCockosGetParameterText (args.value, args.ptr, args.opt);

            if (auto callbackHandler = dynamic_cast<VSTCallbackHandler*> (processor.get()))
                return callbackHandler->handleVstManufacturerSpecific (args.index, args.value, args.ptr, args.opt);

            return 0;
        */
    }
    
    pub fn handle_can_plug_in_do(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            auto text = (const char*) args.ptr;
            auto matches = [=] (const char* s) { return strcmp (text, s) == 0; };

            if (matches ("receiveVstEvents")
             || matches ("receiveVstMidiEvent")
             || matches ("receiveVstMidiEvents"))
            {
               #if AloePlugin_WantsMidiInput || AloePlugin_IsMidiEffect
                return 1;
               #else
                return -1;
               #endif
            }

            if (matches ("sendVstEvents")
             || matches ("sendVstMidiEvent")
             || matches ("sendVstMidiEvents"))
            {
               #if AloePlugin_ProducesMidiOutput || AloePlugin_IsMidiEffect
                return 1;
               #else
                return -1;
               #endif
            }

            if (matches ("receiveVstTimeInfo")
             || matches ("conformsToWindowRules")
             || matches ("supportsViewDpiScaling")
             || matches ("bypass"))
            {
                return 1;
            }

            // This tells Wavelab to use the UI thread to invoke open/close,
            // like all other hosts do.
            if (matches ("openCloseAnyThread"))
                return -1;

            if (matches ("MPE"))
                return processor->supportsMPE() ? 1 : 0;

           #if ALOE_MAC
            if (matches ("hasCockosViewAsConfig"))
            {
                useNSView = true;
                return (int32) 0xbeef0000;
            }
           #endif

            if (matches ("hasCockosExtensions"))
                return (int32) 0xbeef0000;

            if (auto callbackHandler = dynamic_cast<VSTCallbackHandler*> (processor.get()))
                return callbackHandler->handleVstPluginCanDo (args.index, args.value, args.ptr, args.opt);

            return 0;
        */
    }
    
    pub fn handle_get_tail_size(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr)
            {
                int32 result;

                auto tailSeconds = processor->getTailLengthSeconds();

                if (tailSeconds == std::numeric_limits<double>::infinity())
                    result = std::numeric_limits<int32>::max();
                else
                    result = static_cast<int32> (tailSeconds * sampleRate);

                return result; // Vst2 expects an int32 upcasted to a intptr_t here
            }

            return 0;
        */
    }
    
    pub fn handle_keyboard_focus_required(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6326)
            return (AloePlugin_EditorRequiresKeyboardFocus != 0) ? 1 : 0;
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
    
    pub fn handle_get_vst_interface_version(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return kVstVersion;
        */
    }
    
    pub fn handle_get_current_midi_program(&mut self, _0: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return -1;
        */
    }
    
    pub fn handle_get_speaker_configuration(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            auto** pluginInput  = reinterpret_cast<Vst2::VstSpeakerArrangement**> (args.value);
            auto** pluginOutput = reinterpret_cast<Vst2::VstSpeakerArrangement**> (args.ptr);

            if (pluginHasSidechainsOrAuxs() || processor->isMidiEffect())
                return false;

            auto inputLayout  = processor->getChannelLayoutOfBus (true,  0);
            auto outputLayout = processor->getChannelLayoutOfBus (false,  0);

            auto speakerBaseSize = sizeof (Vst2::VstSpeakerArrangement) - (sizeof (Vst2::VstSpeakerProperties) * 8);

            cachedInArrangement .malloc (speakerBaseSize + (static_cast<std::size_t> (inputLayout. size()) * sizeof (Vst2::VstSpeakerArrangement)), 1);
            cachedOutArrangement.malloc (speakerBaseSize + (static_cast<std::size_t> (outputLayout.size()) * sizeof (Vst2::VstSpeakerArrangement)), 1);

            *pluginInput  = cachedInArrangement. getData();
            *pluginOutput = cachedOutArrangement.getData();

            SpeakerMappings::channelSetToVstArrangement (processor->getChannelLayoutOfBus (true,  0), **pluginInput);
            SpeakerMappings::channelSetToVstArrangement (processor->getChannelLayoutOfBus (false, 0), **pluginOutput);

            return 1;
        */
    }
    
    pub fn handle_set_number_of_samples_to_process(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            return args.value;
        */
    }
    
    pub fn handle_set_sample_float_type(&mut self, args: VstOpCodeArguments) -> PointerSizedInt {
        
        todo!();
        /*
            if (! isProcessing)
            {
                if (processor != nullptr)
                {
                    processor->setProcessingPrecision ((args.value == Vst2::kVstProcessPrecision64
                                                         && processor->supportsDoublePrecisionProcessing())
                                                             ? AudioProcessor::doublePrecision
                                                             : AudioProcessor::singlePrecision);

                    return 1;
                }
            }

            return 0;
        */
    }
    
    pub fn handle_set_content_scale_factor(&mut self, scale: f32) -> PointerSizedInt {
        
        todo!();
        /*
            #if ! ALOE_MAC
            if (editorComp != nullptr)
                editorComp->setContentScaleFactor (scale);
           #else
            ignoreUnused (scale);
           #endif

            return 1;
        */
    }
    
    pub fn handle_cockos_get_parameter_text(&mut self, 
        param_index: PointerSizedInt,
        dest:        *mut c_void,
        value:       f32) -> PointerSizedInt {
        
        todo!();
        /*
            if (processor != nullptr && dest != nullptr)
            {
                if (auto* param = aloeParameters.getParamForIndex ((int) paramIndex))
                {
                    if (! LegacyAudioParameter::isLegacy (param))
                    {
                        String text (param->getText (value, 1024));
                        memcpy (dest, text.toRawUTF8(), ((size_t) text.length()) + 1);
                        return 0xbeef;
                    }
                }
            }

            return 0;
        */
    }
    
    pub fn handle_get_num_midi_input_channels(&mut self) -> PointerSizedInt {
        
        todo!();
        /*
            #if AloePlugin_WantsMidiInput || AloePlugin_IsMidiEffect
            #ifdef AloePlugin_VSTNumMidiInputs
             return AloePlugin_VSTNumMidiInputs;
            #else
             return 16;
            #endif
           #else
            return 0;
           #endif
        */
    }
    
    pub fn handle_get_num_midi_output_channels(&mut self) -> PointerSizedInt {
        
        todo!();
        /*
            #if AloePlugin_ProducesMidiOutput || AloePlugin_IsMidiEffect
            #ifdef AloePlugin_VSTNumMidiOutputs
             return AloePlugin_VSTNumMidiOutputs;
            #else
             return 16;
            #endif
           #else
            return 0;
           #endif
        */
    }
}
