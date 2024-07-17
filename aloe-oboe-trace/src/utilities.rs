crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/Utilities.cpp]

pub const OBOE_SCALE_I16_TO_FLOAT: f32 = 1.0 / 32768.0;

pub fn oboe_convert_float_to_pcm16(
        source:      *const f32,
        destination: *mut i16,
        num_samples: i32)  {
    
    todo!();
    /*
        for (int i = 0; i < numSamples; i++) {
            float fval = source[i];
            fval += 1.0; // to avoid discontinuity at 0.0 caused by truncation
            fval *= 32768.0f;
            auto sample = static_cast<int32_t>(fval);
            // clip to 16-bit range
            if (sample < 0) sample = 0;
            else if (sample > 0x0FFFF) sample = 0x0FFFF;
            sample -= 32768; // center at zero
            destination[i] = static_cast<int16_t>(sample);
        }
    */
}

pub fn oboe_convert_pcm_16to_float(
        source:      *const i16,
        destination: *mut f32,
        num_samples: i32)  {
    
    todo!();
    /*
        for (int i = 0; i < numSamples; i++) {
            destination[i] = source[i] * kScaleI16ToFloat;
        }
    */
}

pub fn oboe_convert_format_to_size_in_bytes(format: OboeAudioFormat) -> i32 {
    
    todo!();
    /*
        int32_t size = 0;
        switch (format) {
            case AudioFormat::I16:
                size = sizeof(int16_t);
                break;
            case AudioFormat::Float:
                size = sizeof(float);
                break;
            case AudioFormat::I24:
                size = 3; // packed 24-bit data
                break;
            case AudioFormat::I32:
                size = sizeof(int32_t);
                break;
            default:
                break;
        }
        return size;
    */
}

pub fn oboe_convert_to_text_result(return_code: OboeResult) -> *const u8 {
    
    todo!();
    /*
        switch (returnCode) {
            case OboeResult::OK:                    return "OK";
            case OboeResult::ErrorDisconnected:     return "ErrorDisconnected";
            case OboeResult::ErrorIllegalArgument:  return "ErrorIllegalArgument";
            case OboeResult::ErrorInternal:         return "ErrorInternal";
            case OboeResult::ErrorInvalidState:     return "ErrorInvalidState";
            case OboeResult::ErrorInvalidHandle:    return "ErrorInvalidHandle";
            case OboeResult::ErrorUnimplemented:    return "ErrorUnimplemented";
            case OboeResult::ErrorUnavailable:      return "ErrorUnavailable";
            case OboeResult::ErrorNoFreeHandles:    return "ErrorNoFreeHandles";
            case OboeResult::ErrorNoMemory:         return "ErrorNoMemory";
            case OboeResult::ErrorNull:             return "ErrorNull";
            case OboeResult::ErrorTimeout:          return "ErrorTimeout";
            case OboeResult::ErrorWouldBlock:       return "ErrorWouldBlock";
            case OboeResult::ErrorInvalidFormat:    return "ErrorInvalidFormat";
            case OboeResult::ErrorOutOfRange:       return "ErrorOutOfRange";
            case OboeResult::ErrorNoService:        return "ErrorNoService";
            case OboeResult::ErrorInvalidRate:      return "ErrorInvalidRate";
            case OboeResult::ErrorClosed:           return "ErrorClosed";
            default:                            return "Unrecognized result";
        }
    */
}

pub fn oboe_convert_to_text_audio_format(format: OboeAudioFormat) -> *const u8 {
    
    todo!();
    /*
        switch (format) {
            case AudioFormat::Invalid:      return "Invalid";
            case AudioFormat::Unspecified:  return "Unspecified";
            case AudioFormat::I16:          return "I16";
            case AudioFormat::Float:        return "Float";
            case AudioFormat::I24:          return "I24";
            case AudioFormat::I32:          return "I32";
            default:                        return "Unrecognized format";
        }
    */
}

