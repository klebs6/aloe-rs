crate::ix!();

pub enum AUCarbonViewControlType {
    kTypeContinuous,    // e.g. slider
    kTypeDiscrete,      // e.g. pop-up menu
    kTypeText
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUCarbonViewControl.h]

/**
  |  AUCarbonViewControl
  |
  |  Wrapper for a control that is wired to an
  |  AudioUnit parameter.
  */
pub struct AUCarbonViewControl {
    base:                      CarbonEventHandler,
    owner_view:                *mut AUCarbonViewBase,
    listener:                  AUParameterListenerRef,
    ty:                        AUCarbonViewControlType,
    param:                     CAAUParameter,
    control:                   ControlRef,
    in_control_initialization: i16,
}

impl Into<ControlRef> for AUCarbonViewControl {

    #[inline] fn into(self) -> ControlRef {
        todo!();
        /*
            return mControl;
        */
    }
}

impl Drop for AUCarbonViewControl {

    fn drop(&mut self) {
        todo!();
        /*
            AUListenerRemoveParameter(mListener, this, &mParam);
        */
    }
}

lazy_static!{
    /*
    AUCarbonViewControl* AUCarbonViewControl::mLastControl = NULL;
    */
}

impl AUCarbonViewControl {
    
    pub fn new(
        owner_view: *mut AUCarbonViewBase,
        listener:   AUParameterListenerRef,
        ty:         AUCarbonViewControlType,
        param:      &CAAUParameter,
        control:    ControlRef) -> Self {
    
        todo!();
        /*
        : owner_view(ownerView),
        : listener(listener),
        : ty(type),
        : param(param),
        : control(control),
        : in_control_initialization(0),

            #if !__LP64__
        SetControlReference(control, SRefCon(this));
    #endif
        */
    }
    
    pub fn bind(&mut self)  {
        
        todo!();
        /*
            #if !__LP64__
        mInControlInitialization = 1;   // true
        AUListenerAddParameter(mListener, this, &mParam);
            // will cause an almost-immediate callback

        EventTypeSpec events[] = {
            { kEventClassControl, kEventControlValueFieldChanged }  // N.B. OS X only
        };

        WantEventTypes(GetControlEventTarget(mControl), GetEventTypeCount(events), events);

        if (mType == kTypeContinuous || mType == kTypeText || mType == kTypeDiscrete) {
            EventTypeSpec controlEvents[] = {
                { kEventClassControl, kEventControlHit },
                { kEventClassControl, kEventControlClick },
                { kEventClassControl, kEventControlTrack }
            };
            WantEventTypes(GetControlEventTarget(mControl), GetEventTypeCount(controlEvents), controlEvents);
        }

        if (mType == kTypeText) {
            EventTypeSpec controlFocusEvents[] = {
                { kEventClassControl, kEventControlSetFocusPart }
            };
            WantEventTypes(GetControlEventTarget(mControl), GetEventTypeCount(controlFocusEvents), controlFocusEvents);
            ControlKeyFilterUPP proc = mParam.ValuesHaveStrings() ? StdKeyFilterCallback : NumericKeyFilterCallback;
                // this will fail for a static text field
            SetControlData(mControl, 0, kControlEditTextKeyFilterTag, sizeof(proc), &proc);
        }

        Update(true);
        mInControlInitialization = 0;   // false
    #endif
        */
    }
    
