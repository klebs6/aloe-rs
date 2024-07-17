crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait CallOutBoxLookAndFeelMethods {

    fn draw_call_out_box_background(
        &mut self, 
        _0: &mut CallOutBox,
        _1: &mut Graphics,
        _2: &Path,
        _3: &mut Image
    );

    fn get_call_out_box_border_size(&mut self, _0: &CallOutBox) -> i32;

    fn get_call_out_box_corner_size(&mut self, _0: &CallOutBox) -> f32;
}

