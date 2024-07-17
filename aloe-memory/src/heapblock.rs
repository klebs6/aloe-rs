crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_HeapBlock.h]

#[cfg(ALOE_EXCEPTIONS_ENABLED)]
pub mod heap_block_helper
{
    use super::*;

    lazy_static!{
        /*
        template <bool shouldThrow>
            struct ThrowOnFail          { static void checkPointer (void*) {} };

            template <>
            struct ThrowOnFail<true>    { static void checkPointer (void* data) { if (data == nullptr) throw std::bad_alloc(); } };
        */
    }
}

/**
  | Very simple container class to hold
  | a pointer to some data on the heap.
  | 
  | When you need to allocate some heap storage
  | for something, always try to use this
  | class instead of allocating the memory
  | directly using malloc/free.
  | 
  | A HeapBlock<char> object can be treated
  | in pretty much exactly the same way as
  | an char*, but as long as you allocate
  | it on the stack or as a class member, it's
  | almost impossible for it to leak memory.
  | 
  | It also makes your code much more concise
  | and readable than doing the same thing
  | using direct allocations,
  | 
  | E.g. instead of this:
  | 
  | 
  | -----------
  | @code
  | 
  | int* temp = (int*) malloc (1024 * sizeof (int));
  | memcpy (temp, xyz, 1024 * sizeof (int));
  | free (temp);
  | temp = (int*) calloc (2048 * sizeof (int));
  | temp[0] = 1234;
  | memcpy (foobar, temp, 2048 * sizeof (int));
  | free (temp);
  | 
  | ..you could just write this:
  | ----------
  | @code
  | 
  | HeapBlock<int> temp (1024);
  | memcpy (temp, xyz, 1024 * sizeof (int));
  | temp.calloc (2048);
  | temp[0] = 1234;
  | memcpy (foobar, temp, 2048 * sizeof (int));
  | 
  | The class is extremely lightweight,
  | containing only a pointer to the data,
  | and exposes malloc/realloc/calloc/free
  | methods that do the same jobs as their
  | less object-oriented counterparts.
  | Despite adding safety, you probably
  | won't sacrifice any performance by
  | using this in place of normal pointers.
  | 
  | The THROW_ON_FAILURE template parameter
  | can be set to true if you'd like the class
  | to throw a std::bad_alloc exception
  | when an allocation fails. If this is
  | false, then a failed allocation will
  | just leave the heapblock with a null
  | pointer (assuming that the system's
  | malloc() function doesn't throw).
  | 
  | @see Array, OwnedArray, MemoryBlock
  | 
  | @tags{Core}
  |
  */
/*
how do we say no_copy and prevent_heap_allocation only if the cfg condition is met?
#[cfg(not(any(ALOE_DLL,ALOE_DLL_BUILD)))]
#[no_copy]
#[prevent_heap_allocation]
*/
pub struct HeapBlock<ElementType, const THROW_ON_FAILURE: bool = false> {

    data: *mut ElementType, // default = nullptr
}

pub trait HasAllowConversion {
    type AllowConversion;
}

lazy_static!{
    /*
    impl<IndexType,ElementType,const ThrowOnFailure: bool> HasAllowConversion for HeapBlock<ElementType,ThrowOnFailure> {

        lazy_static!{
            /*
            template <class OtherElementType>
                using AllowConversion = typename std::enable_if<std::is_base_of<typename std::remove_pointer<ElementType>::type,
                                                                                typename std::remove_pointer<OtherElementType>::type>::value>::type;
            */
        }
    }
    */
}

impl<ElementType,const ThrowOnFailure: bool> Default for HeapBlock<ElementType,ThrowOnFailure> {
    
    /**
      | Creates a HeapBlock which is initially
      | just a null pointer.
      | 
      | After creation, you can resize the array
      | using the malloc(), calloc(), or realloc()
      | methods.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl<ElementType,const ThrowOnFailure: bool> 
Drop for HeapBlock<ElementType,ThrowOnFailure> {
    /**
      | Destructor. This will free the data,
      | if any has been allocated.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
            std::free (data);
         */
    }
}

