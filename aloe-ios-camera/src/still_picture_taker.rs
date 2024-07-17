crate::ix!();

pub struct StillPictureTaker<'a> {
    capture_session:       &'a mut CaptureSession<'a>,
    capture_output:        *mut AVCaptureOutput,
    photo_output_delegate: Box<NSObject,NSObjectDeleter>,
    taking_picture:        bool, // default = false
}

impl<'a> StillPictureTaker<'a> {

    pub fn new(cs: &mut CaptureSession) -> Self {
    
        todo!();
        /*


            : captureSession (cs),
                      captureOutput (createCaptureOutput()),
                      photoOutputDelegate (nullptr)
                    if (CameraDevicePimpl::getIOSVersion().major >= 10)
                    {
                        static PhotoOutputDelegateClass cls;
                        photoOutputDelegate.reset ([cls.createInstance() init]);
                        PhotoOutputDelegateClass::setOwner (photoOutputDelegate.get(), this);
                    }

                    captureSession.addOutputIfPossible (captureOutput);
        */
    }
    
    pub fn take_picture(&mut self, orientation_to_use: AVCaptureVideoOrientation)  {
        
        todo!();
        /*
            if (takingPicture)
                    {
                        // Picture taking already in progress!
                        jassertfalse;
                        return;
                    }

                    takingPicture = true;

                    printImageOutputDebugInfo (captureOutput);

                    if (auto* connection = findVideoConnection (captureOutput))
                    {
                       #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                        if (CameraDevicePimpl::getIOSVersion().major >= 10 && [captureOutput isKindOfClass: [AVCapturePhotoOutput class]])
                        {
                            auto* photoOutput = (AVCapturePhotoOutput*) captureOutput;
                            auto outputConnection = [photoOutput connectionWithMediaType: AVMediaTypeVideo];
                            outputConnection.videoOrientation = orientationToUse;

                            [photoOutput capturePhotoWithSettings: [AVCapturePhotoSettings photoSettings]
                                                         delegate: id<AVCapturePhotoCaptureDelegate> (photoOutputDelegate.get())];

                            return;
                        }
                       #endif

                        auto* stillImageOutput = (AVCaptureStillImageOutput*) captureOutput;
                        auto outputConnection = [stillImageOutput connectionWithMediaType: AVMediaTypeVideo];
                        outputConnection.videoOrientation = orientationToUse;

                        [stillImageOutput captureStillImageAsynchronouslyFromConnection: connection completionHandler:
                             ^(CMSampleBufferRef imageSampleBuffer, NSError* error)
                             {
                                 takingPicture = false;

                                 if (error != nil)
                                 {
                                     ALOE_CAMERA_LOG ("Still picture capture failed, error: " + nsStringToAloe (error.localizedDescription));
                                     jassertfalse;
                                     return;
                                 }

                                 NSData* imageData = [AVCaptureStillImageOutput jpegStillImageNSDataRepresentation: imageSampleBuffer];

                                 auto image = ImageFileFormat::loadFrom (imageData.bytes, (size_t) imageData.length);

                                 callListeners (image);

                                 MessageManager::callAsync ([this, image] { notifyPictureTaken (image); });
                             }];
                    }
                    else
                    {
                        // Could not find a connection of video type
                        jassertfalse;
        */
    }
    
    pub fn create_capture_output() -> *mut AVCaptureOutput {
        
        todo!();
        /*
            #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                    if (CameraDevicePimpl::getIOSVersion().major >= 10)
                        return [AVCapturePhotoOutput new];
                   #endif

                    return [AVCaptureStillImageOutput new];
        */
    }
    
