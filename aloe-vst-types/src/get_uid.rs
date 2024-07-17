crate::ix!();

/**
  | @return
  | 
  | the TUID for a SKI interface which uses
  | the SKI::UID method.
  |
  */
#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
lazy_static!{
    /*
    template <typename T,
              typename std::enable_if<FUnknownPrivate::HasIIDType<T>::value>::type* = nullptr>
    const TUID& getTUID ()
    {
        return T::IID::toTUID ();
    }
    */
}

/**
  | @return
  | 
  | the TUID for a SKI interface which uses
  | the FUID and DECLARE_CLASS_IID method.
  |
  */
#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
lazy_static!{
    /*
    template <typename T,
              typename std::enable_if<!FUnknownPrivate::HasIIDType<T>::value>::type* = nullptr>
    const TUID& getTUID ()
    {
        return T::iid.toTUID ();
    }
    */
}


#[cfg(not(SMTG_CPP11_STDLIBSUPPORT))]
pub fn gettuid<'a, T>() -> &'a TUID {

    todo!();
        /*
            return T::iid.toTUID ();
        */
}
