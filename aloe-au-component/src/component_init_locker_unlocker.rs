crate::ix!();

/**
  | There are situations (11844772) where
  | we need to be able to release the lock
  | early.
  |
  */
#[cfg(TARGET_OS_MAC)]
pub struct ComponentInitLockerUnlocker { }

#[cfg(TARGET_OS_MAC)]
impl Drop for ComponentInitLockerUnlocker {

    fn drop(&mut self) {
        todo!();
        /*
            pthread_mutex_lock(&sComponentOpenMutex);
        */
    }
}

#[cfg(TARGET_OS_MAC)]
impl Default for ComponentInitLockerUnlocker {
    
    fn default() -> Self {
        todo!();
        /*


            pthread_mutex_unlock(&sComponentOpenMutex);
        */
    }
}
