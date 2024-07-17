crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_KeyboardFocusTraverser.h]

/**
  | Controls the order in which keyboard
  | focus moves between components.
  | 
  | The default behaviour of this class
  | uses a FocusTraverser object internally
  | to determine the default/next/previous
  | component until it finds one which wants
  | keyboard focus, as set by the Component::setWantsKeyboardFocus()
  | method.
  | 
  | If you need keyboard focus traversal
  | in a more customised way, you can create
  | a subclass of ComponentTraverser that
  | uses your own algorithm, and use
  | 
  | Component::createKeyboardFocusTraverser()
  | to create it.
  | 
  | @see FocusTraverser, ComponentTraverser,
  | Component::createKeyboardFocusTraverser
  | 
  | @tags{GUI}
  |
  */
pub struct KeyboardFocusTraverser {

}

impl ComponentTraverser for KeyboardFocusTraverser {

}

impl GetAllComponents for KeyboardFocusTraverser {

    /**
      | Returns all of the components that can
      | receive keyboard focus within the given
      | parent component in traversal order.
      | 
      | The default implementation will return
      | all focusable child components (as
      | determined by FocusTraverser) that
      | also wants keyboard focus.
      |
      */
    fn get_all_components<'a>(&mut self, parent_component: *mut Component<'a>) 
        -> Vec<*mut Component<'a>> 
    {
        todo!();
        /*
            std::vector<Component*> components;
        FocusHelpers::findAllComponents (parentComponent,
                                         components,
                                         &Component::isKeyboardFocusContainer);

        auto removePredicate = [parentComponent] (const Component* comp)
        {
            return ! KeyboardFocusTraverserHelpers::isKeyboardFocusable (comp, parentComponent);
        };

        components.erase (std::remove_if (std::begin (components), std::end (components), std::move (removePredicate)),
                          std::end (components));

        return components;
        */
    }
}

impl GetPreviousComponent for KeyboardFocusTraverser {

    /**
      | Returns the component that should be
      | given keyboard focus after the specified
      | one when moving "backwards".
      | 
      | The default implementation will return
      | the previous focusable component (as
      | determined by FocusTraverser) that
      | also wants keyboard focus, or nullptr
      | if there is no suitable component.
      |
      */
    fn get_previous_component(&mut self, current: *mut Component<'_>) -> *mut Component<'_> 
    {
        todo!();
        /*
            return KeyboardFocusTraverserHelpers::traverse (current, current->findKeyboardFocusContainer(),
                                                        FocusHelpers::NavigationDirection::backwards);
        */
    }
}

impl GetNextComponent for KeyboardFocusTraverser {

    /**
      | Returns the component that should be
      | given keyboard focus after the specified
      | one when moving "forwards".
      | 
      | The default implementation will return
      | the next focusable component (as determined
      | by FocusTraverser) that also wants
      | keyboard focus, or nullptr if there
      | is no suitable component.
      |
      */
    fn get_next_component(&mut self, current: *mut Component<'_>) -> *mut Component<'_> 
    {
        todo!();
        /*
            return KeyboardFocusTraverserHelpers::traverse (current, current->findKeyboardFocusContainer(),
                                                        FocusHelpers::NavigationDirection::forwards);
        */
    }
}

impl GetDefaultComponent for KeyboardFocusTraverser {

    /**
      | Returns the component that should receive
      | keyboard focus by default within the
      | given parent component.
      | 
      | The default implementation will return
      | the foremost focusable component (as
      | determined by FocusTraverser) that
      | also wants keyboard focus, or nullptr
      | if there is no suitable component.
      |
      */
    fn get_default_component(&mut self, parent_component: *mut Component<'_>) 
        -> *mut Component<'_> 
    {
        todo!();
        /*
            for (auto* comp : getAllComponents (parentComponent))
            if (KeyboardFocusTraverserHelpers::isKeyboardFocusable (comp, parentComponent))
                return comp;

        return nullptr;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_KeyboardFocusTraverser.cpp]
impl KeyboardFocusTraverser {

    pub fn is_keyboard_focusable<'a>(
        comp:      *const Component<'a>,
        container: *const Component<'a>) -> bool {

        todo!();
        /*
            return comp->getWantsKeyboardFocus() && container->isParentOf (comp);
        */
    }

    pub fn traverse<'a>(
        current:   *mut Component<'a>,
        container: *mut Component<'a>,
        direction: NavigationDirection

    ) -> *mut Component<'a> {
        
        todo!();
        /*
            if (auto* comp = FocusHelpers::navigateFocus (current, container, direction,
                                                              &Component::isKeyboardFocusContainer))
                {
                    if (isKeyboardFocusable (comp, container))
                        return comp;

                    return traverse (comp, container, direction);
                }

                return nullptr;
        */
    }
}
