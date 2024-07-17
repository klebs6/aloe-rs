crate::ix!();

pub const TAU_BUFFER_ALIGN_INTERVAL: usize = 0x10;
pub const TAU_BUFFER_ALIGN_MASK:     usize = TAU_BUFFER_ALIGN_INTERVAL - 1;

/**
  | Allocates an array of samples (type
  | T), to be optimally aligned for the processor
  |
  */
pub struct TAUBuffer<T> {

    /**
      | null when using an external buffer
      |
      */
    mem_object:        *mut c_void,

    /**
      | always valid once allocated
      |
      */
    aligned_buffer:    *mut T,

    buffer_size_bytes: u32,
}

impl<T> Default for TAUBuffer<T> {
    
    fn default() -> Self {
        todo!();
        /*
        : mem_object(NULL),
        : aligned_buffer(NULL),
        : buffer_size_bytes(0),

        
        */
    }
}

impl<T> Drop for TAUBuffer<T> {

    fn drop(&mut self) {
        todo!();
        /*
            Deallocate();
        */
    }
}

impl<T> TAUBuffer<T> {

    #[inline] pub fn into_type(self) -> T {
        todo!();
        /*
            return mAlignedBuffer;
        */
    }

    pub fn new(
        num_elems:    u32,
        num_channels: u32) -> Self {
    
        todo!();
        /*
        : mem_object(NULL),
        : aligned_buffer(NULL),
        : buffer_size_bytes(0),

            Allocate(numElems, numChannels);
        */
    }

    /**
      | can also re-allocate
      |
      */
    pub fn allocate(&mut self, num_elems: u32)  {
        
        todo!();
        /*
            UInt32 reqSize = numElems * sizeof(T);

            if (mMemObject != NULL && reqSize == mBufferSizeBytes)
                return; // already allocated

            mBufferSizeBytes = reqSize;
            mMemObject = CA_realloc(mMemObject, reqSize);
            UInt32 misalign = (uintptr_t)mMemObject & kAlignMask;
            if (misalign) {
                mMemObject = CA_realloc(mMemObject, reqSize + kAlignMask);
                mAlignedBuffer = (T *)((char *)mMemObject + kAlignInterval - misalign);
            } else
                mAlignedBuffer = (T *)mMemObject;
        */
    }
    
    pub fn deallocate(&mut self)  {
        
        todo!();
        /*
            if (mMemObject == NULL) return;         // so this method has no effect if we're using
                                                    // an external buffer

            free(mMemObject);
            mMemObject = NULL;
            mAlignedBuffer = NULL;
            mBufferSizeBytes = 0;
        */
    }

    /**
      | can also re-allocate
      |
      */
    pub fn allocate_clear(&mut self, num_elems: u32)  {
        
        todo!();
        /*
            Allocate(numElems);
            Clear();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            memset(mAlignedBuffer, 0, mBufferSizeBytes);
        */
    }

    /* ------------------- accessors  ------------------- */
}
