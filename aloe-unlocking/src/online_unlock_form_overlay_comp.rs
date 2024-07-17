crate::ix!();

#[leak_detector]
pub struct OnlineUnlockFormOverlayComp<'a> {
    base:          Component<'a>,
    base2:         Thread,
    base3:         Timer,
    form:          &'a mut OnlineUnlockForm<'a>,
    spinner:       Spinner<'a>,
    result:        OnlineUnlockStatusUnlockResult,
    email:         String,
    password:      String,
    cancel_button: Box<TextButton<'a>>,
}

impl<'a> ButtonListener for OnlineUnlockFormOverlayComp<'a> {

    fn button_clicked(&mut self, button: *mut Button)  {
        
        todo!();
        /*
            if (button == cancelButton.get())
                {
                    form.status.userCancelled();

                    spinner.setVisible (false);
                    stopTimer();

                    delete this;
                }
        */
    }
}

impl<'a> Drop for OnlineUnlockFormOverlayComp<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            stopThread (10000);
        */
    }
}

impl<'a> OnlineUnlockFormOverlayComp<'a> {

    pub fn new(
        f:                 &mut OnlineUnlockForm,
        has_cancel_button: Option<bool>

    ) -> Self {

        let has_cancel_button: bool =
                 has_cancel_button.unwrap_or(false);
        todo!();
        /*
        : thread(String()),
        : form(f),

            result.succeeded = false;
                email = form.emailBox.getText();
                password = form.passwordBox.getText();
                addAndMakeVisible (spinner);

                if (hasCancelButton)
                {
                    cancelButton.reset (new TextButton (TRANS ("Cancel")));
                    addAndMakeVisible (cancelButton.get());
                    cancelButton->addListener (this);
                }

                startThread (4);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::white.withAlpha (0.97f));

                g.setColour (Colours::black);
                g.setFont (15.0f);

                g.drawFittedText (TRANS("Contacting XYZ...").replace ("XYZ", form.status.getWebsiteName()),
                getLocalBounds().reduced (20, 0).removeFromTop (proportionOfHeight (0.6f)),
                Justification::centred, 5);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            const int spinnerSize = 40;
                spinner.setBounds ((getWidth() - spinnerSize) / 2, proportionOfHeight (0.6f), spinnerSize, spinnerSize);

                if (cancelButton != nullptr)
                    cancelButton->setBounds (getLocalBounds().removeFromBottom (50).reduced (getWidth() / 4, 5));
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            result = form.status.attemptWebserverUnlock (email, password);
                startTimer (100);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            spinner.setVisible (false);
                stopTimer();

                if (result.errorMessage.isNotEmpty())
                {
                    AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                        TRANS("Registration Failed"),
                        result.errorMessage);
                }
                else if (result.informativeMessage.isNotEmpty())
                {
                    AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon,
                        TRANS("Registration Complete!"),
                        result.informativeMessage);
                }
                else if (result.urlToLaunch.isNotEmpty())
                {
                    Url url (result.urlToLaunch);
                    url.launchInDefaultBrowser();
                }

                // (local copies because we're about to delete this)
                const bool worked = result.succeeded;
                OnlineUnlockForm& f = form;

                delete this;

                if (worked)
                    f.dismiss();
        */
    }
}
