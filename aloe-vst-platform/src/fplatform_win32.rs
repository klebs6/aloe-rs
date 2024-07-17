crate::ix!();

/* ----------- WIN32 AND WIN64 (WINDOWS)  ----------- */

#[cfg(_WIN32)]
pub mod win32 {

    use super::*;

    /* ----------- ARM32 AND ARM64 (WINDOWS)  ----------- */
    #[cfg(any(_M_ARM64,_M_ARM))]
    pub const SMTG_OS_WINDOWS_ARM: usize = 1;

    pub const SMTG_OS_LINUX:   usize = 0;
    pub const SMTG_OS_MACOS:   usize = 0;
    pub const SMTG_OS_WINDOWS: usize = 1;
    pub const SMTG_OS_IOS:     usize = 0;
    pub const SMTG_OS_OSX:     usize = 0;



    pub const COM_COMPATIBLE: usize = 1;
    pub const SMTG_PTHREADS:  usize = 0;

    #[cfg(not(_CRT_SECURE_NO_WARNINGS))]
    pub const _CRT_SECURE_NO_WARNINGS: bool = true;

    #[cfg(any(_WIN64,_M_ARM64))]
    pub const SMTG_PLATFORM_64: usize = 1;

    #[cfg(not(any(_WIN64,_M_ARM64)))]
    pub const SMTG_PLATFORM_64: usize = 0;

    #[cfg(not(WIN32))]
    pub const WIN32: usize = 1;

}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! smtg_cpu_x86 {
    () => {
        /*
                _M_IX86
        */
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! smtg_cpu_x86_64 {
    () => {
        /*
                _M_AMD64
        */
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! smtg_cpu_arm {
    () => {
        /*
                (_M_ARM && !_M_ARM64)
        */
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! smtg_cpu_arm_64 {
    () => {
        /*
                _M_ARM64
        */
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! byteorder {
    () => {
        Endianness::LittleEndian
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! plugin_api {
    () => {
        /*
                __stdcall
        */
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! smtg_export_symbol {
    () => {
        /*
                __declspec (dllexport)
        */
    }
}

#[cfg(target_os="win32")]
#[cfg(__cplusplus)]
#[macro_export]
macro_rules! smtg_cpp11 {
    () => {
        /*
                __cplusplus >= 201103L || _MSC_VER > 1600 || SMTG_INTEL_CXX11_MODE
        */
    }
}

#[cfg(target_os="win32")]
#[cfg(__cplusplus)]
#[macro_export]
macro_rules! smtg_cpp11_stdlibsupport {
    () => {
        /*
                SMTG_CPP11
        */
    }
}

#[cfg(target_os="win32")]
#[cfg(__cplusplus)]
#[macro_export]
macro_rules! smtg_has_noexcept {
    () => {
        /*
                _MSC_VER >= 1900 || (SMTG_INTEL_CXX11_MODE && SMTG_INTEL_COMPILER >= 1300)
        */
    }
}

#[cfg(target_os="win32")]
#[macro_export]
macro_rules! smtg_deprecated_attribute {
    ($message:ident) => {
        /*
                __declspec (deprecated ("Is Deprecated: " message))
        */
    }
}


