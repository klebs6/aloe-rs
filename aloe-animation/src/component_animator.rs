crate::ix!();

/**
  | Animates a set of components, moving
  | them to a new position and/or fading
  | their alpha levels.
  | 
  | To animate a component, create a ComponentAnimator
  | instance or (preferably) use the global
  | animator object provided by Desktop::getAnimator(),
  | and call its animateComponent() method
  | to commence the movement.
  | 
  | If you're using your own ComponentAnimator
  | instance, you'll need to make sure it
  | isn't deleted before it finishes moving
  | the components, or they'll be abandoned
  | before reaching their destinations.
  | 
  | It's ok to delete components while they're
  | being animated - the animator will detect
  | this and safely stop using them.
  | 
  | The class is a ChangeBroadcaster and
  | sends a notification when any components
  | start or finish being animated.
  | 
  | @see Desktop::getAnimator
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentAnimator<'a> {
    base:      ChangeBroadcaster<'a>,
    base2:     Timer,
    tasks:     Vec<Box<AnimationTask<'a>>>,
    last_time: u32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ComponentAnimator.cpp]
impl<'a> Default for ComponentAnimator<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : last_time(0),
        */
    }
}

impl<'a> ComponentAnimator<'a> {

    pub fn find_task_for(&self, component: *mut Component) -> *mut AnimationTask {
        
        todo!();
        /*
            for (int i = tasks.size(); --i >= 0;)
            if (component == tasks.getUnchecked(i)->component.get())
                return tasks.getUnchecked(i);

        return nullptr;
        */
    }

    /**
      | Starts a component moving from its current
      | position to a specified position.
      | 
      | If the component is already in the middle
      | of an animation, that will be abandoned,
      | and a new animation will begin, moving
      | the component from its current location.
      | 
      | The start and end speed parameters let
      | you apply some acceleration to the component's
      | movement.
      | 
      | -----------
      | @param component
      | 
      | the component to move
      | ----------
      | @param finalBounds
      | 
      | the destination bounds to which the
      | component should move. To leave the
      | component in the same place, just pass
      | component->getBounds() for this value
      | ----------
      | @param finalAlpha
      | 
      | the alpha value that the component should
      | have at the end of the animation
      | ----------
      | @param animationDurationMilliseconds
      | 
      | how long the animation should last,
      | in milliseconds
      | ----------
      | @param useProxyComponent
      | 
      | if true, this means the component should
      | be replaced by an internally managed
      | temporary component which is a snapshot
      | of the original component.
      | 
      | This avoids the component having to
      | paint itself as it moves, so may be more
      | efficient. This option also allows
      | you to delete the original component
      | immediately after starting the animation,
      | because the animation can proceed without
      | it. If you use a proxy, the original component
      | will be made invisible by this call,
      | and then will become visible again at
      | the end of the animation. It'll also
      | mean that the proxy component will be
      | temporarily added to the component's
      | parent, so avoid it if this might confuse
      | the parent component, or if there's
      | a chance the parent might decide to delete
      | its children.
      | ----------
      | @param startSpeed
      | 
      | a value to indicate the relative start
      | speed of the animation. If this is 0,
      | the component will start by accelerating
      | from rest; higher values mean that it
      | will have an initial speed greater than
      | zero. If the value is greater than 1,
      | it will decelerate towards the middle
      | of its journey. To move the component
      | at a constant rate for its entire animation,
      | set both the start and end speeds to 1.0
      | ----------
      | @param endSpeed
      | 
      | a relative speed at which the component
      | should be moving when the animation
      | finishes.
      | 
      | If this is 0, the component will decelerate
      | to a standstill at its final position;
      | higher values mean the component will
      | still be moving when it stops. To move
      | the component at a constant rate for
      | its entire animation, set both the start
      | and end speeds to 1.0
      |
      */
    pub fn animate_component(&mut self, 
        component:                    *mut Component,
        final_bounds:                 &Rectangle<i32>,
        final_alpha:                  f32,
        milliseconds_to_spend_moving: i32,
        use_proxy_component:          bool,
        start_speed:                  f64,
        end_speed:                    f64)  {
        
        todo!();
        /*
            // the speeds must be 0 or greater!
        jassert (startSpeed >= 0 && endSpeed >= 0);

        if (component != nullptr)
        {
            auto* at = findTaskFor (component);

            if (at == nullptr)
            {
                at = new AnimationTask (component);
                tasks.add (at);
                sendChangeMessage();
            }

            at->reset (finalBounds, finalAlpha, millisecondsToSpendMoving,
                       useProxyComponent, startSpeed, endSpeed);

            if (! isTimerRunning())
            {
                lastTime = Time::getMillisecondCounter();
                startTimerHz (50);
            }
        }
        */
    }
    
