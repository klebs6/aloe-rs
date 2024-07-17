crate::ix!();

pub trait CaptureSessionModeBase
{
    fn is_video_record_session(&self) -> bool;

    fn trigger_still_picture_capture(&mut self);
}

///--------------------
#[weak_referenceable]
pub struct CaptureSessionMode<'a,Mode> {
    owner:                     &'a mut CameraDevicePimpl<'a>,
    scoped_camera_device:      &'a mut ScopedCameraDevice<'a>,
    handler:                   &'a mut GlobalRef,
    preview_display:           &'a mut PreviewDisplay<'a>,
    camera_sensor_orientation: i32,
    camera_lens_facing:        i32,
    stream_configuration_map:  &'a mut StreamConfigurationMap,
    capture_session:           Box<CaptureSession<'a>>,
    _0:                        PhantomData<Mode>,
}

impl<'a,Mode> CaptureSessionConfiguredCallback for CaptureSessionMode<'a,Mode> {

    fn capture_session_configured(&mut self, session: *mut CaptureSession)  {
        
        todo!();
        /*
            if (session == nullptr)
                {
                    owner.cameraDeviceError ("Failed to configure camera session.");
                    return;
                }

                jassert (session == captureSession.get());

                startSession();
        */
    }
}

impl<'a,Mode> PreviewDisplayListener for CaptureSessionMode<'a,Mode> {

    fn preview_display_ready(&mut self)  {
        
        todo!();
        /*
            jassert (previewDisplay.isReady());

                ALOE_CAMERA_LOG ("previewDisplayReady()");

                // close previous capture session first
                captureSession.reset();

                if (scopedCameraDevice.hasErrorOccurred())
                {
                    ALOE_CAMERA_LOG ("Device error detected, not recreating a new camera session. The device needs to be reopened.");
                    return;
                }

                captureSession.reset (scopedCameraDevice.createCaptureSession (*this, crtp().getCaptureSessionSurfaces(),
                                                                               handler, Mode::getTemplate()));
        */
    }
    
    fn preview_display_about_to_be_destroyed(&mut self)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("previewDisplayAboutToBeDestroyed()");

                stopPreview();
        */
    }
}

impl<'a,Mode> CaptureSessionModeBase for CaptureSessionMode<'a,Mode> {

    fn is_video_record_session(&self) -> bool {
        
        todo!();
        /*
            return Mode::isVideoRecord();
        */
    }
    
    fn trigger_still_picture_capture(&mut self)  {
        
        todo!();
        /*
            if (captureSession == nullptr)
                {
                    // The capture session must be ready before taking a still picture.
                    // Did you remember to create and show a preview display?
                    jassertfalse;
                    return;
                }

                crtp().takeStillPicture();
        */
    }
}

impl<'a,Mode> Drop for CaptureSessionMode<'a,Mode> {

    fn drop(&mut self) {
        todo!();
        /*
            captureSession.reset();

                previewDisplay.removeListener (this);
        */
    }
}

impl<'a,Mode> CaptureSessionMode<'a,Mode> {
    
    pub fn new(
        owner_to_use:                     &mut CameraDevicePimpl,
        camera_device_to_use:             &mut ScopedCameraDevice,
        handler_to_use:                   &mut GlobalRef,
        pd:                               &mut PreviewDisplay,
        camera_sensor_orientation_to_use: i32,
        camera_lens_facing_to_use:        i32,
        stream_configuration_map_to_use:  &mut StreamConfigurationMap) -> Self {
    
        todo!();
        /*
        : owner(ownerToUse),
        : scoped_camera_device(cameraDeviceToUse),
        : handler(handlerToUse),
        : preview_display(pd),
        : camera_sensor_orientation(cameraSensorOrientationToUse),
        : camera_lens_facing(cameraLensFacingToUse),
        : stream_configuration_map(streamConfigurationMapToUse),

            // async so that the object is fully constructed before the callback gets invoked
                MessageManager::callAsync ([weakRef = WeakReference<CaptureSessionMode<Mode>> { this }]
                {
                    if (weakRef != nullptr)
                        weakRef->previewDisplay.addListener (weakRef.get());
                });
        */
    }
    
    pub fn crtp(&mut self) -> &mut Mode {
        
        todo!();
        /*
            return static_cast<Mode&> (*this);
        */
    }
    
    pub fn start_session(&mut self)  {
        
        todo!();
        /*
            if (! captureSession->start (crtp().getTargetSurfaces(), handler))
                {
                    jassertfalse;
                    ALOE_CAMERA_LOG ("Could not start capture session");
                }

                crtp().sessionStarted();
        */
    }
    
    pub fn stop_preview(&mut self)  {
        
        todo!();
        /*
            if (captureSession != nullptr)
                {
                    auto session = captureSession->getNativeSession();

                    auto* env = getEnv();

                    env->CallVoidMethod (session, CameraCaptureSession.stopRepeating);

                    if (jniCheckHasExceptionOccurredAndClear())
                        return;

                    env->CallVoidMethod (session, CameraCaptureSession.abortCaptures);

                    jniCheckHasExceptionOccurredAndClear();
                }
        */
    }
}
