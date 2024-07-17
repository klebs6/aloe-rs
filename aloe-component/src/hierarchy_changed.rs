crate::ix!();

pub trait ParentHierarchyChanged {

    /**
      | Called to indicate that the component's
      | parents have changed.
      | 
      | When a component is added or removed
      | from its parent, this method will be
      | called on all of its children (recursively
      | - so all children of its children will
      | also be called as well).
      | 
      | Subclasses can override this if they
      | need to react to this in some way.
      | 
      | @see getParentComponent, isShowing,
      | ComponentListener::componentParentHierarchyChanged
      |
      */
    fn parent_hierarchy_changed(&mut self);
}

pub trait ChildrenChanged {

    /**
      | Subclasses can use this callback to
      | be told when children are added or removed,
      | or when their z-order changes. @see
      | parentHierarchyChanged, ComponentListener::componentChildrenChanged
      |
      */
    fn children_changed(&mut self);
}


