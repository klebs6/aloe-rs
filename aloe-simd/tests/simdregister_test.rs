crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/containers/aloe_SIMDRegister_test.cpp]

pub struct RandomPrimitive<type> {
    
    pub fn next_float(random: &mut Random) -> HistCell {
        
        todo!();
        /*
            return static_cast<type> (std::is_signed<type>::value ? (random.nextFloat() * 16.0) - 8.0
                                                                  : (random.nextFloat() * 8.0));
        */
    }
    
    pub fn next_integral(random: &mut Random) -> HistCell {
        
        todo!();
        /*
            return static_cast<type> (random.nextInt64());
        */
    }
}

//---------------------------------------
pub struct RandomValue<type> {

}

impl RandomValue<type> {
    
    pub fn next(random: &mut Random) -> HistCell {
        
        todo!();
        /*
            return RandomPrimitive<type>::next (random);
        */
    }
    
    pub fn next_complex(random: &mut Random) -> Complex<HistCell> {
        
        todo!();
        /*
            return {RandomPrimitive<type>::next (random), RandomPrimitive<type>::next (random)};
        */
    }
}

//---------------------------------------
pub struct VecFiller<type> {

}

impl VecFiller<type> {
    
    pub fn fill(
        dst:    *mut HistCell,
        size:   i32,
        random: &mut Random)  {
        
        todo!();
        /*
            for (int i = 0; i < size; ++i)
                dst[i] = RandomValue<type>::next (random);
        */
    }

    // We need to specialise for complex types: otherwise
    // GCC 6 gives us an ICE internal compiler error after
    // which the compiler seg faults.
    //
    pub fn fill_complex(
        dst:    *mut Complex<HistCell>,
        size:   i32,
        random: &mut Random)  {
        
        todo!();
        /*
            for (int i = 0; i < size; ++i)
                dst[i] = std::complex<type> (RandomValue<type>::next (random), RandomValue<type>::next (random));
        */
    }
    
    pub fn fill_simd_register(random: &mut Random) -> SIMDRegister<HistCell> {
        
        todo!();
        /*
            constexpr int size = (int) SIMDRegister<type>::SIMDNumElements;
           #ifdef _MSC_VER
            __declspec(align(sizeof (SIMDRegister<type>))) type elements[size];
           #else
            type elements[(size_t) size] __attribute__((aligned(sizeof (SIMDRegister<type>))));
           #endif

            VecFiller<type>::fill (elements, size, random);
            return SIMDRegister<type>::fromRawArray (elements);
        */
    }
}

pub fn safe_abs<type>(a: HistCell) -> HistCell {

    todo!();
        /*
            return static_cast<type> (std::abs (static_cast<double> (a)));
        */
}

pub fn safe_abs<type>(a: Complex<HistCell>) -> HistCell {

    todo!();
        /*
            return std::abs (a);
        */
}

pub fn difference<type>(a: HistCell) -> f64 {

    todo!();
        /*
            return static_cast<double> (safeAbs (a));
        */
}

pub fn difference<type>(
    a: HistCell,
    b: HistCell) -> f64 {

    todo!();
        /*
            return difference (a - b);
        */
}

/**
  | These tests need to be strictly run on
  | all platforms supported by Aloe as the
  | SIMD code is highly platform dependent.
  |
  */
pub struct SIMDRegisterUnitTests {
    base: UnitTest,
}

impl Default for SIMDRegisterUnitTests {
    
    fn default() -> Self {
        todo!();
        /*
        : unit_test("SIMDRegister UnitTests", UnitTestCategories::dsp),

        
        */
    }
}

impl SIMDRegisterUnitTests {
    
    pub fn all_values_equal_to<type>(
        vec:    &SIMDRegister<HistCell>,
        scalar: HistCell) -> bool {
    
        todo!();
        /*
            #ifdef _MSC_VER
            __declspec(align(sizeof (SIMDRegister<type>))) type elements[SIMDRegister<type>::SIMDNumElements];
           #else
            type elements[SIMDRegister<type>::SIMDNumElements] __attribute__((aligned(sizeof (SIMDRegister<type>))));
           #endif

            vec.copyToRawArray (elements);

            // as we do not want to rely on the access operator we cast this to a primitive pointer
            for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                if (elements[i] != scalar) return false;

            return true;
        */
    }
    
