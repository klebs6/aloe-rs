crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/RTAS/aloe_RTAS_Wrapper.cpp]

#[cfg(AloePlugin_Build_RTAS)]
pub mod rtas {
    use super::*;

    /**
      (this is a workaround for a build problem in VC9)
      */
    #[cfg(_MSC_VER)]
    pub const _DO_NOT_DECLARE_INTERLOCKED_INTRINSICS_IN_MEMORY: bool = true;

    /* Note about include paths
       ------------------------

       To be able to include all the Digidesign headers correctly, you'll need to add this
       lot to your include path:

        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\EffectClasses
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\ProcessClasses
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\ProcessClasses\Interfaces
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\Utilities
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\RTASP_Adapt
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\CoreClasses
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\Controls
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\Meters
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\ViewClasses
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\DSPClasses
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\PluginLibrary\Interfaces
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\common
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\common\Platform
        c:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugins\SignalProcessing\Public
        C:\yourdirectory\PT_80_SDK\AlturaPorts\TDMPlugIns\DSPManager\Interfaces
        c:\yourdirectory\PT_80_SDK\AlturaPorts\SADriver\Interfaces
        c:\yourdirectory\PT_80_SDK\AlturaPorts\DigiPublic\Interfaces
        c:\yourdirectory\PT_80_SDK\AlturaPorts\Fic\Interfaces\DAEClient
        c:\yourdirectory\PT_80_SDK\AlturaPorts\NewFileLibs\Cmn
        c:\yourdirectory\PT_80_SDK\AlturaPorts\NewFileLibs\DOA
        c:\yourdirectory\PT_80_SDK\AlturaPorts\AlturaSource\PPC_H
        c:\yourdirectory\PT_80_SDK\AlturaPorts\AlturaSource\AppSupport
        c:\yourdirectory\PT_80_SDK\AlturaPorts\DigiPublic
        c:\yourdirectory\PT_80_SDK\AvidCode\AVX2sdk\AVX\avx2\avx2sdk\inc
        c:\yourdirectory\PT_80_SDK\xplat\AVX\avx2\avx2sdk\inc

       NB. If you hit a huge pile of bugs around here, make sure that you've not got the
       Apple QuickTime headers before the PT headers in your path, because there are
       some filename clashes between them.

    */

    /**
      | This ALOE_RTAS_LINK_TO_DEBUG_LIB setting can
      | be used to force linkage against only the
      | release build of the RTAS lib, since in older
      | SDKs there can be problems with the debug
      | build.
      */
    #[cfg(_MSC_VER)]
    #[cfg(all(ALOE_DEBUG,not(ALOE_RTAS_LINK_TO_DEBUG_LIB)))]
    pub const ALOE_RTAS_LINK_TO_DEBUG_LIB: usize = 1;

    #[cfg(_MSC_VER)]
    #[cfg(ALOE_RTAS_LINK_TO_DEBUG_LIB)]
    macro_rules! pt_lib_path {
        () => {
            /*
                    AloePlugin_WinBag_path "\\Debug\\lib\\"
            */
        }
    }

    #[cfg(_MSC_VER)]
    #[cfg(not(ALOE_RTAS_LINK_TO_DEBUG_LIB))]
    macro_rules! pt_lib_path {
        () => {
            /*
                    AloePlugin_WinBag_path "\\Release\\lib\\"
            */
        }
    }

    lazy_static!{
        /*
        #if ALOE_WINDOWS
          extern void  attachSubWindow (void* hostWindow, int& titleW, int& titleH, Component* comp);
          extern void  resizeHostWindow (void* hostWindow, int& titleW, int& titleH, Component* comp);
         #if ! AloePlugin_EditorRequiresKeyboardFocus
          extern void  passFocusToHostWindow (void* hostWindow);
         #endif
        #else
          extern void* attachSubWindow (void* hostWindowRef, Component* comp);
          extern void removeSubWindow (void* nsWindow, Component* comp);
          extern void forwardCurrentKeyEventToHostWindow();
        #endif
        */
    }

    #[cfg(not(any(ALOE_DEBUG,ALOE_RTAS_PLUGINGESTALT_IS_CACHEABLE)))]
    pub const ALOE_RTAS_PLUGINGESTALT_IS_CACHEABLE: usize = 1;

    pub const midiBufferSize:     i32 = 1024;
    pub const aloeChunkType:      OSType = "aloe";
    pub const bypassControlIndex: i32 = 1;

    lazy_static!{
        /*
        static int numInstances = 0;
        */
    }

