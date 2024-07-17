crate::ix!();

/**
  | A function type for use in setApplicationCrashHandler().
  | 
  | When called, its void* argument will
  | contain platform-specific data about
  | the crash.
  |
  */
pub type SystemStatsCrashHandlerFunction = fn(_0: *mut c_void) -> c_void;

/**
  | The set of possible results of the getOperatingSystemType()
  | method.
  |
  */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystemType {
    Unknown,
    MacOSX(MacOSXVersion),
    Windows(WindowsVersion),
    Linux,
    Android,
    iOS,
    WASM,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacOSXVersion {
    MacOSX_10_7,
    MacOSX_10_8,
    MacOSX_10_9,
    MacOSX_10_10,
    MacOSX_10_11,
    MacOSX_10_12,
    MacOSX_10_13,
    MacOSX_10_14,
    MacOSX_10_15,
    MacOS_11,
    MacOS_12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsVersion {
    Win2000,
    WinXP,
    WinVista,
    Windows7,
    Windows8_0,
    Windows8_1,
    Windows10,
}

impl OperatingSystemType {
    pub fn value(&self) -> u16 {
        match *self {
            OperatingSystemType::Unknown => 0,
            OperatingSystemType::MacOSX(_) => 0x0100,
            OperatingSystemType::Windows(_) => 0x0200,
            OperatingSystemType::Linux => 0x0400,
            OperatingSystemType::Android => 0x0800,
            OperatingSystemType::iOS => 0x1000,
            OperatingSystemType::WASM => 0x2000,
        }
    }

    pub fn is_windows(&self) -> bool {
        matches!(self, OperatingSystemType::Windows(_))
    }

    pub fn is_macos(&self) -> bool {
        matches!(self, OperatingSystemType::MacOSX(_))
    }
}

