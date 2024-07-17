crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableComposite.h]

/**
  | A drawable object which acts as a container
  | for a set of other Drawables.
  | 
  | -----------
  | @note
  | 
  | although this is a Component, it takes
  | ownership of its child components and
  | will delete them, so that you can use
  | it as a self-contained graphic object.
  | 
  | The intention is that you should not
  | add your own components to it, only add
  | other
  | 
  | Drawable objects.
  | 
  | @see Drawable
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct DrawableComposite<'a> {
    base:                    Drawable<'a>,
    bounds:                  Parallelogram<f32>,
    content_area:            Rectangle<f32>,
    update_bounds_reentrant: bool, // default = false
}
    
impl<'a> Default for DrawableComposite<'a> {
    
    /**
      | Creates a composite Drawable.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : bounds ({ 0.0f, 0.0f, 100.0f, 100.0f })

        setContentArea ({ 0.0f, 0.0f, 100.0f, 100.0f });
        */
    }
}

impl<'a> Drop for DrawableComposite<'a> {

    fn drop(&mut self) {

        todo!();
        /* 
        deleteAllChildren();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawableComposite.cpp]
impl<'a> DrawableComposite<'a> {

    /**
      | Returns the parallelogram that defines
      | the target position of the content rectangle
      | when the drawable is rendered. @see
      | setBoundingBox
      |
      */
    pub fn get_bounding_box(&self) -> Parallelogram<f32> {
        
        todo!();
        /*
            return bounds;
        */
    }

    /**
      | Returns the main content rectangle.
      | @see contentLeftMarkerName, contentRightMarkerName,
      | contentTopMarkerName, contentBottomMarkerName
      |
      */
    pub fn get_content_area(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return contentArea;
        */
    }

    /**
      | Creates a copy of a DrawableComposite.
      |
      */
    pub fn new(other: &DrawableComposite) -> Self {
    
        todo!();
        /*
        : drawable(other),
        : bounds(other.bounds),
        : content_area(other.contentArea),

            for (auto* c : other.getChildren())
            if (auto* d = dynamic_cast<const Drawable*> (c))
                addAndMakeVisible (d->createCopy().release());
        */
    }
    
    pub fn create_copy(&self) -> Box<Drawable> {
        
        todo!();
        /*
            return std::make_unique<DrawableComposite> (*this);
        */
    }
    
    pub fn get_drawable_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            Rectangle<float> r;

        for (auto* c : getChildren())
            if (auto* d = dynamic_cast<const Drawable*> (c))
                r = r.getUnion (d->isTransformed() ? d->getDrawableBounds().transformedBy (d->getTransform())
                                                   : d->getDrawableBounds());

        return r;
        */
    }
    
    /**
      | Changes the main content area. @see
      | setBoundingBox, contentLeftMarkerName,
      | contentRightMarkerName, contentTopMarkerName,
      | contentBottomMarkerName
      |
      */
    pub fn set_content_area(&mut self, new_area: Rectangle<f32>)  {
        
        todo!();
        /*
            contentArea = newArea;
        */
    }
    
    /**
      | Sets the rectangle that defines the
      | target position of the content rectangle
      | when the drawable is rendered. @see
      | setContentArea
      |
    */
    pub fn set_bounding_box_with_rect(&mut self, new_bounds: Rectangle<f32>)  {
        
        todo!();
        /*
            setBoundingBox (Parallelogram<float> (newBounds));
        */
    }
    
    /**
      | Sets the parallelogram that defines
      | the target position of the content rectangle
      | when the drawable is rendered. @see
      | setContentArea
      |
      */
    pub fn set_bounding_box(&mut self, new_bounds: Parallelogram<f32>)  {
        
        todo!();
        /*
            if (bounds != newBounds)
        {
            bounds = newBounds;

            auto t = AffineTransform::fromTargetPoints (contentArea.getTopLeft(),     bounds.topLeft,
                                                        contentArea.getTopRight(),    bounds.topRight,
                                                        contentArea.getBottomLeft(),  bounds.bottomLeft);

            if (t.isSingularity())
                t = {};

            setTransform (t);
        }
        */
    }
    
    /**
      | Changes the bounding box transform
      | to match the content area, so that any
      | sub-items will be drawn at their untransformed
      | positions.
      |
      */
    pub fn reset_bounding_box_to_content_area(&mut self)  {
        
        todo!();
        /*
            setBoundingBox (contentArea);
        */
    }
    
    /**
      | Resets the content area and the bounding
      | transform to fit around the area occupied
      | by the child components.
      |
      */
    pub fn reset_content_area_and_bounding_box_to_fit_children(&mut self)  {
        
        todo!();
        /*
            setContentArea (getDrawableBounds());
        resetBoundingBoxToContentArea();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            if (auto* parent = getParent())
            originRelativeToComponent = parent->originRelativeToComponent - getPosition();
        */
    }
    
    pub fn child_bounds_changed(&mut self, _0: *mut Component<'a>)  {
        
        todo!();
        /*
            updateBoundsToFitChildren();
        */
    }
    
    pub fn children_changed(&mut self)  {
        
        todo!();
        /*
            updateBoundsToFitChildren();
        */
    }
    
    pub fn update_bounds_to_fit_children(&mut self)  {
        
        todo!();
        /*
            if (! updateBoundsReentrant)
        {
            const ScopedValueSetter<bool> setter (updateBoundsReentrant, true, false);

            Rectangle<int> childArea;

            for (auto* c : getChildren())
                childArea = childArea.getUnion (c->getBoundsInParent());

            auto delta = childArea.getPosition();
            childArea += getPosition();

            if (childArea != getBounds())
            {
                if (! delta.isOrigin())
                {
                    originRelativeToComponent -= delta;

                    for (auto* c : getChildren())
                        c->setBounds (c->getBounds() - delta);
                }

                setBounds (childArea);
            }
        }
        */
    }
    
    pub fn get_outline_as_path(&self) -> PathBuf {
        
        todo!();
        /*
            Path p;

        for (auto* c : getChildren())
            if (auto* d = dynamic_cast<Drawable*> (c))
                p.addPath (d->getOutlineAsPath());

        p.applyTransform (getTransform());
        return p;
        */
    }
}
