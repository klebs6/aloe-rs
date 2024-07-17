crate::ix!();

#[no_copy]
pub struct CameraDevicePimpl<'a> {
    base:                               ActivityLifecycleCallbacks,
    owner:                              &'a mut CameraDevice,
    min_width:                          i32,
    min_height:                         i32,
    max_width:                          i32,
    max_height:                         i32,
    camera_id:                          String,
    camera_open_callback:               InternalOpenCameraResultCallback,
    activity_life_listener:             GlobalRef,
    camera_manager:                     GlobalRef,
    camera_characteristics:             GlobalRef,
    handler_thread:                     GlobalRef,
    handler:                            GlobalRef,
    stream_configuration_map:           StreamConfigurationMap,
    preview_display:                    PreviewDisplay<'a>,
    device_orientation_change_listener: DeviceOrientationChangeListener<'a>,
    image_reader:                       Box<ImageReader<'a>>,
    media_recorder:                     Box<MediaRecorder<'a>>,
    current_capture_session_mode:       Box<dyn CaptureSessionModeBase>,
    scoped_camera_device:               Box<ScopedCameraDevice<'a>>,
    listener_lock:                      CriticalSection,
    listeners:                          ListenerList<Box<dyn CameraDeviceListener>>,
    picture_taken_callback:             fn(_0: &Image) -> (),
    first_recorded_frame_time_ms:       Time,
    notified_of_camera_opening:         bool, // default = false
    app_was_paused:                     bool, // default = false
}

impl<'a> Drop for CameraDevicePimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto* env = getEnv();

            env->CallVoidMethod (getAppContext().get(), AndroidApplication.unregisterActivityLifecycleCallbacks, activityLifeListener.get());
            activityLifeListener.clear();
        */
    }
}

aloe_declare_weak_referenceable!{
    CameraDevicePimpl<'a>
}

impl<'a> CameraDevicePimpl<'a> {

    pub fn new(
        owner_to_use:      &mut CameraDevice,
        camera_id_to_use:  &String,
        index:             i32,
        min_width_to_use:  i32,
        min_height_to_use: i32,
        max_width_to_use:  i32,
        max_height_to_use: i32,
        use_high_quality:  bool) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
              minWidth (minWidthToUse),
              minHeight (minHeightToUse),
              maxWidth (maxWidthToUse),
              maxHeight (maxHeightToUse),
              cameraId (cameraIdToUse),
              activityLifeListener (CreateJavaInterface (this, "android/app/Application$ActivityLifecycleCallbacks")),
              cameraManager (initialiseCameraManager()),
              cameraCharacteristics (initialiseCameraCharacteristics (cameraManager, cameraId)),
              streamConfigurationMap (cameraCharacteristics),
              previewDisplay (streamConfigurationMap.getPreviewBufferSize()),
              deviceOrientationChangeListener (previewDisplay)
            startBackgroundThread();
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

            // A valid camera open callback must be passed.
            jassert (cameraOpenCallback != nullptr);

            // The same camera can be opened only once!
            jassert (scopedCameraDevice == nullptr);

            if (cameraOpenCallback == nullptr || scopedCameraDevice != nullptr)
                return;

