crate::ix!();

lazy_static!{
    /*
    static int doubleClickTimeOutMs = 400;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseEvent.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseEvent.cpp]

/**
  | Contains position and status information
  | about a mouse event.
  | 
  | @see MouseListener, Component::mouseMove,
  | Component::mouseEnter, Component::mouseExit,
  | 
  | Component::mouseDown, Component::mouseUp,
  | Component::mouseDrag
  | 
  | @tags{GUI}
  |
  */
pub struct MouseEvent<'a> {

    /**
      | The position of the mouse when the event
      | occurred.
      | 
      | This value is relative to the top-left
      | of the component to which the event applies
      | (as indicated by the MouseEvent::eventComponent
      | field).
      | 
      | This is a more accurate floating-point
      | version of the position returned by
      | getPosition() and the integer x and
      | y member variables.
      |
      */
    position:                   Point<f32>,

    /**
      | The x-position of the mouse when the
      | event occurred.
      | 
      | This value is relative to the top-left
      | of the component to which the event applies
      | (as indicated by the MouseEvent::eventComponent
      | field).
      | 
      | For a floating-point coordinate, see
      | MouseEvent::position
      |
      */
    x:                          i32,

    /**
      | The y-position of the mouse when the
      | event occurred.
      | 
      | This value is relative to the top-left
      | of the component to which the event applies
      | (as indicated by the MouseEvent::eventComponent
      | field).
      | 
      | For a floating-point coordinate, see
      | MouseEvent::position
      |
      */
    y:                          i32,

    /**
      | The key modifiers associated with the
      | event.
      | 
      | This will let you find out which mouse
      | buttons were down, as well as which modifier
      | keys were held down.
      | 
      | When used for mouse-up events, this
      | will indicate the state of the mouse
      | buttons just before they were released,
      | so that you can tell which button they
      | let go of.
      |
      */
    mods:                       ModifierKeys,

    /**
      | The pressure of the touch or stylus for
      | this event.
      | 
      | The range is 0 (soft) to 1 (hard).
      | 
      | If the input device doesn't provide
      | any pressure data, it may return a negative
      | value here, or 0.0 or 1.0, depending
      | on the platform.
      |
      */
    pressure:                   f32,

    /**
      | The orientation of the touch input for
      | this event in radians where 0 indicates
      | a touch aligned with the x-axis and pointing
      | from left to right; increasing values
      | indicate rotation in the clockwise
      | direction. The default is 0.
      |
      */
    orientation:                f32,

    /**
      | The rotation of the pen device for this
      | event in radians. Indicates the clockwise
      | rotation, or twist, of the pen. The default
      | is 0.
      |
      */
    rotation:                   f32,

    /**
      | The tilt of the pen device along the x-axis
      | between -1.0 and 1.0. A positive value
      | indicates a tilt to the right. The default
      | is 0.
      |
      */
    tiltx:                      f32,

    /**
      | The tilt of the pen device along the y-axis
      | between -1.0 and 1.0. A positive value
      | indicates a tilt toward the user. The
      | default is 0.
      |
      */
    tilty:                      f32,

    /**
      | The coordinates of the last place that
      | a mouse button was pressed.
      | 
      | The coordinates are relative to the
      | component specified in MouseEvent::component.
      | @see getDistanceFromDragStart, getDistanceFromDragStartX,
      | mouseWasDraggedSinceMouseDown
      |
      */
    mouse_down_position:        Point<f32>,

    /**
      | The component that this event applies
      | to.
      | 
      | This is usually the component that the
      | mouse was over at the time, but for mouse-drag
      | events the mouse could actually be over
      | a different component and the events
      | are still sent to the component that
      | the button was originally pressed on.
      | 
      | The x and y member variables are relative
      | to this component's position.
      | 
      | If you use getEventRelativeTo() to
      | retarget this object to be relative
      | to a different component, this pointer
      | will be updated, but originalComponent
      | remains unchanged.
      | 
      | @see originalComponent
      |
      */
    event_component:            *const Component<'a>,

    /**
      | The component that the event first occurred
      | on.
      | 
      | If you use getEventRelativeTo() to
      | retarget this object to be relative
      | to a different component, this value
      | remains unchanged to indicate the first
      | component that received it.
      | 
      | @see eventComponent
      |
      */
    original_component:         *const Component<'a>,

    /**
      | The time that this mouse-event occurred.
      |
      */
    event_time:                 Time,

    /**
      | The time that the corresponding mouse-down
      | event occurred.
      |
      */
    mouse_down_time:            Time,

    /**
      | The source device that generated this
      | event.
      |
      */
    source:                     MouseInputSource<'a>,

    number_of_clicks:           u8,

    was_moved_since_mouse_down: u8,
}

