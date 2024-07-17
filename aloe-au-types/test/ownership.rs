crate::ix!();

/**
  | example program showing ownership
  | transfer
  |
  */
#[cfg(test)]
pub mod example {

    use super::*;

    pub fn source() -> CAAutoFree<u8> {
        
        todo!();
            /*
                // source allocates and returns ownership to the caller.
            const char* str = "this is a test";
            size_t size = strlen(str) + 1;
            CAAutoFree<char> captr(size, false);
            strlcpy(captr(), str, size);
            printf("source %08X %08X '%s'\n", &captr, captr(), captr());
            return captr;
            */
    }

    pub fn user(captr: &CAAutoFree<u8>)  {
        
        todo!();
            /*
                // passed by const reference. user can access the pointer but does not take ownership.
            printf("user: %08X %08X '%s'\n", &captr, captr(), captr());
            */
    }

    pub fn sink(captr: CAAutoFree<u8>)  {
        
        todo!();
            /*
                // passed by value. sink takes ownership and frees the pointer on return.
            printf("sink: %08X %08X '%s'\n", &captr, captr(), captr());
            */
    }

    pub fn main(
            argc: i32,
            argv: *mut &[u8]) -> i32 {
        
        todo!();
            /*
                CAAutoFree<char> captr(source());
            printf("main captr A %08X %08X\n", &captr, captr());
            user(captr);
            sink(captr);
            printf("main captr B %08X %08X\n", &captr, captr());
            return 0;
            */
    }
}
