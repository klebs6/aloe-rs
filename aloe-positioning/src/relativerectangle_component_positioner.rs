crate::ix!();

#[no_copy]
#[leak_detector]
pub struct RelativeRectangleComponentPositioner<'a> {
    base:      RelativeCoordinatePositionerBase<'a>,
    rectangle: RelativeRectangle,
}

impl<'a> RelativeRectangleComponentPositioner<'a> {

    pub fn new(
        comp: &mut Component,
        r:    &RelativeRectangle) -> Self {
    
        todo!();
        /*
        : relative_coordinate_positioner_base(comp),
        : rectangle(r),

        
        */
    }
    
    pub fn register_coordinates(&mut self) -> bool {
        
        todo!();
        /*
            bool ok = addCoordinate (rectangle.left);
            ok = addCoordinate (rectangle.right) && ok;
            ok = addCoordinate (rectangle.top) && ok;
            ok = addCoordinate (rectangle.bottom) && ok;
            return ok;
        */
    }
    
    pub fn is_using_rectangle(&self, other: &RelativeRectangle) -> bool {
        
        todo!();
        /*
            return rectangle == other;
        */
    }
    
    pub fn apply_to_component_bounds(&mut self)  {
        
        todo!();
        /*
            for (int i = 32; --i >= 0;)
            {
                ComponentScope scope (getComponent());
                const Rectangle<int> newBounds (rectangle.resolve (&scope).getSmallestIntegerContainer());

                if (newBounds == getComponent().getBounds())
                    return;

                getComponent().setBounds (newBounds);
            }

            jassertfalse; // Seems to be a recursive reference!
        */
    }
    
    pub fn apply_new_bounds(&mut self, new_bounds: &Rectangle<i32>)  {
        
        todo!();
        /*
            if (newBounds != getComponent().getBounds())
            {
                ComponentScope scope (getComponent());
                rectangle.moveToAbsolute (newBounds.toFloat(), &scope);

                applyToComponentBounds();
            }
        */
    }
}
