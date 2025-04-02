#![feature(allocator_api)]

#[cfg(target_os="macos")]
#[macro_use] mod imports; 

#[cfg(target_os="macos")]
use imports::*;

#[cfg(target_os="macos")] x!{native_mac_video}
#[cfg(target_os="macos")] x!{player_async_initializer}
#[cfg(target_os="macos")] x!{player_controller}
#[cfg(target_os="macos")] x!{player_controller_base}
#[cfg(target_os="macos")] x!{player_controller_macos}
#[cfg(target_os="macos")] x!{player_item_playback_status_observer_class}
#[cfg(target_os="macos")] x!{player_item_preparation_status_observer_class}
#[cfg(target_os="macos")] x!{player_status_observer_class}
#[cfg(target_os="macos")] x!{video_component}
#[cfg(target_os="macos")] x!{video_viewer_class}

pub struct Missing {}
pub type AVAsset       = Missing;
pub type AVPlayerItem  = Missing;
pub type AVPlayerLayer = Missing;
pub type AVPlayerView  = Missing;
pub type AVPlayer      = Missing;
pub type AVURLAsset    = Missing;
pub type CMTime        = Missing;
