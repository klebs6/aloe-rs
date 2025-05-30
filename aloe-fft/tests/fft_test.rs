crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/frequency/aloe_FFT_test.cpp]

pub struct FFTUnitTest {
    base: UnitTest,
}

impl Default for FFTUnitTest {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("FFT", UnitTestCategories::dsp
        */
    }
}

pub mod fft_unit_test {
    use super::*;

    pub struct RealTest { }

    impl RealTest {

        pub fn run(u: &mut FFTUnitTest)  {
            
            todo!();
            /*
                Random random (378272);

                    for (size_t order = 0; order <= 8; ++order)
                    {
                        auto n = (1u << order);

                        FFT fft ((int) order);

                        HeapBlock<float> input (n);
                        HeapBlock<Complex<float>> reference (n), output (n);

                        fillRandom (random, input.getData(), n);
                        performReferenceFourier (input.getData(), reference.getData(), n, false);

                        // fill only first half with real numbers
                        zeromem (output.getData(), n * sizeof (Complex<float>));
                        memcpy (reinterpret_cast<float*> (output.getData()), input.getData(), n * sizeof (float));

                        fft.performRealOnlyForwardTransform ((float*) output.getData());
                        u.expect (checkArrayIsSimilar (reference.getData(), output.getData(), n));

                        // fill only first half with real numbers
                        zeromem (output.getData(), n * sizeof (Complex<float>));
                        memcpy (reinterpret_cast<float*> (output.getData()), input.getData(), n * sizeof (float));

                        fft.performRealOnlyForwardTransform ((float*) output.getData(), true);
                        std::fill (reference.getData() + ((n >> 1) + 1), reference.getData() + n, std::complex<float> (0.0f));
                        u.expect (checkArrayIsSimilar (reference.getData(), output.getData(), (n >> 1) + 1));

                        memcpy (output.getData(), reference.getData(), n * sizeof (Complex<float>));
                        fft.performRealOnlyInverseTransform ((float*) output.getData());
                        u.expect (checkArrayIsSimilar ((float*) output.getData(), input.getData(), n));
                    }
            */
        }
    }

    pub struct FrequencyOnlyTest { }

    impl FrequencyOnlyTest {

        pub fn run(u: &mut FFTUnitTest)  {
            
            todo!();
            /*
                Random random (378272);
                    for (size_t order = 0; order <= 8; ++order)
                    {
                        auto n = (1u << order);

                        FFT fft ((int) order);

                        HeapBlock<float> inout (n << 1), reference (n << 1);
                        HeapBlock<Complex<float>> frequency (n);

                        fillRandom (random, inout.getData(), n);
                        zeromem (reference.getData(), sizeof (float) * ((size_t) n << 1));
                        performReferenceFourier (inout.getData(), frequency.getData(), n, false);

                        for (size_t i = 0; i < n; ++i)
                            reference.getData()[i] = std::abs (frequency.getData()[i]);

                        fft.performFrequencyOnlyForwardTransform (inout.getData());

                        u.expect (checkArrayIsSimilar (inout.getData(), reference.getData(), n));
                    }
            */
        }
    }

    pub struct ComplexTest { }

    impl ComplexTest {

        pub fn run(u: &mut FFTUnitTest)  {
            
            todo!();
            /*
                Random random (378272);

                    for (size_t order = 0; order <= 7; ++order)
                    {
                        auto n = (1u << order);

                        FFT fft ((int) order);

                        HeapBlock<Complex<float>> input (n), buffer (n), output (n), reference (n);

                        fillRandom (random, input.getData(), n);
                        performReferenceFourier (input.getData(), reference.getData(), n, false);

                        memcpy (buffer.getData(), input.getData(), sizeof (Complex<float>) * n);
                        fft.perform (buffer.getData(), output.getData(), false);

                        u.expect (checkArrayIsSimilar (output.getData(), reference.getData(), n));

                        memcpy (buffer.getData(), reference.getData(), sizeof (Complex<float>) * n);
                        fft.perform (buffer.getData(), output.getData(), true);

                        u.expect (checkArrayIsSimilar (output.getData(), input.getData(), n));
                    }
            */
        }
    }
}

impl FFTUnitTest {

    pub fn fill_random(
        random: &mut Random,
        buffer: *mut Complex<f32>,
        n:      usize)  {
        
        todo!();
        /*
            for (size_t i = 0; i < n; ++i)
                buffer[i] = Complex<float> ((2.0f * random.nextFloat()) - 1.0f,
                                                 (2.0f * random.nextFloat()) - 1.0f);
        */
    }
    
    pub fn fill_random(
        random: &mut Random,
        buffer: *mut f32,
        n:      usize)  {
        
        todo!();
        /*
            for (size_t i = 0; i < n; ++i)
                buffer[i] = (2.0f * random.nextFloat()) - 1.0f;
        */
    }
    
    pub fn freq_convolution(
        in_:  *const Complex<f32>,
        freq: f32,
        n:    usize) -> Complex<f32> {
        
        todo!();
        /*
            Complex<float> sum (0.0, 0.0);
            for (size_t i = 0; i < n; ++i)
                sum += in[i] * exp (Complex<float> (0, static_cast<float> (i) * freq));

            return sum;
        */
    }
    
    pub fn perform_reference_fourier(
        in_:     *const Complex<f32>,
        out:     *mut Complex<f32>,
        n:       usize,
        reverse: bool)  {
        
        todo!();
        /*
            auto base_freq = static_cast<float> (((reverse ? 1.0 : -1.0) * MathConstants<double>::twoPi)
                                                   / static_cast<float> (n));

            for (size_t i = 0; i < n; ++i)
                out[i] = freqConvolution (in, static_cast<float>(i) * base_freq, n);
        */
    }
    
    pub fn perform_reference_fourier(
        in_:     *const f32,
        out:     *mut Complex<f32>,
        n:       usize,
        reverse: bool)  {
        
        todo!();
        /*
            HeapBlock<Complex<float>> buffer (n);

            for (size_t i = 0; i < n; ++i)
                buffer.getData()[i] = Complex<float> (in[i], 0.0f);

            float base_freq = static_cast<float> (((reverse ? 1.0 : -1.0) * MathConstants<double>::twoPi)
                                                    / static_cast<float> (n));

            for (size_t i = 0; i < n; ++i)
                out[i] = freqConvolution (buffer.getData(), static_cast<float>(i) * base_freq, n);
        */
    }
    
    pub fn check_array_is_similar<Type>(
        a: *mut Type,
        b: *mut Type,
        n: usize) -> bool {
    
        todo!();
        /*
            for (size_t i = 0; i < n; ++i)
                if (std::abs (a[i] - b[i]) > 1e-3f)
                    return false;

            return true;
        */
    }
    
    pub fn run_test_for_all_types<TheTest>(&mut self, unit_test_name: *const u8)  {
    
        todo!();
        /*
            beginTest (unitTestName);

            TheTest::run (*this);
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            runTestForAllTypes<RealTest> ("Real input numbers Test");
            runTestForAllTypes<FrequencyOnlyTest> ("Frequency only Test");
            runTestForAllTypes<ComplexTest> ("Complex input numbers Test");
        */
    }
}

lazy_static!{
    /*
    static FFTUnitTest fftUnitTest;
    */
}
