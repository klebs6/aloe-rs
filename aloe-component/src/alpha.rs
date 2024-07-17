crate::ix!();

pub trait SetAlpha {

    /**
      | Changes the window's transparency.
      |
      */
    fn set_alpha(&mut self, new_alpha: f32);
}

pub trait AlphaChanged {

    /**
      | Called when setAlpha() is used to change
      | the alpha value of this component.
      | 
      | If you override this, you should also
      | invoke the base class's implementation
      | during your overridden function, as
      | it performs some repainting behaviour.
      |
      */
    fn alpha_changed(&mut self);
}
