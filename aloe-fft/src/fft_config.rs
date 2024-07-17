crate::ix!();

pub struct FftConfigFactor { 
    radix:  i32,
    length: i32,
}

#[no_copy]
#[leak_detector]
pub struct FFTConfig {
    fft_size:      i32,
    inverse:       bool,
    factors:       [FftConfigFactor; 32],
    twiddle_table: HeapBlock<Complex<f32>>,
}

impl FFTConfig {

    pub fn new(
        size_offft: i32,
        is_inverse: bool) -> Self {
    
        todo!();
        /*
            : fftSize (sizeOfFFT), inverse (isInverse), twiddleTable ((size_t) sizeOfFFT)

                auto inverseFactor = (inverse ? 2.0 : -2.0) * MathConstants<double>::pi / (double) fftSize;

                if (fftSize <= 4)
                {
                    for (int i = 0; i < fftSize; ++i)
                    {
                        auto phase = i * inverseFactor;

                        twiddleTable[i] = { (float) std::cos (phase),
                                            (float) std::sin (phase) };
                    }
                }
                else
                {
                    for (int i = 0; i < fftSize / 4; ++i)
                    {
                        auto phase = i * inverseFactor;

                        twiddleTable[i] = { (float) std::cos (phase),
                                            (float) std::sin (phase) };
                    }

                    for (int i = fftSize / 4; i < fftSize / 2; ++i)
                    {
                        auto other = twiddleTable[i - fftSize / 4];

                        twiddleTable[i] = { inverse ? -other.imag() :  other.imag(),
                                            inverse ?  other.real() : -other.real() };
                    }

                    twiddleTable[fftSize / 2].real (-1.0f);
                    twiddleTable[fftSize / 2].imag (0.0f);

                    for (int i = fftSize / 2; i < fftSize; ++i)
                    {
                        auto index = fftSize / 2 - (i - fftSize / 2);
                        twiddleTable[i] = conj(twiddleTable[index]);
                    }
                }

                auto root = (int) std::sqrt ((double) fftSize);
                int divisor = 4, n = fftSize;

                for (int i = 0; i < numElementsInArray (factors); ++i)
                {
                    while ((n % divisor) != 0)
                    {
                        if (divisor == 2)       divisor = 3;
                        else if (divisor == 4)  divisor = 2;
                        else                    divisor += 2;

                        if (divisor > root)
                            divisor = n;
                    }

                    n /= divisor;

                    jassert (divisor == 1 || divisor == 2 || divisor == 4);
                    factors[i].radix = divisor;
                    factors[i].length = n;
                }
        */
    }
    
    pub fn perform(&self, 
        input:  *const Complex<f32>,
        output: *mut Complex<f32>)  {
        
        todo!();
        /*
            perform (input, output, 1, 1, factors);
        */
    }
    
    pub fn perform_with_fft_config_factor(
        &self, 
        input:     *const Complex<f32>,
        output:    *mut Complex<f32>,
        stride:    i32,
        stride_in: i32,
        facs:      *const FftConfigFactor

    ) {
        
        todo!();
        /*
            auto factor = *facs++;
                auto* originalOutput = output;
                auto* outputEnd = output + factor.radix * factor.length;

                if (stride == 1 && factor.radix <= 5)
                {
                    for (int i = 0; i < factor.radix; ++i)
                        perform (input + stride * strideIn * i, output + i * factor.length, stride * factor.radix, strideIn, facs);

                    butterfly (factor, output, stride);
                    return;
                }

                if (factor.length == 1)
                {
                    do
                    {
                        *output++ = *input;
                        input += stride * strideIn;
                    }
                    while (output < outputEnd);
                }
                else
                {
                    do
                    {
                        perform (input, output, stride * factor.radix, strideIn, facs);
                        input += stride * strideIn;
                        output += factor.length;
                    }
                    while (output < outputEnd);
                }

                butterfly (factor, originalOutput, stride);
        */
    }
    
    pub fn butterfly(&self, 
        factor: FftConfigFactor,
        data:   *mut Complex<f32>,
        stride: i32)  {
        
        todo!();
        /*
            switch (factor.radix)
                {
                    case 1:   break;
                    case 2:   butterfly2 (data, stride, factor.length); return;
                    case 4:   butterfly4 (data, stride, factor.length); return;
                    default:  jassertfalse; break;
                }

                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6255)
                auto* scratch = static_cast<Complex<float>*> (alloca ((size_t) factor.radix * sizeof (Complex<float>)));
                ALOE_END_IGNORE_WARNINGS_MSVC

                for (int i = 0; i < factor.length; ++i)
                {
                    for (int k = i, q1 = 0; q1 < factor.radix; ++q1)
                    {
                        ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6386)
                        scratch[q1] = data[k];
                        ALOE_END_IGNORE_WARNINGS_MSVC
                        k += factor.length;
                    }

                    for (int k = i, q1 = 0; q1 < factor.radix; ++q1)
                    {
                        int twiddleIndex = 0;
                        data[k] = scratch[0];

                        for (int q = 1; q < factor.radix; ++q)
                        {
                            twiddleIndex += stride * k;

                            if (twiddleIndex >= fftSize)
                                twiddleIndex -= fftSize;

                            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6385)
                            data[k] += scratch[q] * twiddleTable[twiddleIndex];
                            ALOE_END_IGNORE_WARNINGS_MSVC
                        }

                        k += factor.length;
                    }
                }
        */
    }

    pub fn butterfly2(&self, 
        data:   *mut Complex<f32>,
        stride: i32,
        length: i32)  {
        
        todo!();
        /*
            auto* dataEnd = data + length;
                auto* tw = twiddleTable.getData();

                for (int i = length; --i >= 0;)
                {
                    auto s = *dataEnd;
                    s *= (*tw);
                    tw += stride;
                    *dataEnd++ = *data - s;
                    *data++ += s;
                }
        */
    }

    pub fn butterfly4(&self, 
        data:   *mut Complex<f32>,
        stride: i32,
        length: i32)  {
        
        todo!();
        /*
            auto lengthX2 = length * 2;
                auto lengthX3 = length * 3;

                auto strideX2 = stride * 2;
                auto strideX3 = stride * 3;

                auto* twiddle1 = twiddleTable.getData();
                auto* twiddle2 = twiddle1;
                auto* twiddle3 = twiddle1;

                for (int i = length; --i >= 0;)
                {
                    auto s0 = data[length]   * *twiddle1;
                    auto s1 = data[lengthX2] * *twiddle2;
                    auto s2 = data[lengthX3] * *twiddle3;
                    auto s3 = s0;             s3 += s2;
                    auto s4 = s0;             s4 -= s2;
                    auto s5 = *data;          s5 -= s1;

                    *data += s1;
                    data[lengthX2] = *data;
                    data[lengthX2] -= s3;
                    twiddle1 += stride;
                    twiddle2 += strideX2;
                    twiddle3 += strideX3;
                    *data += s3;

                    if (inverse)
                    {
                        data[length] = { s5.real() - s4.imag(),
                                         s5.imag() + s4.real() };

                        data[lengthX3] = { s5.real() + s4.imag(),
                                           s5.imag() - s4.real() };
                    }
                    else
                    {
                        data[length] = { s5.real() + s4.imag(),
                                         s5.imag() - s4.real() };

                        data[lengthX3] = { s5.real() - s4.imag(),
                                           s5.imag() + s4.real() };
                    }

                    ++data;
                }
        */
    }
}
