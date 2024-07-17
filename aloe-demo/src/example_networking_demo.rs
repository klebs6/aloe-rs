crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/NetworkingDemo.h]

#[no_copy]
#[leak_detector]
pub struct NetworkingDemo<'a> {
    base:             Component<'a>,
    base2:            Thread,
    url_box:          TextEditor<'a>,
    fetch_button:     TextButton<'a>,          // default = { "Download Url Contents"  }
    results_document: CodeDocument<'a>,
    results_box:      CodeEditorComponent<'a>, //{ resultsDocument, nullptr };
}

impl<'a> Default for NetworkingDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : thread("Network Demo"),

            setOpaque (true);

            addAndMakeVisible (urlBox);
            urlBox.setText ("https://www.google.com");
            urlBox.onReturnKey = [this] { fetchButton.triggerClick(); };

            addAndMakeVisible (fetchButton);
            fetchButton.onClick = [this] { startThread(); };

            addAndMakeVisible (resultsBox);

            setSize (500, 500)
        */
    }
}

impl<'a> NetworkingDemo<'a> {

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

            {
                auto topArea = area.removeFromTop (40);
                fetchButton.setBounds (topArea.removeFromRight (180).reduced (8));
                urlBox     .setBounds (topArea.reduced (8));
            }

            resultsBox.setBounds (area.reduced (8));
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            auto result = getResultText (urlBox.getText());

            MessageManagerLock mml (this);

            if (mml.lockWasGained())
                resultsBox.loadContent (result);
        */
    }
    
    pub fn get_result_text(&mut self, url: &Url) -> String {
        
        todo!();
        /*
            StringPairArray responseHeaders;
            int statusCode = 0;

            if (auto stream = std::unique_ptr<InputStream> (url.createInputStream (false, nullptr, nullptr, {},
                                                                                   10000, // timeout in millisecs
                                                                                   &responseHeaders, &statusCode)))
            {
                return (statusCode != 0 ? "Status code: " + String (statusCode) + newLine : String())
                        + "Response headers: " + newLine
                        + responseHeaders.getDescription() + newLine
                        + "----------------------------------------------------" + newLine
                        + stream->readEntireStreamAsString();
            }

            if (statusCode != 0)
                return "Failed to connect, status code = " + String (statusCode);

            return "Failed to connect!";
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            urlBox.applyFontToAllText (urlBox.getFont());
        */
    }
}
