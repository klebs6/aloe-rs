crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/native/aloe_mac_CameraDevice.h]

#[no_copy]
#[weak_referenceable]
pub struct CameraDevicePimpl<'a> {
    owner:       &'a mut CameraDevice,
    device_name: String,
    session:     *mut AVCaptureSession, // default = nil
    file_output: *mut AVCaptureMovieFileOutput, // default = nil

    #[cfg(ALOE_USE_NEW_APPLE_CAMERA_API)]
    image_output: PostCatalinaPhotoOutput,

    current_input:           *mut AVCaptureDeviceInput, // default = nil
    callback_delegate:       Id<AVCaptureFileOutputRecordingDelegate>, // default = nil
    opening_error:           String,
    first_presentation_time: Time,
    is_recording:            bool, // default = false
    listener_lock:           CriticalSection,
    listeners:               ListenerList<Box<dyn CameraDeviceListener>>,
    picture_taken_callback:  fn(_0: &Image) -> (), // default = nullptr
}

impl<'a> Drop for CameraDevicePimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            [[NSNotificationCenter defaultCenter] removeObserver: callbackDelegate];

            [session stopRunning];
            removeInput();
            removeImageCapture();
            removeMovieCapture();
            [session release];
            [callbackDelegate release];
        */
    }
}

impl<'a> CameraDevicePimpl<'a> {

    pub fn new(
        owner_to_use:       &mut CameraDevice,
        device_name_to_use: &String,
        index:              i32,
        min_width:          i32,
        min_height:         i32,
        max_width:          i32,
        max_height:         i32,
        use_high_quality:   bool) -> Self {
    
        todo!();
        /*
        : owner(ownerToUse),
        : device_name(deviceNameToUse),

            session = [[AVCaptureSession alloc] init];

            session.sessionPreset = useHighQuality ? AVCaptureSessionPresetHigh
                                                   : AVCaptureSessionPresetMedium;

            refreshConnections();

            static DelegateClass cls;
            callbackDelegate = (id<AVCaptureFileOutputRecordingDelegate>) [cls.createInstance() init];
            DelegateClass::setOwner (callbackDelegate, this);

            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
            [[NSNotificationCenter defaultCenter] addObserver: callbackDelegate
                                                     selector: @selector (captureSessionRuntimeError:)
                                                         name: AVCaptureSessionRuntimeErrorNotification
                                                       object: session];
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return openingError.isEmpty();
        */
    }
    
    pub fn start_session(&mut self)  {
        
        todo!();
        /*
            if (! [session isRunning])
                [session startRunning];
        */
    }
    
    pub fn take_still_picture(&mut self, picture_taken_callback_to_use: fn(_0: &Image) -> ())  {
        
        todo!();
        /*
            if (pictureTakenCallbackToUse == nullptr)
            {
                jassertfalse;
                return;
            }

            pictureTakenCallback = std::move (pictureTakenCallbackToUse);

            triggerImageCapture();
        */
    }
    
