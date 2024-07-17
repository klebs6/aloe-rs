crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct SHA256Tests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for SHA256Tests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("SHA-256", UnitTestCategories::cryptography
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl SHA256Tests {

    pub fn test(&mut self, 
        input:    *const u8,
        expected: *const u8)  {
        
        todo!();
        /*
            {
                SHA256 hash (input, strlen (input));
                expectEquals (hash.toHexString(), String (expected));
            }

            {
                CharPointer_UTF8 utf8 (input);
                SHA256 hash (utf8);
                expectEquals (hash.toHexString(), String (expected));
            }

            {
                MemoryInputStream m (input, strlen (input), false);
                SHA256 hash (m);
                expectEquals (hash.toHexString(), String (expected));
            }
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("SHA256");

            test ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
            test ("The quick brown fox jumps over the lazy dog",  "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592");
            test ("The quick brown fox jumps over the lazy dog.", "ef537f25c895bfa782526529a9b63d97aa631564d5d789c2b765448c8635fb6c");
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static SHA256Tests sha256UnitTests;
    */
}
