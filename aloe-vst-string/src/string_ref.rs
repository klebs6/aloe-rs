crate::ix!();

///-----------------------------
#[cfg(SMTG_OS_MACOS)]
pub fn to_cf_string_ref(
        source:   *const u8,
        encoding: u32)  {
    
    todo!();
        /*
            if (encoding == 0xFFFF)
            encoding = kCFStringEncodingASCII;
        if (source)
            return (void*)CFStringCreateWithCString (Steinberg::kCFAllocator, source, encoding);
        else
            return (void*)CFStringCreateWithCString (Steinberg::kCFAllocator, "", encoding);
        */
}

#[cfg(SMTG_OS_MACOS)]
pub fn from_cf_string_ref(
        dest:      *mut u8,
        dest_size: i32,
        cf_str:    *const c_void,
        encoding:  u32) -> bool {
    
    todo!();
        /*
            CFIndex usedBytes;
        CFRange range = {0, CFStringGetLength ((CFStringRef)cfStr)};
        bool result = CFStringGetBytes ((CFStringRef)cfStr, range, encoding, '?', false, (UInt8*)dest, destSize, &usedBytes);
        dest[usedBytes] = 0;
        return result;
        */
}


