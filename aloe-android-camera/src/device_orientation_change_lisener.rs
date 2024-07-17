crate::ix!();

pub struct DeviceOrientationChangeListener<'a> {
    base:                              Timer,
    preview_display:                   &'a mut PreviewDisplay<'a>,
    orientation_event_listener:        GlobalRef,
    can_detect_change:                 bool,
    device_orientation:                DesktopDisplayOrientation,
    last_known_screen_orientation:     DesktopDisplayOrientation,
    num_checks_for_orientation_change: i32, // default = 10
}

impl<'a> Drop for DeviceOrientationChangeListener<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            setEnabled (false);
        */
    }
}

impl<'a> DeviceOrientationChangeListener<'a> {

    pub fn new(pd: &mut PreviewDisplay) -> Self {
    
        todo!();
        /*


            : previewDisplay (pd),
                  orientationEventListener (createOrientationEventListener()),
                  canDetectChange (getEnv()->CallBooleanMethod (orientationEventListener,
                                                                OrientationEventListener.canDetectOrientation) != 0),
                  deviceOrientation (Desktop::getInstance().getCurrentOrientation()),
                  lastKnownScreenOrientation (deviceOrientation)
                setEnabled (true);
        */
    }
    
    pub fn set_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            if (shouldBeEnabled && ! canDetectChange)
                {
                    // This device does not support orientation listening, photos may have wrong orientation!
                    jassertfalse;
                    return;
                }

                if (shouldBeEnabled)
                    getEnv()->CallVoidMethod (orientationEventListener, OrientationEventListener.enable);
                else
                    getEnv()->CallVoidMethod (orientationEventListener, OrientationEventListener.disable);
        */
    }
    
    pub fn is_supported(&self) -> bool {
        
        todo!();
        /*
            return canDetectChange;
        */
    }
    
    pub fn get_device_orientation(&self) -> DesktopDisplayOrientation {
        
        todo!();
        /*
            return deviceOrientation;
        */
    }
    
    pub fn create_orientation_event_listener(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->NewObject (OrientationEventListener,
                                                               OrientationEventListener.constructor,
                                                               reinterpret_cast<jlong> (this),
                                                               getAppContext().get(),
                                                               sensorDelayUI));
        */
    }
    
    pub fn orientation_changed(&mut self, orientation: i32)  {
        
        todo!();
        /*
            jassert (orientation < 360);

                // -1 == unknown
                if (orientation < 0)
                    return;

                auto oldOrientation = deviceOrientation;

                // NB: this assumes natural position to be portrait always, but some devices may be landscape...
                if (orientation > (360 - 45) || orientation < 45)
                    deviceOrientation = Desktop::upright;
                else if (orientation < 135)
                    deviceOrientation = Desktop::rotatedClockwise;
                else if (orientation < 225)
                    deviceOrientation = Desktop::upsideDown;
                else
                    deviceOrientation = Desktop::rotatedAntiClockwise;

                if (oldOrientation != deviceOrientation)
                {
                    lastKnownScreenOrientation = Desktop::getInstance().getCurrentOrientation();

                    // Need to update preview transform, but screen orientation will change slightly
                    // later than sensor orientation.
                    startTimer (500);
                }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto currentOrientation = Desktop::getInstance().getCurrentOrientation();

                if (lastKnownScreenOrientation != currentOrientation)
                {
                    lastKnownScreenOrientation = currentOrientation;

                    stopTimer();
                    numChecksForOrientationChange = 10;
                    previewDisplay.updateSurfaceTransform();

                    return;
                }

                if (--numChecksForOrientationChange == 0)
                {
                    stopTimer();
                    numChecksForOrientationChange = 10;
                }
        */
    }
    
    pub fn device_orientation_changed(
        _0:          *mut JNIEnv,
        obj:         jobject,
        host:        i64,
        orientation: i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<DeviceOrientationChangeListener*> (host))
                    myself->orientationChanged (orientation);
        */
    }
}
