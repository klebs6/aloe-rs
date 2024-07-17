crate::ix!();

pub struct BlowFishTests {
    base: UnitTest,
}

impl Default for BlowFishTests {
    
    fn default() -> Self {
        todo!();
        /*
            : UnitTest ("BlowFish", UnitTestCategories::cryptography
        */
    }
}

impl BlowFishTests {

    pub fn fill_memory_block_with_random_data(
        block:  &mut MemoryBlock,
        random: &mut Random)  {
        
        todo!();
        /*
            const size_t n = block.getSize() / sizeof (int32);
            auto* dst = reinterpret_cast<uint8*> (block.getData());

            for (size_t i = 0; i < n; ++i)
                dst[i] = static_cast<uint8> (random.nextInt(255));
        */
    }
    
    pub fn expect_equal_data(&mut self, 
        dataa:           *const c_void,
        datab:           *const c_void,
        size:            usize,
        failure_message: &String)  {
        
        todo!();
        /*
            auto* a = reinterpret_cast<const uint8*> (dataA);
            auto* b = reinterpret_cast<const uint8*> (dataB);

            for (size_t i = 0; i < size; ++i)
                expectEquals ((int) a[i], (int) b[i], failureMessage);
        */
    }
    
    pub fn expect_equal_memory_blocks(&mut self, 
        a:               &MemoryBlock,
        b:               &MemoryBlock,
        failure_message: &String)  {
        
        todo!();
        /*
            expectEquals ((int) a.getSize(), (int) b.getSize(), failureMessage);
            expectEqualData (a.getData(), b.getData(), a.getSize(), failureMessage);
        */
    }
    
    pub fn encrypt_decrypt_test(&mut self, 
        blow_fish:   &BlowFish,
        data:        *mut c_void,
        size:        usize,
        buffer_size: usize)  {
        
        todo!();
        /*
            MemoryBlock copy (data, size);

            int encryptedSize = blowFish.encrypt (data, size, bufferSize);
            expectGreaterThan (encryptedSize, (int) size);
            expectLessOrEqual (encryptedSize, (int) bufferSize);

            int decryptedSize = blowFish.decrypt (data, static_cast<size_t> (encryptedSize));
            expectEquals ((int) size, decryptedSize);

            expectEqualData (data, copy.getData(), size, "Length/Content changed during encryption");
        */
    }
    
    pub fn encrypt_decrypt_test(&mut self, 
        blow_fish: &BlowFish,
        data:      &mut MemoryBlock)  {
        
        todo!();
        /*
            MemoryBlock copy (data);

            blowFish.encrypt (data);
            blowFish.decrypt (data);

            expectEqualMemoryBlocks (data, copy, "Length/Content changed during encryption");
        */
    }
    
    pub fn encrypt_decrypt_test(&mut self, 
        blow_fish: &BlowFish,
        data:      &String)  {
        
        todo!();
        /*
            MemoryBlock block (data.toRawUTF8(), static_cast<size_t> (data.length()));
            encryptDecryptTest (blowFish, block);
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("BlowFish");
            auto random = getRandom();

            for (int i = 0; i < 100; ++i)
            {
                const int keySize = (random.nextInt(17) + 1) * static_cast<int> (sizeof (uint32));
                MemoryBlock key (static_cast<size_t> (keySize));
                fillMemoryBlockWithRandomData (key, random);

                BlowFish bf (key.getData(), keySize);

                encryptDecryptTest (bf, "");
                encryptDecryptTest (bf, "a");
                encryptDecryptTest (bf, "Hello World!");

                const int minSize = 8 + sizeof (c_void*);
                const int dataSize = random.nextInt (2048 - minSize) + minSize;
                MemoryBlock data (static_cast<size_t> (dataSize));
                fillMemoryBlockWithRandomData (data, random);

                encryptDecryptTest (bf, data);
                encryptDecryptTest (bf, data.getData(), data.getSize() - 8, data.getSize());
                encryptDecryptTest (bf, data.getData(), 0, 8);

                {
                    // Test unaligned data encryption/decryption. This will be flagged up by a check for
                    // undefined behaviour!
                    auto nudge = static_cast<uintptr_t> (random.nextInt (sizeof(c_void*) - 1));
                    auto unalignedData = (c_void*) (reinterpret_cast<uintptr_t> (data.getData()) + nudge);
                    size_t newSize = data.getSize() - nudge;

                    encryptDecryptTest (bf, unalignedData, newSize - 8, newSize);
                }
            }
        */
    }
}

lazy_static!{
    /*
    static BlowFishTests blowFishUnitTests;
    */
}
