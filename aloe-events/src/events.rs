crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/aloe_events.h]

/**
  | Config: ALOE_EXECUTE_APP_SUSPEND_ON_BACKGROUND_TASK
  | 
  | Will execute your application's suspend
  | method on an iOS background task, giving
  | you extra time to save your applications
  | state.
  |
  */
#[cfg(not(ALOE_EXECUTE_APP_SUSPEND_ON_BACKGROUND_TASK))]
pub const ALOE_EXECUTE_APP_SUSPEND_ON_BACKGROUND_TASK: usize = 0;

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/aloe_events.cpp]

pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:           usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:            usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS:         usize = 1;
pub const ALOE_CORE_INCLUDE_COM_SMART_PTR:          usize = 1;
pub const ALOE_EVENTS_INCLUDE_WIN32_MESSAGE_WINDOW: usize = 1;

#[cfg(ALOE_USE_WINRT_MIDI)]
pub const ALOE_EVENTS_INCLUDE_WINRT_WRAPPER: usize = 1;
