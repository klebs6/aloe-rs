crate::ix!();

pub trait SetVisible {

    /**
      | Makes the component visible or invisible.
      | 
      | This method will show or hide the component.
      | 
      | -----------
      | @note
      | 
      | components default to being non-visible
      | when first created. Also note that visible
      | components won't be seen unless all
      | their parent components are also visible.
      | 
      | This method will call visibilityChanged()
      | and also componentVisibilityChanged()
      | for any component listeners that are
      | interested in this component.
      | 
      | -----------
      | @param shouldBeVisible
      | 
      | whether to show or hide the component
      | @see isVisible, isShowing, visibilityChanged,
      | ComponentListener::componentVisibilityChanged
      |
      */
    fn set_visible(&mut self, should_be_visible: bool);
}

pub trait VisibilityChanged {

    /**
      | Called when this component's visibility
      | changes. @see setVisible, isVisible
      |
      */
    fn visibility_changed(&mut self);
}
