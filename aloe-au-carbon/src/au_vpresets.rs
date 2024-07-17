crate::ix!();

///--------------------------
pub struct AUVPresets {
    base:       AUPropertyControl,
    presets:    CFArrayRef,
    view:       *mut AUCarbonViewBase,
    propertyid: AudioUnitPropertyID,
}

impl Drop for AUVPresets {

    fn drop(&mut self) {
        todo!();
        /*
            CFRelease (mPresets);
        */
    }
}

impl AUVPresets {

    pub fn new(
        in_parent_view: *mut AUCarbonViewBase,
        in_presets:     &mut CFArrayRef,
        in_location:    Point,
        name_width:     i32,
        control_width:  i32,
        in_font_style:  &mut ControlFontStyleRec) -> Self {
    
        todo!();
        /*
        : au_property_control(inParentView),
        : presets(inPresets),
        : view(inParentView),

            #if !__LP64__
        Rect r;

        // ok we now have an array of factory presets
        // get their strings and display them

        r.top = inLocation.v;       r.bottom = r.top;
        r.left = inLocation.h;      r.right = r.left;

        // localize as necessary
        if (!sAUVPresetLocalized) {
            CFBundleRef mainBundle = CFBundleGetBundleWithIdentifier(kLocalizedStringBundle_AUView);
            if (mainBundle) {
                kStringFactoryPreset =  CFCopyLocalizedStringFromTableInBundle(
                                            kAUViewLocalizedStringKey_FactoryPreset, kLocalizedStringTable_AUView,
                                            mainBundle, CFSTR("FactoryPreset title string"));
                sAUVPresetLocalized = true;
            }
        }

        // create localized title string
        CFMutableStringRef factoryPresetsTitle = CFStringCreateMutable(NULL, 0);
        CFStringAppend(factoryPresetsTitle, kStringFactoryPreset);
        CFStringAppend(factoryPresetsTitle, kAUViewUnlocalizedString_TitleSeparator);

        ControlRef theControl;
        verify_noerr(CreateStaticTextControl(mView->GetCarbonWindow(), &r, factoryPresetsTitle, &inFontStyle, &theControl));
        SInt16 width = 0;
        AUCarbonViewControl::SizeControlToFit(theControl, &width, &mHeight);
        CFRelease(factoryPresetsTitle);
        EmbedControl(theControl);

        r.top -= 2;
        r.left += width + 10;
        r.right = r.left;
        r.bottom = r.top;

        verify_noerr(CreatePopupButtonControl ( mView->GetCarbonWindow(), &r, NULL,
                                                -12345, // DON'T GET MENU FROM RESOURCE mMenuID,!!!
                                                FALSE,  // variableWidth,
                                                0,      // titleWidth,
                                                0,      // titleJustification,
                                                0,      // titleStyle,
                                                &mControl));

        MenuRef menuRef;
        verify_noerr(CreateNewMenu(1, 0, &menuRef));

        int numPresets = CFArrayGetCount(mPresets);

        for (int i = 0; i < numPresets; ++i)
        {
            AUPreset* preset = (AUPreset*) CFArrayGetValueAtIndex (mPresets, i);
            verify_noerr(AppendMenuItemTextWithCFString (menuRef, preset->presetName, 0, 0, 0));
        }

        verify_noerr(SetControlData(mControl, 0, kControlPopupButtonMenuRefTag, sizeof(menuRef), &menuRef));
        verify_noerr (SetControlFontStyle (mControl, &inFontStyle));

        SetControl32BitMaximum (mControl, numPresets);

        // size popup
        SInt16 height = 0;

        AUCarbonViewControl::SizeControlToFit(mControl, &width, &height);

        if (height > mHeight) mHeight = height;
        if (mHeight < 0) mHeight = 0;

        // find which menu item is the Default preset
        UInt32 propertySize = sizeof(AUPreset);
        AUPreset defaultPreset;
        OSStatus result = AudioUnitGetProperty (mView->GetEditAudioUnit(),
                                        kAudioUnitProperty_PresentPreset,
                                        kAudioUnitScope_Global,
                                        0,
                                        &defaultPreset,
                                        &propertySize);

        mPropertyID = kAudioUnitProperty_PresentPreset;
    #endif
    #ifndef __LP64__
        if (result != noErr) {  // if the PresentPreset property is not implemented, fall back to the CurrentPreset property
            result = AudioUnitGetProperty (mView->GetEditAudioUnit(),
                                        kAudioUnitProperty_CurrentPreset,
                                        kAudioUnitScope_Global,
                                        0,
                                        &defaultPreset,
                                        &propertySize);
            mPropertyID = kAudioUnitProperty_CurrentPreset;
            if (result == noErr)
                CFRetain (defaultPreset.presetName);
        }
    #endif
    #if !__LP64__
        EmbedControl (mControl);

        HandlePropertyChange(defaultPreset);

        RegisterEvents();
    #endif
        */
    }
    
