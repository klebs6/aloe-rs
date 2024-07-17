/*!
  | Oboe needs to be able to build on old NDKs so
  | we use hard coded constants.
  |
  | The correctness of these constants is verified
  | in "aaudio/AAudioLoader.cpp".
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/Definitions.h]

/**
  | Represents any attribute, property
  | or value which hasn't been specified.
  |
  */
pub const kUnspecified: i32 = 0;

/* ------- TODO: Investigate using std::chrono  ------- */

/**
  | The number of nanoseconds in a microsecond.
  | 1,000.
  |
  */
pub const kNanosPerMicrosecond: i64 = 1000;

/**
  | The number of nanoseconds in a millisecond.
  | 1,000,000.
  |
  */
pub const kNanosPerMillisecond: i64 = kNanosPerMicrosecond * 1000;

/**
  | The number of milliseconds in a second.
  | 1,000.
  |
  */
pub const kMillisPerSecond: i64 = 1000;

/**
  | The number of nanoseconds in a second.
  | 1,000,000,000.
  |
  */
pub const kNanosPerSecond: i64 = kNanosPerMillisecond * kMillisPerSecond;
