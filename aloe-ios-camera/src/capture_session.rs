crate::ix!();

lazy_static!{
    /*
    static int numCaptureSessions;
    int CameraDevice::CameraDeviceImpl::CaptureSession::numCaptureSessions = 0;
    */
}

#[weak_referenceable]
pub struct CaptureSession<'a> {
    owner:                 &'a mut CameraDeviceImpl<'a>,
    capture_session_queue: DispatchQueue,
    capture_session:       Box<AVCaptureSession,NSObjectDeleter>,
    delegate:              Box<NSObject,NSObjectDeleter>,
    still_picture_taker:   StillPictureTaker<'a>,
    video_recorder:        VideoRecorder,
    camera_device:         *mut AVCaptureDevice, // default = nil
    preview_layer:         *mut AVCaptureVideoPreviewLayer, // default = nil
    session_started:       bool, // default = false
    session_closed_event:  WaitableEvent,
}

impl<'a> Drop for CaptureSession<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            [[NSNotificationCenter defaultCenter] removeObserver: delegate.get()];

                stopRecording();

                if (--numCaptureSessions == 0)
                {
                    dispatch_async (captureSessionQueue, ^
                                    {
                                        if (captureSession.get().running)
                                            [captureSession.get() stopRunning];

                                        sessionClosedEvent.signal();
                                    });

                    sessionClosedEvent.wait (-1);
                }
        */
    }
}

impl<'a> CaptureSession<'a> {

    pub fn new(
        owner_to_use:     &mut CameraDeviceImpl,
        use_high_quality: bool) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                  captureSessionQueue (dispatch_queue_create ("AloeCameraDeviceBackgroundDispatchQueue", DISPATCH_QUEUE_SERIAL)),
                  captureSession ([[AVCaptureSession alloc] init]),
                  delegate (nullptr),
                  stillPictureTaker (*this),
                  videoRecorder (*this)
                static SessionDelegateClass cls;
                delegate.reset ([cls.createInstance() init]);
                SessionDelegateClass::setOwner (delegate.get(), this);

                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                [[NSNotificationCenter defaultCenter] addObserver: delegate.get()
                                                         selector: @selector (sessionDidStartRunning:)
                                                             name: AVCaptureSessionDidStartRunningNotification
                                                           object: captureSession.get()];

                [[NSNotificationCenter defaultCenter] addObserver: delegate.get()
                                                         selector: @selector (sessionDidStopRunning:)
                                                             name: AVCaptureSessionDidStopRunningNotification
                                                           object: captureSession.get()];

                [[NSNotificationCenter defaultCenter] addObserver: delegate.get()
                                                         selector: @selector (runtimeError:)
                                                             name: AVCaptureSessionRuntimeErrorNotification
                                                           object: captureSession.get()];

                [[NSNotificationCenter defaultCenter] addObserver: delegate.get()
                                                         selector: @selector (sessionWasInterrupted:)
                                                             name: AVCaptureSessionWasInterruptedNotification
                                                           object: captureSession.get()];

                [[NSNotificationCenter defaultCenter] addObserver: delegate.get()
                                                         selector: @selector (sessionInterruptionEnded:)
                                                             name: AVCaptureSessionInterruptionEndedNotification
                                                           object: captureSession.get()];
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                dispatch_async (captureSessionQueue,^
                                {
                                    [captureSession.get() setSessionPreset: useHighQuality ? AVCaptureSessionPresetHigh
                                                                                           : AVCaptureSessionPresetMedium];
                                });

