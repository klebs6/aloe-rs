crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/DialogsDemo.h]

pub struct DemoBackgroundThread<'a> {
    base: ThreadWithProgressWindow<'a>,
}

impl<'a> Default for DemoBackgroundThread<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : ThreadWithProgressWindow ("busy doing some important things...", true, true)

            setStatusMessage ("Getting ready...")
        */
    }
}

impl<'a> DemoBackgroundThread<'a> {

    pub fn run(&mut self)  {
        
        todo!();
        /*
            setProgress (-1.0); // setting a value beyond the range 0 -> 1 will show a spinning bar..
            setStatusMessage ("Preparing to do some stuff...");
            wait (2000);

            int thingsToDo = 10;

            for (int i = 0; i < thingsToDo; ++i)
            {
                // must check this as often as possible, because this is
                // how we know if the user's pressed 'cancel'
                if (threadShouldExit())
                    return;

                // this will update the progress bar on the dialog box
                setProgress (i / (double) thingsToDo);

                setStatusMessage (String (thingsToDo - i) + " things left to do...");

                wait (500);
            }

            setProgress (-1.0); // setting a value beyond the range 0 -> 1 will show a spinning bar..
            setStatusMessage ("Finishing off the last few bits and pieces!");
            wait (2000);
        */
    }

    /**
      | This method gets called on the message
      | thread once our thread has finished..
      |
      */
    pub fn thread_complete(&mut self, user_pressed_cancel: bool)  {
        
        todo!();
        /*
            const String messageString (userPressedCancel ? "You pressed cancel!" : "Thread finished ok!");

            AlertWindow::showAsync (MessageBoxOptions()
                                      .withIconType (MessageBoxIconType::InfoIcon)
                                      .withTitle ("Progress window")
                                      .withMessage (messageString)
                                      .withButton ("OK"),
                                    nullptr);

            // ..and clean up by deleting our thread object..
            delete this;
        */
    }
}
