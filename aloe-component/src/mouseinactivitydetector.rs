crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseInactivityDetector.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseInactivityDetector.cpp]

/**
  | This object watches for mouse-events
  | happening within a component, and if
  | the mouse remains still for long enough,
  | triggers an event to indicate that it
  | has become inactive.
  | 
  | You'd use this for situations where
  | e.g. you want to hide the mouse-cursor
  | when the user's not actively using the
  | mouse.
  | 
  | After creating an instance of this,
  | use addListener to get callbacks when
  | the activity status changes.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MouseInactivityDetector<'a> {
    base:               Timer,
    target_comp:        &'a mut Component<'a>,
    listener_list:      ListenerList<Box<dyn MouseInactivityDetectorListener>>,
    last_mouse_pos:     Point<i32>,
    delay_ms:           i32,  // default = 1500
    tolerance_distance: i32,  // default = 15
    is_active:          bool, // default = true
}

impl<'a> MouseListener for MouseInactivityDetector<'a> { }
impl<'a> MouseMagnify  for MouseInactivityDetector<'a> { }

impl<'a> MouseWheelMove for MouseInactivityDetector<'a> {

    fn mouse_wheel_move(&mut self, 
        e:  &MouseEvent,
        _1: &MouseWheelDetails)  {
        
        todo!();
        /*
            wakeUp (e, true);
        */
    }
}

impl<'a> MouseDoubleClick for MouseInactivityDetector<'a> {

}

impl<'a> MouseUp for MouseInactivityDetector<'a> {

    fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            wakeUp (e, true);
        */
    }
}

impl<'a> MouseDrag for MouseInactivityDetector<'a> {

    fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            wakeUp (e, true);
        */
    }
}

impl<'a> MouseDown for MouseInactivityDetector<'a> {

    fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            wakeUp (e, true);
        */
    }
}

impl<'a> MouseExit for MouseInactivityDetector<'a> {

    fn mouse_exit(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            wakeUp (e, false);
        */
    }
}

impl<'a> MouseEnter for MouseInactivityDetector<'a> {

    fn mouse_enter(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            wakeUp (e, false);
        */
    }
}

impl<'a> MouseMove for MouseInactivityDetector<'a> {

    fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            wakeUp (e, false);
        */
    }
}

/**
  | Classes should implement this to receive
  | callbacks from a MouseInactivityDetector
  | when the mouse becomes active or inactive.
  |
  */
pub trait MouseInactivityDetectorListener
{
    /**
      | Called when the mouse is moved or clicked
      | for the first time after a period of inactivity.
      |
      */
    fn mouse_became_active(&mut self);

    /**
      | Called when the mouse hasn't been moved
      | for the timeout period.
      |
      */
    fn mouse_became_inactive(&mut self);
}

impl<'a> Drop for MouseInactivityDetector<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        targetComp.removeMouseListener (this);
        */
    }
}

impl<'a> MouseInactivityDetector<'a> {
    
    /**
      | Creates an inactivity watcher, attached
      | to the given component.
      | 
      | The target component must not be deleted
      | while this - it will be monitored for
      | any mouse events in it or its child components.
      |
      */
    pub fn new(c: &mut Component) -> Self {
    
        todo!();
        /*
        : target_comp(c),

            targetComp.addMouseListener (this, true);
        */
    }
    
    /**
      | Sets the time for which the mouse must
      | be still before the callback is triggered.
      |
      */
    pub fn set_delay(&mut self, new_delay: i32)  {
        
        todo!();
        /*
            delayMs = newDelay;
        */
    }
    
    /**
      | Sets the number of pixels by which the
      | cursor is allowed to drift before it
      | is considered to be actively moved.
      |
      */
    pub fn set_mouse_move_tolerance(&mut self, new_distance: i32)  {
        
        todo!();
        /*
            toleranceDistance = newDistance;
        */
    }
    
    /**
      | Registers a listener.
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn MouseInactivityDetectorListener)  {
        
        todo!();
        /*
            listenerList.add (l);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn MouseInactivityDetectorListener)  {
        
        todo!();
        /*
            listenerList.remove (l);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            setActive (false);
        */
    }
    
    pub fn wake_up(&mut self, 
        e:           &MouseEvent,
        always_wake: bool)  {
        
        todo!();
        /*
            auto newPos = e.getEventRelativeTo (&targetComp).getPosition();

        if ((! isActive) && (alwaysWake || e.source.isTouch() || newPos.getDistanceFrom (lastMousePos) > toleranceDistance))
            setActive (true);

        if (lastMousePos != newPos)
        {
            lastMousePos = newPos;
            startTimer (delayMs);
        }
        */
    }
    
    pub fn set_active(&mut self, b: bool)  {
        
        todo!();
        /*
            if (isActive != b)
        {
            isActive = b;

            if (isActive)
                listenerList.call ([] (Listener& l) { l.mouseBecameActive(); });
            else
                listenerList.call ([] (Listener& l) { l.mouseBecameInactive(); });
        }
        */
    }
}
