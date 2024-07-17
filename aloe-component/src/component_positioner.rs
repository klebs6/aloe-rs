crate::ix!();

pub trait PositionerApplyNewBounds {

    /**
      | Attempts to set the component's position
      | to the given rectangle.
      | 
      | Unlike simply calling Component::setBounds(),
      | this may involve the positioner being
      | smart enough to adjust itself to fit
      | the new bounds.
      |
      */
    fn apply_new_bounds(&mut self, new_bounds: &Rectangle<i32>);
}

/**
  | Base class for objects that can be used
  | to automatically position a component
  | according to some kind of algorithm.
  | 
  | The component class simply holds onto
  | a reference to a ComponentPositioner, but doesn't
  | actually do anything with it - all the
  | functionality must be implemented
  | by the positioner itself (e.g. it might
  | choose to watch some kind of value and
  | move the component when the value changes).
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentPositioner<'a> {
    component: &'a mut Component<'a>,
}

impl<'a> ComponentPositioner<'a> {
    
    /**
      | Creates a ComponentPositioner which can control
      | the specified component.
      |
      */
    pub fn new(c: &mut Component<'a>) -> Self {
    
        todo!();
        /*
        : component(c),

        
        */
    }

    /**
      | Returns the component that this positioner
      | controls.
      |
      */
    pub fn get_component(&self) -> &mut Component<'a> {
        
        todo!();
        /*
            return component;
        */
    }
}
