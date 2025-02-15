crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/native/aloe_ScopedLowPowerModeDisabler.h]

/**
  | Disables low-power-mode for the duration
  | of an instance's lifetime.
  | 
  | Currently this is only implemented
  | on macOS, where it will disable the "App
  | Nap" power-saving feature.
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[no_move]
pub struct ScopedLowPowerModeDisabler {

    //impl_: Box<ScopedLowPowerModeDisablerImpl>,
}

impl ScopedLowPowerModeDisabler {

    pub fn new() -> Self {
    
        todo!();
        /*

            : impl (std::make_unique<Impl>())
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/native/aloe_ScopedLowPowerModeDisabler.cpp]

#[cfg(target_os="macos")]
#[test] fn test_scoped_low_power_mode_disabler() {

    todo!();

    /*

    class ScopedLowPowerModeDisabler::Impl
    {

        Impl() = default;
        ~Impl() { [[NSProcessInfo processInfo] endActivity: activity]; }

        id activity { [[NSProcessInfo processInfo] beginActivityWithOptions: NSActivityUserInitiatedAllowingIdleSystemSleep
                                                                     reason: @"App must remain in high-power mode"] };

        ALOE_DECLARE_NON_COPYABLE (Impl)
        ALOE_DECLARE_NON_MOVEABLE (Impl)
    };

    #else

    class ScopedLowPowerModeDisabler::Impl {};

    */
}

