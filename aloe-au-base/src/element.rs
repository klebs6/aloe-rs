crate::ix!();

pub trait SetStreamFormat {

    fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus;
}

pub trait NeedsBufferSpace {

    fn needs_buffer_space(&self) -> bool;
}

pub trait AUElementInterface: 
SetStreamFormat 
+ NeedsBufferSpace 
{
    fn get_number_of_parameters(&mut self) -> u32;

    fn get_parameter_list(&mut self, out_list: *mut AudioUnitParameterID);

    fn use_indexed_parameters(&mut self, in_number_of_parameters: i32);

    fn as_io_element(&mut self) -> *mut AUIOElement;
}

pub struct AUElement {
    audio_unit:             *mut AUBase,
    parameters:             AUElementParameterMap,
    use_indexed_parameters: bool,
    indexed_parameters:     Vec<ParameterMapEvent>,
    element_name:           CFStringRef,
}

pub struct Less<T> { _p0: PhantomData<T>, }

pub type AUElementParameterMap = HashMap<AudioUnitParameterID,ParameterMapEvent,Less<AudioUnitParameterID>>;

impl Drop for AUElement {

    fn drop(&mut self) {
        todo!();
        /*
            if (mElementName) CFRelease (mElementName);
        */
    }
}

impl NeedsBufferSpace for AUElement {

    fn needs_buffer_space(&self) -> bool {
        todo!();
    }
}

impl SetStreamFormat for AUElement {

    fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus {
        todo!();
    }
}

impl AUElementInterface for AUElement {

    fn get_number_of_parameters(&mut self) -> u32 {
        
        todo!();
        /*
            if(mUseIndexedParameters) return static_cast<UInt32>(mIndexedParameters.size()); else return static_cast<UInt32>(mParameters.size());
        */
    }

    fn get_parameter_list(&mut self, out_list: *mut AudioUnitParameterID)  {
        
        todo!();
        /*
        
        */
    }

    fn use_indexed_parameters(&mut self, in_number_of_parameters: i32)  {
        
        todo!();
        /*
        
        */
    }

    fn as_io_element(&mut self) -> *mut AUIOElement {
        
        todo!();
        /*
            return NULL;
        */
    }
}

impl AUElement {

    pub fn new(audio_unit: *mut AUBase) -> Self {
    
        todo!();
        /*
        : audio_unit(audioUnit),
        : use_indexed_parameters(false),
        : element_name(0),

        
        */
    }
    
    pub fn get_audio_unit(&self) -> *mut AUBase {
        
        todo!();
        /*
            return mAudioUnit; }{
        */
    }
    
    pub fn get_name(&self) -> CFStringRef {
        
        todo!();
        /*
            return mElementName;
        */
    }
    
    pub fn has_name(&self) -> bool {
        
        todo!();
        /*
            return mElementName != 0;
        */
    }
    
    /**
      |  By default, parameterIDs may be arbitrarily
      |  spaced, and an STL map will be used for
      |  access.  Calling UseIndexedParameters() will
      |  instead use an STL vector for faster indexed
      |  access.
      |
      |  This assumes the paramIDs are numbered
      |  0.....inNumberOfParameters-1
      |
      |  Call this before defining/adding any
      |  parameters with SetParameter()
      |
      */
    pub fn use_indexed_parameters(&mut self, in_number_of_parameters: i32)  {
        
        todo!();
        /*
            mIndexedParameters.resize (inNumberOfParameters);
        mUseIndexedParameters = true;
        */
    }
    
    /**
      |  Helper method.
      |
      |  returns the ParameterMapEvent object
      |  associated with the paramID
      |
      */
    #[inline] pub fn get_param_event(&mut self, paramid: AudioUnitParameterID) -> &mut ParameterMapEvent {
        
