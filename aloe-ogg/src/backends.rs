/*!
  | function: libvorbis backend and mapping
  | structures; needed for static mode
  | headers
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/backends.h]

/*
  | this is exposed up here because we need
  | it for static modes. Lookups for each
  | backend aren't exposed because there's
  | no reason to do so
  |
  | this would all be simpler/shorter with
  | templates, but....
  */

/**
  | Floor backend generic
  |
  */
pub struct VorbisFuncFloor {

    pack:      fn(_0: *mut VorbisInfoFloor, 
                  _1: *mut OggPackBuffer) -> c_void,

    unpack:    fn(_0: *mut VorbisInfo, 
                  _1: *mut OggPackBuffer) -> *mut VorbisInfoFloor,

    look:      fn(_0: *mut VorbisDspState, 
                  _1: *mut VorbisInfoFloor) -> *mut VorbisLookFloor,

    free_info: fn(_0: *mut VorbisInfoFloor) -> c_void,
    free_look: fn(_0: *mut VorbisLookFloor) -> c_void,

    inverse1:  fn(_0: *mut VorbisBlock, 
                  _1: *mut VorbisLookFloor) -> *mut c_void,

    inverse2:  fn(_0:     *mut VorbisBlock, 
                  _1:     *mut VorbisLookFloor, 
                  buffer: *mut c_void, 
                  _2:     *mut f32) -> i32,
}

pub struct VorbisInfoFloor0 {
    order:       i32,
    rate:        i64,
    barkmap:     i64,
    ampbits:     i32,
    ampdb:       i32,

    /**
      <= 16
      */
    numbooks:    i32,

    books:       [i32; 16],

    /**
      encode-only config setting hacks for
      libvorbis
      */
    lessthan:    f32,

    /**
      encode-only config setting hacks for
      libvorbis
      */
    greaterthan: f32,
}


pub const VIF_POSIT: usize = 63;
pub const VIF_CLASS: usize = 16;
pub const VIF_PARTS: usize = 31;

pub struct VorbisInfoFloor1 {

    /**
      | 0 to 31
      |
      */
    partitions:     i32,

    /**
       0 to 15
      */
    partitionclass: [i32; VIF_PARTS],

    /**
       1 to 8
      */
    class_dim:      [i32; VIF_CLASS],

    /**
       0,1,2,3 (bits: 1<<n poss)
      */
    class_subs:     [i32; VIF_CLASS],

    /**
       subs ^ dim entries
      */
    class_book:     [i32; VIF_CLASS],

    /**
       [VIF_CLASS][subs]
      */
    class_subbook:  [[i32; VIF_CLASS]; 8],

    /**
       1 2 3 or 4
      */
    mult:           i32,

    /**
       first two implicit
      */
    postlist:       [i32; VIF_POSIT+2],

    /**
       encode side analysis parameters
      */
    maxover:        f32,

    maxunder:       f32,
    maxerr:         f32,
    twofitweight:   f32,
    twofitatten:    f32,
    n:              i32,
}

/**
  | Residue backend generic 
  |
  */
pub struct VorbisFuncResidue {
    pack:      fn(_0: *mut VorbisInfoResidue, 
                  _1: *mut OggPackBuffer) -> c_void,

    unpack:    fn(_0: *mut VorbisInfo, 
                  _1: *mut OggPackBuffer) -> *mut VorbisInfoResidue,

    look:      fn(_0: *mut VorbisDspState, 
                  _1: *mut VorbisInfoResidue) -> *mut VorbisLookResidue,

    free_info: fn(_0: *mut VorbisInfoResidue) -> c_void,
    free_look: fn(_0: *mut VorbisLookResidue) -> c_void,

    classx:    fn(_0: *mut VorbisBlock, 
                  _1: *mut VorbisLookResidue, 
                  _2: *mut *mut i32, 
                  _3: *mut i32, 
                  _4: i32) -> *mut *mut i64,

    forward:   fn(_0: *mut OggPackBuffer, 
                  _1: *mut VorbisBlock, 
                  _2: *mut VorbisLookResidue, 
                  _3: *mut *mut i32, 
                  _4: *mut i32, 
                  _5: i32, 
                  _6: *mut *mut i64, 
                  _7: i32) -> i32,

    inverse:   fn(_0: *mut VorbisBlock, 
                  _1: *mut VorbisLookResidue, 
                  _2: *mut *mut f32, 
                  _3: *mut i32, 
                  _4: i32) -> i32,
}

pub struct VorbisInfoResidue0 {

    /* --- block-partitioned VQ coded straight residue  --- */
    begin: i64,
    end:   i64,

    /* ------- first stage (lossless partitioning)  ------- */

    /**
       group n vectors per partition
      */
    grouping:     i32,

    /**
       possible codebooks for a partition
      */
    partitions:   i32,

    /**
       partitions ^ groupbook dim
      */
    partvals:     i32,

    /**
       huffbook for partitioning
      */
    groupbook:    i32,

    /**
       expanded out to pointers in lookup
      */
    secondstages: [i32; 64],

    /**
       list of second stage books
      */
    booklist:     [i32; 512],

    classmetric1: [i32; 64],
    classmetric2: [i32; 64],
}

/**
  | Mapping backend generic
  |
  */
pub struct VorbisFuncMapping {

    pack:      fn(_0: *mut VorbisInfo, 
                  _1: *mut VorbisInfoMapping, 
                  _2: *mut OggPackBuffer) -> c_void,

    unpack:    fn(_0: *mut VorbisInfo, 
                  _1: *mut OggPackBuffer) -> *mut VorbisInfoMapping,

    free_info: fn(_0: *mut VorbisInfoMapping) -> c_void,

    forward:   fn(vb: *mut VorbisBlock) -> i32,

    inverse:   fn(vb: *mut VorbisBlock, 
                  _0: *mut VorbisInfoMapping) -> i32,
}

pub struct VorbisInfoMapping0<'a> {

    /**
      | <= 16
      |
      */
    pub submaps:        i32,

    /**
      | up to 256 channels in a Vorbis stream
      |
      */
    pub chmuxlist:      &'a [i32],//[i32; 256],

    /**
      | [mux] submap to floors
      |
      */
    pub floorsubmap:    &'a [i32],//[i32; 16],

    /**
      | [mux] submap to residue
      |
      */
    pub residuesubmap:  &'a [i32],//[i32; 16],

    pub coupling_steps: i32,
    pub coupling_mag:   &'a [i32],//[i32; 256],
    pub coupling_ang:   &'a [i32],//[i32; 256],
}
