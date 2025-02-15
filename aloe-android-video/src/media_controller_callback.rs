crate::ix!();

lazy_static!{
    /*
    VideoComponent::VideoComponentImpl::MediaSession::MediaSessionController::AndroidMediaControllerCallback_Class
    VideoComponent::VideoComponentImpl::MediaSession::MediaSessionController::AndroidMediaControllerCallback;
    */
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
                            METHOD (constructor, "<init>", "(J)V")                  
                            CALLBACK (audioInfoChanged,     "mediaControllerAudioInfoChanged",      "(JLandroid/media/session/MediaController$PlaybackInfo;)V") 
                            CALLBACK (metadataChanged,      "mediaControllerMetadataChanged",       "(JLandroid/media/MediaMetadata;)V") 
                            CALLBACK (playbackStateChanged, "mediaControllerPlaybackStateChanged",  "(JLandroid/media/session/PlaybackState;)V") 
                            CALLBACK (sessionDestroyed,     "mediaControllerSessionDestroyed",      "(J)V")
        */
    }
}

declare_jni_class_with_bytecode!{
    AndroidMediaControllerCallback, 
    "com/rmsl/aloe/MediaControllerCallback", 
    21, 
    MediaSessionByteCode, 
    sizeof (MediaSessionByteCode)
}
