crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_FocusTraverser.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_FocusTraverser.cpp]

pub enum NavigationDirection { 
    forwards,
    backwards
}

/**
  | Controls the order in which focus moves
  | between components.
  | 
  | The algorithm used by this class to work
  | out the order of traversal is as follows:
  | 
  | - Only visible and enabled components
  | are considered focusable.
  | 
  | - If two components both have an explicit
  | focus order specified then the one with
  | the lowest number comes first (see the
  | Component::setExplicitFocusOrder()
  | method).
  | 
  | - Any component with an explicit focus
  | order greater than 0 comes before ones
  | that don't have an order specified.
  | 
  | - Components with their 'always on top'
  | flag set come before those without.
  | 
  | - Any unspecified components are traversed
  | in a left-to-right, then top-to-bottom
  | order.
  | 
  | If you need focus traversal in a more
  | customised way you can create a
  | 
  | ComponentTraverser subclass that
  | uses your own algorithm and return it
  | from Component::createFocusTraverser().
  | 
  | @see ComponentTraverser, Component::createFocusTraverser
  | 
  | @tags{GUI}
  |
  */
pub struct FocusTraverser;

impl ComponentTraverser for FocusTraverser { }

impl GetAllComponents for FocusTraverser { 

    /**
      | Returns all of the components that can
      | receive focus within the given parent
      | component in traversal order.
      | 
      | The default implementation will return
      | all visible and enabled child components.
      |
      */
    fn get_all_components<'a>(&mut self, parent_component: *mut Component<'a>) -> Vec<*mut Component<'a>> {
        
        todo!();
        /*
            std::vector<Component*> components;
        FocusHelpers::findAllComponents (parentComponent,
                                         components,
                                         &Component::isFocusContainer);

        return components;
        */
    }
}

impl GetPreviousComponent for FocusTraverser { 

    /**
      | Returns the component that should be
      | given focus after the specified one
      | when moving "backwards".
      | 
      | The default implementation will return
      | the previous visible and enabled component
      | which is to the left of or above this one,
      | and will return nullptr if there is no
      | suitable component.
      |
      */
    fn get_previous_component<'a>(&mut self, current: *mut Component<'a>) -> *mut Component {
        
        todo!();
        /*
            jassert (current != nullptr);

        return FocusHelpers::navigateFocus (current,
                                            current->findFocusContainer(),
                                            FocusHelpers::NavigationDirection::backwards,
                                            &Component::isFocusContainer);
        */
    }
}

impl GetNextComponent for FocusTraverser { 

    /**
      | Returns the component that should be
      | given focus after the specified one
      | when moving "forwards".
      | 
      | The default implementation will return
      | the next visible and enabled component
      | which is to the right of or below this
      | one, and will return nullptr if there
      | is no suitable component.
      |
      */
    fn get_next_component<'a>(&mut self, current: *mut Component<'a>) -> *mut Component {
        
        todo!();
        /*
            jassert (current != nullptr);

        return FocusHelpers::navigateFocus (current,
                                            current->findFocusContainer(),
                                            FocusHelpers::NavigationDirection::forwards,
                                            &Component::isFocusContainer);
        */
    }
}

impl GetDefaultComponent for FocusTraverser { 

    /**
      | Returns the component that should receive
      | focus by default within the given parent
      | component.
      | 
      | The default implementation will just
      | return the foremost visible and enabled
      | child component, and will return nullptr
      | if there is no suitable component.
      |
      */
    fn get_default_component<'a>(&mut self, parent_component: *mut Component<'a>) -> *mut Component {
        
        todo!();
        /*
            if (parentComponent != nullptr)
        {
            std::vector<Component*> components;
            FocusHelpers::findAllComponents (parentComponent,
                                             components,
                                             &Component::isFocusContainer);

            if (! components.empty())
                return components.front();
        }

        return nullptr;
        */
    }
}

pub fn focus_traverser_get_order<'a>(c: *const Component<'a>) -> i32 {
    
    todo!();
    /*
        auto order = c->getExplicitFocusOrder();
            return order > 0 ? order : std::numeric_limits<int>::max();
    */
}

pub fn focus_traverser_find_all_components<'a,FocusContainerFn>(
    parent:             *mut Component<'a>,
    components:         &mut Vec<*mut Component<'a>>,
    is_focus_container: FocusContainerFn)  {

    todo!();
    /*
        if (parent == nullptr || parent->getNumChildComponents() == 0)
                return;

            std::vector<Component*> localComponents;

            for (auto* c : parent->getChildren())
                if (c->isVisible() && c->isEnabled())
                    localComponents.push_back (c);

            const auto compareComponents = [&] (const Component* a, const Component* b)
            {
                const auto getComponentOrderAttributes = [] (const Component* c)
                {
                    return std::make_tuple (getOrder (c),
                                            c->isAlwaysOnTop() ? 0 : 1,
                                            c->getY(),
                                            c->getX());
                };

                return getComponentOrderAttributes (a) < getComponentOrderAttributes (b);
            };

            // This will sort so that they are ordered in terms of explicit focus,
            // always on top, left-to-right, and then top-to-bottom.
            std::stable_sort (localComponents.begin(), localComponents.end(), compareComponents);

            for (auto* c : localComponents)
            {
                components.push_back (c);

                if (! (c->*isFocusContainer)())
                    findAllComponents (c, components, isFocusContainer);
            }
    */
}

pub fn focus_traverser_navigate_focus<'a,FocusContainerFn>(
    current:            *mut Component<'a>,
    focus_container:    *mut Component<'a>,
    direction:          NavigationDirection,
    is_focus_container: FocusContainerFn

) -> *mut Component<'a> {

    todo!();
    /*
        if (focusContainer != nullptr)
            {
                std::vector<Component*> components;
                findAllComponents (focusContainer, components, isFocusContainer);

                const auto iter = std::find (components.cbegin(), components.cend(), current);

                if (iter == components.cend())
                    return nullptr;

                switch (direction)
                {
                    case NavigationDirection::forwards:
                        if (iter != std::prev (components.cend()))
                            return *std::next (iter);

                        break;

                    case NavigationDirection::backwards:
                        if (iter != components.cbegin())
                            return *std::prev (iter);

                        break;
                }
            }

            return nullptr;
    */
}
