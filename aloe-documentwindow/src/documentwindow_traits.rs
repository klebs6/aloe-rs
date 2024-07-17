crate::ix!();

pub trait DocumentWindowInterface: 
CloseButtonPressed
+ MinimiseButtonPressed
+ MaximiseButtonPressed {}

pub trait CloseButtonPressed {

    /**
      | This method is called when the user tries
      | to close the window.
      | 
      | This is triggered by the user clicking
      | the close button, or using some other
      | 
      | OS-specific key shortcut or OS menu
      | for getting rid of a window.
      | 
      | If the window is just a pop-up, you should
      | override this closeButtonPressed()
      | method and make it delete the window
      | in whatever way is appropriate for your
      | app. E.g. you might just want to call
      | "delete this".
      | 
      | If your app is centred around this window
      | such that the whole app should quit when
      | the window is closed, then you will probably
      | want to use this method as an opportunity
      | to call ALOEApplicationBase::quit(),
      | and leave the window to be deleted later
      | by your
      | 
      | ALOEApplicationBase::shutdown()
      | method. (Doing it this way means that
      | your window will still get cleaned-up
      | if the app is quit by some other means
      | (e.g. a cmd-Q on the mac or closing it
      | via the taskbar icon on Windows).
      | 
      | (Note that the DocumentWindow class
      | overrides Component::userTriedToCloseWindow()
      | and redirects it to call this method,
      | so any methods of closing the window
      | that are caught by userTriedToCloseWindow()
      | will also end up here).
      |
      */
    fn close_button_pressed(&mut self);
}

pub trait MinimiseButtonPressed {

    /**
      | Callback that is triggered when the
      | minimise button is pressed.
      | 
      | The default implementation of this
      | calls ResizableWindow::setMinimised(),
      | but you can override it to do more customised
      | behaviour.
      |
      */
    fn minimise_button_pressed(&mut self);
}

pub trait MaximiseButtonPressed {

    /**
      | Callback that is triggered when the
      | maximise button is pressed, or when
      | the title-bar is double-clicked.
      | 
      | The default implementation of this
      | calls ResizableWindow::setFullScreen(),
      | but you can override it to do more customised
      | behaviour.
      |
      */
    fn maximise_button_pressed(&mut self);
}
