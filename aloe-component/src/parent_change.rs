crate::ix!();

pub trait ParentSizeChanged {

    /**
      | Called when this component's immediate
      | parent has been resized.
      | 
      | If the component is a top-level window,
      | this indicates that the screen size
      | has changed.
      | 
      | @see childBoundsChanged, moved, resized
      |
      */
    fn parent_size_changed(&mut self);
}


