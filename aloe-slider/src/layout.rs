crate::ix!();

/**
  | A struct defining the placement of the
  | slider area and the text box area relative
  | to the bounds of the whole Slider component.
  |
  */
pub struct SliderLayout
{
    slider_bounds:   Rectangle<i32>,
    text_box_bounds: Rectangle<i32>,
}
