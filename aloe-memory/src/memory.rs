crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_Memory.h]

/**
  | Fills a block of memory with zeros.
  |
  */
#[inline] pub fn zeromem(
        memory:    *mut c_void,
        num_bytes: usize)  {
    
    todo!();
    /*
        memset (memory, 0, numBytes);
    */
}

/**
  | Overwrites a structure or object with
  | zeros.
  |
  */
#[inline] pub fn zerostruct<Type>(structure: &mut Type)  {

    todo!();
    /*
        memset ((void*) &structure, 0, sizeof (structure));
    */
}

/**
  | Delete an object pointer, and sets the
  | pointer to null.
  | 
  | Remember that it's not good c++ practice
  | to use delete directly - always try to
  | use a std::unique_ptr or other automatic
  | lifetime-management system rather
  | than resorting to deleting raw pointers!
  |
  */
#[inline] pub fn delete_and_zero<Type>(pointer: &mut Type)  {

    todo!();
    /*
        delete pointer; pointer = nullptr;
    */
}

/**
  | A handy function to round up a pointer
  | to the nearest multiple of a given number
  | of bytes. alignmentBytes must be a power
  | of two.
  |
  */
#[inline] pub fn snap_pointer_to_alignment<Type, IntegerType>(
        base_pointer:    *mut Type,
        alignment_bytes: IntegerType) -> *mut Type {

    todo!();
    /*
        return (Type*) ((((size_t) basePointer) + (alignmentBytes - 1)) & ~(alignmentBytes - 1));
    */
}

/**
  | A handy function which returns the difference
  | between any two pointers, in bytes.
  | 
  | The address of the second pointer is
  | subtracted from the first, and the difference
  | in bytes is returned.
  |
  */
#[inline] pub fn get_address_difference<Type1, Type2>(
        pointer1: *mut Type1,
        pointer2: *mut Type2) -> i32 {

    todo!();
    /*
        return (int) (((const char*) pointer1) - (const char*) pointer2);
    */
}

/**
  | If a pointer is non-null, this returns
  | a new copy of the object that it points
  | to, or safely returns nullptr if the
  | pointer is null.
  |
  */
#[inline] pub fn create_copy_if_not_null<Type>(object_to_copy: *const Type) -> *mut Type {

    todo!();
    /*
        return objectToCopy != nullptr ? new Type (*objectToCopy) : nullptr;
    */
}

/**
  | A handy function to read un-aligned
  | memory without a performance penalty
  | or bus-error.
  |
  */
#[inline] pub fn read_unaligned<Type>(src_ptr: *const c_void) -> Type {

    todo!();
    /*
        Type value;
        memcpy (&value, srcPtr, sizeof (Type));
        return value;
    */
}

/**
  | A handy function to write un-aligned
  | memory without a performance penalty
  | or bus-error.
  |
  */
#[inline] pub fn write_unaligned<Type>(
        dst_ptr: *mut c_void,
        value:   Type)  {

    todo!();
    /*
        memcpy (dstPtr, &value, sizeof (Type));
    */
}

/**
  | Casts a pointer to another type via `void*`,
  | which suppresses the cast-align warning
  | which sometimes arises when casting
  | pointers to types with different alignment.
  | 
  | You should only use this when you know
  | for a fact that the input pointer points
  | to a region that has suitable alignment
  | for `Type`, e.g. regions returned from
  | malloc/calloc that should be suitable
  | for any non-over-aligned type.
  |
  */
lazy_static!{
    /*
    template <typename Type, typename std::enable_if<std::is_pointer<Type>::value, int>::type = 0>
    inline Type unalignedPointerCast (void* ptr) 
    {
        return reinterpret_cast<Type> (ptr);
    }
    */
}

/**
  | Casts a pointer to another type via `void*`,
  | which suppresses the cast-align warning
  | which sometimes arises when casting
  | pointers to types with different alignment.
  | 
  | You should only use this when you know
  | for a fact that the input pointer points
  | to a region that has suitable alignment
  | for `Type`, e.g. regions returned from
  | malloc/calloc that should be suitable
  | for any non-over-aligned type.
  |
  */
lazy_static!{
    /*
    template <typename Type, typename std::enable_if<std::is_pointer<Type>::value, int>::type = 0>
    inline Type unalignedPointerCast (const void* ptr) 
    {
        return reinterpret_cast<Type> (ptr);
    }
    */
}

