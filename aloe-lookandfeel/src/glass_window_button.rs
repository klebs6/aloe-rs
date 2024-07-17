crate::ix!();

#[no_copy]
#[leak_detector]
pub struct GlassWindowButton<'a> {
    base:          Button<'a>,
    colour:        Colour,
    normal_shape:  Path,
    toggled_shape: Path,
}

impl<'a> GlassWindowButton<'a> {

    pub fn new(
        name:          &String,
        col:           Colour,
        normal_shape:  &Path,
        toggled_shape: &Path) -> Self {
    
        todo!();
        /*
        : button(name),
        : colour(col),
        : normal_shape(normalShape_),
        : toggled_shape(toggledShape_),
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            float alpha = shouldDrawButtonAsHighlighted ? (shouldDrawButtonAsDown ? 1.0f : 0.8f) : 0.55f;

            if (! isEnabled())
                alpha *= 0.5f;

            float x = 0, y = 0, diam;

            if (getWidth() < getHeight())
            {
                diam = (float) getWidth();
                y = (float) (getHeight() - getWidth()) * 0.5f;
            }
            else
            {
                diam = (float) getHeight();
                y = (float) (getWidth() - getHeight()) * 0.5f;
            }

            x += diam * 0.05f;
            y += diam * 0.05f;
            diam *= 0.9f;

            g.setGradientFill (ColourGradient (Colour::greyLevel (0.9f).withAlpha (alpha), 0, y + diam,
                                               Colour::greyLevel (0.6f).withAlpha (alpha), 0, y, false));
            g.fillEllipse (x, y, diam, diam);

            x += 2.0f;
            y += 2.0f;
            diam -= 4.0f;

            LookAndFeel_V2::drawGlassSphere (g, x, y, diam, colour.withAlpha (alpha), 1.0f);

            Path& p = getToggleState() ? toggledShape : normalShape;

            const AffineTransform t (p.getTransformToScaleToFit (x + diam * 0.3f, y + diam * 0.3f,
                                                                 diam * 0.4f, diam * 0.4f, true));

            g.setColour (Colours::black.withAlpha (alpha * 0.6f));
            g.fillPath (p, t);
        */
    }
}
