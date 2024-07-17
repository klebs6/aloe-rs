crate::ix!();
 
/**
  | As we need to modify the
  | ALOEApplication::initialise method to launch
  | the child process based on the command line
  | parameters, we can't just use the normal
  | auto-generated Main.cpp.
  |
  | Instead, we don't do anything in Main.cpp and
  | create a ALOEApplication subclass here with
  | the necessary modifications.
  */
#[cfg(not(ALOE_DEMO_RUNNER))]
#[derive(Default)]
pub struct Application<'a> {
    base:        ALOEApplication,
    main_window: Option<Box<MainWindow<'a>>>,
}

#[cfg(not(ALOE_DEMO_RUNNER))]
start_aloe_application!{ Application }

#[cfg(not(ALOE_DEMO_RUNNER))]
impl<'a> Application<'a> {

    pub fn get_application_name(&mut self) -> String {
        
        todo!();
        /*
            return "ChildProcessDemo";
        */
    }

    pub fn get_application_version(&mut self) -> String {
        
        todo!();
        /*
            return "1.0.0";
        */
    }

    pub fn initialise(&mut self, command_line: &String)  {
        
        todo!();
        /*
            // launches the child process if the command line parameters contain the demo UID
             if (invokeChildProcessDemo (commandLine))
                 return;

             mainWindow.reset (new MainWindow ("ChildProcessDemo", new ChildProcessDemo()));
        */
    }

    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            mainWindow = nullptr;
        */
    }
}
