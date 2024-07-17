crate::ix!();

lazy_static!{
    /*
    constexpr
    VideoComponent::VideoComponentPimpl::MediaSession::MediaSessionPlayer::MediaSessionPlayerStateInfo
    VideoComponent::VideoComponentPimpl::MediaSession::MediaSessionPlayer::stateInfos[];
    */
}

pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_UNKNOWN:               usize = 1;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_VIDEO_RENDERING_START: usize = 3;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_VIDEO_TRACK_LAGGING:   usize = 700;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_BUFFERING_START:       usize = 701;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_BUFFERING_END:         usize = 702;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_NETWORK_BANDWIDTH:     usize = 703;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_BAD_INTERLEAVING:      usize = 800;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_NOT_SEEKABLE:          usize = 801;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_METADATA_UPDATE:       usize = 802;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_AUDIO_NOT_PLAYING:     usize = 804;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_VIDEO_NOT_PLAYING:     usize = 805;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_UNSUPPORTED_SUBTITE:   usize = 901;
pub const MEDIA_SESSION_PLAYER_MEDIA_INFO_SUBTITLE_TIMED_OUT:    usize = 902;

pub struct MediaSessionPlayer<'a> {
    owner:                        &'a mut MediaSession<'a>,
    native_media_player:          GlobalRef,
    media_player_listener:        MediaPlayerListener<'a>,
    native_media_player_listener: GlobalRef,
    last_audio_volume:            f32, //= std::numeric_limits<float>::min();
    video_surface_holder:         GlobalRef,
    current_state:                MediaSessionPlayerState, //= MediaSessionPlayerState::idle;
}

impl<'a> MediaPlayerListenerOwner for MediaSessionPlayer<'a> {

     fn on_prepared(&mut self, media_player: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_VIDEO_LOG ("MediaPlayer::onPrepared()");

                            ignoreUnused (mediaPlayer);

                            currentState = MediaSessionPlayerState::prepared;

                            owner.playerPrepared();
        */
    }
    
    fn on_buffering_update(&mut self, 
        media_player: &mut LocalRef<jobject>,
        progress:     i32)  {
        
        todo!();
        /*
            ignoreUnused (mediaPlayer);

                            owner.playerBufferingUpdated (progress);
        */
    }
    
    fn on_seek_complete(&mut self, media_player: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_VIDEO_LOG ("MediaPlayer::onSeekComplete()");

                            ignoreUnused (mediaPlayer);

                            owner.playerSeekCompleted();
        */
    }
    
    fn on_completion(&mut self, media_player: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_VIDEO_LOG ("MediaPlayer::onCompletion()");

                            ignoreUnused (mediaPlayer);

                            currentState = MediaSessionPlayerState::complete;

                            owner.playerPlaybackCompleted();
        */
    }
    
    fn on_info(&mut self, 
        media_player: &mut LocalRef<jobject>,
        what:         i32,
        extra:        i32) -> bool {
        
        todo!();
        /*
            ALOE_VIDEO_LOG ("MediaPlayer::onInfo(), infoCode: " + String (what) + " (" + infoCodeToString (what) + ")"
                                + ", extraCode: " + String (extra));

                            ignoreUnused (mediaPlayer, extra);

                            if (what == MEDIA_INFO_BUFFERING_START)
                                owner.playerBufferingStarted();
                            else if (what == MEDIA_INFO_BUFFERING_END)
                                owner.playerBufferingEnded();

                            return true;
        */
    }
    
    fn on_error(&mut self, 
        media_player: &mut LocalRef<jobject>,
        what:         i32,
        extra:        i32) -> bool {
        
        todo!();
        /*
            auto errorMessage = errorCodeToString (what);
                            auto extraMessage = errorCodeToString (extra);

                            if (extraMessage.isNotEmpty())
                                errorMessage << ", " << extraMessage;

                            ALOE_VIDEO_LOG ("MediaPlayer::onError(), errorCode: " + String (what) + " (" + errorMessage + ")"
                                + ", extraCode: " + String (extra) + " (" + extraMessage + ")");

                            ignoreUnused (mediaPlayer);

                            currentState = MediaSessionPlayerState::error;

                            owner.errorOccurred (errorMessage);
                            return true;
        */
    }
}

impl<'a> MediaSessionPlayer<'a> {

