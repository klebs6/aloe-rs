crate::ix!();

/**
  | Used to receive callbacks for text changes
  |
  */
pub trait TextPropertyComponentListener {

    /**
      | Called when text has finished being
      | entered (i.e. not per keypress) has
      | changed.
      |
      */
    fn text_property_component_changed(&mut self, _0: *mut TextPropertyComponent);
}
