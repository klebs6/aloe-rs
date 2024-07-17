crate::ix!();

pub struct DelegateClass {
    base: ObjCClass<NSObject>,
}

impl Default for DelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("ALOECameraDelegate_")
                addIvar<CameraDevicePimpl*> ("owner");
                addProtocol (@protocol (AVCaptureFileOutputRecordingDelegate));

                addMethod (@selector (captureOutput:didStartRecordingToOutputFileAtURL:  fromConnections:),       didStartRecordingToOutputFileAtURL,   "v@:@@@");
                addMethod (@selector (captureOutput:didPauseRecordingToOutputFileAtURL:  fromConnections:),       didPauseRecordingToOutputFileAtURL,   "v@:@@@");
                addMethod (@selector (captureOutput:didResumeRecordingToOutputFileAtURL: fromConnections:),       didResumeRecordingToOutputFileAtURL,  "v@:@@@");
                addMethod (@selector (captureOutput:willFinishRecordingToOutputFileAtURL:fromConnections:error:), willFinishRecordingToOutputFileAtURL, "v@:@@@@");

                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                addMethod (@selector (captureSessionRuntimeError:), sessionRuntimeError, "v@:@");
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                registerClass();
        */
    }
}

impl DelegateClass {

    pub fn set_owner(
        self_: Id<Self>,
        owner: *mut CameraDevicePimpl)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", owner);
        */
    }
    
    pub fn get_owner<'a>(self_: Id<Self>) -> &'a mut CameraDevicePimpl<'a> {
        
        todo!();
        /*
            return *getIvar<CameraDevicePimpl*> (self, "owner");
        */
    }
    
    pub fn did_start_recording_to_output_file_aturl(
        _0: Id<Self>,
        _1: Sel,
        _2: *mut AVCaptureFileOutput,
        _3: *mut NSURL,
        _4: *mut NSArray<NSObject>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn did_pause_recording_to_output_file_aturl(
        _0: Id<Self>,
        _1: Sel,
        _2: *mut AVCaptureFileOutput,
        _3: *mut NSURL,
        _4: *mut NSArray<NSObject>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn did_resume_recording_to_output_file_aturl(
        _0: Id<Self>,
        _1: Sel,
        _2: *mut AVCaptureFileOutput,
        _3: *mut NSURL,
        _4: *mut NSArray<NSObject>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn will_finish_recording_to_output_file_aturl(
        _0: Id<Self>,
        _1: Sel,
        _2: *mut AVCaptureFileOutput,
        _3: *mut NSURL,
        _4: *mut NSArray<NSObject>,
        _5: *mut NSError)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn session_runtime_error(
        self_:        Id<Self>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG (nsStringToAloe ([notification description]));

                NSError* error = notification.userInfo[AVCaptureSessionErrorKey];
                auto errorString = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                getOwner (self).cameraSessionRuntimeError (errorString);
        */
    }
}
