#![feature(allocator_api)]
#[macro_use] mod imports; use imports::*;

x!{native_mac_video}
x!{player_async_initializer}
x!{player_controller}
x!{player_controller_base}
x!{player_controller_macos}
x!{player_item_playback_status_observer_class}
x!{player_item_preparation_status_observer_class}
x!{player_status_observer_class}
x!{video_component}
x!{video_viewer_class}

pub struct Missing {}
pub type AVAsset       = Missing;
pub type AVPlayerItem  = Missing;
pub type AVPlayerLayer = Missing;
pub type AVPlayerView  = Missing;
pub type AVPlayer      = Missing;
pub type AVURLAsset    = Missing;
pub type CMTime        = Missing;