    pub struct AloePlugInProcess {
        base:             CEffectProcessMIDI,
        base2:            CEffectProcessRTAS,
        base3:            AudioProcessorListener,
        base4:            AudioPlayHead,
        aloe_filter:      Box<AudioProcessor>,
        midi_events:      MidiBuffer,
        midi_buffer_node: Box<CEffectMIDIOtherBufferedNode>,
        midi_transport:   Box<CEffectMIDITransport>,
        midi_buffer:      [DirectMidiPacket; midiBufferSize],
        temp_filter_data: MemoryBlock,
        channels:         HeapBlock<*mut f32>,
        sample_rate:      f64, // default = 44100.0
    }

    pub mod aloe_plugin_process {
        use super::*;

        #[no_copy]
        pub struct AloePluginControl {
            base:      CPluginControl,
            processor: &mut AudioProcessor,
            index:     i32,
            paramid:   OSType,
        }

        impl AloePluginControl {

            pub fn new(
                p:            &mut AudioProcessor,
                i:            i32,
                rtas_paramid: OSType) -> Self {
            
                todo!();
                /*
                : processor(p),
                : index(i),
                : paramid(rtasParamID),

                    CPluginControl::SetValue (GetDefaultValue());
                */
            }
            
            pub fn getid(&self) -> OSType {
                
                todo!();
                /*
                    return paramID;
                */
            }
            
            pub fn get_default_value(&self) -> i64 {
                
                todo!();
                /*
                    return floatToLong (processor.getParameterDefaultValue (index));
                */
            }
            
            pub fn set_default_value(&mut self, _0: i64)  {
                
                todo!();
                /*
                
                */
            }
            
            pub fn get_num_steps(&self) -> i64 {
                
                todo!();
                /*
                    return processor.getParameterNumSteps (index);
                */
            }
            
            pub fn convert_string_to_value(&self, value_string: *const u8) -> i64 {
                
                todo!();
                /*
                    return floatToLong (String (valueString).getFloatValue());
                */
            }
            
            pub fn is_key_valid(&self, key: i64) -> bool {
                
                todo!();
                /*
                    return true;
                */
            }
            
            pub fn get_name_of_length(&self, 
                name:               *mut u8,
                max_length:         i32,
                in_controller_type: OSType)  {
                
                todo!();
                /*
                    // Pro-tools expects all your parameters to have valid names!
                        jassert (processor.getParameterName (index, maxLength).isNotEmpty());

                        processor.getParameterName (index, maxLength).copyToUTF8 (name, (size_t) maxLength + 1);
                */
            }
            
            pub fn get_priority(&self) -> i64 {
                
                todo!();
                /*
                    return kFicCooperativeTaskPriority;
                */
            }
            
            pub fn get_orientation(&self) -> i64 {
                
                todo!();
                /*
                    return processor.isParameterOrientationInverted (index)
                                 ? kDAE_RightMinLeftMax | kDAE_TopMinBottomMax | kDAE_RotarySingleDotMode | kDAE_RotaryRightMinLeftMax
                                 : kDAE_LeftMinRightMax | kDAE_BottomMinTopMax | kDAE_RotarySingleDotMode | kDAE_RotaryLeftMinRightMax;
                */
            }
            
            pub fn get_control_type(&self) -> i64 {
                
                todo!();
                /*
                    return kDAE_ContinuousValues;
                */
            }
            
            pub fn get_value_string(&self, 
                value_string: *mut u8,
                max_length:   i32,
                value:        i64)  {
                
                todo!();
                /*
                    processor.getParameterText (index, maxLength).copyToUTF8 (valueString, (size_t) maxLength + 1);
                */
            }
            
            pub fn is_automatable(&self) -> bool {
                
                todo!();
                /*
                    return processor.isParameterAutomatable (index);
                */
            }
        }

            
        pub struct AloeCustomUIView {
            base:        CCustomView,
            base2:       Timer,
            filter:      *const AudioProcessor,
            process:     *const AloePlugInProcess,
            wrapper:     Box<Component>,
            editor_comp: Box<AudioProcessorEditor>,
        }

        pub mod aloe_custom_uiview {
            use super::*;
                
            /**
              | A component to hold the
              | AudioProcessorEditor, and cope with
              | some housekeeping chores when it
              | changes or repaints.
              */
            #[no_copy]
            #[leak_detector]
            pub struct EditorCompWrapper {
                base:        Component,

                #[cfg(not(target_os="macos"))]
                base2:       FocusChangeListener,

                host_window: *const c_void,
                ns_window:   *mut c_void,
                owner:       *const AloeCustomUIView,
                titlew:      i32,
                titleh:      i32,
            }

            impl Drop for EditorCompWrapper {
                fn drop(&mut self) {
                    todo!();
                    /* 
                                removeChildComponent (getEditor());

                               #if ALOE_WINDOWS && ! AloePlugin_EditorRequiresKeyboardFocus
                                Desktop::getInstance().removeFocusChangeListener (this);
                               #endif

                               #if ALOE_MAC
                                removeSubWindow (nsWindow, this);
                               #endif
                             */
                }
            }

