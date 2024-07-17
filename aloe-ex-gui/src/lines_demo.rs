crate::ix!();

pub struct LinesDemo<'a> {
    base:      GraphicsDemoBase<'a>,
    offset:    SlowerBouncingNumber,
    positions: [SlowerBouncingNumber; 8],
}

impl<'a> LinesDemo<'a> {
    
    pub fn new(cc: &mut ControllersComponent) -> Self {
    
        todo!();
        /*
        : graphics_demo_base(cc, "Lines"),

        
        */
    }
    
    pub fn draw_demo(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            {
                RectangleList<float> verticalLines;
                verticalLines.ensureStorageAllocated (getWidth());

                auto pos = offset.getValue();

                for (int x = 0; x < getWidth(); ++x)
                {
                    auto y = (float) getHeight() * 0.3f;
                    auto length = y * std::abs (std::sin ((float) x / 100.0f + 2.0f * pos));
                    verticalLines.addWithoutMerging (Rectangle<float> ((float) x, y - length * 0.5f, 1.0f, length));
                }

                g.setColour (Colours::blue.withAlpha (getAlpha()));
                g.fillRectList (verticalLines);
            }

            {
                RectangleList<float> horizontalLines;
                horizontalLines.ensureStorageAllocated (getHeight());

                auto pos = offset.getValue();

                for (int y = 0; y < getHeight(); ++y)
                {
                    auto x = (float) getWidth() * 0.3f;
                    auto length = x * std::abs (std::sin ((float) y / 100.0f + 2.0f * pos));
                    horizontalLines.addWithoutMerging (Rectangle<float> (x - length * 0.5f, (float) y, length, 1.0f));
                }

                g.setColour (Colours::green.withAlpha (getAlpha()));
                g.fillRectList (horizontalLines);
            }

            g.setColour (Colours::red.withAlpha (getAlpha()));

            auto w = (float) getWidth();
            auto h = (float) getHeight();

            g.drawLine (positions[0].getValue() * w,
                        positions[1].getValue() * h,
                        positions[2].getValue() * w,
                        positions[3].getValue() * h);

            g.drawLine (positions[4].getValue() * w,
                        positions[5].getValue() * h,
                        positions[6].getValue() * w,
                        positions[7].getValue() * h);
        */
    }
}
