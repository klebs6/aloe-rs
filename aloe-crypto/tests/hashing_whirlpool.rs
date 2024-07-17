crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct WhirlpoolTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for WhirlpoolTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Whirlpool", UnitTestCategories::cryptography
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl WhirlpoolTests {

    pub fn test(&mut self, 
        input:    *const u8,
        expected: *const u8)  {
        
        todo!();
        /*
            {
                Whirlpool hash (input, strlen (input));
                expectEquals (hash.toHexString(), String (expected));
            }

            {
                CharPointer_UTF8 utf8 (input);
                Whirlpool hash (utf8);
                expectEquals (hash.toHexString(), String (expected));
            }

            {
                MemoryInputStream m (input, strlen (input), false);
                Whirlpool hash (m);
                expectEquals (hash.toHexString(), String (expected));
            }
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Whirlpool");

            test ("", "19fa61d75522a4669b44e39c1d2e1726c530232130d407f89afee0964997f7a73e83be698b288febcf88e3e03c4f0757ea8964e59b63d93708b138cc42a66eb3");
            test ("The quick brown fox jumps over the lazy dog",  "b97de512e91e3828b40d2b0fdce9ceb3c4a71f9bea8d88e75c4fa854df36725fd2b52eb6544edcacd6f8beddfea403cb55ae31f03ad62a5ef54e42ee82c3fb35");
            test ("The quick brown fox jumps over the lazy dog.", "87a7ff096082e3ffeb86db10feb91c5af36c2c71bc426fe310ce662e0338223e217def0eab0b02b80eecf875657802bc5965e48f5c0a05467756f0d3f396faba");
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static WhirlpoolTests whirlpoolUnitTests;
    */
}
