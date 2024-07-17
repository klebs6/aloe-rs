crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentBoundsConstrainer.h]

pub trait ComponentBoundsConstrainerCheckBounds {

    /**
      | This callback changes the given coordinates
      | to impose whatever the current constraints
      | are set to be.
      | 
      | -----------
      | @param bounds
      | 
      | the target position that should be examined
      | and adjusted
      | ----------
      | @param previousBounds
      | 
      | the component's current size
      | ----------
      | @param limits
      | 
      | the region in which the component can
      | be positioned
      | ----------
      | @param isStretchingTop
      | 
      | whether the top edge of the component
      | is being resized
      | ----------
      | @param isStretchingLeft
      | 
      | whether the left edge of the component
      | is being resized
      | ----------
      | @param isStretchingBottom
      | 
      | whether the bottom edge of the component
      | is being resized
      | ----------
      | @param isStretchingRight
      | 
      | whether the right edge of the component
      | is being resized
      |
      */
    fn check_bounds(&mut self, 
        bounds:               &mut Rectangle<i32>,
        previous_bounds:      &Rectangle<i32>,
        limits:               &Rectangle<i32>,
        is_stretching_top:    bool,
        is_stretching_left:   bool,
        is_stretching_bottom: bool,
        is_stretching_right:  bool);

}

pub trait ResizeStart {

    /**
      | This callback happens when the resizer
      | is about to start dragging.
      |
      */
    fn resize_start(&mut self);
}

pub trait ResizeEnd {

    /**
      | This callback happens when the resizer
      | has finished dragging.
      |
      */
    fn resize_end(&mut self);
}

pub trait ApplyBoundsToComponent {

    /**
      | Called by setBoundsForComponent()
      | to apply a new constrained size to a component.
      | 
      | By default this just calls setBounds(),
      | but is virtual in case it's needed for
      | extremely cunning purposes.
      |
      */
    fn apply_bounds_to_component(&mut self, 
        _0:     &mut Component,
        bounds: Rectangle<i32>);
}

