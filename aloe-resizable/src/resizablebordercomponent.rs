crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ResizableBorderComponent.h]

/**
  | A component that resizes its parent
  | component when dragged.
  | 
  | This component forms a frame around
  | the edge of a component, allowing it
  | to be dragged by the edges or corners
  | to resize it - like the way windows are
  | resized in MSWindows or Linux.
  | 
  | To use it, just add it to your component,
  | making it fill the entire parent component
  | (there's a mouse hit-test that only
  | traps mouse-events which land around
  | the edge of the component, so it's even
  | ok to put it on top of any other components
  | you're using). Make sure you rescale
  | the resizer component to fill the parent
  | each time the parent's size changes.
  | 
  | @see ResizableCornerComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ResizableBorderComponent<'a> {
    base:            Component<'a>,
    component:       WeakReference<Component<'a>>,
    constrainer:     *mut ComponentBoundsConstrainer,
    border_size:     BorderSize<i32>,
    original_bounds: Rectangle<i32>,
    mouse_zone:      ResizableBorderComponentZone,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ResizableBorderComponent.cpp]
impl<'a> ResizableBorderComponent<'a> {

    /**
      | Returns the zone in which the mouse was
      | last seen.
      |
      */
    pub fn get_current_zone(&self) -> ResizableBorderComponentZone {
        
        todo!();
        /*
            return mouseZone;
        */
    }
    
    /**
      | Creates a resizer.
      | 
      | Pass in the target component which you
      | want to be resized when this one is dragged.
      | 
      | The target component will usually be
      | a parent of the resizer component, but
      | this isn't mandatory.
      | 
      | Remember that when the target component
      | is resized, it'll need to move and resize
      | this component to keep it in place, as
      | this won't happen automatically.
      | 
      | If the constrainer parameter is not
      | a nullptr, then this object will be used
      | to enforce limits on the size and position
      | that the component can be stretched
      | to.
      | 
      | Make sure that the constrainer isn't
      | deleted while still in use by this object.
      | 
      | @see ComponentBoundsConstrainer
      |
      */
    pub fn new(
        component_to_resize: *mut Component,
        bounds_constrainer:  *mut ComponentBoundsConstrainer) -> Self {
    
        todo!();
        /*
        : component(componentToResize),
        : constrainer(boundsConstrainer),
        : border_size(5),

        
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawResizableFrame (g, getWidth(), getHeight(), borderSize);
        */
    }
    
    pub fn mouse_enter(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateMouseZone (e);
        */
    }
    
    pub fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateMouseZone (e);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (component == nullptr)
        {
            jassertfalse; // You've deleted the component that this resizer was supposed to be using!
            return;
        }

        updateMouseZone (e);

        originalBounds = component->getBounds();

        if (constrainer != nullptr)
            constrainer->resizeStart();
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (component == nullptr)
        {
            jassertfalse; // You've deleted the component that this resizer was supposed to be using!
            return;
        }

        auto newBounds = mouseZone.resizeRectangleBy (originalBounds, e.getOffsetFromDragStart());

        if (constrainer != nullptr)
        {
            constrainer->setBoundsForComponent (component, newBounds,
                                                mouseZone.isDraggingTopEdge(),
                                                mouseZone.isDraggingLeftEdge(),
                                                mouseZone.isDraggingBottomEdge(),
                                                mouseZone.isDraggingRightEdge());
        }
        else
        {
            if (auto* p = component->getPositioner())
                p->applyNewBounds (newBounds);
            else
                component->setBounds (newBounds);
        }
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (constrainer != nullptr)
            constrainer->resizeEnd();
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            return ! borderSize.subtractedFrom (getLocalBounds()).contains (x, y);
        */
    }
    
    /**
      | Specifies how many pixels wide the draggable
      | edges of this component are.
      | 
      | @see getBorderThickness
      |
      */
    pub fn set_border_thickness(&mut self, new_border_size: BorderSize<i32>)  {
        
        todo!();
        /*
            if (borderSize != newBorderSize)
        {
            borderSize = newBorderSize;
            repaint();
        }
        */
    }
    
    /**
      | Returns the number of pixels wide that
      | the draggable edges of this component
      | are.
      | 
      | @see setBorderThickness
      |
      */
    pub fn get_border_thickness(&self) -> BorderSize<i32> {
        
        todo!();
        /*
            return borderSize;
        */
    }
    
    pub fn update_mouse_zone(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto newZone = ResizableBorderComponentZone::fromPositionOnBorder (getLocalBounds(), borderSize, e.getPosition());

        if (mouseZone != newZone)
        {
            mouseZone = newZone;
            setMouseCursor (newZone.getMouseCursor());
        }
        */
    }
}
