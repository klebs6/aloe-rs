/*!
  | Contains classes for different types
  | of physics behaviours - these classes
  | are used as template parameters for
  | the AnimatedPosition class.
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_AnimatedPositionBehaviours.h]

pub trait AnimatedPositionBehavior
: AnimationBehaviorIsStopped
+ AnimationBehaviorGetNextPosition
+ AnimationBehaviorReleasedWithVelocity
{ }

pub trait AnimationBehaviorIsStopped {

    fn is_stopped(&self, position: f64) -> bool;
}

pub trait AnimationBehaviorGetNextPosition {

    fn get_next_position(
        &self, 
        old_pos:         f64,
        elapsed_seconds: f64) -> f64;
}

pub trait AnimationBehaviorReleasedWithVelocity {

    fn released_with_velocity(
        &mut self, 
        position:         f64,
        release_velocity: f64);
}
