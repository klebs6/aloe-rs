crate::ix!();

pub struct MediaRecorder<'a> {
    on_info_listener:      MediaRecorderOnInfoListener<'a>,
    on_error_listener:     MediaRecorderOnErrorListener<'a>,
    media_recorder:        GlobalRef,
    has_started_recording: bool, // default = false
    orientations_enabled:  i32, // default = -1
}

impl<'a> MediaRecorderOnErrorListenerOwner for MediaRecorder<'a> {

    fn on_error(&mut self, 
        recorder: &mut LocalRef<jobject>,
        what:     i32,
        extra:    i32)  {
        
        todo!();
        /*
            ignoreUnused (recorder, what, extra);

                ALOE_CAMERA_LOG ("MediaRecorder::onError: " + getErrorStringFromCode (what)
                                         + ", extra code = " + String (extra));
        */
    }
}

impl<'a> MediaRecorderOnInfoListenerOwner for MediaRecorder<'a> {

    fn on_info(&mut self, 
        recorder: &mut LocalRef<jobject>,
        what:     i32,
        extra:    i32)  {
        
        todo!();
        /*
            ignoreUnused (recorder, what, extra);

                ALOE_CAMERA_LOG ("MediaRecorder::OnInfo: " + getInfoStringFromCode (what)
                                         + ", extra code = " + String (extra));
        */
    }
}

impl<'a> Drop for MediaRecorder<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            getEnv()->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.release);
        */
    }
}

impl<'a> MediaRecorder<'a> {

    pub fn new(
        output_file_path:   &String,
        video_width:        i32,
        video_height:       i32,
        sensor_orientation: i32,
        camera_lens_facing: i32) -> Self {
    
        todo!();
        /*


            : onInfoListener (*this),
                  onErrorListener (*this),
                  mediaRecorder (LocalRef<jobject> (getEnv()->NewObject (AndroidMediaRecorder,
                                                                         AndroidMediaRecorder.constructor)))
                auto* env = getEnv();

                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setOnInfoListener,
                                     CreateJavaInterface (&onInfoListener,
                                                          "android/media/MediaRecorder$OnInfoListener").get());

                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setOnErrorListener,
                                     CreateJavaInterface (&onErrorListener,
                                                          "android/media/MediaRecorder$OnErrorListener").get());

                // NB: the order of function calls here is enforced, and exceptions will be thrown if
                //     the order is changed.
                static constexpr int audioSourceMic = 1;
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setAudioSource, (jint) audioSourceMic);

                static constexpr int videoSourceSurface = 2;
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setVideoSource, (jint) videoSourceSurface);

                static constexpr int outputFormatMPEG4 = 2;
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setOutputFormat, (jint) outputFormatMPEG4);

                static constexpr int audioEncoderAAC = 3;
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setAudioEncoder, (jint) audioEncoderAAC);

                static constexpr int videoEncoderH264 = 2;
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setVideoEncoder, (jint) videoEncoderH264);

                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setVideoEncodingBitRate, (jint) 10000000);
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setVideoFrameRate, (jint) 30);

                auto frontFacing = cameraLensFacing == 0;

                auto useInverseDegrees = frontFacing && sensorOrientation == 90;

                int orientationHint = getOrientationHint (useInverseDegrees, sensorOrientation);
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setOrientationHint, (jint) orientationHint);

                getEnv()->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setVideoSize, (jint) videoWidth, (jint) videoHeight);
                getEnv()->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.setOutputFile, javaString (outputFilePath).get());
                getEnv()->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.prepare);
        */
    }
    
    pub fn get_surface(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->CallObjectMethod (mediaRecorder, AndroidMediaRecorder.getSurface));
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            lockScreenOrientation();

