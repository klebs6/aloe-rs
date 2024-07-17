crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct GZIPTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for GZIPTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("GZIP", UnitTestCategories::compression
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl GZIPTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("GZIP");
            Random rng = getRandom();

            for (int i = 100; --i >= 0;)
            {
                MemoryOutputStream original, compressed, uncompressed;

                {
                    GZIPCompressorOutputStream zipper (compressed, rng.nextInt (10));

                    for (int j = rng.nextInt (100); --j >= 0;)
                    {
                        MemoryBlock data ((unsigned int) (rng.nextInt (2000) + 1));

                        for (int k = (int) data.getSize(); --k >= 0;)
                            data[k] = (char) rng.nextInt (255);

                        original << data;
                        zipper   << data;
                    }
                }

                {
                    MemoryInputStream compressedInput (compressed.getData(), compressed.getDataSize(), false);
                    GZIPDecompressorInputStream unzipper (compressedInput);

                    uncompressed << unzipper;
                }

                expectEquals ((int) uncompressed.getDataSize(),
                              (int) original.getDataSize());

                if (original.getDataSize() == uncompressed.getDataSize())
                    expect (memcmp (uncompressed.getData(),
                                    original.getData(),
                                    original.getDataSize()) == 0);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static GZIPTests gzipTests;
    */
}
