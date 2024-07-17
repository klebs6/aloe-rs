crate::ix!();

/**
  | The result of an operation. All except
  | the `OK` result indicates that an error
  | occurred.
  | 
  | The `OboeResult` can be converted into a
  | human readable string using `convertToText`.
  |
  */
// aaudio_result_t
#[repr(i32)]
#[derive(Default,Clone)]
pub enum OboeResult { 

    /**
      | AAUDIO_OK
      |
      */
    #[default]
    OK                   = 0, 

    /**
      | AAUDIO_ERROR_BASE,
      |
      */
    ErrorBase            = -900, 

    /**
      | AAUDIO_ERROR_DISCONNECTED,
      |
      */
    ErrorDisconnected    = -899, 

    /**
      | AAUDIO_ERROR_ILLEGAL_ARGUMENT,
      |
      */
    ErrorIllegalArgument = -898, 

    /**
      | AAUDIO_ERROR_INTERNAL,
      |
      */
    ErrorInternal        = -896, 

    /**
      | AAUDIO_ERROR_INVALID_STATE,
      |
      */
    ErrorInvalidState    = -895, 

    /**
      | AAUDIO_ERROR_INVALID_HANDLE,
      |
      */
    ErrorInvalidHandle   = -892, 

    /**
      | AAUDIO_ERROR_UNIMPLEMENTED,
      |
      */
    ErrorUnimplemented   = -890, 

    /**
      | AAUDIO_ERROR_UNAVAILABLE,
      |
      */
    ErrorUnavailable     = -889, 

    /**
      | AAUDIO_ERROR_NO_FREE_HANDLES,
      |
      */
    ErrorNoFreeHandles   = -888, 

    /**
      | AAUDIO_ERROR_NO_MEMORY,
      |
      */
    ErrorNoMemory        = -887, 

    /**
      | AAUDIO_ERROR_NULL,
      |
      */
    ErrorNull            = -886, 

    /**
      | AAUDIO_ERROR_TIMEOUT,
      |
      */
    ErrorTimeout         = -885, 

    /**
      | AAUDIO_ERROR_WOULD_BLOCK,
      |
      */
    ErrorWouldBlock      = -884, 

    /**
      | AAUDIO_ERROR_INVALID_FORMAT,
      |
      */
    ErrorInvalidFormat   = -883, 

    /**
      | AAUDIO_ERROR_OUT_OF_RANGE,
      |
      */
    ErrorOutOfRange      = -882, 

    /**
      | AAUDIO_ERROR_NO_SERVICE,
      |
      */
    ErrorNoService       = -881, 

    /**
      | AAUDIO_ERROR_INVALID_RATE,
      |
      */
    ErrorInvalidRate     = -880, 

    /**
      | Reserved for future AAudio result types
      |
      */
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    ErrorClosed          = -869,
}