    pub fn new(owner_to_use: &mut MediaSession) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                            mediaPlayerListener (*this),
                            nativeMediaPlayerListener (GlobalRef (CreateJavaInterface (&mediaPlayerListener,
                                        getNativeMediaPlayerListenerInterfaces())))
        */
    }
    
    pub fn set_display(&mut self, surface_holder: &LocalRef<jobject>)  {
        
        todo!();
        /*
            if (surfaceHolder == nullptr)
                            {
                                videoSurfaceHolder.clear();

                                if (nativeMediaPlayer.get() != nullptr)
                                    getEnv()->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setDisplay, nullptr);

                                return;
                            }

                            videoSurfaceHolder = GlobalRef (surfaceHolder);

                            if (nativeMediaPlayer.get() != nullptr)
                                getEnv()->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setDisplay, videoSurfaceHolder.get());
        */
    }
    
    pub fn load(&mut self, 
        media_id: &LocalRef<jstring>,
        extras:   &LocalRef<jobject>)  {
        
        todo!();
        /*
            ignoreUnused (extras);

                            closeVideo();

                            auto* env = getEnv();

                            nativeMediaPlayer = GlobalRef (LocalRef<jobject> (env->NewObject (AndroidMediaPlayer, AndroidMediaPlayer.constructor)));

                            currentState = MediaSessionPlayerState::idle;

                            auto uri = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidUri, AndroidUri.parse, mediaId.get()));
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setDataSource, getAppContext().get(), uri.get());

                            if (jniCheckHasExceptionOccurredAndClear())
                            {
                                owner.errorOccurred ("Could not find video under path provided (" + aloeString (mediaId) + ")");
                                return;
                            }

                            currentState = MediaSessionPlayerState::initialised;

                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setOnBufferingUpdateListener,   nativeMediaPlayerListener.get());
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setOnCompletionListener, nativeMediaPlayerListener.get());
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setOnErrorListener,      nativeMediaPlayerListener.get());
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setOnInfoListener,       nativeMediaPlayerListener.get());
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setOnPreparedListener,   nativeMediaPlayerListener.get());
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setOnSeekCompleteListener,   nativeMediaPlayerListener.get());

                            if (videoSurfaceHolder != nullptr)
                                env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setDisplay, videoSurfaceHolder.get());

                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.prepareAsync);

                            currentState = MediaSessionPlayerState::preparing;
        */
    }
    
    pub fn close_video(&mut self)  {
        
        todo!();
        /*
            if (nativeMediaPlayer.get() == nullptr)
                                return;

                            auto* env = getEnv();

                            if (getCurrentStateInfo().canCallStop)
                                env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.stop);

                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.release);
                            nativeMediaPlayer.clear();

                            currentState = MediaSessionPlayerState::end;
        */
    }
    
    pub fn is_video_open(&self) -> bool {
        
        todo!();
        /*
            return currentState == MediaSessionPlayerState::prepared || currentState == MediaSessionPlayerState::started
                                || currentState == MediaSessionPlayerState::paused || currentState == MediaSessionPlayerState::complete;
        */
    }
    
    pub fn get_playback_state_flag(&self) -> i32 {
        
        todo!();
        /*
            return getCurrentStateInfo().playbackStateFlag;
        */
    }
    
    pub fn get_allowed_actions(&self) -> i32 {
        
        todo!();
        /*
            return getCurrentStateInfo().allowedActions;
        */
    }
    
    pub fn get_video_duration(&self) -> i64 {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallGetVideoDuration)
                                return 0;

                            return getEnv()->CallIntMethod (nativeMediaPlayer, AndroidMediaPlayer.getDuration);
        */
    }
    
    pub fn get_video_native_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallGetVideoHeight)
                            {
                                jassertfalse;
                                return {};
                            }

                            auto* env = getEnv();

                            auto width  = (int) env->CallIntMethod (nativeMediaPlayer, AndroidMediaPlayer.getVideoWidth);
                            auto height = (int) env->CallIntMethod (nativeMediaPlayer, AndroidMediaPlayer.getVideoHeight);

                            return Rectangle<int> (0, 0, width, height);
        */
    }
    
    pub fn play(&mut self)  {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallStart)
                            {
                                jassertfalse;
                                return;
                            }

                            auto* env = getEnv();

                            // Perform a potentially pending volume setting
                            if (lastAudioVolume != std::numeric_limits<float>::min())
                                env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setVolume, (jfloat) lastAudioVolume, (jfloat) lastAudioVolume);

                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.start);

                            currentState = MediaSessionPlayerState::started;
        */
    }
    
    pub fn pause(&mut self)  {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallPause)
                            {
                                jassertfalse;
                                return;
                            }

                            getEnv()->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.pause);

                            currentState = MediaSessionPlayerState::paused;
        */
    }
    
    pub fn is_playing(&self) -> bool {
        
        todo!();
        /*
            return getCurrentStateInfo().isPlaying;
        */
    }
    
    pub fn set_play_position(&mut self, new_position_ms: i32)  {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallSeekTo)
                            {
                                jassertfalse;
                                return;
                            }

                            getEnv()->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.seekTo, (jint) newPositionMs);
        */
    }
    
    pub fn get_play_position(&self) -> i32 {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallGetCurrentPosition)
                                return 0.0;

                            return getEnv()->CallIntMethod (nativeMediaPlayer, AndroidMediaPlayer.getCurrentPosition);
        */
    }
    
    pub fn set_play_speed(&mut self, new_speed: f64)  {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallSetPlaybackParams)
                            {
                                jassertfalse;
                                return;
                            }

                            auto* env = getEnv();

                            auto playbackParams = LocalRef<jobject> (env->CallObjectMethod (nativeMediaPlayer, AndroidMediaPlayer.getPlaybackParams));
                            LocalRef<jobject> (env->CallObjectMethod (playbackParams, AndroidPlaybackParams.setSpeed, (jfloat) newSpeed));
                            env->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setPlaybackParams, playbackParams.get());

                            if (jniCheckHasExceptionOccurredAndClear())
                            {
                                // MediaPlayer can't handle speed provided!
                                jassertfalse;
                            }
        */
    }
    
    pub fn get_play_speed(&self) -> f64 {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallGetPlaybackParams)
                                return 0.0;

                            auto* env = getEnv();

                            auto playbackParams = LocalRef<jobject> (env->CallObjectMethod (nativeMediaPlayer, AndroidMediaPlayer.getPlaybackParams));
                            return (double) env->CallFloatMethod (playbackParams, AndroidPlaybackParams.getSpeed);
        */
    }
    
    pub fn set_audio_volume(&mut self, new_volume: f32)  {
        
        todo!();
        /*
            if (! getCurrentStateInfo().canCallSetVolume)
                            {
                                jassertfalse;
                                return;
                            }

                            lastAudioVolume = jlimit (0.0f, 1.0f, newVolume);

                            if (nativeMediaPlayer.get() != nullptr)
                                getEnv()->CallVoidMethod (nativeMediaPlayer, AndroidMediaPlayer.setVolume, (jfloat) lastAudioVolume, (jfloat) lastAudioVolume);
        */
    }
    
    pub fn get_audio_volume(&self) -> f32 {
        
        todo!();
        /*
            // There is NO getVolume() in MediaPlayer, so the value returned here can be incorrect!
                            return lastAudioVolume;
        */
    }
    
    pub fn get_current_state_info(&self) -> MediaSessionPlayerStateInfo {
        
        todo!();
        /*
            return stateInfos[static_cast<int> (currentState)];
        */
    }
    
    pub fn info_code_to_string(code: i32) -> String {
        
        todo!();
        /*
            switch (code)
                            {
                                case MEDIA_INFO_UNKNOWN:               return "Unknown";
                                case MEDIA_INFO_VIDEO_RENDERING_START: return "Rendering start";
                                case MEDIA_INFO_VIDEO_TRACK_LAGGING:   return "Video track lagging";
                                case MEDIA_INFO_BUFFERING_START:       return "Buffering start";
                                case MEDIA_INFO_BUFFERING_END:         return "Buffering end";
                                case MEDIA_INFO_NETWORK_BANDWIDTH:     return "Network bandwidth info available";
                                case MEDIA_INFO_BAD_INTERLEAVING:      return "Bad interleaving";
                                case MEDIA_INFO_NOT_SEEKABLE:          return "Video not seekable";
                                case MEDIA_INFO_METADATA_UPDATE:       return "Metadata updated";
                                case MEDIA_INFO_AUDIO_NOT_PLAYING:     return "Audio not playing";
                                case MEDIA_INFO_VIDEO_NOT_PLAYING:     return "Video not playing";
                                case MEDIA_INFO_UNSUPPORTED_SUBTITE:   return "Unsupported subtitle";
                                case MEDIA_INFO_SUBTITLE_TIMED_OUT:    return "Subtitle timed out";
                                default:                               return "";
                            }
        */
    }

    pub fn error_code_to_string(code: i32) -> String {
        
        todo!();
        /*
            enum
                            {
                                MEDIA_ERROR_UNSUPPORTED                        = -1010,
                                MEDIA_ERROR_MALFORMED                          = -1007,
                                MEDIA_ERROR_IO                                 = -1004,
                                MEDIA_ERROR_TIMED_OUT                          = -110,
                                MEDIA_ERROR_UNKNOWN                            = 1,
                                MEDIA_ERROR_SERVER_DIED                        = 100,
                                MEDIA_ERROR_NOT_VALID_FOR_PROGRESSIVE_PLAYBACK = 200
                            };

                            switch (code)
                            {
                                case MEDIA_ERROR_UNSUPPORTED:                        return "Unsupported bitstream";
                                case MEDIA_ERROR_MALFORMED:                          return "Malformed bitstream";
                                case MEDIA_ERROR_IO:                                 return "File/Network I/O error";
                                case MEDIA_ERROR_TIMED_OUT:                          return "Timed out";
                                case MEDIA_ERROR_UNKNOWN:                            return "Unknown error";
                                case MEDIA_ERROR_SERVER_DIED:                        return "Media server died (playback restart required)";
                                case MEDIA_ERROR_NOT_VALID_FOR_PROGRESSIVE_PLAYBACK: return "Video container not valid for progressive playback";
                                default:                                             return "";
                            }
        */
    }
    
    pub fn get_native_media_player_listener_interfaces() -> StringArray {
        
        todo!();
        /*
            #define IFPREFIX "android/media/MediaPlayer$"

                            return { IFPREFIX "OnCompletionListener", IFPREFIX "OnErrorListener",
                                IFPREFIX "OnInfoListener", IFPREFIX "OnPreparedListener",
                                IFPREFIX "OnBufferingUpdateListener", IFPREFIX "OnSeekCompleteListener"
                            };

                            #undef IFPREFIX
        */
    }
}
