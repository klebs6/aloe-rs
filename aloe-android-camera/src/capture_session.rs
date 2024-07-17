crate::ix!();

#[weak_referenceable]
#[no_copy]
pub struct CaptureSession<'a> {
    scoped_camera_device:           &'a mut ScopedCameraDevice<'a>,
    configured_callback:            &'a mut dyn CaptureSessionConfiguredCallback,
    handler:                        &'a mut GlobalRef,
    capture_request_builder:        GlobalRef,
    preview_capture_request:        GlobalRef,
    capture_session_state_callback: GlobalRef,
    auto_focus_mode:                i32,
    capture_session:                GlobalRef,
    capture_session_lock:           CriticalSection,
    pending_close:                  Atomic<i32>, // default = { 0  }
    still_picture_taker:            Box<StillPictureTaker<'a>>,
    closed_event:                   WaitableEvent,
}

impl<'a> Drop for CaptureSession<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            bool calledClose = false;

                    auto* env = getEnv();

                    {
                        const ScopedLock lock (captureSessionLock);

                        if (captureSession.get() != nullptr)
                        {
                            calledClose = true;

                            env->CallVoidMethod (captureSession, CameraCaptureSession.close);
                        }
                    }

                    // When exception occurs, CameraCaptureSession.close will never finish, so
                    // we should not wait for it. For fatal error an exception does occur, but
                    // it is caught internally in Java...
                    if (jniCheckHasExceptionOccurredAndClear() || scopedCameraDevice.fatalErrorOccurred.get())
                    {
                        ALOE_CAMERA_LOG ("Exception or fatal error occurred while closing Capture Session, closing by force");
                    }
                    else if (calledClose)
                    {
                        pendingClose.set (1);
                        closedEvent.wait (-1);
                    }
        */
    }
}

impl<'a> CaptureSession<'a> {
    
    pub fn opened_ok(&self) -> bool {
        
        todo!();
        /*
            return captureSession != nullptr;
        */
    }
    
    pub fn get_native_session(&self) -> &GlobalRef {
        
        todo!();
        /*
            return captureSession;
        */
    }
    
    pub fn start(&mut self, 
        target_surfaces_list: &LocalRef<jobject>,
        handler_to_use:       &mut GlobalRef) -> bool {
        
        todo!();
        /*
            if (! openedOk())
                    {
                        jassertfalse;
                        return false;
                    }

                    auto* env = getEnv();

                    auto numSurfaces = env->CallIntMethod (targetSurfacesList, JavaArrayList.size);

                    for (int i = 0; i < numSurfaces; ++i)
                    {
                        auto surface = LocalRef<jobject> (env->CallObjectMethod (targetSurfacesList, JavaArrayList.get, (jint) i));
                        env->CallVoidMethod (captureRequestBuilder, CaptureRequestBuilder.addTarget, surface.get());
                    }

                    previewCaptureRequest = GlobalRef (LocalRef<jobject>(env->CallObjectMethod (captureRequestBuilder, CaptureRequestBuilder.build)));

                    env->CallIntMethod (captureSession, CameraCaptureSession.setRepeatingRequest,
                                        previewCaptureRequest.get(), nullptr, handlerToUse.get());

                    return true;
        */
    }
    
    pub fn take_still_picture(&mut self, target_surface: jobject)  {
        
        todo!();
        /*
            if (stillPictureTaker == nullptr)
                    {
                        // Can only take picture once session was successfully configured!
                        jassertfalse;
                        return;
                    }

                    auto* env = getEnv();

                    static constexpr int templateStillCapture = 2;
                    auto builder = LocalRef<jobject> (env->CallObjectMethod (scopedCameraDevice.cameraDevice,
                                                                             AndroidCameraDevice.createCaptureRequest,
                                                                             (jint) templateStillCapture));

                    env->CallVoidMethod (builder, CaptureRequestBuilder.addTarget, targetSurface);

                    setCaptureRequestBuilderIntegerKey (builder, CaptureRequest.CONTROL_AF_MODE, autoFocusMode);

                    auto stillPictureCaptureRequest = LocalRef<jobject> (env->CallObjectMethod (builder, CaptureRequestBuilder.build));

                    stillPictureTaker->takePicture (stillPictureCaptureRequest);
        */
    }
    
