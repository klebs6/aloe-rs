crate::ix!();

pub struct fxChunkSet
{
    /**
      | 'CcnK'
      |
      */
    chunk_magic:  i32,

    /**
      | of this chunk, excl. magic + byteSize
      |
      */
    byte_size:    i32,

    /**
      | 'FxCh', 'FPCh', or 'FBCh'
      |
      */
    fx_magic:     i32,

    version:      i32,

    /**
      | fx unique id
      |
      */
    fxid:         i32,

    fx_version:   i32,
    num_programs: i32,
    future:       [u8; 128],
    chunk_size:   i32,

    /**
      | variable
      |
      */
    chunk:        [u8; 8],
}
