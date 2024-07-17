crate::ix!();

/**
  | Implement this class if you need to receive
  | callbacks when the value of an AnimatedPosition
  | changes. @see AnimatedPosition::addListener,
  | AnimatedPosition::removeListener
  |
  */
pub trait AnimatedPositionListener<B: AnimatedPositionBehavior> {

    /**
      | Called synchronously when an AnimatedPosition
      | changes.
      |
      */
    fn position_changed(
        &mut self, 
        _0:           &mut B,
        new_position: f64
    );
}

pub trait AnyAnimatedPositionListener {

    fn position_changed_any(
        &mut self,
        behavior: &mut dyn AnimatedPositionBehavior,
        new_position: f64,
    );
}

/*TODO
impl<B, T> AnyAnimatedPositionListener for T
where
    B: AnimatedPositionBehavior,
    T: AnimatedPositionListener<B>,
{
    fn position_changed_any(
        &mut self,
        behavior: &mut dyn AnimatedPositionBehavior,
        new_position: f64,
    ) {
        if let Some(specific_behavior) = behavior.downcast_mut::<B>() {
            self.position_changed(specific_behavior, new_position);
        } else {
            // Handle error: the behavior type doesn't match
        }
    }
}
*/
