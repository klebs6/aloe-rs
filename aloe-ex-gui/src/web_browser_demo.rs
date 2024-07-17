crate::ix!();

#[no_copy]
#[leak_detector]
pub struct WebBrowserDemo<'a> {
    base:             Component<'a>,
    web_view:         Box<DemoBrowserComponent<'a>>,
    address_text_box: TextEditor<'a>,
    go_button:        TextButton<'a>, // default = { "Go", "Go to Url"  }
    back_button:      TextButton<'a>, // default = { "<<", "Back"  }
    forward_button:   TextButton<'a>, // default = { ">>", "Forward"  }
}

impl<'a> Default for WebBrowserDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            // Create an address box..
            addAndMakeVisible (addressTextBox);
            addressTextBox.setTextToShowWhenEmpty ("Enter a web address, e.g. https://www.aloe.com", Colours::grey);
            addressTextBox.onReturnKey = [this] { webView->goToURL (addressTextBox.getText()); };

            // create the actual browser component
            webView.reset (new DemoBrowserComponent (addressTextBox));
            addAndMakeVisible (webView.get());

            // add some buttons..
            addAndMakeVisible (goButton);
            goButton.onClick = [this] { webView->goToURL (addressTextBox.getText()); };
            addAndMakeVisible (backButton);
            backButton.onClick = [this] { webView->goBack(); };
            addAndMakeVisible (forwardButton);
            forwardButton.onClick = [this] { webView->goForward(); };

            // send the browser to a start page..
            webView->goToURL ("https://www.aloe.com");

            setSize (1000, 1000)
        */
    }
}

impl<'a> WebBrowserDemo<'a> {

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
            webView->setBounds       (10, 45, getWidth() - 20, getHeight() - 55);
            goButton      .setBounds (getWidth() - 45, 10, 35, 25);
            addressTextBox.setBounds (100, 10, getWidth() - 155, 25);
            backButton    .setBounds (10, 10, 35, 25);
            forwardButton .setBounds (55, 10, 35, 25);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            addressTextBox.applyFontToAllText (addressTextBox.getFont());
        */
    }
}