/**
  | A class that imposes restrictions on
  | a Component's size or position.
  | 
  | This is used by classes such as ResizableCornerComponent,
  | 
  | ResizableBorderComponent and ResizableWindow.
  | 
  | The base class can impose some basic
  | size and position limits, but you can
  | also subclass this for custom uses.
  | 
  | @see ResizableCornerComponent, ResizableBorderComponent,
  | ResizableWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentBoundsConstrainer {
    minw:           i32, // default = 0
    maxw:           i32, // default = 0x3fffffff
    minh:           i32, // default = 0
    maxh:           i32, // default = 0x3fffffff
    min_off_top:    i32, // default = 0
    min_off_left:   i32, // default = 0
    min_off_bottom: i32, // default = 0
    min_off_right:  i32, // default = 0
    aspect_ratio:   f64, // default = 0
}

impl Default for ComponentBoundsConstrainer {
    
    /**
      | When first created, the object will
      | not impose any restrictions on the components.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentBoundsConstrainer.cpp]
impl ComponentBoundsConstrainer {

    /**
      | Returns the current minimum width.
      |
      */
    pub fn get_minimum_width(&self) -> i32 {
        
        todo!();
        /*
            return minW;
        */
    }

    /**
      | Returns the current maximum width.
      |
      */
    pub fn get_maximum_width(&self) -> i32 {
        
        todo!();
        /*
            return maxW;
        */
    }

    /**
      | Returns the current minimum height.
      |
      */
    pub fn get_minimum_height(&self) -> i32 {
        
        todo!();
        /*
            return minH;
        */
    }

    /**
      | Returns the current maximum height.
      |
      */
    pub fn get_maximum_height(&self) -> i32 {
        
        todo!();
        /*
            return maxH;
        */
    }

    /**
      | Returns the minimum distance the bounds
      | can be off-screen. @see setMinimumOnscreenAmounts
      |
      */
    pub fn get_minimum_when_off_the_top(&self) -> i32 {
        
        todo!();
        /*
            return minOffTop;
        */
    }
    /**
      | Returns the minimum distance the bounds
      | can be off-screen. @see setMinimumOnscreenAmounts
      |
      */
    pub fn get_minimum_when_off_the_left(&self) -> i32 {
        
        todo!();
        /*
            return minOffLeft;
        */
    }
    /**
      | Returns the minimum distance the bounds
      | can be off-screen. @see setMinimumOnscreenAmounts
      |
      */
    pub fn get_minimum_when_off_the_bottom(&self) -> i32 {
        
        todo!();
        /*
            return minOffBottom;
        */
    }
    /**
      | Returns the minimum distance the bounds
      | can be off-screen. @see setMinimumOnscreenAmounts
      |
      */
    pub fn get_minimum_when_off_the_right(&self) -> i32 {
        
        todo!();
        /*
            return minOffRight;
        */
    }
    
    /**
      | Imposes a minimum width limit.
      |
      */
    pub fn set_minimum_width(&mut self, minimum_width: i32)  {
        
        todo!();
        /*
            minW = minimumWidth;
        */
    }
    
    /**
      | Imposes a maximum width limit.
      |
      */
    pub fn set_maximum_width(&mut self, maximum_width: i32)  {
        
        todo!();
        /*
            maxW = maximumWidth;
        */
    }
    
    /**
      | Imposes a minimum height limit.
      |
      */
    pub fn set_minimum_height(&mut self, minimum_height: i32)  {
        
        todo!();
        /*
            minH = minimumHeight;
        */
    }
    
    /**
      | Imposes a maximum height limit.
      |
      */
    pub fn set_maximum_height(&mut self, maximum_height: i32)  {
        
        todo!();
        /*
            maxH = maximumHeight;
        */
    }
    
    /**
      | Imposes a minimum width and height limit.
      |
      */
    pub fn set_minimum_size(&mut self, 
        minimum_width:  i32,
        minimum_height: i32)  {
        
        todo!();
        /*
            jassert (maxW >= minimumWidth);
        jassert (maxH >= minimumHeight);
        jassert (minimumWidth > 0 && minimumHeight > 0);

        minW = minimumWidth;
        minH = minimumHeight;

        if (minW > maxW)  maxW = minW;
        if (minH > maxH)  maxH = minH;
        */
    }
    
    /**
      | Imposes a maximum width and height limit.
      |
      */
    pub fn set_maximum_size(&mut self, 
        maximum_width:  i32,
        maximum_height: i32)  {
        
        todo!();
        /*
            jassert (maximumWidth >= minW);
        jassert (maximumHeight >= minH);
        jassert (maximumWidth > 0 && maximumHeight > 0);

        maxW = jmax (minW, maximumWidth);
        maxH = jmax (minH, maximumHeight);
        */
    }
    
    /**
      | Set all the maximum and minimum dimensions.
      |
      */
    pub fn set_size_limits(&mut self, 
        minimum_width:  i32,
        minimum_height: i32,
        maximum_width:  i32,
        maximum_height: i32)  {
        
        todo!();
        /*
            jassert (maximumWidth >= minimumWidth);
        jassert (maximumHeight >= minimumHeight);
        jassert (maximumWidth > 0 && maximumHeight > 0);
        jassert (minimumWidth > 0 && minimumHeight > 0);

        minW = jmax (0, minimumWidth);
        minH = jmax (0, minimumHeight);
        maxW = jmax (minW, maximumWidth);
        maxH = jmax (minH, maximumHeight);
        */
    }
    
    /**
      | Sets the amount by which the component
      | is allowed to go off-screen.
      | 
      | The values indicate how many pixels
      | must remain on-screen when dragged
      | off one of its parent's edges, so e.g.
      | if minimumWhenOffTheTop is set to 10,
      | then when the component goes off the
      | top of the screen, its y-position will
      | be clipped so that there are always at
      | least 10 pixels on-screen. In other
      | words, the lowest y-position it can
      | take would be (10 - the component's height).
      | 
      | If you pass 0 or less for one of these amounts,
      | the component is allowed to move beyond
      | that edge completely, with no restrictions
      | at all.
      | 
      | If you pass a very large number (i.e.
      | larger that the dimensions of the component
      | itself), then the component won't be
      | allowed to overlap that edge at all.
      | So e.g. setting minimumWhenOffTheLeft
      | to 0xffffff will mean that the component
      | will bump into the left side of the screen
      | and go no further.
      |
      */
    pub fn set_minimum_onscreen_amounts(&mut self, 
        minimum_when_off_the_top:    i32,
        minimum_when_off_the_left:   i32,
        minimum_when_off_the_bottom: i32,
        minimum_when_off_the_right:  i32)  {
        
        todo!();
        /*
            minOffTop    = minimumWhenOffTheTop;
        minOffLeft   = minimumWhenOffTheLeft;
        minOffBottom = minimumWhenOffTheBottom;
        minOffRight  = minimumWhenOffTheRight;
        */
    }
    
    /**
      | Specifies a width-to-height ratio
      | that the resizer should always maintain.
      | 
      | If the value is 0, no aspect ratio is enforced.
      | If it's non-zero, the width will always
      | be maintained as this multiple of the
      | height.
      | 
      | @see setResizeLimits
      |
      */
    pub fn set_fixed_aspect_ratio(&mut self, width_over_height: f64)  {
        
        todo!();
        /*
            aspectRatio = jmax (0.0, widthOverHeight);
        */
    }
    
    /**
      | Returns the aspect ratio that was set
      | with setFixedAspectRatio().
      | 
      | If no aspect ratio is being enforced,
      | this will return 0.
      |
      */
    pub fn get_fixed_aspect_ratio(&self) -> f64 {
        
        todo!();
        /*
            return aspectRatio;
        */
    }
    
    /**
      | Checks the given bounds, and then sets
      | the component to the corrected size.
      |
      */
    pub fn set_bounds_for_component(&mut self, 
        component:            *mut Component,
        target_bounds:        Rectangle<i32>,
        is_stretching_top:    bool,
        is_stretching_left:   bool,
        is_stretching_bottom: bool,
        is_stretching_right:  bool)  {
        
        todo!();
        /*
            jassert (component != nullptr);

        Rectangle<int> limits, bounds (targetBounds);
        BorderSize<int> border;

        if (auto* parent = component->getParentComponent())
        {
            limits.setSize (parent->getWidth(), parent->getHeight());
        }
        else
        {
            if (auto* peer = component->getPeer())
                border = peer->getFrameSize();

            auto screenBounds = Desktop::getInstance().getDisplays().getDisplayForPoint (targetBounds.getCentre())->userArea;

            limits = component->getLocalArea (nullptr, screenBounds) + component->getPosition();
        }

        border.addTo (bounds);

        checkBounds (bounds,
                     border.addedTo (component->getBounds()), limits,
                     isStretchingTop, isStretchingLeft,
                     isStretchingBottom, isStretchingRight);

        border.subtractFrom (bounds);

        applyBoundsToComponent (*component, bounds);
        */
    }
    
    /**
      | Performs a check on the current size
      | of a component, and moves or resizes
      | it if it fails the constraints.
      |
      */
    pub fn check_component_bounds(&mut self, component: *mut Component)  {
        
        todo!();
        /*
            setBoundsForComponent (component, component->getBounds(),
                               false, false, false, false);
        */
    }
    
    pub fn apply_bounds_to_component(&mut self, 
        component: &mut Component,
        bounds:    Rectangle<i32>)  {
        
        todo!();
        /*
            if (auto* positioner = component.getPositioner())
            positioner->applyNewBounds (bounds);
        else
            component.setBounds (bounds);
        */
    }
    
    pub fn resize_start(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn resize_end(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn check_bounds(&mut self, 
        bounds:               &mut Rectangle<i32>,
        old:                  &Rectangle<i32>,
        limits:               &Rectangle<i32>,
        is_stretching_top:    bool,
        is_stretching_left:   bool,
        is_stretching_bottom: bool,
        is_stretching_right:  bool)  {
        
        todo!();
        /*
            if (isStretchingLeft)
            bounds.setLeft (jlimit (old.getRight() - maxW, old.getRight() - minW, bounds.getX()));
        else
            bounds.setWidth (jlimit (minW, maxW, bounds.getWidth()));

        if (isStretchingTop)
            bounds.setTop (jlimit (old.getBottom() - maxH, old.getBottom() - minH, bounds.getY()));
        else
            bounds.setHeight (jlimit (minH, maxH, bounds.getHeight()));

        if (bounds.isEmpty())
            return;

        if (minOffTop > 0)
        {
            const int limit = limits.getY() + jmin (minOffTop - bounds.getHeight(), 0);

            if (bounds.getY() < limit)
            {
                if (isStretchingTop)
                    bounds.setTop (limits.getY());
                else
                    bounds.setY (limit);
            }
        }

        if (minOffLeft > 0)
        {
            const int limit = limits.getX() + jmin (minOffLeft - bounds.getWidth(), 0);

            if (bounds.getX() < limit)
            {
                if (isStretchingLeft)
                    bounds.setLeft (limits.getX());
                else
                    bounds.setX (limit);
            }
        }

        if (minOffBottom > 0)
        {
            const int limit = limits.getBottom() - jmin (minOffBottom, bounds.getHeight());

            if (bounds.getY() > limit)
            {
                if (isStretchingBottom)
                    bounds.setBottom (limits.getBottom());
                else
                    bounds.setY (limit);
            }
        }

        if (minOffRight > 0)
        {
            const int limit = limits.getRight() - jmin (minOffRight, bounds.getWidth());

            if (bounds.getX() > limit)
            {
                if (isStretchingRight)
                    bounds.setRight (limits.getRight());
                else
                    bounds.setX (limit);
            }
        }

        // constrain the aspect ratio if one has been specified..
        if (aspectRatio > 0.0)
        {
            bool adjustWidth;

            if ((isStretchingTop || isStretchingBottom) && ! (isStretchingLeft || isStretchingRight))
            {
                adjustWidth = true;
            }
            else if ((isStretchingLeft || isStretchingRight) && ! (isStretchingTop || isStretchingBottom))
            {
                adjustWidth = false;
            }
            else
            {
                const double oldRatio = (old.getHeight() > 0) ? std::abs (old.getWidth() / (double) old.getHeight()) : 0.0;
                const double newRatio = std::abs (bounds.getWidth() / (double) bounds.getHeight());

                adjustWidth = (oldRatio > newRatio);
            }

            if (adjustWidth)
            {
                bounds.setWidth (roundToInt (bounds.getHeight() * aspectRatio));

                if (bounds.getWidth() > maxW || bounds.getWidth() < minW)
                {
                    bounds.setWidth (jlimit (minW, maxW, bounds.getWidth()));
                    bounds.setHeight (roundToInt (bounds.getWidth() / aspectRatio));
                }
            }
            else
            {
                bounds.setHeight (roundToInt (bounds.getWidth() / aspectRatio));

                if (bounds.getHeight() > maxH || bounds.getHeight() < minH)
                {
                    bounds.setHeight (jlimit (minH, maxH, bounds.getHeight()));
                    bounds.setWidth (roundToInt (bounds.getHeight() * aspectRatio));
                }
            }

            if ((isStretchingTop || isStretchingBottom) && ! (isStretchingLeft || isStretchingRight))
            {
                bounds.setX (old.getX() + (old.getWidth() - bounds.getWidth()) / 2);
            }
            else if ((isStretchingLeft || isStretchingRight) && ! (isStretchingTop || isStretchingBottom))
            {
                bounds.setY (old.getY() + (old.getHeight() - bounds.getHeight()) / 2);
            }
            else
            {
                if (isStretchingLeft)
                    bounds.setX (old.getRight() - bounds.getWidth());

                if (isStretchingTop)
                    bounds.setY (old.getBottom() - bounds.getHeight());
            }
        }

        jassert (! bounds.isEmpty());
        */
    }
}
