crate::ix!();

pub trait EditorViewportPositionChanged {

    /**
      | Called when the view position is scrolled
      | horizontally or vertically.
      |
      */
    fn editor_viewport_position_changed(&mut self);
}
