/*!
  | @file base/thread/include/flock.h
  | 
  | Locks.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/thread/include/flock.h]

/** @defgroup baseLocks Locks */

#[cfg(SMTG_OS_WINDOWS)]
pub struct CRITSECT                         
{
    /**
      | PRTL_CRITICAL_SECTION_DEBUG DebugInfo;
      |
      */
    debug_info:      *mut c_void,

    /**
      | LONG LockCount;
      |
      */
    lock_count:      Steinberg::int32,

    /**
      | LONG RecursionCount;
      |
      */
    recursion_count: Steinberg::int32,

    /**
      | HANDLE OwningThread
      |
      */
    owning_thread:   *mut c_void,

    /**
      | HANDLE LockSemaphore
      |
      */
    lock_semaphore:  *mut c_void,

    /**
      | ULONG_PTR SpinCount
      |
      */
    spin_count:      Steinberg::int32,
}
