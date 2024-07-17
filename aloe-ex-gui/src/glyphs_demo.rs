crate::ix!();

pub struct GlyphsDemo<'a> {
    base:   GraphicsDemoBase<'a>,
    glyphs: GlyphArrangement,
}

impl<'a> GlyphsDemo<'a> {

    pub fn new(cc: &mut ControllersComponent) -> Self {
    
        todo!();
        /*
            : GraphicsDemoBase (cc, "Glyphs")

            glyphs.addFittedText ({ 20.0f }, "The Quick Brown Fox Jumped Over The Lazy Dog",
                                  -120, -50, 240, 100, Justification::centred, 2, 1.0f);
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::black.withAlpha (getAlpha()));
            glyphs.draw (g, getTransform());
        */
    }
}
