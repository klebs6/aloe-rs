/*!
  | @file base/source/fobject.h
  | 
  | Basic Object implementing FUnknown.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fobject.h]

pub type FClassID = FIDString;

// Basic FObject - implements FUnknown + IDependent

/**
  | Special UID that is used to cast an FUnknown
  | pointer to a FObject
  |
  */
lazy_static!{
    /*
    static const FUID fobject_iid;
    const FUID FObject::iid;
    */
}

lazy_static!{
    /*
    IUpdateHandler* FObject_gUpdateHandler = nullptr;
    */
}

/**
  | Implements FUnknown and IDependent.
  | 
  | FObject is a polymorphic class that
  | implements IDependent (of SKI module)
  | and therefore derived from
  | 
  | FUnknown, which is the most abstract
  | base class of all.
  | 
  | All COM-like virtual methods of FUnknown
  | such as queryInterface(), addRef(),
  | release() are implemented here. On
  | top of that, dependency-related methods
  | are implemented too.
  | 
  | Pointer casting is done via the template
  | methods FCast, either FObject to FObject
  | or FUnknown to
  | 
  | FObject.
  | 
  | FObject supports a new singleton concept,
  | therefore these objects are deleted
  | automatically upon program termination.
  | 
  | - Runtime type information: An object
  | can be queried at runtime, of what class
  | it is. To do this correctly, every class
  | must override some methods. This is
  | simplified by using the OBJ_METHODS
  | macros
  | 
  | @see
  | 
  | - FUnknown
  | - IDependent
  | - IUpdateHandler
  |
  */
pub struct FObject {

    /**
      | COM-model local reference count
      |
      */
    ref_count: i32,
}

impl FUnknown for FObject {

    /**
      | please refer to FUnknown::queryInterface()
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn query_interface(&mut self, 
        iid: TUID,
        obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            QUERY_INTERFACE (_iid, obj, FUnknown::iid, FUnknown)             
        QUERY_INTERFACE (_iid, obj, IDependent::iid, IDependent)             
        QUERY_INTERFACE (_iid, obj, FObject::iid, FObject)             
        *obj = nullptr;
        return kNoInterface;
        */
    }

    /**
      | please refer to FUnknown::addref ()
      |
      */
    #[PLUGIN_API]
    fn add_ref(&mut self) -> u32 {
        
        todo!();
        /*
            return FUnknownPrivate::atomicAdd (refCount, 1);
        */
    }
    
    /**
      | please refer to FUnknown::release
      | ()
      |
      */
    #[PLUGIN_API]
    fn release(&mut self) -> u32 {
        
        todo!();
        /*
            if (FUnknownPrivate::atomicAdd (refCount, -1) == 0)
        {
            refCount = -1000;
            delete this;
            return 0;
        }                                   
        return refCount;
        */
    }
}

impl IDependent for FObject {

    /**
      | empty virtual method that should be
      | overridden by derived classes for data
      | updates upon changes
      |
      */
    fn update(
        &mut self, 
        changed_unknown: *mut dyn FUnknown,
        message:         i32

    ) {
        
        todo!();
        /*
        
        */
    }
}

impl Default for FObject {
    
    fn default() -> Self {
        todo!();
        /*
        : ref_count(1),

        
        */
    }
}

impl FobjectInterface for FObject {

    /**
      | a local alternative to getFClassID()
      |
      */
    fn isa(&self) -> FClassID {
        
        todo!();
        /*
            return FObject::getFClassID ();
        */
    }

    /**
      | evaluates if the passed ID is of the FObject
      | type
      |
      */
    fn isa_with(&self, s: FClassID) -> bool {
        
        todo!();
        /*
            return isTypeOf (s, false);
        */
    }

    /**
      | evaluates if the passed ID is of the FObject
      | type
      |
      */
    fn is_type_of(
        &self, 
        s:              FClassID,
        ask_base_class: Option<bool>

    ) -> bool {

        let ask_base_class: bool = ask_base_class.unwrap_or(true);

        todo!();
        /*
            return classIDsEqual (s, FObject::getFClassID ());
        */
    }

    /**
      | Inform all dependents, that the object
      | has changed.
      |
      */
    fn changed(&mut self, msg: Option<IDependentChangeMessage>)  {

        let msg = msg.unwrap_or(IDependentChangeMessage::Changed);

        todo!();
        /*
        
        */
    }

