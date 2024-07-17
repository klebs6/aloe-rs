crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeDebug.h]

#[cfg(not(MODULE_NAME))]
pub const MODULE_NAME: &'static str = "OboeAudio";

/**
   Always log INFO and errors.
  */
macro_rules! logi {
    ($($arg:ident),*) => {
        /*
                __android_log_print(ANDROID_LOG_INFO, MODULE_NAME, __VA_ARGS__)
        */
    }
}

macro_rules! logw {
    ($($arg:ident),*) => {
        /*
                __android_log_print(ANDROID_LOG_WARN, MODULE_NAME, __VA_ARGS__)
        */
    }
}

macro_rules! loge {
    ($($arg:ident),*) => {
        /*
                __android_log_print(ANDROID_LOG_ERROR, MODULE_NAME, __VA_ARGS__)
        */
    }
}

macro_rules! logf {
    ($($arg:ident),*) => {
        /*
                __android_log_print(ANDROID_LOG_FATAL, MODULE_NAME, __VA_ARGS__)
        */
    }
}

///-----------------------------
#[cfg(OBOE_ENABLE_LOGGING)]
macro_rules! logv {
    ($($arg:ident),*) => {
        /*
                __android_log_print(ANDROID_LOG_VERBOSE, MODULE_NAME, __VA_ARGS__)
        */
    }
}


#[cfg(OBOE_ENABLE_LOGGING)]
macro_rules! logd {
    ($($arg:ident),*) => {
        /*
                __android_log_print(ANDROID_LOG_DEBUG, MODULE_NAME, __VA_ARGS__)
        */
    }
}

///-----------------------------
#[cfg(not(OBOE_ENABLE_LOGGING))]
macro_rules! logv {
    ($($arg:ident),*) => { }
}

#[cfg(not(OBOE_ENABLE_LOGGING))]
macro_rules! logd {
    ($($arg:ident),*) => { }
}
