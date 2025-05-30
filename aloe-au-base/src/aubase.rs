crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUBase.h]

pub struct AUBase {
    base:                     ComponentBase,
    elements_created:         bool,
    initialized:              bool,
    has_begun_initializing:   bool,
    audio_unit_api_version:   UInt8,
    init_num_input_els:       u32,
    init_num_output_els:      u32,

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    init_num_group_els:       u32,

    scopes:                   [AUScope; AU_BASE_K_NUM_SCOPES],
    render_callbacks:         RenderCallbackList,
    render_callbacks_touched: bool,

    #[cfg(TARGET_OS_MAC)]
    render_threadid:          PThread,

    #[cfg(TARGET_OS_WIN32)]
    render_threadid:          u32,

    wants_render_threadid:    bool,
    current_render_time:      AudioTimeStamp,
    max_frames_per_slice:     u32,
    last_render_error:        OSStatus,
    current_preset:           AUPreset,
    uses_fixed_block_size:    bool,
    param_list:               ParameterEventList,
    property_listeners:       AUBasePropertyListeners,
    buffers_allocated:        bool,

    /**
       if this is NOT null, it will contain
       identifying info about this AU.
      */
    log_string:               *mut u8,

    nick_name:                CFStringRef,
    au_mutex:                 *mut CAMutex,

    #[cfg(not(CA_NO_AU_HOST_CALLBACKS))]
    host_callback_info:       HostCallbackInfo,

    #[cfg(not(CA_NO_AU_UI_FEATURES))]
    context_name:             CFStringRef,
}

impl Drop for AUBase {

    fn drop(&mut self) {
        todo!();
        /*
            if (mCurrentPreset.presetName) CFRelease (mCurrentPreset.presetName);
    #if !CA_NO_AU_UI_FEATURES
        if (mContextName) CFRelease (mContextName);
    #endif
        if (mLogString) delete [] mLogString;
        if (mNickName) CFRelease(mNickName);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUBase.cpp]

impl AUBase {
    
    pub fn is_initialized(&self) -> bool {
        
        todo!();
        /*
            return mInitialized;
        */
    }
    
    pub fn has_begun_initializing(&self) -> bool {
        
        todo!();
        /*
            return mHasBegunInitializing;
        */
    }

    /* ---------- Methods useful for subclasses  ---------- */

    pub fn get_scope(&mut self, in_scope: AudioUnitScope) -> &mut AUScope {
        
        todo!();
        /*
            if (inScope >= kNumScopes) {
                AUScope * scope = GetScopeExtended(inScope);
                if (!scope) COMPONENT_THROW(kAudioUnitErr_InvalidScope);
                return *scope;
            }
            return mScopes[inScope];
        */
    }
    
    pub fn global_scope(&mut self) -> &mut AUScope {
        
        todo!();
        /*
            return mScopes[kAudioUnitScope_Global];
        */
    }
    
    pub fn inputs(&mut self) -> &mut AUScope {
        
        todo!();
        /*
            return mScopes[kAudioUnitScope_Input];
        */
    }
    
    pub fn outputs(&mut self) -> &mut AUScope {
        
        todo!();
        /*
            return mScopes[kAudioUnitScope_Output];
        */
    }

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    pub fn groups(&mut self) -> &mut AUScope {
        
        todo!();
        /*
            return mScopes[kAudioUnitScope_Group];
        */
    }
    
    pub fn globals(&mut self) -> *mut AUElement {
        
        todo!();
        /*
            return mScopes[kAudioUnitScope_Global].GetElement(0);
        */
    }
    