/**
  | A handy function which adds a number
  | of bytes to any type of pointer and returns
  | the result.
  | 
  | This can be useful to avoid casting pointers
  | to a char* and back when you want to move
  | them by a specific number of bytes,
  |
  */
#[inline] pub fn add_bytes_to_pointer_mut<Type, IntegerType>(
        base_pointer: *mut Type,
        bytes:        IntegerType) -> *mut Type {

    todo!();
    /*
        return unalignedPointerCast<Type*> (reinterpret_cast<char*> (basePointer) + bytes);
    */
}

/**
  | A handy function which adds a number
  | of bytes to any type of pointer and returns
  | the result.
  | 
  | This can be useful to avoid casting pointers
  | to a char* and back when you want to move
  | them by a specific number of bytes,
  |
  */
#[inline] pub fn add_bytes_to_pointer<Type, IntegerType>(
        base_pointer: *const Type,
        bytes:        IntegerType) -> *const Type {

    todo!();
    /*
        return unalignedPointerCast<const Type*> (reinterpret_cast<const char*> (basePointer) + bytes);
    */
}



/**
  | A handy C++ wrapper that creates and
  | deletes an NSAutoreleasePool object
  | using RAII.
  | 
  | You should use the ALOE_AUTORELEASEPOOL
  | macro to create a local auto-release
  | pool on the stack.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[cfg(any(target_os="macos",target_os="ios"))]
pub struct ScopedAutoReleasePool {
    pool: *mut c_void,
}

/**
  | A macro that can be used to easily declare
  | a local ScopedAutoReleasePool object
  | for RAII-based obj-C autoreleasing.
  | 
  | Because this may use the \@autoreleasepool
  | syntax, you must follow the macro with
  | a set of braces to mark the scope of the
  | pool.
  |
  */
#[cfg(any(target_os="macos",target_os="ios"))]
lazy_static!{
    /*
    #if (ALOE_COMPILER_SUPPORTS_ARC && defined (__OBJC__)) || DOXYGEN
     #define ALOE_AUTORELEASEPOOL  @autoreleasepool
    #else
     #define ALOE_AUTORELEASEPOOL  const ScopedAutoReleasePool ALOE_JOIN_MACRO (autoReleasePool_, __LINE__);
    #endif

    #else
     #define ALOE_AUTORELEASEPOOL
    #endif
    */
}

/**
  | In a Windows DLL build, we'll expose
  | some malloc/free functions that live
  | inside the DLL, and use these for allocating
  | all the objects - that way all aloe objects
  | in the DLL and in the host will live in
  | the same heap, avoiding problems when
  | an object is created in one module and
  | passed across to another where it is
  | deleted. By piggy-backing on the ALOE_LEAK_DETECTOR
  | macro, these allocators can be injected
  | into most aloe classes.
  |
  */
#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(all(all(ALOE_MSVC,any(ALOE_DLL,ALOE_DLL_BUILD)),not(ALOE_DISABLE_DLL_ALLOCATORS)))]
lazy_static!{
    /*
    extern  void* aloeDLL_malloc (size_t);
     extern  void  aloeDLL_free (void*);
    */
}

#[cfg(any(target_os="macos",target_os="ios"))]
macro_rules! aloe_leak_detector {
    ($OwnerClass:ident) => {
        /*
        
            static void* operator new (size_t sz)           { return aloeDLL_malloc (sz); } 
            static void* operator new (size_t, void* p)     { return p; } 
            static void operator delete (void* p)           { aloeDLL_free (p); } 
            static void operator delete (void*, void*)      {}
        */
    }
}

/**
  | Converts an owning raw pointer into
  | a unique_ptr, deriving the type of the
  | unique_ptr automatically.
  | 
  | This should only be used with pointers
  | to single objects. Do NOT pass a pointer
  | to an array to this function, as the destructor
  | of the unique_ptr will incorrectly
  | call `delete` instead of `delete[]`
  | on the pointer.
  |
  */
#[cfg(any(target_os="macos",target_os="ios"))]
pub fn raw_to_unique_ptr<T>(ptr: *mut T) -> Box<T> {

    todo!();
    /*
        return std::unique_ptr<T> (ptr);
    */
}
