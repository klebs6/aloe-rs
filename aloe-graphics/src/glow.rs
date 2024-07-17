crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/effects/aloe_GlowEffect.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/effects/aloe_GlowEffect.cpp]

/**
  | A component effect that adds a coloured
  | blur around the component's contents.
  | 
  | (This will only work on non-opaque components).
  | 
  | @see Component::setComponentEffect,
  | DropShadowEffect
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct GlowEffect {
    radius: f32,    // default = 2.0f
    colour: Colour, // default = { Colours::white  }
    offset: Point<i32>,
}

impl ImageEffectFilter for GlowEffect {

    fn apply_effect(&mut self, 
        image:        &mut Image,
        g:            &mut Graphics,
        scale_factor: f32,
        alpha:        f32)  {
        
        todo!();
        /*
            Image temp (image.getFormat(), image.getWidth(), image.getHeight(), true);

        ImageConvolutionKernel blurKernel (roundToInt (radius * scaleFactor * 2.0f));

        blurKernel.createGaussianBlur (radius);
        blurKernel.rescaleAllValues (radius);

        blurKernel.applyToImage (temp, image, image.getBounds());

        g.setColour (colour.withMultipliedAlpha (alpha));
        g.drawImageAt (temp, offset.x, offset.y, true);

        g.setOpacity (alpha);
        g.drawImageAt (image, offset.x, offset.y, false);
        */
    }
}

impl Default for GlowEffect {
    
    /**
      | Creates a default 'glow' effect.
      | 
      | To customise its appearance, use the
      | setGlowProperties() method.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

impl GlowEffect {
    
    /**
      | Sets the glow's radius and colour.
      | 
      | The radius is how large the blur should
      | be, and the colour is used to render it
      | (for a less intense glow, lower the colour's
      | opacity).
      |
      */
    pub fn set_glow_properties(&mut self, 
        new_radius: f32,
        new_colour: Colour,
        pos:        Point<i32>)  {
        
        todo!();
        /*
            radius = newRadius;
        colour = newColour;
        offset = pos;
        */
    }
}
