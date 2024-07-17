crate::ix!();

pub const steinberg_platform_string_win:   FIDString = "WIN";
pub const steinberg_platform_string_mac:   FIDString = "MAC";
pub const steinberg_platform_stringios:    FIDString = "IOS";
pub const steinberg_platform_string_linux: FIDString = "Linux";

///-----------------------
#[cfg(SMTG_OS_WINDOWS)]
pub const steinberg_platform_string: FIDString = kPlatformStringWin;

#[cfg(SMTG_OS_IOS)]
pub const steinberg_platform_string: FIDString = kPlatformStringIOS;

#[cfg(SMTG_OS_MACOS)]
pub const steinberg_platform_string: FIDString = kPlatformStringMac;

#[cfg(SMTG_OS_LINUX)]
pub const steinberg_platform_string: FIDString = kPlatformStringLinux;
