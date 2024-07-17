crate::ix!();

/**
  | The Usage attribute expresses *why*
  | you are playing a sound, what is this
  | sound used for.
  | 
  | This information is used by certain
  | platforms or routing policies to make
  | more refined volume or routing decisions.
  | 
  | Note that these match the equivalent
  | values in AudioAttributes in the Android
  | Java API.
  | 
  | This attribute only has an effect on
  | Android API 28+.
  |
  */
#[repr(i32)]
// aaudio_usage_t
#[derive(Clone)]
pub enum OboeUsage { 

    /**
      | Use this for streaming media, music
      | performance, video, podcasts, etcetera.
      |
      */
    Media =  1, // AAUDIO_USAGE_MEDIA

    /**
      | Use this for voice over IP, telephony,
      | etcetera.
      |
      */
    VoiceCommunication = 2, // AAUDIO_USAGE_VOICE_COMMUNICATION

    /**
      | Use this for sounds associated with
      | telephony such as busy tones, DTMF,
      | etcetera.
      |
      */
    VoiceCommunicationSignalling = 3, // AAUDIO_USAGE_VOICE_COMMUNICATION_SIGNALLING

    /**
      | Use this to demand the users attention.
      |
      */
    Alarm = 4, // AAUDIO_USAGE_ALARM

    /**
      | Use this for notifying the user when
      | a message has arrived or some other background
      | event has occured.
      |
      */
    Notification = 5, // AAUDIO_USAGE_NOTIFICATION

    /**
      | Use this when the phone rings.
      |
      */
    NotificationRingtone = 6, // AAUDIO_USAGE_NOTIFICATION_RINGTONE

    /**
      | Use this to attract the users attention
      | when, for example, the battery is low.
      |
      */
    NotificationEvent = 10, // AAUDIO_USAGE_NOTIFICATION_EVENT

    /**
      | Use this for screen readers, etcetera.
      |
      */
    AssistanceAccessibility = 11, // AAUDIO_USAGE_ASSISTANCE_ACCESSIBILITY

    /**
      | Use this for driving or navigation directions.
      |
      */
    AssistanceNavigationGuidance = 12, // AAUDIO_USAGE_ASSISTANCE_NAVIGATION_GUIDANCE

    /**
      | Use this for user interface sounds,
      | beeps, etcetera.
      |
      */
    AssistanceSonification = 13, // AAUDIO_USAGE_ASSISTANCE_SONIFICATION

    /**
      | Use this for game audio and sound effects.
      |
      */
    Game = 14, // AAUDIO_USAGE_GAME

    /**
      | Use this for audio responses to user
      | queries, audio instructions or help
      | utterances.
      |
      */
    Assistant = 16, // AAUDIO_USAGE_ASSISTANT
}
