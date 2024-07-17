crate::ix!();

///=============================== X11 - Keys ===================================

pub enum MouseButtons
{
    NoButton     = 0,
    LeftButton   = 1,
    MiddleButton = 2,
    RightButton  = 3,
    WheelUp      = 4,
    WheelDown    = 5
}

lazy_static!{
    /*
    static int AltMask = 0;
        static int NumLockMask = 0;
        static bool numLock = false;
        static bool capsLock = false;
        static char keyStates [32];
    */
}

pub const EXTENDED_KEY_MODIFIER: u32 = 0x10000000;
