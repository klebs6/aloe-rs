crate::ix!();

/**
  | The ContentType attribute describes
  | *what* you are playing.
  | 
  | It expresses the general category of
  | the content. This information is optional.
  | But in case it is known (for instance
  | {@link Movie} for a movie streaming
  | service or {@link Speech} for an audio
  | book application) this information
  | might be used by the audio framework
  | to enforce audio focus.
  | 
  | Note that these match the equivalent
  | values in AudioAttributes in the Android
  | Java API.
  | 
  | This attribute only has an effect on
  | Android API 28+.
  |
  */
// aaudio_content_type_t
#[repr(i32)]
#[derive(Clone)]
pub enum OboeContentType { 

    /**
      | Use this for spoken voice, audio books,
      | etcetera.
      |
      */
    Speech = 1, // AAUDIO_CONTENT_TYPE_SPEECH

    /**
      | Use this for pre-recorded or live music.
      |
      */
    Music = 2, // AAUDIO_CONTENT_TYPE_MUSIC

    /**
      | Use this for a movie or video soundtrack.
      |
      */
    Movie = 3, // AAUDIO_CONTENT_TYPE_MOVIE

    /**
      | Use this for sound is designed to accompany
      | a user action, such as a click or beep
      | sound made when the user presses a button.
      |
      */
    Sonification = 4, // AAUDIO_CONTENT_TYPE_SONIFICATION
}
