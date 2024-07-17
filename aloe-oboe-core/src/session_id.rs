crate::ix!();

/**
  | This attribute can be used to allocate
  | a session ID to the audio stream.
  | 
  | This attribute only has an effect on
  | Android API 28+.
  |
  */
#[derive(Clone)]
pub enum OboeSessionId {

    /**
      | Do not allocate a session ID.
      | 
      | Effects cannot be used with this stream.
      | Default.
      |
      */
    None = -1, // AAUDIO_SESSION_ID_NONE

    /**
      | Allocate a session ID that can be used
      | to attach and control effects using
      | the Java AudioEffects API.
      | 
      | -----------
      | @note
      | 
      | the use of this flag may result in higher
      | latency.
      | ----------
      | @note
      | 
      | this matches the value of AudioManager.AUDIO_SESSION_ID_GENERATE.
      |
      */
    Allocate = 0, // AAUDIO_SESSION_ID_ALLOCATE
}
