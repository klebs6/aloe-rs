crate::ix!();

#[no_copy]
pub struct ColourSpaceView<'a> {
    base:     Component<'a>,
    owner:    &'a mut ColourSelector<'a>,
    h:        &'a mut f32,
    s:        &'a mut f32,
    v:        &'a mut f32,
    last_hue: f32, // default = 0
    edge:     i32,
    colours:  Image,
    marker:   ColourSpaceMarker<'a>,
}

impl<'a> ColourSpaceView<'a> {

    pub fn new(
        cs:        &mut ColourSelector,
        hue:       &mut f32,
        sat:       &mut f32,
        val:       &mut f32,
        edge_size: i32) -> Self {
    
        todo!();
        /*
        : owner(cs),
        : h(hue),
        : s(sat),
        : v(val),
        : edge(edgeSize),

            addAndMakeVisible (marker);
            setMouseCursor (MouseCursor::CrosshairCursor);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (colours.isNull())
            {
                auto width = getWidth() / 2;
                auto height = getHeight() / 2;
                colours = Image (Image::RGB, width, height, false);

                ImageBitmapData pixels (colours, ImageBitmapData::writeOnly);

                for (int y = 0; y < height; ++y)
                {
                    auto val = 1.0f - (float) y / (float) height;

                    for (int x = 0; x < width; ++x)
                    {
                        auto sat = (float) x / (float) width;
                        pixels.setPixelColour (x, y, Colour (h, sat, val, 1.0f));
                    }
                }
            }

            g.setOpacity (1.0f);
            g.drawImageTransformed (colours,
                                    RectanglePlacement (RectanglePlacement::stretchToFit)
                                        .getTransformToFit (colours.getBounds().toFloat(),
                                                            getLocalBounds().reduced (edge).toFloat()),
                                    false);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDrag (e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto sat =        (float) (e.x - edge) / (float) (getWidth()  - edge * 2);
            auto val = 1.0f - (float) (e.y - edge) / (float) (getHeight() - edge * 2);

            owner.setSV (sat, val);
        */
    }
    
    pub fn update_if_needed(&mut self)  {
        
        todo!();
        /*
            if (lastHue != h)
            {
                lastHue = h;
                colours = {};
                repaint();
            }

            updateMarker();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            colours = {};
            updateMarker();
        */
    }
    
    pub fn update_marker(&mut self)  {
        
        todo!();
        /*
            auto markerSize = jmax (14, edge * 2);
            auto area = getLocalBounds().reduced (edge);

            marker.setBounds (Rectangle<int> (markerSize, markerSize)
                                .withCentre (area.getRelativePoint (s, 1.0f - v)));
        */
    }
}
