crate::ix!();

pub struct fxSet
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
      | 'FxBk'
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

    /**
      | variable no. of programs
      |
      */
    programs:     [fxProgram; 1],
}
