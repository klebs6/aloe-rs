crate::ix!();

/**
  | Byte-order Conversion Macros
  |
  */
macro_rules! swap_32 {
    ($l:ident) => {
        /*
                { 
            unsigned char* p = (unsigned char*)& (l); 
            unsigned char t; 
            t = p[0]; p[0] = p[3]; p[3] = t; t = p[1]; p[1] = p[2]; p[2] = t; }
        */
    }
}

macro_rules! swap_16 {
    ($w:ident) => {
        /*
                { 
            unsigned char* p = (unsigned char*)& (w); 
            unsigned char t; 
            t = p[0]; p[0] = p[1]; p[1] = t; }
        */
    }
}

macro_rules! swap_64 {
    ($i:ident) => {
        /*
                { 
            unsigned char* p = (unsigned char*)& (i); 
            unsigned char t; 
            t = p[0]; p[0] = p[7]; p[7] = t; t = p[1]; p[1] = p[6]; p[6] = t; 
            t = p[2]; p[2] = p[5]; p[5] = t; t = p[3]; p[3] = p[4]; p[4] = t;}
        */
    }
}

//---------------------------------
#[inline] pub fn steinberg_swap_i16(_0: &mut i16)  {
    
    todo!();
        /*
            SWAP_16 (i16)
        */
}

#[inline] pub fn steinberg_swap_u16(_0: &mut u16)  {
    
    todo!();
        /*
            SWAP_16 (i16)
        */
}

#[inline] pub fn steinberg_swap_i32(_0: &mut i32)  {
    
    todo!();
        /*
            SWAP_32 (i32)
        */
}

#[inline] pub fn steinberg_swap_u32(_0: &mut u32)  {
    
    todo!();
        /*
            SWAP_32 (i32)
        */
}

#[inline] pub fn steinberg_swap_i64(_0: &mut i64)  {
    
    todo!();
        /*
            SWAP_64 (i64)
        */
}

#[inline] pub fn steinberg_swap_u64(_0: &mut u64)  {
    
    todo!();
        /*
            SWAP_64 (i64)
        */
}
