#[macro_use] mod imports; use imports::*;

x!{opensles_audiostreamopensles}
x!{opensles_openslesutilities}
x!{opensles_audiooutputstreamopensles}
x!{opensles_audiostreambuffered}
x!{opensles_engineopensles}
x!{opensles_audioinputstreamopensles}
x!{opensles_outputmixeropensles}

pub struct Missing {}
pub type SLAndroidConfigurationItf     = Missing;
pub type SLAndroidDataFormat_PCM_EX    = Missing;
pub type SLAndroidSimpleBufferQueueItf = Missing;
pub type SLDataFormat_PCM              = Missing;
pub type SLDataSink                    = Missing;
pub type SLDataSource                  = Missing;
pub type SLEngineItf                   = Missing;
pub type SLObjectItf                   = Missing;
pub type SLPlayItf                     = Missing;
pub type SLRecordItf                   = Missing;
pub type SLresult                      = Missing;

pub const MISSING:                  i32 = 0;
pub const SL_SPEAKER_FRONT_LEFT:    i32 = MISSING;
pub const SL_SPEAKER_FRONT_RIGHT:   i32 = MISSING;
pub const SL_SPEAKER_BACK_LEFT:     i32 = MISSING;
pub const SL_SPEAKER_BACK_RIGHT:    i32 = MISSING;
pub const SL_SPEAKER_FRONT_CENTER:  i32 = MISSING;
pub const SL_SPEAKER_LOW_FREQUENCY: i32 = MISSING;
pub const SL_SPEAKER_SIDE_LEFT:     i32 = MISSING;
pub const SL_SPEAKER_SIDE_RIGHT:    i32 = MISSING;
