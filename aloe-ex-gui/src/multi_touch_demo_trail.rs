crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MultiTouchDemoTrail<'a> {
    source:           MouseInputSource<'a>,
    path:             Path,
    colour:           Colour, //{ getRandomBrightColour().withAlpha (0.6f) };
    last_point:       Point<f32>,
    current_position: Point<f32>,
    modifier_keys:    ModifierKeys,
}

impl<'a> MultiTouchDemoTrail<'a> {

    pub fn new(ms: &MouseInputSource) -> Self {
    
        todo!();
        /*
        : source(ms),

        
        */
    }
    
    pub fn push_point(&mut self, 
        new_point: Point<f32>,
        new_mods:  ModifierKeys,
        pressure:  f32)  {
        
        todo!();
        /*
            currentPosition = newPoint;
                modifierKeys = newMods;

                if (lastPoint.getDistanceFrom (newPoint) > 5.0f)
                {
                    if (lastPoint != Point<float>())
                    {
                        Path newSegment;
                        newSegment.startNewSubPath (lastPoint);
                        newSegment.lineTo (newPoint);

                        auto diameter = 20.0f * (pressure > 0 && pressure < 1.0f ? pressure : 1.0f);

                        PathStrokeType (diameter, PathStrokeType::curved, PathStrokeType::rounded).createStrokedPath (newSegment, newSegment);
                        path.addPath (newSegment);
                    }

                    lastPoint = newPoint;
                }
        */
    }
}
