crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/capture/aloe_CameraDevice.h]

/**
  | Controls any video capture devices
  | that might be available.
  | 
  | Use getAvailableDevices() to list
  | the devices that are attached to the
  | system, then call openDevice() or openDeviceAsync()
  | to open one for use.
  | 
  | Once you have a CameraDevice object,
  | you can get a viewer component from it,
  | and use its methods to stream to a file
  | or capture still-frames.
  | 
  | @tags{Video}
  |
  */
//#[cfg(ALOE_USE_CAMERA)]
#[no_copy]
#[leak_detector]
pub struct CameraDevice {

    /**
      | Set this callback to be notified whenever
      | an error occurs. You may need to close
      | and reopen the device to be able to use
      | it further.
      |
      */
    on_error_occurred: fn(error: &String) -> (),
    name:              String,
    pimpl:             Box<dyn CameraDeviceImplInterface>,
}

impl CameraDevice {
    
    pub fn get_ios_file_extension(&mut self) -> String {
        
        todo!();
        /*
            return ".mov";
        */
    }
}

pub trait CameraDeviceImplInterface {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/capture/aloe_CameraDevice.cpp]
impl Drop for CameraDevice {

    fn drop(&mut self) {
        todo!();
        /*
            jassert (MessageManager::getInstance()->currentThreadHasLockedMessageManager());

        stopRecording();
        pimpl.reset();
        */
    }
}

impl CameraDevice {
    
    pub fn get_mac_camera_file_extension(&mut self) -> String {
        
        todo!();
        /*
            return ".mov";
        */
    }