    pub fn add_interest(&mut self, 
        in_listener: AUEventListenerRef,
        in_object:   *mut c_void)  {
        
        todo!();
        /*
            AudioUnitEvent e;
        e.mEventType = kAudioUnitEvent_PropertyChange;
        e.mArgument.mProperty.mAudioUnit = mView->GetEditAudioUnit();
        e.mArgument.mProperty.mPropertyID = mPropertyID;
        e.mArgument.mProperty.mScope = kAudioUnitScope_Global;
        e.mArgument.mProperty.mElement = 0;

        AUEventListenerAddEventType(inListener, inObject, &e);
        */
    }
    
    pub fn remove_interest(&mut self, 
        in_listener: AUEventListenerRef,
        in_object:   *mut c_void)  {
        
        todo!();
        /*
            AudioUnitEvent e;
        e.mEventType = kAudioUnitEvent_PropertyChange;
        e.mArgument.mProperty.mAudioUnit = mView->GetEditAudioUnit();
        e.mArgument.mProperty.mPropertyID = mPropertyID;
        e.mArgument.mProperty.mScope = kAudioUnitScope_Global;
        e.mArgument.mProperty.mElement = 0;

        AUEventListenerRemoveEventType(inListener, inObject, &e);
        */
    }
    
    pub fn handle_control_change(&mut self)  {
        
        todo!();
        /*
            #if !__LP64__
        SInt32 i = GetControl32BitValue(mControl);
        if (i > 0)
        {
            AUPreset* preset = (AUPreset*) CFArrayGetValueAtIndex (mPresets, i-1);

            verify_noerr(AudioUnitSetProperty (mView->GetEditAudioUnit(),
                                        mPropertyID,    // either currentPreset or PresentPreset depending on which is supported
                                        kAudioUnitScope_Global,
                                        0,
                                        preset,
                                        sizeof(AUPreset)));

            // when we change a preset we can't expect the AU to update its state
            // as it isn't meant to know that its being viewed!
            // so we broadcast a notification to all listeners that all parameters on this AU have changed
            AudioUnitParameter changedUnit;
            changedUnit.mAudioUnit = mView->GetEditAudioUnit();
            changedUnit.mParameterID = kAUParameterListener_AnyParameter;
            verify_noerr (AUParameterListenerNotify (NULL, NULL, &changedUnit) );
        }
    #endif
        */
    }
    
    pub fn handle_property_change_with_preset(&mut self, preset: &mut AUPreset)  {
        
        todo!();
        /*
            #if !__LP64__
        // check to see if the preset is in our menu
        int numPresets = CFArrayGetCount(mPresets);
        if (preset.presetNumber < 0) {
            SetControl32BitValue (mControl, 0); //controls are one-based
        } else {
            for (SInt32 i = 0; i < numPresets; ++i) {
                AUPreset* currPreset = (AUPreset*) CFArrayGetValueAtIndex (mPresets, i);
                if (preset.presetNumber == currPreset->presetNumber) {
                    SetControl32BitValue (mControl, ++i); //controls are one-based
                    break;
                }
            }
        }

        if (preset.presetName)
            CFRelease (preset.presetName);
    #endif
        */
    }
    
    pub fn handle_property_change(&mut self, in_prop: &AudioUnitProperty) -> bool {
        
        todo!();
        /*
            if (inProp.mPropertyID == mPropertyID)
        {
            UInt32 theSize = sizeof(AUPreset);
            AUPreset currentPreset;

            OSStatus result = AudioUnitGetProperty(inProp.mAudioUnit,
                                                    inProp.mPropertyID,
                                                    inProp.mScope,
                                                    inProp.mElement, &currentPreset, &theSize);

            if (result == noErr) {
    #ifndef __LP64__
                if (inProp.mPropertyID == kAudioUnitProperty_CurrentPreset && currentPreset.presetName)
                    CFRetain (currentPreset.presetName);
    #endif
                HandlePropertyChange(currentPreset);
                return true;
            }
        }
        return false;
        */
    }
}
