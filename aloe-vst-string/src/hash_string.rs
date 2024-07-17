crate::ix!();

/**
  | Helper functions to create hash codes
  | from string data.
  |
  */
lazy_static!{
    /*
    extern uint32 hashString8 (const char8* s, uint32 m);
    extern uint32 hashString16 (const char16* s, uint32 m);
    */
}

#[inline] pub fn hash_string(
    s: *const u16,
    m: u32

) -> u32 {
    
    todo!();
        /*
            #ifdef UNICODE
        return hashString16 (s, m);
    #else
        return hashString8 (s, m);
    #endif
        */
}

pub fn hash_string8(
        s: *const u8,
        m: u32) -> u32 {
    
    todo!();
        /*
            uint32 h = 0;
        if (s)
        {
            for (h = 0; *s != '\0'; s++)
                h = (64 * h + *s) % m;
        }
        return h;
        */
}

pub fn hash_string16(
        s: *const u16,
        m: u32) -> u32 {
    
    todo!();
        /*
            uint32 h = 0;
        if (s)
        {
            for (h = 0; *s != 0; s++)
                h = (64 * h + *s) % m;
        }
        return h;
        */
}