            impl EditorCompWrapper {

                pub fn new(
                    host_window: *mut c_void,
                    editor_comp: *mut Component,
                    owner:       *mut AloeCustomUIView) -> Self {
                
                    todo!();
                    /*


                        : hostWindow (hostWindow_),
                                  owner (owner_),
                                  titleW (0),
                                  titleH (0)

                               #if ! AloePlugin_EditorRequiresKeyboardFocus
                                setMouseClickGrabsKeyboardFocus (false);
                                setWantsKeyboardFocus (false);
                               #endif
                                setOpaque (true);
                                setBroughtToFrontOnMouseClick (true);
                                setBounds (editorComp->getBounds());
                                editorComp->setTopLeftPosition (0, 0);
                                addAndMakeVisible (*editorComp);

                               #if ALOE_WINDOWS
                                attachSubWindow (hostWindow, titleW, titleH, this);
                               #else
                                nsWindow = attachSubWindow (hostWindow, this);
                               #endif
                                setVisible (true);

                               #if ALOE_WINDOWS && ! AloePlugin_EditorRequiresKeyboardFocus
                                Desktop::getInstance().addFocusChangeListener (this);
                               #endif
                    */
                }
                
                pub fn paint(&mut self, _0: &mut Graphics)  {
                    
                    todo!();
                    /*
                    
                    */
                }
                
                pub fn resized(&mut self)  {
                    
                    todo!();
                    /*
                        if (Component* const ed = getEditor())
                                    ed->setBounds (getLocalBounds());

                                repaint();
                    */
                }

                #[cfg(target_os="windows")]
                pub fn global_focus_changed(&mut self, _0: *mut Component)  {
                    
                    todo!();
                    /*
                        #if ! AloePlugin_EditorRequiresKeyboardFocus
                                if (hasKeyboardFocus (true))
                                    passFocusToHostWindow (hostWindow);
                               #endif
                    */
                }
                
                pub fn child_bounds_changed(&mut self, child: *mut Component)  {
                    
                    todo!();
                    /*
                        setSize (child->getWidth(), child->getHeight());
                                child->setTopLeftPosition (0, 0);

                               #if ALOE_WINDOWS
                                resizeHostWindow (hostWindow, titleW, titleH, this);
                               #endif
                                owner->updateSize();
                    */
                }
                
                pub fn user_tried_to_close_window(&mut self)  {
                    
                    todo!();
                    /*
                    
                    */
                }

                #[cfg(all(target_os="macos",AloePlugin_EditorRequiresKeyboardFocus))]
                pub fn key_pressed(&mut self, kp: &KeyPress) -> bool {
                    
                    todo!();
                    /*
                        owner->updateSize();
                                forwardCurrentKeyEventToHostWindow();
                                return true;
                    */
                }
                
                pub fn get_editor(&self) -> *mut Component {
                    
                    todo!();
                    /*
                        return getChildComponent (0);
                    */
                }
            }
        }

        impl Drop for AloeCustomUIView {
            fn drop(&mut self) {
                todo!();
                /* 
                        deleteEditorComp();
                     */
            }
        }

        impl AloeCustomUIView {
            
            pub fn new(
                ap: *mut AudioProcessor,
                p:  *mut AloePlugInProcess) -> Self {
            
                todo!();
                /*


                    : filter (ap), process (p)
                        // setting the size in here crashes PT for some reason, so keep it simple..
                */
            }
            
            pub fn update_size(&mut self)  {
                
                todo!();
                /*
                    if (editorComp == nullptr)
                        {
                            editorComp.reset (filter->createEditorIfNeeded());
                            jassert (editorComp != nullptr);
                        }

                        if (editorComp->getWidth() != 0 && editorComp->getHeight() != 0)
                        {
                            Rect oldRect;
                            GetRect (&oldRect);

                            Rect r;
                            r.left = 0;
                            r.top = 0;
                            r.right = editorComp->getWidth();
                            r.bottom = editorComp->getHeight();
                            SetRect (&r);

                            if (oldRect.right != r.right || oldRect.bottom != r.bottom)
                                startTimer (50);
                        }
                */
            }
            
            pub fn timer_callback(&mut self)  {
                
                todo!();
                /*
                    if (! Component::isMouseButtonDownAnywhere())
                        {
                            stopTimer();

                            // Send a token to the host to tell it about the resize
                            SSetProcessWindowResizeToken token (process->fRootNameId, process->fRootNameId);
                            FicSDSDispatchToken (&token);
                        }
                */
            }
            
