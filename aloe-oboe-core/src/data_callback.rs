crate::ix!();

/**
  | The result of an audio callback.
  |
  */
#[repr(i32)]
// aaudio_data_callback_result_t
#[derive(Clone)]
pub enum OboeDataCallbackResult { 

    /**
      | Indicates to the caller that the callbacks
      | should continue.
      |
      */
    Continue = 0, // AAUDIO_CALLBACK_RESULT_CONTINUE,

    /**
      | Indicates to the caller that the callbacks
      | should stop immediately.
      |
      */
    Stop = 1, // AAUDIO_CALLBACK_RESULT_STOP,
}
