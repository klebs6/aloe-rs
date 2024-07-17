crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_WeakReference.h]

/**
  | This class is used internally by the
  | WeakReference class - don't use it directly
  | in your code! @see WeakReference
  |
  */
#[no_copy]
pub struct WeakReferenceSharedPointer<ObjectType,ReferenceCountingType = ReferenceCountedObject> {

    base:  ReferenceCountingType,
    owner: *mut ObjectType,
}

impl<ObjectType,ReferenceCountingType> 
    WeakReferenceSharedPointer<ObjectType,ReferenceCountingType> {

    pub fn new(obj: *mut ObjectType) -> Self {
    
        todo!();
        /*
        : owner(obj),

        
        */
    }
    
    #[inline] pub fn get(&self) -> *mut ObjectType {
        
        todo!();
        /*
            return owner;
        */
    }
    
    pub fn clear_pointer(&mut self)  {
        
        todo!();
        /*
            owner = nullptr;
        */
    }
}

pub type WeakReferenceSharedRef<ObjectType,ReferenceCountingType = ReferenceCountedObject> 
    = ReferenceCountedObjectPtr<WeakReferenceSharedPointer<ObjectType,ReferenceCountingType>>;

/**
  | This class is embedded inside an object
  | to which you want to attach WeakReference
  | pointers.
  | 
  | See the WeakReference class notes for
  | an example of how to use this class. @see
  | WeakReference
  |
  */
#[derive(Default)]
#[no_copy]
pub struct WeakReferenceMaster<ObjectType,ReferenceCountingType = ReferenceCountedObject> {
    shared_pointer: WeakReferenceSharedRef<ObjectType,ReferenceCountingType>,
}

impl<ObjectType,ReferenceCountingType> 
    Drop for WeakReferenceMaster<ObjectType,ReferenceCountingType> {

    fn drop(&mut self) {
        todo!();
        /* 
                // You must remember to call clear() in your source object's destructor! See the notes
                // for the WeakReference class for an example of how to do this.
                jassert (sharedPointer == nullptr || sharedPointer->get() == nullptr);
             */
    }
}

impl<ObjectType,ReferenceCountingType> 
    WeakReferenceMaster<ObjectType,ReferenceCountingType> {

    /**
      | The first call to this method will create
      | an internal object that is shared by
      | all weak references to the object.
      |
      */
    pub fn get_shared_pointer(&mut self, object: *mut ObjectType) -> WeakReferenceSharedRef<ObjectType> {
        
        todo!();
        /*
            if (sharedPointer == nullptr)
                {
                    sharedPointer = *new WeakReferenceSharedPointer (object);
                }
                else
                {
                    // You're trying to create a weak reference to an object that has already been deleted!!
                    jassert (sharedPointer->get() != nullptr);
                }

                return sharedPointer;
        */
    }

    /**
      | The object that owns this master pointer
      | should call this before it gets destroyed,
      | to zero all the references to this object
      | that may be out there. See the WeakReference
      | class notes for an example of how to do
      | this.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            if (sharedPointer != nullptr)
                    sharedPointer->clearPointer();
        */
    }

    /**
      | Returns the number of WeakReferences
      | that are out there pointing to this object.
      |
      */
    pub fn get_num_active_weak_references(&self) -> i32 {
        
        todo!();
        /*
            return sharedPointer == nullptr ? 0 : (sharedPointer->getReferenceCount() - 1);
        */
    }
}

/**
  | This class acts as a pointer which will
  | automatically become null if the object
  | to which it points is deleted.
  | 
  | To accomplish this, the source object
  | needs to cooperate by performing a couple
  | of simple tasks. It must embed a WeakReferenceMaster
  | object, which stores a shared pointer
  | object, and must clear this master pointer
  | in its destructor.
  | 
  | -----------
  | @note
  | 
  | WeakReference is not designed to be
  | thread-safe, so if you're accessing
  | it from different threads, you'll need
  | to do your own locking around all uses
  | of the pointer and the object it refers
  | to.
  | 
  | E.g.
  | 
  | -----------
  | @code
  | 
  | class MyObject
  | {
  | 
  |     MyObject() {}
  | 
  |     ~MyObject()
  |     {
  |         // This will zero all the references - you need to call this in your destructor.
  |         masterReference.clear();
  |     }
  | 
  |     // You need to embed a variable of this type, with the name "masterReference" inside your object. If the
  |     // variable is not public, you should make your class a friend of WeakReference<MyObject> so that the
  |     // WeakReference class can access it.
  |     WeakReference<MyObject>::WeakReferenceMaster masterReference;
  |     friend class WeakReference<MyObject>;
  | };
  | 
  | OR: just use the handy ALOE_DECLARE_WEAK_REFERENCEABLE macro to do all this for you.
  | 
  | // Here's an example of using a pointer..
  | 
  | auto* n = new MyObject();
  | WeakReference<MyObject> myObjectRef = n;
  | 
  | auto pointer1 = myObjectRef.get();  // returns a valid pointer to 'n'
  | delete n;
  | auto pointer2 = myObjectRef.get();  // now returns nullptr
  | 
  | @see WeakReferenceMaster
  | 
  | @tags{Core}
  |
  */
