crate::ix!();

/**
  | Stream with a size. \ingroup pluginBase
  | [extends IBStream] when stream type
  | supports it (like file and memory stream)
  |
  */
pub trait ISizeableStream: FUnknown {

    /**
      | Return the stream size
      |
      */
    #[PLUGIN_API]
    fn get_stream_size(&mut self, size: &mut i64) -> tresult;

    /**
      | Set the steam size. File streams can
      | only be resized if they are write enabled.
      |
      */
    #[PLUGIN_API]
    fn set_stream_size(&mut self, size: i64) -> tresult;
}

lazy_static!{
    /*
    static const FUID isizeable_stream_iid;
    */
}

declare_class_iid!{
    ISizeableStream, 
    0x04F9549E, 
    0xE02F4E6E, 
    0x87E86A87, 
    0x47F4E17F
}
