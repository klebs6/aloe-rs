crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/aaudio/AudioStreamAAudio.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/aaudio/AudioStreamAAudio.cpp]

/**
  | Workaround state problems in AAudio
  |
  | TODO Which versions does this occur in? Verify
  | fixed in Q.
  */
#[cfg(not(OBOE_FIX_FORCE_STARTING_TO_STARTED))]
pub const OBOE_FIX_FORCE_STARTING_TO_STARTED: usize = 1;

lazy_static!{
    /*
    AAudioLoader *AudioStreamAAudio::mLibLoader = nullptr;
    */
}

/**
  | 'C' wrapper for the data callback method
  |
  */
pub fn oboe_aaudio_data_callback_proc(
        stream:     *mut AAudioStream,
        user_data:  *mut c_void,
        audio_data: *mut c_void,
        num_frames: i32) -> AAudioDataCallbackResult {
    
    todo!();
    /*
        AudioStreamAAudio *oboeStream = reinterpret_cast<AudioStreamAAudio*>(userData);
        if (oboeStream != nullptr) {
            return static_cast<aaudio_data_callback_result_t>(
                    oboeStream->callOnAudioReady(stream, audioData, numFrames));

        } else {
            return static_cast<aaudio_data_callback_result_t>(DataCallbackResult<(),()>::Stop);
        }
    */
}

/**
  | This runs in its own thread.
  |
  | Only one of these threads will be launched from
  | internalErrorCallback().
  |
  | It calls app error callbacks from a static
  | function in case the stream gets deleted.
  */
pub fn oboe_aaudio_error_thread_proc(
        oboe_stream: *mut AudioStreamAAudio,
        error:       OboeResult)  {
    
    todo!();
    /*
        LOGD("%s(,%d) - entering >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>", __func__, error);
        AudioStreamErrorCallback *errorCallback = oboeStream->getErrorCallback();
        if (errorCallback == nullptr) return; // should be impossible
        bool isErrorHandled = errorCallback->onError(oboeStream, error);

        if (!isErrorHandled) {
            oboeStream->requestStop();
            errorCallback->onErrorBeforeClose(oboeStream, error);
            oboeStream->close();
            // Warning, oboeStream may get deleted by this callback.
            errorCallback->onErrorAfterClose(oboeStream, error);
        }
        LOGD("%s() - exiting <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<", __func__);
    */
}

/**
  | This runs in its own thread.
  |
  | Only one of these threads will be launched from
  | internalErrorCallback().
  |
  | Prevents deletion of the stream if the app is
  | using AudioStreamBuilder::openSharedStream()
  */
pub fn oboe_aaudio_error_thread_proc_shared(
        shared_stream: Arc<AudioStream>,
        error:         OboeResult)  {
    
    todo!();
    /*
        AudioStreamAAudio *oboeStream = reinterpret_cast<AudioStreamAAudio*>(sharedStream.get());
        oboe_aaudio_error_thread_proc(oboeStream, error);
    */
}

pub fn oboe_stop_thread_proc(oboe_stream: *mut AudioStream)  {
    
    todo!();
    /*
        if (oboeStream != nullptr) {
            oboeStream->requestStop();
        }
    */
}
