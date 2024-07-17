crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/threads/aloe_DynamicLibrary.h]

/**
  | Handles the opening and closing of DLLs.
  | 
  | This class can be used to open a DLL and
  | get some function pointers from it.
  | Since the DLL is freed when this object
  | is deleted, it's handy for managing
  | library lifetimes using RAII.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DynamicLibrary {
    handle: *mut c_void, // default = nullptr
}

impl Default for DynamicLibrary {
    
    /**
      | Creates an unopened DynamicLibrary
      | object.
      | 
      | Call open() to actually open one.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Drop for DynamicLibrary {
    /**
      | Destructor.
      | 
      | If a library is currently open, it will
      | be closed when this object is destroyed.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*      close();  */
    }
}

impl DynamicLibrary {

    pub fn new(name: &String) -> Self {
    
        todo!();
        /*


            open (name);
        */
    }

    /**
      | Move constructor
      |
      */
    pub fn new_from_other(other: DynamicLibrary) -> Self {
    
        todo!();
        /*


            std::swap (handle, other.handle);
        */
    }

    /**
      | Opens a DLL.
      | 
      | The name and the method by which it gets
      | found is of course platform-specific,
      | and may or may not include a path, depending
      | on the OS.
      | 
      | If a library is already open when this
      | method is called, it will first close
      | the library before attempting to load
      | the new one.
      | 
      | -----------
      | @return
      | 
      | true if the library was successfully
      | found and opened.
      |
      */
    pub fn open(&mut self, name: &String) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Releases the currently-open DLL, or
      | has no effect if none was open.
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to find a named function in the
      | currently-open DLL, and returns a pointer
      | to it.
      | 
      | If no library is open, or if the function
      | isn't found, this will return a null
      | pointer.
      |
      */
    pub fn get_function(&mut self, function_name: &String)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the platform-specific native
      | library handle.
      | 
      | You'll need to cast this to whatever
      | is appropriate for the OS that's in use.
      |
      */
    pub fn get_native_handle(&self)  {
        
        todo!();
        /*
            return handle;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(&mut self, name: &String) -> bool {
        
        todo!();
        /*
            close();
        handle = dlopen (name.isEmpty() ? nullptr : name.toUTF8().getAddress(), RTLD_LOCAL | RTLD_NOW);
        return handle != nullptr;
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (handle != nullptr)
        {
            dlclose (handle);
            handle = nullptr;
        }
        */
    }
    
    #[cfg(feature = "aloe_posix")]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_function(&mut self, function_name: &String)  {
        
        todo!();
        /*
            return handle != nullptr ? dlsym (handle, functionName.toUTF8()) : nullptr;
        */
    }
}