    /**
      | Similar to triggerUpdates, except only
      | delivered in idle (usefull in collecting
      | updates).
      */
    fn defer_update(&mut self, msg: Option<IDependentChangeMessage>)  {

        let msg = msg.unwrap_or(IDependentChangeMessage::Changed);

        todo!();
        /*
        
        */
    }

    /**
      | empty virtual method that should be
      | overridden by derived classes
      |
      */
    fn update_done(&mut self, msg: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    fn is_equal_instance(&mut self, d: *mut dyn FUnknown) -> bool {
        
        todo!();
        /*
            return this == d;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fobject.cpp]

impl FObject {

    /**
      | overloaded constructor...
      |
      */
    pub fn new(_0: &FObject) -> Self {
    
        todo!();
        /*
        : ref_count(1),

        
        */
    }
    
    /**
      | overloads operator "=" as the reference
      | assignment
      |
      */
    pub fn assign_from(&mut self, _0: &FObject) -> &mut FObject {
        
        todo!();
        /*
            return *this;
        */
    }

    /* ---------------- OBJECT_METHODS  ---------------- */

    /**
      | return Class ID as an ASCII string (statically)
      |
      */
    #[inline] pub fn get_fclassid() -> FClassID {
        
        todo!();
        /*
            return "FObject";
        */
    }

    /**
      | returns the current interface reference
      | count
      |
      */
    pub fn get_ref_count(&mut self) -> i32 {
        
        todo!();
        /*
            return refCount;
        */
    }

    /**
      | get FUnknown interface from object
      |
      */
    pub fn unknown_cast(&mut self) -> *mut dyn FUnknown {
        
        todo!();
        /*
            return this;
        */
    }

    /* ------------------- FUnknown  ------------------- */

    /* ------------------ IDependent  ------------------ */


    /* ------------------ IDependency  ------------------ */

    /**
      | set method for the local attribute
      |
      */
    pub fn set_update_handler(handler: *mut dyn IUpdateHandler)  {
        
        todo!();
        /*
            gUpdateHandler = handler;
        */
    }

    /**
      | get method for the local attribute
      |
      */
    pub fn get_update_handler() -> *mut dyn IUpdateHandler {
        
        todo!();
        /*
            return gUpdateHandler;
        */
    }

    /* ------------ static helper functions  ------------ */

    /* ------ conversion from FUnknown to FObject  ------ */

    /**
      | pointer conversion from FUnknown to
      | FObject
      |
      */
    #[inline] pub fn unknown_to_object(&mut self, unknown: *mut dyn FUnknown) -> *mut FObject {
        
        todo!();
        /*
            FObject* object = nullptr;
        if (unknown) 
        {
            unknown->queryInterface (FObject::iid, (void**)&object);
            if (object)
                object->release (); // queryInterface has added ref     
        }
        return object;
        */
    }
    
    /**
      | compares (evaluates) 2 class IDs
      |
      */
    #[inline] pub fn class_ids_equal(&mut self, 
        ci1: FClassID,
        ci2: FClassID) -> bool {
        
        todo!();
        /*
            return (ci1 && ci2) ? (strcmp (ci1, ci2) == 0) : false;
        */
    }
    
    /**
      | adds dependency to the object
      |
      */
    pub fn add_dependent(&mut self, dep: *mut dyn IDependent)  {
        
        todo!();
        /*
            if (gUpdateHandler)
            gUpdateHandler->addDependent (unknownCast (), dep);
        */
    }
    
    /**
      | removes dependency from the object
      |
      */
    pub fn remove_dependent(&mut self, dep: *mut dyn IDependent)  {
        
        todo!();
        /*
            if (gUpdateHandler)
            gUpdateHandler->removeDependent (unknownCast (), dep);
        */
    }
    
    pub fn changed(&mut self, msg: i32)  {
        
        todo!();
        /*
            if (gUpdateHandler)
            gUpdateHandler->triggerUpdates (unknownCast (), msg);
        else
            updateDone (msg);
        */
    }
    
    pub fn defer_update(&mut self, msg: i32)  {
        
        todo!();
        /*
            if (gUpdateHandler)
            gUpdateHandler->deferUpdates (unknownCast (), msg);
        else
            updateDone (msg);
        */
    }
}
