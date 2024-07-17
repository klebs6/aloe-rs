crate::ix!();

/**
  | The underlying audio API used by the
  | audio stream.
  |
  */
#[repr(i32)]
#[derive(Clone)]
pub enum OboeAudioApi {

    /**
      | Try to use AAudio. If not available then
      | use OpenSL ES.
      |
      */
    Unspecified = kUnspecified,

    /**
      | Use OpenSL ES.
      |
      */
    OpenSLES,

    /**
      | Try to use AAudio. Fail if unavailable.
      |
      */
    AAudio
}
