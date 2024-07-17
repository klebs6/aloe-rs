crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_ComponentListener.h]

/**
  | Gets informed about changes to a component's
  | hierarchy or position.
  | 
  | To monitor a component for changes,
  | register a subclass of ComponentListener
  | with the component using Component::addComponentListener().
  | 
  | Be sure to deregister listeners before
  | you delete them!
  | 
  | @see Component::addComponentListener,
  | Component::removeComponentListener
  | 
  | @tags{GUI}
  |
  */
pub trait ComponentListener: 
ComponentEnablementChanged
+ ComponentBeingDeleted
+ ComponentNameChanged
+ ComponentParentHierarchyChanged
+ ComponentChildrenChanged
+ ComponentVisibilityChanged
+ ComponentBroughtToFront
+ ComponentMovedOrResized { }

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_ComponentListener.cpp]

pub trait ComponentMovedOrResized {

    /**
      | Called when the component's position
      | or size changes.
      | 
      | -----------
      | @param component
      | 
      | the component that was moved or resized
      | ----------
      | @param wasMoved
      | 
      | true if the component's top-left corner
      | has just moved
      | ----------
      | @param wasResized
      | 
      | true if the component's width or height
      | has just changed @see Component::setBounds,
      | Component::resized, Component::moved
      |
      | This callback happens when the component
      | that is being watched is moved relative
      | to its top-level peer window, or when
      | it is resized.
      |
      */
    fn component_moved_or_resized<'a>(
        &mut self, 
        component:   &mut Component<'a>,
        was_moved:   bool,
        was_resized: bool) {}
}

pub trait ComponentBroughtToFront {

    /**
      | Called when the component is brought
      | to the top of the z-order.
      | 
      | -----------
      | @param component
      | 
      | the component that was moved @see Component::toFront,
      | Component::broughtToFront
      |
      */
    fn component_brought_to_front<'a>(&mut self, component: &mut Component<'a>) {}
}

pub trait ComponentVisibilityChanged {

    /**
      | Called when the component is made visible
      | or invisible.
      | 
      | -----------
      | @param component
      | 
      | the component that changed @see Component::setVisible
      |
      */
    fn component_visibility_changed<'a>(&mut self, component: &mut Component<'a>) {}
}

pub trait ComponentChildrenChanged {

    /**
      | Called when the component has children
      | added or removed, or their z-order changes.
      | 
      | -----------
      | @param component
      | 
      | the component whose children have changed
      | @see Component::childrenChanged,
      | Component::addChildComponent,
      | 
      | Component::removeChildComponent
      |
      */
    fn component_children_changed<'a>(&mut self, component: &mut Component<'a>) {}
}

pub trait ComponentParentHierarchyChanged {

    /**
      | Called to indicate that the component's
      | parents have changed.
      | 
      | When a component is added or removed
      | from its parent, all of its children
      | will produce this notification (recursively
      | - so all children of its children will
      | also be called as well).
      | 
      | -----------
      | @param component
      | 
      | the component that this listener is
      | registered with @see Component::parentHierarchyChanged
      |
      */
    fn component_parent_hierarchy_changed(&mut self, component: &mut Component<'_>) {}
}

pub trait ComponentNameChanged {

    /**
      | Called when the component's name is
      | changed.
      | 
      | -----------
      | @param component
      | 
      | the component that had its name changed
      | @see Component::setName, Component::getName
      |
      */
    fn component_name_changed<'a>(&mut self, component: &mut Component<'a>) {}
}

pub trait ComponentBeingDeleted {

    /**
      | Called when the component is in the process
      | of being deleted.
      | 
      | This callback is made from inside the
      | destructor, so be very, very cautious
      | about what you do in here.
      | 
      | In particular, bear in mind that it's
      | the Component base class's destructor
      | that calls this - so if the object that's
      | being deleted is a subclass of Component<'a>,
      | then the subclass layers of the object
      | will already have been destructed when
      | it gets to this point!
      | 
      | -----------
      | @param component
      | 
      | the component that was deleted
      |
      */
    fn component_being_deleted<'a>(&mut self, component: &mut Component<'a>) {}
}

pub trait ComponentEnablementChanged {

    /**
      | Called when the component's enablement
      | is changed.
      | 
      | -----------
      | @param component
      | 
      | the component that had its enablement
      | changed @see Component::setEnabled,
      | Component::isEnabled, Component::enablementChanged
      |
      */
    fn component_enablement_changed<'a>(&mut self, component: &mut Component<'a>) {}
}
