crate::ix!();

pub trait CanModalEventBeSentToComponent {

    /**
      | When a component is modal, this callback
      | allows it to choose which other components
      | can still receive events.
      | 
      | When a modal component is active and
      | the user clicks on a non-modal component,
      | this method is called on the modal component,
      | and if it returns true, the event is allowed
      | to reach its target. If it returns false,
      | the event is blocked and the inputAttemptWhenModal()
      | callback is made.
      | 
      | It called by the isCurrentlyBlockedByAnotherModalComponent()
      | method. The default implementation
      | just returns false in all cases.
      |
      */
    fn can_modal_event_be_sent_to_component<'a>(&mut self, target_component: *const Component<'a>) -> bool;
}

pub trait InputAttemptWhenModal {

    /**
      | Called when the user tries to click on
      | a component that is blocked by another
      | modal component.
      | 
      | When a component is modal and the user
      | clicks on one of the other components,
      | the modal component will receive this
      | callback.
      | 
      | The default implementation of this
      | method will play a beep, and bring the
      | currently modal component to the front,
      | but it can be overridden to do other tasks.
      | 
      | @see isCurrentlyBlockedByAnotherModalComponent,
      | canModalEventBeSentToComponent
      |
      */
    fn input_attempt_when_modal(&mut self);
}