impl<'a> MouseEvent<'a> {
    
    /**
      | For a click event, the number of times
      | the mouse was clicked in succession.
      | 
      | So for example a double-click event
      | will return 2, a triple-click 3, etc.
      |
      */
    pub fn get_number_of_clicks(&self) -> i32 {
        
        todo!();
        /*
            return numberOfClicks;
        */
    }

    /**
      | Creates a MouseEvent.
      | 
      | Normally an application will never
      | need to use this.
      | 
      | -----------
      | @param source
      | 
      | the source that's invoking the event
      | ----------
      | @param position
      | 
      | the position of the mouse, relative
      | to the component that is passed-in
      | ----------
      | @param modifiers
      | 
      | the key modifiers at the time of the event
      | ----------
      | @param pressure
      | 
      | the pressure of the touch or stylus,
      | in the range 0 to 1. Devices that do not
      | support force information may return
      | 0.0, 1.0, or a negative value, depending
      | on the platform
      | ----------
      | @param orientation
      | 
      | the orientation of the touch input for
      | this event in radians. The default is
      | 0
      | ----------
      | @param rotation
      | 
      | the rotation of the pen device for this
      | event in radians. The default is 0
      | ----------
      | @param tiltX
      | 
      | the tilt of the pen device along the x-axis
      | between -1.0 and 1.0. The default is
      | 0
      | ----------
      | @param tiltY
      | 
      | the tilt of the pen device along the y-axis
      | between -1.0 and 1.0. The default is
      | 0
      | ----------
      | @param eventComponent
      | 
      | the component that the mouse event applies
      | to
      | ----------
      | @param originator
      | 
      | the component that originally received
      | the event
      | ----------
      | @param eventTime
      | 
      | the time the event happened
      | ----------
      | @param mouseDownPos
      | 
      | the position of the corresponding mouse-down
      | event (relative to the component that
      | is passed-in).
      | 
      | If there isn't a corresponding mouse-down
      | (e.g. for a mouse-move), this will just
      | be the same as the current mouse-x position.
      | ----------
      | @param mouseDownTime
      | 
      | the time at which the corresponding
      | mouse-down event happened
      | 
      | If there isn't a corresponding mouse-down
      | (e.g. for a mouse-move), this will just
      | be the same as the current mouse-event
      | time.
      | ----------
      | @param numberOfClicks
      | 
      | how many clicks, e.g. a double-click
      | event will be 2, a triple-click will
      | be 3, etc
      | ----------
      | @param mouseWasDragged
      | 
      | whether the mouse has been dragged significantly
      | since the previous mouse-down
      |
      */
    pub fn new(
        input_source:      MouseInputSource,
        pos:               Point<f32>,
        mod_keys:          ModifierKeys,
        force:             f32,
        o:                 f32,
        r:                 f32,
        tx:                f32,
        ty:                f32,
        event_comp:        *mut Component<'a>,
        originator:        *mut Component<'a>,
        time:              Time,
        down_pos:          Point<f32>,
        down_time:         Time,
        num_clicks:        i32,
        mouse_was_dragged: bool) -> Self {
    
        todo!();
        /*


            : position (pos),
          x (roundToInt (pos.x)),
          y (roundToInt (pos.y)),
          mods (modKeys),
          pressure (force),
          orientation (o), rotation (r),
          tiltX (tX), tiltY (tY),
          mouseDownPosition (downPos),
          eventComponent (eventComp),
          originalComponent (originator),
          eventTime (time),
          mouseDownTime (downTime),
          source (inputSource),
          numberOfClicks ((uint8) numClicks),
          wasMovedSinceMouseDown ((uint8) (mouseWasDragged ? 1 : 0))
        */
    }
    
