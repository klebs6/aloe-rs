crate::ix!();

/**
  | This class implements the desktop window
  | that contains an instance of our MainComponent
  | class.
  |
  */
#[no_copy]
#[leak_detector]
pub struct MainWindow<'a> {
    base: DocumentWindow<'a>,
}

impl<'a> Default for MainWindow<'a> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a> MainWindow<'a> {

    /**
      | -----------
      | @note
      | 
      | Be careful if you override any
      | 
      | DocumentWindow methods - the base class
      | uses a lot of them, so by overriding you
      | might break its functionality. It's
      | best to do all your work in your content
      | component instead, but if you really
      | have to override any DocumentWindow
      | methods, make sure your subclass also
      | calls the superclass's method.
      |
      */
    pub fn new(name: String) -> Self {
    
        todo!();
        /*


            : DocumentWindow (name,
                                  Desktop::getInstance().getDefaultLookAndFeel()
                                                              .findColour (ResizableWindow::backgroundColourId),
                                  DocumentWindow::allButtons)

                setUsingNativeTitleBar (true);
                setContentOwned (new MainComponent(), true);

               #if ALOE_IOS || ALOE_ANDROID
                setFullScreen (true);
               #else
                setResizable (true, true);
                centreWithSize (getWidth(), getHeight());
               #endif

                setVisible (true);
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            // This is called when the user tries to close this window. Here, we'll just
                // ask the app to quit when this happens, but you can change this to do
                // whatever you need.
                ALOEApplication::getInstance()->systemRequestedQuit();
        */
    }
}