/*
impl<ElementType,const ThrowOnFailure: bool> 
Into<ElementType> for HeapBlock<ElementType,ThrowOnFailure> {

    /**
      | Returns a raw pointer to the allocated
      | data.
      | 
      | This may be a null pointer if the data
      | hasn't yet been allocated, or if it has
      | been freed by calling the free() method.
      |
      */
    fn into(self) -> ElementType {
        todo!();
        /*
            return data;
        */
    }
}
*/

/*
impl<ElementType,const ThrowOnFailure: bool> 
Into<*mut c_void> for HeapBlock<ElementType,ThrowOnFailure> {
    
    /**
      | Returns a void pointer to the allocated
      | data.
      | 
      | This may be a null pointer if the data
      | hasn't yet been allocated, or if it has
      | been freed by calling the free() method.
      |
      */
    fn into(self) -> *mut c_void {
        todo!();
        /*
            return static_cast<void*> (data);
        */
    }
}
*/

impl<ElementType,const ThrowOnFailure: bool> 
Into<*const c_void> for HeapBlock<ElementType,ThrowOnFailure> {
    
    /**
      | Returns a void pointer to the allocated
      | data.
      | 
      | This may be a null pointer if the data
      | hasn't yet been allocated, or if it has
      | been freed by calling the free() method.
      |
      */
    fn into(self) -> *const c_void {
        todo!();
        /*
            return static_cast<const void*> (data);
        */
    }
}

impl<ElementType,const ThrowOnFailure: bool> 
Deref for HeapBlock<ElementType,ThrowOnFailure> {

    type Target = ElementType;

    /**
      | Lets you use indirect calls to the first
      | element in the array.
      | 
      | Obviously this will cause problems
      | if the array hasn't been initialised,
      | because it'll be referencing a null
      | pointer.
      |
      */
    fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return data;
        */
    }
}

impl<IndexType,ElementType,const ThrowOnFailure: bool> 
Index<IndexType> for HeapBlock<ElementType,ThrowOnFailure> {

    type Output = ElementType;
    
    /**
      | Returns a reference to one of the data
      | elements.
      | 
      | Obviously there's no bounds-checking
      | here, as this object is just a dumb pointer
      | and has no idea of the size it currently
      | has allocated.
      |
      */
    #[inline] fn index(&self, index: IndexType) -> &Self::Output {
        todo!();
        /*
            return data [index];
        */
    }
}

impl<IndexType,ElementType,const ThrowOnFailure: bool> 
Add<&IndexType> for HeapBlock<ElementType,ThrowOnFailure> {

    type Output = ElementType;
    
    /**
      | Returns a pointer to a data element at
      | an offset from the start of the array.
      | 
      | This is the same as doing pointer arithmetic
      | on the raw pointer itself.
      |
      */
    #[inline]fn add(self, other: &IndexType) -> Self::Output {
        todo!();
        /*
            return data + index;
        */
    }
}

impl<ElementType,const ThrowOnFailure: bool> 
PartialEq<*const ElementType> for HeapBlock<ElementType,ThrowOnFailure> {

    /**
      | Compares the pointer with another pointer. This
      | can be handy for checking whether this is a null
      | pointer.
      */
    #[inline] fn eq(&self, other: &*const ElementType) -> bool {
        todo!();
        /*
            return otherPointer == data;
        */
    }
}

pub trait HasType {

    type Type;
}

impl<ElementType,const ThrowOnFailure: bool> 
HasType for HeapBlock<ElementType,ThrowOnFailure> {

    /**
      | This typedef can be used to get the type
      | of the heapblock's elements.
      |
      */
    type Type = ElementType;
}

