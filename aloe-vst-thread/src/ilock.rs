crate::ix!();

/**
  | Lock interface declaration. @ingroup
  | baseLocks
  |
  */
pub trait ILock
{
    /**
      | Enables lock.
      |
      */
    fn lock(&mut self);

    /**
      | Disables lock.
      |
      */
    fn unlock(&mut self);

    /**
      | Tries to disable lock.
      |
      */
    fn trylock(&mut self) -> bool;
}

