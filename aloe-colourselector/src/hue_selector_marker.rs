crate::ix!();

pub struct HueSelectorMarker<'a> {
    base: Component<'a>,
}

impl<'a> Default for HueSelectorMarker<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setInterceptsMouseClicks (false, false);
        */
    }
}

impl<'a> HueSelectorMarker<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto cw = (float) getWidth();
                auto ch = (float) getHeight();

                Path p;
                p.addTriangle (1.0f, 1.0f,
                               cw * 0.3f, ch * 0.5f,
                               1.0f, ch - 1.0f);

                p.addTriangle (cw - 1.0f, 1.0f,
                               cw * 0.7f, ch * 0.5f,
                               cw - 1.0f, ch - 1.0f);

                g.setColour (Colours::white.withAlpha (0.75f));
                g.fillPath (p);

                g.setColour (Colours::black.withAlpha (0.75f));
                g.strokePath (p, PathStrokeType (1.2f));
        */
    }
}
