crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OpenSLESUtilities.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OpenSLESUtilities.cpp]

/**
  | Creates an extended PCM format from
  | the supplied format and data representation.
  | This method should only be called on
  | Android devices with API level 21+.
  | API 21 introduced the SLAndroidDataFormat_PCM_EX
  | object which allows audio samples to
  | be represented using single precision
  | floating-point.
  | 
  | -----------
  | @param format
  | 
  | @param representation
  | 
  | -----------
  | @return
  | 
  | the extended PCM format
  |
  */
/* ------------------ OSLES Helpers  ------------------ */

pub fn get_sl_err_str(code: SLresult) -> *const u8 {
    
    todo!();
    /*
        switch (code) {
            case SL_RESULT_SUCCESS:
                return "SL_RESULT_SUCCESS";
            case SL_RESULT_PRECONDITIONS_VIOLATED:
                return "SL_RESULT_PRECONDITIONS_VIOLATED";
            case SL_RESULT_PARAMETER_INVALID:
                return "SL_RESULT_PARAMETER_INVALID";
            case SL_RESULT_MEMORY_FAILURE:
                return "SL_RESULT_MEMORY_FAILURE";
            case SL_RESULT_RESOURCE_ERROR:
                return "SL_RESULT_RESOURCE_ERROR";
            case SL_RESULT_RESOURCE_LOST:
                return "SL_RESULT_RESOURCE_LOST";
            case SL_RESULT_IO_ERROR:
                return "SL_RESULT_IO_ERROR";
            case SL_RESULT_BUFFER_INSUFFICIENT:
                return "SL_RESULT_BUFFER_INSUFFICIENT";
            case SL_RESULT_CONTENT_CORRUPTED:
                return "SL_RESULT_CONTENT_CORRUPTED";
            case SL_RESULT_CONTENT_UNSUPPORTED:
                return "SL_RESULT_CONTENT_UNSUPPORTED";
            case SL_RESULT_CONTENT_NOT_FOUND:
                return "SL_RESULT_CONTENT_NOT_FOUND";
            case SL_RESULT_PERMISSION_DENIED:
                return "SL_RESULT_PERMISSION_DENIED";
            case SL_RESULT_FEATURE_UNSUPPORTED:
                return "SL_RESULT_FEATURE_UNSUPPORTED";
            case SL_RESULT_INTERNAL_ERROR:
                return "SL_RESULT_INTERNAL_ERROR";
            case SL_RESULT_UNKNOWN_ERROR:
                return "SL_RESULT_UNKNOWN_ERROR";
            case SL_RESULT_OPERATION_ABORTED:
                return "SL_RESULT_OPERATION_ABORTED";
            case SL_RESULT_CONTROL_LOST:
                return "SL_RESULT_CONTROL_LOST";
            default:
                return "Unknown SL error";
        }
    */
}

pub fn opensles_create_extended_format(
        format:         SLDataFormat_PCM,
        representation: u32) -> SLAndroidDataFormat_PCM_EX {
    
    todo!();
    /*
        SLAndroidDataFormat_PCM_EX format_pcm_ex;
        format_pcm_ex.formatType = SL_ANDROID_DATAFORMAT_PCM_EX;
        format_pcm_ex.numChannels = format.numChannels;
        format_pcm_ex.sampleRate = format.samplesPerSec;
        format_pcm_ex.bitsPerSample = format.bitsPerSample;
        format_pcm_ex.containerSize = format.containerSize;
        format_pcm_ex.channelMask = format.channelMask;
        format_pcm_ex.endianness = format.endianness;
        format_pcm_ex.representation = representation;
        return format_pcm_ex;
    */
}

pub fn opensles_convert_format_to_representation(format: AudioFormat) -> u32 {
    
    todo!();
    /*
        switch(format) {
            case AudioFormat::I16:
                return SL_ANDROID_PCM_REPRESENTATION_SIGNED_INT;
            case AudioFormat::Float:
                return SL_ANDROID_PCM_REPRESENTATION_FLOAT;
            case AudioFormat::I24:
            case AudioFormat::I32:
            case AudioFormat::Invalid:
            case AudioFormat::Unspecified:
            default:
                return 0;
        }
    */
}