    /**
      | Creates a version of this event that
      | is relative to a different component.
      | 
      | The x and y positions of the event that
      | is returned will have been adjusted
      | to be relative to the new component.
      | 
      | The component pointer that is passed-in
      | must not be null.
      |
      */
    pub fn get_event_relative_to(&self, other_component: *mut Component<'a>) -> MouseEvent {
        
        todo!();
        /*
            jassert (otherComponent != nullptr);

        return MouseEvent (source, otherComponent->getLocalPoint (eventComponent, position),
                           mods, pressure, orientation, rotation, tiltX, tiltY,
                           otherComponent, originalComponent, eventTime,
                           otherComponent->getLocalPoint (eventComponent, mouseDownPosition),
                           mouseDownTime, numberOfClicks, wasMovedSinceMouseDown != 0);
        */
    }
    
    /**
      | Creates a copy of this event with a different
      | position.
      | 
      | All other members of the event object
      | are the same, but the x and y are replaced
      | with these new values.
      |
      */
    pub fn with_new_position(&self, new_position: Point<f32>) -> MouseEvent {
        
        todo!();
        /*
            return MouseEvent (source, newPosition, mods, pressure, orientation, rotation, tiltX, tiltY,
                           eventComponent, originalComponent, eventTime, mouseDownPosition, mouseDownTime,
                           numberOfClicks, wasMovedSinceMouseDown != 0);
        */
    }
    
    /**
      | Creates a copy of this event with a different
      | position.
      | 
      | All other members of the event object
      | are the same, but the x and y are replaced
      | with these new values.
      |
      */
    pub fn with_new_position_i32(&self, new_position: Point<i32>) -> MouseEvent {
        
        todo!();
        /*
            return MouseEvent (source, newPosition.toFloat(), mods, pressure, orientation, rotation,
                           tiltX, tiltY, eventComponent,  originalComponent, eventTime, mouseDownPosition,
                           mouseDownTime, numberOfClicks, wasMovedSinceMouseDown != 0);
        */
    }
    
    /**
      | Returns true if the user seems to be performing
      | a drag gesture.
      | 
      | This is only meaningful if called in
      | either a mouseUp() or mouseDrag() method.
      | 
      | It will return true if the user has dragged
      | the mouse more than a few pixels from
      | the place where the mouse-down occurred
      | or the mouse has been held down for a significant
      | amount of time.
      | 
      | Once they have dragged it far enough
      | for this method to return true, it will
      | continue to return true until the mouse-up,
      | even if they move the mouse back to the
      | same location at which the mouse-down
      | happened. This means that it's very
      | handy for objects that can either be
      | clicked on or dragged, as you can use
      | it in the mouseDrag() callback to ignore
      | small movements they might make while
      | trying to click.
      |
      */
    pub fn mouse_was_dragged_since_mouse_down(&self) -> bool {
        
        todo!();
        /*
            return wasMovedSinceMouseDown != 0;
        */
    }
    
    /**
      | Returns true if the mouse event is part
      | of a click gesture rather than a drag.
      | 
      | This is effectively the opposite of
      | mouseWasDraggedSinceMouseDown()
      |
      */
    pub fn mouse_was_clicked(&self) -> bool {
        
        todo!();
        /*
            return ! mouseWasDraggedSinceMouseDown();
        */
    }
    
