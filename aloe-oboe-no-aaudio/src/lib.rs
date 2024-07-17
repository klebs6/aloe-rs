/*!
  | If the NDK is before O then define this
  | in your build so that AAudio.h will not
  | be included.
  |
  */
#[macro_use] mod imports; use imports::*;

//#[cfg(OBOE_NO_INCLUDE_AAUDIO)]
x!{no_aaudio}

pub struct Missing {}
pub type AAudioStreamStruct        = Missing;
pub type AAudioStreamBuilderStruct = Missing;
