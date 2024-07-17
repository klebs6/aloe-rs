crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/placement/aloe_Justification.h]

/**
  | Flag values that can be combined and
  | used in the constructor.
  |
  */
pub enum JustificationFlags
{
    /**
      | Indicates that the item should be aligned
      | against the left edge of the available
      | space.
      |
      */
    left                            = 1,

    /**
      | Indicates that the item should be aligned
      | against the right edge of the available
      | space.
      |
      */
    right                           = 2,

    /**
      | Indicates that the item should be placed
      | in the centre between the left and right
      | sides of the available space.
      |
      */
    horizontallyCentred             = 4,

    /**
      | Indicates that the item should be aligned
      | against the top edge of the available
      | space.
      |
      */
    top                             = 8,

    /**
      | Indicates that the item should be aligned
      | against the bottom edge of the available
      | space.
      |
      */
    bottom                          = 16,

    /**
      | Indicates that the item should be placed
      | in the centre between the top and bottom
      | sides of the available space.
      |
      */
    verticallyCentred               = 32,

    /**
      | Indicates that lines of text should
      | be spread out to fill the maximum width
      | available, so that both margins are
      | aligned vertically.
      |
      */
    horizontallyJustified           = 64,

    /**
      | Indicates that the item should be centred
      | vertically and horizontally. This
      | is equivalent to (horizontallyCentred | verticallyCentred)
      |
      */
    centred                         = 36,

    /**
      | Indicates that the item should be centred
      | vertically but placed on the left hand
      | side. This is equivalent to (left | verticallyCentred)
      |
      */
    centredLeft                     = 33,

    /**
      | Indicates that the item should be centred
      | vertically but placed on the right hand
      | side. This is equivalent to (right | verticallyCentred)
      |
      */
    centredRight                    = 34,

    /**
      | Indicates that the item should be centred
      | horizontally and placed at the top.
      | This is equivalent to (horizontallyCentred | top)
      |
      */
    centredTop                      = 12,

    /**
      | Indicates that the item should be centred
      | horizontally and placed at the bottom.
      | This is equivalent to (horizontallyCentred | bottom)
      |
      */
    centredBottom                   = 20,

    /**
      | Indicates that the item should be placed
      | in the top-left corner. This is equivalent
      | to (left | top)
      |
      */
    topLeft                         = 9,

    /**
      | Indicates that the item should be placed
      | in the top-right corner. This is equivalent
      | to (right | top)
      |
      */
    topRight                        = 10,

    /**
      | Indicates that the item should be placed
      | in the bottom-left corner. This is equivalent
      | to (left | bottom)
      |
      */
    bottomLeft                      = 17,

    /**
      | Indicates that the item should be placed
      | in the bottom-left corner. This is equivalent
      | to (right | bottom)
      |
      */
    bottomRight                     = 18
}

/**
  | Represents a type of justification
  | to be used when positioning graphical
  | items.
  | 
  | e.g. it indicates whether something
  | should be placed top-left, top-right,
  | centred, etc.
  | 
  | It is used in various places wherever
  | this kind of information is needed.
  | 
  | @tags{Graphics}
  |
  */
#[derive(Copy,Clone)]
pub struct Justification {
    flags: i32,
}

impl PartialEq<Justification> for Justification {
    
    #[inline] fn eq(&self, other: &Justification) -> bool {
        todo!();
        /*
            return flags == other.flags;
        */
    }
}

impl Eq for Justification {}

impl Justification {

    /**
      | Creates a Justification object using
      | a combination of flags from the Flags
      | enum.
      |
      */
    pub fn new(justification_flags: i32) -> Self {
    
        todo!();
        /*
        : flags(justificationFlags),

        
        */
    }

    /**
      | Returns the raw flags that are set for
      | this Justification object.
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
      | Returns just the flags from this object
      | that deal with vertical layout.
      |
      */
    pub fn get_only_vertical_flags(&self) -> i32 {
        
        todo!();
        /*
            return flags & (top | bottom | verticallyCentred);
        */
    }

    /**
      | Returns just the flags from this object
      | that deal with horizontal layout.
      |
      */
    pub fn get_only_horizontal_flags(&self) -> i32 {
        
        todo!();
        /*
            return flags & (left | right | horizontallyCentred | horizontallyJustified);
        */
    }
    
    /**
      | Adjusts the position of a rectangle
      | to fit it into a space.
      | 
      | The (x, y) position of the rectangle
      | will be updated to position it inside
      | the given space according to the justification
      | flags.
      |
      */
    pub fn apply_to_rectangle<ValueType>(
        &self, 
        x:      &mut ValueType,
        y:      &mut ValueType,
        w:      ValueType,
        h:      ValueType,
        spacex: ValueType,
        spacey: ValueType,
        spacew: ValueType,
        spaceh: ValueType)  {
    
        todo!();
        /*
            x = spaceX;
            if ((flags & horizontallyCentred) != 0)     x += (spaceW - w) / (ValueType) 2;
            else if ((flags & right) != 0)              x += spaceW - w;

            y = spaceY;
            if ((flags & verticallyCentred) != 0)       y += (spaceH - h) / (ValueType) 2;
            else if ((flags & bottom) != 0)             y += spaceH - h;
        */
    }

    /**
      | Returns the new position of a rectangle
      | that has been justified to fit within
      | a given space.
      |
      */
    pub fn applied_to_rectangle<ValueType>(
        &self, 
        area_to_adjust: &Rectangle<ValueType>,
        target_space:   &Rectangle<ValueType>

    ) -> Rectangle<ValueType> 

        where ValueType: Copy + Clone,
    {
    
        todo!();
        /*
            ValueType x = areaToAdjust.getX(), y = areaToAdjust.getY();
            applyToRectangle (x, y, areaToAdjust.getWidth(), areaToAdjust.getHeight(),
                              targetSpace.getX(), targetSpace.getY(), targetSpace.getWidth(), targetSpace.getHeight());
            return areaToAdjust.withPosition (x, y);
        */
    }
}
