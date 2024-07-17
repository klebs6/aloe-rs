crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ArrowButton.h]

/**
  | A button with an arrow in it.
  | 
  | @see Button
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ArrowButton<'a> {
    base:   Button<'a>,
    colour: Colour,
    path:   Path,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ArrowButton.cpp]
impl<'a> ArrowButton<'a> {

    /**
      | Creates an ArrowButton.
      | 
      | -----------
      | @param buttonName
      | 
      | the name to give the button
      | ----------
      | @param arrowDirection
      | 
      | the direction the arrow should point
      | in, where 0.0 is pointing right, 0.25
      | is down, 0.5 is left, 0.75 is up
      | ----------
      | @param arrowColour
      | 
      | the colour to use for the arrow
      |
      */
    pub fn new(
        name:                       &String,
        arrow_direction_in_radians: f32,
        arrow_colour:               Colour) -> Self {
    
        todo!();
        /*
        : button(name),
        : colour(arrowColour),

            path.addTriangle (0.0f, 0.0f, 0.0f, 1.0f, 1.0f, 0.5f);
        path.applyTransform (AffineTransform::rotation (MathConstants<float>::twoPi * arrowDirectionInRadians, 0.5f, 0.5f));
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            Path p (path);

        const float offset = shouldDrawButtonAsDown ? 1.0f : 0.0f;
        p.applyTransform (path.getTransformToScaleToFit (offset, offset, (float) getWidth() - 3.0f, (float) getHeight() - 3.0f, false));

        DropShadow (Colours::black.withAlpha (0.3f), shouldDrawButtonAsDown ? 2 : 4, Point<int>()).drawForPath (g, p);

        g.setColour (colour);
        g.fillPath (p);
        */
    }
}