            pub fn attach_to_window(&mut self, port: GrafPtr)  {
                
                todo!();
                /*
                    if (port != 0)
                        {
                            ALOE_AUTORELEASEPOOL
                            {
                                updateSize();

                               #if ALOE_WINDOWS
                                auto hostWindow = (void*) ASI_GethWnd ((WindowPtr) port);
                               #else
                                auto hostWindow = (void*) GetWindowFromPort (port);
                               #endif
                                wrapper.reset();
                                wrapper.reset (new EditorCompWrapper (hostWindow, editorComp.get(), this));
                            }
                        }
                        else
                        {
                            deleteEditorComp();
                        }
                */
            }
            
            pub fn draw_contents(&mut self, _0: *mut Rect)  {
                
                todo!();
                /*
                    #if ALOE_WINDOWS
                        if (wrapper != nullptr)
                            if (auto peer = wrapper->getPeer())
                                peer->repaint (wrapper->getLocalBounds());  // (seems to be required in PT6.4, but not in 7.x)
                       #endif
                */
            }
            
            pub fn draw_background(&mut self, _0: *mut Rect)  {
                
                todo!();
                /*
                
                */
            }
            
            pub fn delete_editor_comp(&mut self)  {
                
                todo!();
                /*
                    if (editorComp != nullptr || wrapper != nullptr)
                        {
                            ALOE_AUTORELEASEPOOL
                            {
                                PopupMenu::dismissAllActiveMenus();

                                if (Component* const modalComponent = Component::getCurrentlyModalComponent())
                                    modalComponent->exitModalState (0);

                                filter->editorBeingDeleted (editorComp.get());

                                editorComp.reset();
                                wrapper.reset();
                            }
                        }
                */
            }
        }
    }
        
    impl Drop for AloePlugInProcess {
        fn drop(&mut self) {
            todo!();
            /* 
                ALOE_AUTORELEASEPOOL
                {
                    if (mLoggedIn)
                        MIDILogOut();

                    midiBufferNode.reset();
                    midiTransport.reset();

                    if (aloeFilter != nullptr)
                    {
                        aloeFilter->releaseResources();
                        aloeFilter.reset();
                    }

                    if (--numInstances == 0)
                    {
                       #if ALOE_MAC
                        // Hack to allow any NSWindows to clear themselves up before returning to PT..
                        for (int i = 20; --i >= 0;)
                            MessageManager::getInstance()->runDispatchLoopUntil (1);
                       #endif

                        shutdownAloe_GUI();
                    }
                }
             */
        }
    }

    impl AloePlugInProcess {

        pub fn get_view(&self) -> *mut AloeCustomUIView {
            
            todo!();
            /*
                return dynamic_cast<AloeCustomUIView*> (fOurPlugInView);
            */
        }
        
        pub fn get_view_rect(&mut self, size: *mut Rect)  {
            
            todo!();
            /*
                if (AloeCustomUIView* const v = getView())
                    v->updateSize();

                CEffectProcessRTAS::GetViewRect (size);
            */
        }
        
        pub fn create_cplug_in_view(&mut self) -> *mut CPlugInView {
            
            todo!();
            /*
                return new AloeCustomUIView (aloeFilter.get(), this);
            */
        }
        
        pub fn set_view_port(&mut self, port: GrafPtr)  {
            
            todo!();
            /*
                CEffectProcessRTAS::SetViewPort (port);

                if (AloeCustomUIView* const v = getView())
                    v->attachToWindow (port);
            */
        }
        
        pub fn get_delay_samples_long(&mut self, a_num_samples: *mut i64) -> ComponentResult {
            
            todo!();
            /*
                if (aNumSamples != nullptr)
                    *aNumSamples = aloeFilter != nullptr ? aloeFilter->getLatencySamples() : 0;

                return noErr;
            */
        }
        
