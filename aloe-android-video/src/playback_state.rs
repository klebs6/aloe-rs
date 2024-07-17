crate::ix!();

pub const PLAYBACK_STATE_NONE:                   usize = 0;
pub const PLAYBACK_STATE_STOPPED:                usize = 1;
pub const PLAYBACK_STATE_PAUSED:                 usize = 2;
pub const PLAYBACK_STATE_PLAYING:                usize = 3;
pub const PLAYBACK_STATE_FAST_FORWARDING:        usize = 4;
pub const PLAYBACK_STATE_REWINDING:              usize = 5;
pub const PLAYBACK_STATE_BUFFERING:              usize = 6;
pub const PLAYBACK_STATE_ERROR:                  usize = 7;
pub const PLAYBACK_STATE_CONNECTING:             usize = 8;
pub const PLAYBACK_STATE_SKIPPING_TO_PREVIOUS:   usize = 9;
pub const PLAYBACK_STATE_SKIPPING_TO_NEXT:       usize = 10;
pub const PLAYBACK_STATE_SKIPPING_TO_QUEUE_ITEM: usize = 11;

///----------------------
pub const PLAYBACK_ACTION_PAUSE:                 usize = 0x2;
pub const PLAYBACK_ACTION_PLAY:                  usize = 0x4;
pub const PLAYBACK_ACTION_PLAY_FROM_MEDIA_ID:    usize = 0x8000;
pub const PLAYBACK_ACTION_PLAY_PAUSE:            usize = 0x200;
pub const PLAYBACK_ACTION_SEEK_TO:               usize = 0x100;
pub const PLAYBACK_ACTION_STOP:                  usize = 0x1;
