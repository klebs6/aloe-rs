crate::ix!();

///---------------------
#[no_copy]
#[leak_detector]
pub struct LookAndFeel_V4_DocumentWindowButton<'a> {
    base:          Button<'a>,
    colour:        Colour,
    normal_shape:  Path,
    toggled_shape: Path,
}

impl<'a> LookAndFeel_V4_DocumentWindowButton<'a> {

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
            auto background = Colours::grey;

            if (auto* rw = findParentComponentOfClass<ResizableWindow>())
                if (auto lf = dynamic_cast<LookAndFeel_V4*> (&rw->getLookAndFeel()))
                    background = lf->getCurrentColourScheme().getUIColour (LookAndFeel_V4::ColourScheme::widgetBackground);

            g.fillAll (background);

            g.setColour ((! isEnabled() || shouldDrawButtonAsDown) ? colour.withAlpha (0.6f)
                                                         : colour);

            if (shouldDrawButtonAsHighlighted)
            {
                g.fillAll();
                g.setColour (background);
            }

            auto& p = getToggleState() ? toggledShape : normalShape;

            auto reducedRect = Justification (Justification::centred)
                                  .appliedToRectangle (Rectangle<int> (getHeight(), getHeight()), getLocalBounds())
                                  .toFloat()
                                  .reduced ((float) getHeight() * 0.3f);

            g.fillPath (p, p.getTransformToScaleToFit (reducedRect, true));
        */
    }
}
