crate::ix!();

pub trait IsHeapAllocated {

    fn is_heap_allocated(&self) -> bool;
}

pub trait EnsureSize {

    /** 
      | Preallocates some memory for the buffer to
      | use.
      |
      | This helps to avoid needing to reallocate
      | space when the buffer has messages added
      | to it.
      */
    fn ensure_size(&mut self, minimum_num_bytes: usize);
}

pub trait SwapWith {

    /** 
      | Exchanges the contents of this buffer with
      | another one.
      |
      | This is a quick operation, because no
      | memory allocating or copying is done, it
      | just swaps the internal state of the two
      | buffers.
      */
    fn swap_with(&mut self, other: &mut Self) where Self: Sized;
}

pub trait GetNumBytesUsed {

    /**
      | Returns the number of bytes currently
      | being mapped
      |
      */
    fn get_num_bytes_used(&self) -> usize;
}