    /**
      | Begins a fade-out of this components
      | alpha level.
      | 
      | This is a quick way of invoking animateComponent()
      | with a target alpha value of 0.0f, using
      | a proxy. You're safe to delete the component
      | after calling this method, and this
      | won't interfere with the animation's
      | progress.
      |
      */
    pub fn fade_out(&mut self, 
        component:            *mut Component,
        milliseconds_to_take: i32)  {
        
        todo!();
        /*
            if (component != nullptr)
        {
            if (component->isShowing() && millisecondsToTake > 0)
                animateComponent (component, component->getBounds(), 0.0f, millisecondsToTake, true, 1.0, 1.0);

            component->setVisible (false);
        }
        */
    }
    
    /**
      | Begins a fade-in of a component.
      | 
      | This is a quick way of invoking animateComponent()
      | with a target alpha value of 1.0f.
      |
      */
    pub fn fade_in(&mut self, 
        component:            *mut Component,
        milliseconds_to_take: i32)  {
        
        todo!();
        /*
            if (component != nullptr && ! (component->isVisible() && component->getAlpha() == 1.0f))
        {
            component->setAlpha (0.0f);
            component->setVisible (true);
            animateComponent (component, component->getBounds(), 1.0f, millisecondsToTake, false, 1.0, 1.0);
        }
        */
    }
    
    /**
      | Clears all of the active animations.
      | 
      | If moveComponentsToTheirFinalPositions
      | is true, all the components will be immediately
      | set to their final positions. If false,
      | they will be left in whatever locations
      | they currently occupy.
      |
      */
    pub fn cancel_all_animations(&mut self, move_components_to_their_final_positions: bool)  {
        
        todo!();
        /*
            if (tasks.size() > 0)
        {
            if (moveComponentsToTheirFinalPositions)
                for (int i = tasks.size(); --i >= 0;)
                    tasks.getUnchecked(i)->moveToFinalDestination();

            tasks.clear();
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Stops a component if it's currently
      | being animated.
      | 
      | If moveComponentToItsFinalPosition
      | is true, then the component will be immediately
      | moved to its destination position and
      | size. If false, it will be left in whatever
      | location it currently occupies.
      |
      */
    pub fn cancel_animation(&mut self, 
        component:                            *mut Component,
        move_component_to_its_final_position: bool)  {
        
        todo!();
        /*
            if (auto* at = findTaskFor (component))
        {
            if (moveComponentToItsFinalPosition)
                at->moveToFinalDestination();

            tasks.removeObject (at);
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Returns the destination position for
      | a component.
      | 
      | If the component is being animated,
      | this will return the target position
      | that was specified when animateComponent()
      | was called.
      | 
      | If the specified component isn't currently
      | being animated, this method will just
      | return its current position.
      |
      */
    pub fn get_component_destination(&mut self, component: *mut Component) -> Rectangle<i32> {
        
        todo!();
        /*
            jassert (component != nullptr);

        if (auto* at = findTaskFor (component))
            return at->destination;

        return component->getBounds();
        */
    }
    
    /**
      | Returns true if the specified component
      | is currently being animated.
      |
      */
    pub fn is_component_animating(&self, component: *mut Component) -> bool {
        
        todo!();
        /*
            return findTaskFor (component) != nullptr;
        */
    }
    
    /**
      | Returns true if any components are currently
      | being animated.
      |
      */
    pub fn is_animating(&self) -> bool {
        
        todo!();
        /*
            return tasks.size() != 0;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto timeNow = Time::getMillisecondCounter();

        if (lastTime == 0)
            lastTime = timeNow;

        auto elapsed = (int) (timeNow - lastTime);

        for (auto* task : Vec<AnimationTask*> (tasks.begin(), tasks.size()))
        {
            if (tasks.contains (task) && ! task->useTimeslice (elapsed))
            {
                tasks.removeObject (task);
                sendChangeMessage();
            }
        }

        lastTime = timeNow;

        if (tasks.size() == 0)
            stopTimer();
        */
    }
}
