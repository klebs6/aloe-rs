crate::ix!();

#[cfg(any(__gnu_linux__,__linux__))]
pub mod linux {

    use super::*;

    pub const SMTG_OS_LINUX:   usize = 1;
    pub const SMTG_OS_MACOS:   usize = 0;
    pub const SMTG_OS_WINDOWS: usize = 0;
    pub const SMTG_OS_IOS:     usize = 0;
    pub const SMTG_OS_OSX:     usize = 0;

    pub const COM_COMPATIBLE: usize = 0;
    pub const SMTG_PTHREADS:  usize = 1;


    #[cfg(__LP64__)]
    pub const SMTG_PLATFORM_64: usize = 1;

    #[cfg(not(__LP64__))]
    pub const SMTG_PLATFORM_64: usize = 0;

    #[cfg(__cplusplus)]
    pub const SMTG_CPP11: usize = __cplusplus >= 201103;

    #[cfg(__cplusplus)]
    #[cfg(all(__GNUG__,__GNUG___LT_8))]
    pub const SMTG_CPP11_STDLIBSUPPORT: usize = 0;

    #[cfg(__cplusplus)]
    #[cfg(not(all(__GNUG__,__GNUG___LT_8)))]
    pub const SMTG_CPP11_STDLIBSUPPORT: usize = 1;

    #[cfg(__cplusplus)]
    pub const SMTG_HAS_NOEXCEPT: usize = 1;

}

#[cfg(target_os="linux")]
#[macro_export]
macro_rules! smtg_cpu_x86 {
    () => {
        /*
                __i386__
        */
    }
}

#[cfg(target_os="linux")]
#[macro_export]
macro_rules! smtg_cpu_x86_64 {
    () => {
        /*
                __x86_64__
        */
    }
}

#[cfg(target_os="linux")]
#[macro_export]
macro_rules! smtg_cpu_arm {
    () => {
        /*
                __arm__
        */
    }
}

#[cfg(target_os="linux")]
#[macro_export]
macro_rules! smtg_cpu_arm_64 {
    () => {
        /*
                __aarch64__
        */
    }
}


#[cfg(target_endian = "little")]
#[cfg(target_os="linux")]
#[macro_export]
macro_rules! byteorder {
    () => {
        Endianness::LittleEndian
    }
}

#[cfg(target_endian = "big")]
#[cfg(target_os="linux")]
#[macro_export]
macro_rules! byteorder {
    () => {
        Endianness::BigEndian
    }
}

#[cfg(target_os="linux")]
#[macro_export]
macro_rules! plugin_api { () => { } }

#[cfg(target_os="linux")]
#[macro_export]
macro_rules! smtg_export_symbol {
    () => {
        /*
                __attribute__ ((visibility ("default")))
        */
    }
}
