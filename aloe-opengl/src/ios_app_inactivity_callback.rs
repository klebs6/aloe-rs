crate::ix!();

/**
  | NB: this is a duplicate of an internal
  | declaration in aloe_core
  |
  */
#[cfg(target_os="ios")]
pub trait AppInactivityCallback {
    fn app_becoming_inactive(&mut self);
}

#[cfg(target_os="ios")]
lazy_static!{
    /*
    extern Vec<AppInactivityCallback*> appBecomingInactiveCallbacks;
    */
}
