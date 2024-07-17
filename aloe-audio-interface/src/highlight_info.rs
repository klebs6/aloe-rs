crate::ix!();

/**
  | Used by the setParameterHighlighting()
  | method.
  |
  */
pub struct AudioProcessorEditorParameterControlHighlightInfo
{
    parameter_index:  i32,
    is_highlighted:   bool,
    suggested_colour: Colour,
}
