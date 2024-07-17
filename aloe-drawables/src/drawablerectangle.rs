crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableRectangle.h]

/**
  | A Drawable object which draws a rectangle.
  | 
  | For details on how to change the fill
  | and stroke, see the DrawableShape class.
  | 
  | @see Drawable, DrawableShape
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
#[derive(Default)]
pub struct DrawableRectangle<'a> {
    base:        DrawableShape<'a>,
    bounds:      Parallelogram<f32>,
    corner_size: Point<f32>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableRectangle.cpp]
impl<'a> DrawableRectangle<'a> {
    
    /**
      | Returns the rectangle's bounds.
      |
      */
    pub fn get_rectangle(&self) -> Parallelogram<f32> {
        
        todo!();
        /*
            return bounds;
        */
    }

    /**
      | Returns the corner size to be used.
      |
      */
    pub fn get_corner_size(&self) -> Point<f32> {
        
        todo!();
        /*
            return cornerSize;
        */
    }

    pub fn new(other: &DrawableRectangle) -> Self {
    
        todo!();
        /*
        : drawable_shape(other),
        : bounds(other.bounds),
        : corner_size(other.cornerSize),

            rebuildPath();
        */
    }
    
    pub fn create_copy(&self) -> Box<Drawable> {
        
        todo!();
        /*
            return std::make_unique<DrawableRectangle> (*this);
        */
    }
    
    /**
      | Sets the rectangle's bounds.
      |
      */
    pub fn set_rectangle(&mut self, new_bounds: Parallelogram<f32>)  {
        
        todo!();
        /*
            if (bounds != newBounds)
        {
            bounds = newBounds;
            rebuildPath();
        }
        */
    }
    
    /**
      | Sets a new corner size for the rectangle
      |
      */
    pub fn set_corner_size(&mut self, new_size: Point<f32>)  {
        
        todo!();
        /*
            if (cornerSize != newSize)
        {
            cornerSize = newSize;
            rebuildPath();
        }
        */
    }
    
    pub fn rebuild_path(&mut self)  {
        
        todo!();
        /*
            auto w = bounds.getWidth();
        auto h = bounds.getHeight();

        Path newPath;

        if (cornerSize.x > 0 && cornerSize.y > 0)
            newPath.addRoundedRectangle (0, 0, w, h, cornerSize.x, cornerSize.y);
        else
            newPath.addRectangle (0, 0, w, h);

        newPath.applyTransform (AffineTransform::fromTargetPoints (Point<float>(),       bounds.topLeft,
                                                                   Point<float> (w, 0),  bounds.topRight,
                                                                   Point<float> (0, h),  bounds.bottomLeft));

        if (path != newPath)
        {
            path.swapWithPath (newPath);
            pathChanged();
        }
        */
    }
}
