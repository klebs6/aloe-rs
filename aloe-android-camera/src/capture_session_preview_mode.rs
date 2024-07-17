crate::ix!();

pub struct CaptureSessionPreviewMode<'a> {
    base:         CaptureSessionMode<'a,CaptureSessionPreviewMode<'a>>,
    image_reader: &'a mut ImageReader<'a>,
}

impl<'a> CaptureSessionPreviewMode<'a> {

    pub fn new(
        owner_to_use:                    &mut CameraDevicePimpl,
        camera_device_to_use:            &mut ScopedCameraDevice,
        handler_to_use:                  &mut GlobalRef,
        pd:                              &mut PreviewDisplay,
        ir:                              &mut ImageReader,
        sensor_orientation:              i32,
        camera_lens_facing_to_use:       i32,
        stream_configuration_map_to_use: &mut StreamConfigurationMap) -> Self {
    
        todo!();
        /*


            : CaptureSessionMode<CaptureSessionPreviewMode> (ownerToUse, cameraDeviceToUse, handlerToUse, pd,
                                                                 sensorOrientation, cameraLensFacingToUse, streamConfigurationMapToUse),
                  imageReader (ir)
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
                auto imageSurface = LocalRef<jobject> (imageReader.getSurface());

                auto arrayList = LocalRef<jobject> (env->NewObject (JavaArrayList, JavaArrayList.constructor, 2));
                env->CallBooleanMethod (arrayList, JavaArrayList.add, previewSurface.get());
                env->CallBooleanMethod (arrayList, JavaArrayList.add, imageSurface.get());

                auto supported = streamConfigurationMap.isOutputSupportedForSurface (imageSurface);

                // Output surface is not supported by this device, still image capture will not work!
                jassert (supported);

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
            auto* env = getEnv();

                auto previewSurface = LocalRef<jobject> (previewDisplay.createSurface());

                auto arrayList = LocalRef<jobject> (env->NewObject (JavaArrayList, JavaArrayList.constructor, 1));
                env->CallBooleanMethod (arrayList, JavaArrayList.add, previewSurface.get());

                return arrayList;
        */
    }
    
    pub fn get_template() -> i32 {
        
        todo!();
        /*
            static constexpr int templatePreview = 1;
                return templatePreview;
        */
    }
    
    pub fn is_video_record() -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn session_started(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn take_still_picture(&mut self)  {
        
        todo!();
        /*
            imageReader.resetNotificationFlag();
                captureSession->takeStillPicture (imageReader.getSurface());
        */
    }
}
