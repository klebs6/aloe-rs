crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_Atomic.h]

/**
  | A simple wrapper around std::atomic.
  | 
  | @tags{Core}
  |
  */
pub struct Atomic<Type> {

    /**
      | The std::atomic object that this class
      | operates on.
      |
      */
    value: std::sync::atomic::AtomicPtr<Type>,
}

impl<T> Default for Atomic<T> {
    
    /**
      | Creates a new value, initialised to
      | zero.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        : value(Type()),
        */
    }
}

impl<T> Drop for Atomic<T> {

    fn drop(&mut self) {

        todo!();

        /* 
           #if __cpp_lib_atomic_is_always_lock_free
            static_assert (std::atomic<Type>::is_always_lock_free,
                           "This class can only be used for lock-free types");
           #endif
         */
    }
}

impl<Type,DiffType> AddAssign<DiffType> for Atomic<Type> {

    /**
      | Atomically adds a number to this value,
      | returning the new value.
      |
      */
    #[inline]fn add_assign(&mut self, other: DiffType) {
        todo!();
        /*
            return value += amountToAdd;
        */
    }
}

impl<Type,DiffType> SubAssign<DiffType> for Atomic<Type> {

    /**
      | Atomically subtracts a number from
      | this value, returning the new value.
      |
      */
    #[inline]fn sub_assign(&mut self, other: DiffType) {
        todo!();
        /*
            return value -= amountToSubtract;
        */
    }
}

impl<Type> Atomic<Type> {

    /**
      | Creates a new value, with a given initial
      | value.
      |
      */
    pub fn new_from_initial_value(initial_value: Type) -> Self {
    
        todo!();
        /*
        : value(initialValue),

        
        */
    }

    /**
      | Copies another value (atomically).
      |
      */
    pub fn new(other: &Atomic<Type>) -> Self {
    
        todo!();
        /*
        : value(other.get()),

        
        */
    }

    /**
      | Atomically reads and returns the current
      | value.
      |
      */
    pub fn get(&self) -> Type {
        
        todo!();
        /*
            return value.load();
        */
    }

    /**
      | Atomically sets the current value.
      |
      */
    pub fn set(&mut self, new_value: Type)  {
        
        todo!();
        /*
            value = newValue;
        */
    }

    /**
      | Atomically sets the current value,
      | returning the value that was replaced.
      |
      */
    pub fn exchange(&mut self, new_value: Type) -> Type {
        
        todo!();
        /*
            return value.exchange (newValue);
        */
    }

    /**
      | Atomically compares this value with
      | a target value, and if it is equal, sets
      | this to be equal to a new value.
      | 
      | This operation is the atomic equivalent
      | of doing this:
      | 
      | 
      | -----------
      | @code
      | 
      | bool compareAndSetBool (Type newValue, Type valueToCompare)
      | {
      |     if (get() == valueToCompare)
      |     {
      |         set (newValue);
      |         return true;
      |     }
      | 
      |     return false;
      | }
      | 
      | Internally, this method calls std::atomic::compare_exchange_strong
      | with memory_order_seq_cst (the strictest
      | std::memory_order).
      | 
      | -----------
      | @return
      | 
      | true if the comparison was true and the
      | value was replaced; false if the comparison
      | failed and the value was left unchanged.
      | @see compareAndSetValue
      |
      */
    pub fn compare_and_set_bool(&mut self, 
        new_value:        Type,
        value_to_compare: Type) -> bool {
        
        todo!();
        /*
            return value.compare_exchange_strong (valueToCompare, newValue);
        */
    }

    /**
      | Copies another value into this one (atomically).
      |
      */
    pub fn assign_from_other(&mut self, other: &Atomic<Type>) -> &mut Atomic<Type> {
        
        todo!();
        /*
            value = other.value.load();
            return *this;
        */
    }

    /**
      | Copies another value into this one (atomically).
      |
      */
    pub fn assign_from(&mut self, new_value: Type) -> &mut Atomic<Type> {
        
        todo!();
        /*
            value = newValue;
            return *this;
        */
    }
    
    /**
      | Atomically increments this value,
      | returning the new value.
      |
      */
    pub fn prefix_increment(&mut self) -> Type {
        
        todo!();
        /*
            return ++value;
        */
    }

    /**
      | Atomically decrements this value,
      | returning the new value.
      |
      */
    pub fn prefix_decrement(&mut self) -> Type {
        
        todo!();
        /*
            return --value;
        */
    }

    /**
      | Implements a memory read/write barrier.
      | 
      | Internally this calls std::atomic_thread_fence
      | with memory_order_seq_cst (the strictest
      | std::memory_order).
      |
      */
    pub fn memory_barrier(&mut self)  {
        
        todo!();
        /*
            atomic_thread_fence (std::memory_order_seq_cst);
        */
    }
}

