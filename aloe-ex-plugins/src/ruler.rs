crate::ix!();

pub struct Ruler<'a> {
    base:                        Component<'a>,
    visible_range:               VisibleRangeDataModel<'a>,
    visible_range_on_mouse_down: Range<f64>,
    time_on_mouse_down:          f64,
}

impl<'a> VisibleRangeDataModelListener for Ruler<'a> {

}

impl<'a> Ruler<'a> {

    pub fn new(model: &VisibleRangeDataModel) -> Self {
    
        todo!();
        /*
        : visible_range(model),

            visibleRange.addListener (*this);
            setMouseCursor (MouseCursor::LeftRightResizeCursor);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto minDivisionWidth = 50.0f;
            auto maxDivisions     = (float) getWidth() / minDivisionWidth;

            auto lookFeel = dynamic_cast<LookAndFeel_V4*> (&getLookAndFeel());
            auto bg = lookFeel->getCurrentColourScheme()
                               .getUIColour (LookAndFeel_V4::ColourScheme::UIColour::widgetBackground);

            g.setGradientFill (ColourGradient (bg.brighter(),
                                               0,
                                               0,
                                               bg.darker(),
                                               0,
                                               (float) getHeight(),
                                               false));

            g.fillAll();
            g.setColour (bg.brighter());
            g.drawHorizontalLine (0, 0.0f, (float) getWidth());
            g.setColour (bg.darker());
            g.drawHorizontalLine (1, 0.0f, (float) getWidth());
            g.setColour (Colours::lightgrey);

            auto minLog = std::ceil (std::log10 (visibleRange.getVisibleRange().getLength() / maxDivisions));
            auto precision = 2 + std::abs (minLog);
            auto divisionMagnitude = std::pow (10, minLog);
            auto startingDivision = std::ceil (visibleRange.getVisibleRange().getStart() / divisionMagnitude);

            for (auto div = startingDivision; div * divisionMagnitude < visibleRange.getVisibleRange().getEnd(); ++div)
            {
                auto time = div * divisionMagnitude;
                auto xPos = (time - visibleRange.getVisibleRange().getStart()) * getWidth()
                                  / visibleRange.getVisibleRange().getLength();

                std::ostringstream outStream;
                outStream << std::setprecision (roundToInt (precision)) << time;

                const auto bounds = Rectangle<int> (Point<int> (roundToInt (xPos) + 3, 0),
                                                    Point<int> (roundToInt (xPos + minDivisionWidth), getHeight()));

                g.drawText (outStream.str(), bounds, Justification::centredLeft, false);

                g.drawVerticalLine (roundToInt (xPos), 2.0f, (float) getHeight());
            }
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            visibleRangeOnMouseDown = visibleRange.getVisibleRange();
            timeOnMouseDown = visibleRange.getVisibleRange().getStart()
                           + (visibleRange.getVisibleRange().getLength() * e.getMouseDownX()) / getWidth();
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            // Work out the scale of the new range
            auto unitDistance = 100.0f;
            auto scaleFactor  = 1.0 / std::pow (2, (float) e.getDistanceFromDragStartY() / unitDistance);

            // Now position it so that the mouse continues to point at the same
            // place on the ruler.
            auto visibleLength = std::max (0.12, visibleRangeOnMouseDown.getLength() * scaleFactor);
            auto rangeBegin = timeOnMouseDown - visibleLength * e.x / getWidth();
            const Range<double> range (rangeBegin, rangeBegin + visibleLength);
            visibleRange.setVisibleRange (range, nullptr);
        */
    }
    
    pub fn visible_range_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
            repaint();
        */
    }
}
