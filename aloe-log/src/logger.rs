crate::ix!();

pub trait LoggerInterface {

    /**
      | This is overloaded by subclasses to
      | implement custom logging behaviour.
      | @see setCurrentLogger
      |
      */
    fn log_message(&mut self, message: &str);
}

lazy_static!{
    /*
    static Logger* currentLogger;
    Logger* Logger::currentLogger = nullptr;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/logging/aloe_Logger.h]

/**
  | Acts as an application-wide logging
  | class.
  | 
  | A subclass of Logger can be created and
  | passed into the Logger::setCurrentLogger
  | method and this will then be used by all
  | calls to writeToLog.
  | 
  | The logger class also contains methods
  | for writing messages to the debugger's
  | output stream.
  | 
  | @see FileLogger
  | 
  | @tags{Core}
  |
  */
#[derive(Default)]
pub struct Logger {

}

impl Drop for Logger {

    fn drop(&mut self) {
        todo!();
        /* 
        // You're deleting this logger while it's still being used!
        // Always call Logger::setCurrentLogger (nullptr) before deleting the active logger.
        jassert (currentLogger != this);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/logging/aloe_Logger.cpp]
impl Logger {
    
    /**
      | Sets the current logging class to use.
      | 
      | -----------
      | @note
      | 
      | the object passed in will not be owned
      | or deleted by the logger, so the caller
      | must make sure that it is not deleted
      | while still being used. A null pointer
      | can be passed-in to reset the system
      | to the default logger.
      |
      */
    pub fn set_current_logger(&mut self, new_logger: *mut Logger)  {
        
        todo!();
        /*
            currentLogger = newLogger;
        */
    }
    
    /**
      | Returns the current logger, or nullptr
      | if no custom logger has been set.
      |
      */
    pub fn get_current_logger(&mut self) -> *mut Logger {
        
        todo!();
        /*
            return currentLogger;
        */
    }
    
    /**
      | Writes a string to the current logger.
      | 
      | This will pass the string to the logger's
      | logMessage() method if a logger has
      | been set.
      | 
      | @see logMessage
      |
      */
    pub fn write_to_log(&mut self, message: &str)  {
        
        todo!();
        /*
            if (currentLogger != nullptr)
            currentLogger->logMessage (message);
        else
            outputDebugString (message);
        */
    }
    
    //-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_android_Misc.cpp]

    /**
      | Writes a message to the standard error
      | stream.
      | 
      | This can be called directly, or by using
      | the DBG() macro in aloe_PlatformDefs.h
      | (which will avoid calling the method
      | in non-debug builds).
      |
      */
    #[cfg(target_os="android")]
    pub fn output_debug_string(&mut self, text: &str)  {
        
        todo!();
        /*
            char* data = text.toUTF8().getAddress();
        const size_t length = CharPointer_UTF8::getBytesRequiredFor (text.getCharPointer());
        const size_t chunkSize = 1023;

        size_t position = 0;
        size_t numToRead = jmin (chunkSize, length);

        while (numToRead > 0)
        {
            __android_log_print (ANDROID_LOG_INFO, "Aloe", "%s", data + position);

            position += numToRead;
            numToRead = jmin (chunkSize, length - position);
        }
        */
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn output_debug_string(&mut self, text: &str)  {
        
        todo!();
        /*
            std::cerr << text << std::endl;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn output_debug_string(&mut self, text: &str)  {
        
        todo!();
        /*
            std::cerr << text << std::endl;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn output_debug_string(&mut self, text: &str)  {
        
        todo!();
        /*
            std::cerr << text << std::endl;
        */
    }
}
