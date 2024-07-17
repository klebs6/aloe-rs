crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_BorderSize.h]

/**
  | Specifies a set of gaps to be left around
  | the sides of a rectangle.
  | 
  | This is basically the size of the spaces
  | at the top, bottom, left and right of
  | a rectangle. It's used by various component
  | classes to specify borders.
  | 
  | @see Rectangle
  | 
  | @tags{Graphics}
  |
  */
#[derive(Copy,Clone)]
pub struct BorderSize<ValueType: Copy + Clone> {
    top:    ValueType,
    left:   ValueType,
    bottom: ValueType,
    right:  ValueType,
}

impl<ValueType: Copy + Clone> Default for BorderSize<ValueType> {
    
    /**
      | Creates a null border.
      | 
      | All sizes are left as 0.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

impl<ValueType: Copy + Clone> PartialEq<BorderSize<ValueType>> for BorderSize<ValueType> {
    
    #[inline] fn eq(&self, other: &BorderSize<ValueType>) -> bool {
        todo!();
        /*
            return top == other.top && left == other.left && bottom == other.bottom && right == other.right;
        */
    }
}

impl<ValueType: Copy + Clone> Eq for BorderSize<ValueType> {}

impl<ValueType: Copy + Clone> BorderSize<ValueType> {

    /**
      | Creates a border with the given gaps.
      |
      */
    pub fn new_from_gaps(
        top_gap:    ValueType,
        left_gap:   ValueType,
        bottom_gap: ValueType,
        right_gap:  ValueType) -> Self {
    
        todo!();
        /*
        : top(topGap),
        : left(leftGap),
        : bottom(bottomGap),
        : right(rightGap),

        
        */
    }

    /**
      | Creates a border with the given gap on
      | all sides.
      |
      */
    pub fn new(all_gaps: ValueType) -> Self {
    
        todo!();
        /*
        : top(allGaps),
        : left(allGaps),
        : bottom(allGaps),
        : right(allGaps),

        
        */
    }
    
    /**
      | Returns the gap that should be left at
      | the top of the region.
      |
      */
    pub fn get_top(&self) -> ValueType {
        
        todo!();
        /*
            return top;
        */
    }

    /**
      | Returns the gap that should be left at
      | the left of the region.
      |
      */
    pub fn get_left(&self) -> ValueType {
        
        todo!();
        /*
            return left;
        */
    }

    /**
      | Returns the gap that should be left at
      | the bottom of the region.
      |
      */
    pub fn get_bottom(&self) -> ValueType {
        
        todo!();
        /*
            return bottom;
        */
    }

    /**
      | Returns the gap that should be left at
      | the right of the region.
      |
      */
    pub fn get_right(&self) -> ValueType {
        
        todo!();
        /*
            return right;
        */
    }

    /**
      | Returns the sum of the top and bottom
      | gaps.
      |
      */
    pub fn get_top_and_bottom(&self) -> ValueType {
        
        todo!();
        /*
            return top + bottom;
        */
    }

    /**
      | Returns the sum of the left and right
      | gaps.
      |
      */
    pub fn get_left_and_right(&self) -> ValueType {
        
        todo!();
        /*
            return left + right;
        */
    }

    /**
      | Returns true if this border has no thickness
      | along any edge.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return left + right + top + bottom == ValueType();
        */
    }

    /**
      | Changes the top gap.
      |
      */
    pub fn set_top(&mut self, new_top_gap: ValueType)  {
        
        todo!();
        /*
            top = newTopGap;
        */
    }

    /**
      | Changes the left gap.
      |
      */
    pub fn set_left(&mut self, new_left_gap: ValueType)  {
        
        todo!();
        /*
            left = newLeftGap;
        */
    }

    /**
      | Changes the bottom gap.
      |
      */
    pub fn set_bottom(&mut self, new_bottom_gap: ValueType)  {
        
        todo!();
        /*
            bottom = newBottomGap;
        */
    }

    /**
      | Changes the right gap.
      |
      */
    pub fn set_right(&mut self, new_right_gap: ValueType)  {
        
        todo!();
        /*
            right = newRightGap;
        */
    }
    
    /**
      | Returns a rectangle with these borders
      | removed from it.
      |
      */
    pub fn subtracted_from(&self, original: &Rectangle<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return Rectangle<ValueType> (original.getX() + left,
                                         original.getY() + top,
                                         original.getWidth() - (left + right),
                                         original.getHeight() - (top + bottom));
        */
    }

    /**
      | Removes this border from a given rectangle.
      |
      */
    pub fn subtract_from(&self, rectangle: &mut Rectangle<ValueType>)  {
        
        todo!();
        /*
            rectangle = subtractedFrom (rectangle);
        */
    }

    /**
      | Returns a rectangle with these borders
      | added around it.
      |
      */
    pub fn added_to(&self, original: &Rectangle<ValueType>) -> Rectangle<ValueType> {
        
        todo!();
        /*
            return Rectangle<ValueType> (original.getX() - left,
                                         original.getY() - top,
                                         original.getWidth() + (left + right),
                                         original.getHeight() + (top + bottom));
        */
    }

    /**
      | Adds this border around a given rectangle.
      |
      */
    pub fn add_to(&self, rectangle: &mut Rectangle<ValueType>)  {
        
        todo!();
        /*
            rectangle = addedTo (rectangle);
        */
    }
}
