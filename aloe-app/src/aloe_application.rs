crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/application/aloe_Application.h]

/**
  | An instance of this class is used to specify
  | initialisation and shutdown code for
  | the application.
  | 
  | Any application that wants to run an
  | event loop must declare a subclass of
  | ALOEApplicationBase or ALOEApplication,
  | and implement its various pure virtual
  | methods.
  | 
  | It then needs to use the START_ALOE_APPLICATION
  | macro somewhere in a CPP file to declare
  | an instance of this class and generate
  | suitable platform-specific boilerplate
  | code to launch the app.
  | 
  | -----------
  | @note
  | 
  | this class is derived from ALOEApplicationBase,
  | which contains most of the useful methods
  | and functionality. This derived class
  | is here simply as a convenient way to
  | also inherit from an ApplicationCommandTarget,
  | and to implement default versions of
  | some of the pure virtual base class methods.
  | But you can derive your app object directly
  | from ALOEApplicationBase if you want
  | to, and by doing so can avoid having a
  | dependency on the aloe_gui_basics
  | module.
  | 
  | e.g.
  | 
  | -----------
  | @code
  | 
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
  |     std::unique_ptr<MyApplicationWindow> myMainWindow;
  | };
  | 
  | // this generates boilerplate code to launch our app class:
  | START_ALOE_APPLICATION (MyALOEApp)
  | 
  | @see ALOEApplicationBase, START_ALOE_APPLICATION
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
pub struct ALOEApplication {
    base:  ALOEApplicationBase,
    base2: ApplicationCommandTarget,
}

impl Default for ALOEApplication {
    
    /**
      | Constructs a Aloe app object.
      | 
      | If subclasses implement a constructor
      | or destructor, they shouldn't call
      | any Aloe code in there - put your startup/shutdown
      | code in initialise() and shutdown()
      | instead.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        */
    }
}

impl Drop for ALOEApplication {

    /**
      | Destructor.
      | 
      | If subclasses implement a constructor
      | or destructor, they shouldn't call
      | any Aloe code in there - put your startup/shutdown
      | code in initialise() and shutdown()
      | instead.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/application/aloe_Application.cpp]
impl ALOEApplication {

    /**
      | Returns the global instance of the application
      | object being run.
      |
      */
    pub fn get_instance(&mut self) -> *mut ALOEApplication {
        
        todo!();
        /*
            return dynamic_cast<ALOEApplication*> (ALOEApplicationBase::getInstance());
        */
    }
    
    /**
      | Checks whether multiple instances
      | of the app are allowed.
      | 
      | If your application class returns true
      | for this, more than one instance is permitted
      | to run (except on OSX where the OS automatically
      | stops you launching a second instance
      | of an app without explicitly starting
      | it from the command-line).
      | 
      | If it's false, the second instance won't
      | start, but you will still get a callback
      | to anotherInstanceStarted() to tell
      | you about this - which gives you a chance
      | to react to what the user was trying to
      | do.
      |
      */
    pub fn more_than_one_instance_allowed(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    /**
      | Indicates that the user has tried to
      | start up another instance of the app.
      | 
      | This will get called even if moreThanOneInstanceAllowed()
      | is false.
      |
      */
    pub fn another_instance_started(&mut self, _0: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | This method is called when the application
      | is being put into background mode by
      | the operating system.
      |
      */
    pub fn suspended(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | This method is called when the application
      | is being woken from background mode
      | by the operating system.
      |
      */
    pub fn resumed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
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
    pub fn system_requested_quit(&mut self)  {
        
        todo!();
        /*
            quit();
        */
    }
    
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
    pub fn unhandled_exception<E: Error>(
        &mut self, 
        _0: *const E,
        _1: &String,
        _2: i32

    ) {
        
        todo!();
        /*
            jassertfalse;
        */
    }
    
    pub fn get_next_command_target(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn get_all_commands(&mut self, commands: &mut Vec<CommandID>)  {
        
        todo!();
        /*
            commands.add (StandardApplicationCommandIDs::quit);
        */
    }
    
    pub fn get_command_info(&mut self, 
        commandid: CommandID,
        result:    &mut ApplicationCommandInfo)  {
        
        todo!();
        /*
            if (commandID == StandardApplicationCommandIDs::quit)
        {
            result.setInfo (TRANS("Quit"),
                            TRANS("Quits the application"),
                            "Application", 0);

            result.defaultKeypresses.add (KeyPress ('q', ModifierKeys::commandModifier, 0));
        }
        */
    }
    
    pub fn perform<'a>(&mut self, info: &ApplicationCommandTargetInvocationInfo<'a>) -> bool {
        
        todo!();
        /*
            if (info.commandID == StandardApplicationCommandIDs::quit)
        {
            systemRequestedQuit();
            return true;
        }

        return false;
        */
    }
    
    pub fn initialise_app(&mut self) -> bool {
        
        todo!();
        /*
            if (ALOEApplicationBase::initialiseApp())
        {
           #if ALOE_MAC
            aloe_initialiseMacMainMenu(); // (needs to get the app's name)
           #endif

            return true;
        }

        return false;
        */
    }
}

#[cfg(target_os="macos")]
lazy_static!{
    /*
    extern void aloe_initialiseMacMainMenu();
    */
}

