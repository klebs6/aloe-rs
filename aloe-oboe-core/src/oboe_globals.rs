crate::ix!();

#[derive(Clone)]
pub struct OboeGlobals {

}

lazy_static!{
    /*
    static bool mWorkaroundsEnabled;
    */
}

impl OboeGlobals {

    pub fn are_workarounds_enabled() -> bool {
        
        todo!();
        /*
            return mWorkaroundsEnabled;
        */
    }

    /**
      | Disable this when writing tests to reproduce
      | bugs in AAudio or OpenSL ES that have
      | workarounds in Oboe. @param enabled
      |
      */
    pub fn set_workarounds_enabled(enabled: bool)  {
        
        todo!();
        /*
            mWorkaroundsEnabled = enabled;
        */
    }
}
