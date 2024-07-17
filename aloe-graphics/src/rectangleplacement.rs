crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/placement/aloe_RectanglePlacement.h]

/**
  | Flag values that can be combined and
  | used in the constructor.
  |
  */
bitflags!{

    pub struct RectanglePlacementFlags: u32 {

        /**
          | Indicates that the source rectangle's
          | left edge should be aligned with the
          | left edge of the target rectangle.
          |
          */
        const xLeft              = 1;

        /**
          | Indicates that the source rectangle's
          | right edge should be aligned with the
          | right edge of the target rectangle.
          |
          */
        const xRight             = 2;

        /**
          | Indicates that the source should be
          | placed in the centre between the left
          | and right sides of the available space.
          |
          */
        const xMid               = 4;

        /**
          | Indicates that the source's top edge
          | should be aligned with the top edge of
          | the destination rectangle.
          |
          */
        const yTop               = 8;

        /**
          | Indicates that the source's bottom
          | edge should be aligned with the bottom
          | edge of the destination rectangle.
          |
          */
        const yBottom            = 16;

        /**
          | Indicates that the source should be
          | placed in the centre between the top
          | and bottom sides of the available space.
          |
          */
        const yMid               = 32;

        /**
          | If this flag is set, then the source rectangle
          | will be resized to completely fill the
          | destination rectangle, and all other
          | flags are ignored.
          |
          */
        const stretchToFit       = 64;

        /**
          | If this flag is set, then the source rectangle
          | will be resized so that it is the minimum
          | size to completely fill the destination
          | rectangle, without changing its aspect
          | ratio. This means that some of the source
          | rectangle may fall outside the destination.
          | 
          | If this flag is not set, the source will
          | be given the maximum size at which none
          | of it falls outside the destination
          | rectangle.
          |
          */
        const fillDestination    = 128;

        /**
          | Indicates that the source rectangle
          | can be reduced in size if required, but
          | should never be made larger than its
          | original size.
          |
          */
        const onlyReduceInSize   = 256;

        /**
          | Indicates that the source rectangle
          | can be enlarged if required, but should
          | never be made smaller than its original
          | size.
          |
          */
        const onlyIncreaseInSize = 512;

        /**
          | Indicates that the source rectangle's
          | size should be left unchanged.
          |
          */
        const doNotResize        = (Self::onlyIncreaseInSize.bits() | Self::onlyReduceInSize.bits());

        /**
          | A shorthand value that is equivalent
          | to (xMid | yMid).
          |
          */
        const centred            = (Self::xMid.bits() | Self::yMid.bits());
    }
}

/**
  | Defines the method used to position
  | some kind of rectangular object within
  | a rectangular viewport.
  | 
  | Although similar to Justification,
  | this is more specific, and has some extra
  | options.
  | 
  | @tags{Graphics}
  |
  */
#[derive(Copy,Clone)]
pub struct RectanglePlacement {
    flags: i32, // default = { centred  }
}

impl Default for RectanglePlacement {
    
    /**
      | Creates a default RectanglePlacement
      | object, which is equivalent to using
      | the 'centred' flag.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl PartialEq<RectanglePlacement> for RectanglePlacement {
    
    #[inline] fn eq(&self, other: &RectanglePlacement) -> bool {
        todo!();
        /*
            return flags == other.flags;
        */
    }
}

impl Eq for RectanglePlacement {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/placement/aloe_RectanglePlacement.cpp]
impl RectanglePlacement {

    /**
      | Creates a RectanglePlacement object
      | using a combination of flags from the
      | Flags enum.
      |
      */
    pub fn new(placement_flags: i32) -> Self {
    
        todo!();
        /*
        : flags(placementFlags),
        */
    }

    /**
      | Copies another RectanglePlacement
      | object.
      |
      */
    pub fn assign_from(&mut self, _0: &RectanglePlacement) -> &mut RectanglePlacement {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the raw flags that are set for
      | this object.
      |
      */
    #[inline] pub fn get_flags(&self) -> i32 {
        
        todo!();
        /*
            return flags;
        */
    }

    /**
      | Tests a set of flags for this object.
      | 
      | -----------
      | @return
      | 
      | true if any of the flags passed in are
      | set on this object.
      |
      */
    #[inline] pub fn test_flags(&self, flags_to_test: i32) -> bool {
        
