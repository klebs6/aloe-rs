crate::ix!();

/**
  | Spinlocks. These use memory barriers
  | as required to synchronize access to
  | shared memory protected by the lock.
  | The lock operation spins, but employs
  | various strategies to back off if the
  | lock is held, making it immune to most
  | priority-inversion livelocks.
  | 
  | The try operation immediately returns
  | false if the lock was held, true if it
  | took the lock.
  | 
  | The convention is that unlocked is zero,
  | locked is nonzero.
  |
  */
pub const CA_SPINLOCK_INIT: usize = 0;

pub type CASpinLock = i32;

#[inline] pub fn ca_spin_lock_lock(lock: Volatile<*mut CASpinLock>)  {
    
    todo!();
        /*
            #if TARGET_OS_MAC
        OSSpinLockLock(__lock);
    #else
        while (CAAtomicTestAndSetBarrier(0, (void*)__lock))
            usleep(1000); // ???
    #endif
        */
}

#[inline] pub fn ca_spin_lock_unlock(lock: Volatile<*mut CASpinLock>)  {
    
    todo!();
        /*
            #if TARGET_OS_MAC
        OSSpinLockUnlock(__lock);
    #else
        CAAtomicTestAndClearBarrier(0, (void*)__lock);
    #endif
        */
}

#[inline] pub fn ca_spin_lock_try(lock: Volatile<*mut CASpinLock>) -> bool {
    
    todo!();
        /*
            #if TARGET_OS_MAC
        return OSSpinLockTry(__lock);
    #else
        return (CAAtomicTestAndSetBarrier(0, (void*)__lock) == 0);
    #endif
        */
}
