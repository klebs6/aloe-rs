crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/containers/aloe_AudioBlock_test.cpp]

pub struct AudioBlockUnitTests<SampleType> 
    where SampleType: SampleTypeInterface
{
    base:        UnitTest,
    data:        Vec<*mut SampleType,numChannels>,
    other_data:  Vec<*mut SampleType,numChannels>,
    block:       AudioBlock<SampleType>,
    other_block: AudioBlock<SampleType>,
}

impl<SampleType: SampleTypeInterface> HasNumericType for AudioBlockUnitTests<SampleType> 
    where SampleType: SampleTypeInterface
{
    type Type = ElementType<SampleType>::Type;
}

impl HasScalarVoid for AudioBlockUnitTests<SampleType> {

    lazy_static!{
        /*
        template <typename T>
            using ScalarVoid = typename std::enable_if_t <  std::is_scalar <T>::value, void>;

        */
    }
}

impl HasSimdVoid for AudioBlockUnitTests<SampleType> {
    lazy_static!{
        /*
        template <typename T>
                    using SIMDVoid   = typename std::enable_if_t <! std::is_scalar <T>::value, void>;
        */
    }
}

impl Default for AudioBlockUnitTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("AudioBlock", UnitTestCategories::dsp)

            for (auto v : { &data, &otherData })
                for (auto& channel : *v)
                    channel = allocateAlignedMemory (numSamples);

            block      = { data.data(),      data.size(),      (size_t) numSamples };
            otherBlock = { otherData.data(), otherData.size(), (size_t) numSamples };

            resetBlocks()
        */
    }
}

impl Drop for AudioBlockUnitTests {
    fn drop(&mut self) {
        todo!();
        /* 
            for (auto v : { &data, &otherData })
                for (auto channel : *v)
                    deallocateAlignedMemory (channel);
         */
    }
}

impl AudioBlockUnitTests<SampleType> {

    pub const num_channels: i32 = 2;
    pub const num_samples:  i32 = 6;

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Equality");
            {
                expect (block == block);
                expect (block != otherBlock);
            }

            beginTest ("Constructors");
            {
                expect (block == AudioBlock<SampleType> (data.data(), data.size(), numSamples));
                expect (block == AudioBlock<SampleType> (data.data(), data.size(), (size_t) 0, numSamples));
                expect (block == AudioBlock<SampleType> (block));

                expect (block == AudioBlock<const SampleType> (data.data(), data.size(), numSamples));
                expect (block == AudioBlock<const SampleType> (data.data(), data.size(), (size_t) 0, numSamples));
                expect (block == AudioBlock<const SampleType> (block));
            }

            beginTest ("Swap");
            {
                resetBlocks();

                expect (block != otherBlock);
                expect (block.getSample (0, 0) == SampleType (1.0));
                expect (block.getSample (0, 4) == SampleType (5.0));
                expect (otherBlock.getSample (0, 0) == SampleType (-1.0));
                expect (otherBlock.getSample (0, 3) == SampleType (-4.0));

                block.swap (otherBlock);

                expect (block != otherBlock);
                expect (otherBlock.getSample (0, 0) == SampleType (1.0));
                expect (otherBlock.getSample (0, 4) == SampleType (5.0));
                expect (block.getSample (0, 0) == SampleType (-1.0));
                expect (block.getSample (0, 3) == SampleType (-4.0));

                block.swap (otherBlock);

                expect (block.getSample (0, 0) == SampleType (1.0));
                expect (block.getSample (0, 4) == SampleType (5.0));
                expect (otherBlock.getSample (0, 0) == SampleType (-1.0));
                expect (otherBlock.getSample (0, 3) == SampleType (-4.0));
            }

