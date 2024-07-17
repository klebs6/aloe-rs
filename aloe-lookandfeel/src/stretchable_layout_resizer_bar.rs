crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait StretchableLayoutResizerBarLookAndFeelMethods
{
    fn draw_stretchable_layout_resizer_bar(&mut self, 
        _0:                &mut Graphics,
        w:                 i32,
        h:                 i32,
        is_vertical_bar:   bool,
        is_mouse_over:     bool,
        is_mouse_dragging: bool);
}
