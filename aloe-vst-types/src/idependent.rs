crate::ix!();

pub enum IDependentChangeMessage 
{
    WillChange,
    Changed,
    Destroyed,
    WillDestroy,
}

impl IDependentChangeMessage {

    pub fn std_change_message_last() -> Self {
        Self::WillDestroy
    }
}

/**
  | A dependent will get notified about
  | changes of a model. [plug imp]
  | 
  | - notify changes of a model
  | 
  | \see IUpdateHandler \ingroup frameworkHostClasses
  |
  */
pub trait IDependent: FUnknown {

    /**
      | Inform the dependent, that the passed
      | FUnknown has changed.
      |
      */
    #[PLUGIN_API]
    fn update(
        &mut self, 
        changed_unknown: *mut dyn FUnknown,
        message:         i32
    );
}

lazy_static!{
    /*
    static const FUID independent_iid;
    */
}

declare_class_iid!{
    IDependent, 
    0xF52B7AAE, 
    0xDE72416d, 
    0x8AF18ACE, 
    0x9DD7BD5E
}

#[inline] pub fn assign_shared_dependent_from_iptr<T>(
    this:    *mut dyn IDependent,
    dest:    &mut *mut T,
    new_ptr: *mut T

) {

    todo!();
        /*
            if (dest == newPtr)
            return;

        if (dest)
            dest->removeDependent (_this);
        AssignShared (dest, newPtr);
        if (dest)
            dest->addDependent (_this);
        */
}

#[inline] pub fn assign_shared_dependent<T>(
    this:    *mut dyn IDependent,
    dest:    &mut IPtr<T>,
    new_ptr: *mut T

) {

    todo!();
        /*
            if (dest == newPtr)
            return;

        if (dest)
            dest->removeDependent (_this);
        dest = newPtr;
        if (dest)
            dest->addDependent (_this);
        */
}
