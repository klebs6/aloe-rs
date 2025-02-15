crate::ix!();

#[no_copy]
#[leak_detector]
pub struct VideoComponentImpl<'a> {
    base:                    AndroidViewComponent<'a>,
    base2:                   ActivityLifecycleCallbacks,
    base3:                   SurfaceHolderCallback,
    current_file:            File,
    currenturl:              Url,

    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] owner:                   &mut VideoComponent,
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] media_session:           MediaSession,
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] activity_life_listener:  GlobalRef,
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] system_volume_listener:  SystemVolumeListener,
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] surface_holder_callback: GlobalRef,
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] load_finished_callback:  fn(_0: &Url, _1: Result) -> (),
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)] was_open:                bool, // default = false
}

impl<'a> Drop for VideoComponentImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto* env = getEnv();

                if (surfaceHolderCallback != nullptr)
                {
                    jobject view = reinterpret_cast<jobject> (getView());

                    if (view != nullptr)
                    {
                        LocalRef <jobject> holder (env->CallObjectMethod (view, AndroidSurfaceView.getHolder));

                        env->CallVoidMethod (holder, AndroidSurfaceHolder.removeCallback, surfaceHolderCallback.get());
                        SurfaceHolderCallback::clear();
                        surfaceHolderCallback.clear();
                    }
                }

                if (activityLifeListener != nullptr)
                {
                    env->CallVoidMethod (getAppContext().get(), AndroidApplication.unregisterActivityLifecycleCallbacks, activityLifeListener.get());
                    ActivityLifecycleCallbacks::clear();
                    activityLifeListener.clear();
                }
        */
    }
}

impl<'a> VideoComponentImpl<'a> {

    pub fn new(
        owner_to_use: &mut VideoComponent,
        _1:           bool) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                mediaSession (*this)
                  #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
                  , systemVolumeListener (*this)
                      #endif

                          // Video requires SDK version 21 or higher
                          jassert (getAndroidSDKVersion() >= 21);

                          setVisible (true);

                          auto* env = getEnv();

                          LocalRef<jobject> appContext (getAppContext());

                          if (appContext != nullptr)
                          {
                              ActivityLifecycleCallbacks* callbacks = dynamic_cast<ActivityLifecycleCallbacks*> (this);

                              activityLifeListener = GlobalRef (CreateJavaInterface (callbacks, "android/app/Application$ActivityLifecycleCallbacks"));
                              env->CallVoidMethod (appContext.get(), AndroidApplication.registerActivityLifecycleCallbacks, activityLifeListener.get());
                          }

