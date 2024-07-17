crate::ix!();

/**
  | Defines the audio source.
  | 
  | An audio source defines both a default
  | physical source of audio signal, and
  | a recording configuration.
  | 
  | -----------
  | @note
  | 
  | these match the equivalent values in
  | MediaRecorder.AudioSource in the
  | Android Java API.
  | 
  | This attribute only has an effect on
  | Android API 28+.
  |
  */
#[repr(i32)]
// aaudio_input_preset_t
#[derive(Clone)]
pub enum OboeInputPreset { 

    /**
      | Use this preset when other presets do
      | not apply.
      |
      */
    Generic = 1, // AAUDIO_INPUT_PRESET_GENERIC

    /**
      | Use this preset when recording video.
      |
      */
    Camcorder = 5, // AAUDIO_INPUT_PRESET_CAMCORDER

    /**
      | Use this preset when doing speech recognition.
      |
      */
    VoiceRecognition = 6, // AAUDIO_INPUT_PRESET_VOICE_RECOGNITION

    /**
      | Use this preset when doing telephony
      | or voice messaging.
      |
      */
    VoiceCommunication = 7, // AAUDIO_INPUT_PRESET_VOICE_COMMUNICATION

    /**
      | Use this preset to obtain an input with
      | no effects.
      | 
      | -----------
      | @note
      | 
      | this input will not have automatic gain
      | control so the recorded volume may be
      | very low.
      |
      */
    Unprocessed = 9, // AAUDIO_INPUT_PRESET_UNPROCESSED

    /**
      | Use this preset for capturing audio
      | meant to be processed in real time and
      | played back for live performance (e.g
      | karaoke).
      | 
      | The capture path will minimize latency
      | and coupling with playback path.
      |
      */
    VoicePerformance = 10, // AAUDIO_INPUT_PRESET_VOICE_PERFORMANCE
}