        todo!();
        /*
            ParameterMapEvent *event;

        if(mUseIndexedParameters)
        {
            if(paramID >= mIndexedParameters.size() )
                COMPONENT_THROW(kAudioUnitErr_InvalidParameter);

            event = &mIndexedParameters[paramID];
        }
        else
        {
            ParameterMap::iterator i = mParameters.find(paramID);
            if (i == mParameters.end())
                COMPONENT_THROW(kAudioUnitErr_InvalidParameter);

            event = &(*i).second;
        }

        return *event;
        */
    }

    /**
      |  Helper method.
      |
      |  returns whether the specified paramID is known
      |  to the element
      |
      */
    pub fn has_parameterid(&self, paramid: AudioUnitParameterID) -> bool {
        
        todo!();
        /*
            if(mUseIndexedParameters)
        {
            if(paramID >= mIndexedParameters.size() )
                return false;

            return true;
        }

        ParameterMap::const_iterator i = mParameters.find(paramID);
        if (i == mParameters.end())
            return false;

        return true;
        */
    }

    /**
      |  caller assumes that this is actually an
      |  immediate parameter
      |
      */
    pub fn get_parameter(&mut self, paramid: AudioUnitParameterID) -> AudioUnitParameterValue {
        
        todo!();
        /*
            ParameterMapEvent &event = GetParamEvent(paramID);

        return event.GetValue();
        */
    }
    
    /**
      | interpolates the start and end values
      | corresponding to the current processing
      | slice most ramp parameter implementations
      | will want to use this method
      */
    pub fn get_ramp_slice_start_end(&mut self, 
        paramid:                   AudioUnitParameterID,
        out_start_value:           &mut AudioUnitParameterValue,
        out_end_value:             &mut AudioUnitParameterValue,
        out_value_per_frame_delta: &mut AudioUnitParameterValue)  {
        
        todo!();
        /*
            ParameterMapEvent &event = GetParamEvent(paramID);

        // works even if the value is constant (immediate parameter value)
        event.GetRampSliceStartEnd(outStartValue, outEndValue, outValuePerFrameDelta );
        */
    }
    
    pub fn get_end_value(&mut self, paramid: AudioUnitParameterID) -> AudioUnitParameterValue {
        
        todo!();
        /*
            ParameterMapEvent &event = GetParamEvent(paramID);

        // works even if the value is constant (immediate parameter value)
        return event.GetEndValue();
        */
    }
    
    /**
      | Only set okWhenInitialized to true when you
      | know the outside world cannot access this
      | element. Otherwise the parameter map could
      | get corrupted.
      */
    pub fn set_parameter(
        &mut self, 
        paramid:             AudioUnitParameterID,
        in_value:            AudioUnitParameterValue,
        ok_when_initialized: Option<bool>
    ) {

        let ok_when_initialized: bool = ok_when_initialized.unwrap_or(false);
        
        todo!();
        /*
            if(mUseIndexedParameters)
        {
            ParameterMapEvent &event = GetParamEvent(paramID);
            event.SetValue(inValue);
        }
        else
        {
            ParameterMap::iterator i = mParameters.find(paramID);

            if (i == mParameters.end())
            {
                if (mAudioUnit->IsInitialized() && !okWhenInitialized) {
                    // The AU should not be creating new parameters once initialized.
                    // If a client tries to set an undefined parameter, we could throw as follows,
                    // but this might cause a regression. So it is better to just fail silently.
                    // COMPONENT_THROW(kAudioUnitErr_InvalidParameter);
    #if DEBUG
                    fprintf(stderr, "WARNING: %s SetParameter for undefined param ID %d while initialized. Ignoring..\n",
                                    mAudioUnit->GetLoggingString(), (int)paramID);
    #endif
                } else {
                    // create new entry in map for the paramID (only happens first time)
                    ParameterMapEvent event(inValue);
                    mParameters[paramID] = event;
                }
            }
            else
            {
                // paramID already exists in map so simply change its value
                ParameterMapEvent &event = (*i).second;
                event.SetValue(inValue);
            }
        }
        */
    }
    
    /**
      | Only set okWhenInitialized to true when you
      | know the outside world cannot access this
      | element. Otherwise the parameter map could
      | get corrupted.
      */
    pub fn set_scheduled_event(
        &mut self, 
        paramid:                   AudioUnitParameterID,
        in_event:                  &AudioUnitParameterEvent,
        in_slice_offset_in_buffer: u32,
        in_slice_duration_frames:  u32,
        ok_when_initialized:       Option<bool>
    ) {

        let ok_when_initialized: bool = ok_when_initialized.unwrap_or(false );
        
        todo!();
        /*
            if(mUseIndexedParameters)
        {
            ParameterMapEvent &event = GetParamEvent(paramID);
            event.SetScheduledEvent(inEvent, inSliceOffsetInBuffer, inSliceDurationFrames );
        }
        else
        {
            ParameterMap::iterator i = mParameters.find(paramID);

            if (i == mParameters.end())
            {
                if (mAudioUnit->IsInitialized() && !okWhenInitialized) {
                    // The AU should not be creating new parameters once initialized.
                    // If a client tries to set an undefined parameter, we could throw as follows,
                    // but this might cause a regression. So it is better to just fail silently.
                    // COMPONENT_THROW(kAudioUnitErr_InvalidParameter);
    #if DEBUG
                    fprintf(stderr, "WARNING: %s SetScheduledEvent for undefined param ID %d while initialized. Ignoring..\n",
                                    mAudioUnit->GetLoggingString(), (int)paramID);
    #endif
                } else {
                    // create new entry in map for the paramID (only happens first time)
                    ParameterMapEvent event(inEvent, inSliceOffsetInBuffer, inSliceDurationFrames);
                    mParameters[paramID] = event;
                }
            }
            else
            {
                // paramID already exists in map so simply change its value
                ParameterMapEvent &event = (*i).second;

                event.SetScheduledEvent(inEvent, inSliceOffsetInBuffer, inSliceDurationFrames );
            }
        }
        */
    }
    
    pub fn get_parameter_list(&mut self, out_list: *mut AudioUnitParameterID)  {
        
        todo!();
        /*
            if(mUseIndexedParameters)
        {
            UInt32 nparams = static_cast<UInt32>(mIndexedParameters.size());
            for (UInt32 i = 0; i < nparams; i++ )
                *outList++ = (AudioUnitParameterID)i;
        }
        else
        {
            for (ParameterMap::iterator i = mParameters.begin(); i != mParameters.end(); ++i)
                *outList++ = (*i).first;
        }
        */
    }
    
    pub fn save_state(&mut self, data: CFMutableDataRef)  {
        
        todo!();
        /*
            if(mUseIndexedParameters)
        {
            UInt32 nparams = static_cast<UInt32>(mIndexedParameters.size());
            UInt32 theData = CFSwapInt32HostToBig(nparams);
            CFDataAppendBytes(data, (UInt8 *)&theData, sizeof(nparams));

            for (UInt32 i = 0; i < nparams; i++)
            {
                struct {
                    UInt32              paramID;
                    //CFSwappedFloat32  value; crashes gcc3 PFE
                    UInt32              value;  // really a big-endian float
                } entry;

                entry.paramID = CFSwapInt32HostToBig(i);

                AudioUnitParameterValue v = mIndexedParameters[i].GetValue();
                entry.value = CFSwapInt32HostToBig(*(UInt32 *)&v );

                CFDataAppendBytes(data, (UInt8 *)&entry, sizeof(entry));
            }
        }
        else
        {
            UInt32 nparams = CFSwapInt32HostToBig(static_cast<uint32_t>(mParameters.size()));
            CFDataAppendBytes(data, (UInt8 *)&nparams, sizeof(nparams));

            for (ParameterMap::iterator i = mParameters.begin(); i != mParameters.end(); ++i) {
                struct {
                    UInt32              paramID;
                    //CFSwappedFloat32  value; crashes gcc3 PFE
                    UInt32              value;  // really a big-endian float
                } entry;

                entry.paramID = CFSwapInt32HostToBig((*i).first);

                AudioUnitParameterValue v = (*i).second.GetValue();
                entry.value = CFSwapInt32HostToBig(*(UInt32 *)&v );

                CFDataAppendBytes(data, (UInt8 *)&entry, sizeof(entry));
            }
        }
        */
    }
    
    pub fn restore_state(&mut self, state: *const UInt8) -> *const UInt8 {
        
        todo!();
        /*
            union FloatInt32 { UInt32 i; AudioUnitParameterValue f; };
        const UInt8 *p = state;
        UInt32 nparams = CFSwapInt32BigToHost(*(UInt32 *)p);
        p += sizeof(UInt32);

        for (UInt32 i = 0; i < nparams; ++i) {
            struct {
                AudioUnitParameterID        paramID;
                AudioUnitParameterValue     value;
            } entry;

            entry.paramID = CFSwapInt32BigToHost(*(UInt32 *)p);
            p += sizeof(UInt32);
            FloatInt32 temp;
            temp.i = CFSwapInt32BigToHost(*(UInt32 *)p);
            entry.value = temp.f;
            p += sizeof(AudioUnitParameterValue);

            SetParameter(entry.paramID, entry.value);
        }
        return p;
        */
    }
    
    pub fn set_name(&mut self, in_name: CFStringRef)  {
        
        todo!();
        /*
            if (mElementName) CFRelease (mElementName);
        mElementName = inName;
        if (mElementName) CFRetain (mElementName);
        */
    }
}
