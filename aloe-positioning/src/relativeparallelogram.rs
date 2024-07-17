crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeParallelogram.h]

/**
  | A parallelogram defined by three RelativePoint
  | positions.
  | 
  | @see RelativePoint, RelativeCoordinate
  | 
  | @tags{GUI}
  |
  */
pub struct RelativeParallelogram {
    top_left:    RelativePoint,
    top_right:   RelativePoint,
    bottom_left: RelativePoint,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativeParallelogram.cpp]

impl PartialEq<RelativeParallelogram> for RelativeParallelogram {
    
    #[inline] fn eq(&self, other: &RelativeParallelogram) -> bool {
        todo!();
        /*
            return topLeft == other.topLeft && topRight == other.topRight && bottomLeft == other.bottomLeft;
        */

    }
}

impl Eq for RelativeParallelogram {}

impl From<&Rectangle<f32>> for RelativeParallelogram {

    fn from(r: &Rectangle<f32>) -> Self {
    
        todo!();
        /*
        : top_left(r.getTopLeft()),
        : top_right(r.getTopRight()),
        : bottom_left(r.getBottomLeft()),

        
        */

    }
}
    
impl RelativeParallelogram {

    pub fn new_from_3_relative_points(
        top_left:    &RelativePoint,
        top_right:   &RelativePoint,
        bottom_left: &RelativePoint

    ) -> Self {
    
        todo!();
        /*
        : top_left(topLeft_),
        : top_right(topRight_),
        : bottom_left(bottomLeft_),

        
        */

    }
    
    pub fn new(
        top_left:    &String,
        top_right:   &String,
        bottom_left: &String) -> Self {
    
        todo!();
        /*
        : top_left(topLeft_),
        : top_right(topRight_),
        : bottom_left(bottomLeft_),

        
        */

    }
    
    pub fn resolve_three_points(
        &self, 
        points: *mut Point<f32>,
        scope:  *mut dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            points[0] = topLeft.resolve (scope);
        points[1] = topRight.resolve (scope);
        points[2] = bottomLeft.resolve (scope);
        */

    }
    
    pub fn resolve_four_corners(
        &self, 
        points: *mut Point<f32>,
        scope:  *mut dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            resolveThreePoints (points, scope);
        points[3] = points[1] + (points[2] - points[0]);
        */

    }
    
    pub fn get_bounds(&self, scope: *mut dyn ExpressionScopeInterface) -> Rectangle<f32> {
        
        todo!();
        /*
            Point<float> points[4];
        resolveFourCorners (points, scope);
        return Rectangle<float>::findAreaContainingPoints (points, 4);
        */

    }
    
    pub fn get_path(
        &self, 
        path:  &mut Path,
        scope: *mut dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            Point<float> points[4];
        resolveFourCorners (points, scope);

        path.startNewSubPath (points[0]);
        path.lineTo (points[1]);
        path.lineTo (points[3]);
        path.lineTo (points[2]);
        path.closeSubPath();
        */

    }
    
    pub fn reset_to_perpendicular(&mut self, scope: *mut dyn ExpressionScopeInterface) -> AffineTransform {
        
        todo!();
        /*
            Point<float> corners[3];
        resolveThreePoints (corners, scope);

        const Line<float> top (corners[0], corners[1]);
        const Line<float> left (corners[0], corners[2]);
        const Point<float> newTopRight (corners[0] + Point<float> (top.getLength(), 0.0f));
        const Point<float> newBottomLeft (corners[0] + Point<float> (0.0f, left.getLength()));

        topRight.moveToAbsolute (newTopRight, scope);
        bottomLeft.moveToAbsolute (newBottomLeft, scope);

        return AffineTransform::fromTargetPoints (corners[0], corners[0],
                                                  corners[1], newTopRight,
                                                  corners[2], newBottomLeft);
        */

    }
    
    pub fn is_dynamic(&self) -> bool {
        
        todo!();
        /*
            return topLeft.isDynamic() || topRight.isDynamic() || bottomLeft.isDynamic();
        */

    }
    
    pub fn get_internal_coord_for_point(
        &mut self, 
        corners: *const Point<f32>,
        target:  Point<f32>

    ) -> Point<f32> {
        
        todo!();
        /*
            const Point<float> tr (corners[1] - corners[0]);
        const Point<float> bl (corners[2] - corners[0]);
        target -= corners[0];

        return Point<float> (Line<float> (Point<float>(), tr).getIntersection (Line<float> (target, target - bl)).getDistanceFromOrigin(),
                             Line<float> (Point<float>(), bl).getIntersection (Line<float> (target, target - tr)).getDistanceFromOrigin());
        */

    }
    
    pub fn get_point_for_internal_coord(
        &mut self, 
        corners: *const Point<f32>,
        point:   Point<f32>

    ) -> Point<f32> {
        
        todo!();
        /*
            return corners[0]
                + Line<float> (Point<float>(), corners[1] - corners[0]).getPointAlongLine (point.x)
                + Line<float> (Point<float>(), corners[2] - corners[0]).getPointAlongLine (point.y);
        */

    }
    
    pub fn get_bounding_box(&mut self, p: *const Point<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            const Point<float> points[] = { p[0], p[1], p[2], p[1] + (p[2] - p[0]) };
        return Rectangle<float>::findAreaContainingPoints (points, 4);
        */

    }
}