            beginTest ("Getters and setters");
            {
                resetBlocks();

                expectEquals ((int) block.getNumChannels(), (int) data.size());
                expectEquals ((int) block.getNumSamples(), numSamples);

                expect (block.getChannelPointer (0)[2] == SampleType (3.0));
                block.getChannelPointer (0)[2] = SampleType (999.0);
                expect (block.getChannelPointer (0)[2] == SampleType (999.0));

                expect (block.getSample (0, 4) == SampleType (5.0));
                expect (block.getSample (1, 4) == SampleType (11.0));

                expect (block.getSingleChannelBlock (1).getSample (0, 3) == block.getSample (1, 3));

                expect (block.getSubsetChannelBlock (0, 2).getSample (1, 3) == block.getSample (1, 3));
                expect (block.getSubsetChannelBlock (1, 1).getSample (0, 3) == block.getSample (1, 3));

                block.setSample (1, 1, SampleType (777.0));
                expect (block.getSample (1, 1) == SampleType (777.0));

                block.addSample (1, 1, SampleType (1.0));
                expect (block.getSample (1, 1) == SampleType (778.0));
            }

            beginTest ("Basic copying");
            {
                block.clear();
                expect (block.getSample (0, 2) == SampleType (0.0));
                expect (block.getSample (1, 4) == SampleType (0.0));

                block.fill ((NumericType) 456.0);
                expect (block.getSample (0, 2) == SampleType (456.0));
                expect (block.getSample (1, 4) == SampleType (456.0));

                block.copyFrom (otherBlock);
                expect (block != otherBlock);
                expect (block.getSample (0, 2) == otherBlock.getSample (0, 2));
                expect (block.getSample (1, 4) == otherBlock.getSample (1, 4));

                resetBlocks();

                SampleType testSample1 = block.getSample (0, 2);
                SampleType testSample2 = block.getSample (1, 3);
                expect (testSample1 != block.getSample (0, 4));
                expect (testSample2 != block.getSample (1, 5));
                block.move (0, 2);
                expect (block.getSample (0, 4) == testSample1);
                expect (block.getSample (1, 5) == testSample2);
            }

            beginTest ("Addition");
            {
                resetBlocks();

                block.add ((NumericType) 15.0);
                expect (block.getSample (0, 4) == SampleType (20.0));
                expect (block.getSample (1, 4) == SampleType (26.0));

                block.add (otherBlock);
                expect (block.getSample (0, 4) == SampleType (15.0));
                expect (block.getSample (1, 4) == SampleType (15.0));

                block.replaceWithSumOf (otherBlock, (NumericType) 9.0);
                expect (block.getSample (0, 4) == SampleType (4.0));
                expect (block.getSample (1, 4) == SampleType (-2.0));

                resetBlocks();

                block.replaceWithSumOf (block, otherBlock);
                expect (block.getSample (0, 4) == SampleType (0.0));
                expect (block.getSample (1, 4) == SampleType (0.0));
            }

            beginTest ("Subtraction");
            {
                resetBlocks();

                block.subtract ((NumericType) 15.0);
                expect (block.getSample (0, 4) == SampleType (-10.0));
                expect (block.getSample (1, 4) == SampleType (-4.0));

                block.subtract (otherBlock);
                expect (block.getSample (0, 4) == SampleType (-5.0));
                expect (block.getSample (1, 4) == SampleType (7.0));

                block.replaceWithDifferenceOf (otherBlock, (NumericType) 9.0);
                expect (block.getSample (0, 4) == SampleType (-14.0));
                expect (block.getSample (1, 4) == SampleType (-20.0));

                resetBlocks();

                block.replaceWithDifferenceOf (block, otherBlock);
                expect (block.getSample (0, 4) == SampleType (10.0));
                expect (block.getSample (1, 4) == SampleType (22.0));
            }

