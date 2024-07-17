crate::ix!();

pub trait GetxRunCount {

    /**
      | Returns the number of under- or over
      | runs reported by the OS since playback/recording
      | has started.
      | 
      | This number may be different than determining
      | the Xrun count manually (by measuring
      | the time spent in the audio callback)
      | as the OS may be doing some buffering
      | internally - especially on mobile devices.
      | 
      | Returns -1 if playback/recording has
      | not started yet or if getting the underrun
      | count is not supported for this device
      | (Android SDK 23 and lower).
      |
      */
    fn getx_run_count(&self) -> i32 { -1 }
}
