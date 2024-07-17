crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/LiveConstantDemo.h]

#[derive(Default)]
pub struct LiveConstantDemoComponent<'a> {
    base: Component<'a>,
}

impl<'a> LiveConstantDemoComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (ALOE_LIVE_CONSTANT (Colour (0xffe5e7a7)));

            g.setColour (ALOE_LIVE_CONSTANT (Colours::red.withAlpha (0.2f)));
            auto blockWidth  = ALOE_LIVE_CONSTANT (0x120);
            auto blockHeight = ALOE_LIVE_CONSTANT (200);
            g.fillRect ((getWidth() - blockWidth) / 2, (getHeight() - blockHeight) / 2, blockWidth, blockHeight);

            auto fontColour = ALOE_LIVE_CONSTANT (Colour (0xff000a55));
            auto fontSize   = ALOE_LIVE_CONSTANT (30.0f);

            g.setColour (fontColour);
            g.setFont (fontSize);

            g.drawFittedText (getDemoText(), getLocalBounds(), Justification::centred, 2);
        */
    }
    
    pub fn get_demo_text() -> String {
        
        todo!();
        /*
            return ALOE_LIVE_CONSTANT ("Hello world!");
        */
    }
}
