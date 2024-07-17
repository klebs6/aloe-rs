crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait ConcertinaPanelLookAndFeelMethods
{
    fn draw_concertina_panel_header(&mut self, 
        _0:            &mut Graphics,
        area:          &Rectangle<i32>,
        is_mouse_over: bool,
        is_mouse_down: bool,
        _4:            &mut ConcertinaPanel,
        _5:            &mut Component);
}
