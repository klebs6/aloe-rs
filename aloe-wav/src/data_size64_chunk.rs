crate::ix!();

/**
  | chunk ID = 'ds64' if data size > 0xffffffff,
  | 'JUNK' otherwise
  |
  */
#[bitfield]
pub struct DataSize64Chunk {

    /**
      | low 4 byte size of RF64 block
      |
      */
    riff_size_low:     u32,

    /**
      | high 4 byte size of RF64 block
      |
      */
    riff_size_high:    u32,

    /**
      | low 4 byte size of data chunk
      |
      */
    data_size_low:     u32,

    /**
      | high 4 byte size of data chunk
      |
      */
    data_size_high:    u32,

    /**
      | low 4 byte sample count of fact chunk
      |
      */
    sample_count_low:  u32,

    /**
      | high 4 byte sample count of fact chunk
      |
      */
    sample_count_high: u32,

    /**
      | number of valid entries in array 'table'
      |
      */
    table_length:      u32,
}
