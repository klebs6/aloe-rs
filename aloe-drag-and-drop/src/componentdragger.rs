crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_ComponentDragger.h]

/**
  | An object to take care of the logic for
  | dragging components around with the
  | mouse.
  | 
  | Very easy to use - in your mouseDown()
  | callback, call startDraggingComponent(),
  | then in your mouseDrag() callback,
  | call dragComponent().
  | 
  | When starting a drag, you can give it
  | a ComponentBoundsConstrainer to use
  | to limit the component's position and
  | keep it on-screen.
  | 
  | -----------
  | @code
  | 
  | class MyDraggableComp
  | {
  |     ComponentDragger myDragger;
  | 
  |     void mouseDown (const MouseEvent& e)
  |     {
  |         myDragger.startDraggingComponent (this, e);
  |     }
  | 
  |     void mouseDrag (const MouseEvent& e)
  |     {
  |         myDragger.dragComponent (this, e, nullptr);
  |     }
  | };
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
#[derive(Default)]
pub struct ComponentDragger {
    mouse_down_within_target: Point<i32>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_ComponentDragger.cpp]
impl ComponentDragger {
    
    /**
      | Call this from your component's mouseDown()
      | method, to prepare for dragging.
      | 
      | -----------
      | @param componentToDrag
      | 
      | the component that you want to drag
      | ----------
      | @param e
      | 
      | the mouse event that is triggering the
      | drag @see dragComponent
      |
      */
    pub fn start_dragging_component(
        &mut self, 
        component_to_drag: *mut Component,
        e:                 &MouseEvent

    )  {
        
        todo!();
        /*
            jassert (componentToDrag != nullptr);
        jassert (e.mods.isAnyMouseButtonDown()); // The event has to be a drag event!

        if (componentToDrag != nullptr)
            mouseDownWithinTarget = e.getEventRelativeTo (componentToDrag).getMouseDownPosition();
        */
    }
    
    /**
      | Call this from your mouseDrag() callback
      | to move the component.
      | 
      | This will move the component, using
      | the given constrainer object to check
      | the new position.
      | 
      | -----------
      | @param componentToDrag
      | 
      | the component that you want to drag
      | ----------
      | @param e
      | 
      | the current mouse-drag event
      | ----------
      | @param constrainer
      | 
      | an optional constrainer object that
      | should be used to apply limits to the
      | component's position. Pass null if
      | you don't want to constrain the movement.
      | @see startDraggingComponent
      |
      */
    pub fn drag_component(
        &mut self, 
        component_to_drag: *mut Component,
        e:                 &MouseEvent,
        constrainer:       *mut ComponentBoundsConstrainer

    ) {
        
        todo!();
        /*
            jassert (componentToDrag != nullptr);
        jassert (e.mods.isAnyMouseButtonDown()); // The event has to be a drag event!

        if (componentToDrag != nullptr)
        {
            auto bounds = componentToDrag->getBounds();

            // If the component is a window, multiple mouse events can get queued while it's in the same position,
            // so their coordinates become wrong after the first one moves the window, so in that case, we'll use
            // the current mouse position instead of the one that the event contains...
            if (componentToDrag->isOnDesktop())
                bounds += componentToDrag->getLocalPoint (nullptr, e.source.getScreenPosition()).roundToInt() - mouseDownWithinTarget;
            else
                bounds += e.getEventRelativeTo (componentToDrag).getPosition() - mouseDownWithinTarget;

            if (constrainer != nullptr)
                constrainer->setBoundsForComponent (componentToDrag, bounds, false, false, false, false);
            else
                componentToDrag->setBounds (bounds);
        }
        */
    }
}