    pub fn new(
        scoped_camera_device_to_use: &mut ScopedCameraDevice,
        configured_callback_to_use:  &mut dyn CaptureSessionConfiguredCallback,
        surfaces_list:               &LocalRef<jobject>,
        handler_to_use:              &mut GlobalRef,
        capture_session_template:    i32,
        auto_focus_mode_to_use:      i32) -> Self {
    
        todo!();
        /*


            : scopedCameraDevice (scopedCameraDeviceToUse),
                      configuredCallback (configuredCallbackToUse),
                      handler (handlerToUse),
                      captureRequestBuilder (LocalRef<jobject> (getEnv()->CallObjectMethod (scopedCameraDevice.cameraDevice,
                                                                                            AndroidCameraDevice.createCaptureRequest,
                                                                                            (jint) captureSessionTemplate))),
                      captureSessionStateCallback (LocalRef<jobject> (getEnv()->NewObject (CameraCaptureSessionStateCallback,
                                                                                           CameraCaptureSessionStateCallback.constructor,
                                                                                           reinterpret_cast<jlong> (this)))),
                      autoFocusMode (autoFocusModeToUse)
                    auto* env = getEnv();

                    env->CallVoidMethod (scopedCameraDevice.cameraDevice, AndroidCameraDevice.createCaptureSession,
                                         surfacesList.get(), captureSessionStateCallback.get(), handler.get());

                    static constexpr int controlModeAuto = 1;
                    setCaptureRequestBuilderIntegerKey (captureRequestBuilder.get(), CaptureRequest.CONTROL_MODE, controlModeAuto);

                    setCaptureRequestBuilderIntegerKey (captureRequestBuilder.get(), CaptureRequest.CONTROL_AF_MODE, autoFocusMode);
        */
    }
    
    pub fn set_capture_request_builder_integer_key(
        capture_request_builder: jobject,
        key:                     JFieldID,
        value:                   i32)  {
        
        todo!();
        /*
            auto* env = getEnv();

                    auto jKey = LocalRef<jobject> (env->GetStaticObjectField (CaptureRequest, key));
                    auto jValue = LocalRef<jobject> (env->CallStaticObjectMethod (JavaInteger, JavaInteger.valueOf, (jint) value));

                    env->CallVoidMethod (captureRequestBuilder, CaptureRequestBuilder.set, jKey.get(), jValue.get());
        */
    }
    
    pub fn camera_capture_session_active(&mut self, session: jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionActive()");
                    ignoreUnused (session);
        */
    }
    
    pub fn camera_capture_session_closed(&mut self, session: jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionClosed()");
                    ignoreUnused (session);

                    closedEvent.signal();
        */
    }
    
    pub fn camera_capture_session_configure_failed(&mut self, session: jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionConfigureFailed()");
                    ignoreUnused (session);

                    MessageManager::callAsync ([weakRef = WeakReference<CaptureSession> { this }]
                    {
                        if (weakRef != nullptr)
                            weakRef->configuredCallback.captureSessionConfigured (nullptr);
                    });
        */
    }
    
    pub fn camera_capture_session_configured(&mut self, session: &LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionConfigured()");

                    if (pendingClose.get() == 1)
                    {
                        // Already closing, bailout.
                        closedEvent.signal();

                        GlobalRef s (session);

                        MessageManager::callAsync ([s]()
                            {
                                getEnv()->CallVoidMethod (s, CameraCaptureSession.close);
                            });

                        return;
                    }

                    {
                        const ScopedLock lock (captureSessionLock);
                        captureSession = GlobalRef (session);
                    }

                    MessageManager::callAsync ([weakRef = WeakReference<CaptureSession> { this }]
                    {
                        if (weakRef == nullptr)
                            return;

                        weakRef->stillPictureTaker.reset (new StillPictureTaker (weakRef->captureSession,
                                                                                 weakRef->captureRequestBuilder,
                                                                                 weakRef->previewCaptureRequest,
                                                                                 weakRef->handler,
                                                                                 weakRef->autoFocusMode));

                        weakRef->configuredCallback.captureSessionConfigured (weakRef.get());
                    });
        */
    }
    
    pub fn camera_capture_session_ready(&mut self, session: &LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionReady()");
                    ignoreUnused (session);
        */
    }
    
    pub fn camera_capture_session_active_callback(
        _0:          *mut JNIEnv,
        _1:          jobject,
        host:        i64,
        raw_session: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<CaptureSession*> (host))
                    {
                        LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                        myself->cameraCaptureSessionActive (session);
                    }
        */
    }
    
    pub fn camera_capture_session_closed_callback(
        _0:          *mut JNIEnv,
        _1:          jobject,
        host:        i64,
        raw_session: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<CaptureSession*> (host))
                    {
                        LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                        myself->cameraCaptureSessionClosed (session);
                    }
        */
    }
    
    pub fn camera_capture_session_configure_failed_callback(
        _0:          *mut JNIEnv,
        _1:          jobject,
        host:        i64,
        raw_session: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<CaptureSession*> (host))
                    {
                        LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                        myself->cameraCaptureSessionConfigureFailed (session);
                    }
        */
    }
    
    pub fn camera_capture_session_configured_callback(
        _0:          *mut JNIEnv,
        _1:          jobject,
        host:        i64,
        raw_session: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<CaptureSession*> (host))
                    {
                        LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                        myself->cameraCaptureSessionConfigured (session);
                    }
        */
    }
    
    pub fn camera_capture_session_ready_callback(
        _0:          *mut JNIEnv,
        _1:          jobject,
        host:        i64,
        raw_session: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<CaptureSession*> (host))
                    {
                        LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                        myself->cameraCaptureSessionReady (session);
                    }
        */
    }
}
