crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait ImageButtonLookAndFeelMethods
{
    fn draw_image_button(
        &mut self, 
        _0:             &mut Graphics,
        _1:             *mut Image,
        imagex:         i32,
        imagey:         i32,
        imagew:         i32,
        imageh:         i32,
        overlay_colour: &Colour,
        image_opacity:  f32,
        _8:             &mut ImageButton
    );
}

