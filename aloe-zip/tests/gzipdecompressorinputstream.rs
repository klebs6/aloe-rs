crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct GZIPDecompressorInputStreamTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for GZIPDecompressorInputStreamTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("GZIPDecompressorInputStreamTests", UnitTestCategories::streams
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl GZIPDecompressorInputStreamTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            const MemoryBlock data ("abcdefghijklmnopqrstuvwxyz", 26);

            MemoryOutputStream mo;
            GZIPCompressorOutputStream gzipOutputStream (mo);
            gzipOutputStream.write (data.getData(), data.getSize());
            gzipOutputStream.flush();

            MemoryInputStream mi (mo.getData(), mo.getDataSize(), false);
            GZIPDecompressorInputStream stream (&mi, false, GZIPDecompressorInputStream::zlibFormat, (int64) data.getSize());

            beginTest ("Read");

            expectEquals (stream.getPosition(), (int64) 0);
            expectEquals (stream.getTotalLength(), (int64) data.getSize());
            expectEquals (stream.getNumBytesRemaining(), stream.getTotalLength());
            expect (! stream.isExhausted());

            size_t numBytesRead = 0;
            MemoryBlock readBuffer (data.getSize());

            while (numBytesRead < data.getSize())
            {
                numBytesRead += (size_t) stream.read (&readBuffer[numBytesRead], 3);

                expectEquals (stream.getPosition(), (int64) numBytesRead);
                expectEquals (stream.getNumBytesRemaining(), (int64) (data.getSize() - numBytesRead));
                expect (stream.isExhausted() == (numBytesRead == data.getSize()));
            }

            expectEquals (stream.getPosition(), (int64) data.getSize());
            expectEquals (stream.getNumBytesRemaining(), (int64) 0);
            expect (stream.isExhausted());

            expect (readBuffer == data);

            beginTest ("Skip");

            stream.setPosition (0);
            expectEquals (stream.getPosition(), (int64) 0);
            expectEquals (stream.getTotalLength(), (int64) data.getSize());
            expectEquals (stream.getNumBytesRemaining(), stream.getTotalLength());
            expect (! stream.isExhausted());

            numBytesRead = 0;
            const int numBytesToSkip = 5;

            while (numBytesRead < data.getSize())
            {
                stream.skipNextBytes (numBytesToSkip);
                numBytesRead += numBytesToSkip;
                numBytesRead = std::min (numBytesRead, data.getSize());

                expectEquals (stream.getPosition(), (int64) numBytesRead);
                expectEquals (stream.getNumBytesRemaining(), (int64) (data.getSize() - numBytesRead));
                expect (stream.isExhausted() == (numBytesRead == data.getSize()));
            }

            expectEquals (stream.getPosition(), (int64) data.getSize());
            expectEquals (stream.getNumBytesRemaining(), (int64) 0);
            expect (stream.isExhausted());
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static GZIPDecompressorInputStreamTests gzipDecompressorInputStreamTests;
    */
}
