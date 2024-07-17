crate::ix!();

pub struct PathsDemo<'a> {
    base:                  GraphicsDemoBase<'a>,
    logo_path:             Path,
    use_linear_gradient:   bool,
    use_radial_gradient:   bool,
    gradient_colours:      [SlowerBouncingNumber; 9],
    gradient_positions:    [SlowerBouncingNumber; 9],
    gradient_intermediate: [SlowerBouncingNumber; 9],
}

impl<'a> PathsDemo<'a> {

    pub fn new(
        cc:     &mut ControllersComponent,
        linear: bool,
        radial: bool) -> Self {
    
        todo!();
        /*


            : GraphicsDemoBase (cc, String ("Paths") + (radial ? ": Radial Gradients"
                                                               : (linear ? ": Linear Gradients"
                                                                         : ": Solid"))),
              useLinearGradient (linear), useRadialGradient (radial)

            logoPath = getALOELogoPath();

            // rescale the logo path so that it's centred about the origin and has the right size.
            logoPath.applyTransform (RectanglePlacement (RectanglePlacement::centred)
                                     .getTransformToFit (logoPath.getBounds(),
                                                         Rectangle<float> (-120.0f, -120.0f, 240.0f, 240.0f)));

            // Surround it with some other shapes..
            logoPath.addStar ({ -300.0f, -50.0f }, 7, 30.0f, 70.0f, 0.1f);
            logoPath.addStar ({ 300.0f, 50.0f }, 6, 40.0f, 70.0f, 0.1f);
            logoPath.addEllipse (-100.0f, 150.0f, 200.0f, 140.0f);
            logoPath.addRectangle (-100.0f, -280.0f, 200.0f, 140.0f);
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& p = logoPath;

            if (useLinearGradient || useRadialGradient)
            {
                Colour c1 (gradientColours[0].getValue(), gradientColours[1].getValue(), gradientColours[2].getValue(), 1.0f);
                Colour c2 (gradientColours[3].getValue(), gradientColours[4].getValue(), gradientColours[5].getValue(), 1.0f);
                Colour c3 (gradientColours[6].getValue(), gradientColours[7].getValue(), gradientColours[8].getValue(), 1.0f);

                auto x1 = gradientPositions[0].getValue() * (float) getWidth()  * 0.25f;
                auto y1 = gradientPositions[1].getValue() * (float) getHeight() * 0.25f;
                auto x2 = gradientPositions[2].getValue() * (float) getWidth()  * 0.75f;
                auto y2 = gradientPositions[3].getValue() * (float) getHeight() * 0.75f;

                ColourGradient gradient (c1, x1, y1,
                                         c2, x2, y2,
                                         useRadialGradient);

                gradient.addColour (gradientIntermediate.getValue(), c3);

                g.setGradientFill (gradient);
            }
            else
            {
                g.setColour (Colours::blue);
            }

            g.setOpacity (getAlpha());
            g.fillPath (p, getTransform());
        */
    }
}