    pub fn vec_equal_to_array<type>(
        vec:   &SIMDRegister<HistCell>,
        array: *const HistCell) -> bool {
    
        todo!();
        /*
            HeapBlock<type> vecElementsStorage (SIMDRegister<type>::SIMDNumElements * 2);
            auto* ptr = SIMDRegister<type>::getNextSIMDAlignedPtr (vecElementsStorage.getData());
            vec.copyToRawArray (ptr);

            for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
            {
                double delta = SIMDRegister_test_internal::difference (ptr[i], array[i]);
                if (delta > 1e-4)
                {
                    DBG ("a: " << SIMDRegister_test_internal::difference (ptr[i]) << " b: " << SIMDRegister_test_internal::difference (array[i]) << " difference: " << delta);
                    return false;
                }
            }

            return true;
        */
    }
    
    pub fn copy_<type>(
        vec: &mut SIMDRegister<HistCell>,
        ptr: *const HistCell)  {
    
        todo!();
        /*
            if (SIMDRegister<type>::isSIMDAligned (ptr))
            {
                vec = SIMDRegister<type>::fromRawArray (ptr);
            }
            else
            {
                for (size_t i = 0; i < SIMDRegister<type>::SIMDNumElements; ++i)
                    vec[i] = ptr[i];
            }
        */
    }
}

pub fn run_test_floating_point<TheTest>(unit_test_name: *const u8)  {

    todo!();
        /*
            beginTest (unitTestName);

            Random random = getRandom();

            TheTest::template run<float>  (*this, random);
            TheTest::template run<double> (*this, random);
        */
}

pub fn run_test_for_all_types<TheTest>(unit_test_name: *const u8)  {

    todo!();
        /*
            beginTest (unitTestName);

            Random random = getRandom();

            TheTest::template run<float>   (*this, random);
            TheTest::template run<double>  (*this, random);
            TheTest::template run<int8_t>  (*this, random);
            TheTest::template run<uint8_t> (*this, random);
            TheTest::template run<int16_t> (*this, random);
            TheTest::template run<uint16_t>(*this, random);
            TheTest::template run<int32_t> (*this, random);
            TheTest::template run<uint32_t>(*this, random);
            TheTest::template run<int64_t> (*this, random);
            TheTest::template run<uint64_t>(*this, random);
            TheTest::template run<std::complex<float>>   (*this, random);
            TheTest::template run<std::complex<double>>  (*this, random);
        */
}

pub fn run_test_non_complex<TheTest>(unit_test_name: *const u8)  {

    todo!();
        /*
            beginTest (unitTestName);

            Random random = getRandom();

            TheTest::template run<float>   (*this, random);
            TheTest::template run<double>  (*this, random);
            TheTest::template run<int8_t>  (*this, random);
            TheTest::template run<uint8_t> (*this, random);
            TheTest::template run<int16_t> (*this, random);
            TheTest::template run<uint16_t>(*this, random);
            TheTest::template run<int32_t> (*this, random);
            TheTest::template run<uint32_t>(*this, random);
            TheTest::template run<int64_t> (*this, random);
            TheTest::template run<uint64_t>(*this, random);
        */
}

pub fn run_test_signed<TheTest>(unit_test_name: *const u8)  {

    todo!();
        /*
            beginTest (unitTestName);

            Random random = getRandom();

            TheTest::template run<float>   (*this, random);
            TheTest::template run<double>  (*this, random);
            TheTest::template run<int8_t>  (*this, random);
            TheTest::template run<int16_t> (*this, random);
            TheTest::template run<int32_t> (*this, random);
            TheTest::template run<int64_t> (*this, random);
        */
}

pub fn run_test()  {
    
    todo!();
        /*
            runTestForAllTypes<InitializationTest> ("InitializationTest");

            runTestForAllTypes<AccessTest> ("AccessTest");

            runTestForAllTypes<OperatorTests<Addition>> ("AdditionOperators");
            runTestForAllTypes<OperatorTests<Subtraction>> ("SubtractionOperators");
            runTestForAllTypes<OperatorTests<Multiplication>> ("MultiplicationOperators");

            runTestForAllTypes<BitOperatorTests<BitAND>> ("BitANDOperators");
            runTestForAllTypes<BitOperatorTests<BitOR>>  ("BitOROperators");
            runTestForAllTypes<BitOperatorTests<BitXOR>> ("BitXOROperators");

            runTestNonComplex<CheckComparisonOps> ("CheckComparisons");
            runTestNonComplex<CheckBoolEquals> ("CheckBoolEquals");
            runTestNonComplex<CheckMinMax> ("CheckMinMax");

            runTestForAllTypes<CheckMultiplyAdd> ("CheckMultiplyAdd");
            runTestForAllTypes<CheckSum> ("CheckSum");

            runTestSigned<CheckAbs> ("CheckAbs");

            runTestFloatingPoint<CheckTruncate> ("CheckTruncate");
        */
}

lazy_static!{
    /*
    static SIMDRegisterUnitTests SIMDRegisterUnitTests;
    */
}
