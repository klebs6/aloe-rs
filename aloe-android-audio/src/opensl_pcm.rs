crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
        DECLARE_JNI_CLASS (AndroidAudioManager, "android/media/AudioManager")
        */
    }
}

#[cfg(target_os="android")]
#[cfg(not(SL_ANDROID_DATAFORMAT_PCM_EX))]
macro_rules! sl_android_dataformat_pcm_ex {
    () => {
        /*
                ((SLuint32) 0x00000004)
        */
    }
}

#[cfg(target_os="android")]
#[cfg(not(SL_ANDROID_PCM_REPRESENTATION_FLOAT))]
macro_rules! sl_android_pcm_representation_float {
    () => {
        /*
                ((SLuint32) 0x00000003)
        */
    }
}

#[cfg(target_os="android")]
#[cfg(not(SL_ANDROID_RECORDING_PRESET_UNPROCESSED))]
macro_rules! sl_android_recording_preset_unprocessed {
    () => {
        /*
                ((SLuint32) 0x00000005)
        */
    }
}
