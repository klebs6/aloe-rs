crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ResizableEdgeComponent.h]

pub enum ResizableEdgeComponentEdge
{
    /**
      | Indicates a vertical bar that can be
      | dragged left/right to move the component's
      | left-hand edge.
      |
      */
    leftEdge,   

    /**
      | Indicates a vertical bar that can be
      | dragged left/right to move the component's
      | right-hand edge.
      |
      */
    rightEdge,  

    /**
      | Indicates a horizontal bar that can
      | be dragged up/down to move the top of
      | the component.
      |
      */
    topEdge,    

    /**
      | Indicates a horizontal bar that can
      | be dragged up/down to move the bottom
      | of the component.
      |
      */
    bottomEdge,  
}

/**
  | A component that resizes its parent
  | component when dragged.
  | 
  | This component forms a bar along one
  | edge of a component, allowing it to be
  | dragged by that edges to resize it.
  | 
  | To use it, just add it to your component,
  | positioning it along the appropriate
  | edge. Make sure you reposition the resizer
  | component each time the parent's size
  | changes, to keep it in the correct position.
  | 
  | @see ResizableBorderComponent, ResizableCornerComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ResizableEdgeComponent<'a> {
    base:            Component<'a>,
    component:       WeakReference<Component<'a>>,
    constrainer:     *mut ComponentBoundsConstrainer,
    original_bounds: Rectangle<i32>,
    edge:            ResizableEdgeComponentEdge,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ResizableEdgeComponent.cpp]
impl<'a> ResizableEdgeComponent<'a> {

    /**
      | Creates a resizer bar.
      | 
      | Pass in the target component which you
      | want to be resized when this one is dragged.
      | The target component will usually be
      | this component's parent, but this isn't
      | mandatory.
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
        bounds_constrainer:  *mut ComponentBoundsConstrainer,
        e:                   ResizableEdgeComponentEdge) -> Self {
    
        todo!();
        /*
        : component(componentToResize),
        : constrainer(boundsConstrainer),
        : edge(e),

            setRepaintsOnMouseActivity (true);
        setMouseCursor (isVertical() ? MouseCursor::LeftRightResizeCursor
                                     : MouseCursor::UpDownResizeCursor);
        */
    }
    
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return edge == leftEdge || edge == rightEdge;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawStretchableLayoutResizerBar (g, getWidth(), getHeight(), isVertical(),
                                                          isMouseOver(), isMouseButtonDown());
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (component == nullptr)
        {
            jassertfalse; // You've deleted the component that this resizer was supposed to be using!
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
            jassertfalse; // You've deleted the component that this resizer was supposed to be using!
            return;
        }

        auto newBounds = originalBounds;

        switch (edge)
        {
            case leftEdge:      newBounds.setLeft (jmin (newBounds.getRight(), newBounds.getX() + e.getDistanceFromDragStartX())); break;
            case rightEdge:     newBounds.setWidth (jmax (0, newBounds.getWidth() + e.getDistanceFromDragStartX())); break;
            case topEdge:       newBounds.setTop (jmin (newBounds.getBottom(), newBounds.getY() + e.getDistanceFromDragStartY())); break;
            case bottomEdge:    newBounds.setHeight (jmax (0, newBounds.getHeight() + e.getDistanceFromDragStartY())); break;
            default:            jassertfalse; break;
        }

        if (constrainer != nullptr)
        {
            constrainer->setBoundsForComponent (component, newBounds,
                                                edge == topEdge,
                                                edge == leftEdge,
                                                edge == bottomEdge,
                                                edge == rightEdge);
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
}
