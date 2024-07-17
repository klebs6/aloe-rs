crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/thread/source/flock.cpp]

pub const DEBUG_LOCK: usize = 0;

#[cfg(SMTG_OS_WINDOWS)]
#[cfg(not(WINVER))]
pub const WINVER: usize = 0x0500;

#[cfg(SMTG_OS_WINDOWS)]
#[cfg(not(_WIN32_WINNT))]
macro_rules! win32_winnt {
    () => {
        /*
                WINVER
        */
    }
}

#[cfg(SMTG_OS_WINDOWS)]
macro_rules! init_cs {
    ($cs:ident) => {
        /*
        
            InitializeCriticalSection ((LPCRITICAL_SECTION)&cs);
        */
    }
}
