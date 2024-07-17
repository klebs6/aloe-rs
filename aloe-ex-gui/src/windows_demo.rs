crate::ix!();

#[no_copy]
#[leak_detector]
pub struct WindowsDemo<'a> {
    base:                 Component<'a>,

    /**
      | Because in this demo the windows delete
      | themselves, we'll use the
      | Component::SafePointer class to point to
      | them, which automatically becomes null when
      | the component that it points to is deleted.
      */
    windows:              Vec<ComponentSafePointer<'a,Component<'a>>>,
    dialog_window:        ComponentSafePointer<'a, DialogWindow<'a>>,
    show_windows_button:  TextButton<'a>, // default = { "Show Windows"  }
    close_windows_button: TextButton<'a>, // default = { "Close Windows"  }
}

pub enum WindowsDemoWindows
{
    dialog,
    document,
    alert,
    numWindows
}

impl<'a> Default for WindowsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            addAndMakeVisible (showWindowsButton);
            showWindowsButton.onClick = [this] { showAllWindows(); };

            addAndMakeVisible (closeWindowsButton);
            closeWindowsButton.onClick = [this] { closeAllWindows(); };

            setSize (250, 250)
        */
    }
}

impl<'a> Drop for WindowsDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            if (dialogWindow != nullptr)
            {
                dialogWindow->exitModalState (0);

                // we are shutting down: can't wait for the message manager
                // to eventually delete this
                delete dialogWindow;
            }

            closeAllWindows();
         */
    }
}

impl<'a> WindowsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colours::grey));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Rectangle<int> buttonSize (0, 0, 108, 28);

            Rectangle<int> area ((getWidth()  / 2) - (buttonSize.getWidth() / 2),
                                 (getHeight() / 2) -  buttonSize.getHeight(),
                                 buttonSize.getWidth(), buttonSize.getHeight());

            showWindowsButton .setBounds (area.reduced (2));
            closeWindowsButton.setBounds (area.translated (0, buttonSize.getHeight()).reduced (2));
        */
    }
    
    pub fn show_all_windows(&mut self)  {
        
        todo!();
        /*
            closeAllWindows();

            showDocumentWindow (false);
            showDocumentWindow (true);
            showTransparentWindow();
            showDialogWindow();
        */
    }
    
    pub fn close_all_windows(&mut self)  {
        
        todo!();
        /*
            for (auto& window : windows)
                window.deleteAndZero();

            windows.clear();
        */
    }
    
    pub fn show_dialog_window(&mut self)  {
        
        todo!();
        /*
            String m;

            m << "Dialog Windows can be used to quickly show a component, usually blocking mouse input to other windows." << newLine
              << newLine
              << "They can also be quickly closed with the escape key, try it now.";

            DialogWindowLaunchOptions options;
            auto* label = new Label();
            label->setText (m, dontSendNotification);
            label->setColour (Label::textColourId, Colours::whitesmoke);
            options.content.setOwned (label);

            Rectangle<int> area (0, 0, 300, 200);

            options.content->setSize (area.getWidth(), area.getHeight());

            options.dialogTitle                   = "Dialog Window";
            options.dialogBackgroundColour        = Colour (0xff0e345a);
            options.escapeKeyTriggersCloseButton  = true;
            options.useNativeTitleBar             = false;
            options.resizable                     = true;

            dialogWindow = options.launchAsync();

            if (dialogWindow != nullptr)
                dialogWindow->centreWithSize (300, 200);
        */
    }
    
    pub fn show_document_window(&mut self, native: bool)  {
        
        todo!();
        /*
            auto* dw = new ColourSelectorWindow ("Document Window", getRandomBrightColour(), DocumentWindow::allButtons);
            windows.add (dw);

            Rectangle<int> area (0, 0, 300, 400);

            RectanglePlacement placement ((native ? RectanglePlacement::xLeft
                                                  : RectanglePlacement::xRight)
                                           | RectanglePlacement::yTop
                                           | RectanglePlacement::doNotResize);

            auto result = placement.appliedTo (area, Desktop::getInstance().getDisplays()
                                                             .getPrimaryDisplay()->userArea.reduced (20));
            dw->setBounds (result);

            dw->setResizable (true, ! native);
            dw->setUsingNativeTitleBar (native);
            dw->setVisible (true);
        */
    }
    
    pub fn show_transparent_window(&mut self)  {
        
        todo!();
        /*
            auto* balls = new BouncingBallsContainer (3);
            balls->addToDesktop (ComponentPeer::windowIsTemporary);
            windows.add (balls);

            Rectangle<int> area (0, 0, 200, 200);

            RectanglePlacement placement (RectanglePlacement::xLeft
                                           | RectanglePlacement::yBottom
                                           | RectanglePlacement::doNotResize);

            auto result = placement.appliedTo (area, Desktop::getInstance().getDisplays()
                                                             .getPrimaryDisplay()->userArea.reduced (20));
            balls->setBounds (result);

            balls->setVisible (true);
        */
    }
}
