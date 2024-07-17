crate::ix!();

pub struct ScopedCameraDevice<'a> {
    owner:                 &'a mut CameraDevicePimpl<'a>,
    camera_id:             String,
    camera_manager:        &'a mut GlobalRef,
    handler:               &'a mut GlobalRef,
    camera_state_callback: GlobalRef,
    auto_focus_mode:       i32,
    camera_device:         GlobalRef,
    pending_open:          Atomic<i32>, // default = { 0  }
    pending_close:         Atomic<i32>, // default = { 0  }
    fatal_error_occurred:  Atomic<i32>, // default = { 0  }
    open_error:            String,
    closed_event:          WaitableEvent,
}

impl<'a> Drop for ScopedCameraDevice<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            close();
        */
    }
}

impl<'a> ScopedCameraDevice<'a> {

    pub fn new(
        owner_to_use:           &mut CameraDevicePimpl,
        camera_id_to_use:       &String,
        camera_manager_to_use:  &mut GlobalRef,
        handler_to_use:         &mut GlobalRef,
        auto_focus_mode_to_use: i32) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                  cameraId (cameraIdToUse),
                  cameraManager (cameraManagerToUse),
                  handler (handlerToUse),
                  cameraStateCallback (createCameraStateCallbackObject()),
                  autoFocusMode (autoFocusModeToUse)
                open();
        */
    }
    
    pub fn open(&mut self)  {
        
        todo!();
        /*
            pendingOpen.set (1);

                auto* env = getEnv();

                env->CallVoidMethod (cameraManager, CameraManager.openCamera,
                                     javaString (cameraId).get(),
                                     cameraStateCallback.get(), handler.get());

                // If something went wrong we will be pinged in cameraDeviceStateError()
                // callback, silence the redundant exception.
                jniCheckHasExceptionOccurredAndClear();
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (pendingClose.compareAndSetBool (1, 0))
                {
                    auto* env = getEnv();

                    if (cameraDevice.get() != nullptr)
                    {
                        env->CallVoidMethod (cameraDevice, AndroidCameraDevice.close);
                        closedEvent.wait (-1);
                    }

                    pendingClose.set (0);
                    pendingOpen .set (0);
                    cameraDevice.clear();
                }
        */
    }
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return cameraDevice != nullptr;
        */
    }
    
    pub fn has_error_occurred(&self) -> bool {
        
        todo!();
        /*
            return fatalErrorOccurred.get();
        */
    }
    
    pub fn create_capture_session(&mut self, 
        cc:                       &mut dyn CaptureSessionConfiguredCallback,
        surfaces_list:            &LocalRef<jobject>,
        handler_to_use:           &mut GlobalRef,
        capture_session_template: i32) -> *mut CaptureSession {
        
        todo!();
        /*
            if (! openedOk())
                {
                    jassertfalse;
                    return nullptr;
                }

                return new CaptureSession (*this, cc, surfacesList, handlerToUse, captureSessionTemplate, autoFocusMode);
        */
    }
    
    pub fn create_camera_state_callback_object(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->NewObject (CameraDeviceStateCallback,
                                                               CameraDeviceStateCallback.constructor,
                                                               reinterpret_cast<jlong> (this)));
        */
    }
    
    pub fn camera_device_state_closed(&mut self)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraDeviceStateClosed()");

                closedEvent.signal();
        */
    }
    
    pub fn camera_device_state_disconnected(&mut self)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraDeviceStateDisconnected()");

                if (pendingOpen.compareAndSetBool (0, 1))
                {
                    openError = "Device disconnected";

                    notifyOpenResult();
                }

                MessageManager::callAsync ([this]() { close(); });
        */
    }
    
    pub fn camera_device_state_error(&mut self, error_code: i32)  {
        
        todo!();
        /*
            String error = cameraErrorCodeToString (errorCode);

                ALOE_CAMERA_LOG ("cameraDeviceStateError(), error: " + error);

                if (pendingOpen.compareAndSetBool (0, 1))
                {
                    openError = error;

                    notifyOpenResult();
                }

                fatalErrorOccurred.set (1);

                MessageManager::callAsync ([this, error]()
                                           {
                                               owner.cameraDeviceError (error);
                                               close();
                                           });
        */
    }
    
    pub fn camera_device_state_opened(&mut self, camera_device_to_use: &LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraDeviceStateOpened()");

                pendingOpen.set (0);

                cameraDevice = GlobalRef (cameraDeviceToUse);

                notifyOpenResult();
        */
    }
    
    pub fn notify_open_result(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([this]() { owner.cameraOpenFinished (openError); });
        */
    }

    #[JNICALL]
    pub fn camera_device_state_closed_callback(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64,
        _3:   jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<ScopedCameraDevice*>(host))
                    myself->cameraDeviceStateClosed();
        */
    }
    
    #[JNICALL]
    pub fn camera_device_state_disconnected_callback(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64,
        _3:   jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<ScopedCameraDevice*>(host))
                    myself->cameraDeviceStateDisconnected();
        */
    }
    
    #[JNICALL]
    pub fn camera_device_state_error_callback(
        _0:    *mut JNIEnv,
        _1:    jobject,
        host:  i64,
        _3:    jobject,
        error: i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<ScopedCameraDevice*>(host))
                    myself->cameraDeviceStateError (error);
        */
    }
    
    #[JNICALL]
    pub fn camera_device_state_opened_callback(
        _0:         *mut JNIEnv,
        _1:         jobject,
        host:       i64,
        raw_camera: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<ScopedCameraDevice*>(host))
                {
                    LocalRef<jobject> camera(getEnv()->NewLocalRef(rawCamera));

                    myself->cameraDeviceStateOpened (camera);
                }
        */
    }
}
