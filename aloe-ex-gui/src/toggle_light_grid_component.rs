crate::ix!();

const NUM_X: usize = 20;
const NUM_Y: usize = 20;

/**
  | This is the parent class that holds multiple
  | ToggleLightComponents in a grid.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ToggleLightGridComponent<'a> {
    base:           Component<'a>,
    toggle_lights: [ToggleLightComponent<'a>; NUM_X * NUM_Y],
}

impl<'a> Default for ToggleLightGridComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            // Adds the child light components and makes them visible
            // within this component.
            // (they currently rely on having a default constructor
            // so they don't have to be individually initialised)
            for (auto i = 0; i < numX * numY; ++i)
                addAndMakeVisible (toggleLights[i])
        */
    }
}

impl<'a> ToggleLightGridComponent<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This creates a grid of rectangles to use as the bounds
            // for all of our lights. The grid is defined with the
            // width and height of this component.

            auto stepX = getWidth()  / numX;
            auto stepY = getHeight() / numY;

            for (auto x = 0; x < numX; ++x)
            {
                for (auto y = 0; y < numY; ++y)
                {
                    // creates the rectangle     (x,         y,         width, height)
                    Rectangle<int> elementBounds (x * stepX, y * stepY, stepX, stepY);

                    // set the size and position of the Toggle light to this rectangle.
                    toggleLights[x + numX * y].setBounds (elementBounds);
                }
            }
        */
    }
}
