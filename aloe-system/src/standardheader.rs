crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/system/aloe_StandardHeader.h]

/**
  | Current Aloe version number.
  | 
  | See also SystemStats::getALOEVersion()
  | for a string version.
  |
  */
pub const ALOE_MAJOR_VERSION: usize = 6;
pub const ALOE_MINOR_VERSION: usize = 1;
pub const ALOE_BUILDNUMBER:   usize = 2;

/**
  | Current Aloe version number.
  | 
  | Bits 16 to 32 = major version.
  | 
  | Bits 8 to 16 = minor version.
  | 
  | Bits 0 to 8 = point release.
  | 
  | See also SystemStats::getALOEVersion()
  | for a string version.
  |
  */
macro_rules! aloe_version {
    () => {
        /*
                ((ALOE_MAJOR_VERSION << 16) + (ALOE_MINOR_VERSION << 8) + ALOE_BUILDNUMBER)
        */
    }
}