    /**
      | Returns the name of this device
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    /**
      | Returns the file extension that should
      | be used for the files that you pass to
      | startRecordingToFile().
      | 
      | This may be platform-specific, e.g.
      | ".mov" or ".avi".
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn get_file_extension() -> String {
        
        todo!();
        /*
        
        */
    }

    #[cfg(ALOE_USE_CAMERA)]
    pub fn new(
        nm:               &String,
        index:            i32,
        min_width:        i32,
        min_height:       i32,
        max_width:        i32,
        max_height:       i32,
        use_high_quality: bool) -> Self {
    
        todo!();
        /*


            : name (nm), pimpl (new Pimpl (*this, name, index, minWidth, minHeight, maxWidth, maxHeight, useHighQuality))
        */
    }
    
    /**
      | Creates a component that can be used
      | to display a preview of the video from
      | this camera.
      | 
      | -----------
      | @note
      | 
      | While you can change the size of the preview
      | component, the actual preview display
      | may be smaller than the size requested,
      | because the correct aspect ratio is
      | maintained automatically.
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn create_viewer_component(&mut self) -> *mut Component {
        
        todo!();
        /*
            return new ViewerComponent (*this);
        */
    }
    
    /**
      | Triggers a still picture capture. Upon
      | completion, pictureTakenCallback
      | will be invoked on a message thread.
      | 
      | On Android, before calling takeStillPicture(),
      | you need to create a preview with createViewerComponent()
      | and you need to make it visible on screen.
      | 
      | Android does not support simultaneous
      | video recording and still picture capture.
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn take_still_picture(&mut self, picture_taken_callback: fn(_0: &Image) -> ())  {
        
        todo!();
        /*
            pimpl->takeStillPicture (pictureTakenCallback);
        */
    }
    
    /**
      | Starts recording video to the specified
      | file.
      | 
      | You should use getFileExtension()
      | to find out the correct extension to
      | use for your filename.
      | 
      | If the file exists, it will be deleted
      | before the recording starts.
      | 
      | This method may not start recording
      | instantly, so if you need to know the
      | exact time at which the file begins,
      | you can call getTimeOfFirstRecordedFrame()
      | after the recording has finished.
      | 
      | The quality parameter can be 0, 1, or
      | 2, to indicate low, medium, or high.
      | It may or may not be used, depending on
      | the driver.
      | 
      | On Android, before calling startRecordingToFile(),
      | you need to create a preview with createViewerComponent()
      | and you need to make it visible on screen.
      | 
      | The Android camera also requires exclusive
      | access to the audio device, so make sure
      | you close any open audio devices with
      | AudioDeviceManager::closeAudioDevice()
      | first.
      | 
      | Android does not support simultaneous
      | video recording and still picture capture.
      | 
      | @see AudioDeviceManager::closeAudioDevice,
      | AudioDeviceManager::restartLastAudioDevice
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn start_recording_to_file(
        &mut self, 
        file:    &File,
        quality: Option<i32>
    ) {

        let quality: i32 = quality.unwrap_or(2);
        
        todo!();
        /*
            stopRecording();
        pimpl->startRecordingToFile (file, quality);
        */
    }
    
    /**
      | After calling stopRecording(), this
      | method can be called to return the timestamp
      | of the first frame that was written to
      | the file.
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn get_time_of_first_recorded_frame(&self) -> Time {
        
        todo!();
        /*
            return pimpl->getTimeOfFirstRecordedFrame();
        */
    }
    
    /**
      | Stops recording, after a call to startRecordingToFile().
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            pimpl->stopRecording();
        */
    }
    
    /**
      | Adds a listener to receive images from
      | the camera.
      | 
      | Be very careful not to delete the listener
      | without first removing it by calling
      | removeListener().
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn add_listener(&mut self, listener_to_add: *mut dyn CameraDeviceListener)  {
        
        todo!();
        /*
            if (listenerToAdd != nullptr)
            pimpl->addListener (listenerToAdd);
        */
    }
    
    /**
      | Removes a listener that was previously
      | added with addListener().
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn CameraDeviceListener)  {
        
        todo!();
        /*
            if (listenerToRemove != nullptr)
            pimpl->removeListener (listenerToRemove);
        */
    }
    
    /**
      | Returns a list of the available cameras
      | on this machine.
      | 
      | You can open one of these devices by calling
      | openDevice() or openDeviceAsync().
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn get_available_devices(&mut self) -> StringArray {
        
        todo!();
        /*
            ALOE_AUTORELEASEPOOL
        {
            return Pimpl::getAvailableDevices();
        }
        */
    }
    
    /**
      | Synchronously opens a camera device.
      | This function should not be used on iOS
      | or
      | 
      | Android, use openDeviceAsync() instead.
      | 
      | The index parameter indicates which
      | of the items returned by getAvailableDevices()
      | to open.
      | 
      | The size constraints allow the method
      | to choose between different resolutions
      | if the camera supports this. If the resolution
      | can't be specified (e.g. on the Mac)
      | then these will be ignored.
      | 
      | On Mac, if highQuality is false, then
      | the camera will be opened in preview
      | mode which will allow the OS to drop frames
      | if the computer cannot keep up in processing
      | the frames.
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn open_device(
        &mut self, 
        index:            Option<i32>,
        min_width:        Option<i32>,
        min_height:       Option<i32>,
        max_width:        Option<i32>,
        max_height:       Option<i32>,
        use_high_quality: Option<bool>

    ) -> *mut CameraDevice {

        let min_width:    i32  = min_width.unwrap_or(128);
        let min_height:   i32  = min_height.unwrap_or(64);
        let max_width:    i32  = max_width.unwrap_or(1024);
        let max_height:   i32  = max_height.unwrap_or(768);
        let high_quality: bool = high_quality.unwrap_or(true);
        
        todo!();
        /*
            jassert (MessageManager::getInstance()->currentThreadHasLockedMessageManager());

       #if ! ALOE_ANDROID && ! ALOE_IOS
        std::unique_ptr<CameraDevice> d (new CameraDevice (getAvailableDevices() [index], index,
                                                           minWidth, minHeight, maxWidth, maxHeight, useHighQuality));
        if (d != nullptr && d->pimpl->openedOk())
            return d.release();
       #else
        ignoreUnused (index, minWidth, minHeight);
        ignoreUnused (maxWidth, maxHeight, useHighQuality);

        // Use openDeviceAsync to open a camera device on iOS or Android.
        jassertfalse;
       #endif

        return nullptr;
        */
    }
    
    /**
      | Asynchronously opens a camera device
      | on iOS (iOS 7+) or Android (API 21+).
      | 
      | On other platforms, the function will
      | simply call openDevice(). Upon completion,
      | resultCallback will be invoked with
      | valid CameraDevice* and an empty error
      | 
      | String on success, or nullptr CameraDevice
      | and a non-empty error String on failure.
      | 
      | This is the preferred method of opening
      | a camera device, because it works on
      | all platforms, whereas synchronous
      | openDevice() does not work on iOS & Android.
      | 
      | The index parameter indicates which
      | of the items returned by getAvailableDevices()
      | to open.
      | 
      | The size constraints allow the method
      | to choose between different resolutions
      | if the camera supports this. If the resolution
      | can't be specified then these will be
      | ignored.
      | 
      | On iOS, if you want to switch a device,
      | it is more efficient to open a new device
      | before closing the older one, because
      | this way both devices can share the same
      | underlying camera session. Otherwise,
      | the session needs to be close first,
      | and this is a lengthy process that can
      | take several seconds.
      | 
      | The Android implementation currently
      | supports a maximum recording resolution
      | of 1080p. Choosing a larger size will
      | result in larger pictures taken, but
      | the video will be capped at 1080p.
      |
      */
    #[cfg(ALOE_USE_CAMERA)]
    pub fn open_device_async(
        &mut self, 
        index:            i32,
        result_callback:  CameraDeviceOpenCameraResultCallback,
        min_width:        Option<i32>,
        min_height:       Option<i32>,
        max_width:        Option<i32>,
        max_height:       Option<i32>,
        use_high_quality: Option<bool>

    ) {

        let min_width:     i32 = min_width.unwrap_or(128);
        let min_height:    i32 = min_height.unwrap_or(64);
        let max_width:     i32 = max_width.unwrap_or(1024);
        let max_height:    i32 = max_height.unwrap_or(768);
        let high_quality: bool = high_quality.unwrap_or(true);
        
        todo!();
        /*
            jassert (MessageManager::getInstance()->currentThreadHasLockedMessageManager());

        if (resultCallback == nullptr)
        {
            // A valid callback must be passed.
            jassertfalse;
            return;
        }

       #if ALOE_ANDROID || ALOE_IOS
        CameraFactory::getInstance().openCamera (index, std::move (resultCallback),
                                                 minWidth, minHeight, maxWidth, maxHeight, useHighQuality);
       #else
        auto* device = openDevice (index, minWidth, minHeight, maxWidth, maxHeight, useHighQuality);

        resultCallback (device, device != nullptr ? String() : "Could not open camera device");
       #endif
        */
    }
}
