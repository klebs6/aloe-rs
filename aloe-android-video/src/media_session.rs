crate::ix!();

pub struct MediaSession<'a> {
    owner:                              &'a mut VideoComponentPimpl<'a>,
    sdk_version:                        i32,
    audio_attributes:                   GlobalRef,
    native_media_session:               GlobalRef,
    media_session_callback:             GlobalRef,
    playback_state_builder:             GlobalRef,
    controller:                         MediaSessionController<'a>,
    player:                             MediaSessionPlayer<'a>,
    audio_manager:                      GlobalRef,
    audio_focus_change_listener:        AudioManagerOnAudioFocusChangeListener<'a>,
    native_audio_focus_change_listener: GlobalRef,
    audio_focus_request:                GlobalRef,
    stored_playback_state:              GlobalRef,
    pending_seek_request:               bool, // default = false
    player_buffering_in_progress:       bool, // default = false
    uses_buffering:                     bool, // default = false
    buffered_regions:                   SparseSet<i32>,
    play_speed_mult:                    f64, // default = 1.0
    has_audio_focus:                    bool, // default = false
}

impl<'a> AudioManagerOnAudioFocusChangeListenerOwner for MediaSession<'a> {

    fn on_audio_focus_change(&mut self, change_type: i32)  {
        
        todo!();
        /*
            static constexpr jint audioFocusGain = 1;

                                if (changeType == audioFocusGain)
                                    ALOE_VIDEO_LOG ("Audio focus gained");
                                else
                                    ALOE_VIDEO_LOG ("Audio focus lost");

                                if (changeType != audioFocusGain)
                                {
                                    if (isPlaying())
                                    {
                                        ALOE_VIDEO_LOG ("Received a request to abandon audio focus. Stopping playback...");
                                        stop();
                                    }

                                    abandonAudioFocus();
                                }
        */
    }
}

impl<'a> Drop for MediaSession<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto* env = getEnv();

                    env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setCallback, nullptr);

                    controller.stop();
                    env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.release);
        */
    }
}

impl<'a> MediaSession<'a> {

    pub fn new(owner_to_use: &mut VideoComponentPimpl) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                    sdkVersion (getAndroidSDKVersion()),
                    audioAttributes (getAudioAttributes()),
                    nativeMediaSession (LocalRef<jobject> (getEnv()->NewObject (AndroidMediaSession,
                                AndroidMediaSession.constructor,
                                getAppContext().get(),
                                javaString ("AloeVideoMediaSession").get()))),
                                mediaSessionCallback (createCallbackObject()),
                                playbackStateBuilder (LocalRef<jobject> (getEnv()->NewObject (AndroidPlaybackStateBuilder,
                                            AndroidPlaybackStateBuilder.constructor))),
                                            controller (*this, LocalRef<jobject> (getEnv()->CallObjectMethod (nativeMediaSession,
                                                        AndroidMediaSession.getController))),
                                                        player (*this),
                                                        audioManager (LocalRef<jobject> (getEnv()->CallObjectMethod (getAppContext().get(), AndroidContext.getSystemService,
                                                        javaString ("audio").get()))),
                                                        audioFocusChangeListener (*this),
                                                        nativeAudioFocusChangeListener (GlobalRef (CreateJavaInterface (&audioFocusChangeListener,
                                                                    "android/media/AudioManager$OnAudioFocusChangeListener"))),
                                                                    audioFocusRequest (createAudioFocusRequestIfNecessary (sdkVersion, audioAttributes,
                                                                            nativeAudioFocusChangeListener))
                    auto* env = getEnv();

