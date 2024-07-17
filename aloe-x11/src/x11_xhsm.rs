crate::ix!();

#[cfg(ALOE_USE_XSHM)]
lazy_static!{
    /*
    static int trappedErrorCode = 0;
    */
}

#[cfg(ALOE_USE_XSHM)]
extern "C" {

    pub fn error_trap_handler(
            _0:  *mut Display,
            err: *mut XErrorEvent) -> i32 {
        
        todo!();
            /*
                trappedErrorCode = err->error_code;
                 return 0;
            */
    }
}

#[cfg(ALOE_USE_XSHM)]
pub fn is_shm_available(display: *mut Display) -> bool {
    
    todo!();
        /*
            static bool isChecked = false;
             static bool isAvailable = false;

             if (! isChecked)
             {
                 isChecked = true;

                 if (display != nullptr)
                 {
                     int major, minor;
                     Bool pixmaps;

                     XWindowSystemUtilities::ScopedXLock xLock;

                     if (X11Symbols::getInstance()->xShmQueryVersion (display, &major, &minor, &pixmaps))
                     {
                         trappedErrorCode = 0;
                         auto oldHandler = X11Symbols::getInstance()->xSetErrorHandler (errorTrapHandler);

                         XShmSegmentInfo segmentInfo;
                         zerostruct (segmentInfo);

                         if (auto* xImage = X11Symbols::getInstance()->xShmCreateImage (display,
                                                                                        X11Symbols::getInstance()->xDefaultVisual (display, X11Symbols::getInstance()->xDefaultScreen (display)),
                                                                                        24, ZPixmap, nullptr, &segmentInfo, 50, 50))
                         {
                             if ((segmentInfo.shmid = shmget (IPC_PRIVATE,
                                                              (size_t) (xImage->bytes_per_line * xImage->height),
                                                              IPC_CREAT | 0777)) >= 0)
                             {
                                 segmentInfo.shmaddr = (char*) shmat (segmentInfo.shmid, nullptr, 0);

                                 if (segmentInfo.shmaddr != (void*) -1)
                                 {
                                     segmentInfo.readOnly = False;
                                     xImage->data = segmentInfo.shmaddr;
                                     X11Symbols::getInstance()->xSync (display, False);

                                     if (X11Symbols::getInstance()->xShmAttach (display, &segmentInfo) != 0)
                                     {
                                         X11Symbols::getInstance()->xSync (display, False);
                                         X11Symbols::getInstance()->xShmDetach (display, &segmentInfo);

                                         isAvailable = true;
                                     }
                                 }

                                 X11Symbols::getInstance()->xFlush (display);
                                 X11Symbols::getInstance()->xDestroyImage (xImage);

                                 shmdt (segmentInfo.shmaddr);
                             }

                             shmctl (segmentInfo.shmid, IPC_RMID, nullptr);

                             X11Symbols::getInstance()->xSetErrorHandler (oldHandler);

                             if (trappedErrorCode != 0)
                                 isAvailable = false;
                         }
                     }
                 }
             }

             return isAvailable;
        */
}
