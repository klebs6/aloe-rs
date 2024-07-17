crate::ix!();

/**
   Define missing types from AAudio.h
  */
pub type AAudioStreamState        = i32;
pub type AAudioDirection          = i32;
pub type AAudioFormat             = i32;
pub type AAudioDataCallbackResult = i32;
pub type AAudioResult             = i32;
pub type AAudioSharingMode        = i32;
pub type AAudioPerformanceMode    = i32;

pub type AAudioStream             = AAudioStreamStruct;
pub type AAudioStreamBuilder      = AAudioStreamBuilderStruct;

pub type AAudioStreamDataCallback  = fn(
    stream:     *mut AAudioStream, 
    user_data:  *mut c_void, 
    audio_data: *mut c_void, 
    num_frames: i32) -> AAudioDataCallbackResult;

pub type AAudioStreamErrorCallback = fn(
    stream:    *mut AAudioStream, 
    user_data: *mut c_void, 
    error:     AAudioResult
) -> c_void;

/**
   These were defined in P
  */
pub type AAudioUsage       = i32;
pub type AAudioContentType = i32;
pub type AAudioInputPreset = i32;
pub type AAudioSessionId   = i32;

/**
   There are a few definitions used by Oboe.
  */
macro_rules! aaudio_ok {
    () => {
        /*
                static_cast<AAudioResult>(OboeResult::OK)
        */
    }
}

macro_rules! aaudio_error_timeout {
    () => {
        /*
                static_cast<AAudioResult>(OboeResult::ErrorTimeout)
        */
    }
}

macro_rules! aaudio_stream_state_starting {
    () => {
        /*
                static_cast<AAudioStreamState>(StreamState::Starting)
        */
    }
}

macro_rules! aaudio_stream_state_started {
    () => {
        /*
                static_cast<AAudioStreamState>(StreamState::Started)
        */
    }
}
