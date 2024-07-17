crate::ix!();

pub struct RectangleFillTypesDemo<'a> {
    base:    GraphicsDemoBase<'a>,
    colour1: Colour, // default = Colours::red 
    colour2: Colour, // default = Colours::green 
}

impl<'a> RectangleFillTypesDemo<'a> {

    pub fn new(cc: &mut ControllersComponent) -> Self {
    
        todo!();
        /*
        : graphics_demo_base(cc, "Fill Types: Rectangles"),

        
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.addTransform (getTransform());

            const int rectSize = jmin (getWidth(), getHeight()) / 2 - 20;

            g.setColour (colour1.withAlpha (getAlpha()));
            g.fillRect (-rectSize, -rectSize, rectSize, rectSize);

            g.setGradientFill (ColourGradient (colour1, 10.0f, (float) -rectSize,
                                               colour2, 10.0f + (float) rectSize, 0.0f, false));
            g.setOpacity (getAlpha());
            g.fillRect (10, -rectSize, rectSize, rectSize);

            g.setGradientFill (ColourGradient (colour1, (float) rectSize * -0.5f, 10.0f + (float) rectSize * 0.5f,
                                               colour2, 0, 10.0f + (float) rectSize, true));
            g.setOpacity (getAlpha());
            g.fillRect (-rectSize, 10, rectSize, rectSize);

            g.setGradientFill (ColourGradient (colour1, 10.0f, 10.0f,
                                               colour2, 10.0f + (float) rectSize, 10.0f + (float) rectSize, false));
            g.setOpacity (getAlpha());
            g.drawRect (10, 10, rectSize, rectSize, 5);
        */
    }
}
