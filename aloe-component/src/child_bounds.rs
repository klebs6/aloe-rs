crate::ix!();

pub trait ChildBoundsChanged {

    /**
      | Called when one of this component's
      | children is moved or resized.
      | 
      | If the parent wants to know about changes
      | to its immediate children (not to children
      | of its children), this is the method
      | to override.
      | 
      | @see moved, resized, parentSizeChanged
      |
      */
    fn child_bounds_changed<'a>(&mut self, child: *mut Component<'a>);
}
