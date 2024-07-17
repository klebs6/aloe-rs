crate::ix!();

pub struct SpinLockedPosInfo {
    mutex: SpinLock,
    info:  AudioPlayHeadCurrentPositionInfo,
}

impl Default for SpinLockedPosInfo {
    
    fn default() -> Self {
        todo!();
        /*


            info.resetToDefault()
        */
    }
}

impl SpinLockedPosInfo {

    /**
      | Wait-free, but setting new info may
      | fail if the main thread is currently
      | calling `get`. This is unlikely to
      | matter in practice because we'll be
      | calling `set` much more frequently than
      | `get`.
      */
    pub fn set(&mut self, new_info: &AudioPlayHeadCurrentPositionInfo)  {
        
        todo!();
        /*
            const SpinLock::ScopedTryLockType lock (mutex);

                if (lock.isLocked())
                    info = newInfo;
        */
    }
    
    pub fn get(&self) -> AudioPlayHeadCurrentPositionInfo {
        
        todo!();
        /*
            const SpinLock::ScopedLockType lock (mutex);
                return info;
        */
    }
}