    /**
      | Returns the time that the mouse button
      | has been held down for.
      | 
      | If called from a mouseDrag or mouseUp
      | callback, this will return the number
      | of milliseconds since the corresponding
      | mouseDown event occurred.
      | 
      | If called in other contexts, e.g. a mouseMove,
      | then the returned value may be 0 or an
      | undefined value.
      |
      */
    pub fn get_length_of_mouse_press(&self) -> i32 {
        
        todo!();
        /*
            if (mouseDownTime.toMilliseconds() > 0)
            return jmax (0, (int) (eventTime - mouseDownTime).inMilliseconds());

        return 0;
        */
    }
    
    /**
      | The position of the mouse when the event
      | occurred.
      | 
      | This position is relative to the top-left
      | of the component to which the event applies
      | (as indicated by the MouseEvent::eventComponent
      | field).
      | 
      | For a floating-point position, see
      | MouseEvent::position
      |
      */
    pub fn get_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return Point<int> (x, y);
        */
    }
    
    /**
      | Returns the mouse position of this event,
      | in global screen coordinates.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor. @see getMouseDownScreenPosition
      |
      */
    pub fn get_screen_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return eventComponent->localPointToGlobal (getPosition());
        */
    }
    
    /**
      | Returns the coordinates of the last
      | place that a mouse was pressed.
      | 
      | The coordinates are relative to the
      | component specified in MouseEvent::component.
      | 
      | For a floating point version of this
      | value, see mouseDownPosition. @see
      | mouseDownPosition, getDistanceFromDragStart,
      | getDistanceFromDragStartX, mouseWasDraggedSinceMouseDown
      |
      */
    pub fn get_mouse_down_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return mouseDownPosition.roundToInt();
        */
    }
    
    /**
      | Returns the coordinates at which the
      | mouse button was last pressed.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor. @see getScreenPosition
      |
      */
    pub fn get_mouse_down_screen_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return eventComponent->localPointToGlobal (mouseDownPosition).roundToInt();
        */
    }
    
    /**
      | Returns the difference between the
      | mouse's current position and where
      | it was when the button was last pressed.
      | 
      | @see getDistanceFromDragStart
      |
      */
    pub fn get_offset_from_drag_start(&self) -> Point<i32> {
        
        todo!();
        /*
            return (position - mouseDownPosition).roundToInt();
        */
    }
    
    /**
      | Returns the straight-line distance
      | between where the mouse is now and where
      | it was the last time the button was pressed.
      | 
      | This is quite handy for things like deciding
      | whether the user has moved far enough
      | for it to be considered a drag operation.
      | 
      | @see getDistanceFromDragStartX
      |
      */
    pub fn get_distance_from_drag_start(&self) -> i32 {
        
        todo!();
        /*
            return roundToInt (mouseDownPosition.getDistanceFrom (position));
        */
    }
    
    /**
      | Returns the x coordinate of the last
      | place that a mouse was pressed.
      | 
      | The coordinate is relative to the component
      | specified in MouseEvent::component.
      | @see getDistanceFromDragStart, getDistanceFromDragStartX,
      | mouseWasDraggedSinceMouseDown
      |
      */
    pub fn get_mouse_downx(&self) -> i32 {
        
        todo!();
        /*
            return roundToInt (mouseDownPosition.x);
        */
    }
    
    /**
      | Returns the y coordinate of the last
      | place that a mouse was pressed.
      | 
      | The coordinate is relative to the component
      | specified in MouseEvent::component.
      | @see getDistanceFromDragStart, getDistanceFromDragStartX,
      | mouseWasDraggedSinceMouseDown
      |
      */
    pub fn get_mouse_downy(&self) -> i32 {
        
        todo!();
        /*
            return roundToInt (mouseDownPosition.y);
        */
    }
    
    /**
      | Returns the difference between the
      | mouse's current x position and where
      | it was when the button was last pressed.
      | 
      | @see getDistanceFromDragStart
      |
      */
    pub fn get_distance_from_drag_startx(&self) -> i32 {
        
        todo!();
        /*
            return getOffsetFromDragStart().x;
        */
    }
    
    /**
      | Returns the difference between the
      | mouse's current y position and where
      | it was when the button was last pressed.
      | 
      | @see getDistanceFromDragStart
      |
      */
    pub fn get_distance_from_drag_starty(&self) -> i32 {
        
        todo!();
        /*
            return getOffsetFromDragStart().y;
        */
    }
    
    /**
      | Returns the mouse x position of this
      | event, in global screen coordinates.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor. @see getScreenPosition
      |
      */
    pub fn get_screenx(&self) -> i32 {
        
        todo!();
        /*
            return getScreenPosition().x;
        */
    }
    
    /**
      | Returns the mouse y position of this
      | event, in global screen coordinates.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor. @see getScreenPosition
      |
      */
    pub fn get_screeny(&self) -> i32 {
        
        todo!();
        /*
            return getScreenPosition().y;
        */
    }
    
    /**
      | Returns the x coordinate at which the
      | mouse button was last pressed.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor. @see getMouseDownScreenPosition
      |
      */
    pub fn get_mouse_down_screenx(&self) -> i32 {
        
        todo!();
        /*
            return getMouseDownScreenPosition().x;
        */
    }
    
    /**
      | Returns the y coordinate at which the
      | mouse button was last pressed.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor. @see getMouseDownScreenPosition
      |
      */
    pub fn get_mouse_down_screeny(&self) -> i32 {
        
        todo!();
        /*
            return getMouseDownScreenPosition().y;
        */
    }
    
    /**
      | Returns true if the pressure value for
      | this event is meaningful.
      |
      */
    pub fn is_pressure_valid(&self) -> bool {
        
        todo!();
        /*
            return pressure > 0.0f && pressure < 1.0f;
        */
    }
    
    /**
      | Returns true if the orientation value
      | for this event is meaningful.
      |
      */
    pub fn is_orientation_valid(&self) -> bool {
        
        todo!();
        /*
            return orientation >= 0.0f && orientation <= MathConstants<float>::twoPi;
        */
    }
    
    /**
      | Returns true if the rotation value for
      | this event is meaningful.
      |
      */
    pub fn is_rotation_valid(&self) -> bool {
        
        todo!();
        /*
            return rotation >= 0 && rotation <= MathConstants<float>::twoPi;
        */
    }
    
    /**
      | Returns true if the current tilt value
      | (either x- or y-axis) is meaningful.
      |
      */
    pub fn is_tilt_valid(&self, isx: bool) -> bool {
        
        todo!();
        /*
            return isX ? (tiltX >= -1.0f && tiltX <= 1.0f) : (tiltY >= -1.0f && tiltY <= 1.0f);
        */
    }
    
    /**
      | Returns the application-wide setting
      | for the double-click time limit.
      | 
      | This is the maximum length of time between
      | mouse-clicks for it to be considered
      | a double-click. It's used by the Component
      | class.
      | 
      | @see setDoubleClickTimeout, MouseListener::mouseDoubleClick
      |
      */
    pub fn get_double_click_timeout(&mut self) -> i32 {
        
        todo!();
        /*
            return doubleClickTimeOutMs;
        */
    }
    
    /**
      | Changes the application-wide setting
      | for the double-click time limit.
      | 
      | This is the maximum length of time between
      | mouse-clicks for it to be considered
      | a double-click. It's used by the Component
      | class.
      | 
      | @see getDoubleClickTimeout, MouseListener::mouseDoubleClick
      |
      */
    pub fn set_double_click_timeout(&mut self, new_time: i32)  {
        
        todo!();
        /*
            doubleClickTimeOutMs = newTime;
        */
    }
}
