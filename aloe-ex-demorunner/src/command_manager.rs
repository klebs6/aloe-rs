crate::ix!();

lazy_static!{
    /*
    std::unique_ptr<AudioDeviceManager> sharedAudioDeviceManager;
    */
}

pub fn get_global_command_manager<'a>() -> &'a mut ApplicationCommandManager<'a> {
    
    todo!();
    /*
        return dynamic_cast<DemoRunnerApplication*> (ALOEApplication::getInstance())->getGlobalCommandManager();
    */
}

/**
  | This macro generates the main() routine
  | that launches the app.
  |
  */
start_aloe_application!{
    DemoRunnerApplication
}