pub fn oboe_convert_to_text_performance_mode(mode: OboePerformanceMode) -> *const u8 {
    
    todo!();
    /*
        switch (mode) {
            case PerformanceMode::LowLatency:   return "LowLatency";
            case PerformanceMode::None:         return "None";
            case PerformanceMode::PowerSaving:  return "PowerSaving";
            default:                            return "Unrecognized performance mode";
        }
    */
}

pub fn oboe_convert_to_text_sharing_mode(mode: OboeSharingMode) -> *const u8 {
    
    todo!();
    /*
        switch (mode) {
            case SharingMode::Exclusive:    return "Exclusive";
            case SharingMode::Shared:       return "Shared";
            default:                        return "Unrecognized sharing mode";
        }
    */
}

pub fn oboe_convert_to_text_data_callback_result(result: OboeDataCallbackResult) -> *const u8 {
    
    todo!();
    /*
        switch (result) {
            case DataCallbackResult::Continue:  return "Continue";
            case DataCallbackResult::Stop:      return "Stop";
            default:                            return "Unrecognized data callback result";
        }
    */
}

pub fn oboe_convert_to_text_direction(direction: OboeDirection) -> *const u8 {
    
    todo!();
    /*
        switch (direction) {
            case Direction::Input:  return "Input";
            case Direction::Output: return "Output";
            default:                return "Unrecognized direction";
        }
    */
}

pub fn oboe_convert_to_text_stream_state(state: OboeStreamState) -> *const u8 {
    
    todo!();
    /*
        switch (state) {
            case StreamState::Closed:           return "Closed";
            case StreamState::Closing:          return "Closing";
            case StreamState::Disconnected:     return "Disconnected";
            case StreamState::Flushed:          return "Flushed";
            case StreamState::Flushing:         return "Flushing";
            case StreamState::Open:             return "Open";
            case StreamState::Paused:           return "Paused";
            case StreamState::Pausing:          return "Pausing";
            case StreamState::Started:          return "Started";
            case StreamState::Starting:         return "Starting";
            case StreamState::Stopped:          return "Stopped";
            case StreamState::Stopping:         return "Stopping";
            case StreamState::Uninitialized:    return "Uninitialized";
            case StreamState::Unknown:          return "Unknown";
            default:                            return "Unrecognized stream state";
        }
    */
}

pub fn oboe_convert_to_text_audio_api(audio_api: OboeAudioApi) -> *const u8 {
    
    todo!();
    /*
        switch (audioApi) {
            case AudioApi::Unspecified: return "Unspecified";
            case AudioApi::OpenSLES:    return "OpenSLES";
            case AudioApi::AAudio:      return "AAudio";
            default:                    return "Unrecognized audio API";
        }
    */
}

pub fn oboe_convert_to_text_audio_stream_ptr(stream: *mut AudioStream) -> *const u8 {
    
    todo!();
    /*
        static std::string streamText;
        std::stringstream s;

        s<<"StreamID: "<< static_cast<void*>(stream)<<std::endl
         <<"DeviceId: "<<stream->getDeviceId()<<std::endl
         <<"Direction: "<<OboeconvertToText(stream->getDirection())<<std::endl
         <<"API type: "<<OboeconvertToText(stream->getAudioApi())<<std::endl
         <<"BufferCapacity: "<<stream->getBufferCapacityInFrames()<<std::endl
         <<"BufferSize: "<<stream->getBufferSizeInFrames()<<std::endl
         <<"FramesPerBurst: "<< stream->getFramesPerBurst()<<std::endl
         <<"FramesPerDataCallback: "<<stream->getFramesPerDataCallback()<<std::endl
         <<"SampleRate: "<<stream->getSampleRate()<<std::endl
         <<"ChannelCount: "<<stream->getChannelCount()<<std::endl
         <<"Format: "<<OboeconvertToText(stream->getFormat())<<std::endl
         <<"SharingMode: "<<OboeconvertToText(stream->getSharingMode())<<std::endl
         <<"PerformanceMode: "<<OboeconvertToText(stream->getPerformanceMode())
         <<std::endl
         <<"CurrentState: "<<OboeconvertToText(stream->getState())<<std::endl
         <<"XRunCount: "<<stream->getXRunCount()<<std::endl
         <<"FramesRead: "<<stream->getFramesRead()<<std::endl
         <<"FramesWritten: "<<stream->getFramesWritten()<<std::endl;

        streamText = s.str();
        return streamText.c_str();
    */
}

