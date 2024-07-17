crate::ix!();

#[cfg(not(__NDK_MAJOR__))]
pub const __NDK_MAJOR__: usize = 0;

#[cfg(not(__ANDROID_API_S__))]
pub const __ANDROID_API_S__: usize = 31;

pub const LIB_AAUDIO_NAME: &'static str = "libaaudio.so";

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/aaudio/OboeAAudioLoader.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/aaudio/OboeAAudioLoader.cpp]
/**
  | Ensure that all AAudio primitive data
  | types are int32_t
  |
  */
macro_rules! oboe_assert_int32 {
    ($type:ident) => {
        /*
                static_assert(std::is_same<int32_t, type>::value, 
            #type" must be int32_t")
        */
    }
}

pub const OBOE_ERRMSG: &'static str = "Oboe constants must match AAudio constants.";

/**
  | These asserts help verify that the Oboe
  | definitions match the equivalent AAudio
  | definitions.
  |
  | This code is in this .cpp file so it only
  | gets tested once.
  */
#[cfg(AAUDIO_AAUDIO_H)]
lazy_static!{
    /*
    ASSERT_INT32(aaudio_stream_state_t);
            ASSERT_INT32(aaudio_direction_t);
            ASSERT_INT32(aaudio_format_t);
            ASSERT_INT32(aaudio_data_callback_result_t);
            ASSERT_INT32(aaudio_result_t);
            ASSERT_INT32(aaudio_sharing_mode_t);
            ASSERT_INT32(aaudio_performance_mode_t);

            static_assert((int32_t)StreamState::Uninitialized == AAUDIO_STREAM_STATE_UNINITIALIZED, ERRMSG);
            static_assert((int32_t)StreamState::Unknown == AAUDIO_STREAM_STATE_UNKNOWN, ERRMSG);
            static_assert((int32_t)StreamState::Open == AAUDIO_STREAM_STATE_OPEN, ERRMSG);
            static_assert((int32_t)StreamState::Starting == AAUDIO_STREAM_STATE_STARTING, ERRMSG);
            static_assert((int32_t)StreamState::Started == AAUDIO_STREAM_STATE_STARTED, ERRMSG);
            static_assert((int32_t)StreamState::Pausing == AAUDIO_STREAM_STATE_PAUSING, ERRMSG);
            static_assert((int32_t)StreamState::Paused == AAUDIO_STREAM_STATE_PAUSED, ERRMSG);
            static_assert((int32_t)StreamState::Flushing == AAUDIO_STREAM_STATE_FLUSHING, ERRMSG);
            static_assert((int32_t)StreamState::Flushed == AAUDIO_STREAM_STATE_FLUSHED, ERRMSG);
            static_assert((int32_t)StreamState::Stopping == AAUDIO_STREAM_STATE_STOPPING, ERRMSG);
            static_assert((int32_t)StreamState::Stopped == AAUDIO_STREAM_STATE_STOPPED, ERRMSG);
            static_assert((int32_t)StreamState::Closing == AAUDIO_STREAM_STATE_CLOSING, ERRMSG);
            static_assert((int32_t)StreamState::Closed == AAUDIO_STREAM_STATE_CLOSED, ERRMSG);
            static_assert((int32_t)StreamState::Disconnected == AAUDIO_STREAM_STATE_DISCONNECTED, ERRMSG);

            static_assert((int32_t)Direction::Output == AAUDIO_DIRECTION_OUTPUT, ERRMSG);
            static_assert((int32_t)Direction::Input == AAUDIO_DIRECTION_INPUT, ERRMSG);

            static_assert((int32_t)AudioFormat::Invalid == AAUDIO_FORMAT_INVALID, ERRMSG);
            static_assert((int32_t)AudioFormat::Unspecified == AAUDIO_FORMAT_UNSPECIFIED, ERRMSG);
            static_assert((int32_t)AudioFormat::I16 == AAUDIO_FORMAT_PCM_I16, ERRMSG);
            static_assert((int32_t)AudioFormat::Float == AAUDIO_FORMAT_PCM_FLOAT, ERRMSG);

            static_assert((int32_t)DataCallbackResult::Continue == AAUDIO_CALLBACK_RESULT_CONTINUE, ERRMSG);
            static_assert((int32_t)DataCallbackResult::Stop == AAUDIO_CALLBACK_RESULT_STOP, ERRMSG);

            static_assert((int32_t)OboeResult::OK == AAUDIO_OK, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorBase == AAUDIO_ERROR_BASE, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorDisconnected == AAUDIO_ERROR_DISCONNECTED, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorIllegalArgument == AAUDIO_ERROR_ILLEGAL_ARGUMENT, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorInternal == AAUDIO_ERROR_INTERNAL, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorInvalidState == AAUDIO_ERROR_INVALID_STATE, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorInvalidHandle == AAUDIO_ERROR_INVALID_HANDLE, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorUnimplemented == AAUDIO_ERROR_UNIMPLEMENTED, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorUnavailable == AAUDIO_ERROR_UNAVAILABLE, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorNoFreeHandles == AAUDIO_ERROR_NO_FREE_HANDLES, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorNoMemory == AAUDIO_ERROR_NO_MEMORY, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorNull == AAUDIO_ERROR_NULL, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorTimeout == AAUDIO_ERROR_TIMEOUT, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorWouldBlock == AAUDIO_ERROR_WOULD_BLOCK, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorInvalidFormat == AAUDIO_ERROR_INVALID_FORMAT, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorOutOfRange == AAUDIO_ERROR_OUT_OF_RANGE, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorNoService == AAUDIO_ERROR_NO_SERVICE, ERRMSG);
            static_assert((int32_t)OboeResult::ErrorInvalidRate == AAUDIO_ERROR_INVALID_RATE, ERRMSG);

            static_assert((int32_t)SharingMode::Exclusive == AAUDIO_SHARING_MODE_EXCLUSIVE, ERRMSG);
            static_assert((int32_t)SharingMode::Shared == AAUDIO_SHARING_MODE_SHARED, ERRMSG);

            static_assert((int32_t)PerformanceMode::None == AAUDIO_PERFORMANCE_MODE_NONE, ERRMSG);
            static_assert((int32_t)PerformanceMode::PowerSaving
                    == AAUDIO_PERFORMANCE_MODE_POWER_SAVING, ERRMSG);
            static_assert((int32_t)PerformanceMode::LowLatency
                    == AAUDIO_PERFORMANCE_MODE_LOW_LATENCY, ERRMSG);
    */
}

