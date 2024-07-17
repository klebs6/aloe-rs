crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_ApplicationBase.h]
pub trait ALOEApplicationBaseInterface:
GetInstance
+ GetApplicationName
+ GetApplicationVersion
+ MoreThanOneInstanceAllowed
+ InitializeApplication
+ ShutdownApplication
+ AnotherInstanceStarted
+ SystemRequestedQuit
+ ApplicationSuspended
+ ApplicationResumed
+ UnhandledException
+ MemoryWarningReceived
+ BackButtonPressed
+ InitializeApp { }

pub trait GetInstance {

    /**
      | Returns the global instance of the application
      | object that's running.
      |
      */
    fn get_instance() -> *mut ALOEApplicationBase {

        todo!();
        /*
           return appInstance;
           */
    }
}

pub trait GetApplicationName {

    /**
      | Returns the application's name.
      |
      */
    fn get_application_name(&mut self) -> String;

}

pub trait GetApplicationVersion {

    /**
      | Returns the application's version
      | number.
      |
      */
    fn get_application_version(&mut self) -> String;
}

pub trait MoreThanOneInstanceAllowed {

    /**
      | Checks whether multiple instances
      | of the app are allowed.
      | 
      | If your application class returns true
      | for this, more than one instance is permitted
      | to run (except on the Mac where this isn't
      | possible).
      | 
      | If it's false, the second instance won't
      | start, but you will still get a callback
      | to anotherInstanceStarted() to tell
      | you about this - which gives you a chance
      | to react to what the user was trying to
      | do.
      | 
      | @see anotherInstanceStarted
      |
      */
    fn more_than_one_instance_allowed(&mut self) -> bool;
}

pub trait InitializeApplication {

    /**
      | Called when the application starts.
      | 
      | This will be called once to let the application
      | do whatever initialisation it needs,
      | create its windows, etc.
      | 
      | After the method returns, the normal
      | event-dispatch loop will be run, until
      | the quit() method is called, at which
      | point the shutdown() method will be
      | called to let the application clear
      | up anything it needs to delete.
      | 
      | If during the initialise() method,
      | the application decides not to start-up
      | after all, it can just call the quit()
      | method and the event loop won't be run.
      | 
      | -----------
      | @param commandLineParameters
      | 
      | the line passed in does not include the
      | name of the executable, just the parameter
      | list. To get the parameters as an array,
      | you can call ALOEApplication::getCommandLineParameters()
      | @see shutdown, quit
      |
      */
    fn initialise(&mut self, command_line_parameters: &String);
}

pub trait ShutdownApplication {

    /**
      | Called to allow the application to clear
      | up before exiting.
      | 
      | After ALOEApplication::quit() has
      | been called, the event-dispatch loop
      | will terminate, and this method will
      | get called to allow the app to sort itself
      | out.
      | 
      | Be careful that nothing happens in this
      | method that might rely on messages being
      | sent, or any kind of window activity,
      | because the message loop is no longer
      | running at this point.
      | 
      | @see DeletedAtShutdown
      |
      */
    fn shutdown(&mut self);
}

pub trait AnotherInstanceStarted {

    /**
      | Indicates that the user has tried to
      | start up another instance of the app.
      | 
      | This will get called even if moreThanOneInstanceAllowed()
      | is false. It is currently only implemented
      | on Windows and Mac.
      | 
      | @see moreThanOneInstanceAllowed
      |
      */
    fn another_instance_started(&mut self, command_line: &String);
}

pub trait SystemRequestedQuit {

    /**
      | Called when the operating system is
      | trying to close the application.
      | 
      | The default implementation of this
      | method is to call quit(), but it may be
      | overloaded to ignore the request or
      | do some other special behaviour instead.
      | For example, you might want to offer
      | the user the chance to save their changes
      | before quitting, and give them the chance
      | to cancel.
      | 
      | If you want to send a quit signal to your
      | app, this is the correct method to call,
      | because it means that requests that
      | come from the system get handled in the
      | same way as those from your own application
      | code. So e.g. you'd call this method
      | from a "quit" item on a menu bar.
      |
      */
    fn system_requested_quit(&mut self);
}

pub trait ApplicationSuspended {

    /**
      | This method is called when the application
      | is being put into background mode by
      | the operating system.
      |
      */
    fn suspended(&mut self);
}

pub trait ApplicationResumed {

    /**
      | This method is called when the application
      | is being woken from background mode
      | by the operating system.
      |
      */
    fn resumed(&mut self);
}

