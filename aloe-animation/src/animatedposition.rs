crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_AnimatedPosition.h]

/**
  | Models a 1-dimensional position that
  | can be dragged around by the user, and
  | which will then continue moving with
  | a customisable physics behaviour when
  | released.
  | 
  | This is useful for things like scrollable
  | views or objects that can be dragged
  | and thrown around with the mouse/touch,
  | and by writing your own behaviour class,
  | you can customise the trajectory that
  | it follows when released.
  | 
  | The class uses its own Timer to continuously
  | change its value when a drag ends, and
  | 
  | Listener objects can be registered
  | to receive callbacks whenever the value
  | changes.
  | 
  | The value is stored as a double, and can
  | be used to represent whatever units
  | you need.
  | 
  | The template parameter Behaviour must
  | be a class that implements various methods
  | to return the physics of the value's
  | movement - you can use the classes provided
  | for this in the AnimatedPositionBehaviours
  | namespace, or write your own custom
  | behaviour.
  | 
  | @see AnimatedPositionBehaviours::ContinuousWithMomentum,
  | 
  | AnimatedPositionBehaviours::SnapToPageBoundaries
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AnimatedPosition<Behaviour: AnimatedPositionBehavior> {

    base:             Timer,

    /**
      | The behaviour object. This is public
      | to let you tweak any parameters that
      | it provides.
      |
      */
    behaviour:        Behaviour,

    position:         f64, // default = 0.0
    grabbed_pos:      f64, // default = 0.0
    release_velocity: f64, // default = 0.0
    range:            Range<f64>,
    last_update:      Time,
    last_drag:        Time,
    listeners:        ListenerList<Rc<RefCell<dyn AnyAnimatedPositionListener>>>,
}

impl<Behavior: AnimatedPositionBehavior> Default for AnimatedPosition<Behavior> {
    
    fn default() -> Self {
        todo!();
        /*
            :  range (-std::numeric_limits<double>::max(),
                       std::numeric_limits<double>::max()
        */
    }
}

impl<Behavior: AnimatedPositionBehavior> AnimatedPosition<Behavior> {

    /**
      | Sets a range within which the value will
      | be constrained.
      |
      */
    pub fn set_limits(&mut self, new_range: Range<f64>)  {
        
        todo!();
        /*
            range = newRange;
        */
    }
    
    /**
      | Called to indicate that the object is
      | now being controlled by a mouse-drag
      | or similar operation.
      | 
      | After calling this method, you should
      | make calls to the drag() method each
      | time the mouse drags the position around,
      | and always be sure to finish with a call
      | to endDrag() when the mouse is released,
      | which allows the position to continue
      | moving freely according to the specified
      | behaviour.
      |
      */
    pub fn begin_drag(&mut self)  {
        
        todo!();
        /*
            grabbedPos = position;
            releaseVelocity = 0;
            stopTimer();
        */
    }

    /**
      | Called during a mouse-drag operation,
      | to indicate that the mouse has moved.
      | 
      | The delta is the difference between
      | the position when beginDrag() was called
      | and the new position that's required.
      |
      */
    pub fn drag(&mut self, delta_from_start_of_drag: f64)  {
        
        todo!();
        /*
            moveTo (grabbedPos + deltaFromStartOfDrag);
        */
    }

    /**
      | Called after beginDrag() and drag()
      | to indicate that the drag operation
      | has now finished.
      |
      */
    pub fn end_drag(&mut self)  {
        
        todo!();
        /*
            startTimerHz (60);
        */
    }

    /**
      | Called outside of a drag operation to
      | cause a nudge in the specified direction.
      | 
      | This is intended for use by e.g. mouse-wheel
      | events.
      |
      */
    pub fn nudge(&mut self, delta_from_current_position: f64)  {
        
        todo!();
        /*
            startTimerHz (10);
            moveTo (position + deltaFromCurrentPosition);
        */
    }
    
    /**
      | Returns the current position.
      |
      */
    pub fn get_position(&self) -> f64 {
        
        todo!();
        /*
            return position;
        */
    }

    /**
      | Explicitly sets the position and stops
      | any further movement.
      | 
      | This will cause a synchronous call to
      | any listeners if the position actually
      | changes.
      |
      */
    pub fn set_position(&mut self, new_position: f64)  {
        
        todo!();
        /*
            stopTimer();
            setPositionAndSendChange (newPosition);
        */
    }

    /**
      | Adds a listener to be called when the
      | value changes.
      |
      */
    pub fn add_listener(&mut self, listener: Rc<RefCell<dyn AnyAnimatedPositionListener>>)  {
        
        todo!();
        /*
            listeners.add (listener);
        */
    }

    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_listener(&mut self, listener: Rc<RefCell<dyn AnyAnimatedPositionListener>>)  {
        
        todo!();
        /*
            listeners.remove (listener);
        */
    }
    
    pub fn get_speed(
        last:     Time,
        last_pos: f64,
        now:      Time,
        new_pos:  f64) -> f64 {
        
        todo!();
        /*
            auto elapsedSecs = jmax (0.005, (now - last).inSeconds());
            auto v = (newPos - lastPos) / elapsedSecs;
            return std::abs (v) > 0.2 ? v : 0.0;
        */
    }
    
    pub fn move_to(&mut self, new_pos: f64)  {
        
        todo!();
        /*
            auto now = Time::getCurrentTime();
            releaseVelocity = getSpeed (lastDrag, position, now, newPos);
            behaviour.releasedWithVelocity (newPos, releaseVelocity);
            lastDrag = now;

            setPositionAndSendChange (newPos);
        */
    }
    
    pub fn set_position_and_send_change(&mut self, new_position: f64)  {
        
        todo!();
        /*
            newPosition = range.clipValue (newPosition);

            if (position != newPosition)
            {
                position = newPosition;
                listeners.call ([this, newPosition] (Listener& l) { l.positionChanged (*this, newPosition); });
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto now = Time::getCurrentTime();
            auto elapsed = jlimit (0.001, 0.020, (now - lastUpdate).inSeconds());
            lastUpdate = now;
            auto newPos = behaviour.getNextPosition (position, elapsed);

            if (behaviour.isStopped (newPos))
                stopTimer();
            else
                startTimerHz (60);

            setPositionAndSendChange (newPos);
        */
    }
}
