crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativePointPath.h]

/**
  | A path object that consists of RelativePoint
  | coordinates rather than the normal
  | fixed ones.
  | 
  | One of these paths can be converted into
  | a Path object for drawing and manipulation,
  | but unlike a Path, its points can be dynamic
  | instead of just fixed.
  | 
  | @see RelativePoint, RelativeCoordinate
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct RelativePointPath {
    elements:                Vec<Box<RelativePointPathElementBase>>,
    uses_non_zero_winding:   bool,
    contains_dynamic_points: bool,
}

impl PartialEq<RelativePointPath> for RelativePointPath {
    
    #[inline] fn eq(&self, other: &RelativePointPath) -> bool {
        todo!();
        /*
            if (elements.size() != other.elements.size()
             || usesNonZeroWinding != other.usesNonZeroWinding
             || containsDynamicPoints != other.containsDynamicPoints)
            return false;

        for (int i = 0; i < elements.size(); ++i)
        {
            RelativePointPathElementBase* const e1 = elements.getUnchecked(i);
            RelativePointPathElementBase* const e2 = other.elements.getUnchecked(i);

            if (e1->type != e2->type)
                return false;

            int numPoints1, numPoints2;
            const RelativePoint* const points1 = e1->getControlPoints (numPoints1);
            const RelativePoint* const points2 = e2->getControlPoints (numPoints2);

            jassert (numPoints1 == numPoints2);

            for (int j = numPoints1; --j >= 0;)
                if (points1[j] != points2[j])
                    return false;
        }

        return true;
        */

    }
}

impl Eq for RelativePointPath {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/positioning/aloe_RelativePointPath.cpp]
impl Default for RelativePointPath {

    fn default() -> Self {
    
        todo!();
        /*
        : uses_non_zero_winding(true),
        : contains_dynamic_points(false),

        
        */

    }
}

impl From<&RelativePointPath> for RelativePointPath {

    fn from(other: &RelativePointPath) -> Self {
    
        todo!();
        /*
        : uses_non_zero_winding(true),
        : contains_dynamic_points(false),

            for (int i = 0; i < other.elements.size(); ++i)
            elements.add (other.elements.getUnchecked(i)->clone());
        */

    }
}

impl From<&Path> for RelativePointPath {

    fn from(path: &Path) -> Self {
    
        todo!();
        /*
        : uses_non_zero_winding(path.isUsingNonZeroWinding()),
        : contains_dynamic_points(false),

            for (Path::Iterator i (path); i.next();)
        {
            switch (i.elementType)
            {
                case Path::Iterator::startNewSubPath:   elements.add (new RelativePointPathStartSubPath (RelativePoint (i.x1, i.y1))); break;
                case Path::Iterator::lineTo:            elements.add (new RelativePointPathLineTo (RelativePoint (i.x1, i.y1))); break;
                case Path::Iterator::quadraticTo:       elements.add (new RelativePointPathQuadraticTo (RelativePoint (i.x1, i.y1), RelativePoint (i.x2, i.y2))); break;
                case Path::Iterator::cubicTo:           elements.add (new RelativePointPathCubicTo (RelativePoint (i.x1, i.y1), RelativePoint (i.x2, i.y2), RelativePoint (i.x3, i.y3))); break;
                case Path::Iterator::closePath:         elements.add (new RelativePointPathCloseSubPath()); break;
                default:                                jassertfalse; break;
            }
        }
        */

    }
}

impl RelativePointPath {
    
    pub fn apply_to(&self, path: &mut DrawablePath)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Quickly swaps the contents of this path
      | with another.
      |
      */
    pub fn swap_with(&mut self, other: &mut RelativePointPath)  {
        
        todo!();
        /*
            elements.swapWith (other.elements);
        std::swap (usesNonZeroWinding, other.usesNonZeroWinding);
        std::swap (containsDynamicPoints, other.containsDynamicPoints);
        */

    }
    
    /**
      | Resolves this points in this path and
      | adds them to a normal Path object.
      |
      */
    pub fn create_path(
        &self, 
        path:  &mut Path,
        scope: *mut dyn ExpressionScopeInterface

    )  {
        
        todo!();
        /*
            for (int i = 0; i < elements.size(); ++i)
            elements.getUnchecked(i)->addToPath (path, scope);
        */

    }
    
    /**
      | Returns true if the path contains any
      | non-fixed points.
      |
      */
    pub fn contains_any_dynamic_points(&self) -> bool {
        
        todo!();
        /*
            return containsDynamicPoints;
        */

    }
    
    pub fn add_element(&mut self, new_element: *mut RelativePointPathElementBase)  {
        
        todo!();
        /*
            if (newElement != nullptr)
        {
            elements.add (newElement);
            containsDynamicPoints = containsDynamicPoints || newElement->isDynamic();
        }
        */

    }
}
