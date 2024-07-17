crate::ix!();

pub struct MediaSessionPlayerStateInfo
{
    playback_state_flag:           i32, // default = 0
    allowed_actions:               i32, // default = 0
    is_playing:                    bool,
    can_call_get_current_position: bool,
    can_call_get_video_duration:   bool,
    can_call_get_video_height:     bool,
    can_call_get_video_width:      bool,
    can_call_get_playback_params:  bool,
    can_call_pause:                bool,
    can_call_prepare:              bool,
    can_call_seek_to:              bool,
    can_call_set_audio_attributes: bool,
    can_call_set_data_source:      bool,
    can_call_set_playback_params:  bool,
    can_call_set_volume:           bool,
    can_call_start:                bool,
    can_call_stop:                 bool,
}

pub enum MediaSessionPlayerState
{
    idle, 
    initialised, 
    preparing, 
    prepared, 
    started, 
    paused, 
    stopped, 
    complete, 
    error, 
    end
}

lazy_static!{
    /*
    static constexpr MediaSessionPlayerStateInfo stateInfos[] = {
            /* idle */
            {PlaybackState::STATE_NONE, PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
            false, true, false, true, true, false, false, false, false, true,
            true,  false, true, false, false},
            /* initialised */
            {PlaybackState::STATE_NONE, 0, // NB: could use action prepare, but that's API 24 onwards only
                false, true, false, true, true, true, false, true, false, true,
                false,  true, true, false, false},
            /* preparing */
            {PlaybackState::STATE_BUFFERING, 0,
                false, false, false, false, false, true, false, false, false, false,
                false,  false, false, false, false},
            /* prepared */
            {PlaybackState::STATE_PAUSED,
                PlaybackState::ACTION_PLAY | PlaybackState::ACTION_PLAY_PAUSE | PlaybackState::ACTION_PLAY_FROM_MEDIA_ID | PlaybackState::ACTION_STOP | PlaybackState::ACTION_SEEK_TO,
                false, true, true, true, true, true, false, false, true, true,
                false, true, true, true, true},
            /* started */
            {PlaybackState::STATE_PLAYING,
                PlaybackState::ACTION_PAUSE | PlaybackState::ACTION_PLAY_PAUSE | PlaybackState::ACTION_SEEK_TO | PlaybackState::ACTION_STOP | PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
                true, true, true, true, true, true, true, false, true, true,
                false, true, true, true, true},
            /* paused */
            {PlaybackState::STATE_PAUSED,
                PlaybackState::ACTION_PLAY | PlaybackState::ACTION_PLAY_PAUSE | PlaybackState::ACTION_SEEK_TO | PlaybackState::ACTION_STOP | PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
                false, true, true, true, true, true, true, false, true, true,
                false, true, true, true, true},
            /* stopped */
            {PlaybackState::STATE_STOPPED,
                PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
                false, true, true, true, true, true, false, true, false, true,
                false, false, true, false, true},
            /* complete */
            {PlaybackState::STATE_PAUSED,
                PlaybackState::ACTION_SEEK_TO | PlaybackState::ACTION_STOP | PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
                false, true, true, true, true, true, true, false, true, true,
                false, true, true, true, true},
            /* error */
            {PlaybackState::STATE_ERROR,
                PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false},
            /* end */
            {PlaybackState::STATE_NONE,
                PlaybackState::ACTION_PLAY_FROM_MEDIA_ID,
                false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false}
        };
    */
}