pub trait UnhandledException {

    /**
      | If any unhandled exceptions make it
      | through to the message dispatch loop,
      | this callback will be triggered, in
      | case you want to log them or do some other
      | type of error-handling.
      | 
      | If the type of exception is derived from
      | the std::exception class, the pointer
      | passed-in will be valid. If the exception
      | is of unknown type, this pointer will
      | be null.
      |
      */
    fn unhandled_exception<E: Error>(
        &mut self, 
        _0:              *const E,
        source_filename: &String,
        line_number:     i32
    );
}

pub trait BackButtonPressed {

    /**
      | This will be called when the back button
      | on a device is pressed. The return value
      | should be used to indicate whether the
      | back button event has been handled by
      | the application, for example if you
      | want to implement custom navigation
      | instead of the standard behaviour on
      | Android.
      | 
      | This is currently only implemented
      | on Android devices.
      | 
      | -----------
      | @return
      | 
      | true if the event has been handled, or
      | false if the default OS behaviour should
      | happen
      |
      */
    fn back_button_pressed(&mut self) -> bool {
        false
    }
}

pub trait InitializeApp {

    fn initialise_app(&mut self) -> bool;
}

/**
  | Abstract base class for application classes.
  |
  | Note that in the aloe_gui_basics module,
  | there's a utility class ALOEApplication which
  | derives from ALOEApplicationBase, and takes
  | care of a few chores. Most of the time you'll
  | want to derive your class from ALOEApplication
  | rather than using ALOEApplicationBase
  | directly, but if you're not using the
  | aloe_gui_basics module then you might need to
  | go straight to this base class.
  |
  | Any application that wants to run an event
  | loop must declare a subclass of
  | ALOEApplicationBase, and implement its various
  | pure virtual methods.
  |
  | It then needs to use the
  | START_ALOE_APPLICATION macro somewhere in
  | a CPP file to declare an instance of this
  | class and generate suitable platform-specific
  | boilerplate code to launch the app.
  |
  | e.g. @code
  | class MyALOEApp  : public ALOEApplication
  | {
  |
  |     MyALOEApp()  {}
  |     ~MyALOEApp() {}
  |
  |     void initialise (const String& commandLine) override
  |     {
  |         myMainWindow.reset (new MyApplicationWindow());
  |         myMainWindow->setBounds (100, 100, 400, 500);
  |         myMainWindow->setVisible (true);
  |     }
  |
  |     void shutdown() override
  |     {
  |         myMainWindow = nullptr;
  |     }
  |
  |     const String getApplicationName() override
  |     {
  |         return "Super Aloe-o-matic";
  |     }
  |
  |     const String getApplicationVersion() override
  |     {
  |         return "1.0";
  |     }
  |
  |
  |     std::unique_ptr<MyApplicationWindow> myMainWindow;
  | };
  |
  | // this generates boilerplate code to launch our app class:
  | START_ALOE_APPLICATION (MyALOEApp)
  | @endcode
  |
  | @see ALOEApplication, START_ALOE_APPLICATION
  |
  | @tags{Events}
  */
#[no_copy]
pub struct ALOEApplicationBase {
    app_return_value:          i32, // default = 0
    still_initialising:        bool, // default = true

    #[cfg(ALOE_HANDLE_MULTIPLE_INSTANCES)]
    multiple_instance_handler: Box<aloe_application_base::MultipleInstanceHandler>,
}

pub mod aloe_application_base {

    use super::*;

    pub type CreateInstanceFunction = fn() -> *mut ALOEApplicationBase;

    lazy_static!{
        /*
        static CreateInstanceFunction createInstance;
        static ALOEApplicationBase* appInstance;

        #[cfg(target_os="ios")]
        static void* iOSCustomDelegate;

        //-------------------------
        ALOEApplicationBase::CreateInstanceFunction ALOEApplicationBase::createInstance = nullptr;
        ALOEApplicationBase* ALOEApplicationBase::appInstance = nullptr;

        #if ALOE_IOS
        void* ALOEApplicationBase::iOSCustomDelegate = nullptr;
        #endif
        */
    }

    ///---------------------------
    #[cfg(ALOE_HANDLE_MULTIPLE_INSTANCES)]
    #[no_copy]
    #[leak_detector]
    pub struct MultipleInstanceHandler {
        base:     ActionListener,
        app_lock: InterProcessLock,
    }

