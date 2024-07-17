crate::ix!();

/**
  | A behaviour that gravitates an AnimatedPosition
  | object towards the nearest integer
  | position when released.
  | 
  | This class is intended to be used as a
  | template parameter to the
  | 
  | AnimatedPosition class. It's handy
  | when using an AnimatedPosition to show
  | a series of pages, because it allows
  | the pages can be scrolled smoothly,
  | but when released, snaps back to show
  | a whole page.
  | 
  | @see AnimatedPosition
  | 
  | @tags{GUI}
  |
  */
#[derive(Default)]
pub struct SnapToPageBoundaries {
    target_snap_position: f64, // default = 0.0
}

impl AnimatedPositionBehavior for SnapToPageBoundaries {}

impl AnimationBehaviorReleasedWithVelocity for SnapToPageBoundaries {

    /**
      | Called by the AnimatedPosition class.
      | This tells us the position and velocity
      | at which the user is about to release
      | the object.
      | 
      | The velocity is measured in units/second.
      |
      */
    fn released_with_velocity(&mut self, 
        position:         f64,
        release_velocity: f64)  {
        
        todo!();
        /*
            targetSnapPosition = std::floor (position + 0.5);

                if (releaseVelocity >  1.0 && targetSnapPosition < position)  ++targetSnapPosition;
                if (releaseVelocity < -1.0 && targetSnapPosition > position)  --targetSnapPosition;
        */
    }
}

impl AnimationBehaviorGetNextPosition for  SnapToPageBoundaries {

    /**
      | Called by the AnimatedPosition class
      | to get the new position, after the given
      | time has elapsed.
      |
      */
    fn get_next_position(&self, 
        old_pos:         f64,
        elapsed_seconds: f64) -> f64 {
        
        todo!();
        /*
            if (isStopped (oldPos))
                    return targetSnapPosition;

                const double snapSpeed = 10.0;
                const double velocity = (targetSnapPosition - oldPos) * snapSpeed;
                const double newPos = oldPos + velocity * elapsedSeconds;

                return isStopped (newPos) ? targetSnapPosition : newPos;
        */
    }
}

impl AnimationBehaviorIsStopped for  SnapToPageBoundaries {

    /**
      | Called by the AnimatedPosition class
      | to check whether the object is now stationary.
      |
      */
    fn is_stopped(&self, position: f64) -> bool {
        
        todo!();
        /*
            return std::abs (targetSnapPosition - position) < 0.001;
        */
    }
}
