crate::ix!();

lazy_static!{
    /*
    static IOSVersion iosVersion;
    CameraDevice::CameraDevicePimpl::IOSVersion CameraDevice::CameraDevicePimpl::iosVersion = CameraDevice::CameraDevicePimpl::getIOSVersion();
    */
}

#[no_copy]
pub struct CameraDevicePimpl<'a> {
    owner:                      &'a mut CameraDevice,
    camera_id:                  String,
    camera_open_callback:       InternalOpenCameraResultCallback,
    listener_lock:              CriticalSection,
    listeners:                  ListenerList<Box<dyn CameraDeviceListener>>,
    picture_taken_callback:     fn(_0: &Image) -> (),
    capture_session:            CaptureSession<'a>,
    notified_of_camera_opening: bool, // default = false
}

impl<'a> CameraDevicePimpl<'a> {

    pub fn new(
        owner_to_use:     &mut CameraDevice,
        camera_id_to_use: &String,
        index:            i32,
        min_width:        i32,
        min_height:       i32,
        max_width:        i32,
        max_height:       i32,
        use_high_quality: bool) -> Self {
    
        todo!();
        /*

            : owner (ownerToUse),
              cameraId (cameraIdToUse),
              captureSession (*this, useHighQuality)
        */
    }
    
    pub fn get_camera_id(&self) -> String {
        
        todo!();
        /*
            return cameraId;
        */
    }
    
    pub fn open(&mut self, camera_open_callback_to_use: InternalOpenCameraResultCallback)  {
        
        todo!();
        /*
            cameraOpenCallback = std::move (cameraOpenCallbackToUse);

            if (cameraOpenCallback == nullptr)
            {
                // A valid camera open callback must be passed.
                jassertfalse;
                return;
            }

            [AVCaptureDevice requestAccessForMediaType: AVMediaTypeVideo
                                     completionHandler: ^(BOOL granted)
             {
                 // Access to video is required for camera to work,
                 // black images will be produced otherwise!
                 jassertquiet (granted);
             }];

            [AVCaptureDevice requestAccessForMediaType: AVMediaTypeAudio
                                     completionHandler: ^(BOOL granted)
             {
                 // Access to audio is required for camera to work,
                 // silence will be produced otherwise!
                 jassertquiet (granted);
             }];

            captureSession.startSessionForDeviceWithId (cameraId);
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return captureSession.openedOk();
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

            triggerStillPictureCapture();
        */
    }
    
