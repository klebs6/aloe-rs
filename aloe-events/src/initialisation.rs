crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_Initialisation.h]

/**
  | Initialises Aloe's GUI classes.
  | 
  | If you're embedding Aloe into an application
  | that uses its own event-loop rather
  | than using the START_ALOE_APPLICATION
  | macro, call this function before making
  | any Aloe calls, to make sure things are
  | initialised correctly.
  | 
  | -----------
  | @note
  | 
  | if you're creating a Aloe DLL for Windows,
  | you may also need to call the Process::setCurrentModuleInstanceHandle()
  | method.
  | 
  | @see shutdownAloe_GUI()
  |
  */
pub fn initialise_aloe_gui()  {
    
    todo!();
    /*
    
    */
}

/**
  | Clears up any static data being used
  | by Aloe's GUI classes.
  | 
  | If you're embedding Aloe into an application
  | that uses its own event-loop rather
  | than using the START_ALOE_APPLICATION
  | macro, call this function in your shutdown
  | code to clean up any Aloe objects that
  | might be lying around.
  | 
  | @see initialiseAloe_GUI()
  |
  */
pub fn shutdown_aloe_gui()  {
    
    todo!();
    /*
    
    */
}

/**
  | A utility object that helps you initialise
  | and shutdown Aloe correctly using an
  | RAII pattern.
  | 
  | When the first instance of this class
  | is created, it calls initialiseAloe_GUI(),
  | and when the last instance is deleted,
  | it calls shutdownAloe_GUI(), so that
  | you can easily be sure that as long as
  | at least one instance of the class exists,
  | the library will be initialised.
  | 
  | This class is particularly handy to
  | use at the beginning of a console app's
  | main() function, because it'll take
  | care of shutting down whenever you return
  | from the main() call.
  | 
  | Be careful with your threading though
  | - to be safe, you should always make sure
  | that these objects are created and deleted
  | on the message thread.
  | 
  | @tags{Events}
  |
  */
pub struct ScopedAloeInitialiser_GUI {

}

impl Drop for ScopedAloeInitialiser_GUI {

    /**
      | The destructor simply calls shutdownAloe_GUI().
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      if (--numScopedInitInstances == 0) shutdownAloe_GUI();  */
    }
}

/**
  | To start a Aloe app, use this macro: START_ALOE_APPLICATION
  | (AppSubClass) where AppSubClass is
  | the name of a class derived from ALOEApplication
  | or ALOEApplicationBase.
  | 
  | See the ALOEApplication and ALOEApplicationBase
  | class documentation for more details.
  |
  */
#[cfg(all(target_os="windows",not(_CONSOLE)))]
#[macro_export]
macro_rules! aloe_main_function {
    () => {
        /*
        
              int __stdcall WinMain (struct HINSTANCE__*, struct HINSTANCE__*, char*, int)  
        */
    }
}

#[cfg(all(target_os="windows",not(_CONSOLE)))]
#[macro_export]
macro_rules! aloe_main_function_args {
    () => { }
}

#[cfg(not(all(target_os="windows",not(_CONSOLE))))]
#[macro_export]
macro_rules! aloe_main_function {
    () => {
        /*
                int main (int argc, char* argv[])
        */
    }
}

#[cfg(not(all(target_os="windows",not(_CONSOLE))))]
#[macro_export]
macro_rules! aloe_main_function_args {
    () => {
        /*
                argc, (const char**) argv
        */
    }
}

///----------------------
#[cfg(target_os="ios")]
#[macro_export]
macro_rules! aloe_create_application_define {
    ($AppClass:ident) => {
        /*
        
            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wmissing-prototypes") 
            ALOEApplicationBase* aloe_CreateApplication() { return new AppClass(); } 
            void* aloe_GetIOSCustomDelegateClass()              { return nullptr; } 
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
}

#[cfg(target_os="ios")]
#[macro_export]
macro_rules! aloe_create_application_define_custom_delegate {
    ($AppClass:ident, $DelegateClass:ident) => {
        /*
        
            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wmissing-prototypes") 
            ALOEApplicationBase* aloe_CreateApplication() { return new AppClass(); } 
            void* aloe_GetIOSCustomDelegateClass()              { return [DelegateClass class]; } 
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
}

#[cfg(target_os="ios")]
#[macro_export]
macro_rules! aloe_main_function_definition {
    () => {
        /*
        
            extern "C" ALOE_MAIN_FUNCTION 
            { 
               ALOEApplicationBase::createInstance = &aloe_CreateApplication; 
               ALOEApplicationBase::iOSCustomDelegate = aloe_GetIOSCustomDelegateClass(); 
               return ALOEApplicationBase::main (ALOE_MAIN_FUNCTION_ARGS); 
            }
        */
    }
}

#[cfg(target_os="android")]
#[macro_export]
macro_rules! aloe_create_application_define {
    ($AppClass:ident) => {
        /*
        
            extern "C" ALOEApplicationBase* aloe_CreateApplication() { return new AppClass(); }
        */
    }
}

#[cfg(target_os="android")]
pub const ALOE_MAIN_FUNCTION_DEFINITION: bool = true;