        pub fn effect_init(&mut self)  {
            
            todo!();
            /*
                sampleRate = (double) GetSampleRate();
                jassert (sampleRate > 0);
                const int maxBlockSize = (int) CEffectProcessRTAS::GetMaximumRTASQuantum();
                jassert (maxBlockSize > 0);

                SFicPlugInStemFormats stems;
                GetProcessType()->GetStemFormats (&stems);

                aloeFilter->setPlayConfigDetails (fNumInputs, fNumOutputs, sampleRate, maxBlockSize);

                AddControl (new CPluginControl_OnOff ('bypa', "Master Bypass\nMastrByp\nMByp\nByp", false, true));
                DefineMasterBypassControlIndex (bypassControlIndex);

                const int numParameters = aloeFilter->getNumParameters();

               #if ALOE_FORCE_USE_LEGACY_PARAM_IDS
                const bool usingManagedParameters = false;
               #else
                const bool usingManagedParameters = (aloeFilter->getParameters().size() == numParameters);
               #endif

                for (int i = 0; i < numParameters; ++i)
                {
                    OSType rtasParamID = static_cast<OSType> (usingManagedParameters ? aloeFilter->getParameterID (i).hashCode() : i);
                    AddControl (new AloePluginControl (*aloeFilter, i, rtasParamID));
                }

                // we need to do this midi log-in to get timecode, regardless of whether
                // the plugin actually uses midi...
                if (MIDILogIn() == noErr)
                {
                   #if AloePlugin_WantsMidiInput
                    if (CEffectType* const type = dynamic_cast<CEffectType*> (this->GetProcessType()))
                    {
                        char nodeName[80] = { 0 };
                        type->GetProcessTypeName (63, nodeName);
                        nodeName[nodeName[0] + 1] = 0;

                        midiBufferNode.reset (new CEffectMIDIOtherBufferedNode (&mMIDIWorld,
                                                                                8192,
                                                                                eLocalNode,
                                                                                nodeName + 1,
                                                                                midiBuffer));

                        midiBufferNode->Initialize (0xffff, true);
                    }
                   #endif
                }

                midiTransport.reset (new CEffectMIDITransport (&mMIDIWorld));
                midiEvents.ensureSize (2048);

                channels.calloc (jmax (aloeFilter->getTotalNumInputChannels(),
                                       aloeFilter->getTotalNumOutputChannels()));

                aloeFilter->setPlayHead (this);
                aloeFilter->addListener (this);

                aloeFilter->prepareToPlay (sampleRate, maxBlockSize);
            */
        }
        
        pub fn render_audio(&mut self, 
            inputs:      *mut *mut f32,
            outputs:     *mut *mut f32,
            num_samples: i64)  {
            
            todo!();
            /*
                #if AloePlugin_WantsMidiInput
                midiEvents.clear();

                const Cmn_UInt32 bufferSize = mRTGlobals->mHWBufferSizeInSamples;

                if (midiBufferNode != nullptr)
                {
                    if (midiBufferNode->GetAdvanceScheduleTime() != bufferSize)
                        midiBufferNode->SetAdvanceScheduleTime (bufferSize);

                    if (midiBufferNode->FillMIDIBuffer (mRTGlobals->mRunningTime, numSamples) == noErr)
                    {
                        jassert (midiBufferNode->GetBufferPtr() != nullptr);
                        const int numMidiEvents = midiBufferNode->GetBufferSize();

                        for (int i = 0; i < numMidiEvents; ++i)
                        {
                            const DirectMidiPacket& m = midiBuffer[i];

                            jassert ((int) m.mTimestamp < numSamples);

                            midiEvents.addEvent (m.mData, m.mLength,
                                                 jlimit (0, (int) numSamples - 1, (int) m.mTimestamp));
                        }
                    }
                }
               #endif

               #if ALOE_DEBUG || ALOE_LOG_ASSERTIONS
                const int numMidiEventsComingIn = midiEvents.getNumEvents();
                ignoreUnused (numMidiEventsComingIn);
               #endif

                {
                    const ScopedLock sl (aloeFilter->getCallbackLock());

                    const int numIn  = aloeFilter->getTotalNumInputChannels();
                    const int numOut = aloeFilter->getTotalNumOutputChannels();
                    const int totalChans = jmax (numIn, numOut);

                    if (aloeFilter->isSuspended())
                    {
                        for (int i = 0; i < numOut; ++i)
                            FloatVectorOperations::clear (outputs [i], numSamples);
                    }
                    else
                    {
                        {
                            int i;
                            for (i = 0; i < numOut; ++i)
                            {
                                channels[i] = outputs [i];

                                if (i < numIn && inputs != outputs)
                                    FloatVectorOperations::copy (outputs [i], inputs[i], numSamples);
                            }

                            for (; i < numIn; ++i)
                                channels [i] = inputs [i];
                        }

                        AudioBuffer<float> chans (channels, totalChans, numSamples);

                        if (mBypassed)
                            aloeFilter->processBlockBypassed (chans, midiEvents);
                        else
                            aloeFilter->processBlock (chans, midiEvents);
                    }
                }

                if (! midiEvents.isEmpty())
                {
                   #if AloePlugin_ProducesMidiOutput
                    for (const auto metadata : midiEvents)
                    {
                        //jassert (metadata.samplePosition >= 0 && metadata.samplePosition < (int) numSamples);
                    }
                   #elif ALOE_DEBUG || ALOE_LOG_ASSERTIONS
                    // if your plugin creates midi messages, you'll need to set
                    // the AloePlugin_ProducesMidiOutput macro to 1 in your
                    // AloePluginCharacteristics.h file
                    jassert (midiEvents.getNumEvents() <= numMidiEventsComingIn);
                   #endif

                    midiEvents.clear();
                }
            */
        }
        
