crate::ix!();

pub struct MediaSessionController<'a> {
    owner:                         &'a mut MediaSession<'a>,
    native_controller:             GlobalRef,
    controller_transport_controls: GlobalRef,
    controller_callback:           GlobalRef,
    was_playing:                   bool, // default = false
    was_paused:                    bool, // default = true
}

impl<'a> Drop for MediaSessionController<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto* env = getEnv();
                            env->CallVoidMethod (nativeController, AndroidMediaController.unregisterCallback, controllerCallback.get());
        */
    }
}

impl<'a> MediaSessionController<'a> {

    pub fn new(
        owner_to_use:             &mut MediaSession,
        native_controller_to_use: &LocalRef<jobject>) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                            nativeController (GlobalRef (nativeControllerToUse)),
                            controllerTransportControls (LocalRef<jobject> (getEnv()->CallObjectMethod (nativeControllerToUse,
                                        AndroidMediaController.getTransportControls))),
                                        controllerCallback (createControllerCallbacks())

                            auto* env = getEnv();

                            env->CallVoidMethod (nativeController, AndroidMediaController.registerCallback, controllerCallback.get());
        */
    }
    
    pub fn load(&mut self, url: &Url)  {
        
        todo!();
        /*
            // NB: would use playFromUri, but it was only introduced in API 23...
                            getEnv()->CallVoidMethod (controllerTransportControls, AndroidMediaControllerTransportControls.playFromMediaId,
                                javaString (url.toString (true)).get(), nullptr);
        */
    }
    
    pub fn close_video(&mut self)  {
        
        todo!();
        /*
            getEnv()->CallVoidMethod (controllerTransportControls, AndroidMediaControllerTransportControls.stop);
        */
    }
    
    pub fn play(&mut self)  {
        
        todo!();
        /*
            getEnv()->CallVoidMethod (controllerTransportControls, AndroidMediaControllerTransportControls.play);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            // NB: calling pause, rather than stop, because after calling stop, we would have to call load() again.
                            getEnv()->CallVoidMethod (controllerTransportControls, AndroidMediaControllerTransportControls.pause);
        */
    }
    
    pub fn set_position(&mut self, new_position: f64)  {
        
        todo!();
        /*
            auto seekPos = static_cast<jlong> (newPosition * 1000);

                            getEnv()->CallVoidMethod (controllerTransportControls, AndroidMediaControllerTransportControls.seekTo, seekPos);
        */
    }
    
    pub fn get_position(&self) -> f64 {
        
        todo!();
        /*
            auto* env = getEnv();

                            auto playbackState = LocalRef<jobject> (env->CallObjectMethod (nativeController, AndroidMediaController.getPlaybackState));

                            if (playbackState != nullptr)
                                return (double) env->CallLongMethod (playbackState, AndroidPlaybackState.getPosition) / 1000.0;

                            return 0.0;
        */
    }
    
    pub fn get_play_speed(&self) -> f64 {
        
        todo!();
        /*
            auto* env = getEnv();

                            auto playbackState = LocalRef<jobject> (env->CallObjectMethod (nativeController, AndroidMediaController.getPlaybackState));

                            if (playbackState != nullptr)
                                return (double) env->CallFloatMethod (playbackState, AndroidPlaybackState.getPlaybackSpeed);

                            return 1.0;
        */
    }
    
    pub fn set_volume(&mut self, new_volume: f32)  {
        
        todo!();
        /*
            auto* env = getEnv();

                            auto playbackInfo = LocalRef<jobject> (env->CallObjectMethod (nativeController, AndroidMediaController.getPlaybackInfo));

                            auto maxVolume = env->CallIntMethod (playbackInfo, AndroidMediaControllerPlaybackInfo.getMaxVolume);

                            auto targetVolume = jmin (jint ((float) maxVolume * newVolume), maxVolume);

                            static constexpr jint flagShowUI = 1;
                            env->CallVoidMethod (nativeController, AndroidMediaController.setVolumeTo, targetVolume, flagShowUI);
        */
    }
    
    pub fn get_volume(&self) -> f32 {
        
        todo!();
        /*
            auto* env = getEnv();

                            auto playbackInfo = LocalRef<jobject> (env->CallObjectMethod (nativeController, AndroidMediaController.getPlaybackInfo));

                            auto maxVolume = (int) (env->CallIntMethod (playbackInfo, AndroidMediaControllerPlaybackInfo.getMaxVolume));
                            auto curVolume = (int) (env->CallIntMethod (playbackInfo, AndroidMediaControllerPlaybackInfo.getCurrentVolume));

                            return static_cast<float> (curVolume) / (float) maxVolume;
        */
    }
    
    pub fn state_changed(&mut self, playback_state: jobject)  {
        
        todo!();
        /*
            ALOE_VIDEO_LOG ("MediaSessionController::playbackStateChanged()");

                            if (playbackState == nullptr)
                                return;

                            auto state = getEnv()->CallIntMethod (playbackState, AndroidPlaybackState.getState);

                            static constexpr jint statePaused  = 2;
                            static constexpr jint statePlaying = 3;

                            if (wasPlaying == false && state == statePlaying)
                                owner.playbackStarted();
                            else if (wasPaused == false && state == statePaused)
                                owner.playbackStopped();

                            wasPlaying = state == statePlaying;
                            wasPaused  = state == statePaused;
        */
    }
    
    pub fn create_controller_callbacks(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->NewObject (AndroidMediaControllerCallback,
                                        AndroidMediaControllerCallback.constructor,
                                        reinterpret_cast<jlong> (this)));
        */
    }

    /**
       MediaSessionController callbacks
      */
    pub fn audio_info_changed(
        _0:            *mut JNIEnv,
        _1:            jobject,
        host:          i64,
        playback_info: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentImpl::MediaSession::MediaSessionController*> (host))
                            {
                                ignoreUnused (playbackInfo);
                                ALOE_VIDEO_LOG ("MediaSessionController::audioInfoChanged()");
                            }
        */
    }
    
    pub fn metadata_changed(
        _0:       *mut JNIEnv,
        _1:       jobject,
        host:     i64,
        metadata: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentImpl::MediaSession::MediaSessionController*> (host))
                            {
                                ignoreUnused (metadata);
                                ALOE_VIDEO_LOG ("MediaSessionController::metadataChanged()");
                            }
        */
    }
    
    pub fn playback_state_changed(
        _0:    *mut JNIEnv,
        _1:    jobject,
        host:  i64,
        state: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentImpl::MediaSession::MediaSessionController*> (host))
                                myself->stateChanged (state);
        */
    }
    
    pub fn session_destroyed(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentImpl::MediaSession::MediaSessionController*> (host))
                                ALOE_VIDEO_LOG ("MediaSessionController::sessionDestroyed()");
        */
    }
}
