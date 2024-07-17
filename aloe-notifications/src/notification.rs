crate::ix!();

/**
  | Represents a notification that can
  | be sent or received.
  |
  */
#[derive(Default)]
pub struct PushNotification {

    /* -------------- @name Common fields  -------------- */

    /**
      | Required: unique id that can be used
      | to later dismiss the notification (on
      | iOS available from version 10).
      |
      */
    identifier:    String,

    /**
      | Required: the title of the notification,
      | usually displayed in the first row.
      |
      */
    title:         String,

    /**
      | Required: the content of the notification,
      | usually displayed in the second row.
      |
      */
    body:          String,

    /**
      | Optional: additional text, that may
      | be displayed e.g. in the third row or
      | in the header area. Note that on Android,
      | depending on OS version, this may fight
      | for space with other components of the
      | notification, so use this field judiciously.
      | On iOS available from version 10. On
      | Android available from API 16.
      |
      */
    subtitle:      String,

    /**
      | Optional: allows the OS to visually
      | group, collapse, and expand a set of
      | notifications, note that OS may automatically
      | group notifications if no groupId is
      | specified. Available on Android API
      | 20 or above and iOS 10 or above.
      |
      */
    group_id:      String,

    /**
      | Optional: on platforms that support
      | it, can set a number this notification
      | represents.
      |
      */
    badge_number:  i32, // default = 0

    /**
      | Optional: empty when the notification
      | should be silent. When the name is set
      | to "default_os_sound", then a default
      | sound will be used.
      | 
      | For a custom sound on OSX, set the Url
      | to the name of a sound file (preferably
      | without an extension) and place the
      | sound file directly in bundle's "Resources"
      | directory (you can use "Xcode Resource"
      | tickbox in Proaloer to achieve that),
      | i.e. it cannot be in a subdirectory of
      | "Resources" like "Resources/sound".
      | Alternatively, if a sound file cannot
      | be found in bundle's "Resources" directory,
      | the OS may look for the sound in the following
      | paths: "~/Library/Sounds", "/Library/Sounds",
      | "/Network/Library/Sounds", "/System/Library/Sounds".
      | 
      | For a custom sound on iOS, set the Url
      | to a relative path within your bundle,
      | including file extension. For instance,
      | if your bundle contains "sounds" folder
      | with "my_sound.caf" file, then the
      | Url should be "sounds/my_sound.caf".
      | 
      | For a custom sound on Android, set Url
      | to the name of a raw resource file (without
      | an extension) that was included when
      | exporting an Android project in
      | 
      | Proaloer (see "Extra Android Raw Resources"
      | setting).
      |
      */
    sound_to_play: Url,

    /**
      | Optional: collection of additional
      | properties that may be passed as a dictionary.
      |
      */
    properties:    Var,

    /* ------------- @name iOS only fields  ------------- */

    /**
      | Required: determines set of actions
      | that will appear (as per setup done in
      | requestPermissionsWithSettings()).
      |
      */
    #[cfg(target_os="ios")]
    category:             String,

    /**
      | Optional: specifies number of seconds
      | before the notification should trigger.
      |
      */
    #[cfg(target_os="ios")]
    trigger_interval_sec: f64, // default = 0.

    /**
      | Optional: allows the notification
      | to continuously retrigger after triggerIntervalSec
      | seconds. Available from iOS 10.
      |
      */
    #[cfg(target_os="ios")]
    repeat:               bool, // default = false

    /* ----------- @name Android only fields  ----------- */

    /**
      | Required: name of an icon file (without
      | an extension) to be used for this notification.
      | This must be the name of one of the image
      | files included into resources when
      | exporting an Android project (see "Extra
      | Android Raw Resources" setting in Proaloer).
      |
      */
    #[cfg(target_os="android")]
    icon:                   String,


    /**
      | Required for Android API level 26 or
      | above: specifies notification channel
      | id. Refer to setupChannels(). Ignored
      | on earlier Android versions.
      |
      */
    #[cfg(target_os="android")]
    channel_id:             String,


    /**
      | Optional: an additional large icon
      | displayed in the notification content
      | view.
      |
      */
    #[cfg(target_os="android")]
    large_icon:             Image,


    /**
      | Optional: ticker text used for accessibility
      | services.
      |
      */
    #[cfg(target_os="android")]
    ticker_text:            String,


    /**
      | Optional: actions associated with
      | the notification. Note that the OS may
      | allow only a limited number of actions
      | to be presented, so always present most
      | important actions first. Available
      | from Android API 16 or above.
      |
      */
    #[cfg(target_os="android")]
    actions:                Vec<PushNotificationAction>,


    /**
      | Optional: set to default (0, 0, false),
      | to disable progress display.
      |
      */
    #[cfg(target_os="android")]
    progress:               PushNotificationProgress,


    /**
      | Optional: additional metadata used
      | as a hint to OS that a notification is
      | related to a specific person. Can be
      | useful for instance messaging apps.
      | Available from Android API 21 or above.
      |
      */
    #[cfg(target_os="android")]
    person:                 String,


    /**
      | Optional. Available from Android API
      | 21 or above.
      |
      */
    #[cfg(target_os="android")]
    ty:                     PushNotificationType, // default = unspecified


    /**
      | Optional. Available from Android API
      | 16 or above.
      |
      */
    #[cfg(target_os="android")]
    priority:               PushNotificationPriority, // default = medium


    /**
      | Optional.
      |
      */
    #[cfg(target_os="android")]
    lock_screen_appearance: PushNotificationLockScreenAppearance, // default = showPartially


    /**
      | Optional: if you set lockScreenAppearance
      | to showPartially, then you can provide
      | "public version" of your notification
      | that will be displayed on the lock screen.
      | This way you can control what information
      | is visible when the screen is locked.
      |
      */
    #[cfg(target_os="android")]
    public_version:         Box<PushNotification>,


