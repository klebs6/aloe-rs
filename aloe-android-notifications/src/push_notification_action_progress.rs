crate::ix!();

/**
  | Used to represent a progress of some
  | operation.
  |
  */
#[derive(Default)]
pub struct PushNotificationProgress
{
    /**
      | Max possible value of a progress. A typical
      | usecase is to set max to 100 and increment
      | current's value as percentage complete.
      |
      */
    max:           i32, // default = 0

    /**
      | Current progress value, should be from
      | 0 to max.
      |
      */
    current:       i32, // default = 0

    /**
      | If true, then the progress represents
      | a continuing activity indicator with
      | ongoing animation and no numeric value.
      |
      */
    indeterminate: bool, // default = false
}
