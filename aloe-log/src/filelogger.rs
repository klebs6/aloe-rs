crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/logging/aloe_FileLogger.h]

/**
  | A simple implementation of a Logger
  | that writes to a file.
  | 
  | @see Logger
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct FileLogger {
    base:     Logger,
    log_file: File,
    log_lock: CriticalSection,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/logging/aloe_FileLogger.cpp]
impl FileLogger {

    /**
      | Returns the file that this logger is
      | writing to.
      |
      */
    pub fn get_log_file(&self) -> &File {
        
        todo!();
        /*
            return logFile;
        */
    }
    
    /**
      | Creates a FileLogger for a given file.
      | 
      | -----------
      | @param fileToWriteTo
      | 
      | the file that to use - new messages will
      | be appended to the file. If the file doesn't
      | exist, it will be created, along with
      | any parent directories that are needed.
      | ----------
      | @param welcomeMessage
      | 
      | when opened, the logger will write a
      | header to the log, along with the current
      | date and time, and this welcome message
      | ----------
      | @param maxInitialFileSizeBytes
      | 
      | if this is zero or greater, then if the
      | file already exists but is larger than
      | this number of bytes, then the start
      | of the file will be truncated to keep
      | the size down. This prevents a log file
      | getting ridiculously large over time.
      | The file will be truncated at a new-line
      | boundary. If this value is less than
      | zero, no size limit will be imposed;
      | if it's zero, the file will always be
      | deleted. Note that the size is only checked
      | once when this object is created - any
      | logging that is done later will be appended
      | without any checking
      |
      */
    pub fn new(
        file:                        &File,
        welcome_message:             &str,
        max_initial_file_size_bytes: Option<i64>

    ) -> Self {

        let max_initial_file_size_bytes: i64 =
            max_initial_file_size_bytes.unwrap_or(128 * 1024);
    
        todo!();
        /*
        : log_file(file),

            if (maxInitialFileSizeBytes >= 0)
            trimFileSize (logFile, maxInitialFileSizeBytes);

        if (! file.exists())
            file.create();  // (to create the parent directories)

        String welcome;
        welcome << newLine
                << "**********************************************************" << newLine
                << welcomeMessage << newLine
                << "Log started: " << Time::getCurrentTime().toString (true, true) << newLine;

        FileLogger::logMessage (welcome);
        */
    }
    
    /**
      | (implementation of the Logger virtual
      | method)
      |
      */
    pub fn log_message(&mut self, message: &str) {
        
        todo!();
        /*
            const ScopedLock sl (logLock);
        DBG (message);
        FileOutputStream out (logFile, 256);
        out << message << newLine;
        */
    }
    
    /**
      | This is a utility function which removes
      | lines from the start of a text file to
      | make sure that its total size is below
      | the given size.
      |
      */
    pub fn trim_file_size(
        &mut self, 
        file:                &File,
        max_file_size_bytes: i64

    ) {
        
        todo!();
        /*
            if (maxFileSizeBytes <= 0)
        {
            file.deleteFile();
        }
        else
        {
            const int64 fileSize = file.getSize();

            if (fileSize > maxFileSizeBytes)
            {
                TemporaryFile tempFile (file);

                {
                    FileOutputStream out (tempFile.getFile());
                    FileInputStream in (file);

                    if (! (out.openedOk() && in.openedOk()))
                        return;

                    in.setPosition (fileSize - maxFileSizeBytes);

                    for (;;)
                    {
                        const char c = in.readByte();
                        if (c == 0)
                            return;

                        if (c == '\n' || c == '\r')
                        {
                            out << c;
                            break;
                        }
                    }

                    out.writeFromInputStream (in, -1);
                }

                tempFile.overwriteTargetFileWithTemporary();
            }
        }
        */
    }
    
    /**
      | Returns an OS-specific folder where
      | log-files should be stored.
      | 
      | On Windows this will return a logger
      | with a path such as: c:\\Documents and
      | Settings\\username\\Application
      | Data\\[logFileSubDirectoryName]\\[logFileName]
      | 
      | On the Mac it'll create something like:
      | ~/Library/Logs/[logFileSubDirectoryName]/[logFileName]
      | 
      | @see createDefaultAppLogger
      |
      */
    pub fn get_system_log_file_folder(&mut self) -> File {
        
        todo!();
        /*
            #if ALOE_MAC
        return File ("~/Library/Logs");
       #else
        return File::getSpecialLocation (File::userApplicationDataDirectory);
       #endif
        */
    }
    
    /**
      | Helper function to create a log file
      | in the correct place for this platform.
      | 
      | The method might return nullptr if the
      | file can't be created for some reason.
      | 
      | -----------
      | @param logFileSubDirectoryName
      | 
      | the name of the subdirectory to create
      | inside the logs folder (as returned
      | by getSystemLogFileFolder). It's
      | best to use something like the name of
      | your application here.
      | ----------
      | @param logFileName
      | 
      | the name of the file to create, e.g. "MyAppLog.txt".
      | ----------
      | @param welcomeMessage
      | 
      | a message that will be written to the
      | log when it's opened.
      | ----------
      | @param maxInitialFileSizeBytes
      | 
      | (see the FileLogger constructor for
      | more info on this)
      |
      */
    pub fn create_default_app_logger(
        &mut self, 
        log_file_sub_directory_name: &str,
        log_file_name:               &str,
        welcome_message:             &str,
        max_initial_file_size_bytes: Option<i64>

    ) -> *mut FileLogger {

        let max_initial_file_size_bytes: i64 = max_initial_file_size_bytes.unwrap_or(128 * 1024);
        
        todo!();
        /*
            return new FileLogger (getSystemLogFileFolder().getChildFile (logFileSubDirectoryName)
                                                       .getChildFile (logFileName),
                               welcomeMessage, maxInitialFileSizeBytes);
        */
    }
    
    /**
      | Helper function to create a log file
      | in the correct place for this platform.
      | 
      | The filename used is based on the root
      | and suffix strings provided, along
      | with a time and date string, meaning
      | that a new, empty log file will be always
      | be created rather than appending to
      | an existing one.
      | 
      | The method might return nullptr if the
      | file can't be created for some reason.
      | 
      | -----------
      | @param logFileSubDirectoryName
      | 
      | the name of the subdirectory to create
      | inside the logs folder (as returned
      | by getSystemLogFileFolder). It's
      | best to use something like the name of
      | your application here.
      | ----------
      | @param logFileNameRoot
      | 
      | the start of the filename to use, e.g.
      | "MyAppLog_". This will have a timestamp
      | and the logFileNameSuffix appended
      | to it
      | ----------
      | @param logFileNameSuffix
      | 
      | the file suffix to use, e.g. ".txt"
      | ----------
      | @param welcomeMessage
      | 
      | a message that will be written to the
      | log when it's opened.
      |
      */
    pub fn create_date_stamped_logger(
        &mut self, 
        log_file_sub_directory_name: &str,
        log_file_name_root:          &str,
        log_file_name_suffix:        &str,
        welcome_message:             &str
    ) -> *mut FileLogger {
        
        todo!();
        /*
            return new FileLogger (getSystemLogFileFolder().getChildFile (logFileSubDirectoryName)
                                                       .getChildFile (logFileNameRoot + Time::getCurrentTime().formatted ("%Y-%m-%d_%H-%M-%S"))
                                                       .withFileExtension (logFileNameSuffix)
                                                       .getNonexistentSibling(),
                               welcomeMessage, 0);
        */
    }
}
