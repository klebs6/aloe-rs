crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct MemoryStreamTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for MemoryStreamTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("MemoryInputStream & MemoryOutputStream", UnitTestCategories::streams
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl MemoryStreamTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Basics");
            Random r = getRandom();

            int randomInt = r.nextInt();
            int64 randomInt64 = r.nextInt64();
            double randomDouble = r.nextDouble();
            String randomString (createRandomWideCharString (r));

            MemoryOutputStream mo;
            mo.writeInt (randomInt);
            mo.writeIntBigEndian (randomInt);
            mo.writeCompressedInt (randomInt);
            mo.writeString (randomString);
            mo.writeInt64 (randomInt64);
            mo.writeInt64BigEndian (randomInt64);
            mo.writeDouble (randomDouble);
            mo.writeDoubleBigEndian (randomDouble);

            MemoryInputStream mi (mo.getData(), mo.getDataSize(), false);
            expect (mi.readInt() == randomInt);
            expect (mi.readIntBigEndian() == randomInt);
            expect (mi.readCompressedInt() == randomInt);
            expectEquals (mi.readString(), randomString);
            expect (mi.readInt64() == randomInt64);
            expect (mi.readInt64BigEndian() == randomInt64);
            expect (mi.readDouble() == randomDouble);
            expect (mi.readDoubleBigEndian() == randomDouble);

            const MemoryBlock data ("abcdefghijklmnopqrstuvwxyz", 26);
            MemoryInputStream stream (data, true);

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

    pub fn create_random_wide_char_string(r: &mut Random) -> String {
        
        todo!();
        /*
            aloe_wchar buffer [50] = { 0 };

            for (int i = 0; i < numElementsInArray (buffer) - 1; ++i)
            {
                if (r.nextBool())
                {
                    do
                    {
                        buffer[i] = (aloe_wchar) (1 + r.nextInt (0x10ffff - 1));
                    }
                    while (! CharPointer_UTF16::canRepresent (buffer[i]));
                }
                else
                    buffer[i] = (aloe_wchar) (1 + r.nextInt (0xff));
            }

            return CharPointer_UTF32 (buffer);
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static MemoryStreamTests memoryInputStreamUnitTests;
    */
}
