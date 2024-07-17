crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/CameraDemo.h]

#[no_copy]
#[leak_detector]
pub struct CameraDemo<'a> {

    base:                      Component<'a>,

    /**
      if this PIP is running inside the demo
      runner, we'll use the shared device manager
      instead
      */
    #[cfg(not(ALOE_DEMO_RUNNER))]
    audio_device_manager:      AudioDeviceManager<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    audio_device_manager:      &mut AudioDeviceManager, // default = getSharedAudioDeviceManager (0, 2) 

    camera_device:             Box<CameraDevice>,
    camera_preview_comp:       Box<Component<'a>>,
    last_snapshot:             ImageComponent<'a>,
    camera_selector_combo_box: ComboBox<'a>, // default = "Camera" 
    snapshot_button:           TextButton<'a>, // default = "Take a snapshot" 

    #[cfg(all(not(target_os="android"),not(target_os="ios")))]
    record_movie_button:       TextButton<'a>, //{ "Record a movie (to your desktop)..." };

    #[cfg(not(all(not(target_os="android"),not(target_os="ios"))))]
    record_movie_button:       TextButton<'a>, //{ "Record a movie" };

    recording_movie:           bool, // default = false
    recording_file:            File,
    content_sharing_pending:   bool, // default = false
}

impl<'a> Default for CameraDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

           #if ALOE_ANDROID
            // Android requires exclusive access to the audio device when recording videos.
            audioDeviceManager.closeAudioDevice();
           #endif

            addAndMakeVisible (cameraSelectorComboBox);
            updateCameraList();
            cameraSelectorComboBox.setSelectedId (1);
            cameraSelectorComboBox.onChange = [this] { cameraChanged(); };

            addAndMakeVisible (snapshotButton);
            snapshotButton.onClick = [this] { takeSnapshot(); };
            snapshotButton.setEnabled (false);

            addAndMakeVisible (recordMovieButton);
            recordMovieButton.onClick = [this] { startRecording(); };
            recordMovieButton.setEnabled (false);

            addAndMakeVisible (lastSnapshot);

            cameraSelectorComboBox.setSelectedId (2);

            setSize (500, 500);

           #if ALOE_IOS || ALOE_ANDROID
            setPortraitOrientationEnabled (true);
           #endi
        */
    }
}

impl<'a> Drop for CameraDemo<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
           #if ALOE_IOS || ALOE_ANDROID
            setPortraitOrientationEnabled (false);
           #endif

           #if ALOE_ANDROID
            audioDeviceManager.restartLastAudioDevice();
           #endif
         */
    }
}

