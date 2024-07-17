crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct SubregionInputStreamTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for SubregionInputStreamTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("SubregionInputStream", UnitTestCategories::streams
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl SubregionInputStreamTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            const MemoryBlock data ("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz", 52);
            MemoryInputStream mi (data, true);

            const int offset = getRandom().nextInt ((int) data.getSize());
            const size_t subregionSize = data.getSize() - (size_t) offset;

            SubregionStream stream (&mi, offset, (int) subregionSize, false);

            beginTest ("Read");

            expectEquals (stream.getPosition(), (int64) 0);
            expectEquals (stream.getTotalLength(), (int64) subregionSize);
            expectEquals (stream.getNumBytesRemaining(), stream.getTotalLength());
            expect (! stream.isExhausted());

            size_t numBytesRead = 0;
            MemoryBlock readBuffer (subregionSize);

            while (numBytesRead < subregionSize)
            {
                numBytesRead += (size_t) stream.read (&readBuffer[numBytesRead], 3);

                expectEquals (stream.getPosition(), (int64) numBytesRead);
                expectEquals (stream.getNumBytesRemaining(), (int64) (subregionSize - numBytesRead));
                expect (stream.isExhausted() == (numBytesRead == subregionSize));
            }

            expectEquals (stream.getPosition(), (int64) subregionSize);
            expectEquals (stream.getNumBytesRemaining(), (int64) 0);
            expect (stream.isExhausted());

            const MemoryBlock memoryBlockToCheck (data.begin() + (size_t) offset, data.getSize() - (size_t) offset);
            expect (readBuffer == memoryBlockToCheck);

            beginTest ("Skip");

            stream.setPosition (0);
            expectEquals (stream.getPosition(), (int64) 0);
            expectEquals (stream.getTotalLength(), (int64) subregionSize);
            expectEquals (stream.getNumBytesRemaining(), stream.getTotalLength());
            expect (! stream.isExhausted());

            numBytesRead = 0;
            const int64 numBytesToSkip = 5;

            while (numBytesRead < subregionSize)
            {
                stream.skipNextBytes (numBytesToSkip);
                numBytesRead += numBytesToSkip;
                numBytesRead = std::min (numBytesRead, subregionSize);

                expectEquals (stream.getPosition(), (int64) numBytesRead);
                expectEquals (stream.getNumBytesRemaining(), (int64) (subregionSize - numBytesRead));
                expect (stream.isExhausted() == (numBytesRead == subregionSize));
            }

            expectEquals (stream.getPosition(), (int64) subregionSize);
            expectEquals (stream.getNumBytesRemaining(), (int64) 0);
            expect (stream.isExhausted());
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static SubregionInputStreamTests subregionInputStreamTests;
    */
}
