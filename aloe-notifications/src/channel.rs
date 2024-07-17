crate::ix!();

/**
  | Controls how interruptive the notification
  | posted on this channel are.
  |
  */
pub enum PushNotificationChannelImportance
{
    none,
    min,
    low,
    normal,
    high,
    max
}

#[cfg(not(target_os="android"))]
pub struct PushNotificationChannel { }

/**
  | Android API level 26 or higher only:
  | Represents notification channel through
  | which notifications will be sent. Starting
  | from Android API level 26, you should
  | call setupChannels() at the start of
  | your application, before posting any
  | notifications. Then, when sending
  | notifications, assign a channel to
  | each created notification.
  |
  */
#[cfg(target_os="android")]
pub struct PushNotificationChannel {

    /**
      | Required: Unique channel identifier.
      |
      */
    identifier:             String,

    /**
      | Required: User facing name of the channel.
      |
      */
    name:                   String,

    /**
      | Required.
      |
      */
    importance:             PushNotificationChannelImportance, // default = normal

    /**
      | Optional.
      |
      */
    lock_screen_appearance: PushNotificationLockScreenAppearance, // default = Notification_showPartially

    /**
      | Optional: user visible description
      | of the channel.
      |
      */
    description:            String,

    /**
      | Required: group this channel belongs
      | to (see PushNotificationChannelGroup).
      |
      */
    group_id:               String,

    /**
      | Optional: sets the led colour for notifications
      | in this channel.
      |
      */
    led_colour:             Colour,

    /**
      | Optional: true if notifications in
      | this channel can bypass do not disturb
      | setting.
      |
      */
    bypass_do_not_disturb:  bool, // default = false

    /**
      | Optional: true if notifications in
      | this channel can show badges in a Launcher
      | application.
      |
      */
    can_show_badge:         bool, // default = false

    /**
      | Optional: true if notifications in
      | this channel should show lights (subject
      | to hardware support).
      |
      */
    enable_lights:          bool, // default = false

    /**
      | Optional: true if notifications in
      | this channel should trigger vibrations.
      |
      */
    enable_vibration:       bool, // default = false

    /**
      | Optional: sound to play in this channel.
      | See PushNotification::soundToPlay for
      | more info.
      |
      */
    sound_to_play:          Url,

    /**
      | Optional: vibration pattern for this
      | channel. See PushNotification::vibrationPattern
      | for more info.
      |
      */
    vibration_pattern:      Vec<i32>,
}
