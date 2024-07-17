crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICMETHOD (getMinBufferSize,            "getMinBufferSize",             "(III)I") 
         STATICMETHOD (getNativeOutputSampleRate,   "getNativeOutputSampleRate",    "(I)I") 
         METHOD (constructor,   "<init>",   "(IIIIII)V") 
         METHOD (getState,      "getState", "()I") 
         METHOD (play,          "play",     "()V") 
         METHOD (stop,          "stop",     "()V") 
         METHOD (release,       "release",  "()V") 
         METHOD (flush,         "flush",    "()V") 
         METHOD (write,         "write",    "([SII)I") 
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class!{
    AudioTrack, "android/media/AudioTrack"
}

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICMETHOD (getMinBufferSize, "getMinBufferSize", "(III)I") 
         METHOD (constructor,       "<init>",           "(IIIII)V") 
         METHOD (getState,          "getState",         "()I") 
         METHOD (startRecording,    "startRecording",   "()V") 
         METHOD (stop,              "stop",             "()V") 
         METHOD (read,              "read",             "([SII)I") 
         METHOD (release,           "release",          "()V") 
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class!{
    AudioRecord, "android/media/AudioRecord"
}
