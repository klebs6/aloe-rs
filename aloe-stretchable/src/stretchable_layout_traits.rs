crate::ix!();

pub trait HasBeenMoved {

    /**
      | This is called when the bar is dragged.
      | 
      | This method must update the positions
      | of any components whose position is
      | determined by the StretchableLayoutManager,
      | because they might have just moved.
      | 
      | The default implementation calls the
      | resized() method of this component's
      | parent component, because that's often
      | where you're likely to apply the layout,
      | but it can be overridden for more specific
      | needs.
      |
      */
    fn has_been_moved(&mut self);
}
