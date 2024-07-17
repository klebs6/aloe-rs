crate::ix!();

/**
  | Allows to control the time the device's
  | led is on and off.
  |
  */
#[derive(Default)]
pub struct PushNotificationLedBlinkPattern
{
    /**
      | The led will be on for the given number
      | of milliseconds, after which it will
      | turn off.
      |
      */
    ms_to_be_on:  i32, // default = 0

    /**
      | The led will be off for the given number
      | of milliseconds, after which it will
      | turn on.
      |
      */
    ms_to_be_off: i32, // default = 0
}
