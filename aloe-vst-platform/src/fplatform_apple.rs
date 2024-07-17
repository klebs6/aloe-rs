crate::ix!();

#[cfg(__APPLE__)]
pub mod apple {
    use super::*;

    pub const SMTG_OS_LINUX:   usize = 0;
    pub const SMTG_OS_MACOS:   usize = 1;
    pub const SMTG_OS_WINDOWS: usize = 0;



    #[cfg(not(SMTG_OS_IOS))]
    #[cfg(not(__CF_USE_FRAMEWORK_INCLUDES__))]
    macro_rules! cf_use_framework_includes { () => { } }

    #[cfg(not(SMTG_OS_IOS))]
    #[cfg(not(TARGET_API_MAC_CARBON))]
    pub const TARGET_API_MAC_CARBON: usize = 1;

    ///-------------------
    #[cfg(__LP64__)]
    pub const SMTG_PLATFORM_64: usize = 1;

    #[cfg(not(__LP64__))]
    pub const SMTG_PLATFORM_64: usize = 0;


    ///-------------------
    pub const COM_COMPATIBLE: usize = 0;
    pub const SMTG_PTHREADS:  usize = 1;


    #[cfg(__cplusplus)]
    #[cfg(all(_LIBCPP_VERSION,SMTG_CPP11))]
    pub const SMTG_CPP11_STDLIBSUPPORT: usize = 1;

    #[cfg(__cplusplus)]
    #[cfg(all(_LIBCPP_VERSION,SMTG_CPP11))]
    pub const SMTG_HAS_NOEXCEPT:        usize = 1;

    #[cfg(__cplusplus)]
    #[cfg(not(all(_LIBCPP_VERSION,SMTG_CPP11)))]
    pub const SMTG_CPP11_STDLIBSUPPORT: usize = 0;

    #[cfg(__cplusplus)]
    #[cfg(not(all(_LIBCPP_VERSION,SMTG_CPP11)))]
    pub const SMTG_HAS_NOEXCEPT:        usize = 0;

}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_os_ios {
    () => {
        /*
                TARGET_OS_IPHONE
        */
    }
}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_os_osx {
    () => {
        /*
                TARGET_OS_MAC && !TARGET_OS_IPHONE
        */
    }
}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_cpu_x86 {
    () => {
        /*
                TARGET_CPU_X86
        */
    }
}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_cpu_x86_64 {
    () => {
        /*
                TARGET_CPU_X86_64
        */
    }
}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_cpu_arm {
    () => {
        /*
                TARGET_CPU_ARM
        */
    }
}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_cpu_arm_64 {
    () => {
        /*
                TARGET_CPU_ARM64
        */
    }
}

#[repr(C)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[cfg(__BIG_ENDIAN__)]
#[cfg(target_os="macos")]
#[macro_export]
macro_rules! byteorder {
    () => {
        Endianness::BigEndian
    }
}

#[cfg(not(__BIG_ENDIAN__))]
#[cfg(target_os="macos")]
#[macro_export]
macro_rules! byteorder {
    () => {
        Endianness::LittleEndian
    }
}

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! plugin_api { () => { } }

#[cfg(target_os="macos")]
#[macro_export]
macro_rules! smtg_export_symbol {
    () => {
        /*
                __attribute__ ((visibility ("default")))
        */
    }
}

#[cfg(target_os="macos")]
#[macro_export]
#[cfg(__cplusplus)]
macro_rules! smtg_cpp11 {
    () => {
        /*
                (__cplusplus >= 201103L || SMTG_INTEL_CXX11_MODE)
        */
    }
}
