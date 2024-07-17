/*! 
  alloc - Convenience routines for safely allocating memory
  please see toplevel LICENSE for copyright information
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/alloc.h]

/**
  | avoid malloc()ing 0 bytes, see: 
  | https://www.securecoding.cert.org/confluence/display/seccode/MEM04-A.+Do+not+make+assumptions+about+the+result+of+allocating+0+bytes?focusedCommentId=5407003
  |
  */
#[inline] pub fn safe_malloc(size: usize)  {
    
    todo!();
        /*
            /* malloc(0) is undefined; FLAC src convention is to always allocate */
        if(!size)
            size++;
        return malloc(size);
        */
}

#[inline] pub fn safe_calloc(
        nmemb: usize,
        size:  usize)  {
    
    todo!();
        /*
            if(!nmemb || !size)
            return malloc(1); /* malloc(0) is undefined; FLAC src convention is to always allocate */
        return calloc(nmemb, size);
        */
}

/**
  | @@@ there's probably a better way to
  | prevent overflows when allocating
  | untrusted sums but this works for now
  |
  */
#[inline] pub fn safe_malloc_add_2op(
        size1: usize,
        size2: usize)  {
    
    todo!();
        /*
            size2 += size1;
        if(size2 < size1)
            return 0;
        return safe_malloc_(size2);
        */
}

#[inline] pub fn safe_malloc_add_3op(
        size1: usize,
        size2: usize,
        size3: usize)  {
    
    todo!();
        /*
            size2 += size1;
        if(size2 < size1)
            return 0;
        size3 += size2;
        if(size3 < size2)
            return 0;
        return safe_malloc_(size3);
        */
}

#[inline] pub fn safe_malloc_add_4op(
        size1: usize,
        size2: usize,
        size3: usize,
        size4: usize)  {
    
    todo!();
        /*
            size2 += size1;
        if(size2 < size1)
            return 0;
        size3 += size2;
        if(size3 < size2)
            return 0;
        size4 += size3;
        if(size4 < size3)
            return 0;
        return safe_malloc_(size4);
        */
}

pub fn safe_malloc_mul_2op(
        size1: usize,
        size2: usize)  {
    
    todo!();
        /*
        
        */
}

#[inline] pub fn safe_malloc_mul_3op(
        size1: usize,
        size2: usize,
        size3: usize)  {
    
    todo!();
        /*
            if(!size1 || !size2 || !size3)
            return malloc(1); /* malloc(0) is undefined; FLAC src convention is to always allocate */
        if(size1 > SIZE_MAX / size2)
            return 0;
        size1 *= size2;
        if(size1 > SIZE_MAX / size3)
            return 0;
        return malloc(size1*size3);
        */
}

/**
  | size1*size2 + size3
  |
  */
#[inline] pub fn safe_malloc_mul2add(
        size1: usize,
        size2: usize,
        size3: usize)  {
    
    todo!();
        /*
            if(!size1 || !size2)
            return safe_malloc_(size3);
        if(size1 > SIZE_MAX / size2)
            return 0;
        return safe_malloc_add_2op_(size1*size2, size3);
        */
}

/**
  | size1 * (size2 + size3)
  |
  */
#[inline] pub fn safe_malloc_muladd2(
        size1: usize,
        size2: usize,
        size3: usize)  {
    
    todo!();
        /*
            if(!size1 || (!size2 && !size3))
            return malloc(1); /* malloc(0) is undefined; FLAC src convention is to always allocate */
        size2 += size3;
        if(size2 < size3)
            return 0;
        if(size1 > SIZE_MAX / size2)
            return 0;
        return malloc(size1*size2);
        */
}

#[inline] pub fn safe_realloc_add_2op(
        ptr:   *mut c_void,
        size1: usize,
        size2: usize)  {
    
    todo!();
        /*
            size2 += size1;
        if(size2 < size1)
            return 0;
        return realloc(ptr, size2);
        */
}

#[inline] pub fn safe_realloc_add_3op(
        ptr:   *mut c_void,
        size1: usize,
        size2: usize,
        size3: usize)  {
    
    todo!();
        /*
            size2 += size1;
        if(size2 < size1)
            return 0;
        size3 += size2;
        if(size3 < size2)
            return 0;
        return realloc(ptr, size3);
        */
}

#[inline] pub fn safe_realloc_add_4op(
        ptr:   *mut c_void,
        size1: usize,
        size2: usize,
        size3: usize,
        size4: usize)  {
    
    todo!();
        /*
            size2 += size1;
        if(size2 < size1)
            return 0;
        size3 += size2;
        if(size3 < size2)
            return 0;
        size4 += size3;
        if(size4 < size3)
            return 0;
        return realloc(ptr, size4);
        */
}

#[inline] pub fn safe_realloc_mul_2op(
        ptr:   *mut c_void,
        size1: usize,
        size2: usize)  {
    
    todo!();
        /*
            if(!size1 || !size2)
            return realloc(ptr, 0); /* preserve POSIX realloc(ptr, 0) semantics */
        if(size1 > SIZE_MAX / size2)
            return 0;
        return realloc(ptr, size1*size2);
        */
}

/**
  | size1 * (size2 + size3)
  |
  */
#[inline] pub fn safe_realloc_muladd2(
        ptr:   *mut c_void,
        size1: usize,
        size2: usize,
        size3: usize)  {
    
    todo!();
        /*
            if(!size1 || (!size2 && !size3))
            return realloc(ptr, 0); /* preserve POSIX realloc(ptr, 0) semantics */
        size2 += size3;
        if(size2 < size3)
            return 0;
        return safe_realloc_mul_2op_(ptr, size1, size2);
        */
}
