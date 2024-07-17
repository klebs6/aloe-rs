crate::ix!();

pub struct LogoDrawComponent<'a> {
    base:      Component<'a>,
    base2:     Timer,
    logo_path: PathBuf, // default = getALOELogoPath() 
    elapsed:   f32, // default = 0.0f
}

impl<'a> Default for LogoDrawComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setTitle ("Aloe Logo");
                startTimerHz (30); // repaint at 30 fp
        */
    }
}

impl<'a> LogoDrawComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            Path wavePath;

                auto waveStep = 10.0f;
                auto waveY = (float) getHeight() * 0.5f;
                int i = 0;

                for (auto x = waveStep * 0.5f; x < (float) getWidth(); x += waveStep)
                {
                    auto y1 = waveY + (float) getHeight() * 0.05f * std::sin ((float) i * 0.38f + elapsed);
                    auto y2 = waveY + (float) getHeight() * 0.10f * std::sin ((float) i * 0.20f + elapsed * 2.0f);

                    wavePath.addLineSegment ({ x, y1, x, y2 }, 2.0f);
                    wavePath.addEllipse (x - waveStep * 0.3f, y1 - waveStep * 0.3f, waveStep * 0.6f, waveStep * 0.6f);
                    wavePath.addEllipse (x - waveStep * 0.3f, y2 - waveStep * 0.3f, waveStep * 0.6f, waveStep * 0.6f);

                    ++i;
                }

                g.setColour (Colour::greyLevel (0.4f));
                g.fillPath (wavePath);

                g.setColour (Colour (0xc4f39082));
                g.fillPath (logoPath, RectanglePlacement (RectanglePlacement::centred)
                                        .getTransformToFit (logoPath.getBounds(),
                                                            getLocalBounds().reduced (20, getHeight() / 4).toFloat()));
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
                elapsed += 0.02f;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::image);
        */
    }
}
