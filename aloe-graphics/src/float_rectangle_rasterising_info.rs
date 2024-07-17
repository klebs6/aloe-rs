crate::ix!();

/**
  | Calculates the alpha values and positions
  | for rendering the edges of a non-pixel-aligned
  | rectangle.
  | 
  | @tags{Graphics}
  |
  */
pub struct FloatRectangleRasterisingInfo {

    /**
      | bounds of the solid central area, excluding
      | anti-aliased edges
      |
      */
    left:   i32,
    top:    i32,
    right:  i32,
    bottom: i32,

    /**
      | bounds of the total area, including
      | edges
      |
      */
    total_top:    i32,
    total_left:   i32,
    total_bottom: i32,
    total_right:  i32,

    /**
      | alpha of each anti-aliased edge
      |
      */
    top_alpha:    i32,
    left_alpha:   i32,
    bottom_alpha: i32,
    right_alpha:  i32,
}

impl FloatRectangleRasterisingInfo {

    pub fn new(area: Rectangle<f32>) -> Self {
    
        todo!();
        /*


            : left   (roundToInt (256.0f * area.getX())),
              top    (roundToInt (256.0f * area.getY())),
              right  (roundToInt (256.0f * area.getRight())),
              bottom (roundToInt (256.0f * area.getBottom()))
            if ((top >> 8) == (bottom >> 8))
            {
                topAlpha = bottom - top;
                bottomAlpha = 0;
                totalTop = top >> 8;
                totalBottom = bottom = top = totalTop + 1;
            }
            else
            {
                if ((top & 255) == 0)
                {
                    topAlpha = 0;
                    top = totalTop = (top >> 8);
                }
                else
                {
                    topAlpha = 255 - (top & 255);
                    totalTop = (top >> 8);
                    top = totalTop + 1;
                }

                bottomAlpha = bottom & 255;
                bottom >>= 8;
                totalBottom = bottom + (bottomAlpha != 0 ? 1 : 0);
            }

            if ((left >> 8) == (right >> 8))
            {
                leftAlpha = right - left;
                rightAlpha = 0;
                totalLeft = (left >> 8);
                totalRight = right = left = totalLeft + 1;
            }
            else
            {
                if ((left & 255) == 0)
                {
                    leftAlpha = 0;
                    left = totalLeft = (left >> 8);
                }
                else
                {
                    leftAlpha = 255 - (left & 255);
                    totalLeft = (left >> 8);
                    left = totalLeft + 1;
                }

                rightAlpha = right & 255;
                right >>= 8;
                totalRight = right + (rightAlpha != 0 ? 1 : 0);
            }
        */
    }
    
    
    pub fn iterate<Callback>(&self, callback: &mut Callback)  {
    
        todo!();
        /*
            if (topAlpha != 0)       callback (totalLeft, totalTop, totalRight - totalLeft, 1, topAlpha);
            if (bottomAlpha != 0)    callback (totalLeft, bottom,   totalRight - totalLeft, 1, bottomAlpha);
            if (leftAlpha != 0)      callback (totalLeft, totalTop, 1, totalBottom - totalTop, leftAlpha);
            if (rightAlpha != 0)     callback (right,     totalTop, 1, totalBottom - totalTop, rightAlpha);

            callback (left, top, right - left, bottom - top, 255);
        */
    }
    
    #[inline] pub fn is_one_pixel_wide(&self) -> bool {
        
        todo!();
        /*
            return right - left == 1 && leftAlpha + rightAlpha == 0;
        */
    }
    
    #[inline] pub fn get_top_left_corner_alpha(&self) -> i32 {
        
        todo!();
        /*
            return (topAlpha * leftAlpha) >> 8;
        */
    }
    
    #[inline] pub fn get_top_right_corner_alpha(&self) -> i32 {
        
        todo!();
        /*
            return (topAlpha * rightAlpha) >> 8;
        */
    }
    
    #[inline] pub fn get_bottom_left_corner_alpha(&self) -> i32 {
        
        todo!();
        /*
            return (bottomAlpha * leftAlpha) >> 8;
        */
    }
    
    #[inline] pub fn get_bottom_right_corner_alpha(&self) -> i32 {
        
        todo!();
        /*
            return (bottomAlpha * rightAlpha) >> 8;
        */
    }
}