    #[cfg(ALOE_HANDLE_MULTIPLE_INSTANCES)]
    impl MultipleInstanceHandler {

        pub fn new(app_name: &String) -> Self {
        
            todo!();
            /*


                : appLock ("aloeAppLock_" + appName)
            */
        }
        
        pub fn send_command_line_to_preexisting_instance(&mut self) -> bool {
            
            todo!();
            /*
                if (appLock.enter (0))
                        return false;

                    if (auto* app = ALOEApplicationBase::getInstance())
                    {
                        MessageManager::broadcastMessage (app->getApplicationName() + "/" + app->getCommandLineParameters());
                        return true;
                    }

                    jassertfalse;
                    return false;
            */
        }
        
        pub fn action_listener_callback(&mut self, message: &String)  {
            
            todo!();
            /*
                if (auto* app = ALOEApplicationBase::getInstance())
                    {
                        auto appName = app->getApplicationName();

                        if (message.startsWith (appName + "/"))
                            app->anotherInstanceStarted (message.substring (appName.length() + 1));
                    }
            */
        }
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_ApplicationBase.cpp]
impl ALOEApplicationBase {

    fn default() -> Self {
    
        todo!();
        /*
        jassert (isStandaloneApp() && appInstance == nullptr);
        appInstance = this;
        */
    }
}
    
impl ALOEApplicationBase {

    /**
      | Returns the application's command
      | line parameters as a set of strings.
      | @see getCommandLineParameters
      |
      */
    pub fn get_command_line_parameter_array() -> Vec<String> {

        #[cfg(target_os="android")]
        fn _get_command_line_parameter_array(&mut self) -> StringArray {
            
            todo!();
            /*
                return {};
            */
        }
        
        #[cfg(all(target_os="windows",NOT_CONSOLE))]
        fn _get_command_line_parameter_array(&mut self) -> StringArray {
            
            todo!();
            /*
                StringArray s;
            int argc = 0;

            if (auto argv = CommandLineToArgvW (GetCommandLineW(), &argc))
            {
                s = StringArray (argv + 1, argc - 1);
                LocalFree (argv);
            }

            return s;
            */
        }

        #[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
        fn _get_command_line_parameter_array() -> Vec<String> {
            
            todo!();
            /*
                return StringArray (aloe_argv + 1, aloe_argc - 1);
            */
        }
            
        todo!();
        /*
        
        */
    }

