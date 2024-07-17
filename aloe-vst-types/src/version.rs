crate::ix!();

/**
  | Vst3 SDK Version
  |
  */
/**
   SDK version for PClassInfo2
  */
#[cfg(not(kVstVersionString))]
pub const kVstVersionString: &'static str = "Vst 3.7.2";

pub const kVstVersionMajor: usize = 3;
pub const kVstVersionMinor: usize = 7;
pub const kVstVersionSub:   usize = 2;

macro_rules! vst_version {
    () => {
        /*
           ((kVstVersionMajor << 16) | (kVstVersionMinor << 8) | kVstVersionSub)
           */
    }
}

/**
  | Versions History which allows to write such
  | code:
  |
  | #if Vst_VERSION >= Vst_3_6_5_VERSION
  */
pub const Vst_3_7_2_VERSION:  usize = 0x030702;
pub const Vst_3_7_1_VERSION:  usize = 0x030701;
pub const Vst_3_7_0_VERSION:  usize = 0x030700;
pub const Vst_3_6_14_VERSION: usize = 0x03060E;
pub const Vst_3_6_13_VERSION: usize = 0x03060D;
pub const Vst_3_6_12_VERSION: usize = 0x03060C;
pub const Vst_3_6_11_VERSION: usize = 0x03060B;
pub const Vst_3_6_10_VERSION: usize = 0x03060A;
pub const Vst_3_6_9_VERSION:  usize = 0x030609;
pub const Vst_3_6_8_VERSION:  usize = 0x030608;
pub const Vst_3_6_7_VERSION:  usize = 0x030607;
pub const Vst_3_6_6_VERSION:  usize = 0x030606;
pub const Vst_3_6_5_VERSION:  usize = 0x030605;
pub const Vst_3_6_0_VERSION:  usize = 0x030600;
pub const Vst_3_5_0_VERSION:  usize = 0x030500;
pub const Vst_3_1_0_VERSION:  usize = 0x030100;
pub const Vst_3_0_0_VERSION:  usize = 0x030000;