        pub fn get_chunk_size(&mut self, 
            chunkid: OSType,
            size:    *mut i64) -> ComponentResult {
            
            todo!();
            /*
                if (chunkID == aloeChunkType)
                {
                    tempFilterData.reset();
                    aloeFilter->getStateInformation (tempFilterData);

                    *size = sizeof (SFicPlugInChunkHeader) + tempFilterData.getSize();
                    return noErr;
                }

                return CEffectProcessMIDI::GetChunkSize (chunkID, size);
            */
        }
        
        pub fn get_chunk(&mut self, 
            chunkid: OSType,
            chunk:   *mut SFicPlugInChunk) -> ComponentResult {
            
            todo!();
            /*
                if (chunkID == aloeChunkType)
                {
                    if (tempFilterData.getSize() == 0)
                        aloeFilter->getStateInformation (tempFilterData);

                    chunk->fSize = sizeof (SFicPlugInChunkHeader) + tempFilterData.getSize();
                    tempFilterData.copyTo ((void*) chunk->fData, 0, tempFilterData.getSize());

                    tempFilterData.reset();

                    return noErr;
                }

                return CEffectProcessMIDI::GetChunk (chunkID, chunk);
            */
        }
        
        pub fn set_chunk(&mut self, 
            chunkid: OSType,
            chunk:   *mut SFicPlugInChunk) -> ComponentResult {
            
            todo!();
            /*
                if (chunkID == aloeChunkType)
                {
                    tempFilterData.reset();

                    if (chunk->fSize - sizeof (SFicPlugInChunkHeader) > 0)
                    {
                        aloeFilter->setStateInformation ((void*) chunk->fData,
                                                         chunk->fSize - sizeof (SFicPlugInChunkHeader));
                    }

                    return noErr;
                }

                return CEffectProcessMIDI::SetChunk (chunkID, chunk);
            */
        }
        
        pub fn update_control_value(&mut self, 
            control_index: i64,
            value:         i64) -> ComponentResult {
            
            todo!();
            /*
                if (controlIndex != bypassControlIndex)
                {
                    auto paramIndex = controlIndex - 2;
                    auto floatValue = longToFloat (value);

                    if (auto* param = aloeFilter->getParameters()[paramIndex])
                    {
                        param->setValue (floatValue);
                        param->sendValueChangedMessageToListeners (floatValue);
                    }
                    else
                    {
                        aloeFilter->setParameter (paramIndex, floatValue);
                    }
                }
                else
                {
                    mBypassed = (value > 0);
                }

                return CProcess::UpdateControlValue (controlIndex, value);
            */
        }

        #[cfg(target_os="windows")]
        pub fn handle_keystroke(&mut self, e: *mut EventRecord) -> bool {
            
            todo!();
            /*
                if (Component* modalComp = Component::getCurrentlyModalComponent())
                {
                    if (Component* focused = modalComp->getCurrentlyFocusedComponent())
                    {
                        switch (e->message & charCodeMask)
                        {
                            case kReturnCharCode:
                            case kEnterCharCode:    focused->keyPressed (KeyPress (KeyPress::returnKey)); break;
                            case kEscapeCharCode:   focused->keyPressed (KeyPress (KeyPress::escapeKey)); break;
                            default: break;
                        }

                        return true;
                    }
                }

                return false;
            */
        }
        
        pub fn get_current_position(&mut self, info: &mut AudioPlayHeadCurrentPositionInfo) -> bool {
            
            todo!();
            /*
                Cmn_Float64 bpm = 120.0;
                Cmn_Int32 num = 4, denom = 4;
                Cmn_Int64 ticks = 0;
                Cmn_Bool isPlaying = false;

                if (midiTransport != nullptr)
                {
                    midiTransport->GetCurrentTempo (&bpm);
                    midiTransport->IsTransportPlaying (&isPlaying);
                    midiTransport->GetCurrentMeter (&num, &denom);

                    // (The following is a work-around because GetCurrentTickPosition() doesn't work correctly).
                    Cmn_Int64 sampleLocation;

                    if (isPlaying)
                        midiTransport->GetCurrentRTASSampleLocation (&sampleLocation);
                    else
                        midiTransport->GetCurrentTDMSampleLocation (&sampleLocation);

                    midiTransport->GetCustomTickPosition (&ticks, sampleLocation);

                    info.timeInSamples = (int64) sampleLocation;
                    info.timeInSeconds = sampleLocation / sampleRate;
                }
                else
                {
                    info.timeInSamples = 0;
                    info.timeInSeconds = 0;
                }

                info.bpm = bpm;
                info.timeSigNumerator = num;
                info.timeSigDenominator = denom;
                info.isPlaying = isPlaying;
                info.isRecording = false;
                info.ppqPosition = ticks / 960000.0;
                info.ppqPositionOfLastBarStart = 0; //xxx no idea how to get this correctly..
                info.isLooping = false;
                info.ppqLoopStart = 0;
                info.ppqLoopEnd = 0;

                double framesPerSec = 24.0;

                switch (fTimeCodeInfo.mFrameRate)
                {
                    case ficFrameRate_24Frame:       info.frameRate = AudioPlayHead::fps24;       break;
                    case ficFrameRate_25Frame:       info.frameRate = AudioPlayHead::fps25;       framesPerSec = 25.0; break;
                    case ficFrameRate_2997NonDrop:   info.frameRate = AudioPlayHead::fps2997;     framesPerSec = 30.0 * 1000.0 / 1001.0; break;
                    case ficFrameRate_2997DropFrame: info.frameRate = AudioPlayHead::fps2997drop; framesPerSec = 30.0 * 1000.0 / 1001.0; break;
                    case ficFrameRate_30NonDrop:     info.frameRate = AudioPlayHead::fps30;       framesPerSec = 30.0; break;
                    case ficFrameRate_30DropFrame:   info.frameRate = AudioPlayHead::fps30drop;   framesPerSec = 30.0; break;
                    case ficFrameRate_23976:         info.frameRate = AudioPlayHead::fps23976;    framesPerSec = 24.0 * 1000.0 / 1001.0; break;
                    default:                         info.frameRate = AudioPlayHead::fpsUnknown;  break;
                }

                info.editOriginTime = fTimeCodeInfo.mFrameOffset / framesPerSec;

                return true;
            */
        }
        
