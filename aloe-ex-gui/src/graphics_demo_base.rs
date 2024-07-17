crate::ix!();

pub trait GraphicsDemoBaseInterface {
    fn draw_demo(&mut self, _0: &mut Graphics);
}

#[no_copy]
#[leak_detector]
pub struct GraphicsDemoBase<'a> {
    base:                   Component<'a>,
    controls:               &'a mut ControllersComponent<'a>,
    offsetx:                SlowerBouncingNumber,
    offsety:                SlowerBouncingNumber,
    rotation:               SlowerBouncingNumber,
    size:                   SlowerBouncingNumber,
    shear:                  SlowerBouncingNumber,
    alpha:                  SlowerBouncingNumber,
    clip_rectx:             SlowerBouncingNumber,
    clip_recty:             SlowerBouncingNumber,
    clip_pathx:             SlowerBouncingNumber,
    clip_pathy:             SlowerBouncingNumber,
    clip_path_depth:        SlowerBouncingNumber,
    clip_path_angle:        SlowerBouncingNumber,
    clip_imagex:            SlowerBouncingNumber,
    clip_imagey:            SlowerBouncingNumber,
    clip_image_angle:       SlowerBouncingNumber,
    clip_image_size:        SlowerBouncingNumber,
    last_render_start_time: f64, // default = 0.0
    average_time_ms:        f64, // default = 0.0
    average_actualfps:      f64, // default = 0.0
    clip_image:             Image,
    display_font:           Font,
}

impl<'a> GraphicsDemoBase<'a> {

    pub fn new(
        cc:   &mut ControllersComponent,
        name: &String) -> Self {
    
        todo!();
        /*
        : component(name),
        : controls(cc),

            displayFont = Font (Font::getDefaultMonospacedFontName(), 12.0f, Font::bold);
        */
    }
    
    pub fn get_transform(&mut self) -> AffineTransform {
        
        todo!();
        /*
            auto hw = 0.5f * (float) getWidth();
            auto hh = 0.5f * (float) getHeight();

            AffineTransform t;

            if (controls.animateRotation.getToggleState())
                t = t.rotated (rotation.getValue() * MathConstants<float>::twoPi);

            if (controls.animateSize.getToggleState())
                t = t.scaled (0.3f + size.getValue() * 2.0f);

            if (controls.animatePosition.getToggleState())
                t = t.translated (hw + hw * (offsetX.getValue() - 0.5f),
                                  hh + hh * (offsetY.getValue() - 0.5f));
            else
                t = t.translated (hw, hh);

            if (controls.animateShear.getToggleState())
                t = t.sheared (shear.getValue() * 2.0f - 1.0f, 0.0f);

            return t;
        */
    }
    
    pub fn get_alpha(&self) -> f32 {
        
        todo!();
        /*
            if (controls.animateAlpha.getToggleState())
                return alpha.getValue();

            return 1.0f;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto startTime = 0.0;

            {
                // A ScopedSaveState will return the Graphics context to the state it was at the time of
                // construction when it goes out of scope. We use it here to avoid clipping the fps text
                const Graphics::ScopedSaveState state (g);

                if (controls.clipToRectangle.getToggleState())  clipToRectangle (g);
                if (controls.clipToPath     .getToggleState())  clipToPath (g);
                if (controls.clipToImage    .getToggleState())  clipToImage (g);

                g.setImageResamplingQuality (controls.quality.getToggleState() ? Graphics::highResamplingQuality
                                                                               : Graphics::mediumResamplingQuality);

                // take a note of the time before the render
                startTime = Time::getMillisecondCounterHiRes();

                // then let the demo draw itself..
                drawDemo (g);
            }

            auto now = Time::getMillisecondCounterHiRes();
            auto filtering = 0.08;

            auto elapsedMs = now - startTime;
            averageTimeMs += (elapsedMs - averageTimeMs) * filtering;

            auto sinceLastRender = now - lastRenderStartTime;
            lastRenderStartTime = now;

            auto effectiveFPS = 1000.0 / averageTimeMs;
            auto actualFPS = sinceLastRender > 0 ? (1000.0 / sinceLastRender) : 0;
            averageActualFPS += (actualFPS - averageActualFPS) * filtering;

            GlyphArrangement ga;
            ga.addFittedText (displayFont,
                              "Time: " + String (averageTimeMs, 2)
                                + " ms\nEffective FPS: " + String (effectiveFPS, 1)
                                + "\nActual FPS: " + String (averageActualFPS, 1),
                              0, 10.0f, (float) getWidth() - 10.0f, (float) getHeight(), Justification::topRight, 3);

            g.setColour (Colours::white.withAlpha (0.5f));
            g.fillRect (ga.getBoundingBox (0, ga.getNumGlyphs(), true).getSmallestIntegerContainer().expanded (4));

            g.setColour (Colours::black);
            ga.draw (g);
        */
    }
    
    pub fn clip_to_rectangle(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto w = getWidth()  / 2;
            auto h = getHeight() / 2;

            auto x = (int) ((float) w * clipRectX.getValue());
            auto y = (int) ((float) h * clipRectY.getValue());

            g.reduceClipRegion (x, y, w, h);
        */
    }
    
    pub fn clip_to_path(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto pathSize = (float) jmin (getWidth(), getHeight());

            Path p;
            p.addStar (Point<float> (clipPathX.getValue(),
                                     clipPathY.getValue()) * pathSize,
                       7,
                       pathSize * (0.5f + clipPathDepth.getValue()),
                       pathSize * 0.5f,
                       clipPathAngle.getValue());

            g.reduceClipRegion (p, AffineTransform());
        */
    }
    
    pub fn clip_to_image(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (! clipImage.isValid())
                createClipImage();

            AffineTransform transform (AffineTransform::translation ((float) clipImage.getWidth()  / -2.0f,
                                                                     (float) clipImage.getHeight() / -2.0f)
                                       .rotated (clipImageAngle.getValue() * MathConstants<float>::twoPi)
                                       .scaled (2.0f + clipImageSize.getValue() * 3.0f)
                                       .translated ((float) getWidth()  * 0.5f,
                                                    (float) getHeight() * 0.5f));

            g.reduceClipRegion (clipImage, transform);
        */
    }
    
    pub fn create_clip_image(&mut self)  {
        
        todo!();
        /*
            clipImage = Image (Image::ARGB, 300, 300, true);

            Graphics g (clipImage);

            g.setGradientFill (ColourGradient (Colours::transparentBlack, 0, 0,
                                               Colours::black, 0, 300, false));

            for (int i = 0; i < 20; ++i)
                g.fillRect (Random::getSystemRandom().nextInt (200),
                            Random::getSystemRandom().nextInt (200),
                            Random::getSystemRandom().nextInt (100),
                            Random::getSystemRandom().nextInt (100));
        */
    }
}
