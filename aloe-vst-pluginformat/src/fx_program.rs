crate::ix!();

pub const fxb_version_num: i32 = 1;

pub struct fxProgram
{
    /**
      | 'CcnK'
      |
      */
    chunk_magic: i32,

    /**
      | of this chunk, excl. magic + byteSize
      |
      */
    byte_size:   i32,

    /**
      | 'FxCk'
      |
      */
    fx_magic:    i32,
    version:     i32,

    /**
      | fx unique id
      |
      */
    fxid:        i32,

    fx_version:  i32,
    num_params:  i32,
    prg_name:    [u8; 28],

    /**
      | variable no. of parameters
      |
      */
    params:      [f32; 1],
}


