crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_FillType.h]

/**
  | Represents a colour or fill pattern
  | to use for rendering paths.
  | 
  | This is used by the Graphics and DrawablePath
  | classes as a way to encapsulate a brush
  | type. It can either be a solid colour,
  | a gradient, or a tiled image.
  | 
  | @see Graphics::setFillType, DrawablePath::setFill
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct FillType {

    /**
      | The solid colour being used.
      | 
      | If the fill type is not a solid colour,
      | the alpha channel of this colour indicates
      | the opacity that should be used for the
      | fill, and the RGB channels are ignored.
      |
      */
    colour:         Colour,

    /**
      | Returns the gradient that should be
      | used for filling.
      | 
      | This will be nullptr if the object is
      | some other type of fill.
      | 
      | If a gradient is active, the overall
      | opacity with which it should be applied
      | is indicated by the alpha channel of
      | the colour variable.
      |
      */
    gradient:       Box<ColourGradient>,

    /**
      | The image that should be used for tiling.
      | 
      | If an image fill is active, the overall
      | opacity with which it should be applied
      | is indicated by the alpha channel of
      | the colour variable.
      |
      */
    image:          Image,

    /**
      | The transform that should be applied
      | to the image or gradient that's being
      | drawn.
      |
      */
    transform:      AffineTransform,
}

impl Default for FillType {
    
    /**
      | Creates a default fill type, of solid
      | black.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*

            : colour (0xff000000)
        */
    }
}

impl PartialEq<FillType> for FillType {
    
    #[inline] fn eq(&self, other: &FillType) -> bool {
        todo!();
        /*
            return colour == other.colour && image == other.image
                && transform == other.transform
                && (gradient == other.gradient
                     || (gradient != nullptr && other.gradient != nullptr && *gradient == *other.gradient));
        */
    }
}

impl Eq for FillType {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_FillType.cpp]

impl FillType {

    /**
      | Returns true if this is a solid colour
      | fill, and not a gradient or image.
      |
      */
    pub fn is_colour(&self) -> bool {
        
        todo!();
        /*
            return gradient == nullptr && image.isNull();
        */
    }

    /**
      | Returns true if this is a gradient fill.
      |
      */
    pub fn is_gradient(&self) -> bool {
        
        todo!();
        /*
            return gradient != nullptr;
        */
    }

    /**
      | Returns true if this is a tiled image
      | pattern fill.
      |
      */
    pub fn is_tiled_image(&self) -> bool {
        
        todo!();
        /*
            return image.isValid();
        */
    }

    /**
      | Returns the current opacity to be applied
      | to the colour, gradient, or image. @see
      | setOpacity
      |
      */
    pub fn get_opacity(&self) -> f32 {
        
        todo!();
        /*
            return colour.getFloatAlpha();
        */
    }

    
    /**
      | Creates a fill type of a solid colour.
      | @see setColour
      |
      */
    pub fn new_from_colour(c: Colour) -> Self {
    
        todo!();
        /*
        : colour(c),

        
        */
    }
    
    /**
      | Creates a gradient fill type. @see setGradient
      |
      */
    pub fn new_from_colour_gradient_ref(g: &ColourGradient) -> Self {
    
        todo!();
        /*

            : colour (0xff000000), gradient (new ColourGradient (g))
        */
    }
    
    /**
      | Creates a gradient fill type. @see setGradient
      |
      */
    pub fn new_from_colour_gradient(g: ColourGradient) -> Self {
    
        todo!();
        /*

            : colour (0xff000000), gradient (new ColourGradient (std::move (g)))
        */
    }
    
    /**
      | Creates a tiled image fill type. The
      | transform allows you to set the scaling,
      | offset and rotation of the pattern.
      | @see setTiledImage
      |
      */
    pub fn new_from_image_and_affine_transform(
        im: &Image,
        t:  &AffineTransform) -> Self {
    
        todo!();
        /*

            : colour (0xff000000), image (im), transform (t)
        */
    }
    
    /**
      | Creates a copy of another FillType.
      |
      */
    pub fn new_from_filltype_ref(other: &FillType) -> Self {
    
        todo!();
        /*
        : colour(other.colour),
        : gradient(createCopyIfNotNull (other.gradient.get())),
        : image(other.image),
        : transform(other.transform),

        
        */
    }
    
    /**
      | Makes a copy of another FillType.
      |
      */
    pub fn assign_from_ref(&mut self, other: &FillType) -> &mut FillType {
        
        todo!();
        /*
            if (this != &other)
        {
            colour = other.colour;
            gradient.reset (createCopyIfNotNull (other.gradient.get()));
            image = other.image;
            transform = other.transform;
        }

        return *this;
        */
    }
    
    pub fn new_from_filltype(other: FillType) -> Self {
    
        todo!();
        /*
        : colour(other.colour),
        : gradient(std::move (other.gradient)),
        : image(std::move (other.image)),
        : transform(other.transform),

        
        */
    }
    
    pub fn assign_from(&mut self, other: FillType) -> &mut FillType {
        
        todo!();
        /*
            jassert (this != &other); // hopefully the compiler should make this situation impossible!

        colour = other.colour;
        gradient = std::move (other.gradient);
        image = std::move (other.image);
        transform = other.transform;
        return *this;
        */
    }
    
    /**
      | Turns this object into a solid colour
      | fill.
      | 
      | If the object was an image or gradient,
      | those fields will no longer be valid.
      |
      */
    pub fn set_colour(&mut self, new_colour: Colour)  {
        
        todo!();
        /*
            gradient.reset();
        image = {};
        colour = newColour;
        */
    }
    
    /**
      | Turns this object into a gradient fill.
      |
      */
    pub fn set_gradient(&mut self, new_gradient: &ColourGradient)  {
        
        todo!();
        /*
            if (gradient != nullptr)
        {
            *gradient = newGradient;
        }
        else
        {
            image = {};
            gradient.reset (new ColourGradient (newGradient));
            colour = Colours::black;
        }
        */
    }
    
    /**
      | Turns this object into a tiled image
      | fill type. The transform allows you
      | to set the scaling, offset and rotation
      | of the pattern.
      |
      */
    pub fn set_tiled_image(&mut self, 
        new_image:     &Image,
        new_transform: &AffineTransform)  {
        
        todo!();
        /*
            gradient.reset();
        image = newImage;
        transform = newTransform;
        colour = Colours::black;
        */
    }
    
    /**
      | Changes the opacity that should be used.
      | 
      | If the fill is a solid colour, this just
      | changes the opacity of that colour.
      | For gradients and image tiles, it changes
      | the opacity that will be used for them.
      |
      */
    pub fn set_opacity(&mut self, new_opacity: f32)  {
        
        todo!();
        /*
            colour = colour.withAlpha (newOpacity);
        */
    }
    
    /**
      | Returns true if this fill type is completely
      | transparent.
      |
      */
    pub fn is_invisible(&self) -> bool {
        
        todo!();
        /*
            return colour.isTransparent() || (gradient != nullptr && gradient->isInvisible());
        */
    }
    
    /**
      | Returns a copy of this fill, adding the
      | specified transform applied to the
      | existing transform.
      |
      */
    pub fn transformed(&self, t: &AffineTransform) -> FillType {
        
        todo!();
        /*
            FillType f (*this);
        f.transform = f.transform.followedBy (t);
        return f;
        */
    }
}
