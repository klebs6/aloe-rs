crate::ix!();

/**
  | FLockObj declaration. Reference counted
  | lock @ingroup baseLocks
  |
  */
pub struct FLockObject {
    base:  FObject,
    base2: FLock,
}

obj_methods!{
    FLockObject, FObject
}

