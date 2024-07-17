crate::ix!();

#[derive(Default)]
pub struct DemoRunnerApplication<'a> {
    base:            ALOEApplication,
    main_window:     Box<MainAppWindow<'a>>,
    command_manager: ApplicationCommandManager<'a>,
}

impl<'a> Drop for DemoRunnerApplication<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            sharedAudioDeviceManager.reset();
         */
    }
}

impl<'a> DemoRunnerApplication<'a> {

    pub fn get_application_name(&mut self) -> String {
        
        todo!();
        /*
            return ProjectInfo::projectName;
        */
    }
    
    pub fn get_application_version(&mut self) -> String {
        
        todo!();
        /*
            return ProjectInfo::versionString;
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
            registerAllDemos();

          #if ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
            // (This function call is for one of the demos, which involves launching a child process)
            if (invokeChildProcessDemo (commandLine))
                return;
          #else
            ignoreUnused (commandLine);
          #endif

            mainWindow.reset (new MainAppWindow (getApplicationName()));
        */
    }
    
    pub fn back_button_pressed(&mut self) -> bool {
        
        todo!();
        /*
            mainWindow->getMainComponent().getSidePanel().showOrHide (false); return true;
        */
    }
    
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            mainWindow = nullptr;
        */
    }
    
    pub fn system_requested_quit(&mut self)  {
        
        todo!();
        /*
            quit();
        */
    }
    
    pub fn another_instance_started(&mut self, _0: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_global_command_manager(&mut self) -> &mut ApplicationCommandManager {
        
        todo!();
        /*
            return commandManager;
        */
    }
}
