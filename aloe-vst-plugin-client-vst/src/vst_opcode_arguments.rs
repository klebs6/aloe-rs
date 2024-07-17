crate::ix!();

/**
  | Use the same names as the VST SDK.
  |
  */
pub struct VstOpCodeArguments
{
    index: i32,
    value: PointerSizedInt,
    ptr:   *mut c_void,
    opt:   f32,
}
