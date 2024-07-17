crate::ix!();

pub struct StillPictureTaker<'a> {
    capture_session:                                &'a mut GlobalRef,
    capture_request_builder:                        &'a mut GlobalRef,
    preview_capture_request:                        &'a mut GlobalRef,
    handler:                                        &'a mut GlobalRef,
    runnable:                                       AndroidRunnable<'a>,
    delayed_capture_runnable:                       GlobalRef,
    capture_session_preview_capture_callback:       GlobalRef,
    still_picture_capture_request:                  GlobalRef,
    capture_session_still_picture_capture_callback: GlobalRef,
    auto_focus_mode:                                i32,
    current_state:                                  StillPictureTakerState, //= StillPictureTakerState::idle;
}

impl<'a> AndroidRunnableOwner for StillPictureTaker<'a> {

    fn run(&mut self)  {
        
        todo!();
        /*
            captureStillPicture();
        */
    }
}

impl<'a> StillPictureTaker<'a> {

    pub fn new(
        capture_session_to_use:         &mut GlobalRef,
        capture_request_builder_to_use: &mut GlobalRef,
        preview_capture_request_to_use: &mut GlobalRef,
        handler_to_use:                 &mut GlobalRef,
        auto_focus_mode_to_use:         i32) -> Self {
    
        todo!();
        /*


            : captureSession (captureSessionToUse),
                  captureRequestBuilder (captureRequestBuilderToUse),
                  previewCaptureRequest (previewCaptureRequestToUse),
                  handler (handlerToUse),
                  runnable (*this),
                  captureSessionPreviewCaptureCallback (createCaptureSessionCallback (true)),
                  captureSessionStillPictureCaptureCallback (createCaptureSessionCallback (false)),
                  autoFocusMode (autoFocusModeToUse)
        */
    }
    
    pub fn take_picture(&mut self, still_picture_capture_request_to_use: &LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("Taking picture...");

                stillPictureCaptureRequest = GlobalRef (LocalRef<jobject>(stillPictureCaptureRequestToUse));

                lockFocus();
        */
    }
    
