crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/smartpointer.h]

/**
  | Assigning newly created object to an
  | IPtr.
  | 
  | Example:
  | 
  | -----------
  | @code
  | 
  | IPtr<IPath> path = owned (FHostCreate (IPath, hostClasses));
  | which is a slightly shorter form of writing:
  | ----------
  | @code
  | 
  | IPtr<IPath> path = OPtr<IPath> (FHostCreate (IPath, hostClasses));
  |
  */
pub fn owned<I>(p: *mut I) -> IPtr<I> {

    todo!();
        /*
            return IPtr<I> (p, false);
        */
}

/**
  | Assigning shared object to an IPtr.
  | 
  | Example:
  | 
  | -----------
  | @code
  | 
  | IPtr<IPath> path = shared (iface.getXY ());
  |
  */
pub fn shared<I>(p: *mut I) -> IPtr<I> {

    todo!();
        /*
            return IPtr<I> (p, true);
        */
}

/* ------------ * Ownership functionality  ------------ */

/**
  | Strong typedef for shared reference
  | counted objects.
  | 
  | Use SKI::adopt to unwrap the provided
  | object.
  | 
  | -----------
  | @param T
  | 
  | Referenced counted type.
  |
  */
pub struct SKIShared<T> {
    obj: *mut T, // default = nullptr
}

/**
  | Strong typedef for transferring the
  | ownership of reference counted objects.
  | 
  | Use SKI::adopt to unwrap the provided
  | object.
  | 
  | After calling adopt the reference in
  | this object is null.
  | 
  | -----------
  | @param T
  | 
  | Referenced counted type.
  |
  */
pub struct SKIOwned<T> {
    obj: *mut T, // default = nullptr
}

/**
  | Strong typedef for using reference
  | counted objects.
  | 
  | Use SKI::adopt to unwrap the provided
  | object.
  | 
  | After calling adopt the reference in
  | this object is null.
  | 
  | -----------
  | @param T
  | 
  | Referenced counted type.
  |
  */
pub struct SKIUsed<T> {
    obj: *mut T, // default = nullptr
}
    
///--------------------
pub struct SKIAdopt {

}

impl SKIAdopt {

    pub fn adopt_shared<T>(ref_: &mut SKIShared<T>) -> IPtr<T> {
    
        todo!();
        /*
            using typename Steinberg::shared;
            return shared (ref.obj);
        */
    }
    
    pub fn adopt_owned<T>(ref_: &mut SKIOwned<T>) -> IPtr<T> {
    
        todo!();
        /*
            using typename Steinberg::owned;
            IPtr<T> out = owned (ref.obj);
            ref.obj = nullptr;
            return out;
        */
    }
    
    pub fn adopt_used<T>(ref_: &mut SKIUsed<T>) -> *mut T {
    
        todo!();
        /*
            return ref.obj;
        */
    }
    
    pub fn to_owner_type<OwnerType, T>(obj: *mut T) -> OwnerType {
    
        todo!();
        /*
            OwnerType<T> out;
            out.obj = obj;
            return out;
        */
    }
}

pub trait SkiAdopt<T> {

    fn ski_adopt(ref_: Self) -> IPtr<T>;
}

/**
  | Common function to adopt referenced
  | counted object.
  | 
  | -----------
  | @param T
  | 
  | Referenced counted type.
  | ----------
  | @param ref
  | 
  | The reference to be adopted in a smart
  | pointer.
  |
  */
impl<T> SkiAdopt<T> for &mut SKIShared<T> {

    fn ski_adopt(ref_: Self) -> IPtr<T> {

        todo!();
            /*
                return Detail::SKIAdopt::adopt (ref);
            */
    }
}

impl<T> SkiAdopt<T> for SKIShared<T> {

    fn ski_adopt(ref_: Self) -> IPtr<T> {

        todo!();
            /*
                return Detail::SKIAdopt::adopt (ref);
            */
    }
}

impl<T> SkiAdopt<T> for &mut SKIOwned<T> {

    /**
      | Common function to adopt referenced
      | counted object.
      | 
      | -----------
      | @param T
      | 
      | Referenced counted type.
      | ----------
      | @param ref
      | 
      | The reference to be adopted in a smart
      | pointer.
      |
      */
    fn ski_adopt(ref_: Self) -> IPtr<T> {

        todo!();
            /*
                return Detail::SKIAdopt::adopt (ref);
            */
    }
}

impl<T> SkiAdopt<T> for SKIOwned<T> {

    fn ski_adopt(ref_: Self) -> IPtr<T> {

        todo!();
            /*
                return Detail::SKIAdopt::adopt (ref);
            */
    }
}

impl<T> SkiAdopt<T> for &mut SKIUsed<T> {

    /**
      | Common function to adopt referenced
      | counted object.
      | 
      | -----------
      | @param T
      | 
      | Referenced counted type.
      | ----------
      | @param ref
      | 
      | The reference to be adopted in a smart
      | pointer.
      |
      */
    fn ski_adopt(ref_: Self) -> IPtr<T> {

        todo!();
            /*
                return Detail::SKIAdopt::adopt (ref);
            */
    }
}

impl<T> SkiAdopt<T> for SKIUsed<T> {

    fn ski_adopt(ref_: Self) -> IPtr<T> {

        todo!();
            /*
                return Detail::SKIAdopt::adopt (ref);
            */
    }
}

/**
  | Common function to wrap owned instances.
  |
  */
pub fn ski_to_owned<T>(obj: *mut T) -> SKIOwned<T> {

    todo!();
        /*
            return Detail::SKIAdopt::toOwnerType<SKIOwned> (obj);
        */
}

/**
  | Common function to wrap shared instances.
  |
  */
pub fn ski_to_shared<T>(obj: *mut T) -> SKIShared<T> {

    todo!();
        /*
            return Detail::SKIAdopt::toOwnerType<SKIShared> (obj);
        */
}

/**
  | Common function to wrap used instances.
  |
  */
pub fn ski_to_used<T>(obj: *mut T) -> SKIUsed<T> {

    todo!();
        /*
            return Detail::SKIAdopt::toOwnerType<SKIUsed> (obj);
        */
}