impl<'a> CameraDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (5);

            auto top = r.removeFromTop (25);
            cameraSelectorComboBox.setBounds (top.removeFromLeft (250));

            r.removeFromTop (4);
            top = r.removeFromTop (25);

            snapshotButton.changeWidthToFitText (24);
            snapshotButton.setBounds (top.removeFromLeft (snapshotButton.getWidth()));
            top.removeFromLeft (4);
            recordMovieButton.changeWidthToFitText (24);
            recordMovieButton.setBounds (top.removeFromLeft (recordMovieButton.getWidth()));

            r.removeFromTop (4);
            auto previewArea = shouldUseLandscapeLayout() ? r.removeFromLeft (r.getWidth() / 2)
                                                          : r.removeFromTop (r.getHeight() / 2);

            if (cameraPreviewComp.get() != nullptr)
                cameraPreviewComp->setBounds (previewArea);

            if (shouldUseLandscapeLayout())
                r.removeFromLeft (4);
            else
                r.removeFromTop (4);

            lastSnapshot.setBounds (r);
        */
    }
    
    pub fn set_portrait_orientation_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            auto allowedOrientations = Desktop::getInstance().getOrientationsEnabled();

            if (shouldBeEnabled)
                allowedOrientations |= Desktop::upright;
            else
                allowedOrientations &= ~Desktop::upright;

            Desktop::getInstance().setOrientationsEnabled (allowedOrientations);
        */
    }
    
    pub fn should_use_landscape_layout(&self) -> bool {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS
            auto orientation = Desktop::getInstance().getCurrentOrientation();
            return orientation == Desktop::rotatedClockwise || orientation == Desktop::rotatedAntiClockwise;
           #else
            return false;
           #endif
        */
    }
    
    pub fn update_camera_list(&mut self)  {
        
        todo!();
        /*
            cameraSelectorComboBox.clear();
            cameraSelectorComboBox.addItem ("No camera", 1);
            cameraSelectorComboBox.addSeparator();

            auto cameras = CameraDevice::getAvailableDevices();

            for (int i = 0; i < cameras.size(); ++i)
                cameraSelectorComboBox.addItem (cameras[i], i + 2);
        */
    }
    
    pub fn camera_changed(&mut self)  {
        
        todo!();
        /*
            // This is called when the user chooses a camera from the drop-down list.
           #if ALOE_IOS
            // On iOS, when switching camera, open the new camera first, so that it can
            // share the underlying camera session with the old camera. Otherwise, the
            // session would have to be closed first, which can take several seconds.
            if (cameraSelectorComboBox.getSelectedId() == 1)
                cameraDevice.reset();
           #else
            cameraDevice.reset();
           #endif
            cameraPreviewComp.reset();
            recordingMovie = false;

            if (cameraSelectorComboBox.getSelectedId() > 1)
            {
               #if ALOE_ANDROID || ALOE_IOS
                openCameraAsync();
               #else
                cameraDeviceOpenResult (CameraDevice::openDevice (cameraSelectorComboBox.getSelectedId() - 2), {});
               #endif
            }
            else
            {
                snapshotButton   .setEnabled (cameraDevice != nullptr && ! contentSharingPending);
                recordMovieButton.setEnabled (cameraDevice != nullptr && ! contentSharingPending);
                resized();
            }
        */
    }
    
    pub fn open_camera_async(&mut self)  {
        
        todo!();
        /*
            SafePointer<CameraDemo> safeThis (this);

            CameraDevice::openDeviceAsync (cameraSelectorComboBox.getSelectedId() - 2,
                                           [safeThis] (CameraDevice* device, const String& error) mutable
                                           {
                                               if (safeThis)
                                                   safeThis->cameraDeviceOpenResult (device, error);
                                           });
        */
    }
    
    pub fn camera_device_open_result(&mut self, 
        device: *mut CameraDevice,
        error:  &String)  {
        
        todo!();
        /*
            // If camera opening worked, create a preview component for it..
            cameraDevice.reset (device);

            if (cameraDevice.get() != nullptr)
            {
               #if ALOE_ANDROID
                SafePointer<CameraDemo> safeThis (this);
                cameraDevice->onErrorOccurred = [safeThis] (const String& cameraError) mutable { if (safeThis) safeThis->errorOccurred (cameraError); };
               #endif
                cameraPreviewComp.reset (cameraDevice->createViewerComponent());
                addAndMakeVisible (cameraPreviewComp.get());
            }
            else
            {
                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon, "Camera open failed",
                                                  "Camera open failed, reason: " + error);
            }

            snapshotButton   .setEnabled (cameraDevice.get() != nullptr && ! contentSharingPending);
            recordMovieButton.setEnabled (cameraDevice.get() != nullptr && ! contentSharingPending);
            resized();
        */
    }
    
    pub fn start_recording(&mut self)  {
        
        todo!();
        /*
            if (cameraDevice.get() != nullptr)
            {
                // The user has clicked the record movie button..
                if (! recordingMovie)
                {
                    // Start recording to a file on the user's desktop..
                    recordingMovie = true;

                   #if ALOE_ANDROID || ALOE_IOS
                    recordingFile = File::getSpecialLocation (File::tempDirectory)
                   #else
                    recordingFile = File::getSpecialLocation (File::userDesktopDirectory)
                   #endif
                                     .getNonexistentChildFile ("AloeCameraVideoDemo", CameraDevice::getFileExtension());

                   #if ALOE_ANDROID
                    // Android does not support taking pictures while recording video.
                    snapshotButton.setEnabled (false);
                   #endif

                    cameraSelectorComboBox.setEnabled (false);
                    cameraDevice->startRecordingToFile (recordingFile);
                    recordMovieButton.setButtonText ("Stop Recording");
                }
                else
                {
                    // Already recording, so stop...
                    recordingMovie = false;
                    cameraDevice->stopRecording();
                   #if ! ALOE_ANDROID && ! ALOE_IOS
                    recordMovieButton.setButtonText ("Start recording (to a file on your desktop)");
                   #else
                    recordMovieButton.setButtonText ("Record a movie");
                   #endif
                    cameraSelectorComboBox.setEnabled (true);

                   #if ALOE_ANDROID
                    snapshotButton.setEnabled (true);
                   #endif

                   #if ALOE_CONTENT_SHARING
                    Url url (recordingFile);

                    snapshotButton   .setEnabled (false);
                    recordMovieButton.setEnabled (false);
                    contentSharingPending = true;

                    SafePointer<CameraDemo> safeThis (this);

                    ContentSharer::getInstance()->shareFiles ({url},
                                                                    [safeThis] (bool success, const String&) mutable
                                                                    {
                                                                        if (safeThis)
                                                                            safeThis->sharingFinished (success, false);
                                                                    });
                   #endif
                }
            }
        */
    }
    
    pub fn take_snapshot(&mut self)  {
        
        todo!();
        /*
            SafePointer<CameraDemo> safeThis (this);
            cameraDevice->takeStillPicture ([safeThis] (const Image& image) mutable { safeThis->imageReceived (image); });
        */
    }

    /**
      | This is called by the camera device when
      | a new image arrives
      |
      */
    pub fn image_received(&mut self, image: &Image)  {
        
        todo!();
        /*
            if (! image.isValid())
                return;

            lastSnapshot.setImage (image);

           #if ALOE_CONTENT_SHARING
            auto imageFile = File::getSpecialLocation (File::tempDirectory).getNonexistentChildFile ("AloeCameraPhotoDemo", ".jpg");

            FileOutputStream stream (imageFile);

            if (stream.openedOk()
                 && JPEGImageFormat().writeImageToStream (image, stream))
            {
                Url url (imageFile);

                snapshotButton   .setEnabled (false);
                recordMovieButton.setEnabled (false);
                contentSharingPending = true;

                SafePointer<CameraDemo> safeThis (this);

                ContentSharer::getInstance()->shareFiles ({url},
                                                                [safeThis] (bool success, const String&) mutable
                                                                {
                                                                    if (safeThis)
                                                                        safeThis->sharingFinished (success, true);
                                                                });
            }
           #endif
        */
    }
    
    pub fn error_occurred(&mut self, error: &String)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon,
                                              "Camera Device Error",
                                              "An error has occurred: " + error + " Camera will be closed.");

            cameraDevice.reset();

            cameraSelectorComboBox.setSelectedId (1);
            snapshotButton   .setEnabled (false);
            recordMovieButton.setEnabled (false);
        */
    }
    
    pub fn sharing_finished(&mut self, 
        success:    bool,
        is_capture: bool)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon,
                                              isCapture ? "Image sharing result" : "Video sharing result",
                                              success ? "Success!" : "Failed!");

            contentSharingPending = false;
            snapshotButton   .setEnabled (true);
            recordMovieButton.setEnabled (true);
        */
    }
}
