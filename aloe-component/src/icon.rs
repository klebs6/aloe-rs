crate::ix!();

pub trait SetIcon {

    /**
      | Attempts to change the icon associated
      | with this window.
      |
      */
    fn set_icon(&mut self, new_icon: &Image);
}
