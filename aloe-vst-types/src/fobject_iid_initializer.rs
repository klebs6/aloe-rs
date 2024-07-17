crate::ix!();

pub struct FObjectIIDInitializer {

}

impl Default for FObjectIIDInitializer {
    
    /**
      | the object iid is always generated so that
      | different components only can cast to their
      | own objects this initializer must be after
      | the definition of FObject::iid, otherwise
      | the default constructor of FUID will clear
      | the generated iid
      */
    fn default() -> Self {
        todo!();
        /*
            const_cast<FUID&> (FObject::iid).generate ();
        */
    }
}

lazy_static!{ 
    static ref gFObjectIidInitializer: FObjectIIDInitializer = FObjectIIDInitializer::default();
}
