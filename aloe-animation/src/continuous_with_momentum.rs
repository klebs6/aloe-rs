crate::ix!();

/**
  | A non-snapping behaviour that allows
  | the content to be freely flicked in either
  | direction, with momentum based on the
  | velocity at which it was released, and
  | variable friction to make it come to
  | a halt.
  | 
  | This class is intended to be used as a
  | template parameter to the
  | 
  | AnimatedPosition class.
  | 
  | @see AnimatedPosition
  | 
  | @tags{GUI}
  |
  */
#[derive(Default)]
pub struct ContinuousWithMomentum {
    velocity:         f64, // default = 0
    damping:          f64, // default = 0.92
    minimum_velocity: f64, // default = 0.05
}

impl ContinuousWithMomentum {

    /**
      | Sets the friction that damps the movement
      | of the value.
      | 
      | A typical value is 0.08; higher values
      | indicate more friction.
      |
      */
    pub fn set_friction(&mut self, new_friction: f64)  {
        
        todo!();
        /*
            damping = 1.0 - newFriction;
        */
    }

    /**
      | Sets the minimum velocity of the movement.
      | Any velocity that's slower than this
      | will stop the animation. The default
      | is 0.05.
      |
      */
    pub fn set_minimum_velocity(&mut self, new_minimum_velocity_to_use: f64)  {
        
        todo!();
        /*
            minimumVelocity = newMinimumVelocityToUse;
        */
    }
}

impl AnimatedPositionBehavior for ContinuousWithMomentum { }

impl AnimationBehaviorGetNextPosition for ContinuousWithMomentum {

    /**
      | Called by the AnimatedPosition class
      | to get the new position, after the given
      | time has elapsed.
      |
      */
    fn get_next_position(
        &self, 
        old_pos:         f64,
        elapsed_seconds: f64) -> f64 {
        
        todo!();
        /*
            velocity *= damping;

                if (std::abs (velocity) < minimumVelocity)
                    velocity = 0;

                return oldPos + velocity * elapsedSeconds;
        */
    }
}

impl AnimationBehaviorIsStopped for ContinuousWithMomentum {

    /**
      | Called by the AnimatedPosition class
      | to check whether the object is now stationary.
      |
      */
    fn is_stopped(&self, position: f64) -> bool {
        
        todo!();
        /*
            return velocity == 0.0;
        */
    }
}

impl AnimationBehaviorReleasedWithVelocity for ContinuousWithMomentum {

    /**
      | Called by the AnimatedPosition class.
      | This tells us the position and velocity
      | at which the user is about to release
      | the object.
      | 
      | The velocity is measured in units/second.
      |
      */
    fn released_with_velocity(
        &mut self, 
        position:         f64,
        release_velocity: f64

    ) {
        
        todo!();
        /*
            velocity = releaseVelocity;
        */
    }
}