        pub fn audio_processor_parameter_changed(&mut self, 
            _0:        *mut AudioProcessor,
            index:     i32,
            new_value: f32)  {
            
            todo!();
            /*
                SetControlValue (index + 2, floatToLong (newValue));
            */
        }
        
        pub fn audio_processor_parameter_change_gesture_begin(&mut self, 
            _0:    *mut AudioProcessor,
            index: i32)  {
            
            todo!();
            /*
                TouchControl (index + 2);
            */
        }
        
        pub fn audio_processor_parameter_change_gesture_end(&mut self, 
            _0:    *mut AudioProcessor,
            index: i32)  {
            
            todo!();
            /*
                ReleaseControl (index + 2);
            */
        }
        
        pub fn audio_processor_changed(&mut self, 
            _0: *mut AudioProcessor,
            _1: &ChangeDetails)  {
            
            todo!();
            /*
                // xxx is there an RTAS equivalent?
            */
        }
        
        pub fn long_to_float(n: i64) -> f32 {
            
            todo!();
            /*
                return (float) ((((double) n) + (double) 0x80000000) / (double) 0xffffffff);
            */
        }
        
        pub fn float_to_long(n: f32) -> i64 {
            
            todo!();
            /*
                return roundToInt (jlimit (-(double) 0x80000000, (double) 0x7fffffff,
                                           n * (double) 0xffffffff - (double) 0x80000000));
            */
        }
        
        pub fn bypass_buffers(&self, 
            inputs:      *mut *mut f32,
            outputs:     *mut *mut f32,
            num_samples: i64)  {
            
            todo!();
            /*
                for (int i = fNumOutputs; --i >= 0;)
                {
                    if (i < fNumInputs)
                        FloatVectorOperations::copy (outputs[i], inputs[i], numSamples);
                    else
                        FloatVectorOperations::clear (outputs[i], numSamples);
                }
            */
        }
    }

    ///----------------------
    pub struct AloePlugInGroup {
        base: CEffectGroupMIDI,
    }

    impl Default for AloePlugInGroup {
        
        fn default() -> Self {
            todo!();
            /*


                DefineManufacturerNamesAndID (AloePlugin_Manufacturer, AloePlugin_RTASManufacturerCode);
                DefinePlugInNamesAndVersion (createRTASName().toUTF8(), AloePlugin_VersionCode);

               #if ALOE_RTAS_PLUGINGESTALT_IS_CACHEABLE
                AddGestalt (pluginGestalt_IsCacheable);
               #endi
            */
        }
    }

    impl Drop for AloePlugInGroup {
        fn drop(&mut self) {
            todo!();
            /* 
                shutdownAloe_GUI();
             */
        }
    }

    impl AloePlugInGroup {

        pub fn rtas_channel_set(num_channels: i32) -> AudioChannelSet {
            
            todo!();
            /*
                if (numChannels == 0) return AudioChannelSet::disabled();
                if (numChannels == 1) return AudioChannelSet::mono();
                if (numChannels == 2) return AudioChannelSet::stereo();
                if (numChannels == 3) return AudioChannelSet::createLCR();
                if (numChannels == 4) return AudioChannelSet::quadraphonic();
                if (numChannels == 5) return AudioChannelSet::create5point0();
                if (numChannels == 6) return AudioChannelSet::create5point1();

                #if PT_VERS_MAJOR >= 9
                if (numChannels == 7) return AudioChannelSet::create7point0();
                if (numChannels == 8) return AudioChannelSet::create7point1();
                #else
                if (numChannels == 7) return AudioChannelSet::create7point0SDDS();
                if (numChannels == 8) return AudioChannelSet::create7point1SDDS();
                #endif

                jassertfalse;

                return AudioChannelSet::discreteChannels (numChannels);
            */
        }
        
