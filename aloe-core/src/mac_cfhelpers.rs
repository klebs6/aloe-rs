/*!
  | This file contains a few helper functions
  | that are used internally but which need
  | to be kept away from the public headers
  | because they use obj-C symbols.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_mac_CFHelpers.h]

pub struct CFUniquePtr<T>(Box<T>);

impl<T> Drop for CFUniquePtr<T> {

    fn drop(&mut self) {
        todo!();
        /*
            if (object != nullptr)
                CFRelease (object);
        */
    }
}

impl<T> Deref for CFUniquePtr<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target { &*self.0 }
}

#[derive(Default)]
pub struct CFObjectHolder<CFType> {

    /**
      | Public to facilitate passing the pointer
      | address to functions
      |
      */
    object: CFType, // default = nullptr
}

impl<CFType> Drop for CFObjectHolder<CFType> {

    fn drop(&mut self) {
        todo!();
        /* 
            if (object != nullptr)
                CFRelease (object);
         */
    }
}
