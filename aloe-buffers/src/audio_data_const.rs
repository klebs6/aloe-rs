crate::ix!();

/**
  | Used as a template parameter for
  | AudioData::AudioDataPointer. Indicates
  | that the samples can only be used for
  | const data..
  |
  */
pub struct AudioDataConst {

}

pub mod const_ {

    pub type VoidType = libc::c_void;

    pub const isConst: usize = 1;
}

impl AudioDataConst {

    pub fn to_void_ptr(v: *mut const_::VoidType)  {
        
        todo!();
        /*
            return const_cast<void*> (v);
        */
    }
}
