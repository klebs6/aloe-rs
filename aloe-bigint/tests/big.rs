
#[cfg(ALOE_UNIT_TESTS)]
pub struct BigIntegerTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for BigIntegerTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("BigInteger", UnitTestCategories::maths
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl BigIntegerTests {

    pub fn get_big_random(r: &mut Random) -> BigInteger {
        
        todo!();
        /*
            BigInteger b;

            while (b < 2)
                r.fillBitsRandomly (b, 0, r.nextInt (150) + 1);

            return b;
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            {
                beginTest ("BigInteger");

                Random r = getRandom();

                expect (BigInteger().isZero());
                expect (BigInteger(1).isOne());

                for (int j = 10000; --j >= 0;)
                {
                    BigInteger b1 (getBigRandom(r)),
                               b2 (getBigRandom(r));

                    BigInteger b3 = b1 + b2;
                    expect (b3 > b1 && b3 > b2);
                    expect (b3 - b1 == b2);
                    expect (b3 - b2 == b1);

                    BigInteger b4 = b1 * b2;
                    expect (b4 > b1 && b4 > b2);
                    expect (b4 / b1 == b2);
                    expect (b4 / b2 == b1);
                    expect (((b4 << 1) >> 1) == b4);
                    expect (((b4 << 10) >> 10) == b4);
                    expect (((b4 << 100) >> 100) == b4);

                    // TODO: should add tests for other ops (although they also get pretty well tested in the RSA unit test)

                    BigInteger b5;
                    b5.loadFromMemoryBlock (b3.toMemoryBlock());
                    expect (b3 == b5);
                }
            }

            {
                beginTest ("Bit setting");

                Random r = getRandom();
                static uint8 test[2048];

                for (int j = 100000; --j >= 0;)
                {
                    uint32 offset = static_cast<uint32> (r.nextInt (200) + 10);
                    uint32 num = static_cast<uint32> (r.nextInt (32) + 1);
                    uint32 value = static_cast<uint32> (r.nextInt());

                    if (num < 32)
                        value &= ((1u << num) - 1);

                    auto old1 = readLittleEndianBitsInBuffer (test, offset - 6, 6);
                    auto old2 = readLittleEndianBitsInBuffer (test, offset + num, 6);
                    writeLittleEndianBitsInBuffer (test, offset, num, value);
                    auto result = readLittleEndianBitsInBuffer (test, offset, num);

                    expect (result == value);
                    expect (old1 == readLittleEndianBitsInBuffer (test, offset - 6, 6));
                    expect (old2 == readLittleEndianBitsInBuffer (test, offset + num, 6));
                }
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static BigIntegerTests bigIntegerTests;
    */
}