            RuntimePermissions::request (RuntimePermissions::camera,
                                         [safeThis = WeakReference<CameraDevicePimpl> { this }] (bool granted) mutable
                                         {
                                             if (safeThis != nullptr)
                                                 safeThis->continueOpenRequest (granted);
                                         });
        */
    }
    
    pub fn continue_open_request(&mut self, granted: bool)  {
        
        todo!();
        /*
            if (getAndroidSDKVersion() >= 21)
            {
                if (granted)
                {
                    getEnv()->CallVoidMethod (getAppContext().get(), AndroidApplication.registerActivityLifecycleCallbacks, activityLifeListener.get());
                    scopedCameraDevice.reset (new ScopedCameraDevice (*this, cameraId, cameraManager, handler, getAutoFocusModeToUse()));
                }
                else
                {
                    invokeCameraOpenCallback ("Camera permission not granted");
                }
            }
            else
            {
                invokeCameraOpenCallback ("Camera requires android sdk version 21 or greater");
            }
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return scopedCameraDevice->openedOk();
        */
    }
    
    pub fn take_still_picture(&mut self, picture_taken_callback_to_use: fn(_0: &Image) -> ())  {
        
        todo!();
        /*
            if (pictureTakenCallbackToUse == nullptr || currentCaptureSessionMode == nullptr)
            {
                jassertfalse;
                return;
            }

            if (currentCaptureSessionMode->isVideoRecordSession())
            {
                // Taking still pictures while recording video is not supported on Android.
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
            if (! openedOk())
            {
                jassertfalse;
                return;
            }

            if (! previewDisplay.isReady())
            {
                // Did you remember to create and show a preview display?
                jassertfalse;
                return;
            }

            file.deleteFile();
            file.create();
            jassert (file.existsAsFile());

            // MediaRecorder can't handle videos larger than 1080p
            auto videoSize = chooseBestSize (minWidth, minHeight, jmin (maxWidth, 1080), maxHeight,
                                             streamConfigurationMap.getSupportedVideoRecordingOutputSizes());

            mediaRecorder.reset (new MediaRecorder (file.getFullPathName(), videoSize.getWidth(), videoSize.getHeight(),
                                                    getCameraSensorOrientation(), getCameraLensFacing()));

            firstRecordedFrameTimeMs = Time::getCurrentTime();

            currentCaptureSessionMode.reset();
            startVideoRecordingMode (*mediaRecorder);
        */
    }
    
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            currentCaptureSessionMode.reset();
            mediaRecorder.reset();

            startPreviewMode (*imageReader);
        */
    }
    
    pub fn get_time_of_first_recorded_frame(&self) -> Time {
        
        todo!();
        /*
            return firstRecordedFrameTimeMs;
        */
    }
    
    pub fn get_available_devices() -> StringArray {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 21)
                return StringArray(); // Camera requires SDK version 21 or later

            StringArray results;

            auto* env = getEnv();

            auto cameraManagerToUse = initialiseCameraManager();
            auto cameraIdArray = LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (cameraManagerToUse,
                                                                                               CameraManager.getCameraIdList));

            results = javaStringArrayToAloe (cameraIdArray);

            for (auto& result : results)
                printDebugCameraInfo (cameraManagerToUse, result);

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
    
    pub fn camera_error_code_to_string(error_code: i32) -> String {
        
        todo!();
        /*
            switch (errorCode)
            {
                case ERROR_CAMERA_IN_USE:      return "Camera already in use.";
                case ERROR_MAX_CAMERAS_IN_USE: return "Too many opened camera devices.";
                case ERROR_CAMERA_DISABLED:    return "Camera disabled.";
                case ERROR_CAMERA_DEVICE:      return "Fatal error.";
                case ERROR_CAMERA_SERVICE:     return "Fatal error. Reboot required or persistent hardware problem.";
                default:                       return "Unknown error.";
            }
        */
    }
    
    pub fn initialise_camera_manager() -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->CallObjectMethod (getAppContext().get(), AndroidContext.getSystemService,
                                                                  javaString ("camera").get()));
        */
    }
    
    pub fn initialise_camera_characteristics(
        camera_manager: &GlobalRef,
        camera_id:      &String) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->CallObjectMethod (cameraManager,
                                                                  CameraManager.getCameraCharacteristics,
                                                                  javaString (cameraId).get()));
        */
    }
    
    pub fn print_debug_camera_info(
        camera_manager_to_use: &LocalRef<jobject>,
        camera_id:             &String)  {
        
        todo!();
        /*
            auto* env = getEnv();

            auto characteristics = LocalRef<jobject> (env->CallObjectMethod (cameraManagerToUse,
                                                                             CameraManager.getCameraCharacteristics,
                                                                             javaString (cameraId).get()));

            auto keysList = LocalRef<jobject> (env->CallObjectMethod (characteristics, CameraCharacteristics.getKeys));

            const int size = env->CallIntMethod (keysList, JavaList.size);

            ALOE_CAMERA_LOG ("Camera id: " + cameraId + ", characteristics keys num: " + String (size));

            for (int i = 0; i < size; ++i)
            {
                auto key = LocalRef<jobject> (env->CallObjectMethod (keysList, JavaList.get, i));
                auto jKeyName = LocalRef<jstring> ((jstring) env->CallObjectMethod (key, CameraCharacteristicsKey.getName));
                auto keyName = aloeString (jKeyName);

                auto keyValue = LocalRef<jobject> (env->CallObjectMethod (characteristics, CameraCharacteristics.get, key.get()));
                auto jKeyValueString = LocalRef<jstring> ((jstring) env->CallObjectMethod (keyValue, JavaObject.toString));
                auto keyValueString = aloeString (jKeyValueString);

                auto &kvs = keyValueString;

                if (kvs.startsWith ("[I") || kvs.startsWith ("[F") || kvs.startsWith ("[Z") || kvs.startsWith ("[B"))
                {
                    printPrimitiveArrayElements (keyValue, keyName, keyValueString);
                }
                else if (kvs.startsWith ("[Landroid.util.Range"))
                {
                    printRangeArrayElements (keyValue, keyName);
                }
                else
                {
                    int chunkSize = 256;

                    if (keyValueString.length() > chunkSize)
                    {
                        ALOE_CAMERA_LOG ("Key: " + keyName);

                        for (int j = 0, k = 1; j < keyValueString.length(); j += chunkSize, ++k)
                            ALOE_CAMERA_LOG ("value part " + String (k) + ": " + keyValueString.substring (j, k + chunkSize));
                    }
                    else
                    {
                        ALOE_CAMERA_LOG ("Key: " + keyName + ", value: " + keyValueString);
                    }
                }

                ignoreUnused (keyName);
            }
        */
    }
    
    pub fn print_primitive_array_elements(
        key_value:        &LocalRef<jobject>,
        key_name:         &String,
        key_value_string: &String)  {
        
        todo!();
        /*
            ignoreUnused (keyName);

            String result = "[";

            auto* env = getEnv();

            #define PRINT_ELEMENTS(elem_type, array_type, fun_name_middle)                                              \
            {                                                                                                           \
                elem_type* elements = env->Get##fun_name_middle##ArrayElements ((array_type) keyValue.get(), nullptr);  \
                int size = env->GetArrayLength ((array_type) keyValue.get());                                           \
                                                                                                                        \
                for (int i = 0; i < size - 1; ++i)                                                                      \
                    result << String (elements[i]) << " ";                                                              \
                                                                                                                        \
                if (size > 0)                                                                                           \
                    result << String (elements[size - 1]);                                                              \
                                                                                                                        \
                env->Release##fun_name_middle##ArrayElements ((array_type) keyValue.get(), elements, 0);                \
            }

            if (keyValueString.startsWith ("[I"))
                PRINT_ELEMENTS (jint, jintArray, Int)
            else if (keyValueString.startsWith ("[F"))
                PRINT_ELEMENTS (float, jfloatArray, Float)
            else if (keyValueString.startsWith ("[Z"))
                PRINT_ELEMENTS (jboolean, jbooleanArray, Boolean)
            else if (keyValueString.startsWith ("[B"))
                PRINT_ELEMENTS (jbyte, jbyteArray, Byte);

            #undef PRINT_ELEMENTS

            result << "]";
            ALOE_CAMERA_LOG ("Key: " + keyName + ", value: " + result);
        */
    }
    
    pub fn print_range_array_elements(
        range_array: &LocalRef<jobject>,
        key_name:    &String)  {
        
        todo!();
        /*
            auto* env = getEnv();

            jobjectArray ranges = static_cast<jobjectArray> (rangeArray.get());

            int numRanges = env->GetArrayLength (ranges);

            String result;

            for (int i = 0; i < numRanges; ++i)
            {
                auto range = LocalRef<jobject> (env->GetObjectArrayElement (ranges, i));

                auto jRangeString = LocalRef<jstring> ((jstring) env->CallObjectMethod (range, AndroidRange.toString));

                result << aloeString (jRangeString) << " ";
            }

            ignoreUnused (keyName);
            ALOE_CAMERA_LOG ("Key: " + keyName + ", value: " + result);
        */
    }
    
    pub fn get_camera_sensor_orientation(&self) -> i32 {
        
        todo!();
        /*
            return getCameraCharacteristicsIntegerKeyValue (CameraCharacteristics.SENSOR_ORIENTATION);
        */
    }
    
    pub fn get_auto_focus_mode_to_use(&self) -> i32 {
        
        todo!();
        /*
            auto supportedModes = getSupportedAutoFocusModes();

            enum
            {
                CONTROL_AF_MODE_OFF = 0,
                CONTROL_AF_MODE_AUTO = 1,
                CONTROL_AF_MODE_CONTINUOUS_PICTURE = 4
            };

            if (supportedModes.contains (CONTROL_AF_MODE_CONTINUOUS_PICTURE))
                return CONTROL_AF_MODE_CONTINUOUS_PICTURE;

            if (supportedModes.contains (CONTROL_AF_MODE_AUTO))
                return CONTROL_AF_MODE_AUTO;

            return CONTROL_AF_MODE_OFF;
        */
    }
    
    pub fn get_supported_auto_focus_modes(&self) -> Vec<i32> {
        
        todo!();
        /*
            auto* env = getEnv();

            auto jKey = LocalRef<jobject> (env->GetStaticObjectField (CameraCharacteristics, CameraCharacteristics.CONTROL_AF_AVAILABLE_MODES));

            auto supportedModes = LocalRef<jintArray> ((jintArray) env->CallObjectMethod (cameraCharacteristics,
                                                                                          CameraCharacteristics.get,
                                                                                          jKey.get()));

            return jintArrayToAloeArray (supportedModes);
        */
    }
    
    pub fn jint_array_to_aloe_array(j_array: &LocalRef<jintArray>) -> Vec<i32> {
        
        todo!();
        /*
            auto* env = getEnv();

            auto* jArrayElems = env->GetIntArrayElements (jArray, nullptr);
            auto numElems = env->GetArrayLength (jArray);

            Vec<int> aloeArray;

            for (int s = 0; s < numElems; ++s)
                aloeArray.add (jArrayElems[s]);

            env->ReleaseIntArrayElements (jArray, jArrayElems, 0);
            return aloeArray;
        */
    }
    
    pub fn get_camera_characteristics_integer_key_value(&self, key: JFieldID) -> i32 {
        
        todo!();
        /*
            auto* env = getEnv();

            auto jKey = LocalRef<jobject> (env->GetStaticObjectField (CameraCharacteristics, key));

            auto jValue = LocalRef<jobject> (env->CallObjectMethod (cameraCharacteristics,
                                                                    CameraCharacteristics.get,
                                                                    jKey.get()));

            return env->CallIntMethod (jValue, JavaInteger.intValue);
        */
    }
    
    pub fn get_camera_lens_facing(&self) -> i32 {
        
        todo!();
        /*
            return getCameraCharacteristicsIntegerKeyValue (CameraCharacteristics.LENS_FACING);
        */
    }
    
    pub fn camera_open_finished(&mut self, error: &String)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraOpenFinished(), error = " + error);

            if (error.isEmpty())
            {
                setupStillImageSize();
                startPreviewMode (*imageReader);
            }

            // Do not notify about camera being reopened on app resume.
            if (! notifiedOfCameraOpening)
            {
                notifiedOfCameraOpening = true;

                invokeCameraOpenCallback (error);
            }
        */
    }
    
    pub fn camera_device_error(&mut self, error: &String)  {
        
        todo!();
        /*
            if (owner.onErrorOccurred != nullptr)
                owner.onErrorOccurred (error);
        */
    }
    
    pub fn invoke_camera_open_callback(&mut self, error: &String)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("invokeCameraOpenCallback(), error = " + error);

            if (cameraOpenCallback != nullptr)
                cameraOpenCallback (cameraId, error);
        */
    }
    
    pub fn call_listeners(&mut self, image: &Image)  {
        
        todo!();
        /*
            const ScopedLock sl (listenerLock);
            listeners.call ([=] (Listener& l) { l.imageReceived (image); });
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
            currentCaptureSessionMode->triggerStillPictureCapture();
        */
    }
    
    pub fn setup_still_image_size(&mut self)  {
        
        todo!();
        /*
            imageReader.reset();

            auto imageSize = chooseBestSize (minWidth, minHeight, maxWidth, maxHeight,
                                             streamConfigurationMap.getSupportedStillImageOutputSizes());

            imageReader.reset (new ImageReader (*this, handler, imageSize.getWidth(), imageSize.getHeight(),
                                                getCameraSensorOrientation()));
        */
    }
    
    pub fn choose_best_size(
        min_width:       i32,
        min_height:      i32,
        max_width:       i32,
        max_height:      i32,
        supported_sizes: Vec<Rectangle<i32>>) -> Rectangle<i32> {
        
        todo!();
        /*
            Rectangle<int> result;

            for (auto& size : supportedSizes)
            {
                auto width  = size.getWidth();
                auto height = size.getHeight();

                if (width < minWidth || width > maxWidth || height < minHeight || height > maxHeight)
                    continue;

                if (size.contains (result))
                    result = size;
            }

            // None of the supported sizes matches required width & height limitations, picking
            // the first one available...
            jassert (! result.isEmpty());

            if (result.isEmpty())
                result = supportedSizes[0];

            return result;
        */
    }
    
    pub fn start_preview_mode(&mut self, ir: &mut ImageReader)  {
        
        todo!();
        /*
            if (currentCaptureSessionMode != nullptr && ! currentCaptureSessionMode->isVideoRecordSession())
                return;

            // previous mode has to be stopped first
            jassert (currentCaptureSessionMode.get() == nullptr);

            if (scopedCameraDevice == nullptr || ! scopedCameraDevice->openedOk())
                return;

            currentCaptureSessionMode.reset (new CaptureSessionPreviewMode (*this, *scopedCameraDevice, handler,
                                                                            previewDisplay, ir,
                                                                            getCameraSensorOrientation(),
                                                                            getCameraLensFacing(),
                                                                            streamConfigurationMap));
        */
    }
    
    pub fn start_video_recording_mode(&mut self, mr: &mut MediaRecorder)  {
        
        todo!();
        /*
            if (currentCaptureSessionMode != nullptr && currentCaptureSessionMode->isVideoRecordSession())
                return;

            // previous mode has to be stopped first
            jassert (currentCaptureSessionMode.get() == nullptr);

            jassert (scopedCameraDevice != nullptr && scopedCameraDevice->openedOk());

            if (scopedCameraDevice == nullptr || ! scopedCameraDevice->openedOk())
                return;

            currentCaptureSessionMode.reset (new CaptureSessionVideoRecordingMode (*this, *scopedCameraDevice, handler,
                                                                                   previewDisplay, mr,
                                                                                   getCameraSensorOrientation(),
                                                                                   getCameraLensFacing(),
                                                                                   streamConfigurationMap));
        */
    }
    
    pub fn on_activity_paused(&mut self, _0: jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("appPaused, closing camera...");

            appWasPaused = true;

            deviceOrientationChangeListener.setEnabled (false);

            // We need to restart the whole session mode when the app gets resumed.
            currentCaptureSessionMode.reset();

            if (scopedCameraDevice != nullptr)
                scopedCameraDevice->close();

            stopBackgroundThread();
        */
    }
    
    pub fn on_activity_resumed(&mut self, _0: jobject)  {
        
        todo!();
        /*
            // Only care about resumed event when paused event was called first.
            if (! appWasPaused)
                return;

            ALOE_CAMERA_LOG ("appResumed, opening camera...");

            deviceOrientationChangeListener.setEnabled (true);

            startBackgroundThread();

            if (scopedCameraDevice != nullptr)
                scopedCameraDevice->open();
        */
    }
    
    pub fn start_background_thread(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

            handlerThread = GlobalRef (LocalRef<jobject> (env->NewObject (AndroidHandlerThread,
                                                                          AndroidHandlerThread.constructor,
                                                                          javaString ("AloeCameraDeviceBackgroundThread").get())));
            // handler thread has to be started before its looper can be fetched
            env->CallVoidMethod (handlerThread, AndroidHandlerThread.start);
            handler = GlobalRef (LocalRef<jobject> (env->NewObject (AndroidHandler,
                                                                    AndroidHandler.constructorWithLooper,
                                                                    env->CallObjectMethod (handlerThread, AndroidHandlerThread.getLooper))));
        */
    }
    
    pub fn stop_background_thread(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

            auto quitSafelyMethod = env->GetMethodID(AndroidHandlerThread, "quitSafely", "()Z");

            // this code will only run on SDK >= 21
            jassert(quitSafelyMethod != nullptr);

            env->CallBooleanMethod (handlerThread, quitSafelyMethod);
            env->CallVoidMethod (handlerThread, AndroidHandlerThread.join);

            jniCheckHasExceptionOccurredAndClear();

            handlerThread.clear();
            handler.clear();
        */
    }
}
