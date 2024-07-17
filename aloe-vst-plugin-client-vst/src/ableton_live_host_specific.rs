crate::ix!();

/**
  | Ableton Live host specific commands
  |
  */
pub struct AbletonLiveHostSpecific {

    /**
      | 'AbLi'
      |
      */
    magic:        u32,

    /**
      | 5 = realtime properties
      |
      */
    cmd:          i32,

    /**
      | sizeof (int)
      |
      */
    command_size: usize,

    /**
      | KCantBeSuspended = (1 << 2)
      |
      */
    flags:        i32,
}

pub const ABLETON_LIVE_HOST_SPECIFIC_KCANT_BE_SUSPENDED: usize = 1 << 2;