            beginTest ("Multiplication");
            {
                resetBlocks();

                block.multiplyBy ((NumericType) 10.0);
                expect (block.getSample (0, 4) == SampleType (50.0));
                expect (block.getSample (1, 4) == SampleType (110.0));

                block.multiplyBy (otherBlock);
                expect (block.getSample (0, 4) == SampleType (-250.0));
                expect (block.getSample (1, 4) == SampleType (-1210.0));

                block.replaceWithProductOf (otherBlock, (NumericType) 3.0);
                expect (block.getSample (0, 4) == SampleType (-15.0));
                expect (block.getSample (1, 4) == SampleType (-33.0));

                resetBlocks();

                block.replaceWithProductOf (block, otherBlock);
                expect (block.getSample (0, 4) == SampleType (-25.0));
                expect (block.getSample (1, 4) == SampleType (-121.0));
            }

            beginTest ("Multiply add");
            {
                resetBlocks();

                block.addProductOf (otherBlock, (NumericType) -1.0);
                expect (block.getSample (0, 4) == SampleType (10.0));
                expect (block.getSample (1, 4) == SampleType (22.0));

                block.addProductOf (otherBlock, otherBlock);
                expect (block.getSample (0, 4) == SampleType (35.0));
                expect (block.getSample (1, 4) == SampleType (143.0));
            }

            beginTest ("Negative abs min max");
            {
                resetBlocks();
                otherBlock.negate();

                block.add (otherBlock);
                expect (block.getSample (0, 4) == SampleType (10.0));
                expect (block.getSample (1, 4) == SampleType (22.0));

                block.replaceWithNegativeOf (otherBlock);
                expect (block.getSample (0, 4) == SampleType (-5.0));
                expect (block.getSample (1, 4) == SampleType (-11.0));

                block.clear();
                otherBlock.negate();
                block.replaceWithAbsoluteValueOf (otherBlock);
                expect (block.getSample (0, 4) == SampleType (5.0));
                expect (block.getSample (1, 4) == SampleType (11.0));

                resetBlocks();
                block.replaceWithMinOf (block, otherBlock);
                expect (block.getSample (0, 4) == SampleType (-5.0));
                expect (block.getSample (1, 4) == SampleType (-11.0));

                resetBlocks();
                block.replaceWithMaxOf (block, otherBlock);
                expect (block.getSample (0, 4) == SampleType (5.0));
                expect (block.getSample (1, 4) == SampleType (11.0));

                resetBlocks();
                auto range = block.findMinAndMax();
                expect (SampleType (range.getStart()) == SampleType (1.0));
                expect (SampleType (range.getEnd()) == SampleType (12.0));
            }

            beginTest ("Operators");
            {
                resetBlocks();
                block += (NumericType) 10.0;
                expect (block.getSample (0, 4) == SampleType (15.0));
                expect (block.getSample (1, 4) == SampleType (21.0));
                block += otherBlock;
                expect (block.getSample (0, 4) == SampleType (10.0));
                expect (block.getSample (1, 4) == SampleType (10.0));

                resetBlocks();
                block -= (NumericType) 10.0;
                expect (block.getSample (0, 4) == SampleType (-5.0));
                expect (block.getSample (1, 4) == SampleType (1.0));
                block -= otherBlock;
                expect (block.getSample (0, 4) == SampleType (0.0));
                expect (block.getSample (1, 4) == SampleType (12.0));

                resetBlocks();
                block *= (NumericType) 10.0;
                expect (block.getSample (0, 4) == SampleType (50.0));
                expect (block.getSample (1, 4) == SampleType (110.0));
                block *= otherBlock;
                expect (block.getSample (0, 4) == SampleType (-250.0));
                expect (block.getSample (1, 4) == SampleType (-1210.0));
            }

            beginTest ("Process");
            {
                resetBlocks();
                AudioBlock<SampleType>::process (block, otherBlock, [] (SampleType x) { return x + (NumericType) 1.0; });
                expect (otherBlock.getSample (0, 4) == SampleType (6.0));
                expect (otherBlock.getSample (1, 4) == SampleType (12.0));
            }

            beginTest ("Copying");
            {
                resetBlocks();
                copyingTests();
            }

