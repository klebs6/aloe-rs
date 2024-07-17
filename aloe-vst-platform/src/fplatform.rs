/*!
   Description : Detect platform and set define
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/fplatform.h]

pub const kLittleEndian: usize = 0;
pub const kBigEndian:    usize = 1;

#[cfg(not(__INTEL_CXX11_MODE__))]
pub const SMTG_INTEL_CXX11_MODE: usize = 0;

#[cfg(__INTEL_CXX11_MODE__)]
macro_rules! smtg_intel_cxx11_mode {
    () => {
        /*
                __INTEL_CXX11_MODE__
        */
    }
}

#[cfg(not(__INTEL_COMPILER))]
pub const SMTG_INTEL_COMPILER: usize = 0;

#[cfg(__INTEL_COMPILER)]
macro_rules! smtg_intel_compiler {
    () => {
        /*
                __INTEL_COMPILER
        */
    }
}


#[cfg(not(SMTG_RENAME_ASSERT))]
#[cfg(SMTG_OS_WINDOWS)]
macro_rules! windows {
    () => {
        /*
                SMTG_OS_WINDOWS
        */
    }
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(not(SMTG_RENAME_ASSERT))]
macro_rules! mac {
    () => {
        /*
                SMTG_OS_MACOS
        */
    }
}

#[cfg(not(SMTG_RENAME_ASSERT))]
macro_rules! platform_64 {
    () => {
        /*
                SMTG_PLATFORM_64
        */
    }
}

#[cfg(not(SMTG_RENAME_ASSERT))]
macro_rules! pthreads {
    () => {
        /*
                SMTG_PTHREADS
        */
    }
}

/* -------------- Deprecation setting  -------------- */

#[cfg(not(SMTG_DEPRECATED_ATTRIBUTE))]
macro_rules! smtg_deprecated_attribute { ($msg:ident) => { } }

macro_rules! smtg_deprecated_msg {
    ($msg:ident) => {
        /*
                SMTG_DEPRECATED_ATTRIBUTE(msg)
        */
    }
}