    pub fn parameter_to_control(&mut self, param_value: f32)  {
        
        todo!();
        /*
            #if !__LP64__
        ++mInControlInitialization;
        switch (mType) {
        case kTypeContinuous:
            SetValueFract(AUParameterValueToLinear(paramValue, &mParam));
            break;
        case kTypeDiscrete:
            {
                long value = long(paramValue);

                // special case [1] -- menu parameters
                if (mParam.HasNamedParams()) {
                    // if we're dealing with menus they behave differently!
                    // becaue setting min and max doesn't work correctly for the control value
                    // first menu item always reports a control value of 1
                    ControlKind ctrlKind;
                    if (GetControlKind(mControl, &ctrlKind) == noErr) {
                        if ((ctrlKind.kind == kControlKindPopupArrow)
                            || (ctrlKind.kind == kControlKindPopupButton))
                        {
                            value = value - long(mParam.ParamInfo().minValue) + 1;
                        }
                    }
                }

                // special case [2] -- Write-only boolean parameters
                AudioUnitParameterInfo AUPI = mParam.ParamInfo();

                bool isWriteOnlyBoolParameter = (   (AUPI.unit == kAudioUnitParameterUnit_Boolean) &&
                                                    (AUPI.flags & kAudioUnitParameterFlag_IsWritable) &&
                                                    !(AUPI.flags & kAudioUnitParameterFlag_IsReadable)  );
                if (!isWriteOnlyBoolParameter) {
                    SetValue (value);
                }
            }
            break;
        case kTypeText:
            {
                CFStringRef cfstr = mParam.GetStringFromValueCopy(&paramValue);

                if ( !(mParam.ParamInfo().flags & kAudioUnitParameterFlag_IsWritable)           //READ ONLY PARAMS
                        && (mParam.ParamInfo().flags & kAudioUnitParameterFlag_IsReadable))
                {
                    if (mParam.GetParamTag()) {
                        CFMutableStringRef str = CFStringCreateMutableCopy(NULL, 256, cfstr);
                        CFRelease (cfstr);
                        CFStringAppend (str, CFSTR(" "));
                        CFStringAppend (str, mParam.GetParamTag());
                        cfstr = str;
                    }
                }
                SetTextValue(cfstr);
                CFRelease (cfstr);
            }
            break;
        }
        --mInControlInitialization;
    #endif
        */
    }
    
    pub fn control_to_parameter(&mut self)  {
        
        todo!();
        /*
            #if !__LP64__
        if (mInControlInitialization)
            return;

        switch (mType) {
        case kTypeContinuous:
            {
                double controlValue = GetValueFract();
                Float32 paramValue = AUParameterValueFromLinear(controlValue, &mParam);
                mParam.SetValue(mListener, this, paramValue);
            }
            break;
        case kTypeDiscrete:
            {
                long value = GetValue();

                // special case [1] -- Menus
                if (mParam.HasNamedParams()) {
                    // if we're dealing with menus they behave differently!
                    // becaue setting min and max doesn't work correctly for the control value
                    // first menu item always reports a control value of 1
                    ControlKind ctrlKind;
                    if (GetControlKind(mControl, &ctrlKind) == noErr) {
                        if ((ctrlKind.kind == kControlKindPopupArrow)
                            || (ctrlKind.kind == kControlKindPopupButton))
                        {
                            value = value + long(mParam.ParamInfo().minValue) - 1;
                        }
                    }
                }

                // special case [2] -- Write-only boolean parameters
                AudioUnitParameterInfo AUPI = mParam.ParamInfo();

                bool isWriteOnlyBoolParameter = (   (AUPI.unit == kAudioUnitParameterUnit_Boolean) &&
                                                    (AUPI.flags & kAudioUnitParameterFlag_IsWritable) &&
                                                    !(AUPI.flags & kAudioUnitParameterFlag_IsReadable)  );
                if (isWriteOnlyBoolParameter) {
                    value = 1;
                }

                mParam.SetValue (mListener, this, value);
            }
            break;
        case kTypeText:
            {
                Float32 val = mParam.GetValueFromString (GetTextValue());
                mParam.SetValue(mListener, this, (mParam.IsIndexedParam() ? (int)val : val));
                if (mParam.ValuesHaveStrings())
                    ParameterToControl(val); //make sure we display the correct text (from the AU)
            }
            break;
        }
    #endif
        */
    }
    
    pub fn set_value_fract(&mut self, value: f64)  {
        
        todo!();
        /*
            #if !__LP64__
        SInt32 minimum = GetControl32BitMinimum(mControl);
        SInt32 maximum = GetControl32BitMaximum(mControl);
        SInt32 cval = SInt32(value * (maximum - minimum) + minimum + 0.5);
        SetControl32BitValue(mControl, cval);
    //  printf("set: value=%lf, min=%ld, max=%ld, ctl value=%ld\n", value, minimum, maximum, cval);
    #endif
        */
    }
    
    pub fn get_value_fract(&mut self) -> f64 {
        
        todo!();
        /*
            #if !__LP64__
        SInt32 minimum = GetControl32BitMinimum(mControl);
        SInt32 maximum = GetControl32BitMaximum(mControl);
        SInt32 cval = GetControl32BitValue(mControl);
        double result = double(cval - minimum) / double(maximum - minimum);
    //  printf("get: min=%ld, max=%ld, value=%ld, result=%f\n", minimum, maximum, cval, result);
        return result;
    #else
        return 0;
    #endif
        */
    }
    
