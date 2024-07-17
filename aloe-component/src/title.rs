crate::ix!();

pub trait SetName {

    /**
      | Sets the name of this component.
      | 
      | When the name changes, all registered
      | ComponentListeners will receive a
      | ComponentListener::componentNameChanged()
      | callback.
      | 
      | @see getName
      |
      */
    fn set_name(&mut self, new_name: &String);
}

pub trait SetTitle {

    /**
      | Changes the title of the window.
      |
      */
    fn set_title(&mut self, title: &String);
}
