crate::ix!();

pub struct SessionDelegateClass {
    base: ObjCClass<NSObject>,
}

impl Default for SessionDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("SessionDelegateClass_")
                    ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                    addMethod (@selector (sessionDidStartRunning:),   started,           "v@:@");
                    addMethod (@selector (sessionDidStopRunning:),    stopped,           "v@:@");
                    addMethod (@selector (runtimeError:),             runtimeError,      "v@:@");
                    addMethod (@selector (sessionWasInterrupted:),    interrupted,       "v@:@");
                    addMethod (@selector (sessionInterruptionEnded:), interruptionEnded, "v@:@");
                    ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                    addIvar<CaptureSession*> ("owner");

                    registerClass();
        */
    }
}

impl SessionDelegateClass {

    pub fn get_owner<'a>(self_: Id<NSObject>) -> &'a mut CaptureSession<'a> {
        
        todo!();
        /*
            return *getIvar<CaptureSession*> (self, "owner");
        */
    }
    
    pub fn set_owner(
        self_: Id<NSObject>,
        s:     *mut CaptureSession)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", s);
        */
    }
    
    pub fn started(
        self_:        Id<NSObject>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG (nsStringToAloe ([notification description]));

                    ignoreUnused (notification);

                    dispatch_async (dispatch_get_main_queue(),
                                    ^{
                                        getOwner (self).cameraSessionStarted();
                                    });
        */
    }
    
    pub fn stopped(
        _0:           Id<NSObject>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG (nsStringToAloe ([notification description]));

                    ignoreUnused (notification);
        */
    }
    
    pub fn runtime_error(
        self_:        Id<NSObject>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG (nsStringToAloe ([notification description]));

                    dispatch_async (dispatch_get_main_queue(),
                                    ^{
                                        NSError* error = notification.userInfo[AVCaptureSessionErrorKey];
                                        auto errorString = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                                        getOwner (self).cameraSessionRuntimeError (errorString);
                                    });
        */
    }
    
    pub fn interrupted(
        _0:           Id<NSObject>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG (nsStringToAloe ([notification description]));

                    ignoreUnused (notification);
        */
    }
    
    pub fn interruption_ended(
        _0:           Id<NSObject>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG (nsStringToAloe ([notification description]));

                    ignoreUnused (notification);
        */
    }
}