    pub fn start_recording_to_file(&mut self, 
        file:    &File,
        quality: i32)  {
        
        todo!();
        /*
            file.deleteFile();

            captureSession.startRecording (file);
        */
    }
    
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            captureSession.stopRecording();
        */
    }
    
    pub fn get_time_of_first_recorded_frame(&self) -> Time {
        
        todo!();
        /*
            return captureSession.getTimeOfFirstRecordedFrame();
        */
    }
    
    pub fn get_available_devices() -> StringArray {
        
        todo!();
        /*
            StringArray results;

            ALOE_CAMERA_LOG ("Available camera devices: ");

            for (AVCaptureDevice* device in getDevices())
            {
                ALOE_CAMERA_LOG ("Device start----------------------------------");
                printDebugCameraInfo (device);
                ALOE_CAMERA_LOG ("Device end----------------------------------");

                results.add (nsStringToAloe (device.uniqueID));
            }

            return results;
        */
    }
    
    pub fn add_listener(&mut self, listener_to_add: *mut dyn CameraDeviceListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.add (listenerToAdd);

            if (listeners.size() == 1)
                triggerStillPictureCapture();
        */
    }
    
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn CameraDeviceListener)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.remove (listenerToRemove);
        */
    }
    
    pub fn get_devices() -> *mut NSArray<*mut AVCaptureDevice> {
        
        todo!();
        /*
            #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
            if (iosVersion.major >= 10)
            {
                std::unique_ptr<NSMutableArray<AVCaptureDeviceType>, NSObjectDeleter> deviceTypes ([[NSMutableArray alloc] initWithCapacity: 2]);

                [deviceTypes.get() addObject: AVCaptureDeviceTypeBuiltInWideAngleCamera];
                [deviceTypes.get() addObject: AVCaptureDeviceTypeBuiltInTelephotoCamera];

                if ((iosVersion.major == 10 && iosVersion.minor >= 2) || iosVersion.major >= 11)
                    [deviceTypes.get() addObject: AVCaptureDeviceTypeBuiltInDualCamera];

                if ((iosVersion.major == 11 && iosVersion.minor >= 1) || iosVersion.major >= 12)
                    [deviceTypes.get() addObject: AVCaptureDeviceTypeBuiltInTrueDepthCamera];

                auto discoverySession = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes: deviceTypes.get()
                                                                                               mediaType: AVMediaTypeVideo
                                                                                                position: AVCaptureDevicePositionUnspecified];

                return [discoverySession devices];
            }
           #endif

            return [AVCaptureDevice devicesWithMediaType: AVMediaTypeVideo];
        */
    }
    
    pub fn print_debug_camera_info(device: *mut AVCaptureDevice)  {
        
        todo!();
        /*
            auto position = device.position;

            String positionString = position == AVCaptureDevicePositionBack
                                  ? "Back"
                                  : position == AVCaptureDevicePositionFront
                                             ? "Front"
                                             : "Unspecified";

            ALOE_CAMERA_LOG ("Position: " + positionString);
            ALOE_CAMERA_LOG ("Model ID: " + nsStringToAloe (device.modelID));
            ALOE_CAMERA_LOG ("Localized name: " + nsStringToAloe (device.localizedName));
            ALOE_CAMERA_LOG ("Unique ID: " + nsStringToAloe (device.uniqueID));
            ALOE_CAMERA_LOG ("Lens aperture: " + String (device.lensAperture));

            ALOE_CAMERA_LOG ("Has flash: " + String ((int)device.hasFlash));
            ALOE_CAMERA_LOG ("Supports flash always on: " + String ((int)[device isFlashModeSupported: AVCaptureFlashModeOn]));
            ALOE_CAMERA_LOG ("Supports auto flash: " + String ((int)[device isFlashModeSupported: AVCaptureFlashModeAuto]));

            ALOE_CAMERA_LOG ("Has torch: " + String ((int)device.hasTorch));
            ALOE_CAMERA_LOG ("Supports torch always on: " + String ((int)[device isTorchModeSupported: AVCaptureTorchModeOn]));
            ALOE_CAMERA_LOG ("Supports auto torch: " + String ((int)[device isTorchModeSupported: AVCaptureTorchModeAuto]));

            ALOE_CAMERA_LOG ("Low light boost supported: " + String ((int)device.lowLightBoostEnabled));

            ALOE_CAMERA_LOG ("Supports auto white balance: " + String ((int)[device isWhiteBalanceModeSupported: AVCaptureWhiteBalanceModeAutoWhiteBalance]));
            ALOE_CAMERA_LOG ("Supports continuous auto white balance: " + String ((int)[device isWhiteBalanceModeSupported: AVCaptureWhiteBalanceModeContinuousAutoWhiteBalance]));

            ALOE_CAMERA_LOG ("Supports auto focus: " + String ((int)[device isFocusModeSupported: AVCaptureFocusModeAutoFocus]));
            ALOE_CAMERA_LOG ("Supports continuous auto focus: " + String ((int)[device isFocusModeSupported: AVCaptureFocusModeContinuousAutoFocus]));
            ALOE_CAMERA_LOG ("Supports point of interest focus: " + String ((int)device.focusPointOfInterestSupported));
            ALOE_CAMERA_LOG ("Smooth auto focus supported: " + String ((int)device.smoothAutoFocusSupported));
            ALOE_CAMERA_LOG ("Auto focus range restriction supported: " + String ((int)device.autoFocusRangeRestrictionSupported));

            ALOE_CAMERA_LOG ("Supports auto exposure: " + String ((int)[device isExposureModeSupported: AVCaptureExposureModeAutoExpose]));
            ALOE_CAMERA_LOG ("Supports continuous auto exposure: " + String ((int)[device isExposureModeSupported: AVCaptureExposureModeContinuousAutoExposure]));
            ALOE_CAMERA_LOG ("Supports custom exposure: " + String ((int)[device isExposureModeSupported: AVCaptureExposureModeCustom]));
            ALOE_CAMERA_LOG ("Supports point of interest exposure: " + String ((int)device.exposurePointOfInterestSupported));

           #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
            if (iosVersion.major >= 10)
            {
                ALOE_CAMERA_LOG ("Device type: " + nsStringToAloe (device.deviceType));
                ALOE_CAMERA_LOG ("Locking focus with custom lens position supported: " + String ((int)device.lockingFocusWithCustomLensPositionSupported));
            }
           #endif

           #if defined (__IPHONE_11_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_11_0
            if (iosVersion.major >= 11)
            {
                ALOE_CAMERA_LOG ("Min available video zoom factor: " + String (device.minAvailableVideoZoomFactor));
                ALOE_CAMERA_LOG ("Max available video zoom factor: " + String (device.maxAvailableVideoZoomFactor));
                ALOE_CAMERA_LOG ("Dual camera switch over video zoom factor: " + String (device.dualCameraSwitchOverVideoZoomFactor));
            }
           #endif

            ALOE_CAMERA_LOG ("Capture formats start-------------------");
            for (AVCaptureDeviceFormat* format in device.formats)
            {
                ALOE_CAMERA_LOG ("Capture format start------");
                printDebugCameraFormatInfo (format);
                ALOE_CAMERA_LOG ("Capture format end------");
            }
            ALOE_CAMERA_LOG ("Capture formats end-------------------");
        */
    }
    
    pub fn print_debug_camera_format_info(format: *mut AVCaptureDeviceFormat)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("Media type: " + nsStringToAloe (format.mediaType));

            String colourSpaces;

            for (NSNumber* number in format.supportedColorSpaces)
            {
                switch ([number intValue])
                {
                    case AVCaptureColorSpace_sRGB:   colourSpaces << "sRGB ";  break;
                    case AVCaptureColorSpace_P3_D65: colourSpaces << "P3_D65 "; break;
                    default: break;
                }
            }

            ALOE_CAMERA_LOG ("Supported colour spaces: " + colourSpaces);

            ALOE_CAMERA_LOG ("Video field of view: " + String (format.videoFieldOfView));
            ALOE_CAMERA_LOG ("Video max zoom factor: " + String (format.videoMaxZoomFactor));
            ALOE_CAMERA_LOG ("Video zoom factor upscale threshold: " + String (format.videoZoomFactorUpscaleThreshold));

            String videoFrameRateRangesString = "Video supported frame rate ranges: ";

            for (AVFrameRateRange* range in format.videoSupportedFrameRateRanges)
                videoFrameRateRangesString << frameRateRangeToString (range);
            ALOE_CAMERA_LOG (videoFrameRateRangesString);

            ALOE_CAMERA_LOG ("Video binned: " + String (int (format.videoBinned)));

            ALOE_CAMERA_LOG ("Video HDR supported: " + String (int (format.videoHDRSupported)));
            ALOE_CAMERA_LOG ("High resolution still image dimensions: " + getHighResStillImgDimensionsString (format.highResolutionStillImageDimensions));
            ALOE_CAMERA_LOG ("Min ISO: " + String (format.minISO));
            ALOE_CAMERA_LOG ("Max ISO: " + String (format.maxISO));
            ALOE_CAMERA_LOG ("Min exposure duration: " + cmTimeToString (format.minExposureDuration));

            String autoFocusSystemString;
            switch (format.autoFocusSystem)
            {
                case AVCaptureAutoFocusSystemPhaseDetection:    autoFocusSystemString = "PhaseDetection";    break;
                case AVCaptureAutoFocusSystemContrastDetection: autoFocusSystemString = "ContrastDetection"; break;
                case AVCaptureAutoFocusSystemNone:
                default:                                        autoFocusSystemString = "None";
            }
            ALOE_CAMERA_LOG ("Auto focus system: " + autoFocusSystemString);

            ALOE_CAMERA_LOG ("Standard (iOS 5.0) video stabilization supported: " + String ((int) [format isVideoStabilizationModeSupported: AVCaptureVideoStabilizationModeStandard]));
            ALOE_CAMERA_LOG ("Cinematic video stabilization supported: " + String ((int) [format isVideoStabilizationModeSupported: AVCaptureVideoStabilizationModeCinematic]));
            ALOE_CAMERA_LOG ("Auto video stabilization supported: " + String ((int) [format isVideoStabilizationModeSupported: AVCaptureVideoStabilizationModeAuto]));

           #if defined (__IPHONE_11_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_11_0
            if (iosVersion.major >= 11)
            {
                ALOE_CAMERA_LOG ("Min zoom factor for depth data delivery: " + String (format.videoMinZoomFactorForDepthDataDelivery));
                ALOE_CAMERA_LOG ("Max zoom factor for depth data delivery: " + String (format.videoMaxZoomFactorForDepthDataDelivery));
            }
           #endif
        */
    }
    
    pub fn get_high_res_still_img_dimensions_string(d: CMVideoDimensions) -> String {
        
        todo!();
        /*
            return "[" + String (d.width) + " " + String (d.height) + "]";
        */
    }
    
    pub fn cm_time_to_string(time: CMTime) -> String {
        
        todo!();
        /*
            CFUniquePtr<CFStringRef> timeDesc (CMTimeCopyDescription (nullptr, time));
            return String::fromCFString (timeDesc.get());
        */
    }
    
    pub fn frame_rate_range_to_string(range: *mut AVFrameRateRange) -> String {
        
        todo!();
        /*
            String result;
            result << "[minFrameDuration: " + cmTimeToString (range.minFrameDuration);
            result << " maxFrameDuration: " + cmTimeToString (range.maxFrameDuration);
            result << " minFrameRate: " + String (range.minFrameRate);
            result << " maxFrameRate: " + String (range.maxFrameRate) << "] ";

            return result;
        */
    }
    
    pub fn camera_session_started(&mut self)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraSessionStarted()");

            cameraOpenCallback (cameraId, {});
        */
    }
    
    pub fn camera_session_runtime_error(&mut self, error: &String)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraSessionRuntimeError(), error = " + error);

            if (! notifiedOfCameraOpening)
            {
                cameraOpenCallback ({}, error);
            }
            else
            {
                if (owner.onErrorOccurred != nullptr)
                    owner.onErrorOccurred (error);
            }
        */
    }
    
    pub fn call_listeners(&mut self, image: &Image)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.call ([=] (Listener& l) { l.imageReceived (image); });

            if (listeners.size() == 1)
                triggerStillPictureCapture();
        */
    }
    
    pub fn notify_picture_taken(&mut self, image: &Image)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("notifyPictureTaken()");

            if (pictureTakenCallback != nullptr)
                pictureTakenCallback (image);
        */
    }
    
    pub fn trigger_still_picture_capture(&mut self)  {
        
        todo!();
        /*
            captureSession.takeStillPicture();
        */
    }
    
    pub fn get_ios_version() -> IOSVersion {
        
        todo!();
        /*
            auto processInfo = [NSProcessInfo processInfo];

            if (! [processInfo respondsToSelector: @selector (operatingSystemVersion)])
                return {7, 0};   // Below 8.0 in fact, but only care that it's below 8

            return { (int)[processInfo operatingSystemVersion].majorVersion,
                     (int)[processInfo operatingSystemVersion].minorVersion };
        */
    }
}
