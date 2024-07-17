crate::ix!();

//----------------cpp/Aloe/modules/aloe_gui_basics/components/aloe_ComponentTraverser.h]

/**
  | Base class for traversing components.
  | 
  | If you need custom focus or keyboard
  | focus traversal for a component you
  | can create a subclass of ComponentTraverser
  | and return it from
  | 
  | Component::createFocusTraverser()
  | or Component::createKeyboardFocusTraverser().
  | 
  | @see Component::createFocusTraverser,
  | Component::createKeyboardFocusTraverser
  | 
  | @tags{GUI}
  |
  */
pub trait ComponentTraverser: 
GetDefaultComponent
+ GetNextComponent 
+ GetPreviousComponent
+ GetAllComponents { }

pub trait GetDefaultComponent {

    /**
      | Returns the component that should be
      | used as the traversal entry point within
      | the given parent component.
      | 
      | This must return nullptr if there is
      | no default component.
      |
      */
    fn get_default_component<'a>(&mut self, 
        parent_component: *mut Component<'a>) -> *mut Component;
}

pub trait GetNextComponent {

    /**
      | Returns the component that comes after
      | the specified one when moving "forwards".
      | 
      | This must return nullptr if there is
      | no next component.
      |
      */
    fn get_next_component<'a>(&mut self, 
        current: *mut Component<'a>) -> *mut Component;
}

pub trait GetPreviousComponent {

    /**
      | Returns the component that comes after
      | the specified one when moving "backwards".
      | 
      | This must return nullptr if there is
      | no previous component.
      |
      */
    fn get_previous_component<'a>(&mut self, 
        current: *mut Component<'a>) -> *mut Component;
}

pub trait GetAllComponents {

    /**
      | Returns all of the traversable components
      | within the given parent component in
      | traversal order.
      |
      */
    fn get_all_components<'a>(&mut self, 
        parent_component: *mut Component<'a>) -> Vec<*mut Component<'a>>;
}
