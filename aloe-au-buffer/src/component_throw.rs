crate::ix!();

/**
  | make this usable outside the stricter
  | context of AudiUnits
  |
  */
#[cfg(not(COMPONENT_THROW))]
macro_rules! component_throw {
    ($err:ident) => {
        /*
        
                do { DebugMessage(#err); throw static_cast<OSStatus>(err); } while (0)
        */
    }
}