                    env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setPlaybackToLocal, audioAttributes.get());
                    env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setMediaButtonReceiver, nullptr);
                    env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setCallback, mediaSessionCallback.get());
        */
    }
    
    pub fn is_video_open(&self) -> bool {
        
        todo!();
        /*
            return player.isVideoOpen();
        */
    }
    
    pub fn is_playing(&self) -> bool {
        
        todo!();
        /*
            return player.isPlaying();
        */
    }
    
    pub fn load(&mut self, url: &Url)  {
        
        todo!();
        /*
            controller.load (url);
        */
    }
    
    pub fn close_video(&mut self)  {
        
        todo!();
        /*
            resetState();
                    controller.closeVideo();
        */
    }
    
    pub fn set_display(&mut self, surface_holder: &LocalRef<jobject>)  {
        
        todo!();
        /*
            player.setDisplay (surfaceHolder);
        */
    }
    
    pub fn play(&mut self)  {
        
        todo!();
        /*
            controller.play();
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            controller.stop();
        */
    }
    
    pub fn set_position(&mut self, new_position: f64)  {
        
        todo!();
        /*
            controller.setPosition (newPosition);
        */
    }
    
    pub fn get_position(&self) -> f64 {
        
        todo!();
        /*
            return controller.getPosition();
        */
    }
    
    pub fn set_speed(&mut self, new_speed: f64)  {
        
        todo!();
        /*
            playSpeedMult = newSpeed;

                    // Calling non 0.0 speed on a paused player would start it...
                    if (player.isPlaying())
                    {
                        player.setPlaySpeed (playSpeedMult);
                        updatePlaybackState();
                    }
        */
    }
    
    pub fn get_speed(&self) -> f64 {
        
        todo!();
        /*
            return controller.getPlaySpeed();
        */
    }
    
    pub fn get_native_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return player.getVideoNativeSize();
        */
    }
    
    pub fn get_duration(&self) -> f64 {
        
        todo!();
        /*
            return (double) player.getVideoDuration() / 1000.0;
        */
    }
    
    pub fn set_volume(&mut self, new_volume: f32)  {
        
        todo!();
        /*
            #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
                    controller.setVolume (newVolume);
                    #else
                    player.setAudioVolume (newVolume);
                    #endif
        */
    }
    
    pub fn get_volume(&self) -> f32 {
        
        todo!();
        /*
            #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
                    return controller.getVolume();
                    #else
                    return player.getAudioVolume();
                    #endif
        */
    }
    
    pub fn store_state(&mut self)  {
        
        todo!();
        /*
            storedPlaybackState.clear();
                    storedPlaybackState = GlobalRef (getCurrentPlaybackState());
        */
    }
    
    pub fn restore_state(&mut self)  {
        
        todo!();
        /*
            if (storedPlaybackState.get() == nullptr)
                        return;

                    auto* env = getEnv();

                    auto pos = env->CallLongMethod (storedPlaybackState, AndroidPlaybackState.getPosition);
                    setPosition ((double) pos / 1000.0);

                    setSpeed (playSpeedMult);

                    auto state = env->CallIntMethod (storedPlaybackState, AndroidPlaybackState.getState);

                    if (state != PlaybackState::STATE_NONE && state != PlaybackState::STATE_STOPPED
                        && state != PlaybackState::STATE_PAUSED && state != PlaybackState::STATE_ERROR)
                    {
                        play();
                    }
        */
    }
    
    pub fn create_callback_object(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->NewObject (AndroidMediaSessionCallback,
                                            AndroidMediaSessionCallback.constructor,
                                            reinterpret_cast<jlong> (this)));
        */
    }

    /**
       MediaSession callbacks
      */
    pub fn pause_callback(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentPimpl::MediaSession*> (host))
                                {
                                    ALOE_VIDEO_LOG ("MediaSession::pauseCallback()");
                                    myself->player.pause();
                                    myself->updatePlaybackState();

                                    myself->abandonAudioFocus();
                                }
        */
    }
    
    pub fn play_callback(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentPimpl::MediaSession*> (host))
                                {
                                    ALOE_VIDEO_LOG ("MediaSession::playCallback()");

                                    myself->requestAudioFocus();

                                    if (! myself->hasAudioFocus)
                                    {
                                        myself->errorOccurred ("Application has been denied audio focus. Try again later.");
                                        return;
                                    }

                                    getEnv()->CallVoidMethod (myself->nativeMediaSession, AndroidMediaSession.setActive, true);

                                    myself->player.play();
                                    myself->setSpeed (myself->playSpeedMult);
                                    myself->updatePlaybackState();
                                }
        */
    }
    
    pub fn play_from_media_id_callback(
        env:      *mut JNIEnv,
        _1:       jobject,
        host:     i64,
        media_id: jstring,
        extras:   jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentPimpl::MediaSession*> (host))
                                {
                                    ALOE_VIDEO_LOG ("MediaSession::playFromMediaIdCallback()");

                                    myself->player.load (LocalRef<jstring> ((jstring) env->NewLocalRef(mediaId)), LocalRef<jobject> (env->NewLocalRef(extras)));
                                    myself->updatePlaybackState();
                                }
        */
    }
    
    pub fn seek_to_callback(
        env:  *mut JNIEnv,
        _1:   jobject,
        host: i64,
        pos:  i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentPimpl::MediaSession*> (host))
                                {
                                    ALOE_VIDEO_LOG ("MediaSession::seekToCallback()");

                                    myself->pendingSeekRequest = true;
                                    myself->player.setPlayPosition ((jint) pos);
                                    myself->updatePlaybackState();
                                }
        */
    }
    
    pub fn stop_callback(
        env:  *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentPimpl::MediaSession*> (host))
                                {
                                    ALOE_VIDEO_LOG ("MediaSession::stopCallback()");

                                    env->CallVoidMethod (myself->nativeMediaSession, AndroidMediaSession.setActive, false);

                                    myself->player.closeVideo();
                                    myself->updatePlaybackState();

                                    myself->abandonAudioFocus();

                            myself->owner.closeVideoFinished();
                                }
        */
    }
    
    pub fn is_seek_in_progress(&self) -> bool {
        
        todo!();
        /*
            if (pendingSeekRequest)
                                    return true;

                                if (! usesBuffering)
                                    return false;

                                // NB: player sometimes notifies us about buffering, but only for regions that
                                // were previously buffered already. For buffering happening for the first time,
                                // we don't get such notification...
                                if (playerBufferingInProgress)
                                    return true;

                                auto playPos = player.getPlayPosition();
                                auto durationMs = player.getVideoDuration();
                                auto playPosPercent = (int) (100.0 * playPos / static_cast<double> (durationMs));

                                // NB: assuming the playback will start roughly when there is 5% of content loaded...
                                return ! bufferedRegions.containsRange (Range<int> (playPosPercent, jmin (101, playPosPercent + 5)));
        */
    }
    
    pub fn update_playback_state(&mut self)  {
        
        todo!();
        /*
            getEnv()->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setPlaybackState, getCurrentPlaybackState().get());
        */
    }
    
    pub fn get_current_playback_state(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            static constexpr int bufferingState = 6;

                                auto playbackStateFlag = isSeekInProgress() ? bufferingState : player.getPlaybackStateFlag();
                                auto playPos = player.getPlayPosition();
                                auto playSpeed = player.getPlaySpeed();
                                auto allowedActions = player.getAllowedActions();

                                auto* env = getEnv();

                            LocalRef<jobject> (env->CallObjectMethod (playbackStateBuilder, AndroidPlaybackStateBuilder.setState,
                                    (jint) playbackStateFlag, (jlong) playPos, (jfloat) playSpeed));

                            LocalRef<jobject> (env->CallObjectMethod (playbackStateBuilder, AndroidPlaybackStateBuilder.setActions, (jint) allowedActions));

                            return LocalRef<jobject> (env->CallObjectMethod (playbackStateBuilder, AndroidPlaybackStateBuilder.build));
        */
    }
    
    pub fn player_prepared(&mut self)  {
        
        todo!();
        /*
            resetState();

                                updateMetadata();

                                owner.loadFinished();
        */
    }
    
    pub fn player_buffering_started(&mut self)  {
        
        todo!();
        /*
            playerBufferingInProgress = true;
        */
    }
    
    pub fn player_buffering_ended(&mut self)  {
        
        todo!();
        /*
            playerBufferingInProgress = false;
        */
    }
    
    pub fn player_buffering_updated(&mut self, progress: i32)  {
        
        todo!();
        /*
            usesBuffering = true;

                                updatePlaybackState();

                                auto playPos = player.getPlayPosition();
                                auto durationMs = player.getVideoDuration();
                                auto playPosPercent = (int) (100.0 * playPos / static_cast<double> (durationMs));

                                bufferedRegions.addRange (Range<int> (playPosPercent, progress + 1));

                                String ranges;

                                for (auto& r : bufferedRegions.getRanges())
                                    ranges << "[" << r.getStart() << "%, " << r.getEnd() - 1 << "%] ";

                                ALOE_VIDEO_LOG ("Buffering status update, seek pos: " + String (playPosPercent) + "%, buffered regions: " + ranges);
        */
    }
    
    pub fn player_seek_completed(&mut self)  {
        
        todo!();
        /*
            pendingSeekRequest = false;
                                updatePlaybackState();
        */
    }
    
    pub fn player_playback_completed(&mut self)  {
        
        todo!();
        /*
            player.pause();
                                abandonAudioFocus();

                                pendingSeekRequest = true;
                                player.setPlayPosition (0);
                                updatePlaybackState();
        */
    }
    
    pub fn update_metadata(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

                                auto metadataBuilder = LocalRef<jobject> (env->NewObject (AndroidMediaMetadataBuilder,
                                        AndroidMediaMetadataBuilder.constructor));

                                auto durationMs = player.getVideoDuration();

                                auto jDurationKey = javaString ("android.media.metadata.DURATION");
                                LocalRef<jobject> (env->CallObjectMethod (metadataBuilder,
                                        AndroidMediaMetadataBuilder.putLong,
                                        jDurationKey.get(),
                                        (jlong) durationMs));

                                auto jNumTracksKey = javaString ("android.media.metadata.NUM_TRACKS");
                                LocalRef<jobject> (env->CallObjectMethod (metadataBuilder,
                                        AndroidMediaMetadataBuilder.putLong,
                                        jNumTracksKey.get(),
                                        (jlong) 1));

                                env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setMetadata,
                                    env->CallObjectMethod (metadataBuilder, AndroidMediaMetadataBuilder.build));
        */
    }
    
    pub fn error_occurred(&mut self, error_message: &String)  {
        
        todo!();
        /*
            auto* env = getEnv();

                                // Propagate error to session controller(s) and ...
                                LocalRef<jobject> (env->CallObjectMethod (playbackStateBuilder, AndroidPlaybackStateBuilder.setErrorMessage,
                                        javaString (errorMessage).get()));

                                auto state = LocalRef<jobject> (env->CallObjectMethod (playbackStateBuilder, AndroidPlaybackStateBuilder.build));
                                env->CallVoidMethod (nativeMediaSession, AndroidMediaSession.setPlaybackState, state.get());

                        // ...also notify Aloe side client
                                owner.errorOccurred (errorMessage);
        */
    }
    
    pub fn create_audio_focus_request_if_necessary(
        sdk_version:                        i32,
        audio_attributes:                   &GlobalRef,
        native_audio_focus_change_listener: &GlobalRef) -> LocalRef<jobject> {
        
        todo!();
        /*
            if (sdkVersion < 26)
                                    return LocalRef<jobject>();

                                auto* env = getEnv();

                                auto requestBuilderClass = LocalRef<jclass> (env->FindClass ("android/media/AudioFocusRequest$Builder"));

                                static jmethodID constructor = env->GetMethodID (requestBuilderClass, "<init>", "(I)V");
                                static jmethodID buildMethod = env->GetMethodID (requestBuilderClass, "build", "()Landroid/media/AudioFocusRequest;");
                                static jmethodID setAudioAttributesMethod = env->GetMethodID (requestBuilderClass, "setAudioAttributes",
                                    "(Landroid/media/AudioAttributes;)Landroid/media/AudioFocusRequest$Builder;");
                                static jmethodID setOnAudioFocusChangeListenerMethod = env->GetMethodID (requestBuilderClass, "setOnAudioFocusChangeListener",
                                    "(Landroid/media/AudioManager$OnAudioFocusChangeListener;)Landroid/media/AudioFocusRequest$Builder;");

                                static constexpr jint audioFocusGain = 1;

                                auto requestBuilder = LocalRef<jobject> (env->NewObject (requestBuilderClass, constructor, audioFocusGain));
                                LocalRef<jobject> (env->CallObjectMethod (requestBuilder, setAudioAttributesMethod, audioAttributes.get()));
                                LocalRef<jobject> (env->CallObjectMethod (requestBuilder, setOnAudioFocusChangeListenerMethod, nativeAudioFocusChangeListener.get()));

                                return LocalRef<jobject> (env->CallObjectMethod (requestBuilder, buildMethod));
        */
    }
    
    pub fn request_audio_focus(&mut self)  {
        
        todo!();
        /*
            static constexpr jint audioFocusGain = 1;
                                static constexpr jint streamMusic = 3;
                                static constexpr jint audioFocusRequestGranted = 1;

                                jint result = audioFocusRequestGranted;

                                if (sdkVersion >= 26)
                                {
                                    static jmethodID requestAudioFocusMethod = getEnv()->GetMethodID (AndroidAudioManager, "requestAudioFocus",
                                        "(Landroid/media/AudioFocusRequest;)I");

                                    result = getEnv()->CallIntMethod (audioManager, requestAudioFocusMethod, audioFocusRequest.get());
                                }
                                else
                                {
                                    result = getEnv()->CallIntMethod (audioManager, AndroidAudioManager.requestAudioFocus,
                                        nativeAudioFocusChangeListener.get(), streamMusic, audioFocusGain);
                                }

                                hasAudioFocus = result == audioFocusRequestGranted;
        */
    }
    
    pub fn abandon_audio_focus(&mut self)  {
        
        todo!();
        /*
            if (! hasAudioFocus)
                                    return;

                                static constexpr jint audioFocusRequestGranted = 1;

                                jint result = audioFocusRequestGranted;

                                if (sdkVersion >= 26)
                                {
                                    static jmethodID abandonAudioFocusMethod = getEnv()->GetMethodID (AndroidAudioManager, "abandonAudioFocusRequest",
                                        "(Landroid/media/AudioFocusRequest;)I");

                                    result = getEnv()->CallIntMethod (audioManager, abandonAudioFocusMethod, audioFocusRequest.get());
                                }
                                else
                                {
                                    result = getEnv()->CallIntMethod (audioManager, AndroidAudioManager.abandonAudioFocus,
                                        nativeAudioFocusChangeListener.get());
                                }

                                // NB: granted in this case means "granted to change the focus to abandoned"...
                                hasAudioFocus = result != audioFocusRequestGranted;
        */
    }
    
    pub fn playback_started(&mut self)  {
        
        todo!();
        /*
            owner.playbackStarted();
        */
    }
    
    pub fn playback_stopped(&mut self)  {
        
        todo!();
        /*
            owner.playbackStopped();
        */
    }
    
    pub fn reset_state(&mut self)  {
        
        todo!();
        /*
            usesBuffering = false;
                                bufferedRegions.clear();
                                playerBufferingInProgress = false;

                                pendingSeekRequest = false;

                                playSpeedMult = 1.0;
                                hasAudioFocus = false;
        */
    }
    
    pub fn get_audio_attributes() -> LocalRef<jobject> {
        
        todo!();
        /*
            // Video requires SDK version 21 or higher
                                jassert (getAndroidSDKVersion() >= 21);

                                auto* env = getEnv();

                                auto audioAttribsBuilder = LocalRef<jobject> (env->NewObject (AndroidAudioAttributesBuilder,
                                        AndroidAudioAttributesBuilder.constructor));
                                static constexpr jint contentTypeMovie = 3;
                                static constexpr jint usageMedia = 1;

                                LocalRef<jobject> (env->CallObjectMethod (audioAttribsBuilder, AndroidAudioAttributesBuilder.setContentType, contentTypeMovie));
                                LocalRef<jobject> (env->CallObjectMethod (audioAttribsBuilder, AndroidAudioAttributesBuilder.setUsage, usageMedia));

                                return LocalRef<jobject> (env->CallObjectMethod (audioAttribsBuilder, AndroidAudioAttributesBuilder.build));
        */
    }
}