#[cfg(not(any(target_os="ios",target_os="android")))]
#[macro_export]
macro_rules! aloe_create_application_define {
    ($AppClass:ident) => {
        /*
        
            ALOEApplicationBase* aloe_CreateApplication(); 
            ALOEApplicationBase* aloe_CreateApplication() { return new AppClass(); }
        */
    }
}

#[cfg(not(any(target_os="ios",target_os="android")))]
#[macro_export]
macro_rules! aloe_main_function_definition {
    () => {
        /*
        
            extern "C" ALOE_MAIN_FUNCTION 
            { 
               ALOEApplicationBase::createInstance = &aloe_CreateApplication; 
               return ALOEApplicationBase::main (ALOE_MAIN_FUNCTION_ARGS); 
            }
        */
    }
}

#[cfg(feature="aloe-plugin-build-standalone")]
#[cfg(feature="aloe-use-custom-plugin-standalone-app")]
#[macro_export]
macro_rules! start_aloe_application {
    ($AppClass:ident) => {
        /*
                ALOE_CREATE_APPLICATION_DEFINE(AppClass)
        */
    }
}

#[cfg(feature="aloe-plugin-build-standalone")]
#[cfg(not(feature="aloe-use-custom-plugin-standalone-app"))]
#[macro_export]
macro_rules! start_aloe_application {
    ($AppClass:ident) => {
        /*
                static_assert(false, "You are trying to use START_ALOE_APPLICATION in an audio plug-in. Define feature="aloe-use-custom-plugin-standalone-app"=1 if you want to use a custom standalone target app.");
        */
    }
}

#[cfg(not(feature = "aloe-plugin-build-standalone"))]
#[macro_export]
macro_rules! start_aloe_application {
    ($AppClass:expr) => {
        /*
        
             ALOE_CREATE_APPLICATION_DEFINE(AppClass) 
             ALOE_MAIN_FUNCTION_DEFINITION
        */
    }
}

#[cfg(feature="aloe-plugin-build-standalone")]
#[cfg(feature="aloe-use-custom-plugin-standalone-app")]
#[cfg(target_os="ios")]
#[macro_export]
macro_rules! start_aloe_application_with_custom_delegate {
    ($AppClass:ident, $DelegateClass:ident) => {
        /*
                ALOE_CREATE_APPLICATION_DEFINE_CUSTOM_DELEGATE(AppClass, DelegateClass)
        */
    }
}

#[cfg(feature="aloe-plugin-build-standalone")]
#[cfg(not(feature="aloe-use-custom-plugin-standalone-app"))]
#[cfg(target_os="ios")]
#[macro_export]
macro_rules! start_aloe_application_with_custom_delegate {
    ($AppClass:ident, $DelegateClass:ident) => {
        /*
                static_assert(false, "You are trying to use START_ALOE_APPLICATION in an audio plug-in. Define feature="aloe-use-custom-plugin-standalone-app"=1 if you want to use a custom standalone target app.");
        */
    }
}

/**
   | You can instruct Aloe to use a custom iOS
   | app delegate class instead of Aloe's
   | default app delegate. For Aloe to work you
   | must pass all messages to Aloe's internal
   | app delegate.  Below is an example of
   | minimal forwarding custom delegate. Note
   | that you are at your own risk if you decide
   | to use your own delegate and subtle, hard
   | to debug bugs may occur.
   |
   | @interface MyCustomDelegate : NSObject <UIApplicationDelegate> { NSObject<UIApplicationDelegate>* aloeDelegate; } @end
   |
   | @implementation MyCustomDelegate
   |
   | -(id) init
   | {
   |     self = [super init];
   |     aloeDelegate = reinterpret_cast<NSObject<UIApplicationDelegate>*> ([[NSClassFromString (@"AloeAppStartupDelegate") alloc] init]);
   |     return self;
   | }
   |
   | -(void) dealloc
   | {
   |     [aloeDelegate release];
   |     [super dealloc];
   | }
   |
   | - (void) forwardInvocation: (NSInvocation*) anInvocation
   | {
   |     if (aloeDelegate != nullptr && [aloeDelegate respondsToSelector: [anInvocation selector]])
   |         [anInvocation invokeWithTarget: aloeDelegate];
   |     else
   |         [super forwardInvocation: anInvocation];
   | }
   |
   | -(BOOL) respondsToSelector: (SEL) aSelector
   | {
   |     if (aloeDelegate != nullptr && [aloeDelegate respondsToSelector: aSelector])
   |         return YES;
   |
   |     return [super respondsToSelector: aSelector];
   | }
   | @end
*/
#[cfg(not(feature="aloe-plugin-build-standalone"))]
#[cfg(target_os="ios")]
#[macro_export]
macro_rules! start_aloe_application_with_custom_delegate {
    ($AppClass:ident, 
     $DelegateClass:ident) => {
        /*
        
              ALOE_CREATE_APPLICATION_DEFINE_CUSTOM_DELEGATE(AppClass, DelegateClass) 
              ALOE_MAIN_FUNCTION_DEFINITION
        */
    }
}