/**
  | The aaudio_ usage, content and input_preset
  | types were added in NDK 17, which is the
  | first version to support Android Pie (API
  | 28).
  */
#[cfg(AAUDIO_AAUDIO_H)]
#[cfg(__NDK_MAJOR___GTE_17)]
lazy_static!{
    /*
    ASSERT_INT32(aaudio_usage_t);
            ASSERT_INT32(aaudio_content_type_t);
            ASSERT_INT32(aaudio_input_preset_t);

            static_assert((int32_t)Usage::Media == AAUDIO_USAGE_MEDIA, ERRMSG);
            static_assert((int32_t)Usage::VoiceCommunication == AAUDIO_USAGE_VOICE_COMMUNICATION, ERRMSG);
            static_assert((int32_t)Usage::VoiceCommunicationSignalling
                    == AAUDIO_USAGE_VOICE_COMMUNICATION_SIGNALLING, ERRMSG);
            static_assert((int32_t)Usage::Alarm == AAUDIO_USAGE_ALARM, ERRMSG);
            static_assert((int32_t)Usage::Notification == AAUDIO_USAGE_NOTIFICATION, ERRMSG);
            static_assert((int32_t)Usage::NotificationRingtone == AAUDIO_USAGE_NOTIFICATION_RINGTONE, ERRMSG);
            static_assert((int32_t)Usage::NotificationEvent == AAUDIO_USAGE_NOTIFICATION_EVENT, ERRMSG);
            static_assert((int32_t)Usage::AssistanceAccessibility == AAUDIO_USAGE_ASSISTANCE_ACCESSIBILITY, ERRMSG);
            static_assert((int32_t)Usage::AssistanceNavigationGuidance
                    == AAUDIO_USAGE_ASSISTANCE_NAVIGATION_GUIDANCE, ERRMSG);
            static_assert((int32_t)Usage::AssistanceSonification == AAUDIO_USAGE_ASSISTANCE_SONIFICATION, ERRMSG);
            static_assert((int32_t)Usage::Game == AAUDIO_USAGE_GAME, ERRMSG);
            static_assert((int32_t)Usage::Assistant == AAUDIO_USAGE_ASSISTANT, ERRMSG);

            static_assert((int32_t)ContentType::Speech == AAUDIO_CONTENT_TYPE_SPEECH, ERRMSG);
            static_assert((int32_t)ContentType::Music == AAUDIO_CONTENT_TYPE_MUSIC, ERRMSG);
            static_assert((int32_t)ContentType::Movie == AAUDIO_CONTENT_TYPE_MOVIE, ERRMSG);
            static_assert((int32_t)ContentType::Sonification == AAUDIO_CONTENT_TYPE_SONIFICATION, ERRMSG);

            static_assert((int32_t)InputPreset::Generic == AAUDIO_INPUT_PRESET_GENERIC, ERRMSG);
            static_assert((int32_t)InputPreset::Camcorder == AAUDIO_INPUT_PRESET_CAMCORDER, ERRMSG);
            static_assert((int32_t)InputPreset::VoiceRecognition == AAUDIO_INPUT_PRESET_VOICE_RECOGNITION, ERRMSG);
            static_assert((int32_t)InputPreset::VoiceCommunication
                    == AAUDIO_INPUT_PRESET_VOICE_COMMUNICATION, ERRMSG);
            static_assert((int32_t)InputPreset::Unprocessed == AAUDIO_INPUT_PRESET_UNPROCESSED, ERRMSG);

            static_assert((int32_t)SessionId::None == AAUDIO_SESSION_ID_NONE, ERRMSG);
            static_assert((int32_t)SessionId::Allocate == AAUDIO_SESSION_ID_ALLOCATE, ERRMSG);
    */
}

