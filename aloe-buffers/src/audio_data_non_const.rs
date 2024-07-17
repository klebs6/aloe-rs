/*!
  | These types can be used as the Constness
  | template parameter for the
  | AudioData::AudioDataPointer class.
  |
  */

crate::ix!();

/** 
  | Used as a template parameter for
  | AudioData::AudioDataPointer. Indicates that the
  | pointer can be used for non-const data. 
  */
pub struct AudioDataNonConst {

}

pub mod non_const {
    pub const isConst: usize = 0;
    pub type VoidType = libc::c_void;
}

impl AudioDataNonConst {

    pub fn to_void_ptr(v: *mut non_const::VoidType)  {
        
        todo!();
        /*
            return v;
        */
    }
}