            beginTest ("Smoothing");
            {
                resetBlocks();
                smoothedValueTests();
            }
        */
    }
    
    pub fn copying_tests<T = SampleType>(&mut self) -> ScalarVoid<T> {
    
        todo!();
        /*
            auto unchangedElement1 = block.getSample (0, 4);
            auto unchangedElement2 = block.getSample (1, 1);

            AudioBuffer<SampleType> otherBuffer (otherData.data(), (int) otherData.size(), numSamples);

            block.copyFrom (otherBuffer, 1, 2, 2);

            expectEquals (block.getSample (0, 4), unchangedElement1);
            expectEquals (block.getSample (1, 1), unchangedElement2);
            expectEquals (block.getSample (0, 2), otherBuffer.getSample (0, 1));
            expectEquals (block.getSample (1, 3), otherBuffer.getSample (1, 2));

            resetBlocks();

            unchangedElement1 = otherBuffer.getSample (0, 4);
            unchangedElement2 = otherBuffer.getSample (1, 3);

            block.copyTo (otherBuffer, 2, 1, 2);

            expectEquals (otherBuffer.getSample (0, 4), unchangedElement1);
            expectEquals (otherBuffer.getSample (1, 3), unchangedElement2);
            expectEquals (otherBuffer.getSample (0, 1), block.getSample (0, 2));
            expectEquals (otherBuffer.getSample (1, 2), block.getSample (1, 3));
        */
    }

    #[cfg(ALOE_USE_SIMD)]
    pub fn copying_tests<T = SampleType>(&mut self) -> SIMDVoid<T> {
    
        todo!();
        /*
            auto numSIMDElements = SIMDRegister<NumericType>::SIMDNumElements;
            AudioBuffer<NumericType> numericData ((int) block.getNumChannels(),
                                                  (int) (block.getNumSamples() * numSIMDElements));

            for (int c = 0; c < numericData.getNumChannels(); ++c)
                std::fill_n (numericData.getWritePointer (c), numericData.getNumSamples(), (NumericType) 1.0);

            numericData.applyGainRamp (0, numericData.getNumSamples(), (NumericType) 0.127, (NumericType) 17.3);

            auto lastUnchangedIndexBeforeCopiedRange = (int) ((numSIMDElements * 2) - 1);
            auto firstUnchangedIndexAfterCopiedRange = (int) ((numSIMDElements * 4) + 1);
            auto unchangedElement1 = numericData.getSample (0, lastUnchangedIndexBeforeCopiedRange);
            auto unchangedElement2 = numericData.getSample (1, firstUnchangedIndexAfterCopiedRange);

            block.copyTo (numericData, 1, 2, 2);

            expectEquals (numericData.getSample (0, lastUnchangedIndexBeforeCopiedRange), unchangedElement1);
            expectEquals (numericData.getSample (1, firstUnchangedIndexAfterCopiedRange), unchangedElement2);
            expect (SampleType (numericData.getSample (0, 2 * (int) numSIMDElements)) == block.getSample (0, 1));
            expect (SampleType (numericData.getSample (1, 3 * (int) numSIMDElements)) == block.getSample (1, 2));

            numericData.applyGainRamp (0, numericData.getNumSamples(), (NumericType) 15.1, (NumericType) 0.7);

            auto unchangedSIMDElement1 = block.getSample (0, 1);
            auto unchangedSIMDElement2 = block.getSample (1, 4);

            block.copyFrom (numericData, 1, 2, 2);

            expect (block.getSample (0, 1) == unchangedSIMDElement1);
            expect (block.getSample (1, 4) == unchangedSIMDElement2);
            expectEquals (block.getSample (0, 2).get (0), numericData.getSample (0, (int) numSIMDElements));
            expectEquals (block.getSample (1, 3).get (0), numericData.getSample (1, (int) (numSIMDElements * 2)));

            if (numSIMDElements > 1)
            {
                expectEquals (block.getSample (0, 2).get (1), numericData.getSample (0, (int) (numSIMDElements + 1)));
                expectEquals (block.getSample (1, 3).get (1), numericData.getSample (1, (int) ((numSIMDElements * 2) + 1)));
            }
        */
    }
    
    pub fn smoothed_value_tests<T = SampleType>(&mut self) -> ScalarVoid<T> {
    
        todo!();
        /*
            block.fill ((SampleType) 1.0);
            SmoothedValue<SampleType> sv { (SampleType) 1.0 };
            sv.reset (1, 4);
            sv.setTargetValue ((SampleType) 0.0);

            block.multiplyBy (sv);
            expect (block.getSample (0, 2) < (SampleType) 1.0);
            expect (block.getSample (1, 2) < (SampleType) 1.0);
            expect (block.getSample (0, 2) > (SampleType) 0.0);
            expect (block.getSample (1, 2) > (SampleType) 0.0);
            expectEquals (block.getSample (0, 5), (SampleType) 0.0);
            expectEquals (block.getSample (1, 5), (SampleType) 0.0);

            sv.setCurrentAndTargetValue (-1.0f);
            sv.setTargetValue (0.0f);
            otherBlock.fill (-1.0f);
            block.replaceWithProductOf (otherBlock, sv);
            expect (block.getSample (0, 2) < (SampleType) 1.0);
            expect (block.getSample (1, 2) < (SampleType) 1.0);
            expect (block.getSample (0, 2) > (SampleType) 0.0);
            expect (block.getSample (1, 2) > (SampleType) 0.0);
            expectEquals (block.getSample (0, 5), (SampleType) 0.0);
            expectEquals (block.getSample (1, 5), (SampleType) 0.0);
        */
    }
    
    pub fn smoothed_value_tests<T = SampleType>(&mut self) -> SIMDVoid<T> {
    
        todo!();
        /*
        
        */
    }
    
    pub fn reset_blocks(&mut self)  {
        
        todo!();
        /*
            auto value = SampleType (1.0);

            for (size_t c = 0; c < block.getNumChannels(); ++c)
            {
                for (size_t i = 0; i < block.getNumSamples(); ++i)
                {
                    block.setSample ((int) c, (int) i, value);
                    value += SampleType (1.0);
                }
            }

            otherBlock.replaceWithNegativeOf (block);
        */
    }
    
    pub fn allocate_aligned_memory(num_samples_to_allocate: i32) -> *mut SampleType {
        
        todo!();
        /*
            auto alignmentLowerBound = std::alignment_of<SampleType>::value;
           #if ! ALOE_WINDOWS
            alignmentLowerBound = jmax (sizeof (void*), alignmentLowerBound);
           #endif
            auto alignmentOrder = std::ceil (std::log2 (alignmentLowerBound));
            auto requiredAlignment = (size_t) std::pow (2, alignmentOrder);

            auto size = (size_t) numSamplesToAllocate * sizeof (SampleType);

           #if ALOE_WINDOWS
            auto* memory = _aligned_malloc (size, requiredAlignment);
           #else
            void* memory;
            auto result = posix_memalign (&memory, requiredAlignment, size);

            if (result != 0)
            {
                jassertfalse;
                return nullptr;
            }
           #endif

            return static_cast<SampleType*> (memory);
        */
    }
    
    pub fn deallocate_aligned_memory(&mut self, address: *mut c_void)  {
        
        todo!();
        /*
            #if ALOE_WINDOWS
            _aligned_free (address);
           #else
            free (address);
           #endif
        */
    }
}
    
lazy_static!{
    /*
    static AudioBlockUnitTests<float> audioBlockFloatUnitTests;
    static AudioBlockUnitTests<double> audioBlockDoubleUnitTests;

    #[cfg(ALOE_USE_SIMD)]
    static AudioBlockUnitTests<SIMDRegister<float>> audioBlockSIMDFloatUnitTests;

    #[cfg(ALOE_USE_SIMD)]
    static AudioBlockUnitTests<SIMDRegister<double>> audioBlockSIMDDoubleUnitTests;
    */
}