    /**
      | Optional: Used to order notifications
      | within the same group. Available from
      | Android API 20 or above.
      |
      */
    #[cfg(target_os="android")]
    group_sort_key:         String,


    /**
      | Optional: if true, then this notification
      | will be a group summary of the group set
      | with groupId. Available from Android
      | API 20 or above.
      |
      */
    #[cfg(target_os="android")]
    group_summary:          bool, // default = false


    /**
      | Optional: sets accent colour. The default
      | colour will be used if accentColour
      | is not set. Available from Android API
      | 21 or above.
      |
      */
    #[cfg(target_os="android")]
    accent_colour:          Colour,


    /**
      | Optional: Sets the led colour. The hardware
      | will do its best to approximate the colour.
      | The default colour will be used if ledColour
      | is not set.
      |
      */
    #[cfg(target_os="android")]
    led_colour:             Colour,


    /**
      | Optional.
      |
      */
    #[cfg(target_os="android")]
    led_blink_pattern:      PushNotificationLedBlinkPattern,


    /**
      | Optional: sets the vibration pattern
      | in milliseconds. The first value indicates
      | how long to wait until vibration starts.
      | The second value indicates how long
      | to vibrate. The third value will say
      | how long to not vibrate and so on. For
      | instance, if the pattern is: 1000, 2000,
      | 3000, 4000 - then one second after receiving
      | a notification the device will vibrate
      | for two seconds, followed by 3 seconds
      | of no vibration and finally, 4 seconds
      | of vibration.
      |
      */
    #[cfg(target_os="android")]
    vibration_pattern:      Vec<i32>,


    /**
      | Optional: If true, the notification
      | will be automatically cancelled when
      | a user clicks it in the panel.
      |
      */
    #[cfg(target_os="android")]
    should_auto_cancel:     bool, // default = true


    /**
      | Optional: whether or not the notification
      | should bridge to other devices. Available
      | from Android API 20 or above.
      |
      */
    #[cfg(target_os="android")]
    local_only:             bool, // default = true


    /**
      | Optional: If true, then it cannot be
      | dismissed by the user and it must be dismissed
      | manually.
      | 
      | Typically used for ongoing background
      | tasks that the user is actively engaged
      | with. To dismiss such notification,
      | you need to call removeDeliveredNotification()
      | or removeAllDeliveredNotifications().
      |
      */
    #[cfg(target_os="android")]
    ongoing:                bool, // default = false


    /**
      | Optional: Set this flag if you would
      | only like the sound, vibrate and ticker
      | to be played if the notification is not
      | already showing.
      |
      */
    #[cfg(target_os="android")]
    alert_only_once:        bool, // default = false


    /**
      | Optional.
      |
      */
    #[cfg(target_os="android")]
    timestamp_visibility:   PushNotificationTimestampVisibility, // default = normal

    #[cfg(target_os="android")]
    badge_icon_type:        PushNotificationBadgeIconType, // default = large

    #[cfg(target_os="android")]
    group_alert_behaviour:  PushNotificationGroupAlertBehaviour, // default = alertAll

    /**
      | specifies a duration in milliseconds,
      | after which the notification should
      | be cancelled, if it is not already cancelled.
      | Available from Android API 26 or above.
      |
      */
    #[cfg(target_os="android")]
    timeout_after_ms:       i32, // default = 0
}

impl PushNotification {

    /**
      | Checks whether a given notification
      | is correctly configured for a given
      | OS.
      |
      */
    #[cfg(all(all(not(target_os="android"),not(target_os="ios")),not(target_os="macos")))]
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn new(other: &PushNotification) -> Self {
    
        todo!();
        /*


            : identifier (other.identifier),
          title (other.title),
          body (other.body),
          subtitle (other.subtitle),
          groupId (other.groupId),
          badgeNumber (other.badgeNumber),
          soundToPlay (other.soundToPlay),
          properties (other.properties),
          category (other.category),
          triggerIntervalSec (other.triggerIntervalSec),
          repeat (other.repeat),
          icon (other.icon),
          channelId (other.channelId),
          largeIcon (other.largeIcon),
          tickerText (other.tickerText),
          actions (other.actions),
          progress (other.progress),
          person (other.person),
          type (other.type),
          priority (other.priority),
          lockScreenAppearance (other.lockScreenAppearance),
          publicVersion (other.publicVersion.get() != nullptr ? new PushNotification (*other.publicVersion) : nullptr),
          groupSortKey (other.groupSortKey),
          groupSummary (other.groupSummary),
          accentColour (other.accentColour),
          ledColour (other.ledColour),
          ledBlinkPattern (other.ledBlinkPattern),
          vibrationPattern (other.vibrationPattern),
          shouldAutoCancel (other.shouldAutoCancel),
          localOnly (other.localOnly),
          ongoing (other.ongoing),
          alertOnlyOnce (other.alertOnlyOnce),
          timestampVisibility (other.timestampVisibility),
          badgeIconType (other.badgeIconType),
          groupAlertBehaviour (other.groupAlertBehaviour),
          timeoutAfterMs (other.timeoutAfterMs)
        */
    }
    
    #[cfg(target_os="macos")]
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }

    #[cfg(target_os="ios")]
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            const bool iOSEarlierThan10 = std::floor (NSFoundationVersionNumber) <= NSFoundationVersionNumber_iOS_9_x_Max;

        if (iOSEarlierThan10)
            return title.isNotEmpty() && body.isNotEmpty() && category.isNotEmpty();

        return title.isNotEmpty() && body.isNotEmpty() && identifier.isNotEmpty() && category.isNotEmpty();
        */
    }
}
