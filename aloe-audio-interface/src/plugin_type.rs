crate::ix!();

pub trait GetWrapperTypeDescription {

    /**
      | Returns a textual description of a AudioProcessorWrapperType
      | value
      |
      */
    fn get_wrapper_type_description(_0: AudioProcessorWrapperType) 
        -> *const u8 where Self: Sized;
}

pub trait SetTypeOfNextNewPlugin {

    fn set_type_of_next_new_plugin(_0: AudioProcessorWrapperType) where Self: Sized;
}
