#![feature(allocator_api)]
#[macro_use] mod imports; use imports::*;

x!{audiostream_deleter}
x!{audiostream_interface}
x!{audiostreambase}
x!{audio_stream_aaudio}
x!{oboe_stream}
x!{audiostream}
x!{audiostreamcallback}
x!{audiostreambuilder}
x!{aaudio_audiostreamaaudio}
x!{realtime_thread}

pub struct Missing {}

pub type ClockId                      = Missing;
pub type OboeAudioIODeviceOboeStream  = Missing;
pub type OboeAudioStreamCallback      = Missing;
//pub type OboeAudioStreamDataCallback  = Missing;
//pub type OboeAudioStreamErrorCallback = Missing;
pub type OboeManagedStream            = Missing;
pub type SharedMutex                  = Missing;