    pub fn start_recording_to_file(&mut self, 
        file:    &File,
        quality: i32)  {
        
        todo!();
        /*
            stopRecording();
            refreshIfNeeded();
            firstPresentationTime = Time::getCurrentTime();
            file.deleteFile();

            startSession();
            isRecording = true;
            [fileOutput startRecordingToOutputFileURL: createNSURLFromFile (file)
                                    recordingDelegate: callbackDelegate];
        */
    }
    
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            if (isRecording)
            {
                [fileOutput stopRecording];
                isRecording = false;
            }
        */
    }
    
    pub fn get_time_of_first_recorded_frame(&self) -> Time {
        
        todo!();
        /*
            return firstPresentationTime;
        */
    }
    
    pub fn add_listener(&mut self, listener_to_add: *mut dyn CameraDeviceListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.add (listenerToAdd);

            if (listeners.size() == 1)
                triggerImageCapture();
        */
    }
    
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn CameraDeviceListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.remove (listenerToRemove);
        */
    }
    
    pub fn get_available_devices() -> StringArray {
        
        todo!();
        /*
            auto* devices = decltype (imageOutput)::getAvailableDevices();

            StringArray results;

            for (AVCaptureDevice* device : devices)
                results.add (nsStringToAloe ([device localizedName]));

            return results;
        */
    }
    
    pub fn get_capture_session(&mut self) -> *mut AVCaptureSession {
        
        todo!();
        /*
            return session;
        */
    }
    
    pub fn create_video_capture_preview(&mut self) -> *mut NSView {
        
        todo!();
        /*
            // The video preview must be created before the capture session is
            // started. Make sure you haven't called `addListener`,
            // `startRecordingToFile`, or `takeStillPicture` before calling this
            // function.
            jassert (! [session isRunning]);
            startSession();

            ALOE_AUTORELEASEPOOL
            {
                NSView* view = [[NSView alloc] init];
                [view setLayer: [AVCaptureVideoPreviewLayer layerWithSession: getCaptureSession()]];
                return view;
            }
        */
    }
    
    pub fn add_image_capture(&mut self)  {
        
        todo!();
        /*
            imageOutput.addImageCapture (session);
        */
    }
    
    pub fn add_movie_capture(&mut self)  {
        
        todo!();
        /*
            if (fileOutput == nil)
            {
                fileOutput = [[AVCaptureMovieFileOutput alloc] init];
                [session addOutput: fileOutput];
            }
        */
    }
    
    pub fn remove_image_capture(&mut self)  {
        
        todo!();
        /*
            imageOutput.removeImageCapture (session);
        */
    }
    
    pub fn remove_movie_capture(&mut self)  {
        
        todo!();
        /*
            if (fileOutput != nil)
            {
                [session removeOutput: fileOutput];
                [fileOutput release];
                fileOutput = nil;
            }
        */
    }
    
    pub fn remove_current_session_video_inputs(&mut self)  {
        
        todo!();
        /*
            if (session != nil)
            {
                NSArray<AVCaptureDeviceInput*>* inputs = session.inputs;

                for (AVCaptureDeviceInput* input : inputs)
                    if ([input.device hasMediaType: AVMediaTypeVideo])
                        [session removeInput:input];
            }
        */
    }
    
    pub fn add_input(&mut self)  {
        
        todo!();
        /*
            if (currentInput == nil)
            {
                auto* availableDevices = decltype (imageOutput)::getAvailableDevices();

                for (AVCaptureDevice* device : availableDevices)
                {
                    if (deviceName == nsStringToAloe ([device localizedName]))
                    {
                        removeCurrentSessionVideoInputs();

                        NSError* err = nil;
                        AVCaptureDeviceInput* inputDevice = [[AVCaptureDeviceInput alloc] initWithDevice: device
                                                                                                   error: &err];

                        jassert (err == nil);

                        if ([session canAddInput: inputDevice])
                        {
                            [session addInput: inputDevice];
                            currentInput = inputDevice;
                        }
                        else
                        {
                            jassertfalse;
                            [inputDevice release];
                        }

                        return;
                    }
                }
            }
        */
    }
    
    pub fn remove_input(&mut self)  {
        
        todo!();
        /*
            if (currentInput != nil)
            {
                [session removeInput: currentInput];
                [currentInput release];
                currentInput = nil;
            }
        */
    }
    
    pub fn refresh_connections(&mut self)  {
        
        todo!();
        /*
            [session beginConfiguration];
            removeInput();
            removeImageCapture();
            removeMovieCapture();
            addInput();
            addImageCapture();
            addMovieCapture();
            [session commitConfiguration];
        */
    }
    
    pub fn refresh_if_needed(&mut self)  {
        
        todo!();
        /*
            if (getVideoConnection() == nullptr)
                refreshConnections();
        */
    }
    
    pub fn get_video_connection(&self) -> *mut AVCaptureConnection {
        
        todo!();
        /*
            auto* connections = imageOutput.getConnections();

            if (connections != nil)
                for (AVCaptureConnection* connection in connections)
                    if ([connection isActive] && [connection isEnabled])
                        for (AVCaptureInputPort* port in [connection inputPorts])
                            if ([[port mediaType] isEqual: AVMediaTypeVideo])
                                return connection;

            return nil;
        */
    }
    
    pub fn image_capture_finished(&mut self, image: &Image)  {
        
        todo!();
        /*
            handleImageCapture (image);

            MessageManager::callAsync ([weakRef = WeakReference<CameraDevicePimpl> { this }, image]() mutable
            {
                if (weakRef != nullptr && weakRef->pictureTakenCallback != nullptr)
                    weakRef->pictureTakenCallback (image);
            });
        */
    }
    
    pub fn handle_image_capture(&mut self, image: &Image)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.call ([=] (Listener& l) { l.imageReceived (image); });

            if (! listeners.isEmpty())
                triggerImageCapture();
        */
    }
    
    pub fn trigger_image_capture(&mut self)  {
        
        todo!();
        /*
            refreshIfNeeded();

            startSession();

            if (auto* videoConnection = getVideoConnection())
                imageOutput.triggerImageCapture (*this);
        */
    }
    
    pub fn camera_session_runtime_error(&mut self, error: &String)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraSessionRuntimeError(), error = " + error);

            if (owner.onErrorOccurred != nullptr)
                owner.onErrorOccurred (error);
        */
    }
}