                ++numCaptureSessions;
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return sessionStarted;
        */
    }
    
    pub fn start_session_for_device_with_id(&mut self, camera_id_to_use: &String)  {
        
        todo!();
        /*
            dispatch_async (captureSessionQueue,^
                                {
                                    cameraDevice = [AVCaptureDevice deviceWithUniqueID: aloeStringToNS (cameraIdToUse)];
                                    auto audioDevice = [AVCaptureDevice defaultDeviceWithMediaType: AVMediaTypeAudio];

                                    [captureSession.get() beginConfiguration];

                                    // This will add just video...
                                    auto error = addInputToDevice (cameraDevice);

                                    if (error.isNotEmpty())
                                    {
                                        MessageManager::callAsync ([weakRef = WeakReference<CaptureSession> { this }, error]() mutable
                                        {
                                            if (weakRef != nullptr)
                                                weakRef->owner.cameraOpenCallback ({}, error);
                                        });

                                        return;
                                    }

                                    // ... so add audio explicitly here
                                    error = addInputToDevice (audioDevice);

                                    if (error.isNotEmpty())
                                    {
                                        MessageManager::callAsync ([weakRef = WeakReference<CaptureSession> { this }, error]() mutable
                                        {
                                            if (weakRef != nullptr)
                                                weakRef->owner.cameraOpenCallback ({}, error);
                                        });

                                        return;
                                    }

                                    [captureSession.get() commitConfiguration];

                                    if (! captureSession.get().running)
                                        [captureSession.get() startRunning];
                                });
        */
    }
    
    pub fn create_preview_layer(&mut self) -> *mut AVCaptureVideoPreviewLayer {
        
        todo!();
        /*
            if (! openedOk())
                {
                    // A session must be started first!
                    jassertfalse;
                    return nullptr;
                }

                previewLayer = [AVCaptureVideoPreviewLayer layerWithSession: captureSession.get()];
                return previewLayer;
        */
    }
    
    pub fn take_still_picture(&mut self)  {
        
        todo!();
        /*
            if (! openedOk())
                {
                    // A session must be started first!
                    jassert (openedOk());
                    return;
                }

                stillPictureTaker.takePicture (previewLayer.connection.videoOrientation);
        */
    }
    
    pub fn start_recording(&mut self, file: &File)  {
        
        todo!();
        /*
            if (! openedOk())
                {
                    // A session must be started first!
                    jassertfalse;
                    return;
                }

                if (file.existsAsFile())
                {
                    // File overwriting is not supported by iOS video recorder, the target
                    // file must not exist.
                    jassertfalse;
                    return;
                }

                videoRecorder.startRecording (file, previewLayer.connection.videoOrientation);
        */
    }
    
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            videoRecorder.stopRecording();
        */
    }
    
    pub fn get_time_of_first_recorded_frame(&self) -> Time {
        
        todo!();
        /*
            return videoRecorder.getTimeOfFirstRecordedFrame();
        */
    }
    
    pub fn add_input_to_device(&mut self, device: *mut AVCaptureDevice) -> String {
        
        todo!();
        /*
            NSError* error = nil;

                auto input = [AVCaptureDeviceInput deviceInputWithDevice: device
                                                                   error: &error];

                if (error != nil)
                    return nsStringToAloe (error.localizedDescription);

                if (! [captureSession.get() canAddInput: input])
                    return "Could not add input to camera session.";

                [captureSession.get() addInput: input];
                return {};
        */
    }
    
    pub fn add_output_if_possible(&mut self, output: *mut AVCaptureOutput)  {
        
        todo!();
        /*
            dispatch_async (captureSessionQueue,^
                                {
                                    if ([captureSession.get() canAddOutput: output])
                                    {
                                        [captureSession.get() beginConfiguration];
                                        [captureSession.get() addOutput: output];
                                        [captureSession.get() commitConfiguration];

                                        return;
                                    }

                                    // Can't add output to camera session!
                                    jassertfalse;
                                });
        */
    }
    
    pub fn camera_session_started(&mut self)  {
        
        todo!();
        /*
            sessionStarted = true;

                owner.cameraSessionStarted();
        */
    }
    
    pub fn camera_session_runtime_error(&mut self, error: &String)  {
        
        todo!();
        /*
            owner.cameraSessionRuntimeError (error);
        */
    }
    
    pub fn call_listeners(&mut self, image: &Image)  {
        
        todo!();
        /*
            owner.callListeners (image);
        */
    }
    
    pub fn notify_picture_taken(&mut self, image: &Image)  {
        
        todo!();
        /*
            owner.notifyPictureTaken (image);
        */
    }
}
