crate::ix!();

#[no_copy]
#[leak_detector]
#[cfg(not(ALOE_DEMO_RUNNER))]
pub struct MainWindow<'a> {
    base: DocumentWindow<'a>,
}

#[cfg(not(ALOE_DEMO_RUNNER))]
impl<'a> MainWindow<'a> {

    pub fn new(
        name: &String,
        c:    *mut Component) -> Self {
    
        todo!();
        /*

            : DocumentWindow (name,
                                                                              Desktop::getInstance().getDefaultLookAndFeel()
                                                                                                    .findColour (ResizableWindow::backgroundColourId),
                                                                              DocumentWindow::allButtons)

                 setUsingNativeTitleBar (true);
                 setContentOwned (c, true);

                 centreWithSize (getWidth(), getHeight());

                 setVisible (true);
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            ALOEApplication::getInstance()->systemRequestedQuit();
        */
    }
}
