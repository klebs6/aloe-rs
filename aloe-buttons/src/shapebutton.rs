crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ShapeButton.h]

/**
  | A button that contains a filled shape.
  | 
  | @see Button, ImageButton, TextButton,
  | ArrowButton
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ShapeButton<'a> {
    base:                       Button<'a>,
    normal_colour:              Colour,
    over_colour:                Colour,
    down_colour:                Colour,
    normal_colour_on:           Colour,
    over_colour_on:             Colour,
    down_colour_on:             Colour,
    outline_colour:             Colour,
    use_on_colours:             bool,
    shadow:                     DropShadowEffect,
    shape:                      Path,
    border:                     BorderSize<i32>,
    maintain_shape_proportions: bool,
    outline_width:              f32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ShapeButton.cpp]
impl<'a> ShapeButton<'a> {
    
    /**
      | Creates a ShapeButton.
      | 
      | -----------
      | @param name
      | 
      | a name to give the component - see Component::setName()
      | ----------
      | @param normalColour
      | 
      | the colour to fill the shape with when
      | the mouse isn't over
      | ----------
      | @param overColour
      | 
      | the colour to use when the mouse is over
      | the shape
      | ----------
      | @param downColour
      | 
      | the colour to use when the button is in
      | the pressed-down state
      |
      */
    pub fn new(
        t: &String,
        n: Colour,
        o: Colour,
        d: Colour) -> Self {
    
        todo!();
        /*
        : button(t),
        : normal_colour(n),
        : over_colour(o),
        : down_colour(d),
        : normal_colour_on(n),
        : over_colour_on(o),
        : down_colour_on(d),
        : use_on_colours(false),
        : maintain_shape_proportions(false),
        : outline_width(0.0f),

        
        */
    }
    
    /**
      | Set the colours to use for drawing the
      | shape.
      | 
      | -----------
      | @param normalColour
      | 
      | the colour to fill the shape with when
      | the mouse isn't over
      | ----------
      | @param overColour
      | 
      | the colour to use when the mouse is over
      | the shape
      | ----------
      | @param downColour
      | 
      | the colour to use when the button is in
      | the pressed-down state
      |
      */
    pub fn set_colours(&mut self, 
        new_normal_colour: Colour,
        new_over_colour:   Colour,
        new_down_colour:   Colour)  {
        
        todo!();
        /*
            normalColour = newNormalColour;
        overColour   = newOverColour;
        downColour   = newDownColour;
        */
    }
    
    /**
      | Sets the colours to use for drawing the
      | shape when the button's toggle state
      | is 'on'. To enable this behaviour, use
      | the shouldUseOnColours() method.
      | 
      | -----------
      | @param normalColourOn
      | 
      | the colour to fill the shape with when
      | the mouse isn't over and the button's
      | toggle state is 'on'
      | ----------
      | @param overColourOn
      | 
      | the colour to use when the mouse is over
      | the shape and the button's toggle state
      | is 'on'
      | ----------
      | @param downColourOn
      | 
      | the colour to use when the button is in
      | the pressed-down state and the button's
      | toggle state is 'on'
      |
      */
    pub fn set_on_colours(&mut self, 
        new_normal_colour_on: Colour,
        new_over_colour_on:   Colour,
        new_down_colour_on:   Colour)  {
        
        todo!();
        /*
            normalColourOn = newNormalColourOn;
        overColourOn   = newOverColourOn;
        downColourOn   = newDownColourOn;
        */
    }
    
    /**
      | Set whether the button should use the
      | 'on' set of colours when its toggle state
      | is 'on'. By default these will be the
      | same as the normal colours but the setOnColours
      | method can be used to provide a different
      | set of colours.
      |
      */
    pub fn should_use_on_colours(&mut self, should_use: bool)  {
        
        todo!();
        /*
            useOnColours = shouldUse;
        */
    }
    
    /**
      | Sets up an outline to draw around the
      | shape.
      | 
      | -----------
      | @param outlineColour
      | 
      | the colour to use
      | ----------
      | @param outlineStrokeWidth
      | 
      | the thickness of line to draw
      |
      */
    pub fn set_outline(&mut self, 
        new_outline_colour: Colour,
        new_outline_width:  f32)  {
        
        todo!();
        /*
            outlineColour = newOutlineColour;
        outlineWidth = newOutlineWidth;
        */
    }
    
    /**
      | This lets you specify a border to be left
      | around the edge of the button when drawing
      | the shape.
      |
      */
    pub fn set_border_size(&mut self, new_border: BorderSize<i32>)  {
        
        todo!();
        /*
            border = newBorder;
        */
    }
    
    /**
      | Sets the shape to use.
      | 
      | -----------
      | @param newShape
      | 
      | the shape to use
      | ----------
      | @param resizeNowToFitThisShape
      | 
      | if true, the button will be resized to
      | fit the shape's bounds
      | ----------
      | @param maintainShapeProportions
      | 
      | if true, the shape's proportions will
      | be kept fixed when the button is resized
      | ----------
      | @param hasDropShadow
      | 
      | if true, the button will be given a drop-shadow
      | effect
      |
      */
    pub fn set_shape(&mut self, 
        new_shape:                    &Path,
        resize_now_to_fit_this_shape: bool,
        maintain_shape_proportions:   bool,
        has_shadow:                   bool)  {
        
        todo!();
        /*
            shape = newShape;
        maintainShapeProportions = maintainShapeProportions_;

        shadow.setShadowProperties (DropShadow (Colours::black.withAlpha (0.5f), 3, Point<int>()));
        setComponentEffect (hasShadow ? &shadow : nullptr);

        if (resizeNowToFitThisShape)
        {
            auto newBounds = shape.getBounds();

            if (hasShadow)
                newBounds = newBounds.expanded (4.0f);

            shape.applyTransform (AffineTransform::translation (-newBounds.getX(),
                                                                -newBounds.getY()));

            setSize (1 + (int) (newBounds.getWidth()  + outlineWidth) + border.getLeftAndRight(),
                     1 + (int) (newBounds.getHeight() + outlineWidth) + border.getTopAndBottom());
        }

        repaint();
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            if (! isEnabled())
        {
            shouldDrawButtonAsHighlighted = false;
            shouldDrawButtonAsDown = false;
        }

        auto r = border.subtractedFrom (getLocalBounds())
                       .toFloat()
                       .reduced (outlineWidth * 0.5f);

        if (getComponentEffect() != nullptr)
            r = r.reduced (2.0f);

        if (shouldDrawButtonAsDown)
        {
            const float sizeReductionWhenPressed = 0.04f;

            r = r.reduced (sizeReductionWhenPressed * r.getWidth(),
                           sizeReductionWhenPressed * r.getHeight());
        }

        auto trans = shape.getTransformToScaleToFit (r, maintainShapeProportions);

        if      (shouldDrawButtonAsDown)        g.setColour (getToggleState() && useOnColours ? downColourOn   : downColour);
        else if (shouldDrawButtonAsHighlighted) g.setColour (getToggleState() && useOnColours ? overColourOn   : overColour);
        else                                    g.setColour (getToggleState() && useOnColours ? normalColourOn : normalColour);

        g.fillPath (shape, trans);

        if (outlineWidth > 0.0f)
        {
            g.setColour (outlineColour);
            g.strokePath (shape, PathStrokeType (outlineWidth), trans);
        }
        */
    }
}