pub struct WeakReference<ObjectType,ReferenceCountingType = ReferenceCountedObject> {
    holder: WeakReferenceSharedRef<ObjectType,ReferenceCountingType>,
}

/*
impl<ObjectType,ReferenceCountingType> 
Into<ObjectType> for WeakReference<ObjectType,ReferenceCountingType> 
{
    /**
      | Returns the object that this pointer
      | refers to, or null if the object no longer
      | exists.
      |
      */
    #[inline] fn into(self) -> ObjectType {
        todo!();
        /*
            return get();
        */
    }
}
*/

impl<ObjectType,ReferenceCountingType> 
Deref for WeakReference<ObjectType,ReferenceCountingType> 
{
    type Target = ObjectType;
    
    /**
      | Returns the object that this pointer
      | refers to, or null if the object no longer
      | exists.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return get();
        */
    }
}

impl<ObjectType,ReferenceCountingType> 
PartialEq<*mut ObjectType> for WeakReference<ObjectType,ReferenceCountingType> {
    
    #[inline] fn eq(&self, other: &*mut ObjectType) -> bool {
        todo!();
        /*
            return get() == object;
        */
    }
}

impl<ObjectType,ReferenceCountingType> Default for
WeakReference<ObjectType,ReferenceCountingType> 
{
    /**
      | Creates a null WeakReference.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        
        */
    }
}

impl<ObjectType,ReferenceCountingType> 
WeakReference<ObjectType,ReferenceCountingType> {

    /**
      | Creates a WeakReference that points
      | at the given object.
      |
      */
    pub fn new_from_ptr(object: *mut ObjectType) -> Self {
    
        todo!();
        /*
        : holder(getRef (object)),

        
        */
    }

    /**
      | Creates a copy of another WeakReference.
      |
      */
    pub fn new_from_weak_ref_ref(other: &WeakReference<ObjectType>) -> Self {
    
        todo!();
        /*
        : holder(other.holder),

        
        */
    }

    /**
      | Move constructor
      |
      */
    pub fn new_from_weak_ref(other: WeakReference<ObjectType>) -> Self {
    
        todo!();
        /*
        : holder(std::move (other.holder)),

        
        */
    }

    /**
      | Copies another pointer to this one.
      |
      */
    pub fn assign_from_weak_ref_ref(&mut self, other: &WeakReference<ObjectType>) -> &mut WeakReference<ObjectType> {
        
        todo!();
        /*
            holder = other.holder; return *this;
        */
    }

    /**
      | Copies another pointer to this one.
      |
      */
    pub fn assign_from_ptr(&mut self, new_object: *mut ObjectType) -> &mut WeakReference<ObjectType> {
        
        todo!();
        /*
            holder = getRef (newObject); return *this;
        */
    }

    /**
      | Move assignment operator
      |
      */
    pub fn assign_from_weak_ref(&mut self, other: WeakReference<ObjectType>) -> &mut WeakReference<ObjectType> {
        
        todo!();
        /*
            holder = std::move (other.holder); return *this;
        */
    }

    /**
      | Returns the object that this pointer
      | refers to, or null if the object no longer
      | exists.
      |
      */
    pub fn get(&self) -> *mut ObjectType {
        
        todo!();
        /*
            return holder != nullptr ? holder->get() : nullptr;
        */
    }

    /**
      | This returns true if this reference
      | has been pointing at an object, but that
      | object has since been deleted.
      | 
      | If this reference was only ever pointing
      | at a null pointer, this will return false.
      | Using operator=() to make this refer
      | to a different object will reset this
      | flag to match the status of the reference
      | from which you're copying.
      |
      */
    pub fn was_object_deleted(&self) -> bool {
        
        todo!();
        /*
            return holder != nullptr && holder->get() == nullptr;
        */
    }
    
    pub fn get_ref(o: *mut ObjectType) -> WeakReferenceSharedRef<ObjectType> {
        
        todo!();
        /*
            if (o != nullptr)
                return o->masterReference.getSharedPointer (o);

            return {};
        */
    }
}

/**
  | Macro to easily allow a class to be made
  | weak-referenceable. This can be inserted
  | in a class definition to add the requisite
  | weak-ref boilerplate to that class.
  | e.g.
  | 
  | -----------
  | @code
  | 
  | class MyObject
  | {
  | 
  |     MyObject();
  |     ~MyObject();
  | 
  |     ALOE_DECLARE_WEAK_REFERENCEABLE (MyObject)
  | };
  | 
  | @see WeakReference, WeakReferenceMaster
  |
  */
#[macro_export]
macro_rules! aloe_declare_weak_referenceable {
    ($Class:ty) => {
        /*
        
            struct WeakRefMaster  : public WeakReference<Class>::WeakReferenceMaster { ~WeakRefMaster() { this->clear(); } }; 
            WeakRefMaster masterReference; 
            friend class WeakReference<Class>; 
        */
    }
}
