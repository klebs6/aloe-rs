crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/CMake/GuiApp/Main.cpp]

#[derive(Default)]
pub struct GuiAppApplication<'a> {
    base:        ALOEApplication,
    main_window: Box<MainWindow<'a>>,
}

/**
  | This macro generates the main() routine
  | that launches the app.
  |
  */
start_aloe_application!{
    GuiAppApplication
}

impl<'a> GuiAppApplication<'a> {

    /**
      | We inject these as compile definitions from
      | the CMakeLists.txt
      |
      | If you've enabled the aloe header with
      | `aloe_generate_aloe_header(<thisTarget>)`
      | you could `#include <AloeHeader.h>` and
      | use `ProjectInfo::projectName`
      | etc. instead.
      */
    pub fn get_application_name(&mut self) -> String {
        
        todo!();
        /*
            return ALOE_APPLICATION_NAME_STRING;
        */
    }
    
    pub fn get_application_version(&mut self) -> String {
        
        todo!();
        /*
            return ALOE_APPLICATION_VERSION_STRING;
        */
    }
    
    pub fn more_than_one_instance_allowed(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn initialise(&mut self, command_line: &String)  {
        
        todo!();
        /*
            // This method is where you should put your application's initialisation code..
            ignoreUnused (commandLine);

            mainWindow.reset (new MainWindow (getApplicationName()));
        */
    }
    
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            // Add your application's shutdown code here..

            mainWindow = nullptr; // (deletes our window)
        */
    }
    
    pub fn system_requested_quit(&mut self)  {
        
        todo!();
        /*
            // This is called when the app is being asked to quit: you can ignore this
            // request and let the app carry on running, or call quit() to allow the app to close.
            quit();
        */
    }
    
    pub fn another_instance_started(&mut self, command_line: &String)  {
        
        todo!();
        /*
            // When another instance of the app is launched while this one is running,
            // this method is invoked, and the commandLine parameter tells you what
            // the other instance's command-line arguments were.
            ignoreUnused (commandLine);
        */
    }
}
