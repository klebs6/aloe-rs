#[cfg(target_os="ios")] #[macro_use] mod imports; 
#[cfg(target_os="ios")] use imports::*;

#[cfg(target_os="ios")] x!{audio_log}
#[cfg(target_os="ios")] x!{audio_session_holder}
#[cfg(target_os="ios")] x!{io_channel_data}
#[cfg(target_os="ios")] x!{io_channel_data_config}
#[cfg(target_os="ios")] x!{ios_audio_device}
#[cfg(target_os="ios")] x!{ios_audio_device_type}
#[cfg(target_os="ios")] x!{ios_audio_device_type_impl}
#[cfg(target_os="ios")] x!{session_routing}

pub struct Missing {}
pub type AVAudioSessionRouteChangeReason = Missing;
pub type AudioTimeStamp                  = Missing;
pub type AudioUnitPropertyID             = Missing;
pub type AudioUnitRemoteControlEvent     = Missing;
pub type AudioUnitRenderActionFlags      = Missing;
pub type HostCallbackInfo                = Missing;