                getEnv()->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.start);

                hasStartedRecording = true;
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            // A request to stop can be sent before recording has had a chance to start, so
                // ignore the request rather than calling AndroidMediaRecorder.stop because
                // otherwise MediaRecorder will throw an exception and...
                if (! hasStartedRecording)
                    return;

                hasStartedRecording = false;

                auto* env = getEnv();
                env->CallVoidMethod (mediaRecorder, AndroidMediaRecorder.stop);

                // ... ignore RuntimeException that can be thrown if stop() was called after recording
                // has started but before any frame was written to a file. This is not an error.
                jniCheckHasExceptionOccurredAndClear();

                unlockScreenOrientation();
        */
    }
    
    pub fn lock_screen_orientation(&mut self)  {
        
        todo!();
        /*
            orientationsEnabled = Desktop::getInstance().getOrientationsEnabled();

                auto o = Desktop::getInstance().getCurrentOrientation();
                Desktop::getInstance().setOrientationsEnabled (o);
        */
    }
    
    pub fn aloe_orientation_to_native_orientation(orientations: i32) -> i32 {
        
        todo!();
        /*
            enum
                {
                    SCREEN_ORIENTATION_LANDSCAPE          = 0,
                    SCREEN_ORIENTATION_PORTRAIT           = 1,
                    SCREEN_ORIENTATION_USER               = 2,
                    SCREEN_ORIENTATION_REVERSE_LANDSCAPE  = 8,
                    SCREEN_ORIENTATION_REVERSE_PORTRAIT   = 9,
                    SCREEN_ORIENTATION_USER_LANDSCAPE     = 11,
                    SCREEN_ORIENTATION_USER_PORTRAIT      = 12,
                };

                switch (orientations)
                {
                    case Desktop::upright:                                          return (jint) SCREEN_ORIENTATION_PORTRAIT;
                    case Desktop::upsideDown:                                       return (jint) SCREEN_ORIENTATION_REVERSE_PORTRAIT;
                    case Desktop::upright + Desktop::upsideDown:                    return (jint) SCREEN_ORIENTATION_USER_PORTRAIT;
                    case Desktop::rotatedAntiClockwise:                             return (jint) SCREEN_ORIENTATION_LANDSCAPE;
                    case Desktop::rotatedClockwise:                                 return (jint) SCREEN_ORIENTATION_REVERSE_LANDSCAPE;
                    case Desktop::rotatedClockwise + Desktop::rotatedAntiClockwise: return (jint) SCREEN_ORIENTATION_USER_LANDSCAPE;
                    default:                                                        return (jint) SCREEN_ORIENTATION_USER;
                }
        */
    }
    
    pub fn unlock_screen_orientation(&mut self)  {
        
        todo!();
        /*
            Desktop::getInstance().setOrientationsEnabled (orientationsEnabled);
        */
    }
    
    pub fn get_info_string_from_code(what: i32) -> String {
        
        todo!();
        /*
            enum
                {
                    MEDIA_RECORDER_INFO_UNKNOWN = 1,
                    MEDIA_RECORDER_INFO_MAX_DURATION_REACHED = 800,
                    MEDIA_RECORDER_INFO_MAX_FILESIZE_REACHED = 801,
                    MEDIA_RECORDER_INFO_MAX_FILESIZE_APPROACHING = 802,
                    MEDIA_RECORDER_INFO_NEXT_OUTPUT_FILE_STARTED = 803
                };

                switch (what)
                {
                    case MEDIA_RECORDER_INFO_UNKNOWN:                  return { "Unknown info" };
                    case MEDIA_RECORDER_INFO_MAX_DURATION_REACHED:     return { "Max duration reached" };
                    case MEDIA_RECORDER_INFO_MAX_FILESIZE_REACHED:     return { "Max filesize reached" };
                    case MEDIA_RECORDER_INFO_MAX_FILESIZE_APPROACHING: return { "Max filesize approaching" };
                    case MEDIA_RECORDER_INFO_NEXT_OUTPUT_FILE_STARTED: return { "Next output file started" };
                    default: return String (what);
                };
        */
    }
    
    pub fn get_error_string_from_code(what: i32) -> String {
        
        todo!();
        /*
            enum
                {
                    MEDIA_RECORDER_ERROR_UNKNOWN = 1,
                    MEDIA_ERROR_SERVER_DIED = 100
                };

                switch (what)
                {
                    case MEDIA_RECORDER_ERROR_UNKNOWN:   return { "Unknown error" };
                    case MEDIA_ERROR_SERVER_DIED:        return { "Server died" };
                    default: return String (what);
                };
        */
    }
    
    pub fn get_orientation_hint(
        use_inverse_degrees:       bool,
        camera_sensor_orientation: i32) -> i32 {
        
        todo!();
        /*
            auto* env = getEnv();

                auto windowManager = LocalRef<jobject> (env->CallObjectMethod (getAppContext(), AndroidContext.getSystemService, javaString ("window").get()));
                auto display = LocalRef<jobject> (env->CallObjectMethod (windowManager, AndroidWindowManager.getDefaultDisplay));
                auto rotation = env->CallIntMethod (display, AndroidDisplay.getRotation);

                enum
                {
                    ROTATION_0 = 0,
                    ROTATION_90,
                    ROTATION_180,
                    ROTATION_270
                };

                int hint = 0;

                switch (rotation)
                {
                    case ROTATION_0:   hint = cameraSensorOrientation;       break;
                    case ROTATION_90:  hint = useInverseDegrees ? 180 : 0;   break;
                    case ROTATION_180: hint = cameraSensorOrientation + 180; break;
                    case ROTATION_270: hint = useInverseDegrees ? 0 : 180;   break;
                    default: jassertfalse;
                }

                return (hint + 360) % 360;
        */
    }
}
