crate::ix!();

/**
  | NB: FileOutputRecordingDelegateClass
  | callbacks can be called from any thread
  | (incl.  the message thread), so waiting for
  | an event when stopping recording is not an
  | option and VideoRecorder must be alive at
  | all times in order to get stopped recording
  | callback.
  */
pub struct VideoRecorder {
    movie_file_output:            *mut AVCaptureMovieFileOutput,
    delegate:                     Box<AVCaptureFileOutputRecordingDelegate,NSObjectDeleter>,
    recording_in_progress:        bool, // default = false
    first_recorded_frame_time_ms: Atomic<i64>, // default = { 0  }
}

impl Drop for VideoRecorder {

    fn drop(&mut self) {
        todo!();
        /*
            stopRecording();

                // Shutting down a device while recording will stop the recording
                // abruptly and the recording will be lost.
                jassert (! recordingInProgress);
        */
    }
}

impl VideoRecorder {

    pub fn new(session: &mut CaptureSession) -> Self {
    
        todo!();
        /*


            : movieFileOutput ([AVCaptureMovieFileOutput new]),
                  delegate (nullptr)

                static FileOutputRecordingDelegateClass cls;
                delegate.reset ([cls.createInstance() init]);
                FileOutputRecordingDelegateClass::setOwner (delegate.get(), this);

                session.addOutputIfPossible (movieFileOutput);
        */
    }
    
    pub fn start_recording(&mut self, 
        file:               &File,
        orientation_to_use: AVCaptureVideoOrientation)  {
        
        todo!();
        /*
            if (CameraDevicePimpl::getIOSVersion().major >= 10)
                    printVideoOutputDebugInfo (movieFileOutput);

                auto url = [NSURL fileURLWithPath: aloeStringToNS (file.getFullPathName())
                                      isDirectory: NO];

                auto outputConnection = [movieFileOutput connectionWithMediaType: AVMediaTypeVideo];
                outputConnection.videoOrientation = orientationToUse;

                [movieFileOutput startRecordingToOutputFileURL: url recordingDelegate: delegate.get()];
        */
    }
    
    pub fn stop_recording(&mut self)  {
        
        todo!();
        /*
            [movieFileOutput stopRecording];
        */
    }
    
    pub fn get_time_of_first_recorded_frame(&self) -> Time {
        
        todo!();
        /*
            return Time (firstRecordedFrameTimeMs.get());
        */
    }
    
    pub fn print_video_output_debug_info(output: *mut AVCaptureMovieFileOutput)  {
        
        todo!();
        /*
            ignoreUnused (output);

                ALOE_CAMERA_LOG ("Available video codec types:");

               #if ALOE_CAMERA_LOG_ENABLED
                for (AVVideoCodecType type in output.availableVideoCodecTypes)
                    ALOE_CAMERA_LOG (nsStringToAloe (type));
               #endif

                ALOE_CAMERA_LOG ("Output settings per video connection:");

               #if ALOE_CAMERA_LOG_ENABLED
                for (AVCaptureConnection* connection in output.connections)
                    ALOE_CAMERA_LOG (nsStringToAloe ([[output outputSettingsForConnection: connection] description]));
               #endif
        */
    }
}
