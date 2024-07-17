crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_Process.h]

pub enum ProcessPriority
{
    LowPriority         = 0,
    NormalPriority      = 1,
    HighPriority        = 2,
    RealtimePriority    = 3
}

/**
  | Represents the current executable's
  | process.
  | 
  | This contains methods for controlling
  | the current application at the process-level.
  | 
  | @see Thread, ALOEApplicationBase
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct Process {

}

impl Process {
    
    #[cfg(target_os="linux")]
    pub fn is_foreground_process(&mut self) -> bool {
        
        todo!();
        /*
            return LinuxComponentPeer::isActiveApplication;
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn make_foreground_process(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn hide(&mut self)  { }

    /**
      | Returns true if this application process
      | is the one that the user is currently
      | using.
      |
      */
    pub fn is_foreground_process() -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Attempts to make the current process
      | the active one. (This is not possible
      | on some platforms).
      |
      */
    pub fn make_foreground_process()  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Hides the application (on an OS that
      | supports this, e.g. OSX, iOS, Android)
      |
      */
    pub fn hide()  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Raises the current process's privilege
      | level.
      | 
      | Does nothing if this isn't supported
      | by the current OS, or if process privilege
      | level is fixed.
      |
      */
    #[cfg(target_os="linux")]
    pub fn raise_privilege(&mut self)  {
        
        todo!();
        /*
            if (geteuid() != 0 && getuid() == 0) swapUserAndEffectiveUser();
        */
    }
    
    /**
      | Lowers the current process's privilege
      | level.
      | 
      | Does nothing if this isn't supported
      | by the current OS, or if process privilege
      | level is fixed.
      |
      */
    #[cfg(target_os="linux")]
    pub fn lower_privilege(&mut self)  {
        
        todo!();
        /*
            if (geteuid() == 0 && getuid() != 0) swapUserAndEffectiveUser();
        */
    }

    /**
      | Returns true if this process is being
      | hosted by a debugger.
      |
      */
    pub fn is_running_under_debugger(&mut self) -> bool {
        
        todo!();
        /*
            return aloe_isRunningUnderDebugger();
        */
    }
    
    /**
      | Tries to launch the OS's default reader
      | application for a given file or Url.
      |
      */
    pub fn open_document(
        documenturl: &String,
        parameters:  &String) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to launch the OS's default email
      | application to let the user create a
      | message.
      |
      */
    pub fn open_email_with_attachments(
        target_email_address: &String,
        email_subject:        &String,
        body_text:            &String,
        files_to_attach:      &[String]) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | WINDOWS ONLY - This returns the HINSTANCE
      | of the current module.
      | 
      | The return type is a void* to avoid being
      | dependent on windows.h - just cast it
      | to a HINSTANCE to use it.
      | 
      | In a normal Aloe application, this will
      | be automatically set to the module handle
      | of the executable.
      | 
      | If you've built a DLL and plan to use any
      | Aloe messaging or windowing classes,
      | you'll need to make sure you call the
      | setCurrentModuleInstanceHandle()
      | to provide the correct module handle
      | in your DllMain() function, because
      | the system relies on the correct instance
      | handle when opening windows.
      |
      */
    #[cfg(any(target_os="windows",DOXYGEN))]
    pub fn get_current_module_instance_handle()  {
        
        todo!();
        /*
        
        */
    }

    /**
      | WINDOWS ONLY - Sets a new module handle
      | to be used by the library.
      | 
      | The parameter type is a void* to avoid
      | being dependent on windows.h, but it
      | actually expects a HINSTANCE value.
      | 
      | @see getCurrentModuleInstanceHandle()
      |
      */
    #[cfg(any(target_os="windows",DOXYGEN))]
    pub fn set_current_module_instance_handle(new_handle: *mut c_void)  {
        
        todo!();
        /*
        
        */
    }
    
    /** OSX ONLY - Shows or hides the OSX dock icon for this app. */
    #[cfg(all(target_os="macos",ALOE_MODULE_AVAILABLE_aloe_gui_basics))]
    pub fn set_dock_icon_visible(is_visible: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | UNIX ONLY - Attempts to use setrlimit
      | to change the maximum number of file
      | handles that the app can open. Pass 0
      | or less as the parameter to mean 'infinite'.
      | Returns true if it succeeds.
      |
      */
    #[cfg(any(any(target_os="macos",target_os="linux"),any(target_os="bsd",DOXYGEN)))]
    pub fn set_max_number_of_file_handles(max_number_of_files: i32) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Kills the current process immediately.
      | 
      | This is an emergency process terminator
      | that kills the application immediately
      | - it's intended only for use only when
      | something goes horribly wrong.
      | 
      | @see ALOEApplicationBase::quit
      |
      */
    #[cfg(feature = "aloe_posix")]
    pub fn terminate(&mut self)  {
        
        todo!();
        /*
            #if ALOE_ANDROID
        _exit (EXIT_FAILURE);
       #else
        std::_Exit (EXIT_FAILURE);
       #endif
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(any(any(target_os="macos",target_os="linux"),target_os="bsd"))]
    pub fn set_max_number_of_file_handles(&mut self, new_max_number: i32) -> bool {
        
        todo!();
        /*
            rlimit lim;

        if (getrlimit (RLIMIT_NOFILE, &lim) == 0)
        {
            if (newMaxNumber <= 0 && lim.rlim_cur == RLIM_INFINITY && lim.rlim_max == RLIM_INFINITY)
                return true;

            if (newMaxNumber > 0 && lim.rlim_cur >= (rlim_t) newMaxNumber)
                return true;
        }

        lim.rlim_cur = lim.rlim_max = newMaxNumber <= 0 ? RLIM_INFINITY : (rlim_t) newMaxNumber;
        return setrlimit (RLIMIT_NOFILE, &lim) == 0;
        */
    }
}

impl Process {
    
    #[cfg(target_os="linux")]
    pub fn open_document(&mut self, 
        file_name:  &String,
        parameters: &String) -> bool {
        
        todo!();
        /*
            auto cmdString = fileName.replace (" ", "\\ ", false);
        cmdString << " " << parameters;

        if (cmdString.startsWithIgnoreCase ("file:")
             || File::createFileWithoutCheckingPath (fileName).isDirectory()
             || ! isFileExecutable (fileName))
        {
            StringArray cmdLines;

            for (auto browserName : { "xdg-open", "/etc/alternatives/x-www-browser", "firefox", "mozilla",
                                      "google-chrome", "chromium-browser", "opera", "konqueror" })
            {
                cmdLines.add (String (browserName) + " " + cmdString.trim().quoted());
            }

            cmdString = cmdLines.joinIntoString (" || ");
        }

        const char* const argv[4] = { "/bin/sh", "-c", cmdString.toUTF8(), nullptr };

        auto cpid = fork();

        if (cpid == 0)
        {
            setsid();

            // Child process
            execve (argv[0], (char**) argv, environ);
            exit (0);
        }

        return cpid >= 0;
        */
    }
}