    /**
      | Returns the application's command
      | line parameters as a single string.
      | @see getCommandLineParameterArray
      |
      */
    pub fn get_command_line_parameters() -> String {

        #[cfg(target_os="android")]
        fn _get_command_line_parameters() -> String {
            
            todo!();
            /*
                return {};
            */
        }
        
        #[cfg(all(target_os="windows",NOT_CONSOLE))]
        fn _get_command_line_parameters() -> String {
            
            todo!();
            /*
                return CharacterFunctions::findEndOfToken (CharPointer_UTF16 (GetCommandLineW()),
                                                       CharPointer_UTF16 (L" "),
                                                       CharPointer_UTF16 (L"\"")).findEndOfWhitespace();
            */
        }
        
        #[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
        fn _get_command_line_parameters() -> String {
            
            todo!();
            /*
                String argString;

            for (int i = 1; i < aloe_argc; ++i)
            {
                String arg { CharPointer_UTF8 (aloe_argv[i]) };

                if (arg.containsChar (' ') && ! arg.isQuotedString())
                    arg = arg.quoted ('"');

                argString << arg << ' ';
            }

            return argString.trim();
            */
        }
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the value that has been set as
      | the application's exit code. @see setApplicationReturnValue
      |
      */
    pub fn get_application_return_value(&self) -> i32 {
        
        todo!();
        /*
            return appReturnValue;
        */
    }

    /**
      | Returns true if this executable is running
      | as an app (as opposed to being a plugin
      | or other kind of shared library.
      |
      */
    pub fn is_standalone_app() -> bool {
        
        todo!();
        /*
            return createInstance != nullptr;
        */
    }

    /**
      | Returns true if the application hasn't
      | yet completed its initialise() method
      | and entered the main event loop.
      | 
      | This is handy for things like splash
      | screens to know when the app's up-and-running
      | properly.
      |
      */
    pub fn is_initialising(&self) -> bool {
        
        todo!();
        /*
            return stillInitialising;
        */
    }

    /**
      | Sets the value that should be returned
      | as the application's exit code when
      | the app quits.
      | 
      | This is the value that's returned by
      | the main() function. Normally you'd
      | leave this as 0 unless you want to indicate
      | an error code.
      | 
      | @see getApplicationReturnValue
      |
      */
    pub fn set_application_return_value(&mut self, new_return_value: i32)  {
        
        todo!();
        /*
            appReturnValue = newReturnValue;
        */
    }

    /**
       This is called on the Mac and iOS where the
       OS doesn't allow the stack to unwind on
       shutdown..
      */
    pub fn app_will_terminate_by_force(&mut self)  {
        
        todo!();
        /*
            ALOE_AUTORELEASEPOOL
        {
            {
                const std::unique_ptr<ALOEApplicationBase> app (appInstance);

                if (app != nullptr)
                    app->shutdownApp();
            }

            DeletedAtShutdown::deleteAll();
            MessageManager::deleteInstance();
        }
        */
    }
    
    /**
      | Signals that the main message loop should
      | stop and the application should terminate.
      | 
      | This isn't synchronous, it just posts
      | a quit message to the main queue, and
      | when this message arrives, the message
      | loop will stop, the shutdown() method
      | will be called, and the app will exit.
      | 
      | -----------
      | @note
      | 
      | this will cause an unconditional quit
      | to happen, so if you need an extra level
      | before this, e.g. to give the user the
      | chance to save their work and maybe cancel
      | the quit, you'll need to handle this
      | in the systemRequestedQuit() method
      | - see that method's help for more info.
      | 
      | @see MessageManager
      |
      */
    pub fn quit(&mut self)  {
        
        todo!();
        /*
            MessageManager::getInstance()->stopDispatchLoop();
        */
    }
    
    pub fn send_unhandled_exception<E: Error>(
        &mut self, 
        e:           *const E,
        source_file: *const u8,
        line_number: i32)  {
        
        todo!();
        /*
            if (auto* app = ALOEApplicationBase::getInstance())
        {
            // If you hit this assertion then the __FILE__ macro is providing a
            // relative path instead of an absolute path. On Windows this will be
            // a path relative to the build directory rather than the currently
            // running application. To fix this you must compile with the /FC flag.
            jassert (File::isAbsolutePath (sourceFile));

            app->unhandledException (e, sourceFile, lineNumber);
        }
        */
    }
    
    #[cfg(ALOE_HANDLE_MULTIPLE_INSTANCES)]
    pub fn send_command_line_to_preexisting_instance(&mut self) -> bool {
        
        todo!();
        /*
            jassert (multipleInstanceHandler == nullptr); // this must only be called once!

        multipleInstanceHandler.reset (new MultipleInstanceHandler (getApplicationName()));
        return multipleInstanceHandler->sendCommandLineToPreexistingInstance();
        */
    }
    
    #[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
    pub fn main_with_argcv(&mut self, 
        argc: i32,
        argv: *const &[u8]) -> i32 {
        
        todo!();
        /*
            ALOE_AUTORELEASEPOOL
        {
            aloe_argc = argc;
            aloe_argv = argv;

           #if ALOE_MAC
            initialiseNSApplication();
           #endif

           #if (ALOE_LINUX || ALOE_BSD) && ALOE_MODULE_AVAILABLE_aloe_gui_extra && (! defined(ALOE_WEB_BROWSER) || ALOE_WEB_BROWSER)
            if (argc >= 2 && String (argv[1]) == "--aloe-gtkwebkitfork-child")
                return aloe_gtkWebkitMain (argc, argv);
           #endif

           #if ALOE_IOS && ALOE_MODULE_AVAILABLE_aloe_gui_basics
            return aloe_iOSMain (argc, argv, iOSCustomDelegate);
           #else

            return ALOEApplicationBase::main();
           #endif
        }
        */
    }
    
    pub fn main(&mut self) -> i32 {
        
        todo!();
        /*
            ScopedAloeInitialiser_GUI libraryInitialiser;
        jassert (createInstance != nullptr);

        const std::unique_ptr<ALOEApplicationBase> app (createInstance());
        jassert (app != nullptr);

        if (! app->initialiseApp())
            return app->shutdownApp();

        ALOE_TRY
        {
            // loop until a quit message is received..
            MessageManager::getInstance()->runDispatchLoop();
        }
        ALOE_CATCH_EXCEPTION

        return app->shutdownApp();
        */
    }
    
    pub fn initialise_app(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_HANDLE_MULTIPLE_INSTANCES
        if ((! moreThanOneInstanceAllowed()) && sendCommandLineToPreexistingInstance())
        {
            DBG ("Another instance is running - quitting...");
            return false;
        }
       #endif

       #if ALOE_WINDOWS && ALOE_STANDALONE_APPLICATION && (! defined (_CONSOLE)) && (! ALOE_MINGW)
        if (AttachConsole (ATTACH_PARENT_PROCESS) != 0)
        {
            // if we've launched a GUI app from cmd.exe or PowerShell, we need this to enable printf etc.
            // However, only reassign stdout, stderr, stdin if they have not been already opened by
            // a redirect or similar.
            FILE* ignore;

            if (_fileno(stdout) < 0) freopen_s (&ignore, "CONOUT$", "w", stdout);
            if (_fileno(stderr) < 0) freopen_s (&ignore, "CONOUT$", "w", stderr);
            if (_fileno(stdin)  < 0) freopen_s (&ignore, "CONIN$",  "r", stdin);
        }
       #endif

        // let the app do its setting-up..
        initialise (getCommandLineParameters());

        stillInitialising = false;

        if (MessageManager::getInstance()->hasStopMessageBeenSent())
            return false;

       #if ALOE_HANDLE_MULTIPLE_INSTANCES
        if (auto* mih = multipleInstanceHandler.get())
            MessageManager::getInstance()->registerBroadcastListener (mih);
       #endif

        return true;
        */
    }
    
    pub fn shutdown_app(&mut self) -> i32 {
        
        todo!();
        /*
            jassert (ALOEApplicationBase::getInstance() == this);

       #if ALOE_HANDLE_MULTIPLE_INSTANCES
        if (auto* mih = multipleInstanceHandler.get())
            MessageManager::getInstance()->deregisterBroadcastListener (mih);
       #endif

        ALOE_TRY
        {
            // give the app a chance to clean up..
            shutdown();
        }
        ALOE_CATCH_EXCEPTION

        multipleInstanceHandler.reset();
        return getApplicationReturnValue();
        */
    }
}


/**
  | The ALOE_TRY/ALOE_CATCH_EXCEPTION
  | wrappers can be used to pass any uncaught
  | exceptions to the ALOEApplicationBase::sendUnhandledException()
  | method. This functionality can be enabled
  | with the ALOE_CATCH_UNHANDLED_EXCEPTIONS
  | macro.
  |
  */
#[cfg(ALOE_CATCH_UNHANDLED_EXCEPTIONS)]
macro_rules! aloe_catch_exception {
    () => {
        /*
        
            catch (const std::exception& e) { ALOEApplicationBase::sendUnhandledException (&e,      __FILE__, __LINE__); } 
            catch (...)                     { ALOEApplicationBase::sendUnhandledException (nullptr, __FILE__, __LINE__); }
        */
    }
}

#[cfg(not(any(target_os="ios",target_os="android")))]
pub const ALOE_HANDLE_MULTIPLE_INSTANCES: usize = 1;

///---------------------
impl Drop for ALOEApplicationBase {
    fn drop(&mut self) {
        todo!();
        /* 
        jassert (appInstance == this);
        appInstance = nullptr;
 */
    }
}


#[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
#[cfg(all(target_os="ios",ALOE_MODULE_AVAILABLE_aloe_gui_basics))]
lazy_static!{
    /*
    extern int aloe_iOSMain (int argc, const char* argv[], void* classPtr);
    */
}

#[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
#[cfg(target_os="macos")]
lazy_static!{
    /*
    extern void initialiseNSApplication();
    */
}

#[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
#[cfg(all(
        all(
            any(ALOE_LINUX,ALOE_BSD),
            ALOE_MODULE_AVAILABLE_aloe_gui_extra
        ),
        any(not(ALOE_WEB_BROWSER),ALOE_WEB_BROWSER))
)]
lazy_static!{
    /*
    extern int aloe_gtkWebkitMain (int argc, const char* argv[]);
    */
}

#[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
#[cfg(target_os="windows")]
lazy_static!{
    /*
    const char* const* aloe_argv = nullptr;
     int aloe_argc = 0;
    */
}

#[cfg(not(all(target_os="windows",NOT_CONSOLE)))]
#[cfg(not(target_os="windows"))]
lazy_static!{
    /*
    extern const char* const* aloe_argv;  // declared in aloe_core
     extern int aloe_argc;
    */
}