impl<ElementType,const ThrowOnFailure: bool> 
HeapBlock<ElementType,ThrowOnFailure> {

    /**
      | Creates a HeapBlock containing a number
      | of elements.
      | 
      | The contents of the block are undefined,
      | as it will have been created by a malloc
      | call.
      | 
      | If you want an array of zero values, you
      | can use the calloc() method or the other
      | constructor that takes an InitialisationState
      | parameter.
      |
      */
    pub fn new_with_num_elements<SizeType>(num_elements: SizeType) -> Self {
    
        todo!();
        /*


            : data (static_cast<ElementType*> (std::malloc (static_cast<size_t> (numElements) * sizeof (ElementType))))

            throwOnAllocationFailure();
        */
    }

    /**
      | Creates a HeapBlock containing a number
      | of elements.
      | 
      | The initialiseToZero parameter determines
      | whether the new memory should be cleared,
      | or left uninitialised.
      |
      */
    pub fn new_with_num_elements_and_maybe_initzero<SizeType>(
        num_elements:       SizeType,
        initialise_to_zero: bool) -> Self {
    
        todo!();
        /*


            : data (static_cast<ElementType*> (initialiseToZero
                                                   ? std::calloc (static_cast<size_t> (numElements), sizeof (ElementType))
                                                   : std::malloc (static_cast<size_t> (numElements) * sizeof (ElementType))))

            throwOnAllocationFailure();
        */
    }
    
    pub fn new_from_heapblock(other: HeapBlock<ElementType,ThrowOnFailure>) -> Self {
    
        todo!();
        /*
        : data(other.data),

            other.data = nullptr;
        */
    }
    
    pub fn assign_from(&mut self, other: HeapBlock<ElementType,ThrowOnFailure>) -> &mut HeapBlock<ElementType,ThrowOnFailure> {
        
        todo!();
        /*
            std::swap (data, other.data);
            return *this;
        */
    }

    /**
      | Converting move constructor.
      | 
      | Only enabled if this is a HeapBlock<Base*>
      | and the other object is a HeapBlock<Derived*>,
      | where std::is_base_of<Base, Derived>::value
      | == true.
      |
      */
    pub fn new_from_other<OtherElementType, const OtherThrowOnFailure: bool>(other: HeapBlock<OtherElementType,OtherThrowOnFailure>) -> Self {
    
        todo!();
        /*


            : data (reinterpret_cast<ElementType*> (other.data))

            other.data = nullptr;
        */
    }

    /*TODO:
    /**
      | Converting move assignment operator.
      | 
      | Only enabled if this is a HeapBlock<Base*>
      | and the other object is a HeapBlock<Derived*>,
      | where std::is_base_of<Base, Derived>::value
      | == true.
      |
      */
    pub fn assign_from<OtherElementType, T: AllowConversion<OtherElementType>, const OtherThrowOnFailure: bool>(
        &mut self, 
        other: HeapBlock<OtherElementType,OtherThrowOnFailure>
    ) -> &mut HeapBlock<ElementType,ThrowOnFailure> {
    
        todo!();
        /*
            free();
            data = reinterpret_cast<ElementType*> (other.data);
            other.data = nullptr;
            return *this;
        */
    }
    */

    /**
      | Returns a raw pointer to the allocated
      | data.
      | 
      | This may be a null pointer if the data
      | hasn't yet been allocated, or if it has
      | been freed by calling the free() method.
      |
      */
    #[inline] pub fn get(&self) -> *mut ElementType {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns a raw pointer to the allocated
      | data.
      | 
      | This may be a null pointer if the data
      | hasn't yet been allocated, or if it has
      | been freed by calling the free() method.
      |
      */
    #[inline] pub fn get_data(&self) -> *mut ElementType {
        
        todo!();
        /*
            return data;
        */
    }
    
    /**
      | Allocates a specified amount of memory.
      | 
      | This uses the normal malloc to allocate
      | an amount of memory for this object.
      | Any previously allocated memory will
      | be freed by this method.
      | 
      | The number of bytes allocated will be
      | (newNumElements * elementSize). Normally
      | you wouldn't need to specify the second
      | parameter, but it can be handy if you
      | need to allocate a size in bytes rather
      | than in terms of the number of elements.
      | 
      | The data that is allocated will be freed
      | when this object is deleted, or when
      | you call free() or any of the allocation
      | methods.
      |
      */
    pub fn malloc<SizeType>(
        &mut self, 
        new_num_elements: SizeType,
        element_size:     Option<usize>)  {
    
        let element_size: usize =
            element_size.unwrap_or(size_of::<ElementType>());

        todo!();
        /*
            std::free (data);
            data = static_cast<ElementType*> (std::malloc (static_cast<size_t> (newNumElements) * elementSize));
            throwOnAllocationFailure();
        */
    }

    /**
      | Allocates a specified amount of memory
      | and clears it.
      | 
      | This does the same job as the malloc()
      | method, but clears the memory that it
      | allocates.
      |
      */
    pub fn calloc<SizeType>(
        &mut self, 
        new_num_elements: SizeType,
        element_size:     Option<usize>)  {
    
        let element_size: usize =
            element_size.unwrap_or(size_of::<ElementType>());

        todo!();
        /*
            std::free (data);
            data = static_cast<ElementType*> (std::calloc (static_cast<size_t> (newNumElements), elementSize));
            throwOnAllocationFailure();
        */
    }

    /**
      | Allocates a specified amount of memory
      | and optionally clears it.
      | 
      | This does the same job as either malloc()
      | or calloc(), depending on the initialiseToZero
      | parameter.
      |
      */
    pub fn allocate<SizeType>(&mut self, 
        new_num_elements:   SizeType,
        initialise_to_zero: bool)  {
    
        todo!();
        /*
            std::free (data);
            data = static_cast<ElementType*> (initialiseToZero
                                                 ? std::calloc (static_cast<size_t> (newNumElements), sizeof (ElementType))
                                                 : std::malloc (static_cast<size_t> (newNumElements) * sizeof (ElementType)));
            throwOnAllocationFailure();
        */
    }

    /**
      | Re-allocates a specified amount of
      | memory.
      | 
      | The semantics of this method are the
      | same as malloc() and calloc(), but it
      | uses realloc() to keep as much of the
      | existing data as possible.
      |
      */
    pub fn realloc<SizeType>(
        &mut self, 
        new_num_elements: SizeType,
        element_size:     Option<usize>)  {

        let element_size: usize =
            element_size.unwrap_or(size_of::<ElementType>());

        todo!();
        /*
            data = static_cast<ElementType*> (data == nullptr ? std::malloc (static_cast<size_t> (newNumElements) * elementSize)
                                                              : std::realloc (data, static_cast<size_t> (newNumElements) * elementSize));
            throwOnAllocationFailure();
        */
    }

    /**
      | Frees any currently-allocated data.
      | 
      | This will free the data and reset this
      | object to be a null pointer.
      |
      */
    pub fn free(&mut self)  {
        
        todo!();
        /*
            std::free (data);
            data = nullptr;
        */
    }

    /**
      | Swaps this object's data with the data
      | of another HeapBlock.
      | 
      | The two objects simply exchange their
      | data pointers.
      |
      */
    pub fn swap_with<const otherBlockThrows: bool>(&mut self, other: &mut HeapBlock<ElementType,otherBlockThrows>)  {
    
        todo!();
        /*
            std::swap (data, other.data);
        */
    }

    /**
      | This fills the block with zeros, up to
      | the number of elements specified.
      | 
      | Since the block has no way of knowing
      | its own size, you must make sure that
      | the number of elements you specify doesn't
      | exceed the allocated size.
      |
      */
    pub fn clear<SizeType>(&mut self, num_elements: SizeType)  {
    
        todo!();
        /*
            zeromem (data, sizeof (ElementType) * static_cast<size_t> (numElements));
        */
    }
    
    pub fn throw_on_allocation_failure(&self)  {
        
        todo!();
        /*
            #if ALOE_EXCEPTIONS_DISABLED
            jassert (data != nullptr); // without exceptions, you'll need to find a better way to handle this failure case.
           #else
            HeapBlockHelper::ThrowOnFail<THROW_ON_FAILURE>::checkPointer (data);
           #endif
        */
    }
}
