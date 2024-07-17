crate::ix!();

#[cfg(SMTG_OS_MACOS)]
#[cfg(not(SMTG_USE_STDATOMIC_H))]
#[cfg(all(MAC_OS_X_VERSION_10_11,MAC_OS_X_VERSION_MIN_REQUIRED))]
macro_rules! smtg_use_stdatomic_h {
    () => {
        /*
                (MAC_OS_X_VERSION_MIN_REQUIRED > MAC_OS_X_VERSION_10_11)
        */
    }
}

///------------------------
#[cfg(SMTG_OS_MACOS)]
#[cfg(not(SMTG_USE_STDATOMIC_H))]
#[cfg(not(all(MAC_OS_X_VERSION_10_11,MAC_OS_X_VERSION_MIN_REQUIRED)))]
pub const SMTG_USE_STDATOMIC_H: usize = 0;

///------------------------
#[cfg(SMTG_OS_LINUX)]
#[cfg(not(SMTG_USE_STDATOMIC_H))]
#[cfg(any(__ANDROID__,_LIBCPP_VERSION))]
pub const SMTG_USE_STDATOMIC_H: usize = 1;
