crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/HelloWorldDemo.h]

#[no_copy]
#[leak_detector]
pub struct HelloWorldDemo<'a> {
    base:              Component<'a>,
    hello_world_label: Label<'a>, // default = TRANS("Hello World!") 
    quit_button:       TextButton<'a>, // default = TRANS("Quit") 
    internal_path:     Path,
}

impl<'a> Default for HelloWorldDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (helloWorldLabel);

            helloWorldLabel.setFont (Font (40.00f, Font::bold));
            helloWorldLabel.setJustificationType (Justification::centred);
            helloWorldLabel.setEditable (false, false, false);
            helloWorldLabel.setColour (Label::textColourId, Colours::black);
            helloWorldLabel.setColour (TextEditor::textColourId, Colours::black);
            helloWorldLabel.setColour (TextEditor::backgroundColourId, Colour (0x00000000));

            addAndMakeVisible (quitButton);
            quitButton.onClick = [] { ALOEApplication::quit(); };

            setSize (600, 300)
        */
    }
}

impl<'a> Paint for HelloWorldDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colour (0xffc1d0ff));

            g.setColour (Colours::white);
            g.fillPath (internalPath);

            g.setColour (Colour (0xff6f6f6f));
            g.strokePath (internalPath, PathStrokeType (5.200f));
        */
    }
}

impl<'a> Resized for HelloWorldDemo<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            helloWorldLabel.setBounds (152, 80, 296, 48);
            quitButton.setBounds (getWidth() - 176, getHeight() - 60, 120, 32);

            internalPath.clear();
            internalPath.startNewSubPath (136.0f, 80.0f);
            internalPath.quadraticTo (176.0f, 24.0f, 328.0f, 32.0f);
            internalPath.quadraticTo (472.0f, 40.0f, 472.0f, 104.0f);
            internalPath.quadraticTo (472.0f, 192.0f, 232.0f, 176.0f);
            internalPath.lineTo (184.0f, 216.0f);
            internalPath.lineTo (200.0f, 168.0f);
            internalPath.quadraticTo (96.0f, 136.0f, 136.0f, 80.0f);
            internalPath.closeSubPath();
        */
    }
}
