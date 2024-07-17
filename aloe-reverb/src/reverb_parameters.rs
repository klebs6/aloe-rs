crate::ix!();

/**
  | Holds the parameters being used by a
  | Reverb object.
  |
  */
pub struct ReverbParameters
{
    /**
      | Room size, 0 to 1.0, where 1.0 is big,
      | 0 is small.
      |
      */
    room_size:   f32,

    /**
      | Damping, 0 to 1.0, where 0 is not damped,
      | 1.0 is fully damped.
      |
      */
    damping:     f32,

    /**
      | Wet level, 0 to 1.0
      |
      */
    wet_level:   f32,

    /**
      | Dry level, 0 to 1.0
      |
      */
    dry_level:   f32,

    /**
      | Reverb width, 0 to 1.0, where 1.0 is very
      | wide.
      |
      */
    width:       f32,

    /**
      | Freeze mode - values < 0.5 are
      | "normal" mode, values > 0.5 put the
      | reverb into a continuous feedback
      | loop.
      */
    freeze_mode: f32,
}

impl Default for ReverbParameters {

    fn default() -> Self {
        Self {
            room_size:   0.5,
            damping:     0.5,
            wet_level:   0.33,
            dry_level:   0.4,
            width:       1.0,
            freeze_mode: 0.0,
        }
    }
}
