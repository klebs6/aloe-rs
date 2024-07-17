crate::ix!();

declare_jni_class_with_min_sdk!{
    AndroidPlaybackParams, 
    "android/media/PlaybackParams", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getActions,       "getActions",       "()J") 
         METHOD (getErrorMessage,  "getErrorMessage",  "()Ljava/lang/CharSequence;") 
         METHOD (getPlaybackSpeed, "getPlaybackSpeed", "()F") 
         METHOD (getPosition,      "getPosition",      "()J") 
         METHOD (getState,         "getState",         "()I")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidPlaybackState, 
    "android/media/session/PlaybackState", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (build,           "build",           "()Landroid/media/session/PlaybackState;") 
         METHOD (constructor,     "<init>",          "()V") 
         METHOD (setActions,      "setActions",      "(J)Landroid/media/session/PlaybackState$Builder;") 
         METHOD (setErrorMessage, "setErrorMessage", "(Ljava/lang/CharSequence;)Landroid/media/session/PlaybackState$Builder;") 
         METHOD (setState,        "setState",        "(IJF)Landroid/media/session/PlaybackState$Builder;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidPlaybackStateBuilder, 
    "android/media/session/PlaybackState$Builder", 21
}
