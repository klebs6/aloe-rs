crate::ix!();

lazy_static!{
    /*
    static XErrorHandler   oldErrorHandler   = {};
    static XIOErrorHandler oldIOErrorHandler = {};
    */
}

/**
  | Usually happens when client-server
  | connection is broken
  |
  */
pub fn io_error_handler(_0: *mut Display) -> i32 {
    
    todo!();
        /*
            DBG ("ERROR: connection to X server broken.. terminating.");

            if (ALOEApplicationBase::isStandaloneApp())
                MessageManager::getInstance()->stopDispatchLoop();

            return 0;
        */
}

pub fn error_handler(
    display: *mut Display,
    event:   *mut XErrorEvent

) -> i32 {

    todo!();
        /*
            ignoreUnused (display, event);

           #if ALOE_DEBUG_XERRORS
            char errorStr[64]   = { 0 };
            char requestStr[64] = { 0 };

            X11Symbols::getInstance()->xGetErrorText (display, event->error_code, errorStr, 64);
            X11Symbols::getInstance()->xGetErrorDatabaseText (display, "XRequest", String (event->request_code).toUTF8(), "Unknown", requestStr, 64);

            DBG ("ERROR: X returned " << errorStr << " for operation " << requestStr);
           #endif

            return 0;
        */
}

pub fn install_xerror_handlers()  {
    
    todo!();
        /*
            oldIOErrorHandler = X11Symbols::getInstance()->xSetIOErrorHandler (ioErrorHandler);
            oldErrorHandler   = X11Symbols::getInstance()->xSetErrorHandler   (errorHandler);
        */
}

pub fn remove_xerror_handlers()  {
    
    todo!();
        /*
            X11Symbols::getInstance()->xSetIOErrorHandler (oldIOErrorHandler);
            oldIOErrorHandler = {};

            X11Symbols::getInstance()->xSetErrorHandler (oldErrorHandler);
            oldErrorHandler = {};
        */
}