pub fn oboe_convert_to_text_usage(usage: OboeUsage) -> *const u8 {
    
    todo!();
    /*
        switch (usage) {
            case Usage::Media:                         return "Media";
            case Usage::VoiceCommunication:            return "VoiceCommunication";
            case Usage::VoiceCommunicationSignalling:  return "VoiceCommunicationSignalling";
            case Usage::Alarm:                         return "Alarm";
            case Usage::Notification:                  return "Notification";
            case Usage::NotificationRingtone:          return "NotificationRingtone";
            case Usage::NotificationEvent:             return "NotificationEvent";
            case Usage::AssistanceAccessibility:       return "AssistanceAccessibility";
            case Usage::AssistanceNavigationGuidance:  return "AssistanceNavigationGuidance";
            case Usage::AssistanceSonification:        return "AssistanceSonification";
            case Usage::Game:                          return "Game";
            case Usage::Assistant:                     return "Assistant";
            default:                                   return "Unrecognized usage";
        }
    */
}

pub fn oboe_convert_to_text_content_type(content_type: OboeContentType) -> *const u8 {
    
    todo!();
    /*
        switch (contentType) {
            case ContentType::Speech:        return "Speech";
            case ContentType::Music:         return "Music";
            case ContentType::Movie:         return "Movie";
            case ContentType::Sonification:  return "Sonification";
            default:                         return "Unrecognized content type";
        }
    */
}

pub fn oboe_convert_to_text_input_preset(input_preset: OboeInputPreset) -> *const u8 {
    
    todo!();
    /*
        switch (inputPreset) {
            case InputPreset::Generic:             return "Generic";
            case InputPreset::Camcorder:           return "Camcorder";
            case InputPreset::VoiceRecognition:    return "VoiceRecognition";
            case InputPreset::VoiceCommunication:  return "VoiceCommunication";
            case InputPreset::Unprocessed:         return "Unprocessed";
            case InputPreset::VoicePerformance:    return "VoicePerformance";
            default:                               return "Unrecognized input preset";
        }
    */
}

pub fn oboe_convert_to_text_session_id(session_id: OboeSessionId) -> *const u8 {
    
    todo!();
    /*
        switch (sessionId) {
            case SessionId::None:      return "None";
            case SessionId::Allocate:  return "Allocate";
            default:                   return "Unrecognized session id";
        }
    */
}

pub fn oboe_convert_to_text_channel_count(channel_count: OboeChannelCount) -> *const u8 {
    
    todo!();
    /*
        switch (channelCount) {
            case ChannelCount::Unspecified:  return "Unspecified";
            case ChannelCount::Mono:         return "Mono";
            case ChannelCount::Stereo:       return "Stereo";
            default:                         return "Unrecognized channel count";
        }
    */
}

pub fn oboe_get_property_string(name: *const u8) -> String {
    
    todo!();
    /*
        std::string result;
    #ifdef __ANDROID__
        char valueText[PROP_VALUE_MAX] = {0};
        if (__system_property_get(name, valueText) != 0) {
            result = valueText;
        }
    #else
        (void) name;
    #endif
        return result;
    */
}

pub fn oboe_get_property_integer(
        name:          *const u8,
        default_value: i32) -> i32 {
    
    todo!();
    /*
        int result = defaultValue;
    #ifdef __ANDROID__
        char valueText[PROP_VALUE_MAX] = {0};
        if (__system_property_get(name, valueText) != 0) {
            result = atoi(valueText);
        }
    #else
        (void) name;
    #endif
        return result;
    */
}

pub fn oboe_get_sdk_version() -> i32 {
    
    todo!();
    /*
        static int sCachedSdkVersion = -1;
    #ifdef __ANDROID__
        if (sCachedSdkVersion == -1) {
            sCachedSdkVersion = getPropertyInteger("ro.build.version.sdk", -1);
        }
    #endif
        return sCachedSdkVersion;
    */
}
