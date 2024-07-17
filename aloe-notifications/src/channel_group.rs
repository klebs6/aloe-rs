crate::ix!();

#[cfg(not(target_os="android"))]
pub struct PushNotificationChannelGroup { }

/**
  | Android API level 26 or higher only:
  | represents a channel group. This allows
  | for visual grouping of corresponding
  | channels in notification settings
  | presented to the user.
  | 
  | At least one channel group has to be specified
  | before notifications can be sent.
  |
  */
#[cfg(target_os="android")]
pub struct PushNotificationChannelGroup
{
    /**
      | Required: Unique channel group identifier.
      |
      */
    identifier: String,

    /**
      | Required: User visible name of the channel
      | group.
      |
      */
    name:       String,
}
