crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/aloe_video.h]

/**
  | Config: ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
  | 
  | Enables synchronisation between video
  | playback volume and OS media volume.
  | 
  | Currently supported on Android only.
  |
  */
#[cfg(not(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME))]
pub const ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME: usize = 1;

#[cfg(not(ALOE_VIDEO_LOG_ENABLED))]
pub const ALOE_VIDEO_LOG_ENABLED: usize = 1;

#[cfg(ALOE_VIDEO_LOG_ENABLED)]
macro_rules! aloe_video_log {
    ($x:ident) => {
        /*
                DBG(x)
        */
    }
}

#[cfg(not(ALOE_VIDEO_LOG_ENABLED))]
macro_rules! aloe_video_log {
    ($x:ident) => {
        /*
                {}
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/aloe_video.cpp]

pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:   usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:    usize = 1;
pub const ALOE_CORE_INCLUDE_COM_SMART_PTR:  usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS: usize = 1;