        todo!();
        /*
            return (flags & flagsToTest) != 0;
        */
    }
    
    /**
      | Returns the rectangle that should be
      | used to fit the given source rectangle
      | into the destination rectangle using
      | the current flags.
      |
      */
    pub fn applied_to<ValueType: Copy>(&self, 
        source:      &Rectangle<ValueType>,
        destination: &Rectangle<ValueType>) -> Rectangle<ValueType> {
    
        todo!();
        /*
            double x = source.getX(), y = source.getY(), w = source.getWidth(), h = source.getHeight();
            applyTo (x, y, w, h, static_cast<double> (destination.getX()), static_cast<double> (destination.getY()),
                     static_cast<double> (destination.getWidth()), static_cast<double> (destination.getHeight()));
            return Rectangle<ValueType> (static_cast<ValueType> (x), static_cast<ValueType> (y),
                                         static_cast<ValueType> (w), static_cast<ValueType> (h));
        */
    }

    /**
      | Adjusts the position and size of a rectangle
      | to fit it into a space.
      | 
      | The source rectangle coordinates will
      | be adjusted so that they fit into the
      | destination rectangle based on this
      | object's flags.
      |
      */
    pub fn apply_to(&self, 
        x:  &mut f64,
        y:  &mut f64,
        w:  &mut f64,
        h:  &mut f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64)  {
        
        todo!();
        /*
            if (w == 0.0 || h == 0.0)
            return;

        if ((flags & stretchToFit) != 0)
        {
            x = dx;
            y = dy;
            w = dw;
            h = dh;
        }
        else
        {
            double scale = (flags & fillDestination) != 0 ? jmax (dw / w, dh / h)
                                                          : jmin (dw / w, dh / h);

            if ((flags & onlyReduceInSize) != 0)
                scale = jmin (scale, 1.0);

            if ((flags & onlyIncreaseInSize) != 0)
                scale = jmax (scale, 1.0);

            w *= scale;
            h *= scale;

            if ((flags & xLeft) != 0)
                x = dx;
            else if ((flags & xRight) != 0)
                x = dx + dw - w;
            else
                x = dx + (dw - w) * 0.5;

            if ((flags & yTop) != 0)
                y = dy;
            else if ((flags & yBottom) != 0)
                y = dy + dh - h;
            else
                y = dy + (dh - h) * 0.5;
        }
        */
    }
    
    /**
      | Returns the transform that should be
      | applied to these source coordinates
      | to fit them into the destination rectangle
      | using the current flags.
      |
      */
    pub fn get_transform_to_fit(&self, 
        source:      &Rectangle<f32>,
        destination: &Rectangle<f32>) -> AffineTransform {
        
        todo!();
        /*
            if (source.isEmpty())
            return AffineTransform();

        float newX = destination.getX();
        float newY = destination.getY();

        float scaleX = destination.getWidth() / source.getWidth();
        float scaleY = destination.getHeight() / source.getHeight();

        if ((flags & stretchToFit) == 0)
        {
            scaleX = (flags & fillDestination) != 0 ? jmax (scaleX, scaleY)
                                                    : jmin (scaleX, scaleY);

            if ((flags & onlyReduceInSize) != 0)
                scaleX = jmin (scaleX, 1.0f);

            if ((flags & onlyIncreaseInSize) != 0)
                scaleX = jmax (scaleX, 1.0f);

            scaleY = scaleX;

            if ((flags & xRight) != 0)
                newX += destination.getWidth() - source.getWidth() * scaleX;             // right
            else if ((flags & xLeft) == 0)
                newX += (destination.getWidth() - source.getWidth() * scaleX) / 2.0f;    // centre

            if ((flags & yBottom) != 0)
                newY += destination.getHeight() - source.getHeight() * scaleX;             // bottom
            else if ((flags & yTop) == 0)
                newY += (destination.getHeight() - source.getHeight() * scaleX) / 2.0f;    // centre
        }

        return AffineTransform::translation (-source.getX(), -source.getY())
                    .scaled (scaleX, scaleY)
                    .translated (newX, newY);
        */
    }
}
