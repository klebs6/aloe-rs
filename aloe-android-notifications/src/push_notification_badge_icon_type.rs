crate::ix!();

/**
  | Controls badge icon type to use if a notification
  | is shown as a badge. Available from Android
  | API 26 or above.
  |
  */
pub enum PushNotificationBadgeIconType
{
    none,
    small,
    large
}

impl Default for PushNotificationBadgeIconType {

    fn default() -> Self {
        todo!();
    }
}