    pub fn get_element(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> *mut AUElement {
        
        todo!();
        /*
            return GetScope(inScope).GetElement(inElement);
        */
    }
    
    pub fn get_io_element(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> *mut AUIOElement {
        
        todo!();
        /*
            return GetScope(inScope).GetIOElement(inElement);
        */
    }
    
    pub fn safe_get_element(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> *mut AUElement {
        
        todo!();
        /*
            return GetScope(inScope).SafeGetElement(inElement);
        */
    }
    
    pub fn get_input(&mut self, in_element: AudioUnitElement) -> *mut AUInputElement {
        
        todo!();
        /*
            return static_cast<AUInputElement *>(Inputs().SafeGetElement(inElement));
        */
    }
    
    pub fn get_output(&mut self, in_element: AudioUnitElement) -> *mut AUOutputElement {
        
        todo!();
        /*
            return static_cast<AUOutputElement *>(Outputs().SafeGetElement(inElement));
        */
    }

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    pub fn get_group(&mut self, in_element: AudioUnitElement) -> *mut AUElement {
        
        todo!();
        /*
            return Groups().SafeGetElement(inElement);
        */
    }
    
    pub fn pull_input(&mut self, 
        in_bus_number:    u32,
        io_action_flags:  &mut AudioUnitRenderActionFlags,
        in_time_stamp:    &AudioTimeStamp,
        in_number_frames: u32) -> OSStatus {
        
        todo!();
        /*
            AUInputElement *input = GetInput(inBusNumber);  // throws if error
            return input->PullInput(ioActionFlags, inTimeStamp, inBusNumber, inNumberFrames);
        */
    }
    
    pub fn get_max_frames_per_slice(&self) -> u32 {
        
        todo!();
        /*
            return mMaxFramesPerSlice;
        */
    }
    
    pub fn uses_fixed_block_size(&self) -> bool {
        
        todo!();
        /*
            return mUsesFixedBlockSize;
        */
    }
    
    pub fn set_uses_fixed_block_size(&mut self, in_uses_fixed_block_size: bool)  {
        
        todo!();
        /*
            mUsesFixedBlockSize = inUsesFixedBlockSize;
        */
    }
    
    pub fn get_vector_unit_type() -> i32 {
        
        todo!();
        /*
            return sVectorUnitType;
        */
    }
    
    pub fn has_vector_unit() -> bool {
        
        todo!();
        /*
            return sVectorUnitType > 0;
        */
    }
    
    pub fn has_altivec() -> bool {
        
        todo!();
        /*
            return sVectorUnitType == kVecAltivec;
        */
    }
    
    pub fn hassse2() -> bool {
        
        todo!();
        /*
            return sVectorUnitType >= kVecSSE2;
        */
    }
    
    pub fn hassse3() -> bool {
        
        todo!();
        /*
            return sVectorUnitType >= kVecSSE3;
        */
    }
    
    pub fn audio_unit_api_version(&self) -> UInt8 {
        
        todo!();
        /*
            return mAudioUnitAPIVersion;
        */
    }
    
    pub fn in_render_thread(&self) -> bool {
        
        todo!();
        /*
           #if TARGET_OS_MAC
           return (mRenderThreadID ? pthread_equal (mRenderThreadID, pthread_self()) : false);
           #elif TARGET_OS_WIN32
           return (mRenderThreadID ? mRenderThreadID == GetCurrentThreadId() : false);
           #endif
           */
    }

    /**
       says whether an input is connected or has
       a callback
      */
    pub fn has_input(&mut self, in_element: AudioUnitElement) -> bool {
        
        todo!();
        /*
            AUInputElement *in = static_cast<AUInputElement *>(Inputs().GetElement(inElement));
                                        return in != NULL && in->IsActive();
        */
    }

    /*
      | These calls can be used to call a Host's
      | Callbacks. The method returns -1 if the
      | host hasn't supplied the callback. Any
      | other result is returned by the host.
      |
      | As in the API contract, for a parameter's
      | value, you specify a pointer to that data
      | type. Specify NULL for a parameter that you
      | are not interested as this can save work in
      | the host.
      */

    #[cfg(not(CA_NO_AU_UI_FEATURES))]
    pub fn call_host_beat_and_tempo(&mut self, 
        out_current_beat:  *mut f64,
        out_current_tempo: *mut f64) -> OSStatus {
        
        todo!();
        /*
            return (mHostCallbackInfo.beatAndTempoProc
                            ? (*mHostCallbackInfo.beatAndTempoProc) (mHostCallbackInfo.hostUserData,
                                                                        outCurrentBeat,
                                                                        outCurrentTempo)
                            : -1);
        */
    }
    
    #[cfg(not(CA_NO_AU_UI_FEATURES))]
    pub fn call_host_musical_time_location(&mut self, 
        out_delta_sample_offset_to_next_beat: *mut u32,
        out_time_sig_numerator:               *mut Float32,
        out_time_sig_denominator:             *mut u32,
        out_current_measure_down_beat:        *mut f64) -> OSStatus {
        
        todo!();
        /*
            return (mHostCallbackInfo.musicalTimeLocationProc
                            ? (*mHostCallbackInfo.musicalTimeLocationProc) (mHostCallbackInfo.hostUserData,
                                                                                outDeltaSampleOffsetToNextBeat,
                                                                                outTimeSig_Numerator,
                                                                                outTimeSig_Denominator,
                                                                                outCurrentMeasureDownBeat)
                            : -1);
        */
    }
    
    #[cfg(not(CA_NO_AU_UI_FEATURES))]
    pub fn call_host_transport_state(&mut self, 
        out_is_playing:                  *mut bool,
        out_transport_state_changed:     *mut bool,
        out_current_sample_in_time_line: *mut f64,
        out_is_cycling:                  *mut bool,
        out_cycle_start_beat:            *mut f64,
        out_cycle_end_beat:              *mut f64) -> OSStatus {
        
        todo!();
        /*
            return (mHostCallbackInfo.transportStateProc
                            ? (*mHostCallbackInfo.transportStateProc) (mHostCallbackInfo.hostUserData,
                                                                            outIsPlaying,
                                                                            outTransportStateChanged,
                                                                            outCurrentSampleInTimeLine,
                                                                            outIsCycling,
                                                                            outCycleStartBeat,
                                                                            outCycleEndBeat)
                            : -1);
        */
    }
    
    pub fn get_mutex(&mut self) -> *mut CAMutex {
        
        todo!();
        /*
            return mAUMutex;
        */
    }
    
    pub fn fill_in_parameter_name(
        io_info:           &mut AudioUnitParameterInfo,
        in_name:           CFStringRef,
        in_should_release: bool)  {
        
        todo!();
        /*
            ioInfo.cfNameString = inName;
            ioInfo.flags |= kAudioUnitParameterFlag_HasCFNameString;
            if (inShouldRelease)
                ioInfo.flags |= kAudioUnitParameterFlag_CFNameRelease;
            CFStringGetCString (inName, ioInfo.name, offsetof (AudioUnitParameterInfo, clumpID), kCFStringEncodingUTF8);
        */
    }
    
    pub fn has_clump(
        io_info:    &mut AudioUnitParameterInfo,
        in_clumpid: u32)  {
        
        todo!();
        /*
            ioInfo.clumpID = inClumpID;
            ioInfo.flags |= kAudioUnitParameterFlag_HasClump;
        */
    }
    
    pub fn wants_render_threadid(&self) -> bool {
        
        todo!();
        /*
            return mWantsRenderThreadID;
        */
    }
    
    pub fn set_render_error(&mut self, in_err: OSStatus) -> OSStatus {
        
        todo!();
        /*
            if (inErr && mLastRenderError == 0) {
                mLastRenderError = inErr;
                PropertyChanged(kAudioUnitProperty_LastRenderError, kAudioUnitScope_Global, 0);
            }
            return inErr;
        */
    }

    /**
      shared between Render and RenderSlice,
      inlined to minimize function call overhead
      */
    pub fn do_render_bus(&mut self, 
        io_action_flags:  &mut AudioUnitRenderActionFlags,
        in_time_stamp:    &AudioTimeStamp,
        in_bus_number:    u32,
        the_output:       *mut AUOutputElement,
        in_number_frames: u32,
        io_data:          &mut AudioBufferList) -> OSStatus {
        
        todo!();
        /*
            if (theOutput != NULL)
            {
                if (ioData.mBuffers[0].mData == NULL || (theOutput->WillAllocateBuffer() && Outputs().GetNumberOfElements() > 1))
                    // will render into cache buffer
                    theOutput->PrepareBuffer(inNumberFrames);
                else
                    // will render into caller's buffer
                    theOutput->SetBufferList(ioData);
            }

            OSStatus result = RenderBus(ioActionFlags, inTimeStamp, inBusNumber, inNumberFrames);
            if (result == noErr && theOutput != NULL) {
                if (ioData.mBuffers[0].mData == NULL) {
                    theOutput->CopyBufferListTo(ioData);
                    AUTRACE(kCATrace_AUBaseDoRenderBus, mComponentInstance, inNumberFrames, (intptr_t)theOutput->GetBufferList().mBuffers[0].mData, 0, *(UInt32 *)ioData.mBuffers[0].mData);
                } else {
                    theOutput->CopyBufferContentsTo(ioData);
                    AUTRACE(kCATrace_AUBaseDoRenderBus, mComponentInstance, inNumberFrames, (intptr_t)theOutput->GetBufferList().mBuffers[0].mData, (intptr_t)ioData.mBuffers[0].mData, *(UInt32 *)ioData.mBuffers[0].mData);
                    theOutput->InvalidateBufferList();
                }
            }
            return result;
        */
    }
    
    pub fn reset_render_time(&mut self)  {
        
        todo!();
        /*
            memset (&mCurrentRenderTime, 0, sizeof(mCurrentRenderTime));
                                        mCurrentRenderTime.mSampleTime = kNoLastRenderedSampleTime;
        */
    }
    
    pub fn needs_to_render(&mut self, in_time_stamp: &AudioTimeStamp) -> bool {
        
        todo!();
        /*
            bool needsToRender = fnotequal(inTimeStamp.mSampleTime, mCurrentRenderTime.mSampleTime);
                                        if (needsToRender)  // only copy this if we need to render
                                            mCurrentRenderTime = inTimeStamp;
                                        return needsToRender;
        */
    }
    
    pub fn current_render_time(&self) -> &AudioTimeStamp {
        
        todo!();
        /*
            return mCurrentRenderTime;
        */
    }
    
    pub fn new(
        in_instance:         AudioComponentInstance,
        num_input_elements:  u32,
        num_output_elements: u32,
        num_group_elements:  Option<u32>

    ) -> Self {

        let num_group_elements: u32 =
                 num_group_elements.unwrap_or(0);
    
        todo!();
        /*


            :
        ComponentBase(inInstance),
        mElementsCreated(false),
        mInitialized(false),
        mHasBegunInitializing(false),
        mInitNumInputEls(numInputElements), mInitNumOutputEls(numOutputElements),
    #if !CA_BASIC_AU_FEATURES
        mInitNumGroupEls(numGroupElements),
    #endif
        mRenderCallbacksTouched(false),
        mRenderThreadID (NULL),
        mWantsRenderThreadID (false),
        mLastRenderError(0),
        mUsesFixedBlockSize(false),
        mBuffersAllocated(false),
        mLogString (NULL),
        mNickName (NULL),
        mAUMutex(NULL)
        #if !CA_NO_AU_UI_FEATURES
            ,
            mContextName(NULL)
        #endif

        ResetRenderTime ();

        if(!sAUBaseCFStringsInitialized)
        {
            kUntitledString = CFSTR("Untitled");
            kVersionString = CFSTR(kAUPresetVersionKey);
            kTypeString = CFSTR(kAUPresetTypeKey);
            kSubtypeString = CFSTR(kAUPresetSubtypeKey);
            kManufacturerString = CFSTR(kAUPresetManufacturerKey);
            kDataString = CFSTR(kAUPresetDataKey);
            kNameString = CFSTR(kAUPresetNameKey);
            kRenderQualityString = CFSTR(kAUPresetRenderQualityKey);
            kCPULoadString = CFSTR(kAUPresetCPULoadKey);
            kElementNameString = CFSTR(kAUPresetElementNameKey);
            kPartString = CFSTR(kAUPresetPartKey);
            sAUBaseCFStringsInitialized = true;
        }

        if (sVectorUnitType == kVecUninitialized) {
            sVectorUnitType = CAVectorUnit::GetVectorUnitType() ;
        }

        mAudioUnitAPIVersion = 2;

        SetMaxFramesPerSlice(kAUDefaultMaxFramesPerSlice);

        GlobalScope().Initialize(this, kAudioUnitScope_Global, 1);

    #if !CA_NO_AU_UI_FEATURES
        memset (&mHostCallbackInfo, 0, sizeof (mHostCallbackInfo));
    #endif

        mCurrentPreset.presetNumber = -1;
        mCurrentPreset.presetName = kUntitledString;
        CFRetain (mCurrentPreset.presetName);
        */
    }
    
    /**
      | Called immediately after construction, when
      | virtual methods work.
      |
      | Or, a subclass may call this in order to
      | have access to elements in its constructor.
      */
    pub fn create_elements(&mut self)  {
        
        todo!();
        /*
            if (!mElementsCreated) {
            Inputs().Initialize(this, kAudioUnitScope_Input, mInitNumInputEls);
            Outputs().Initialize(this, kAudioUnitScope_Output, mInitNumOutputEls);
    #if !CA_BASIC_AU_FEATURES
            Groups().Initialize(this, kAudioUnitScope_Group, mInitNumGroupEls);
    #endif
            CreateExtendedElements();

            mElementsCreated = true;
        }
        */
    }
    
    pub fn set_max_frames_per_slice(&mut self, n_frames: u32)  {
        
        todo!();
        /*
            mMaxFramesPerSlice = nFrames;
        if (mBuffersAllocated)
            ReallocateBuffers();
        PropertyChanged(kAudioUnitProperty_MaximumFramesPerSlice, kAudioUnitScope_Global, 0);
        */
    }
    
    pub fn can_set_max_frames(&self) -> OSStatus {
        
        todo!();
        /*
            return IsInitialized() ? kAudioUnitErr_Initialized : OSStatus(noErr);
        */
    }
    
    pub fn reallocate_buffers(&mut self)  {
        
        todo!();
        /*
            CreateElements();

        UInt32 nOutputs = Outputs().GetNumberOfElements();
        for (UInt32 i = 0; i < nOutputs; ++i) {
            AUOutputElement *output = GetOutput(i);
            output->AllocateBuffer();   // does no work if already allocated
        }
        UInt32 nInputs = Inputs().GetNumberOfElements();
        for (UInt32 i = 0; i < nInputs; ++i) {
            AUInputElement *input = GetInput(i);
            input->AllocateBuffer();    // does no work if already allocated
        }
        mBuffersAllocated = true;
        */
    }
    
    pub fn deallocate_io_buffers(&mut self)  {
        
        todo!();
        /*
            if (!mBuffersAllocated)
            return;

        UInt32 nOutputs = Outputs().GetNumberOfElements();
        for (UInt32 i = 0; i < nOutputs; ++i) {
            AUOutputElement *output = GetOutput(i);
            output->DeallocateBuffer();
        }
        UInt32 nInputs = Inputs().GetNumberOfElements();
        for (UInt32 i = 0; i < nInputs; ++i) {
            AUInputElement *input = GetInput(i);
            input->DeallocateBuffer();
        }
        mBuffersAllocated = false;
        */
    }
    
    /**
      | this implements the entry point and
      | makes sure that initialization is only
      | attempted exactly once...
      |
      */
    pub fn do_initialize(&mut self) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;

        if (!mInitialized) {
            result = Initialize();
            if (result == noErr) {
                if (CanScheduleParameters())
                    mParamList.reserve(24);
                mHasBegunInitializing = true;
                ReallocateBuffers();    // calls CreateElements()
                mInitialized = true;    // signal that it's okay to render
                CAMemoryBarrier();
            }
        }

        return result;
        */
    }
    
    pub fn initialize(&mut self) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
    
    pub fn pre_destructor(&mut self)  {
        
        todo!();
        /*
            // this is called from the ComponentBase dispatcher, which doesn't know anything about our (optional) lock
        CAMutex::Locker lock(mAUMutex);
        DoCleanup();
        */
    }
    
    /**
      | same pattern as with Initialize
      |
      */
    pub fn do_cleanup(&mut self)  {
        
        todo!();
        /*
            if (mInitialized)
            Cleanup();

        DeallocateIOBuffers();
        ResetRenderTime ();

        mInitialized = false;
        mHasBegunInitializing = false;
        */
    }
    
    pub fn cleanup(&mut self)  { }
    
    pub fn reset(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> OSStatus {
        
        todo!();
        /*
            ResetRenderTime ();
        return noErr;
        */
    }
    
    /**
      | Note about GetPropertyInfo, GetProperty,
      | SetProperty:
      |
      | Certain properties are trapped out in these
      | dispatch functions and handled with
      | different virtual methods.  (To discourage
      | hacks and keep vtable size down, these are
      | non-virtual)
      */
    pub fn dispatch_get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;
        bool validateElement = true;

        switch (inID) {
        case kAudioUnitProperty_MakeConnection:
            ca_require(inScope == kAudioUnitScope_Input || inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(AudioUnitConnection);
            outWritable = true;
            break;

        case kAudioUnitProperty_SetRenderCallback:
            ca_require(AudioUnitAPIVersion() > 1, InvalidProperty);
            ca_require(inScope == kAudioUnitScope_Input || inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(AURenderCallbackStruct);
            outWritable = true;
            break;

        case kAudioUnitProperty_StreamFormat:
            outDataSize = sizeof(CAStreamBasicDescription);
            outWritable = IsStreamFormatWritable(inScope, inElement);
            break;

        case kAudioUnitProperty_SampleRate:
            outDataSize = sizeof(Float64);
            outWritable = IsStreamFormatWritable(inScope, inElement);
            break;

        case kAudioUnitProperty_ClassInfo:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(CFPropertyListRef);
            outWritable = true;
            break;

        case kAudioUnitProperty_FactoryPresets:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            result = GetPresets(NULL);
            if (!result) {
                outDataSize = sizeof(CFArrayRef);
                outWritable = false;
            }
            break;

        case kAudioUnitProperty_PresentPreset:
    #if !CA_USE_AUDIO_PLUGIN_ONLY
    #ifndef __LP64__
        case kAudioUnitProperty_CurrentPreset:
    #endif
    #endif
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(AUPreset);
            outWritable = true;
            break;

        case kAudioUnitProperty_ElementName:
            outDataSize = sizeof (CFStringRef);
            outWritable = true;
            break;

        case kAudioUnitProperty_ParameterList:
            {
                UInt32 nparams = 0;
                result = GetParameterList(inScope, NULL, nparams);

                outDataSize = sizeof(AudioUnitParameterID) * nparams;
                outWritable = false;
                validateElement = false;
            }
            break;

        case kAudioUnitProperty_ParameterInfo:
            outDataSize = sizeof(AudioUnitParameterInfo);
            outWritable = false;
            validateElement = false;
            break;

        case kAudioUnitProperty_ParameterHistoryInfo:
            outDataSize = sizeof(AudioUnitParameterHistoryInfo);
            outWritable = false;
            validateElement = false;
            break;

        case kAudioUnitProperty_ElementCount:
            outDataSize = sizeof(UInt32);
            outWritable = BusCountWritable(inScope);
            validateElement = false;
            break;

        case kAudioUnitProperty_Latency:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(Float64);
            outWritable = false;
            break;

        case kAudioUnitProperty_TailTime:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            if (SupportsTail()) {
                outDataSize = sizeof(Float64);
                outWritable = false;
            } else
                goto InvalidProperty;
            break;

        case kAudioUnitProperty_MaximumFramesPerSlice:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(UInt32);
            outWritable = true;
            break;

        case kAudioUnitProperty_LastRenderError:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(OSStatus);
            outWritable = false;
            break;

        case kAudioUnitProperty_SupportedNumChannels:
        {
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            UInt32 num = SupportedNumChannels (NULL);
            if (num) {
                outDataSize = sizeof (AUChannelInfo) * num;
                result = noErr;
            } else
                goto InvalidProperty;
            outWritable = false;
            break;
        }

        case kAudioUnitProperty_SupportedChannelLayoutTags:
        {
            UInt32 numLayouts = GetChannelLayoutTags(inScope, inElement, NULL);
            if (numLayouts) {
                outDataSize = numLayouts * sizeof(AudioChannelLayoutTag);
                result = noErr;
            } else
                goto InvalidProperty;
            outWritable = false;
            validateElement = false; //already done it
            break;
        }

        case kAudioUnitProperty_AudioChannelLayout:
        {
            outWritable = false;
            outDataSize = GetAudioChannelLayout(inScope, inElement, NULL, outWritable);
            if (outDataSize) {
                result = noErr;
            } else {
                if (GetChannelLayoutTags(inScope, inElement, NULL) == 0)
                    goto InvalidProperty;
                else
                    result = kAudioUnitErr_InvalidPropertyValue;
            }
            validateElement = false; //already done it
            break;
        }

    #if (MAC_OS_X_VERSION_MIN_REQUIRED > MAC_OS_X_VERSION_10_5) || TARGET_OS_IPHONE
        case kAudioUnitProperty_ShouldAllocateBuffer:
            ca_require((inScope == kAudioUnitScope_Input || inScope == kAudioUnitScope_Output), InvalidScope);
            outWritable = true;
            outDataSize = sizeof(UInt32);
            break;
    #endif

    #if !CA_USE_AUDIO_PLUGIN_ONLY
        case kAudioUnitProperty_FastDispatch:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            if (!IsCMgrObject()) goto InvalidProperty;
            outDataSize = sizeof(void *);
            outWritable = false;
            validateElement = false;
            break;

        case kAudioUnitProperty_GetUIComponentList:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = GetNumCustomUIComponents();
            if (outDataSize == 0)
                goto InvalidProperty;
            outDataSize *= sizeof (AudioComponentDescription);

            outWritable = false;
            break;
    #endif

        case kAudioUnitProperty_ParameterValueStrings:
            result = GetParameterValueStrings(inScope, inElement, NULL);
            if (result == noErr) {
                outDataSize = sizeof(CFArrayRef);
                outWritable = false;
                validateElement = false;
            }
            break;

    #if !CA_NO_AU_HOST_CALLBACKS
        case kAudioUnitProperty_HostCallbacks:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(OldHostCallbackInfo);
            outWritable = true;
            break;
    #endif
    #if !CA_NO_AU_UI_FEATURES
        case kAudioUnitProperty_ContextName:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(CFStringRef);
            outWritable = true;
            break;

        case kAudioUnitProperty_IconLocation:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outWritable = false;
            if (!HasIcon())
                goto InvalidProperty;
            outDataSize = sizeof(CFURLRef);
            break;

        case kAudioUnitProperty_ParameterClumpName:
            outDataSize = sizeof(AudioUnitParameterNameInfo );
            outWritable = false;
            break;

    #endif // !CA_NO_AU_UI_FEATURES

        case 'lrst' :  // kAudioUnitProperty_LastRenderedSampleTime
            outDataSize = sizeof(Float64);
            outWritable = false;
            break;

        case /*kAudioUnitProperty_NickName*/ 54:
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            outDataSize = sizeof(CFStringRef);
            outWritable = true;
            break;

        default:
            result = GetPropertyInfo(inID, inScope, inElement, outDataSize, outWritable);
            validateElement = false;
            break;
        }

        if (result == noErr && validateElement) {
            ca_require(GetElement(inScope, inElement) != NULL, InvalidElement);
        }

        return result;
    InvalidProperty:
        return kAudioUnitErr_InvalidProperty;
    InvalidScope:
        return kAudioUnitErr_InvalidScope;
    InvalidElement:
        return kAudioUnitErr_InvalidElement;
        */
    }
    
    pub fn dispatch_get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
            // NOTE: We're currently only called from AUBase::ComponentEntryDispatch, which
        // calls DispatchGetPropertyInfo first, which performs validation of the scope/element,
        // and ensures that the outData buffer is non-null and large enough.
        OSStatus result = noErr;

        switch (inID) {
        case kAudioUnitProperty_StreamFormat:
            *(CAStreamBasicDescription *)outData = GetStreamFormat(inScope, inElement);
            break;

        case kAudioUnitProperty_SampleRate:
            *(Float64 *)outData = GetStreamFormat(inScope, inElement).mSampleRate;
            break;

        case kAudioUnitProperty_ParameterList:
            {
                UInt32 nparams = 0;
                result = GetParameterList(inScope, (AudioUnitParameterID *)outData, nparams);
            }
            break;

        case kAudioUnitProperty_ParameterInfo:
            result = GetParameterInfo(inScope, inElement, *(AudioUnitParameterInfo *)outData);
            break;

        case kAudioUnitProperty_ParameterHistoryInfo:
            {
                AudioUnitParameterHistoryInfo* info = (AudioUnitParameterHistoryInfo*)outData;
                result = GetParameterHistoryInfo(inScope, inElement, info->updatesPerSecond, info->historyDurationInSeconds);
            }
            break;

        case kAudioUnitProperty_ClassInfo:
            {
                *(CFPropertyListRef *)outData = NULL;
                result = SaveState((CFPropertyListRef *)outData);
            }
            break;

        case kAudioUnitProperty_FactoryPresets:
            {
                *(CFArrayRef *)outData = NULL;
                result = GetPresets ((CFArrayRef *)outData);
            }
            break;

        case kAudioUnitProperty_PresentPreset:
    #if !CA_USE_AUDIO_PLUGIN_ONLY
    #ifndef __LP64__
        case kAudioUnitProperty_CurrentPreset:
    #endif
    #endif
            {
                *(AUPreset *)outData = mCurrentPreset;

                    // retain current string (as client owns a reference to it and will release it)
                if (inID == kAudioUnitProperty_PresentPreset && mCurrentPreset.presetName)
                    CFRetain (mCurrentPreset.presetName);

                result = noErr;
            }
            break;

        case kAudioUnitProperty_ElementName:
            {
                AUElement * element = GetElement(inScope, inElement);
                if (element->HasName()) {
                    *(CFStringRef *)outData = element->GetName();
                    CFRetain (element->GetName());
                    result = noErr;
                } else
                    result = kAudioUnitErr_InvalidPropertyValue;
            }
            break;

        case kAudioUnitProperty_ElementCount:
            *(UInt32 *)outData = GetScope(inScope).GetNumberOfElements();
            break;

        case kAudioUnitProperty_Latency:
            *(Float64 *)outData = GetLatency();
            break;

        case kAudioUnitProperty_TailTime:
            if (SupportsTail())
                *(Float64 *)outData = GetTailTime();
            else
                result = kAudioUnitErr_InvalidProperty;
            break;

        case kAudioUnitProperty_MaximumFramesPerSlice:
            *(UInt32 *)outData = mMaxFramesPerSlice;
            break;

        case kAudioUnitProperty_LastRenderError:
            *(OSStatus *)outData = mLastRenderError;
            mLastRenderError = 0;
            break;

        case kAudioUnitProperty_SupportedNumChannels:
            {
                const AUChannelInfo* infoPtr = NULL;
                UInt32 num = SupportedNumChannels (&infoPtr);
                if(num != 0 && infoPtr != NULL)
                    memcpy (outData, infoPtr, num * sizeof (AUChannelInfo));
            }
            break;

        case kAudioUnitProperty_SupportedChannelLayoutTags:
            {
                AudioChannelLayoutTag* ptr = outData ? static_cast<AudioChannelLayoutTag*>(outData) : NULL;
                UInt32 numLayouts = GetChannelLayoutTags (inScope, inElement, ptr);
                if (numLayouts == 0)
                    result = kAudioUnitErr_InvalidProperty;
            }
            break;

        case kAudioUnitProperty_AudioChannelLayout:
        {
            AudioChannelLayout* ptr = outData ? static_cast<AudioChannelLayout*>(outData) : NULL;
            Boolean writable;
            UInt32 dataSize = GetAudioChannelLayout(inScope, inElement, ptr, writable);
            if (!dataSize) {
                result = kAudioUnitErr_InvalidProperty;
            }
            break;
        }

    #if (MAC_OS_X_VERSION_MIN_REQUIRED > MAC_OS_X_VERSION_10_5) || TARGET_OS_IPHONE
        case kAudioUnitProperty_ShouldAllocateBuffer:
        {
            AUIOElement * element = GetIOElement(inScope, inElement);
            *(UInt32*)outData = element->WillAllocateBuffer();
            break;
        }
    #endif

        case kAudioUnitProperty_ParameterValueStrings:
            result = GetParameterValueStrings(inScope, inElement, (CFArrayRef *)outData);
            break;

    #if !CA_USE_AUDIO_PLUGIN_ONLY
        case kAudioUnitProperty_FastDispatch:
            if (!IsCMgrObject()) result = kAudioUnitErr_InvalidProperty;
            else {
                switch (inElement) {
                case kAudioUnitGetParameterSelect:
                    *(AudioUnitGetParameterProc *)outData = (AudioUnitGetParameterProc)CMgr_AudioUnitBaseGetParameter;
                    break;
                case kAudioUnitSetParameterSelect:
                    *(AudioUnitSetParameterProc *)outData = (AudioUnitSetParameterProc)CMgr_AudioUnitBaseSetParameter;
                    break;
                case kAudioUnitRenderSelect:
                    if (AudioUnitAPIVersion() > 1)
                        *(AudioUnitRenderProc *)outData = (AudioUnitRenderProc)CMgr_AudioUnitBaseRender;
                    else result = kAudioUnitErr_InvalidElement;
                    break;
                default:
                    result = GetProperty(inID, inScope, inElement, outData);
                    break;
                }
            }
            break;

        case kAudioUnitProperty_GetUIComponentList:
            GetUIComponentDescs ((ComponentDescription*)outData);
            break;
    #endif

    #if !CA_NO_AU_HOST_CALLBACKS
        case kAudioUnitProperty_HostCallbacks:
            memcpy(outData, &mHostCallbackInfo, sizeof(OldHostCallbackInfo));
            break;
    #endif
    #if !CA_NO_AU_UI_FEATURES
        case kAudioUnitProperty_IconLocation:
            {
                CFURLRef iconLocation = CopyIconLocation();
                if (iconLocation) {
                    *(CFURLRef*)outData = iconLocation;
                } else
                    result = kAudioUnitErr_InvalidProperty;
            }
            break;

        case kAudioUnitProperty_ContextName:
            *(CFStringRef *)outData = mContextName;
            if (mContextName) {
                CFRetain(mContextName);
                // retain CFString (if exists) since client will be responsible for its release
                result = noErr;
            } else {
                result = kAudioUnitErr_InvalidPropertyValue;
            }
            break;

        case kAudioUnitProperty_ParameterClumpName:
            {
                AudioUnitParameterNameInfo * ioClumpInfo = (AudioUnitParameterNameInfo*) outData;
                if (ioClumpInfo->inID == kAudioUnitClumpID_System)  // this ID value is reserved
                    result = kAudioUnitErr_InvalidPropertyValue;
                else
                {
                    result = CopyClumpName(inScope, ioClumpInfo->inID, ioClumpInfo->inDesiredLength, &ioClumpInfo->outName);

                        // this is provided for compatbility with existing implementations that don't know
                        // about this new mechanism
                    if (result == kAudioUnitErr_InvalidProperty)
                        result = GetProperty (inID, inScope, inElement, outData);
                }
            }
            break;

    #endif  // !CA_NO_AU_UI_FEATURES

        case 'lrst' : // kAudioUnitProperty_LastRenderedSampleTime
            *(Float64*)outData = mCurrentRenderTime.mSampleTime;
            break;

        case /*kAudioUnitProperty_NickName*/ 54:
            // Ownership follows Core Foundation's 'Copy Rule'
            if (mNickName) CFRetain(mNickName);
            *(CFStringRef*)outData = mNickName;
            break;

        default:
            result = GetProperty(inID, inScope, inElement, outData);
            break;
        }
        return result;
        */
    }
    
    pub fn dispatch_set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;

        switch (inID) {
        case kAudioUnitProperty_MakeConnection:
            ca_require(inDataSize >= sizeof(AudioUnitConnection), InvalidPropertyValue);
            {
                AudioUnitConnection &connection = *(AudioUnitConnection *)inData;
                result = SetConnection(connection);
            }
            break;

        case kAudioUnitProperty_SetRenderCallback:
            {
                ca_require(inDataSize >= sizeof(AURenderCallbackStruct), InvalidPropertyValue);
                ca_require(AudioUnitAPIVersion() > 1, InvalidProperty);
                AURenderCallbackStruct &callback = *(AURenderCallbackStruct*)inData;
                result = SetInputCallback(kAudioUnitProperty_SetRenderCallback, inElement, callback.inputProc, callback.inputProcRefCon);
            }
            break;

        case kAudioUnitProperty_ElementCount:
            ca_require(inDataSize == sizeof(UInt32), InvalidPropertyValue);
            ca_require(BusCountWritable(inScope), NotWritable);
            result = SetBusCount(inScope, *(UInt32*)inData);
            if (result == noErr) {
                PropertyChanged(inID, inScope, inElement);
            }
            break;

        case kAudioUnitProperty_MaximumFramesPerSlice:
            ca_require(inDataSize == sizeof(UInt32), InvalidPropertyValue);
            result = CanSetMaxFrames();
            if (result) return result;
            SetMaxFramesPerSlice(*(UInt32 *)inData);
            break;

        case kAudioUnitProperty_StreamFormat:
            {
                if (inDataSize < 36) goto InvalidPropertyValue;
                ca_require(GetElement(inScope, inElement) != NULL, InvalidElement);

                CAStreamBasicDescription newDesc;
                    // now we're going to be ultra conservative! because of discrepancies between
                    // sizes of this struct based on aligment padding inconsistencies
                memset (&newDesc, 0, sizeof(newDesc));
                memcpy (&newDesc, inData, 36);

                ca_require(ValidFormat(inScope, inElement, newDesc), InvalidFormat);

                const CAStreamBasicDescription curDesc = GetStreamFormat(inScope, inElement);

                if ( !curDesc.IsEqual(newDesc, false) ) {
                    ca_require(IsStreamFormatWritable(inScope, inElement), NotWritable);
                    result = ChangeStreamFormat(inScope, inElement, curDesc, newDesc);
                }
            }
            break;

        case kAudioUnitProperty_SampleRate:
            {
                ca_require(inDataSize == sizeof(Float64), InvalidPropertyValue);
                ca_require(GetElement(inScope, inElement) != NULL, InvalidElement);

                const CAStreamBasicDescription curDesc = GetStreamFormat(inScope, inElement);
                CAStreamBasicDescription newDesc = curDesc;
                newDesc.mSampleRate = *(Float64 *)inData;

                ca_require(ValidFormat(inScope, inElement, newDesc), InvalidFormat);

                if ( !curDesc.IsEqual(newDesc, false) ) {
                    ca_require(IsStreamFormatWritable(inScope, inElement), NotWritable);
                    result = ChangeStreamFormat(inScope, inElement, curDesc, newDesc);
                }
            }
            break;

        case kAudioUnitProperty_AudioChannelLayout:
            {
                const AudioChannelLayout *layout = static_cast<const AudioChannelLayout *>(inData);
                size_t headerSize = sizeof(AudioChannelLayout) - sizeof(AudioChannelDescription);

                ca_require(inDataSize >= headerSize + layout->mNumberChannelDescriptions * sizeof(AudioChannelDescription), InvalidPropertyValue);
                result = SetAudioChannelLayout(inScope, inElement, layout);
                if (result == noErr)
                    PropertyChanged(inID, inScope, inElement);
                break;
            }

        case kAudioUnitProperty_ClassInfo:
            ca_require(inDataSize == sizeof(CFPropertyListRef *), InvalidPropertyValue);
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            result = RestoreState(*(CFPropertyListRef *)inData);
            break;

        case kAudioUnitProperty_PresentPreset:
    #if !CA_USE_AUDIO_PLUGIN_ONLY
    #ifndef __LP64__
        case kAudioUnitProperty_CurrentPreset:
    #endif
    #endif
            {
                ca_require(inDataSize == sizeof(AUPreset), InvalidPropertyValue);
                ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
                AUPreset & newPreset = *(AUPreset *)inData;

                if (newPreset.presetNumber >= 0)
                {
                    result = NewFactoryPresetSet(newPreset);
                    // NewFactoryPresetSet SHOULD call SetAFactoryPreset if the preset is valid
                    // from its own list of preset number->name
                    if (!result)
                        PropertyChanged(inID, inScope, inElement);
                }
                else if (newPreset.presetName)
                {
                    result = NewCustomPresetSet(newPreset);
                    if (!result)
                        PropertyChanged(inID, inScope, inElement);
                }
                else
                    result = kAudioUnitErr_InvalidPropertyValue;
            }
            break;

        case kAudioUnitProperty_ElementName:
            {
                ca_require(GetElement(inScope, inElement) != NULL, InvalidElement);
                ca_require(inDataSize == sizeof(CFStringRef), InvalidPropertyValue);
                AUElement * element = GetScope(inScope).GetElement (inElement);
                element->SetName (*(CFStringRef *)inData);
                PropertyChanged(inID, inScope, inElement);
            }
            break;

    #if (MAC_OS_X_VERSION_MIN_REQUIRED > MAC_OS_X_VERSION_10_5) || TARGET_OS_IPHONE
        case kAudioUnitProperty_ShouldAllocateBuffer:
            {
                ca_require((inScope == kAudioUnitScope_Input || inScope == kAudioUnitScope_Output), InvalidScope);
                ca_require(GetElement(inScope, inElement) != NULL, InvalidElement);
                ca_require(inDataSize == sizeof(UInt32), InvalidPropertyValue);
                ca_require(!IsInitialized(), Initialized);

                AUIOElement * element = GetIOElement(inScope, inElement);
                element->SetWillAllocateBuffer(*(UInt32 *)inData != 0);
            }
            break;
    #endif

    #if !CA_NO_AU_HOST_CALLBACKS
        case kAudioUnitProperty_HostCallbacks:
        {
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            UInt32 availSize = std::min(inDataSize, (UInt32)sizeof(HostCallbackInfo));
            bool hasChanged = !memcmp (&mHostCallbackInfo, inData, availSize);
            memset (&mHostCallbackInfo, 0, sizeof (mHostCallbackInfo));
            memcpy (&mHostCallbackInfo, inData, availSize);
            if (hasChanged)
                PropertyChanged(inID, inScope, inElement);
            break;
        }
    #endif
    #if !CA_NO_AU_UI_FEATURES
        case kAudioUnitProperty_SetExternalBuffer:
            ca_require(inDataSize >= sizeof(AudioUnitExternalBuffer), InvalidPropertyValue);
            ca_require(IsInitialized(), Uninitialized);
            {
                AudioUnitExternalBuffer &buf = *(AudioUnitExternalBuffer*)inData;
                if (intptr_t(buf.buffer) & 0x0F) result = kAudio_ParamError;
                else if (inScope == kAudioUnitScope_Input) {
                    AUInputElement *input = GetInput(inElement);
                    input->UseExternalBuffer(buf);
                } else {
                    AUOutputElement *output = GetOutput(inElement);
                    output->UseExternalBuffer(buf);
                }
            }
            break;

        case kAudioUnitProperty_ContextName:
            {
                ca_require(inDataSize == sizeof(CFStringRef), InvalidPropertyValue);
                ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
                CFStringRef inStr = *(CFStringRef *)inData;
                if (mContextName) CFRelease(mContextName);
                if (inStr) CFRetain(inStr);
                mContextName = inStr;
                PropertyChanged(inID, inScope, inElement);
            }
            break;

    #endif // !CA_NO_AU_UI_FEATURES

        case /*kAudioUnitProperty_NickName*/ 54:
        {
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            ca_require(inDataSize == sizeof(CFStringRef), InvalidPropertyValue);
            CFStringRef inStr = *(CFStringRef *)inData;
            if (mNickName) CFRelease(mNickName);
            if (inStr) CFRetain(inStr);
            mNickName = inStr;
            PropertyChanged(inID, inScope, inElement);
            break;
        }

        default:
            result = SetProperty(inID, inScope, inElement, inData, inDataSize);
            if (result == noErr)
                PropertyChanged(inID, inScope, inElement);

            break;
        }
        return result;
    NotWritable:
        return kAudioUnitErr_PropertyNotWritable;
    InvalidFormat:
        return kAudioUnitErr_FormatNotSupported;
    #if !CA_NO_AU_UI_FEATURES
    Uninitialized:
        return kAudioUnitErr_Uninitialized;
    #endif
    #if (MAC_OS_X_VERSION_MIN_REQUIRED > MAC_OS_X_VERSION_10_5) || CA_USE_AUDIO_PLUGIN_ONLY
    Initialized:
        return kAudioUnitErr_Initialized;
    #endif
    InvalidScope:
        return kAudioUnitErr_InvalidScope;
    InvalidProperty:
        return kAudioUnitErr_InvalidProperty;
    InvalidPropertyValue:
        return kAudioUnitErr_InvalidPropertyValue;
    InvalidElement:
        return kAudioUnitErr_InvalidElement;
        */
    }
    
    pub fn dispatch_remove_property_value(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;
        switch (inID)
        {
        case kAudioUnitProperty_AudioChannelLayout:
        {
            result = RemoveAudioChannelLayout(inScope, inElement);
            if (result == noErr)
                PropertyChanged(inID, inScope, inElement);
            break;
        }

    #if !CA_NO_AU_HOST_CALLBACKS
        case kAudioUnitProperty_HostCallbacks:
        {
            ca_require(inScope == kAudioUnitScope_Global, InvalidScope);
            bool hasValue = false;
            void* ptr = &mHostCallbackInfo;
            for (unsigned int i = 0; i <  sizeof (HostCallbackInfo); ++i) {
                if (static_cast<char*>(ptr)[i]) {
                    hasValue = true;
                    break;
                }
            }
            if (hasValue) {
                memset (&mHostCallbackInfo, 0, sizeof (HostCallbackInfo));
                PropertyChanged(inID, inScope, inElement);
            }
            break;
        }
    #endif
    #if !CA_NO_AU_UI_FEATURES
        case kAudioUnitProperty_ContextName:
            if (mContextName) CFRelease(mContextName);
            mContextName = NULL;
            result = noErr;
            break;

    #endif // !CA_NO_AU_UI_FEATURES

        case /*kAudioUnitProperty_NickName*/ 54:
        {
            if(inScope == kAudioUnitScope_Global) {
                if (mNickName) CFRelease(mNickName);
                mNickName = NULL;
                PropertyChanged(inID, inScope, inElement);
            } else {
                result = kAudioUnitErr_InvalidScope;
            }
            break;
        }

        default:
            result = RemovePropertyValue (inID, inScope, inElement);
            break;
        }

        return result;
    #if !CA_NO_AU_UI_FEATURES || !CA_NO_AU_HOST_CALLBACKS
    InvalidScope:
        return kAudioUnitErr_InvalidScope;
    #endif
        */
    }
    
    pub fn get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn remove_property_value(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidPropertyValue;
        */
    }
    
    pub fn add_property_listener(&mut self, 
        inid:            AudioUnitPropertyID,
        in_proc:         AudioUnitPropertyListenerProc,
        in_proc_ref_con: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            AUBasePropertyListener pl;

        pl.propertyID = inID;
        pl.listenerProc = inProc;
        pl.listenerRefCon = inProcRefCon;

        if (mPropertyListeners.empty())
            mPropertyListeners.reserve(32);
        mPropertyListeners.push_back(pl);

        return noErr;
        */
    }
    
    pub fn remove_property_listener(&mut self, 
        inid:              AudioUnitPropertyID,
        in_proc:           AudioUnitPropertyListenerProc,
        in_proc_ref_con:   *mut c_void,
        ref_con_specified: bool) -> OSStatus {
        
        todo!();
        /*
            // iterate in reverse so that it's safe to erase in the middle of the vector
        for (int i = (int)mPropertyListeners.size(); --i >=0; ) {
            AUBasePropertyListeners::iterator it = mPropertyListeners.begin() + i;
            if ((*it).propertyID == inID && (*it).listenerProc == inProc && (!refConSpecified || (*it).listenerRefCon == inProcRefCon))
                mPropertyListeners.erase(it);
        }
        return noErr;
        */
    }
    
    pub fn property_changed(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement)  {
        
        todo!();
        /*
            for (AUBasePropertyListeners::iterator it = mPropertyListeners.begin(); it != mPropertyListeners.end(); ++it)
            if ((*it).propertyID == inID)
                ((*it).listenerProc)((*it).listenerRefCon, mComponentInstance, inID, inScope, inElement);
        */
    }
    
    pub fn set_render_notification(&mut self, 
        in_proc:    AURenderCallback,
        in_ref_con: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            if (inProc == NULL)
            return kAudio_ParamError;

        mRenderCallbacksTouched = true;
        mRenderCallbacks.deferred_add(RenderCallback(inProc, inRefCon));
                // this will do nothing if it's already in the list
        return noErr;
        */
    }
    
    pub fn remove_render_notification(&mut self, 
        in_proc:    AURenderCallback,
        in_ref_con: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            mRenderCallbacks.deferred_remove(RenderCallback(inProc, inRefCon));
        return noErr;   // error?
        */
    }
    
    pub fn get_parameter(&mut self, 
        inid:       AudioUnitParameterID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_value:  &mut AudioUnitParameterValue) -> OSStatus {
        
        todo!();
        /*
            AUElement *elem = SafeGetElement(inScope, inElement);
        outValue = elem->GetParameter(inID);
        return noErr;
        */
    }
    
    pub fn set_parameter(&mut self, 
        inid:                       AudioUnitParameterID,
        in_scope:                   AudioUnitScope,
        in_element:                 AudioUnitElement,
        in_value:                   AudioUnitParameterValue,
        in_buffer_offset_in_frames: u32) -> OSStatus {
        
        todo!();
        /*
            AUElement *elem = SafeGetElement(inScope, inElement);
        elem->SetParameter(inID, inValue);
        return noErr;
        */
    }
    
    pub fn schedule_parameter(&mut self, 
        in_parameter_event: *const AudioUnitParameterEvent,
        in_num_events:      u32) -> OSStatus {
        
        todo!();
        /*
            bool canScheduleParameters = CanScheduleParameters();

        for (UInt32 i = 0; i < inNumEvents; ++i)
        {
            if (inParameterEvent[i].eventType == kParameterEvent_Immediate)
            {
                SetParameter (inParameterEvent[i].parameter,
                                inParameterEvent[i].scope,
                                inParameterEvent[i].element,
                                inParameterEvent[i].eventValues.immediate.value,
                                inParameterEvent[i].eventValues.immediate.bufferOffset);
            }
            if (canScheduleParameters) {
                mParamList.push_back (inParameterEvent[i]);
            }
        }

        return noErr;
        */
    }
    
    pub fn process_for_scheduled_params(&mut self, 
        in_param_list:        &mut ParameterEventList,
        in_frames_to_process: u32,
        in_user_data:         *mut c_void) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;

        int totalFramesToProcess = inFramesToProcess;

        int framesRemaining = totalFramesToProcess;

        unsigned int currentStartFrame = 0; // start of the whole buffer


        // sort the ParameterEventList by startBufferOffset
        std::sort(inParamList.begin(), inParamList.end(), SortParameterEventList);

        ParameterEventList::iterator iter = inParamList.begin();

        while(framesRemaining > 0 )
        {
            // first of all, go through the ramped automation events and find out where the next
            // division of our whole buffer will be

            int currentEndFrame = totalFramesToProcess; // start out assuming we'll process all the way to
                                                        // the end of the buffer

            iter = inParamList.begin();

            // find the next break point
            while(iter != inParamList.end() )
            {
                AudioUnitParameterEvent &event = *iter;

                int offset = event.eventType == kParameterEvent_Immediate ?  event.eventValues.immediate.bufferOffset : event.eventValues.ramp.startBufferOffset;

                if(offset > (int)currentStartFrame && offset < currentEndFrame )
                {
                    currentEndFrame = offset;
                    break;
                }

                // consider ramp end to be a possible choice (there may be gaps in the supplied ramp events)
                if(event.eventType == kParameterEvent_Ramped )
                {
                    offset = event.eventValues.ramp.startBufferOffset + event.eventValues.ramp.durationInFrames;

                    if(offset > (int)currentStartFrame && offset < currentEndFrame )
                    {
                        currentEndFrame = offset;
                    }
                }

                iter++;
            }

            int framesThisTime = currentEndFrame - currentStartFrame;

            // next, setup the parameter maps to be current for the ramp parameters active during
            // this time segment...

            for(ParameterEventList::iterator iter2 = inParamList.begin(); iter2 != inParamList.end(); iter2++ )
            {
                AudioUnitParameterEvent &event = *iter2;

                bool eventFallsInSlice;

                if(event.eventType == kParameterEvent_Ramped)
                    eventFallsInSlice = event.eventValues.ramp.startBufferOffset < currentEndFrame
                        && event.eventValues.ramp.startBufferOffset + event.eventValues.ramp.durationInFrames > currentStartFrame;
                else /* kParameterEvent_Immediate */
                    // actually, for the same parameter, there may be future immediate events which override this one,
                    // but it's OK since the event list is sorted in time order, we're guaranteed to end up with the current one
                    eventFallsInSlice = event.eventValues.immediate.bufferOffset <= currentStartFrame;

                if(eventFallsInSlice)
                {
                    AUElement *element = GetElement(event.scope, event.element );

                    if(element) element->SetScheduledEvent( event.parameter,
                                                            event,
                                                            currentStartFrame,
                                                            currentEndFrame - currentStartFrame );
                }
            }


            // Finally, actually do the processing for this slice.....

            result = ProcessScheduledSlice( inUserData,
                                            currentStartFrame,
                                            framesThisTime,
                                            inFramesToProcess );

            if(result != noErr) break;

            framesRemaining -= framesThisTime;
            currentStartFrame = currentEndFrame;    // now start from where we left off last time
        }

        return result;
        */
    }
    
    pub fn set_wants_render_threadid(&mut self, in_flag: bool)  {
        
        todo!();
        /*
            if (inFlag == mWantsRenderThreadID)
            return;

        mWantsRenderThreadID = inFlag;
        if (!mWantsRenderThreadID)
            mRenderThreadID = NULL;
        */
    }
    
    pub fn do_render(&mut self, 
        io_action_flags:      &mut AudioUnitRenderActionFlags,
        in_time_stamp:        &AudioTimeStamp,
        in_bus_number:        u32,
        in_frames_to_process: u32,
        io_data:              &mut AudioBufferList) -> OSStatus {
        
        todo!();
        /*
            OSStatus theError;
        RenderCallbackList::iterator rcit;

        AUTRACE(kCATrace_AUBaseRenderStart, mComponentInstance, (uintptr_t)this, inBusNumber, inFramesToProcess, (uintptr_t)ioData.mBuffers[0].mData);
        DISABLE_DENORMALS

        try {
            ca_require(IsInitialized(), Uninitialized);
            ca_require(mAudioUnitAPIVersion >= 2, ParamErr);
            if (inFramesToProcess > mMaxFramesPerSlice) {
                static time_t lastTimeMessagePrinted = 0;
                time_t now = time(NULL);
                if (now != lastTimeMessagePrinted) {
                    lastTimeMessagePrinted = now;
                    syslog(LOG_ERR, "kAudioUnitErr_TooManyFramesToProcess : inFramesToProcess=%u, mMaxFramesPerSlice=%u", (unsigned)inFramesToProcess, (unsigned)mMaxFramesPerSlice);
                    DebugMessageN4("%s:%d inFramesToProcess=%u, mMaxFramesPerSlice=%u; TooManyFrames", __FILE__, __LINE__, (unsigned)inFramesToProcess, (unsigned)mMaxFramesPerSlice);
                }
                goto TooManyFrames;
            }
            ca_require (!UsesFixedBlockSize() || inFramesToProcess == GetMaxFramesPerSlice(), ParamErr);

            AUOutputElement *output = GetScope(kAudioUnitScope_Output).GetNumberOfElements() > 0 ? GetOutput(inBusNumber) : NULL;   // will throw if non-existant
            if (output != NULL && output->GetStreamFormat().NumberChannelStreams() != ioData.mNumberBuffers) {
                DebugMessageN4("%s:%d ioData.mNumberBuffers=%u, output->GetStreamFormat().NumberChannelStreams()=%u; kAudio_ParamError",
                    __FILE__, __LINE__, (unsigned)ioData.mNumberBuffers, (unsigned)output->GetStreamFormat().NumberChannelStreams());
                goto ParamErr;
            }

            unsigned expectedBufferByteSize = output != NULL ? inFramesToProcess * output->GetStreamFormat().mBytesPerFrame : 0;
            for (unsigned ibuf = 0; ibuf < ioData.mNumberBuffers; ++ibuf) {
                AudioBuffer &buf = ioData.mBuffers[ibuf];
                if (buf.mData != NULL) {
                    // only care about the size if the buffer is non-null
                    if (buf.mDataByteSize < expectedBufferByteSize) {
                        // if the buffer is too small, we cannot render safely. kAudio_ParamError.
                        DebugMessageN7("%s:%d %u frames, %u bytes/frame, expected %u-byte buffer; ioData.mBuffers[%u].mDataByteSize=%u; kAudio_ParamError",
                            __FILE__, __LINE__, (unsigned)inFramesToProcess, (unsigned)output->GetStreamFormat().mBytesPerFrame, expectedBufferByteSize, ibuf, (unsigned)buf.mDataByteSize);
                        goto ParamErr;
                    }
                    // Some clients incorrectly pass bigger buffers than expectedBufferByteSize.
                    // We will generally set the buffer size at the end of rendering, before we return.
                    // However we should ensure that no one, DURING rendering, READS a
                    // potentially incorrect size. This can lead to doing too much work, or
                    // reading past the end of an input buffer into unmapped memory.
                    buf.mDataByteSize = expectedBufferByteSize;
                }
            }

            if (WantsRenderThreadID())
            {
                #if TARGET_OS_MAC
                    mRenderThreadID = pthread_self();
                #elif TARGET_OS_WIN32
                    mRenderThreadID = GetCurrentThreadId();
                #endif
            }

            AudioUnitRenderActionFlags flags;
            if (mRenderCallbacksTouched) {
                mRenderCallbacks.update();
                flags = ioActionFlags | kAudioUnitRenderAction_PreRender;
                for (rcit = mRenderCallbacks.begin(); rcit != mRenderCallbacks.end(); ++rcit) {
                    RenderCallback &rc = *rcit;
                    AUTRACE(kCATrace_AUBaseRenderCallbackStart, mComponentInstance, (intptr_t)this, (intptr_t)rc.mRenderNotify, 1, 0);
                    (*(AURenderCallback)rc.mRenderNotify)(rc.mRenderNotifyRefCon,
                                    &flags,
                                    &inTimeStamp, inBusNumber, inFramesToProcess, &ioData);
                    AUTRACE(kCATrace_AUBaseRenderCallbackEnd, mComponentInstance, (intptr_t)this, (intptr_t)rc.mRenderNotify, 1, 0);
                }
            }

            theError = DoRenderBus(ioActionFlags, inTimeStamp, inBusNumber, output, inFramesToProcess, ioData);

            if (mRenderCallbacksTouched) {
                flags = ioActionFlags | kAudioUnitRenderAction_PostRender;

                if (SetRenderError (theError)) {
                    flags |= kAudioUnitRenderAction_PostRenderError;
                }

                for (rcit = mRenderCallbacks.begin(); rcit != mRenderCallbacks.end(); ++rcit) {
                    RenderCallback &rc = *rcit;
                    AUTRACE(kCATrace_AUBaseRenderCallbackStart, mComponentInstance, (intptr_t)this, (intptr_t)rc.mRenderNotify, 2, 0);
                    (*(AURenderCallback)rc.mRenderNotify)(rc.mRenderNotifyRefCon,
                                    &flags,
                                    &inTimeStamp, inBusNumber, inFramesToProcess, &ioData);
                    AUTRACE(kCATrace_AUBaseRenderCallbackEnd, mComponentInstance, (intptr_t)this, (intptr_t)rc.mRenderNotify, 2, 0);
                }
            }

            // The vector's being emptied
            // because these events should only apply to this Render cycle, so anything
            // left over is from a preceding cycle and should be dumped.  New scheduled
            // parameters must be scheduled from the next pre-render callback.
            if (!mParamList.empty())
                mParamList.clear();

        }
        catch (OSStatus err) {
            theError = err;
            goto errexit;
        }
        catch (...) {
            theError = -1;
            goto errexit;
        }
    done:
        RESTORE_DENORMALS
        AUTRACE(kCATrace_AUBaseRenderEnd, mComponentInstance, (intptr_t)this, theError, ioActionFlags, CATrace::ablData(ioData));

        return theError;

    Uninitialized:  theError = kAudioUnitErr_Uninitialized;             goto errexit;
    ParamErr:       theError = kAudio_ParamError;                       goto errexit;
    TooManyFrames:  theError = kAudioUnitErr_TooManyFramesToProcess;    goto errexit;
    errexit:
        DebugMessageN2 ("  from %s, render err: %d", GetLoggingString(), (int)theError);
        SetRenderError(theError);
        goto done;
        */
    }
    
    pub fn do_process(&mut self, 
        io_action_flags:      &mut AudioUnitRenderActionFlags,
        in_time_stamp:        &AudioTimeStamp,
        in_frames_to_process: u32,
        io_data:              &mut AudioBufferList) -> OSStatus {
        
        todo!();
        /*
            OSStatus theError;
        AUTRACE(kCATrace_AUBaseRenderStart, mComponentInstance, (intptr_t)this, -1, inFramesToProcess, 0);
        DISABLE_DENORMALS

        try {

            if (!(ioActionFlags & (1 << 9)/*kAudioUnitRenderAction_DoNotCheckRenderArgs*/)) {
                ca_require(IsInitialized(), Uninitialized);
                ca_require(inFramesToProcess <= mMaxFramesPerSlice, TooManyFrames);
                ca_require(!UsesFixedBlockSize() || inFramesToProcess == GetMaxFramesPerSlice(), ParamErr);

                AUInputElement *input = GetInput(0);    // will throw if non-existant
                if (input->GetStreamFormat().NumberChannelStreams() != ioData.mNumberBuffers) {
                    DebugMessageN4("%s:%d ioData.mNumberBuffers=%u, input->GetStreamFormat().NumberChannelStreams()=%u; kAudio_ParamError",
                        __FILE__, __LINE__, (unsigned)ioData.mNumberBuffers, (unsigned)input->GetStreamFormat().NumberChannelStreams());
                    goto ParamErr;
                }

                unsigned expectedBufferByteSize = inFramesToProcess * input->GetStreamFormat().mBytesPerFrame;
                for (unsigned ibuf = 0; ibuf < ioData.mNumberBuffers; ++ibuf) {
                    AudioBuffer &buf = ioData.mBuffers[ibuf];
                    if (buf.mData != NULL) {
                        // only care about the size if the buffer is non-null
                        if (buf.mDataByteSize < expectedBufferByteSize) {
                            // if the buffer is too small, we cannot render safely. kAudio_ParamError.
                            DebugMessageN7("%s:%d %u frames, %u bytes/frame, expected %u-byte buffer; ioData.mBuffers[%u].mDataByteSize=%u; kAudio_ParamError",
                                __FILE__, __LINE__, (unsigned)inFramesToProcess, (unsigned)input->GetStreamFormat().mBytesPerFrame, expectedBufferByteSize, ibuf, (unsigned)buf.mDataByteSize);
                            goto ParamErr;
                        }
                        // Some clients incorrectly pass bigger buffers than expectedBufferByteSize.
                        // We will generally set the buffer size at the end of rendering, before we return.
                        // However we should ensure that no one, DURING rendering, READS a
                        // potentially incorrect size. This can lead to doing too much work, or
                        // reading past the end of an input buffer into unmapped memory.
                        buf.mDataByteSize = expectedBufferByteSize;
                    }
                }
            }