    pub fn create_capture_session_callback(&mut self, create_preview_session: bool) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject>(getEnv()->NewObject (CameraCaptureSessionCaptureCallback,
                                                              CameraCaptureSessionCaptureCallback.constructor,
                                                              reinterpret_cast<jlong> (this),
                                                              createPreviewSession ? 1 : 0));
        */
    }
    
    pub fn lock_focus(&mut self)  {
        
        todo!();
        /*
            if (jniCheckHasExceptionOccurredAndClear())
                    return;

                ALOE_CAMERA_LOG ("Performing auto-focus if possible...");

                currentState = StillPictureTakerState::pendingFocusLock;

                auto* env = getEnv();

                // NB: auto-focus may be unavailable on a device, in which case it may have already
                // automatically adjusted the exposure. We check for that in updateState().
                static constexpr int controlAfTriggerStart = 1;
                CaptureSession::setCaptureRequestBuilderIntegerKey (captureRequestBuilder.get(),
                                                                    CaptureRequest.CONTROL_AF_TRIGGER,
                                                                    controlAfTriggerStart);

                auto previewRequest = LocalRef<jobject> (env->CallObjectMethod (captureRequestBuilder,
                                                                                CaptureRequestBuilder.build));

                env->CallIntMethod (captureSession, CameraCaptureSession.capture, previewRequest.get(),
                                    captureSessionPreviewCaptureCallback.get(), handler.get());
        */
    }
    
    pub fn update_state(&mut self, capture_result: jobject)  {
        
        todo!();
        /*
            // IllegalStateException can be thrown when accessing CaptureSession,
                // claiming that capture session was already closed but we may not
                // get relevant callback yet, so check for this and bailout when needed.
                if (jniCheckHasExceptionOccurredAndClear())
                    return;

                switch (currentState)
                {
                    case StillPictureTakerState::pendingFocusLock:
                    {
                        ALOE_CAMERA_LOG ("Still picture capture, updateState(), StillPictureTakerState::pendingFocusLock...");

                        auto controlAfStateValue = getCaptureResultIntegerKeyValue (CaptureResult.CONTROL_AF_STATE, captureResult);

                        if (controlAfStateValue.get() == nullptr)
                        {
                            captureStillPictureDelayed();
                            return;
                        }

                        auto autoToFocusNotAvailable = autoFocusMode == 0;

                        if (autoToFocusNotAvailable || autoFocusHasFinished (controlAfStateValue))
                        {
                            auto controlAeStateIntValue = getControlAEState (captureResult);
                            static constexpr int controlAeStateConverged = 2;

                            if (controlAeStateIntValue == -1 || controlAeStateIntValue == controlAeStateConverged)
                            {
                                currentState = StillPictureTakerState::pictureTaken;
                                captureStillPictureDelayed();
                            }
                            else
                            {
                                runPrecaptureSequence();
                            }
                        }

                        break;
                    }

                    case StillPictureTakerState::pendingExposurePrecapture:
                    {
                        ALOE_CAMERA_LOG ("Still picture capture, updateState(), StillPictureTakerState::pendingExposurePrecapture...");

                        auto controlAeStateIntValue = getControlAEState (captureResult);
                        static constexpr int controlAeStateFlashRequired = 4;
                        static constexpr int controlAeStatePrecapture = 5;

                        if (controlAeStateIntValue == -1 || controlAeStateIntValue == controlAeStateFlashRequired
                                                         || controlAeStateIntValue == controlAeStatePrecapture)
                        {
                            currentState = StillPictureTakerState::pendingExposurePostPrecapture;
                        }

                        break;
                    }

                    case StillPictureTakerState::pendingExposurePostPrecapture:
                    {
                        ALOE_CAMERA_LOG ("Still picture capture, updateState(), StillPictureTakerState::pendingExposurePostPrecapture...");

                        auto controlAeStateIntValue = getControlAEState (captureResult);
                        static constexpr int controlAeStatePrecapture = 5;

                        if (controlAeStateIntValue == -1 || controlAeStateIntValue != controlAeStatePrecapture)
                        {
                            currentState = StillPictureTakerState::pictureTaken;
                            captureStillPictureDelayed();
                        }

                        break;
                    }
                    case StillPictureTakerState::idle:
                    case StillPictureTakerState::pictureTaken:
                        { /* do nothing */ break; }
                };
        */
    }
    
    pub fn get_control_ae_state(capture_result: jobject) -> i32 {
        
        todo!();
        /*
            auto controlAeStateValue = getCaptureResultIntegerKeyValue (CaptureResult.CONTROL_AE_STATE, captureResult);

                return controlAeStateValue.get() != nullptr
                                ? getEnv()->CallIntMethod (controlAeStateValue, JavaInteger.intValue) : -1;
        */
    }
    
    pub fn auto_focus_has_finished(control_af_state_value: &LocalRef<jobject>) -> bool {
        
        todo!();
        /*
            static constexpr int controlAfStateFocusedLocked = 4;
                static constexpr int controlAfStateNotFocusedLocked = 5;

                auto controlAfStateIntValue = getEnv()->CallIntMethod (controlAfStateValue, JavaInteger.intValue);

                return controlAfStateIntValue == controlAfStateFocusedLocked || controlAfStateIntValue == controlAfStateNotFocusedLocked;
        */
    }
    
    pub fn get_capture_result_integer_key_value(
        key:            JFieldID,
        capture_result: jobject) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

                auto jKey = LocalRef<jobject> (env->GetStaticObjectField (CaptureResult, key));
                return LocalRef<jobject> (env->CallObjectMethod (captureResult, CaptureResult.get, jKey.get()));
        */
    }
    
    pub fn capture_still_picture_delayed(&mut self)  {
        
        todo!();
        /*
            if (jniCheckHasExceptionOccurredAndClear())
                    return;

                ALOE_CAMERA_LOG ("Still picture capture, device ready, capturing now...");

                auto* env = getEnv();

                env->CallVoidMethod (captureSession, CameraCaptureSession.stopRepeating);

                if (jniCheckHasExceptionOccurredAndClear())
                    return;

                env->CallVoidMethod (captureSession, CameraCaptureSession.abortCaptures);

                if (jniCheckHasExceptionOccurredAndClear())
                    return;

                // Delay still picture capture for devices that can't handle it right after
                // stopRepeating/abortCaptures calls.
                if (delayedCaptureRunnable.get() == nullptr)
                    delayedCaptureRunnable = GlobalRef (CreateJavaInterface (&runnable, "java/lang/Runnable"));

                env->CallBooleanMethod (handler, AndroidHandler.postDelayed, delayedCaptureRunnable.get(), (jlong) 200);
        */
    }
    
    pub fn run_precapture_sequence(&mut self)  {
        
        todo!();
        /*
            if (jniCheckHasExceptionOccurredAndClear())
                    return;

                auto* env = getEnv();

                static constexpr int controlAePrecaptureTriggerStart = 1;
                CaptureSession::setCaptureRequestBuilderIntegerKey (captureRequestBuilder.get(),
                                                                    CaptureRequest.CONTROL_AE_PRECAPTURE_TRIGGER,
                                                                    controlAePrecaptureTriggerStart);

                currentState = StillPictureTakerState::pendingExposurePrecapture;

                auto previewRequest = LocalRef<jobject> (env->CallObjectMethod (captureRequestBuilder,
                                                                                CaptureRequestBuilder.build));

                env->CallIntMethod (captureSession, CameraCaptureSession.capture, previewRequest.get(),
                                    captureSessionPreviewCaptureCallback.get(), handler.get());
        */
    }
    
    pub fn unlock_focus(&mut self)  {
        
        todo!();
        /*
            if (jniCheckHasExceptionOccurredAndClear())
                    return;

                ALOE_CAMERA_LOG ("Unlocking focus...");

                currentState = StillPictureTakerState::idle;

                auto* env = getEnv();

                static constexpr int controlAfTriggerCancel = 2;
                CaptureSession::setCaptureRequestBuilderIntegerKey (captureRequestBuilder.get(),
                                                                    CaptureRequest.CONTROL_AF_TRIGGER,
                                                                    controlAfTriggerCancel);

                auto resetAutoFocusRequest = LocalRef<jobject> (env->CallObjectMethod (captureRequestBuilder,
                                                                                       CaptureRequestBuilder.build));

                env->CallIntMethod (captureSession, CameraCaptureSession.capture, resetAutoFocusRequest.get(),
                                    nullptr, handler.get());

                if (jniCheckHasExceptionOccurredAndClear())
                    return;

                // NB: for preview, using preview capture request again
                env->CallIntMethod (captureSession, CameraCaptureSession.setRepeatingRequest, previewCaptureRequest.get(),
                                    nullptr, handler.get());
        */
    }
    
    pub fn capture_still_picture(&mut self)  {
        
        todo!();
        /*
            getEnv()->CallIntMethod (captureSession, CameraCaptureSession.capture,
                                         stillPictureCaptureRequest.get(), captureSessionStillPictureCaptureCallback.get(),
                                         nullptr);
        */
    }
    
    pub fn camera_capture_session_capture_completed(&mut self, 
        is_preview: bool,
        session:    jobject,
        request:    jobject,
        result:     jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionCaptureCompleted()");

                ignoreUnused (session, request);

                if (isPreview)
                    updateState (result);
                else if (currentState != StillPictureTakerState::idle)
                    unlockFocus();
        */
    }
    
    pub fn camera_capture_session_capture_failed(&mut self, 
        is_preview: bool,
        session:    jobject,
        request:    jobject,
        failure:    jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionCaptureFailed()");

                ignoreUnused (isPreview, session, request, failure);
        */
    }
    
    pub fn camera_capture_session_capture_progressed(&mut self, 
        is_preview:     bool,
        session:        jobject,
        request:        jobject,
        partial_result: jobject)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionCaptureProgressed()");

                ignoreUnused (session, request);

                if (isPreview)
                    updateState (partialResult);
        */
    }
    
    pub fn camera_capture_session_capture_sequence_aborted(&mut self, 
        is_preview:  bool,
        session:     jobject,
        sequence_id: i32)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionCaptureSequenceAborted()");

                ignoreUnused (isPreview, isPreview, session, sequenceId);
        */
    }
    
    pub fn camera_capture_session_capture_sequence_completed(&mut self, 
        is_preview:   bool,
        session:      jobject,
        sequence_id:  i32,
        frame_number: i64)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionCaptureSequenceCompleted()");

                ignoreUnused (isPreview, session, sequenceId, frameNumber);
        */
    }
    
    pub fn camera_capture_session_capture_started(&mut self, 
        is_preview:   bool,
        session:      jobject,
        request:      jobject,
        timestamp:    i64,
        frame_number: i64)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("cameraCaptureSessionCaptureStarted()");

                ignoreUnused (isPreview, session, request, timestamp, frameNumber);
        */
    }
    
    pub fn camera_capture_session_capture_completed_callback(
        _0:          *mut JNIEnv,
        object:      jobject,
        host:        i64,
        is_preview:  bool,
        raw_session: jobject,
        raw_request: jobject,
        raw_result:  jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<StillPictureTaker*> (host))
                {
                    LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));
                    LocalRef<jobject> request (getEnv()->NewLocalRef(rawRequest));
                    LocalRef<jobject> result (getEnv()->NewLocalRef(rawResult));

                    myself->cameraCaptureSessionCaptureCompleted (isPreview != 0, session, request, result);
                }
        */
    }
    
    pub fn camera_capture_session_capture_failed_callback(
        _0:          *mut JNIEnv,
        object:      jobject,
        host:        i64,
        is_preview:  bool,
        raw_session: jobject,
        raw_request: jobject,
        raw_result:  jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<StillPictureTaker*> (host))
                {
                    LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));
                    LocalRef<jobject> request (getEnv()->NewLocalRef(rawRequest));
                    LocalRef<jobject> result (getEnv()->NewLocalRef(rawResult));

                    myself->cameraCaptureSessionCaptureFailed (isPreview != 0, session, request, result);
                }
        */
    }
    
    pub fn camera_capture_session_capture_progressed_callback(
        _0:          *mut JNIEnv,
        object:      jobject,
        host:        i64,
        is_preview:  bool,
        raw_session: jobject,
        raw_request: jobject,
        raw_result:  jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<StillPictureTaker*> (host))
                {
                    LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));
                    LocalRef<jobject> request (getEnv()->NewLocalRef(rawRequest));
                    LocalRef<jobject> result (getEnv()->NewLocalRef(rawResult));

                    myself->cameraCaptureSessionCaptureProgressed (isPreview != 0, session, request, result);
                }
        */
    }
    
    pub fn camera_capture_session_capture_sequence_aborted_callback(
        _0:          *mut JNIEnv,
        object:      jobject,
        host:        i64,
        is_preview:  bool,
        raw_session: jobject,
        sequence_id: i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<StillPictureTaker*> (host))
                {
                    LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                    myself->cameraCaptureSessionCaptureSequenceAborted (isPreview != 0, session, sequenceId);
                }
        */
    }
    
    pub fn camera_capture_session_capture_sequence_completed_callback(
        _0:           *mut JNIEnv,
        object:       jobject,
        host:         i64,
        is_preview:   bool,
        raw_session:  jobject,
        sequence_id:  i32,
        frame_number: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<StillPictureTaker*> (host))
                {
                    LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));

                    myself->cameraCaptureSessionCaptureSequenceCompleted (isPreview != 0, session, sequenceId, frameNumber);
                }
        */
    }
    
    pub fn camera_capture_session_capture_started_callback(
        _0:           *mut JNIEnv,
        object:       jobject,
        host:         i64,
        is_preview:   bool,
        raw_session:  jobject,
        raw_request:  jobject,
        timestamp:    i64,
        frame_number: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<StillPictureTaker*> (host))
                {
                    LocalRef<jobject> session (getEnv()->NewLocalRef(rawSession));
                    LocalRef<jobject> request (getEnv()->NewLocalRef(rawRequest));

                    myself->cameraCaptureSessionCaptureStarted (isPreview != 0, session, request, timestamp, frameNumber);
                }
        */
    }
}