        pub fn create_effect_types(&mut self)  {
            
            todo!();
            /*
                std::unique_ptr<AudioProcessor> plugin (createPluginFilterOfType (AudioProcessor::wrapperType_RTAS));

               #ifndef AloePlugin_PreferredChannelConfigurations
                #error You need to set the "Plugin Channel Configurations" field in the Proaloer to build RTAS plug-ins
               #endif

                const short channelConfigs[][2] = { AloePlugin_PreferredChannelConfigurations };
                const int numConfigs = numElementsInArray (channelConfigs);

                // You need to actually add some configurations to the AloePlugin_PreferredChannelConfigurations
                // value in your AloePluginCharacteristics.h file..
                jassert (numConfigs > 0);

                for (int i = 0; i < numConfigs; ++i)
                {
                    if (channelConfigs[i][0] <= 8 && channelConfigs[i][1] <= 8)
                    {
                        const AudioChannelSet inputLayout  (rtasChannelSet (channelConfigs[i][0]));
                        const AudioChannelSet outputLayout (rtasChannelSet (channelConfigs[i][1]));

                        const int32 pluginId = plugin->getAAXPluginIDForMainBusConfig (inputLayout, outputLayout, false);

                        CEffectType* const type
                            = new CEffectTypeRTAS (pluginId,
                                                   AloePlugin_RTASProductId,
                                                   AloePlugin_RTASCategory);

                        type->DefineTypeNames (createRTASName().toRawUTF8());
                        type->DefineSampleRateSupport (eSupports48kAnd96kAnd192k);

                        type->DefineStemFormats (getFormatForChans (channelConfigs [i][0] != 0 ? channelConfigs [i][0] : channelConfigs [i][1]),
                                                 getFormatForChans (channelConfigs [i][1] != 0 ? channelConfigs [i][1] : channelConfigs [i][0]));

                       #if ! AloePlugin_RTASDisableBypass
                        type->AddGestalt (pluginGestalt_CanBypass);
                       #endif

                       #if AloePlugin_RTASDisableMultiMono
                        type->AddGestalt (pluginGestalt_DoesntSupportMultiMono);
                       #endif

                        type->AddGestalt (pluginGestalt_SupportsVariableQuanta);
                        type->AttachEffectProcessCreator (createNewProcess);

                        AddEffectType (type);
                    }
                }
            */
        }
        
        pub fn initialize(&mut self)  {
            
            todo!();
            /*
                CEffectGroupMIDI::Initialize();
            */
        }
        
        pub fn create_new_process() -> *mut CEffectProcess {
            
            todo!();
            /*
                #if ALOE_WINDOWS
                Process::setCurrentModuleInstanceHandle (gThisModule);
               #endif
                PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_RTAS;
                initialiseAloe_GUI();

                return new AloePlugInProcess();
            */
        }
        
        pub fn create_rtas_name() -> String {
            
            todo!();
            /*
                return String (AloePlugin_Name) + "\n"
                         + String (AloePlugin_Desc);
            */
        }
        
        pub fn get_format_for_chans(num_chans: i32) -> EPlugIn_StemFormat {
            
            todo!();
            /*
                switch (numChans)
                {
                    case 0:   return ePlugIn_StemFormat_Generic;
                    case 1:   return ePlugIn_StemFormat_Mono;
                    case 2:   return ePlugIn_StemFormat_Stereo;
                    case 3:   return ePlugIn_StemFormat_LCR;
                    case 4:   return ePlugIn_StemFormat_Quad;
                    case 5:   return ePlugIn_StemFormat_5dot0;
                    case 6:   return ePlugIn_StemFormat_5dot1;

                   #if PT_VERS_MAJOR >= 9
                    case 7:   return ePlugIn_StemFormat_7dot0DTS;
                    case 8:   return ePlugIn_StemFormat_7dot1DTS;
                   #else
                    case 7:   return ePlugIn_StemFormat_7dot0;
                    case 8:   return ePlugIn_StemFormat_7dot1;
                   #endif

                    default:  jassertfalse; break; // hmm - not a valid number of chans for RTAS..
                }

                return ePlugIn_StemFormat_Generic;
            */
        }
    }

    pub fn initialise_macrtas()  {
        
        todo!();
        /*
        
        */
    }

    impl CProcessGroup {
        
        pub fn create_process_group(&mut self) -> *mut CProcessGroupInterface {
            
            todo!();
            /*
                #if ALOE_MAC
            initialiseMacRTAS();
           #endif

            return new AloePlugInGroup();
            */
        }
    }
}
