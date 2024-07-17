crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_FIRFilter_test.cpp]

pub struct FIRFilterTest {
    base: UnitTest,
}

impl Default for FIRFilterTest {
    
    fn default() -> Self {
        todo!();
        /*
        : unit_test("FIR Filter", UnitTestCategories::dsp),

        
        */
    }
}

impl FIRFilterTest {

    pub fn fill_random<Type>(
        random: &mut Random,
        buffer: *mut Type,
        n:      usize)  {
    
        todo!();
        /*
            FIRFilterTestHelpers<Type>::fillRandom (random, buffer, n);
        */
    }
    
    pub fn check_array_is_similar<Type>(
        a: *mut Type,
        b: *mut Type,
        n: usize) -> bool {
    
        todo!();
        /*
            return FIRFilterTestHelpers<Type>::checkArrayIsSimilar (a, b, n);
        */
    }

    /**
      | reference implementation of an FIR
      |
      */
    pub fn reference<SampleType, NumericType>(
        fir_coefficients: *const NumericType,
        num_coefficients: usize,
        input:            *const SampleType,
        output:           *mut SampleType,
        n:                usize)  {
    
        todo!();
        /*
            if (numCoefficients == 0)
            {
                zeromem (output, sizeof (SampleType) * n);
                return;
            }

            HeapBlock<SampleType> scratchBuffer (numCoefficients
                                                #if ALOE_USE_SIMD
                                                 + (SIMDRegister<NumericType>::SIMDRegisterSize / sizeof (SampleType))
                                                #endif
                                                 );
           #if ALOE_USE_SIMD
            SampleType* buffer = reinterpret_cast<SampleType*> (SIMDRegister<NumericType>::getNextSIMDAlignedPtr (reinterpret_cast<NumericType*> (scratchBuffer.getData())));
           #else
            SampleType* buffer = scratchBuffer.getData();
           #endif

            zeromem (buffer, sizeof (SampleType) * numCoefficients);

            for (size_t i = 0; i < n; ++i)
            {
                for (size_t j = (numCoefficients - 1); j >= 1; --j)
                    buffer[j] = buffer[j-1];

                buffer[0] = input[i];

                SampleType sum (0);

                for (size_t j = 0; j < numCoefficients; ++j)
                    sum += buffer[j] * firCoefficients[j];

                output[i] = sum;
            }
        */
    }
    
    pub fn run_test_for_type<TheTest, SampleType, NumericType>(&mut self)  {
    
        todo!();
        /*
            Random random (8392829);

            for (auto size : {1, 2, 4, 8, 12, 13, 25})
            {
                constexpr size_t n = 813;

                HeapBlock<char> inputBuffer, outputBuffer, refBuffer;
                AudioBlock<SampleType> input (inputBuffer, 1, n), output (outputBuffer, 1, n), ref (refBuffer, 1, n);
                fillRandom (random, input.getChannelPointer (0), n);

                HeapBlock<char> firBlock;
                AudioBlock<NumericType> fir (firBlock, 1, static_cast<size_t> (size));
                fillRandom (random, fir.getChannelPointer (0), static_cast<size_t> (size));

                FIR::Filter<SampleType> filter (*new FIR::Coefficients<NumericType> (fir.getChannelPointer (0), static_cast<size_t> (size)));
                ProcessSpec spec {0.0, n, 1};
                filter.prepare (spec);

                reference<SampleType, NumericType> (fir.getChannelPointer (0), static_cast<size_t> (size),
                                                    input.getChannelPointer (0), ref.getChannelPointer (0), n);

                TheTest::template run<SampleType> (filter, input.getChannelPointer (0), output.getChannelPointer (0), n);
                expect (checkArrayIsSimilar (output.getChannelPointer (0), ref.getChannelPointer (0), n));
            }
        */
    }
    
    pub fn run_test_for_all_types<TheTest>(&mut self, unit_test_name: *const u8)  {
    
        todo!();
        /*
            beginTest (unitTestName);

            runTestForType<TheTest, float, float>();
            runTestForType<TheTest, double, double>();
           #if ALOE_USE_SIMD
            runTestForType<TheTest, SIMDRegister<float>, float>();
            runTestForType<TheTest, SIMDRegister<double>, double>();
           #endif
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            runTestForAllTypes<FIRFilterTestLargeBlockTest> ("Large Blocks");
            runTestForAllTypes<FIRFilterTestSampleBySampleTest> ("Sample by Sample");
            runTestForAllTypes<FIRFilterTestSplitBlockTest> ("Split Block");
        */
    }
}

lazy_static!{
    /*
    static FIRFilterTest firFilterUnitTest;
    */
}