            if (WantsRenderThreadID())
            {
                #if TARGET_OS_MAC
                    mRenderThreadID = pthread_self();
                #elif TARGET_OS_WIN32
                    mRenderThreadID = GetCurrentThreadId();
                #endif
            }

            if (NeedsToRender (inTimeStamp)) {
                theError = ProcessBufferLists (ioActionFlags, ioData, ioData, inFramesToProcess);
            } else
                theError = noErr;

        }
        catch (OSStatus err) {
            theError = err;
            goto errexit;
        }
        catch (...) {
            theError = -1;
            goto errexit;
        }
    done:
        RESTORE_DENORMALS
        AUTRACE(kCATrace_AUBaseRenderEnd, mComponentInstance, (intptr_t)this, theError, ioActionFlags, CATrace::ablData(ioData));

        return theError;

    Uninitialized:  theError = kAudioUnitErr_Uninitialized;             goto errexit;
    ParamErr:       theError = kAudio_ParamError;                       goto errexit;
    TooManyFrames:  theError = kAudioUnitErr_TooManyFramesToProcess;    goto errexit;
    errexit:
        DebugMessageN2 ("  from %s, process err: %d", GetLoggingString(), (int)theError);
        SetRenderError(theError);
        goto done;
        */
    }
    
    pub fn do_process_multiple(&mut self, 
        io_action_flags:               &mut AudioUnitRenderActionFlags,
        in_time_stamp:                 &AudioTimeStamp,
        in_frames_to_process:          u32,
        in_number_input_buffer_lists:  u32,
        in_input_buffer_lists:         *const *const AudioBufferList,
        in_number_output_buffer_lists: u32,
        io_output_buffer_lists:        *mut *mut AudioBufferList) -> OSStatus {
        
        todo!();
        /*
            OSStatus theError;
        DISABLE_DENORMALS

        try {

            if (!(ioActionFlags & (1 << 9)/*kAudioUnitRenderAction_DoNotCheckRenderArgs*/)) {
                ca_require(IsInitialized(), Uninitialized);
                ca_require(inFramesToProcess <= mMaxFramesPerSlice, TooManyFrames);
                ca_require (!UsesFixedBlockSize() || inFramesToProcess == GetMaxFramesPerSlice(), ParamErr);

                for (unsigned ibl = 0; ibl < inNumberInputBufferLists; ++ibl) {
                    if (inInputBufferLists[ibl] != NULL) {
                        AUInputElement *input = GetInput(ibl);  // will throw if non-existant
                        unsigned expectedBufferByteSize = inFramesToProcess * input->GetStreamFormat().mBytesPerFrame;

                        if (input->GetStreamFormat().NumberChannelStreams() != inInputBufferLists[ibl]->mNumberBuffers) {
                            DebugMessageN5("%s:%d inInputBufferLists[%u]->mNumberBuffers=%u, input->GetStreamFormat().NumberChannelStreams()=%u; kAudio_ParamError",
                                           __FILE__, __LINE__, ibl, (unsigned)inInputBufferLists[ibl]->mNumberBuffers, (unsigned)input->GetStreamFormat().NumberChannelStreams());
                            goto ParamErr;
                        }

                        for (unsigned ibuf = 0; ibuf < inInputBufferLists[ibl]->mNumberBuffers; ++ibuf) {
                            const AudioBuffer &buf = inInputBufferLists[ibl]->mBuffers[ibuf];
                            if (buf.mData != NULL) {
                                if (buf.mDataByteSize < expectedBufferByteSize) {
                                    // the buffer is too small
                                    DebugMessageN8("%s:%d %u frames, %u bytes/frame, expected %u-byte buffer; inInputBufferLists[%u].mBuffers[%u].mDataByteSize=%u; kAudio_ParamError",
                                                   __FILE__, __LINE__, (unsigned)inFramesToProcess, (unsigned)input->GetStreamFormat().mBytesPerFrame, expectedBufferByteSize, ibl, ibuf, (unsigned)buf.mDataByteSize);
                                    goto ParamErr;
                                }
                            } else {
                                // the buffer must exist
                                goto ParamErr;
                            }
                        }
                    } else {
                        // skip NULL input audio buffer list
                    }
                }

                for (unsigned obl = 0; obl < inNumberOutputBufferLists; ++obl) {
                    if (ioOutputBufferLists[obl] != NULL) {
                        AUOutputElement *output = GetOutput(obl);   // will throw if non-existant
                        unsigned expectedBufferByteSize = inFramesToProcess * output->GetStreamFormat().mBytesPerFrame;

                        if (output->GetStreamFormat().NumberChannelStreams() != ioOutputBufferLists[obl]->mNumberBuffers) {
                            DebugMessageN5("%s:%d ioOutputBufferLists[%u]->mNumberBuffers=%u, output->GetStreamFormat().NumberChannelStreams()=%u; kAudio_ParamError",
                                           __FILE__, __LINE__, obl, (unsigned)ioOutputBufferLists[obl]->mNumberBuffers, (unsigned)output->GetStreamFormat().NumberChannelStreams());
                            goto ParamErr;
                        }

                        for (unsigned obuf = 0; obuf < ioOutputBufferLists[obl]->mNumberBuffers; ++obuf) {
                            AudioBuffer &buf = ioOutputBufferLists[obl]->mBuffers[obuf];
                            if (buf.mData != NULL) {
                                // only care about the size if the buffer is non-null
                                if (buf.mDataByteSize < expectedBufferByteSize) {
                                    // if the buffer is too small, we cannot render safely. kAudio_ParamError.
                                    DebugMessageN8("%s:%d %u frames, %u bytes/frame, expected %u-byte buffer; ioOutputBufferLists[%u]->mBuffers[%u].mDataByteSize=%u; kAudio_ParamError",
                                                   __FILE__, __LINE__, (unsigned)inFramesToProcess, (unsigned)output->GetStreamFormat().mBytesPerFrame, expectedBufferByteSize, obl, obuf, (unsigned)buf.mDataByteSize);
                                    goto ParamErr;
                                }
                                // Some clients incorrectly pass bigger buffers than expectedBufferByteSize.
                                // We will generally set the buffer size at the end of rendering, before we return.
                                // However we should ensure that no one, DURING rendering, READS a
                                // potentially incorrect size. This can lead to doing too much work, or
                                // reading past the end of an input buffer into unmapped memory.
                                buf.mDataByteSize = expectedBufferByteSize;
                            }
                        }
                    } else {
                        // skip NULL output audio buffer list
                    }
                }
            }

            if (WantsRenderThreadID())
            {
    #if TARGET_OS_MAC
                mRenderThreadID = pthread_self();
    #elif TARGET_OS_WIN32
                mRenderThreadID = GetCurrentThreadId();
    #endif
            }

            if (NeedsToRender (inTimeStamp)) {
                theError = ProcessMultipleBufferLists (ioActionFlags, inFramesToProcess, inNumberInputBufferLists, inInputBufferLists, inNumberOutputBufferLists, ioOutputBufferLists);
            } else
                theError = noErr;
        }
        catch (OSStatus err) {
            theError = err;
            goto errexit;
        }
        catch (...) {
            theError = -1;
            goto errexit;
        }
    done:
        RESTORE_DENORMALS

        return theError;

    Uninitialized:  theError = kAudioUnitErr_Uninitialized;             goto errexit;
    ParamErr:       theError = kAudio_ParamError;                       goto errexit;
    TooManyFrames:  theError = kAudioUnitErr_TooManyFramesToProcess;    goto errexit;
    errexit:
        DebugMessageN2 ("  from %s, processmultiple err: %d", GetLoggingString(), (int)theError);
        SetRenderError(theError);
        goto done;
        */
    }
    
    pub fn set_input_callback(&mut self, 
        in_propertyid: u32,
        in_element:    AudioUnitElement,
        in_proc:       AURenderCallback,
        in_ref_con:    *mut c_void) -> OSStatus {
        
        todo!();
        /*
            AUInputElement *input = GetInput(inElement);    // may throw

        input->SetInputCallback(inProc, inRefCon);
        PropertyChanged(inPropertyID, kAudioUnitScope_Input, inElement);

        return noErr;
        */
    }
    
    pub fn set_connection(&mut self, in_connection: &AudioUnitConnection) -> OSStatus {
        
        todo!();
        /*
            OSStatus err;
        AUInputElement *input = GetInput(inConnection.destInputNumber); // may throw

        if (inConnection.sourceAudioUnit) {
            // connecting, not disconnecting
            CAStreamBasicDescription sourceDesc;
            UInt32 size = sizeof(CAStreamBasicDescription);
            ca_require_noerr(err = AudioUnitGetProperty(
                                            inConnection.sourceAudioUnit,
                                            kAudioUnitProperty_StreamFormat,
                                            kAudioUnitScope_Output,
                                            inConnection.sourceOutputNumber,
                                            &sourceDesc,
                                            &size), errexit);
            ca_require_noerr(err = DispatchSetProperty (kAudioUnitProperty_StreamFormat,
                                    kAudioUnitScope_Input, inConnection.destInputNumber,
                                    &sourceDesc, sizeof(CAStreamBasicDescription)), errexit);
        }
        input->SetConnection(inConnection);

        PropertyChanged(kAudioUnitProperty_MakeConnection, kAudioUnitScope_Input, inConnection.destInputNumber);
        return noErr;

    errexit:
        return err;
        */
    }
    
    pub fn supported_num_channels(&mut self, out_info: *const *const AUChannelInfo) -> u32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn valid_format(&mut self, 
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        in_new_format: &CAStreamBasicDescription) -> bool {
        
        todo!();
        /*
            return FormatIsCanonical(inNewFormat);
        */
    }
    
    pub fn is_stream_format_writable(&mut self, 
        scope:   AudioUnitScope,
        element: AudioUnitElement) -> bool {
        
        todo!();
        /*
            switch (scope) {
        case kAudioUnitScope_Input:
            {
                AUInputElement *input = GetInput(element);
                if (input->HasConnection()) return false;   // can't write format when input comes from connection
            }
            // ... fall ...
        case kAudioUnitScope_Output:
            return StreamFormatWritable(scope, element);

    //#warning "aliasing of global scope format should be pushed to subclasses"
        case kAudioUnitScope_Global:
            return StreamFormatWritable(kAudioUnitScope_Output, 0);
        }
        return false;
        */
    }
    
    pub fn get_stream_format(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> &CAStreamBasicDescription {
        
        todo!();
        /*
            //#warning "aliasing of global scope format should be pushed to subclasses"
        AUIOElement *element;

        switch (inScope) {
        case kAudioUnitScope_Input:
            element = Inputs().GetIOElement(inElement);
            break;
        case kAudioUnitScope_Output:
            element = Outputs().GetIOElement(inElement);
            break;
        case kAudioUnitScope_Global:    // global stream description is an alias for that of output 0
            element = Outputs().GetIOElement(0);
            break;
        default:
            COMPONENT_THROW(kAudioUnitErr_InvalidScope);
        }
        return element->GetStreamFormat();
        */
    }
    
    pub fn set_bus_count(&mut self, 
        in_scope: AudioUnitScope,
        in_count: u32) -> OSStatus {
        
        todo!();
        /*
            if (IsInitialized())
            return kAudioUnitErr_Initialized;

        GetScope(inScope).SetNumberOfElements(inCount);
        return noErr;
        */
    }
    
    pub fn change_stream_format(&mut self, 
        in_scope:       AudioUnitScope,
        in_element:     AudioUnitElement,
        in_prev_format: &CAStreamBasicDescription,
        in_new_format:  &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
            //#warning "aliasing of global scope format should be pushed to subclasses"
        AUIOElement *element;

        switch (inScope) {
        case kAudioUnitScope_Input:
            element = Inputs().GetIOElement(inElement);
            break;
        case kAudioUnitScope_Output:
            element = Outputs().GetIOElement(inElement);
            break;
        case kAudioUnitScope_Global:
            element = Outputs().GetIOElement(0);
            break;
        default:
            COMPONENT_THROW(kAudioUnitErr_InvalidScope);
        }
        element->SetStreamFormat(inNewFormat);
        PropertyChanged(kAudioUnitProperty_StreamFormat, inScope, inElement);
        return noErr;
        */
    }
    
    pub fn get_channel_layout_tags(&mut self, 
        in_scope:        AudioUnitScope,
        in_element:      AudioUnitElement,
        out_layout_tags: *mut AudioChannelLayoutTag) -> u32 {
        
        todo!();
        /*
            return GetIOElement(inScope, inElement)->GetChannelLayoutTags(outLayoutTags);
        */
    }
    
    pub fn get_audio_channel_layout(&mut self, 
        scope:          AudioUnitScope,
        element:        AudioUnitElement,
        out_layout_ptr: *mut AudioChannelLayout,
        out_writable:   &mut bool) -> u32 {
        
        todo!();
        /*
            AUIOElement * el = GetIOElement(scope, element);
        return el->GetAudioChannelLayout(outLayoutPtr, outWritable);
        */
    }
    
    pub fn remove_audio_channel_layout(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = noErr;
        AUIOElement * el = GetIOElement(inScope, inElement);
        Boolean writable;
        if (el->GetAudioChannelLayout(NULL, writable)) {
            result = el->RemoveAudioChannelLayout();
        }
        return result;
        */
    }
    
    pub fn set_audio_channel_layout(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        in_layout:  *const AudioChannelLayout) -> OSStatus {
        
        todo!();
        /*
            AUIOElement* ioEl = GetIOElement (inScope, inElement);

        // the num channels of the layout HAS TO MATCH the current channels of the Element's stream format
        UInt32 currentChannels = ioEl->GetStreamFormat().NumberChannels();
        UInt32 numChannelsInLayout = CAAudioChannelLayout::NumberChannels(*inLayout);
        if (currentChannels != numChannelsInLayout)
            return kAudioUnitErr_InvalidPropertyValue;

        UInt32 numLayouts = GetChannelLayoutTags (inScope, inElement, NULL);
        if (numLayouts == 0)
            return kAudioUnitErr_InvalidProperty;
        AudioChannelLayoutTag *tags = (AudioChannelLayoutTag *)CA_malloc (numLayouts * sizeof (AudioChannelLayoutTag));
        GetChannelLayoutTags (inScope, inElement, tags);
        bool foundTag = false;
        for (unsigned int i = 0; i < numLayouts; ++i) {
            if (tags[i] == inLayout->mChannelLayoutTag || tags[i] == kAudioChannelLayoutTag_UseChannelDescriptions) {
                foundTag = true;
                break;
            }
        }
        free(tags);

        if (foundTag == false)
            return kAudioUnitErr_InvalidPropertyValue;

        return ioEl->SetAudioChannelLayout(*inLayout);
        */
    }
    
    pub fn save_state(&mut self, out_data: *mut CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
            AudioComponentDescription desc = GetComponentDescription();

        CFMutableDictionaryRef dict = CFDictionaryCreateMutable (NULL, 0,
                                    &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);

    // first step -> save the version to the data ref
        SInt32 value = kCurrentSavedStateVersion;
        AddNumToDictionary (dict, kVersionString, value);

    // second step -> save the component type, subtype, manu to the data ref
        value = desc.componentType;
        AddNumToDictionary (dict, kTypeString, value);

        value = desc.componentSubType;
        AddNumToDictionary (dict, kSubtypeString, value);

        value = desc.componentManufacturer;
        AddNumToDictionary (dict, kManufacturerString, value);

    // fourth step -> save the state of all parameters on all scopes and elements
        CFMutableDataRef data = CFDataCreateMutable(NULL, 0);
        for (AudioUnitScope iscope = 0; iscope < 3; ++iscope) {
            AUScope &scope = GetScope(iscope);
            scope.SaveState (data);
        }

        SaveExtendedScopes(data);

    // save all this in the data section of the dictionary
        CFDictionarySetValue(dict, kDataString, data);
        CFRelease (data);

    //OK - now we're going to do some properties
    //save the preset name...
        CFDictionarySetValue (dict, kNameString, mCurrentPreset.presetName);

    // Does the unit support the RenderQuality property - if so, save it...
        value = 0;
        OSStatus result = DispatchGetProperty (kAudioUnitProperty_RenderQuality,
                                    kAudioUnitScope_Global,
                                    0,
                                    &value);

        if (result == noErr) {
            AddNumToDictionary (dict, kRenderQualityString, value);
        }

    // Does the unit support the CPULoad Quality property - if so, save it...
        Float32 cpuLoad;
        result = DispatchGetProperty (6/*kAudioUnitProperty_CPULoad*/,
                                    kAudioUnitScope_Global,
                                    0,
                                    &cpuLoad);

        if (result == noErr) {
            CFNumberRef num = CFNumberCreate (NULL, kCFNumberFloatType, &cpuLoad);
            CFDictionarySetValue (dict, kCPULoadString, num);
            CFRelease (num);
        }

    // Do we have any element names for any of our scopes?
        // first check to see if we have any names...
        bool foundName = false;
        for (AudioUnitScope i = 0; i < kNumScopes; ++i) {
            foundName = GetScope (i).HasElementWithName();
            if (foundName)
                break;
        }
            // OK - we found a name away we go...
        if (foundName) {
            CFMutableDictionaryRef nameDict = CFDictionaryCreateMutable (NULL, 0,
                                    &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
            for (AudioUnitScope i = 0; i < kNumScopes; ++i) {
                GetScope (i).AddElementNamesToDict (nameDict);
            }

            CFDictionarySetValue (dict, kElementNameString, nameDict);
            CFRelease (nameDict);
        }

    // we're done!!!
        *outData = dict;

        return noErr;
        */
    }
    
    pub fn restore_state(&mut self, plist: CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
            if (CFGetTypeID(plist) != CFDictionaryGetTypeID()) return kAudioUnitErr_InvalidPropertyValue;

        AudioComponentDescription desc = GetComponentDescription();

        CFDictionaryRef dict = static_cast<CFDictionaryRef>(plist);

    // zeroeth step - make sure the Part key is NOT present, as this method is used
    // to restore the GLOBAL state of the dictionary
        if (CFDictionaryContainsKey (dict, kPartString))
            return kAudioUnitErr_InvalidPropertyValue;

    // first step -> check the saved version in the data ref
    // at this point we're only dealing with version==0
        CFNumberRef cfnum = reinterpret_cast<CFNumberRef>(CFDictionaryGetValue (dict, kVersionString));
        if (cfnum == NULL) return kAudioUnitErr_InvalidPropertyValue;
        SInt32 value;
        CFNumberGetValue (cfnum, kCFNumberSInt32Type, &value);
        if (value != kCurrentSavedStateVersion) return kAudioUnitErr_InvalidPropertyValue;

    // second step -> check that this data belongs to this kind of audio unit
    // by checking the component subtype and manuID
    // We're not checking the type, since there may be different versions (effect, format-converter, offline)
    // of essentially the same AU
        cfnum = reinterpret_cast<CFNumberRef>(CFDictionaryGetValue (dict, kSubtypeString));
        if (cfnum == NULL) return kAudioUnitErr_InvalidPropertyValue;
        CFNumberGetValue (cfnum, kCFNumberSInt32Type, &value);
        if (UInt32(value) != desc.componentSubType) return kAudioUnitErr_InvalidPropertyValue;

        cfnum = reinterpret_cast<CFNumberRef>(CFDictionaryGetValue (dict, kManufacturerString));
        if (cfnum == NULL) return kAudioUnitErr_InvalidPropertyValue;
        CFNumberGetValue (cfnum, kCFNumberSInt32Type, &value);
        if (UInt32(value) != desc.componentManufacturer) return kAudioUnitErr_InvalidPropertyValue;

    // fourth step -> restore the state of all of the parameters for each scope and element
        CFDataRef data = reinterpret_cast<CFDataRef>(CFDictionaryGetValue (dict, kDataString));
        if (data != NULL)
        {
            const UInt8 *p, *pend;

            p = CFDataGetBytePtr(data);
            pend = p + CFDataGetLength(data);

            // we have a zero length data, which may just mean there were no parameters to save!
            //  if (p >= pend) return noErr;

            while (p < pend) {
                UInt32 scopeIdx = CFSwapInt32BigToHost(*(UInt32 *)p);
                p += sizeof(UInt32);

                AUScope &scope = GetScope(scopeIdx);
                p = scope.RestoreState(p);
            }
        }

    //OK - now we're going to do some properties
    //restore the preset name...
        CFStringRef name = reinterpret_cast<CFStringRef>(CFDictionaryGetValue (dict, kNameString));
        if (mCurrentPreset.presetName) CFRelease (mCurrentPreset.presetName);
        if (name)
        {
            mCurrentPreset.presetName = name;
            mCurrentPreset.presetNumber = -1;
        }
        else { // no name entry make the default one
            mCurrentPreset.presetName = kUntitledString;
            mCurrentPreset.presetNumber = -1;
        }

        CFRetain (mCurrentPreset.presetName);
    #if !CA_USE_AUDIO_PLUGIN_ONLY
    #ifndef __LP64__
        PropertyChanged(kAudioUnitProperty_CurrentPreset, kAudioUnitScope_Global, 0);
    #endif
    #endif
        PropertyChanged(kAudioUnitProperty_PresentPreset, kAudioUnitScope_Global, 0);

    // Does the dict contain render quality information?
        if (CFDictionaryGetValueIfPresent (dict, kRenderQualityString, reinterpret_cast<const void**>(&cfnum)))
        {
            CFNumberGetValue (cfnum, kCFNumberSInt32Type, &value);
            DispatchSetProperty (kAudioUnitProperty_RenderQuality,
                                    kAudioUnitScope_Global,
                                    0,
                                    &value,
                                    sizeof(value));
        }

    // Does the unit support the CPULoad Quality property - if so, save it...
        if (CFDictionaryGetValueIfPresent (dict, kCPULoadString, reinterpret_cast<const void**>(&cfnum)))
        {
            Float32 floatValue;
            CFNumberGetValue (cfnum, kCFNumberFloatType, &floatValue);
            DispatchSetProperty (6/*kAudioUnitProperty_CPULoad*/,
                                    kAudioUnitScope_Global,
                                    0,
                                    &floatValue,
                                    sizeof(floatValue));
        }

    // Do we have any element names for any of our scopes?
        CFDictionaryRef nameDict;
        if (CFDictionaryGetValueIfPresent (dict, kElementNameString, reinterpret_cast<const void**>(&nameDict)))
        {
            char string[64];
            for (int i = 0; i < kNumScopes; ++i)
            {
                snprintf (string, sizeof(string), "%d", i);
                CFStringRef key = CFStringCreateWithCString (NULL, string, kCFStringEncodingASCII);
                CFDictionaryRef elementDict;
                if (CFDictionaryGetValueIfPresent (nameDict, key, reinterpret_cast<const void**>(&elementDict)))
                {
                    bool didAddElements = GetScope (i).RestoreElementNames (elementDict);
                    if (didAddElements)
                        PropertyChanged (kAudioUnitProperty_ElementCount, i, 0);
                }
                CFRelease (key);
            }
        }

        return noErr;
        */
    }
    
    pub fn get_presets(&self, out_data: *mut CFArrayRef) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn new_factory_preset_set(&mut self, in_new_factory_preset: &AUPreset) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn new_custom_preset_set(&mut self, in_new_custom_preset: &AUPreset) -> OSStatus {
        
        todo!();
        /*
            CFRelease (mCurrentPreset.presetName);
        mCurrentPreset = inNewCustomPreset;
        CFRetain (mCurrentPreset.presetName);
        return noErr;
        */
    }

    /**
      | set the default preset for the unit ->
      | the number of the preset MUST be >=
      | 0 and the name should be valid, or the
      | preset WON'T take
      */
    pub fn set_afactory_preset_as_current(&mut self, in_preset: &AUPreset) -> bool {
        
        todo!();
        /*
            if (inPreset.presetNumber < 0 || inPreset.presetName == NULL) return false;
        CFRelease (mCurrentPreset.presetName);
        mCurrentPreset = inPreset;
        CFRetain (mCurrentPreset.presetName);
        return true;
        */
    }

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn get_num_custom_ui_components(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn get_ui_component_descs(&mut self, in_desc_array: *mut ComponentDescription)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn has_icon(&mut self) -> bool {
        
        todo!();
        /*
            #if !CA_NO_AU_UI_FEATURES
        CFURLRef url = CopyIconLocation();
        if (url) {
            CFRelease (url);
            return true;
        }
    #endif
        return false;
        */
    }
    
    pub fn copy_icon_location(&mut self) -> CFURLRef {
        
        todo!();
        /*
            return NULL;
        */
    }
    
    pub fn get_parameter_list(&mut self, 
        in_scope:           AudioUnitScope,
        out_parameter_list: *mut AudioUnitParameterID,
        out_num_parameters: &mut u32) -> OSStatus {
        
        todo!();
        /*
            AUScope &scope = GetScope(inScope);
        AUElement *elementWithMostParameters = NULL;
        UInt32 maxNumParams = 0;

        int nElems = scope.GetNumberOfElements();
        for (int ielem = 0; ielem < nElems; ++ielem) {
            AUElement *element = scope.GetElement(ielem);
            UInt32 nParams = element->GetNumberOfParameters();
            if (nParams > maxNumParams) {
                maxNumParams = nParams;
                elementWithMostParameters = element;
            }
        }

        if (outParameterList != NULL && elementWithMostParameters != NULL)
            elementWithMostParameters->GetParameterList(outParameterList);

        outNumParameters = maxNumParams;
        return noErr;
        */
    }
    
    pub fn get_parameter_info(&mut self, 
        in_scope:           AudioUnitScope,
        in_parameterid:     AudioUnitParameterID,
        out_parameter_info: &mut AudioUnitParameterInfo) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidParameter;
        */
    }
    
    pub fn get_parameter_value_strings(&mut self, 
        in_scope:       AudioUnitScope,
        in_parameterid: AudioUnitParameterID,
        out_strings:    *mut CFArrayRef) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn get_parameter_history_info(&mut self, 
        in_scope:                        AudioUnitScope,
        in_parameterid:                  AudioUnitParameterID,
        out_updates_per_second:          &mut Float32,
        out_history_duration_in_seconds: &mut Float32) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn copy_clump_name(&mut self, 
        in_scope:               AudioUnitScope,
        in_clumpid:             u32,
        in_desired_name_length: u32,
        out_clump_name:         *mut CFStringRef) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }
    
    pub fn set_number_of_elements(&mut self, 
        in_scope:     AudioUnitScope,
        num_elements: u32)  {
        
        todo!();
        /*
            if (inScope == kAudioUnitScope_Global && numElements != 1)
            COMPONENT_THROW(kAudioUnitErr_InvalidScope);

        GetScope(inScope).SetNumberOfElements(numElements);
        */
    }
    
    pub fn create_element(&mut self, 
        scope:   AudioUnitScope,
        element: AudioUnitElement) -> *mut AUElement {
        
        todo!();
        /*
            switch (scope) {
        case kAudioUnitScope_Global:
            return new AUElement(this);
        case kAudioUnitScope_Input:
            return new AUInputElement(this);
        case kAudioUnitScope_Output:
            return new AUOutputElement(this);
    #if !CA_BASIC_AU_FEATURES
        case kAudioUnitScope_Group:
            return new AUElement(this);
        case kAudioUnitScope_Part:
            return new AUElement(this);
    #endif
        }
        COMPONENT_THROW(kAudioUnitErr_InvalidScope);

        return NULL;    // get rid of compiler warning
        */
    }
    
    pub fn format_is_canonical(&mut self, f: &CAStreamBasicDescription) -> bool {
        
        todo!();
        /*
            return (f.mFormatID == kAudioFormatLinearPCM
            &&  f.mFramesPerPacket == 1
            &&  f.mBytesPerPacket == f.mBytesPerFrame
    //      &&  f.mChannelsPerFrame >= 0    -- this is always true since it's unsigned
            // so far, it's a valid PCM format
    #if CA_PREFER_FIXED_POINT
            &&  (f.mFormatFlags & kLinearPCMFormatFlagIsFloat) == 0
            &&  (((f.mFormatFlags & kLinearPCMFormatFlagsSampleFractionMask) >> kLinearPCMFormatFlagsSampleFractionShift) == kAudioUnitSampleFractionBits)
    #else
            &&  (f.mFormatFlags & kLinearPCMFormatFlagIsFloat) != 0
    #endif
            &&  ((f.mChannelsPerFrame == 1) || ((f.mFormatFlags & kAudioFormatFlagIsNonInterleaved) == 0) == (mAudioUnitAPIVersion == 1))
    #if TARGET_RT_BIG_ENDIAN
            &&  (f.mFormatFlags & kLinearPCMFormatFlagIsBigEndian) != 0
    #else
            &&  (f.mFormatFlags & kLinearPCMFormatFlagIsBigEndian) == 0
    #endif
            &&  f.mBitsPerChannel == 8 * sizeof(AudioUnitSampleType)
            &&  f.mBytesPerFrame == f.NumberInterleavedChannels() * sizeof(AudioUnitSampleType)
            );
        */
    }
    
    pub fn make_canonical_format(&mut self, 
        f:          &mut CAStreamBasicDescription,
        n_channels: i32)  {
        
        todo!();
        /*
            f.SetAUCanonical(nChannels, mAudioUnitAPIVersion < 2);  // interleaved for v1, non for v2
        f.mSampleRate = 0.0;
        */
    }

    pub fn get_logging_string(&self) -> *mut u8 {
        
        todo!();
        /*
            if (mLogString) return mLogString;

        AudioComponentDescription desc = GetComponentDescription();

        const size_t logStringSize = 256;
        const_cast<AUBase*>(this)->mLogString = new char[logStringSize];
        char str[24];
        char str1[24];
        char str2[24];
        snprintf (const_cast<AUBase*>(this)->mLogString, logStringSize, "AU (%p): %s %s %s",
            GetComponentInstance(),
            CAStringForOSType(desc.componentType, str, sizeof(str)),
            CAStringForOSType(desc.componentSubType, str1, sizeof(str1)),
            CAStringForOSType(desc.componentManufacturer, str2, sizeof(str2)));

        return mLogString;
        */
    }

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn component_entry_dispatch(&mut self, 
        params: *mut ComponentParameters,
        this:   *mut AUBase) -> OSStatus {
        
        todo!();
        /*
            if (This == NULL) return kAudio_ParamError;

        OSStatus result = noErr;

        switch (params->what) {
        case kComponentCanDoSelect:
            switch (GetSelectorForCanDo(params)) {
        // any selectors
                case kAudioUnitInitializeSelect:
                case kAudioUnitUninitializeSelect:
                case kAudioUnitGetPropertyInfoSelect:
                case kAudioUnitGetPropertySelect:
                case kAudioUnitSetPropertySelect:
                case kAudioUnitAddPropertyListenerSelect:
    #if (!__LP64__)
                case kAudioUnitRemovePropertyListenerSelect:
    #endif
                case kAudioUnitGetParameterSelect:
                case kAudioUnitSetParameterSelect:
                case kAudioUnitResetSelect:
                    result = 1;
                    break;
        // v1 selectors

        // v2 selectors
                case kAudioUnitRemovePropertyListenerWithUserDataSelect:
                case kAudioUnitAddRenderNotifySelect:
                case kAudioUnitRemoveRenderNotifySelect:
                case kAudioUnitScheduleParametersSelect:
                case kAudioUnitRenderSelect:
                    result = (This->AudioUnitAPIVersion() > 1);
                    break;

                default:
                    return ComponentBase::ComponentEntryDispatch(params, This);
            }
            break;

        case kAudioUnitInitializeSelect:
        {
            CAMutex::Locker lock2(This->GetMutex());
            result = This->DoInitialize();
        }
            break;

        case kAudioUnitUninitializeSelect:
        {
            CAMutex::Locker lock2(This->GetMutex());
            This->DoCleanup();
            result = noErr;
        }
            break;

        case kAudioUnitGetPropertyInfoSelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitPropertyID, pinID, 0, 5);
                PARAM(AudioUnitScope, pinScope, 1, 5);
                PARAM(AudioUnitElement, pinElement, 2, 5);
                PARAM(UInt32 *, poutDataSize, 3, 5);
                PARAM(Boolean *, poutWritable, 4, 5);

                // pass our own copies so that we assume responsibility for testing
                // the caller's pointers against null and our C++ classes can
                // always assume they're non-null
                UInt32 dataSize;
                Boolean writable;

                result = This->DispatchGetPropertyInfo(pinID, pinScope, pinElement, dataSize, writable);
                if (poutDataSize != NULL)
                    *poutDataSize = dataSize;
                if (poutWritable != NULL)
                    *poutWritable = writable;
            }
            break;

        case kAudioUnitGetPropertySelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitPropertyID, pinID, 0, 5);
                PARAM(AudioUnitScope, pinScope, 1, 5);
                PARAM(AudioUnitElement, pinElement, 2, 5);
                PARAM(void *, poutData, 3, 5);
                PARAM(UInt32 *, pioDataSize, 4, 5);

                UInt32 actualPropertySize, clientBufferSize;
                Boolean writable;
                char *tempBuffer;
                void *destBuffer;

                if (pioDataSize == NULL) {
                    ca_debug_string("AudioUnitGetProperty: null size pointer");
                    result = kAudio_ParamError;
                    goto finishGetProperty;
                }
                if (poutData == NULL) {
                    UInt32 dataSize;

                    result = This->DispatchGetPropertyInfo(pinID, pinScope, pinElement, dataSize, writable);
                    *pioDataSize = dataSize;
                    goto finishGetProperty;
                }

                clientBufferSize = *pioDataSize;
                if (clientBufferSize == 0)
                {
                    ca_debug_string("AudioUnitGetProperty: *ioDataSize == 0 on entry");
                    // $$$ or should we allow this as a shortcut for finding the size?
                    result = kAudio_ParamError;
                    goto finishGetProperty;
                }

                result = This->DispatchGetPropertyInfo(pinID, pinScope, pinElement,
                                                        actualPropertySize, writable);
                if (result)
                    goto finishGetProperty;

                if (clientBufferSize < actualPropertySize)
                {
                    tempBuffer = new char[actualPropertySize];
                    destBuffer = tempBuffer;
                } else {
                    tempBuffer = NULL;
                    destBuffer = poutData;
                }

                result = This->DispatchGetProperty(pinID, pinScope, pinElement, destBuffer);

                if (result == noErr) {
                    if (clientBufferSize < actualPropertySize && tempBuffer != NULL)
                    {
                        memcpy(poutData, tempBuffer, clientBufferSize);
                        delete[] tempBuffer;
                        // pioDataSize remains correct, the number of bytes we wrote
                    } else
                        *pioDataSize = actualPropertySize;
                } else
                    *pioDataSize = 0;

                finishGetProperty:
                    ;

            }
            break;

        case kAudioUnitSetPropertySelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitPropertyID, pinID, 0, 5);
                PARAM(AudioUnitScope, pinScope, 1, 5);
                PARAM(AudioUnitElement, pinElement, 2, 5);
                PARAM(const void *, pinData, 3, 5);
                PARAM(UInt32, pinDataSize, 4, 5);

                if (pinData && pinDataSize)
                    result = This->DispatchSetProperty(pinID, pinScope, pinElement, pinData, pinDataSize);
                else {
                    if (pinData == NULL && pinDataSize == 0) {
                        result = This->DispatchRemovePropertyValue (pinID, pinScope, pinElement);
                    } else {
                        if (pinData == NULL) {
                            ca_debug_string("AudioUnitSetProperty: inData == NULL");
                            result = kAudio_ParamError;
                            goto finishSetProperty;
                        }

                        if (pinDataSize == 0) {
                            ca_debug_string("AudioUnitSetProperty: inDataSize == 0");
                            result = kAudio_ParamError;
                            goto finishSetProperty;
                        }
                    }
                }
                finishSetProperty:
                        ;

            }
            break;

        case kAudioUnitAddPropertyListenerSelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitPropertyID, pinID, 0, 3);
                PARAM(AudioUnitPropertyListenerProc, pinProc, 1, 3);
                PARAM(void *, pinProcRefCon, 2, 3);
                result = This->AddPropertyListener(pinID, pinProc, pinProcRefCon);
            }
            break;

    #if (!__LP64__)
        case kAudioUnitRemovePropertyListenerSelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitPropertyID, pinID, 0, 2);
                PARAM(AudioUnitPropertyListenerProc, pinProc, 1, 2);
                result = This->RemovePropertyListener(pinID, pinProc, NULL, false);
            }
            break;
    #endif

        case kAudioUnitRemovePropertyListenerWithUserDataSelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitPropertyID, pinID, 0, 3);
                PARAM(AudioUnitPropertyListenerProc, pinProc, 1, 3);
                PARAM(void *, pinProcRefCon, 2, 3);
                result = This->RemovePropertyListener(pinID, pinProc, pinProcRefCon, true);
            }
            break;

        case kAudioUnitAddRenderNotifySelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AURenderCallback, pinProc, 0, 2);
                PARAM(void *, pinProcRefCon, 1, 2);
                result = This->SetRenderNotification (pinProc, pinProcRefCon);
            }
            break;

        case kAudioUnitRemoveRenderNotifySelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AURenderCallback, pinProc, 0, 2);
                PARAM(void *, pinProcRefCon, 1, 2);
                result = This->RemoveRenderNotification (pinProc, pinProcRefCon);
            }
            break;

        case kAudioUnitGetParameterSelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitParameterID, pinID, 0, 4);
                PARAM(AudioUnitScope, pinScope, 1, 4);
                PARAM(AudioUnitElement, pinElement, 2, 4);
                PARAM(AudioUnitParameterValue *, poutValue, 3, 4);
                result = (poutValue == NULL ? kAudio_ParamError : This->GetParameter(pinID, pinScope, pinElement, *poutValue));
            }
            break;

        case kAudioUnitSetParameterSelect:
            {
                CAMutex::Locker lock(This->GetMutex()); // is this realtime or no???
                PARAM(AudioUnitParameterID, pinID, 0, 5);
                PARAM(AudioUnitScope, pinScope, 1, 5);
                PARAM(AudioUnitElement, pinElement, 2, 5);
                PARAM(AudioUnitParameterValue, pinValue, 3, 5);
                PARAM(UInt32, pinBufferOffsetInFrames, 4, 5);
                result = This->SetParameter(pinID, pinScope, pinElement, pinValue, pinBufferOffsetInFrames);
            }
            break;

        case kAudioUnitScheduleParametersSelect:
            {
                CAMutex::Locker lock(This->GetMutex()); // is this realtime or no???
                if (This->AudioUnitAPIVersion() > 1)
                {
                    PARAM(AudioUnitParameterEvent *, pinParameterEvent, 0, 2);
                    PARAM(UInt32, pinNumParamEvents, 1, 2);
                    result = This->ScheduleParameter (pinParameterEvent, pinNumParamEvents);
                } else
                    result = badComponentSelector;
            }
            break;


        case kAudioUnitRenderSelect:
            {
                // realtime; no lock
                {
                    PARAM(AudioUnitRenderActionFlags *, pinActionFlags, 0, 5);
                    PARAM(const AudioTimeStamp *, pinTimeStamp, 1, 5);
                    PARAM(UInt32, pinOutputBusNumber, 2, 5);
                    PARAM(UInt32, pinNumberFrames, 3, 5);
                    PARAM(AudioBufferList *, pioData, 4, 5);
                    AudioUnitRenderActionFlags tempFlags;

                    if (pinTimeStamp == NULL || pioData == NULL)
                        result = kAudio_ParamError;
                    else {
                        if (pinActionFlags == NULL) {
                            tempFlags = 0;
                            pinActionFlags = &tempFlags;
                        }
                        result = This->DoRender(*pinActionFlags, *pinTimeStamp, pinOutputBusNumber, pinNumberFrames, *pioData);
                    }
                }
            }
            break;

        case kAudioUnitResetSelect:
            {
                CAMutex::Locker lock(This->GetMutex());
                PARAM(AudioUnitScope, pinScope, 0, 2);
                PARAM(AudioUnitElement, pinElement, 1, 2);
                This->ResetRenderTime();
                result = This->Reset(pinScope, pinElement);
            }
            break;

        default:
            result = ComponentBase::ComponentEntryDispatch(params, This);
            break;
        }

        return result;
        */
    }
}