    pub fn set_text_value(&mut self, cfstr: CFStringRef)  {
        
        todo!();
        /*
            #if !__LP64__
        verify_noerr(SetControlData(mControl, 0, kControlEditTextCFStringTag, sizeof(CFStringRef), &cfstr));
    #endif
        */
    }
    
    pub fn get_text_value(&mut self) -> CFStringRef {
        
        todo!();
        /*
            #if !__LP64__
        CFStringRef cfstr;
        verify_noerr(GetControlData(mControl, 0, kControlEditTextCFStringTag, sizeof(CFStringRef), &cfstr, NULL));
        return cfstr;
    #else
        return CFSTR("");
    #endif
        */
    }
    
    pub fn set_value(&mut self, value: i64)  {
        
        todo!();
        /*
            #if !__LP64__
        SetControl32BitValue(mControl, value);
    #endif
        */
    }
    
    pub fn get_value(&mut self) -> i64 {
        
        todo!();
        /*
            #if !__LP64__
        return GetControl32BitValue(mControl);
    #else
        return 0;
    #endif
        */
    }

    /** 
      | Notes on event handling
      |
      |   Button (Click and release on button)
      |       kEventControlClick received
      |       kEventControlTrack received
      |       kEventControlValueFieldChanged received
      |       kEventControlHit received
      |
      |   Button (Click and release outside of button bounds)
      |       kEventControlClick received
      |       kEventControlTrack received
      |
      |   Slider (Click, drag, and release)
      |       kEventControlClick received
      |       kEventControlTrack received
      |       kEventControlValueFieldChanged received
      |       kEventControlValueFieldChanged received
      |       kEventControlHit received
      |
      |   Slider (Click, release without changing value)
      |       kEventControlClick received
      |       kEventControlTrack received
      */
    pub fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool {
        
        todo!();
        /*
            UInt32 eclass = GetEventClass(event);
        UInt32 ekind = GetEventKind(event);
        ControlRef control;
        bool        handled = true;

        switch (eclass) {
            case kEventClassControl:
            {
                AudioUnitParameterInfo AUPI = mParam.ParamInfo();

                bool isWriteOnlyBoolParameter = (   (AUPI.unit == kAudioUnitParameterUnit_Boolean) &&
                                                    (AUPI.flags & kAudioUnitParameterFlag_IsWritable) &&
                                                    !(AUPI.flags & kAudioUnitParameterFlag_IsReadable)  );

                switch (ekind) {
                    case kEventControlSetFocusPart: // tab
                        handled = !handled;     // fall through to next case
                        mLastControl = this;
                    case kEventControlValueFieldChanged:
                        GetEventParameter(event, kEventParamDirectObject, typeControlRef, NULL, sizeof(ControlRef), NULL, &control);
                        verify(control == mControl);
                        ControlToParameter();
                        return handled;
                    case kEventControlClick:
                        if (isWriteOnlyBoolParameter) {
                            GetEventParameter(event, kEventParamDirectObject, typeControlRef, NULL, sizeof(ControlRef), NULL, &control);
                            verify(control == mControl);
                            ControlToParameter();
                        } else if (mLastControl != this) {
                            if (mLastControl != NULL) {
                                mLastControl->Update(false);
                            }
                            mLastControl = this;
                        }
                        mOwnerView->TellListener(mParam, kAudioUnitCarbonViewEvent_MouseDownInControl, NULL);
                        break;  // don't return true, continue normal processing
                    case kEventControlHit:
                        if (mLastControl != this) {
                            if (mLastControl != NULL)
                                mLastControl->Update(false);
                            mLastControl = this;
                        }
                        mOwnerView->TellListener(mParam, kAudioUnitCarbonViewEvent_MouseUpInControl, NULL);
                        break;  // don't return true, continue normal processing
                    case kEventControlTrack:
                        if (mLastControl != this) {
                            if (mLastControl != NULL)
                                mLastControl->Update(false);
                            mLastControl = this;
                        }

                        CallNextEventHandler(inHandlerRef, event);
                        ControlToParameter();                       // new code
                        mOwnerView->TellListener(mParam, kAudioUnitCarbonViewEvent_MouseUpInControl, NULL);
                        // old code:
                        //      break;  // don't return true, continue normal processing

                        return handled; // don't return true, continue normal processing
                }
            }
        }
        return !handled;
        */
    }
    
