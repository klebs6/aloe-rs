crate::ix!();

/**
  | The performance mode of the audio stream.
  |
  */
#[repr(i32)]
// aaudio_performance_mode_t
#[derive(Clone)]
pub enum OboePerformanceMode { 

    /**
      | No particular performance needs. Default.
      |
      */
    None = 10, // AAUDIO_PERFORMANCE_MODE_NONE,

    /**
      | Extending battery life is most important.
      |
      */
    PowerSaving = 11, // AAUDIO_PERFORMANCE_MODE_POWER_SAVING,

    /**
      | Reducing latency is most important.
      |
      */
    LowLatency = 12, // AAUDIO_PERFORMANCE_MODE_LOW_LATENCY
}
