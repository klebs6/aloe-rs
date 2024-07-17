crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getPlaybackInfo,      "getPlaybackInfo",      "()Landroid/media/session/MediaController$PlaybackInfo;") 
         METHOD (getPlaybackState,     "getPlaybackState",     "()Landroid/media/session/PlaybackState;") 
         METHOD (getTransportControls, "getTransportControls", "()Landroid/media/session/MediaController$TransportControls;") 
         METHOD (registerCallback,     "registerCallback",     "(Landroid/media/session/MediaController$Callback;)V") 
         METHOD (setVolumeTo,          "setVolumeTo",          "(II)V") 
         METHOD (unregisterCallback,   "unregisterCallback",   "(Landroid/media/session/MediaController$Callback;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaController, 
    "android/media/session/MediaController", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getAudioAttributes, "getAudioAttributes", "()Landroid/media/AudioAttributes;") 
         METHOD (getCurrentVolume,   "getCurrentVolume",   "()I") 
         METHOD (getMaxVolume,       "getMaxVolume",       "()I")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaControllerPlaybackInfo, 
    "android/media/session/MediaController$PlaybackInfo", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (pause,           "pause",           "()V") 
         METHOD (play,            "play",            "()V") 
         METHOD (playFromMediaId, "playFromMediaId", "(Ljava/lang/String;Landroid/os/Bundle;)V") 
         METHOD (seekTo,          "seekTo",          "(J)V") 
         METHOD (stop,            "stop",            "()V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaControllerTransportControls, 
    "android/media/session/MediaController$TransportControls", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor,                  "<init>",                       "()V") 
         METHOD (getCurrentPosition,           "getCurrentPosition",           "()I") 
         METHOD (getDuration,                  "getDuration",                  "()I") 
         METHOD (getPlaybackParams,            "getPlaybackParams",            "()Landroid/media/PlaybackParams;") 
         METHOD (getVideoHeight,               "getVideoHeight",               "()I") 
         METHOD (getVideoWidth,                "getVideoWidth",                "()I") 
         METHOD (isPlaying,                    "isPlaying",                    "()Z") 
         METHOD (pause,                        "pause",                        "()V") 
         METHOD (prepareAsync,                 "prepareAsync",                 "()V") 
         METHOD (release,                      "release",                      "()V") 
         METHOD (seekTo,                       "seekTo",                       "(I)V") 
         METHOD (setAudioAttributes,           "setAudioAttributes",           "(Landroid/media/AudioAttributes;)V") 
         METHOD (setDataSource,                "setDataSource",                "(Landroid/content/Context;Landroid/net/Uri;)V") 
         METHOD (setDisplay,                   "setDisplay",                   "(Landroid/view/SurfaceHolder;)V") 
         METHOD (setOnBufferingUpdateListener, "setOnBufferingUpdateListener", "(Landroid/media/MediaPlayer$OnBufferingUpdateListener;)V") 
         METHOD (setOnCompletionListener,      "setOnCompletionListener",      "(Landroid/media/MediaPlayer$OnCompletionListener;)V") 
         METHOD (setOnErrorListener,           "setOnErrorListener",           "(Landroid/media/MediaPlayer$OnErrorListener;)V") 
         METHOD (setOnInfoListener,            "setOnInfoListener",            "(Landroid/media/MediaPlayer$OnInfoListener;)V") 
         METHOD (setOnPreparedListener,        "setOnPreparedListener",        "(Landroid/media/MediaPlayer$OnPreparedListener;)V") 
         METHOD (setOnSeekCompleteListener,    "setOnSeekCompleteListener",    "(Landroid/media/MediaPlayer$OnSeekCompleteListener;)V") 
         METHOD (setPlaybackParams,            "setPlaybackParams",            "(Landroid/media/PlaybackParams;)V") 
         METHOD (setVolume,                    "setVolume",                    "(FF)V") 
         METHOD (start,                        "start",                        "()V") 
         METHOD (stop,                         "stop",                         "()V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaPlayer, 
    "android/media/MediaPlayer", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor,            "<init>",                 "(Landroid/content/Context;Ljava/lang/String;)V") 
         METHOD (getController,          "getController",          "()Landroid/media/session/MediaController;") 
         METHOD (release,                "release",                "()V") 
         METHOD (setActive,              "setActive",              "(Z)V") 
         METHOD (setCallback,            "setCallback",            "(Landroid/media/session/MediaSession$Callback;)V") 
         METHOD (setFlags,               "setFlags",               "(I)V") 
         METHOD (setMediaButtonReceiver, "setMediaButtonReceiver", "(Landroid/app/PendingIntent;)V") 
         METHOD (setMetadata,            "setMetadata",            "(Landroid/media/MediaMetadata;)V") 
         METHOD (setPlaybackState,       "setPlaybackState",       "(Landroid/media/session/PlaybackState;)V") 
         METHOD (setPlaybackToLocal,     "setPlaybackToLocal",     "(Landroid/media/AudioAttributes;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaSession, 
    "android/media/session/MediaSession", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (build,       "build",   "()Landroid/media/MediaMetadata;") 
         METHOD (constructor, "<init>",  "()V") 
         METHOD (putLong,     "putLong", "(Ljava/lang/String;J)Landroid/media/MediaMetadata$Builder;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaMetadataBuilder, 
    "android/media/MediaMetadata$Builder", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getSpeed, "getSpeed", "()F") 
         METHOD (setSpeed, "setSpeed", "(F)Landroid/media/PlaybackParams;")
        */
    }
}