    pub fn print_image_output_debug_info(capture_output: *mut AVCaptureOutput)  {
        
        todo!();
        /*
            #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                    if (CameraDevicePimpl::getIOSVersion().major >= 10 && [captureOutput isKindOfClass: [AVCapturePhotoOutput class]])
                    {
                        auto* photoOutput = (AVCapturePhotoOutput*) captureOutput;

                        String typesString;

                        for (AVVideoCodecType type in photoOutput.availablePhotoCodecTypes)
                            typesString << nsStringToAloe (type) << " ";

                        ALOE_CAMERA_LOG ("Available image codec types: " + typesString);

                        ALOE_CAMERA_LOG ("Still image stabilization supported: " + String ((int) photoOutput.stillImageStabilizationSupported));
                        ALOE_CAMERA_LOG ("Dual camera fusion supported: " + String ((int) photoOutput.dualCameraFusionSupported));
                        ALOE_CAMERA_LOG ("Supports flash: "      + String ((int) [photoOutput.supportedFlashModes containsObject: @(AVCaptureFlashModeOn)]));
                        ALOE_CAMERA_LOG ("Supports auto flash: " + String ((int) [photoOutput.supportedFlashModes containsObject: @(AVCaptureFlashModeAuto)]));
                        ALOE_CAMERA_LOG ("Max bracketed photo count: " + String (photoOutput.maxBracketedCapturePhotoCount));
                        ALOE_CAMERA_LOG ("Lens stabilization during bracketed capture supported: " + String ((int) photoOutput.lensStabilizationDuringBracketedCaptureSupported));
                        ALOE_CAMERA_LOG ("Live photo capture supported: " + String ((int) photoOutput.livePhotoCaptureSupported));

                       #if defined (__IPHONE_11_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_11_0
                        if (CameraDevicePimpl::getIOSVersion().major >= 11)
                        {
                            typesString.clear();

                            for (AVFileType type in photoOutput.availablePhotoFileTypes)
                                typesString << nsStringToAloe (type) << " ";

                            ALOE_CAMERA_LOG ("Available photo file types: " + typesString);

                            typesString.clear();

                            for (AVFileType type in photoOutput.availableRawPhotoFileTypes)
                                typesString << nsStringToAloe (type) << " ";

                            ALOE_CAMERA_LOG ("Available RAW photo file types: " + typesString);

                            typesString.clear();

                            for (AVFileType type in photoOutput.availableLivePhotoVideoCodecTypes)
                                typesString << nsStringToAloe (type) << " ";

                            ALOE_CAMERA_LOG ("Available live photo video codec types: " + typesString);

                            ALOE_CAMERA_LOG ("Dual camera dual photo delivery supported: " + String ((int) photoOutput.dualCameraDualPhotoDeliverySupported));
                            ALOE_CAMERA_LOG ("Camera calibration data delivery supported: " + String ((int) photoOutput.cameraCalibrationDataDeliverySupported));
                            ALOE_CAMERA_LOG ("Depth data delivery supported: " + String ((int) photoOutput.depthDataDeliverySupported));
                        }
                       #endif

                        return;
                    }
                   #endif

                    auto* stillImageOutput = (AVCaptureStillImageOutput*) captureOutput;

                    String typesString;

                    for (AVVideoCodecType type in stillImageOutput.availableImageDataCodecTypes)
                        typesString << nsStringToAloe (type) << " ";

                    ALOE_CAMERA_LOG ("Available image codec types: " + typesString);
                    ALOE_CAMERA_LOG ("Still image stabilization supported: " + String ((int) stillImageOutput.stillImageStabilizationSupported));
                    ALOE_CAMERA_LOG ("Automatically enables still image stabilization when available: " + String ((int) stillImageOutput.automaticallyEnablesStillImageStabilizationWhenAvailable));

                    ALOE_CAMERA_LOG ("Output settings for image output: " + nsStringToAloe ([stillImageOutput.outputSettings description]));
        */
    }

    pub fn find_video_connection(output: *mut AVCaptureOutput) -> *mut AVCaptureConnection {
        
        todo!();
        /*
            for (AVCaptureConnection* connection in output.connections)
                        for (AVCaptureInputPort* port in connection.inputPorts)
                            if ([port.mediaType isEqual: AVMediaTypeVideo])
                                return connection;

                    return nullptr;
        */
    }
    
    pub fn call_listeners(&mut self, image: &Image)  {
        
        todo!();
        /*
            captureSession.callListeners (image);
        */
    }
    
    pub fn notify_picture_taken(&mut self, image: &Image)  {
        
        todo!();
        /*
            captureSession.notifyPictureTaken (image);
        */
    }
}
