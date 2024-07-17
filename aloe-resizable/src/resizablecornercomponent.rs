crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ResizableCornerComponent.h]

/**
  | A component that resizes a parent component
  | when dragged.
  | 
  | This is the small triangular stripey
  | resizer component you get in the bottom-right
  | of windows (more commonly on the Mac
  | than Windows). Put one in the corner
  | of a larger component and it will automatically
  | resize its parent when it gets dragged
  | around.
  | 
  | @see ResizableBorderComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ResizableCornerComponent<'a> {
    base:            Component<'a>,
    component:       WeakReference<Component<'a>>,
    constrainer:     *mut ComponentBoundsConstrainer,
    original_bounds: Rectangle<i32>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ResizableCornerComponent.cpp]
impl<'a> ResizableCornerComponent<'a> {

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
      | If a constrainer object is provided,
      | then this object will be used to enforce
      | limits on the size and position that
      | the component can be stretched to. Make
      | sure that the constrainer isn't deleted
      | while still in use by this object. If
      | you pass a nullptr in here, no limits
      | will be put on the sizes it can be stretched
      | to.
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

            setRepaintsOnMouseActivity (true);
        setMouseCursor (MouseCursor::BottomRightCornerResizeCursor);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawCornerResizer (g, getWidth(), getHeight(),
                                            isMouseOverOrDragging(),
                                            isMouseButtonDown());
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (component == nullptr)
        {
            jassertfalse; // You've deleted the component that this resizer is supposed to be controlling!
            return;
        }

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
            jassertfalse; // You've deleted the component that this resizer is supposed to be controlling!
            return;
        }

        auto r = originalBounds.withSize (originalBounds.getWidth() + e.getDistanceFromDragStartX(),
                                          originalBounds.getHeight() + e.getDistanceFromDragStartY());

        if (constrainer != nullptr)
            constrainer->setBoundsForComponent (component, r, false, false, true, true);
        else if (auto pos = component->getPositioner())
            pos->applyNewBounds (r);
        else
            component->setBounds (r);
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
            if (getWidth() <= 0)
            return false;

        auto yAtX = getHeight() - (getHeight() * x / getWidth());
        return y >= yAtX - getHeight() / 4;
        */
    }
}
