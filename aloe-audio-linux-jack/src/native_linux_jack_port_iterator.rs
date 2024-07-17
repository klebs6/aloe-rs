crate::ix!();

pub struct JackPortIteratorFree {

}

unsafe impl std::alloc::Allocator for JackPortIteratorFree {

    fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> { 
        todo!() 
    }

    unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) { 
        todo!() 
    }
}

impl JackPortIteratorFree {

    pub fn invoke(&self, ptr: *const *const u8)  {
        
        todo!();
        /*
            jack_free (ptr);
        */
    }
}

pub struct JackPortIterator {
    ports: Box<*const u8,JackPortIteratorFree>,
    index: i32, // default = -1
    name:  String,
}

impl JackPortIterator {

    pub fn new(
        client:    *mut JackClient,
        for_input: bool) -> Self {
    
        todo!();
        /*


            if (client != nullptr)
                ports.reset (jack_get_ports (client, nullptr, nullptr,
                                                   forInput ? JackPortIsInput : JackPortIsOutput));
        */
    }
    
    pub fn next(&mut self) -> bool {
        
        todo!();
        /*
            if (ports == nullptr || ports.get()[index + 1] == nullptr)
                return false;

            name = CharPointer_UTF8 (ports.get()[++index]);
            return true;
        */
    }
    
    pub fn get_client_name(&self) -> String {
        
        todo!();
        /*
            return name.upToFirstOccurrenceOf (":", false, false);
        */
    }
    
    pub fn get_channel_name(&self) -> String {
        
        todo!();
        /*
            return name.fromFirstOccurrenceOf (":", false, false);
        */
    }
}