                          {
                              LocalRef<jobject> surfaceView (env->NewObject (AndroidSurfaceView, AndroidSurfaceView.constructor, getAppContext().get()));
                              LocalRef<jobject> holder (env->CallObjectMethod (surfaceView.get(), AndroidSurfaceView.getHolder));

                              SurfaceHolderCallback* callbacks = dynamic_cast<SurfaceHolderCallback*> (this);
                              surfaceHolderCallback = GlobalRef (CreateJavaInterface (callbacks, "android/view/SurfaceHolder$Callback"));
                              env->CallVoidMethod (holder, AndroidSurfaceHolder.addCallback, surfaceHolderCallback.get());

                              setView (surfaceView.get());
                          }
        */
    }
    
    pub fn load_async(
        &mut self, 
        url:      &Url,
        callback: fn(_0: &Url, _1: Result<(),()>) -> ()
    ) {
        
        todo!();
        /*
            close();
                wasOpen = false;

                if (url.isEmpty())
                {
                    jassertfalse;
                    return;
                }

                if (! url.isLocalFile())
                {
                    if (! isPermissionDeclaredInManifest ("android.permission.INTERNET"))
                    {
                        // In order to access videos from the Internet, the Internet permission has to be specified in
                        // Android Manifest.
                        jassertfalse;
                        return;
                    }
                }

                currentURL = url;

                jassert (callback != nullptr);

                loadFinishedCallback = std::move (callback);

                static constexpr jint visible = 0;
                getEnv()->CallVoidMethod ((jobject) getView(), AndroidView.setVisibility, visible);

                mediaSession.load (url);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (! isOpen())
                    return;

                mediaSession.closeVideo();

                static constexpr jint invisible = 4;
                getEnv()->CallVoidMethod ((jobject) getView(), AndroidView.setVisibility, invisible);
        */
    }
    
    pub fn is_open(&self) -> bool {
        
        todo!();
        /*
            return mediaSession.isVideoOpen();
        */
    }
    
    pub fn is_playing(&self) -> bool {
        
        todo!();
        /*
            return mediaSession.isPlaying();
        */
    }
    
    pub fn play(&mut self)  {
        
        todo!();
        /*
            mediaSession.play();
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            mediaSession.stop();
        */
    }
    
    pub fn set_position(&mut self, new_position: f64)  {
        
        todo!();
        /*
            mediaSession.setPosition (newPosition);
        */
    }
    
    pub fn get_position(&self) -> f64 {
        
        todo!();
        /*
            return mediaSession.getPosition();
        */
    }
    
    pub fn set_speed(&mut self, new_speed: f64)  {
        
        todo!();
        /*
            mediaSession.setSpeed (newSpeed);
        */
    }
    
    pub fn get_speed(&self) -> f64 {
        
        todo!();
        /*
            return mediaSession.getSpeed();
        */
    }
    
    pub fn get_native_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return mediaSession.getNativeSize();
        */
    }
    
    pub fn get_duration(&self) -> f64 {
        
        todo!();
        /*
            return mediaSession.getDuration();
        */
    }
    
    pub fn set_volume(&mut self, new_volume: f32)  {
        
        todo!();
        /*
            mediaSession.setVolume (newVolume);
        */
    }
    
    pub fn get_volume(&self) -> f32 {
        
        todo!();
        /*
            return mediaSession.getVolume();
        */
    }

    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn load_finished(&mut self)  {
        
        todo!();
        /*
            owner.resized();

                if (loadFinishedCallback != nullptr)
                {
                    loadFinishedCallback (currentURL, Result::ok());
                    loadFinishedCallback = nullptr;
                }
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn close_video_finished(&mut self)  {
        
        todo!();
        /*
            owner.resized();
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn error_occurred(&mut self, error_message: &String)  {
        
        todo!();
        /*
            if (owner.onErrorOccurred != nullptr)
                    owner.onErrorOccurred (errorMessage);
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn playback_started(&mut self)  {
        
        todo!();
        /*
            if (owner.onPlaybackStarted != nullptr)
                    owner.onPlaybackStarted();
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn playback_stopped(&mut self)  {
        
        todo!();
        /*
            if (owner.onPlaybackStopped != nullptr)
                    owner.onPlaybackStopped();
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn surface_changed(&mut self, 
        holder: LocalRef<jobject>,
        format: i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            mediaSession.setDisplay (holder);
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn surface_destroyed(&mut self, holder: LocalRef<jobject>)  {
        
        todo!();
        /*
            mediaSession.setDisplay (LocalRef<jobject>());
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn surface_created(&mut self, holder: LocalRef<jobject>)  {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn on_activity_paused(&mut self, _0: jobject)  {
        
        todo!();
        /*
            wasOpen = isOpen();

                if (! wasOpen)
                    return;

                ALOE_VIDEO_LOG ("App paused, releasing media player...");

                mediaSession.storeState();
                mediaSession.closeVideo();

                #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
                systemVolumeListener.setEnabled (false);
                #endif
        */
    }
    
    #[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
    pub fn on_activity_resumed(&mut self, _0: jobject)  {
        
        todo!();
        /*
            if (! wasOpen)
                    return;

                ALOE_VIDEO_LOG ("App resumed, restoring media player...");

                loadAsync (currentURL, [this] (const Url&, Result r)
                    {
                        if (r.wasOk())
                            mediaSession.restoreState();
                    });

                #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
                systemVolumeListener.setEnabled (true);
                #endif
        */
    }
}
