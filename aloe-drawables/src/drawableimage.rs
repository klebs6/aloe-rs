crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableImage.h]

/**
  | A drawable object which is a bitmap image.
  | 
  | @see Drawable
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct DrawableImage<'a> {

    base:           Drawable<'a>,
    image:          Image,
    opacity:        f32, // default = 1.0f
    overlay_colour: Colour, // default = { 0  }
    bounds:         Parallelogram<f32>,
}

impl<'a> Default for DrawableImage<'a> {

    fn default() -> Self {
    
        todo!();
        /*
            : bounds ({ 0.0f, 0.0f, 1.0f, 1.0f })
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableImage.cpp]
impl<'a> DrawableImage<'a> {

    /**
      | Returns the current image.
      |
      */
    pub fn get_image(&self) -> &Image {
        
        todo!();
        /*
            return image;
        */
    }

    /**
      | Returns the image's opacity.
      |
      */
    pub fn get_opacity(&self) -> f32 {
        
        todo!();
        /*
            return opacity;
        */
    }

    /**
      | Returns the overlay colour.
      |
      */
    pub fn get_overlay_colour(&self) -> Colour {
        
        todo!();
        /*
            return overlayColour;
        */
    }

    /**
      | Returns the position to which the image's
      | top-left corner should be remapped
      | in the target coordinate space when
      | rendering this object. @see setTransform
      |
      */
    pub fn get_bounding_box(&self) -> Parallelogram<f32> {
        
        todo!();
        /*
            return bounds;
        */
    }

    pub fn new_from_drawable(other: &DrawableImage) -> Self {
    
        todo!();
        /*
        : drawable(other),
        : image(other.image),
        : opacity(other.opacity),
        : overlay_colour(other.overlayColour),
        : bounds(other.bounds),

            setBounds (other.getBounds());
        */
    }
    
    /**
      | Sets the image that this drawable will
      | render.
      |
      */
    pub fn new(image_to_use: &Image) -> Self {
    
        todo!();
        /*


            setImageInternal (imageToUse);
        */
    }
    
    pub fn create_copy(&self) -> Box<Drawable> {
        
        todo!();
        /*
            return std::make_unique<DrawableImage> (*this);
        */
    }
    
    /**
      | Sets the image that this drawable will
      | render.
      |
      */
    pub fn set_image(&mut self, image_to_use: &Image)  {
        
        todo!();
        /*
            if (setImageInternal (imageToUse))
            repaint();
        */
    }
    
    /**
      | Sets the opacity to use when drawing
      | the image.
      |
      */
    pub fn set_opacity(&mut self, new_opacity: f32)  {
        
        todo!();
        /*
            opacity = newOpacity;
        */
    }
    
    /**
      | Sets a colour to draw over the image's
      | alpha channel.
      | 
      | By default this is transparent so isn't
      | drawn, but if you set a non-transparent
      | colour here, then it will be overlaid
      | on the image, using the image's alpha
      | channel as a mask.
      | 
      | This is handy for doing things like darkening
      | or lightening an image by overlaying
      | it with semi-transparent black or white.
      |
      */
    pub fn set_overlay_colour(&mut self, new_overlay_colour: Colour)  {
        
        todo!();
        /*
            overlayColour = newOverlayColour;
        */
    }
    
    /**
      | Sets the bounding box within which the
      | image should be displayed.
      |
      */
    pub fn set_bounding_box_from_rect(&mut self, new_bounds: Rectangle<f32>)  {
        
        todo!();
        /*
            setBoundingBox (Parallelogram<float> (newBounds));
        */
    }
    
    /**
      | Sets the bounding box within which the
      | image should be displayed.
      |
      */
    pub fn set_bounding_box(&mut self, new_bounds: Parallelogram<f32>)  {
        
        todo!();
        /*
            if (bounds != newBounds)
        {
            bounds = newBounds;

            if (image.isValid())
            {
                auto tr = bounds.topLeft + (bounds.topRight   - bounds.topLeft) / (float) image.getWidth();
                auto bl = bounds.topLeft + (bounds.bottomLeft - bounds.topLeft) / (float) image.getHeight();

                auto t = AffineTransform::fromTargetPoints (bounds.topLeft.x, bounds.topLeft.y,
                                                            tr.x, tr.y,
                                                            bl.x, bl.y);

                if (t.isSingularity())
                    t = {};

                setTransform (t);
            }
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (image.isValid())
        {
            if (opacity > 0.0f && ! overlayColour.isOpaque())
            {
                g.setOpacity (opacity);
                g.drawImageAt (image, 0, 0, false);
            }

            if (! overlayColour.isTransparent())
            {
                g.setColour (overlayColour.withMultipliedAlpha (opacity));
                g.drawImageAt (image, 0, 0, true);
            }
        }
        */
    }
    
    pub fn get_drawable_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return image.getBounds().toFloat();
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            return Drawable::hitTest (x, y) && image.isValid() && image.getPixelAt (x, y).getAlpha() >= 127;
        */
    }
    
    pub fn get_outline_as_path(&self) -> PathBuf {
        
        todo!();
        /*
            return {}; // not applicable for images
        */
    }
    
    pub fn set_image_internal(&mut self, image_to_use: &Image) -> bool {
        
        todo!();
        /*
            if (image != imageToUse)
        {
            image = imageToUse;
            setBounds (image.getBounds());
            setBoundingBox (image.getBounds().toFloat());
            return true;
        }

        return false;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::image);
        */
    }
}
