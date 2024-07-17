crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MainAppWindow<'a> {
    base:         DocumentWindow<'a>,
    taskbar_icon: Box<Component<'a>>,
}

impl<'a> Default for MainAppWindow<'a> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a> MainAppWindow<'a> {

    pub fn new(name: &String) -> Self {
    
        todo!();
        /*


            : DocumentWindow (name, Desktop::getInstance().getDefaultLookAndFeel()
                                                              .findColour (ResizableWindow::backgroundColourId),
                                  DocumentWindow::allButtons)

                setUsingNativeTitleBar (true);
                setResizable (true, false);
                setResizeLimits (400, 400, 10000, 10000);

               #if ALOE_IOS || ALOE_ANDROID
                setFullScreen (true);

                auto& desktop = Desktop::getInstance();

                desktop.setOrientationsEnabled (Desktop::allOrientations);
                desktop.setKioskModeComponent (this);
               #else
                setBounds ((int) (0.1f * (float) getParentWidth()),
                           (int) (0.1f * (float) getParentHeight()),
                           jmax (850, (int) (0.5f * (float) getParentWidth())),
                           jmax (600, (int) (0.7f * (float) getParentHeight())));
               #endif

                setContentOwned (new MainComponent(), false);
                setVisible (true);

               #if ALOE_MAC || ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
                taskbarIcon.reset (new DemoTaskbarComponent());
               #endif
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            ALOEApplication::getInstance()->systemRequestedQuit();
        */
    }
    
    pub fn get_main_component(&mut self) -> &mut MainComponent {
        
        todo!();
        /*
            return *dynamic_cast<MainComponent*> (getContentComponent());
        */
    }
}
