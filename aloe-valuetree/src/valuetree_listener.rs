crate::ix!();

/**
  | ValueTreeListener class for events that happen
  | to a ValueTree.
  | 
  | To get events from a ValueTree, make
  | your class implement this interface,
  | and use ValueTree::addListener()
  | and ValueTree::removeListener()
  | to register it.
  |
  */
pub trait ValueTreeListener {

    /**
      | This method is called when a property
      | of this tree (or of one of its sub-trees)
      | is changed.
      | 
      | -----------
      | @note
      | 
      | when you register a listener to a tree,
      | it will receive this callback for property
      | changes in that tree, and also for any
      | of its children, (recursively, at any
      | depth). If your tree has sub-trees but
      | you only want to know about changes to
      | the top level tree, simply check the
      | tree parameter in this callback to make
      | sure it's the tree you're interested
      | in.
      |
      */
    fn value_tree_property_changed(&mut self, 
        tree_whose_property_has_changed: &mut ValueTree,
        property:                        &Identifier) {}

    /**
      | This method is called when a child sub-tree
      | is added.
      | 
      | -----------
      | @note
      | 
      | when you register a listener to a tree,
      | it will receive this callback for child
      | changes in both that tree and any of its
      | children, (recursively, at any depth).
      | If your tree has sub-trees but you only
      | want to know about changes to the top
      | level tree, just check the parentTree
      | parameter to make sure it's the one that
      | you're interested in.
      |
      */
    fn value_tree_child_added(&mut self, 
        parent_tree:                &mut ValueTree,
        child_which_has_been_added: &mut ValueTree) {}

    /**
      | This method is called when a child sub-tree
      | is removed.
      | 
      | -----------
      | @note
      | 
      | when you register a listener to a tree,
      | it will receive this callback for child
      | changes in both that tree and any of its
      | children, (recursively, at any depth).
      | If your tree has sub-trees but you only
      | want to know about changes to the top
      | level tree, just check the parentTree
      | parameter to make sure it's the one that
      | you're interested in.
      |
      */
    fn value_tree_child_removed(&mut self, 
        parent_tree:                        &mut ValueTree,
        child_which_has_been_removed:       &mut ValueTree,
        index_from_which_child_was_removed: i32) {}

    /**
      | This method is called when a tree's children
      | have been re-shuffled.
      | 
      | -----------
      | @note
      | 
      | when you register a listener to a tree,
      | it will receive this callback for child
      | changes in both that tree and any of its
      | children, (recursively, at any depth).
      | If your tree has sub-trees but you only
      | want to know about changes to the top
      | level tree, just check the parameter
      | to make sure it's the tree that you're
      | interested in.
      |
      */
    fn value_tree_child_order_changed(&mut self, 
        parent_tree_whose_children_have_moved: &mut ValueTree,
        old_index:                             i32,
        new_index:                             i32) {}

    /**
      | This method is called when a tree has
      | been added or removed from a parent.
      | 
      | This callback happens when the tree
      | to which the listener was registered
      | is added or removed from a parent. Unlike
      | the other callbacks, it applies only
      | to the tree to which the listener is registered,
      | and not to any of its children.
      |
      */
    fn value_tree_parent_changed(&mut self, tree_whose_parent_has_changed: &mut ValueTree) {}

    /**
      | This method is called when a tree is made
      | to point to a different internal shared
      | object.
      | 
      | When operator= is used to make a ValueTree
      | refer to a different object, this callback
      | will be made.
      |
      */
    fn value_tree_redirected(&mut self, tree_which_has_been_changed: &mut ValueTree) {}
}
