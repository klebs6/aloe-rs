crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ChildProcessDemo<'a> {
    base:             Component<'a>,
    base2:            MessageListener,
    master_process:   Box<DemoMasterProcess<'a>>,
    launch_button:    TextButton<'a>, // default = { "Launch Child Process"  }
    ping_button:      TextButton<'a>, // default = { "Send Ping"  }
    kill_button:      TextButton<'a>, // default = { "Kill Child Process"  }
    test_results_box: TextEditor<'a>,
}

impl<'a> Default for ChildProcessDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*

            setOpaque (true);

            addAndMakeVisible (launchButton);
            launchButton.onClick = [this] { launchChildProcess(); };

            addAndMakeVisible (pingButton);
            pingButton.onClick = [this] { pingChildProcess(); };

            addAndMakeVisible (killButton);
            killButton.onClick = [this] { killChildProcess(); };

            addAndMakeVisible (testResultsBox);
            testResultsBox.setMultiLine (true);
            testResultsBox.setFont ({ Font::getDefaultMonospacedFontName(), 12.0f, Font::plain });

            logMessage (String ("This demo uses the ChildProcessMaster and ChildProcessSlave classes to launch and communicate "
                                "with a child process, sending messages in the form of serialised ValueTree objects.") + newLine);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for ChildProcessDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            masterProcess.reset();
         */
    }
}

impl<'a> ChildProcessDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            auto top = area.removeFromTop (40);
            launchButton.setBounds (top.removeFromLeft (180).reduced (8));
            pingButton  .setBounds (top.removeFromLeft (180).reduced (8));
            killButton  .setBounds (top.removeFromLeft (180).reduced (8));

            testResultsBox.setBounds (area.reduced (8));
        */
    }

    /**
      | Appends a message to the textbox that's
      | shown in the demo as the console
      |
      */
    pub fn log_message(&mut self, message: &String)  {
        
        todo!();
        /*
            postMessage (new LogMessage (message));
        */
    }

    /**
      | invoked by the 'launch' button.
      |
      */
    pub fn launch_child_process(&mut self)  {
        
        todo!();
        /*
            if (masterProcess.get() == nullptr)
            {
                masterProcess.reset (new DemoMasterProcess (*this));

                if (masterProcess->launchSlaveProcess (File::getSpecialLocation (File::currentExecutableFile), demoCommandLineUID))
                    logMessage ("Child process started");
            }
        */
    }

    /**
      | invoked by the 'ping' button.
      |
      */
    pub fn ping_child_process(&mut self)  {
        
        todo!();
        /*
            if (masterProcess.get() != nullptr)
                masterProcess->sendPingMessageToSlave();
            else
                logMessage ("Child process is not running!");
        */
    }

    /**
      | invoked by the 'kill' button.
      |
      */
    pub fn kill_child_process(&mut self)  {
        
        todo!();
        /*
            if (masterProcess.get() != nullptr)
            {
                masterProcess.reset();
                logMessage ("Child process killed");
            }
        */
    }

    pub fn handle_message(&mut self, message: &Message)  {
        
        todo!();
        /*
            testResultsBox.moveCaretToEnd();
            testResultsBox.insertTextAtCaret (static_cast<const LogMessage&> (message).message + newLine);
            testResultsBox.moveCaretToEnd();
        */
    }

    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            testResultsBox.applyFontToAllText (testResultsBox.getFont());
        */
    }
}

/**
  | The ALOEApplication::initialise
  | method calls this function to allow
  | the child process to launch when the
  | command line parameters indicate that
  | we're being asked to run as a child process..
  |
  */
pub fn invoke_child_process_demo(command_line: &String) -> bool {
    
    todo!();
    /*
        std::unique_ptr<DemoSlaveProcess> slave (new DemoSlaveProcess());

        if (slave->initialiseFromCommandLine (commandLine, demoCommandLineUID))
        {
            slave.release(); // allow the slave object to stay alive - it'll handle its own deletion.
            return true;
        }

        return false;
    */
}
