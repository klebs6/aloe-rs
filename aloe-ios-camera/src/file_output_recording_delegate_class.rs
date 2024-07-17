crate::ix!();

pub struct FileOutputRecordingDelegateClass {
    base: ObjCClass<AVCaptureFileOutputRecordingDelegate>,
}

impl Default for FileOutputRecordingDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject<AVCaptureFileOutputRecordingDelegate>> ("FileOutputRecordingDelegateClass_")
            addMethod (@selector (captureOutput:didStartRecordingToOutputFileAtURL:fromConnections:),        started, "v@:@@@");
            addMethod (@selector (captureOutput:didFinishRecordingToOutputFileAtURL:fromConnections:error:), stopped, "v@:@@@@");

            addIvar<VideoRecorder*> ("owner");

            registerClass();
        */
    }
}

impl FileOutputRecordingDelegateClass {
    
    pub fn get_owner<'a>(self_: Id<NSObject>) -> &'a mut VideoRecorder {
        
        todo!();
        /*
            return *getIvar<VideoRecorder*> (self, "owner");
        */
    }
    
    pub fn set_owner(
        self_: Id<NSObject>,
        r:     *mut VideoRecorder)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", r);
        */
    }
    
    pub fn started(
        self_: Id<NSObject>,
        _1:    Sel,
        _2:    *mut AVCaptureFileOutput,
        _3:    *mut NSURL,
        _4:    *mut NSArray<*mut AVCaptureConnection>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("Started recording");

            getOwner (self).firstRecordedFrameTimeMs.set (Time::getCurrentTime().toMilliseconds());
            getOwner (self).recordingInProgress = true;
        */
    }
    
    pub fn stopped(
        self_: Id<NSObject>,
        _1:    Sel,
        _2:    *mut AVCaptureFileOutput,
        _3:    *mut NSURL,
        _4:    *mut NSArray<*mut AVCaptureConnection>,
        error: *mut NSError)  {
        
        todo!();
        /*
            String errorString;
            bool recordingPlayable = true;

            // There might have been an error in the recording, yet there may be a playable file...
            if ([error code] != noErr)
            {
                Id<NSObject> value = [[error userInfo] objectForKey: AVErrorRecordingSuccessfullyFinishedKey];

                if (value != nil && ! [value boolValue])
                    recordingPlayable = false;

                errorString = nsStringToAloe (error.localizedDescription) + ", playable: " + String ((int) recordingPlayable);
            }

            ALOE_CAMERA_LOG ("Stopped recording, error = " + errorString);

            getOwner (self).recordingInProgress = false;
        */
    }
}
