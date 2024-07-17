crate::ix!();

/**
  | A handy struct that uses XLockDisplay
  | and XUnlockDisplay to lock the X server
  | using RAII.
  | 
  | @tags{GUI}
  |
  */
pub struct ScopedXLock { }

impl Drop for ScopedXLock {
    fn drop(&mut self) {
        todo!();
        /*
            if (auto* xWindow = XWindowSystem::getInstanceWithoutCreating())
            if (auto* d = xWindow->getDisplay())
                X11Symbols::getInstance()->xUnlockDisplay (d);
        */
    }
}

impl ScopedXLock {
    
    pub fn new() -> Self {
    
        todo!();
        /*


            if (auto* xWindow = XWindowSystem::getInstanceWithoutCreating())
            if (auto* d = xWindow->getDisplay())
                X11Symbols::getInstance()->xLockDisplay (d);
        */
    }
}
