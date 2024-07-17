crate::ix!();

pub struct CaptureSessionVideoRecordingMode<'a> {
    base:           CaptureSessionMode<'a, CaptureSessionVideoRecordingMode<'a>>,
    media_recorder: &'a mut MediaRecorder<'a>,
}

impl<'a> Drop for CaptureSessionVideoRecordingMode<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            // We need to explicitly stop the preview before stopping the media recorder,
                // because legacy devices can't handle recording stop before stopping the preview.
                stopPreview();

                mediaRecorder.stop();
        */
    }
}

impl<'a> CaptureSessionVideoRecordingMode<'a> {

    pub fn new(
        owner_to_use:                    &mut CameraDevicePimpl,
        camera_device_to_use:            &mut ScopedCameraDevice,
        handler_to_use:                  &mut GlobalRef,
        pd:                              &mut PreviewDisplay,
        mr:                              &mut MediaRecorder,
        sensor_orientation:              i32,
        camera_lens_facing_to_use:       i32,
        stream_configuration_map_to_use: &mut StreamConfigurationMap) -> Self {
    
        todo!();
        /*


            : CaptureSessionMode<CaptureSessionVideoRecordingMode> (ownerToUse, cameraDeviceToUse, handlerToUse, pd,
                                                                        sensorOrientation, cameraLensFacingToUse, streamConfigurationMapToUse),
                  mediaRecorder (mr)
        */
    }

    /**
      | Surfaces passed to newly created capture
      | session.
      |
      */
    pub fn get_capture_session_surfaces(&self) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

                auto previewSurface = LocalRef<jobject> (previewDisplay.createSurface());
                auto mediaRecorderSurface = LocalRef<jobject> (mediaRecorder.getSurface());

                auto arrayList = LocalRef<jobject> (env->NewObject (JavaArrayList, JavaArrayList.constructor, 2));
                env->CallBooleanMethod (arrayList, JavaArrayList.add, previewSurface.get());
                env->CallBooleanMethod (arrayList, JavaArrayList.add, mediaRecorderSurface.get());

                return arrayList;
        */
    }

    /**
      | Surfaces set as target during capture.
      |
      */
    pub fn get_target_surfaces(&self) -> LocalRef<jobject> {
        
        todo!();
        /*
            // Same surfaces used.
                return getCaptureSessionSurfaces();
        */
    }
    
    pub fn get_template() -> i32 {
        
        todo!();
        /*
            static constexpr int templateRecord = 3;
                return templateRecord;
        */
    }
    
    pub fn is_video_record() -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn session_started(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([this]() { mediaRecorder.start(); });
        */
    }
    
    pub fn take_still_picture(&mut self)  {
        
        todo!();
        /*
            // Taking still pictures while recording video is not supported on Android.
                jassertfalse;
        */
    }
}
