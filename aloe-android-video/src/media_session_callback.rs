crate::ix!();

lazy_static!{
    /*
    VideoComponent::VideoComponentImpl::MediaSession::AndroidMediaSessionCallback_Class
    VideoComponent::VideoComponentImpl::MediaSession::AndroidMediaSessionCallback;
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
            CALLBACK (pauseCallback,           "mediaSessionPause",           "(J)V") 
            CALLBACK (playCallback,            "mediaSessionPlay",            "(J)V") 
            CALLBACK (playFromMediaIdCallback, "mediaSessionPlayFromMediaId", "(JLjava/lang/String;Landroid/os/Bundle;)V") 
            CALLBACK (seekToCallback,          "mediaSessionSeekTo",          "(JJ)V") 
            CALLBACK (stopCallback,            "mediaSessionStop",            "(J)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaSessionCallback, 
    "com/rmsl/aloe/MediaSessionCallback", 21
}
