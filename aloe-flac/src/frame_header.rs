crate::ix!();

/**
  | FLAC frame header structure. (c.f.
  | <A HREF="../format.html#frame_header">format
  | specification</A>)
  |
  */
pub struct FrameHeader {

    /**
      | The number of samples per subframe.
      |
      */
    blocksize:          u32,

    /**
      | The sample rate in Hz.
      |
      */
    sample_rate:        u32,

    /**
      | The number of channels (== number of
      | subframes).
      |
      */
    channels:           u32,

    /**
      | The channel assignment for the frame.
      |
      */
    channel_assignment: ChannelAssignment,

    /**
      | The sample resolution.
      |
      */
    bits_per_sample:    u32,

    /**
      | The numbering scheme used for the frame.
      | As a convenience, the decoder will always
      | convert a frame number to a sample number
      | because the rules are complex.
      |
      */
    number_type:        FrameNumberType,

    /**
      | The frame number or sample number of
      | first sample in frame; use the \a number_type
      | value to determine which to use.
      |
      */
    number:             FlacFrameHeaderU,

    /**
      | CRC-8 (polynomial = x^8 + x^2 + x^1 + x^0,
      | initialized with 0) of the raw frame
      | header bytes, meaning everything before
      | the CRC byte including the sync code.
      |
      */
    crc:                u8,
}

pub union FlacFrameHeaderU {
    frame_number:  u32,
    sample_number: u64,
}

/**
  | == 0x3ffe; the frame header sync code
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_SYNC;
    */
}

/**
  | == 14 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_SYNC_LEN;
    */
}

/**
  | == 1 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_RESERVED_LEN;
    */
}

/**
  | == 1 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_BLOCKING_STRATEGY_LEN;
    */
}

/**
  | == 4 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_BLOCK_SIZE_LEN;
    */
}

/**
  | == 4 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_SAMPLE_RATE_LEN;
    */
}

/**
  | == 4 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_CHANNEL_ASSIGNMENT_LEN;
    */
}

/**
  | == 3 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_BITS_PER_SAMPLE_LEN;
    */
}

/**
  | == 1 (bit)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_ZERO_PAD_LEN;
    */
}

/**
  | == 8 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_HEADER_CRC_LEN;
    */
}

//-----------------------------------
/**
  | FLAC frame footer structure. (c.f.
  | <A HREF="../format.html#frame_footer">format
  | specification</A>)
  |
  */
pub struct FrameFooter {

    /**
      | CRC-16 (polynomial = x^16 + x^15 + x^2
      | + x^0, initialized with 0) of the bytes
      | before the crc, back to and including
      | the frame header sync code.
      |
      */
    crc: u16,
}

/**
  | == 16 (bits)
  |
  */
lazy_static!{
    /*
    extern  const unsigned FRAME_FOOTER_CRC_LEN;
    */
}
