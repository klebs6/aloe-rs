crate::ix!();

#[no_copy]
#[leak_detector]
pub struct LookAndFeel_V3_DocumentWindowButton<'a> {
    base:          Button<'a>,
    colour:        Colour,
    normal_shape:  Path,
    toggled_shape: Path,
}

impl<'a> LookAndFeel_V3_DocumentWindowButton<'a> {

    pub fn new(
        name:    &String,
        c:       Colour,
        normal:  &Path,
        toggled: &Path) -> Self {
    
        todo!();
        /*
        : button(name),
        : colour(c),
        : normal_shape(normal),
        : toggled_shape(toggled),

        
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            Colour background (Colours::grey);

            if (ResizableWindow* rw = findParentComponentOfClass<ResizableWindow>())
                background = rw->getBackgroundColour();

            const float cx = (float) getWidth() * 0.5f, cy = (float) getHeight() * 0.5f;
            const float diam = jmin (cx, cy) * (shouldDrawButtonAsDown ? 0.60f : 0.65f);

            g.setColour (background);
            g.fillEllipse (cx - diam, cy - diam, diam * 2.0f, diam * 2.0f);

            Colour c (background.contrasting (colour, 0.6f));

            if (! isEnabled())
                c = c.withAlpha (0.6f);
            else if (shouldDrawButtonAsHighlighted)
                c = c.brighter();

            g.setColour (c);
            g.drawEllipse (cx - diam, cy - diam, diam * 2.0f, diam * 2.0f, diam * 0.2f);

            Path& p = getToggleState() ? toggledShape : normalShape;

            float scale = 0.55f;
            g.fillPath (p, p.getTransformToScaleToFit (cx - diam * scale, cy - diam * scale,
                                                       diam * 2.0f * scale, diam * 2.0f * scale, true));
        */
    }
}

