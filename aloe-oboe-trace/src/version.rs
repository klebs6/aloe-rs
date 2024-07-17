crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/Version.cpp]

/**
  | This variable enables the version
  | information to be read from the resulting
  | binary e.g.  by running `objdump -s
  | --section=.data <binary>`
  |
  | Please do not optimize or change in any
  | way.
  |
  | sorry!
  */
pub const OBOE_VERSION_TEXT: &'static str = "OboeVersionTODO";

pub fn oboe_get_version_text() -> *const u8 {
    
    todo!();
    /*
        return kVersionText;
    */
}