    pub fn slider_track_proc(&mut self, 
        the_control: ControlRef,
        part_code:   ControlPartCode)  {
        
        todo!();
        /*
            // this doesn't need to actually do anything
    //  AUCarbonViewControl *This = (AUCarbonViewControl *)GetControlReference(theControl);
        */
    }
    
    pub fn std_key_filter_callback(&mut self, 
        the_control: ControlRef,
        key_code:    *mut i16,
        char_code:   *mut i16,
        modifiers:   *mut EventModifiers) -> ControlKeyFilterResult {
        
        todo!();
        /*
            #if !__LP64__
        SInt16 c = *charCode;
        if (c >= ' ' || c == '\b' || c == 0x7F || (c >= 0x1c && c <= 0x1f) || c == '\t')
            return kControlKeyFilterPassKey;
        if (c == '\r' || c == 3) {  // return or Enter
            AUCarbonViewControl *This = (AUCarbonViewControl *)GetControlReference(theControl);
            ControlEditTextSelectionRec sel = { 0, 32767 };
            SetControlData(This->mControl, 0, kControlEditTextSelectionTag, sizeof(sel), &sel);
            This->ControlToParameter();
        }
    #endif
        return kControlKeyFilterBlockKey;
        */
    }
    
    pub fn numeric_key_filter_callback(&mut self, 
        the_control: ControlRef,
        key_code:    *mut i16,
        char_code:   *mut i16,
        modifiers:   *mut EventModifiers) -> ControlKeyFilterResult {
        
        todo!();
        /*
            #if !__LP64__
        SInt16 c = *charCode;
        if (isdigit(c) || c == '+' || c == '-' || c == '.' || c == '\b' || c == 0x7F || (c >= 0x1c && c <= 0x1f)
        || c == '\t')
            return kControlKeyFilterPassKey;
        if (c == '\r' || c == 3) {  // return or Enter
            AUCarbonViewControl *This = (AUCarbonViewControl *)GetControlReference(theControl);
            ControlEditTextSelectionRec sel = { 0, 32767 };
            SetControlData(This->mControl, 0, kControlEditTextSelectionTag, sizeof(sel), &sel);
            This->ControlToParameter();
        }
    #endif
        return kControlKeyFilterBlockKey;
        */
    }
    
    pub fn size_control_to_fit(&mut self, 
        in_control: ControlRef,
        out_width:  *mut i16,
        out_height: *mut i16) -> bool {
        
        todo!();
        /*
            #if !__LP64__
        if (inControl == 0) return false;

        Boolean bValue = false;
        // this only works on text controls -- returns an error for other controls, but doesn't do anything,
        // so the error is irrelevant
        SetControlData(inControl, kControlEntireControl, 'stim' /* kControlStaticTextIsMultilineTag */, sizeof(Boolean), &bValue);

        SInt16 baseLineOffset;
        Rect bestRect;
        OSErr err = GetBestControlRect(inControl, &bestRect, &baseLineOffset);
        if (err != noErr) return false;

        int width = (bestRect.right - bestRect.left) + 1;
        int height = (bestRect.bottom - bestRect.top) + 1;

        Rect boundsRect;
        GetControlBounds (inControl, &boundsRect);

        Rect newRect;
        newRect.top = boundsRect.top;
        newRect.bottom = newRect.top + height;
        newRect.left = boundsRect.left;
        newRect.right = newRect.left + width;

        SetControlBounds (inControl, &newRect);

        if (outWidth)
            *outWidth = width;

        if (outHeight)
            *outHeight = height;
    #endif
        return true;
        */
    }

    /*
      | note that the controls are never disposed;
      | that's managed by the AUCarbonViewBase's
      | parent pane which contains all of them
      | ... if we later need to be able to delete
      | individual controls on the fly, extra work
      | needed
      */

    pub fn get_owner_view(&mut self) -> *mut AUCarbonViewBase {
        
        todo!();
        /*
            return mOwnerView;
        */
    }
    
    pub fn update(&mut self, in_ui_thread: bool)  {
        
        todo!();
        /*
            if (inUIThread)
                        ParameterToControl (mParam.GetValue());
                    else
                        AUParameterListenerNotify (mListener, this, &mParam);
        */
    }
    
    pub fn param_info(&mut self) -> &AudioUnitParameterInfo {
        
        todo!();
        /*
            return mParam.ParamInfo();
        */
    }
}
