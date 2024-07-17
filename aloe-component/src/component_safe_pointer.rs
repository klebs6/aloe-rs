crate::ix!();

/**
  | Holds a pointer to some type of Component<'a>,
  | which automatically becomes null if
  | the component is deleted.
  | 
  | If you're using a component which may
  | be deleted by another event that's outside
  | of your control, use a ComponentSafePointer instead
  | of a normal pointer to refer to it, and
  | you can test whether it's null before
  | using it to see if something has deleted
  | it.
  | 
  | The ComponentType template parameter
  | must be Component<'a>, or some subclass
  | of Component.
  | 
  | You may also want to use a WeakReference<Component<'a>>
  | object for the same purpose.
  |
  */
pub struct ComponentSafePointer<'a,ComponentType> {
    weak_ref: WeakReference<Component<'a>>,

    phantom:  PhantomData<ComponentType>,
}

impl<'a,ComponentType> Default for ComponentSafePointer<'a,ComponentType> {
    
    /**
      | Creates a null ComponentSafePointer.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<'a,ComponentType> ComponentSafePointer<'a,ComponentType> {

    /**
      | Returns the component that this pointer
      | refers to, or null if the component no
      | longer exists.
      |
      */
    #[inline] fn into_component_type(self) -> ComponentType {
        todo!();
        /*
            return getComponent();
        */
    }
}

impl<'a,ComponentType> Deref for ComponentSafePointer<'a,ComponentType> {

    type Target = ComponentType;

    /**
      | Returns the component that this pointer
      | refers to, or null if the component no
      | longer exists.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return getComponent();
        */
    }
}

impl<'a,ComponentType> PartialEq<ComponentType> for ComponentSafePointer<'a,ComponentType> {
    
    #[inline] fn eq(&self, other: &ComponentType) -> bool {
        todo!();
        /*
            return weakRef == component;
        */
    }
}

impl<'a,ComponentType> ComponentSafePointer<'a,ComponentType> {

    /**
      | Creates a ComponentSafePointer that points at
      | the given component.
      |
      */
    pub fn new_from_component_type(component: *mut ComponentType) -> Self {
    
        todo!();
        /*
        : weak_ref(component),

        
        */
    }

    /**
      | Creates a copy of another ComponentSafePointer.
      |
      */
    pub fn new(other: &ComponentSafePointer<'a,ComponentType>) -> Self {
    
        todo!();
        /*
        : weak_ref(other.weakRef),

        
        */
    }

    /**
      | Copies another pointer to this one.
      |
      */
    pub fn assign_from_other(
        &mut self, 
        other: &ComponentSafePointer<'a,ComponentType>

    ) -> &mut ComponentSafePointer<'a,ComponentType> {
        
        todo!();
        /*
            weakRef = other.weakRef; return *this;
        */
    }

    /**
      | Copies another pointer to this one.
      |
      */
    pub fn assign_from(&mut self, new_component: *mut ComponentType) 
    -> &mut ComponentSafePointer<'a,ComponentType> 
    {
        todo!();
        /*
            weakRef = newComponent; return *this;
        */
    }

    /**
      | Returns the component that this pointer
      | refers to, or null if the component no
      | longer exists.
      |
      */
    pub fn get_component(&self) -> *mut ComponentType {
        
        todo!();
        /*
            return dynamic_cast<ComponentType*> (weakRef.get());
        */
    }


    /**
      | If the component is valid, this deletes
      | it and sets this pointer to null.
      |
      */
    pub fn delete_and_zero(&mut self)  {
        
        todo!();
        /*
            delete getComponent();
        */
    }
}
