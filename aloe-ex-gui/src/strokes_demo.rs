crate::ix!();

pub struct StrokesDemo<'a> {
    base:      GraphicsDemoBase<'a>,
    points:    [SlowerBouncingNumber; 2 + 4 * 8],
    thickness: [SlowerBouncingNumber; 2 + 4 * 8],
}

impl<'a> StrokesDemo<'a> {

    pub fn new(cc: &mut ControllersComponent) -> Self {
    
        todo!();
        /*


            : GraphicsDemoBase (cc, "Paths: Stroked")
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto w = (float) getWidth();
            auto h = (float) getHeight();

            Path p;
            p.startNewSubPath (points[0].getValue() * w,
                               points[1].getValue() * h);

            for (int i = 2; i < numElementsInArray (points); i += 4)
                p.quadraticTo (points[i]    .getValue() * w,
                               points[i + 1].getValue() * h,
                               points[i + 2].getValue() * w,
                               points[i + 3].getValue() * h);

            p.closeSubPath();

            PathStrokeType stroke (0.5f + 10.0f * thickness.getValue());
            g.setColour (Colours::purple.withAlpha (getAlpha()));
            g.strokePath (p, stroke, AffineTransform());
        */
    }
}
