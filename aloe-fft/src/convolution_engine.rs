crate::ix!();

///----------------------------
pub struct ConvolutionEngine {
    block_size:               usize,
    fft_size:                 usize,
    fft_object:               Box<FFT>,
    num_segments:             usize,
    num_input_segments:       usize,
    current_segment:          usize, // default = 0
    input_data_pos:           usize, // default = 0
    buffer_input:             AudioBuffer<f32>,
    buffer_output:            AudioBuffer<f32>,
    buffer_temp_output:       AudioBuffer<f32>,
    buffer_overlap:           AudioBuffer<f32>,
    buffers_input_segments:   Vec<AudioBuffer<f32>>,
    buffers_impulse_segments: Vec<AudioBuffer<f32>>,
}

impl ConvolutionEngine {

    pub fn new(
        samples:        *const f32,
        num_samples:    usize,
        max_block_size: usize) -> Self {
    
        todo!();
        /*


            : blockSize ((size_t) nextPowerOfTwo ((int) maxBlockSize)),
              fftSize (blockSize > 128 ? 2 * blockSize : 4 * blockSize),
              fftObject (std::make_unique<FFT> (roundToInt (std::log2 (fftSize)))),
              numSegments (numSamples / (fftSize - blockSize) + 1u),
              numInputSegments ((blockSize > 128 ? numSegments : 3 * numSegments)),
              bufferInput      (1, static_cast<int> (fftSize)),
              bufferOutput     (1, static_cast<int> (fftSize * 2)),
              bufferTempOutput (1, static_cast<int> (fftSize * 2)),
              bufferOverlap    (1, static_cast<int> (fftSize))

            bufferOutput.clear();

            auto updateSegmentsIfNecessary = [this] (size_t numSegmentsToUpdate,
                                                     std::vector<AudioBuffer<float>>& segments)
            {
                if (numSegmentsToUpdate == 0
                    || numSegmentsToUpdate != (size_t) segments.size()
                    || (size_t) segments[0].getNumSamples() != fftSize * 2)
                {
                    segments.clear();

                    for (size_t i = 0; i < numSegmentsToUpdate; ++i)
                        segments.push_back ({ 1, static_cast<int> (fftSize * 2) });
                }
            };

            updateSegmentsIfNecessary (numInputSegments, buffersInputSegments);
            updateSegmentsIfNecessary (numSegments,      buffersImpulseSegments);

            auto FFTTempObject = std::make_unique<FFT> (roundToInt (std::log2 (fftSize)));
            size_t currentPtr = 0;

            for (auto& buf : buffersImpulseSegments)
            {
                buf.clear();

                auto* impulseResponse = buf.getWritePointer (0);

                if (&buf == &buffersImpulseSegments.front())
                    impulseResponse[0] = 1.0f;

                FloatVectorOperations::copy (impulseResponse,
                                             samples + currentPtr,
                                             static_cast<int> (jmin (fftSize - blockSize, numSamples - currentPtr)));

                FFTTempObject->performRealOnlyForwardTransform (impulseResponse);
                prepareForConvolution (impulseResponse);

                currentPtr += (fftSize - blockSize);
            }

            reset();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            bufferInput.clear();
            bufferOverlap.clear();
            bufferTempOutput.clear();
            bufferOutput.clear();

            for (auto& buf : buffersInputSegments)
                buf.clear();

            currentSegment = 0;
            inputDataPos = 0;
        */
    }
    
    pub fn process_samples(&mut self, 
        input:       *const f32,
        output:      *mut f32,
        num_samples: usize)  {
        
        todo!();
        /*
            // Overlap-add, zero latency convolution algorithm with uniform partitioning
            size_t numSamplesProcessed = 0;

            auto indexStep = numInputSegments / numSegments;

            auto* inputData      = bufferInput.getWritePointer (0);
            auto* outputTempData = bufferTempOutput.getWritePointer (0);
            auto* outputData     = bufferOutput.getWritePointer (0);
            auto* overlapData    = bufferOverlap.getWritePointer (0);

            while (numSamplesProcessed < numSamples)
            {
                const bool inputDataWasEmpty = (inputDataPos == 0);
                auto numSamplesToProcess = jmin (numSamples - numSamplesProcessed, blockSize - inputDataPos);

                FloatVectorOperations::copy (inputData + inputDataPos, input + numSamplesProcessed, static_cast<int> (numSamplesToProcess));

                auto* inputSegmentData = buffersInputSegments[currentSegment].getWritePointer (0);
                FloatVectorOperations::copy (inputSegmentData, inputData, static_cast<int> (fftSize));

                fftObject->performRealOnlyForwardTransform (inputSegmentData);
                prepareForConvolution (inputSegmentData);

                // Complex multiplication
                if (inputDataWasEmpty)
                {
                    FloatVectorOperations::fill (outputTempData, 0, static_cast<int> (fftSize + 1));

                    auto index = currentSegment;

                    for (size_t i = 1; i < numSegments; ++i)
                    {
                        index += indexStep;

                        if (index >= numInputSegments)
                            index -= numInputSegments;

                        convolutionProcessingAndAccumulate (buffersInputSegments[index].getWritePointer (0),
                                                            buffersImpulseSegments[i].getWritePointer (0),
                                                            outputTempData);
                    }
                }

                FloatVectorOperations::copy (outputData, outputTempData, static_cast<int> (fftSize + 1));

                convolutionProcessingAndAccumulate (inputSegmentData,
                                                    buffersImpulseSegments.front().getWritePointer (0),
                                                    outputData);

                updateSymmetricFrequencyDomainData (outputData);
                fftObject->performRealOnlyInverseTransform (outputData);

                // Add overlap
                FloatVectorOperations::add (&output[numSamplesProcessed], &outputData[inputDataPos], &overlapData[inputDataPos], (int) numSamplesToProcess);

                // Input buffer full => Next block
                inputDataPos += numSamplesToProcess;

                if (inputDataPos == blockSize)
                {
                    // Input buffer is empty again now
                    FloatVectorOperations::fill (inputData, 0.0f, static_cast<int> (fftSize));

                    inputDataPos = 0;

                    // Extra step for segSize > blockSize
                    FloatVectorOperations::add (&(outputData[blockSize]), &(overlapData[blockSize]), static_cast<int> (fftSize - 2 * blockSize));

                    // Save the overlap
                    FloatVectorOperations::copy (overlapData, &(outputData[blockSize]), static_cast<int> (fftSize - blockSize));

                    currentSegment = (currentSegment > 0) ? (currentSegment - 1) : (numInputSegments - 1);
                }

                numSamplesProcessed += numSamplesToProcess;
            }
        */
    }
    
    pub fn process_samples_with_added_latency(&mut self, 
        input:       *const f32,
        output:      *mut f32,
        num_samples: usize)  {
        
        todo!();
        /*
            // Overlap-add, zero latency convolution algorithm with uniform partitioning
            size_t numSamplesProcessed = 0;

            auto indexStep = numInputSegments / numSegments;

            auto* inputData      = bufferInput.getWritePointer (0);
            auto* outputTempData = bufferTempOutput.getWritePointer (0);
            auto* outputData     = bufferOutput.getWritePointer (0);
            auto* overlapData    = bufferOverlap.getWritePointer (0);

            while (numSamplesProcessed < numSamples)
            {
                auto numSamplesToProcess = jmin (numSamples - numSamplesProcessed, blockSize - inputDataPos);

                FloatVectorOperations::copy (inputData + inputDataPos, input + numSamplesProcessed, static_cast<int> (numSamplesToProcess));

                FloatVectorOperations::copy (output + numSamplesProcessed, outputData + inputDataPos, static_cast<int> (numSamplesToProcess));

                numSamplesProcessed += numSamplesToProcess;
                inputDataPos += numSamplesToProcess;

                // processing itself when needed (with latency)
                if (inputDataPos == blockSize)
                {
                    // Copy input data in input segment
                    auto* inputSegmentData = buffersInputSegments[currentSegment].getWritePointer (0);
                    FloatVectorOperations::copy (inputSegmentData, inputData, static_cast<int> (fftSize));

                    fftObject->performRealOnlyForwardTransform (inputSegmentData);
                    prepareForConvolution (inputSegmentData);

                    // Complex multiplication
                    FloatVectorOperations::fill (outputTempData, 0, static_cast<int> (fftSize + 1));

                    auto index = currentSegment;

                    for (size_t i = 1; i < numSegments; ++i)
                    {
                        index += indexStep;

                        if (index >= numInputSegments)
                            index -= numInputSegments;

                        convolutionProcessingAndAccumulate (buffersInputSegments[index].getWritePointer (0),
                                                            buffersImpulseSegments[i].getWritePointer (0),
                                                            outputTempData);
                    }

                    FloatVectorOperations::copy (outputData, outputTempData, static_cast<int> (fftSize + 1));

                    convolutionProcessingAndAccumulate (inputSegmentData,
                                                        buffersImpulseSegments.front().getWritePointer (0),
                                                        outputData);

                    updateSymmetricFrequencyDomainData (outputData);
                    fftObject->performRealOnlyInverseTransform (outputData);

                    // Add overlap
                    FloatVectorOperations::add (outputData, overlapData, static_cast<int> (blockSize));

                    // Input buffer is empty again now
                    FloatVectorOperations::fill (inputData, 0.0f, static_cast<int> (fftSize));

                    // Extra step for segSize > blockSize
                    FloatVectorOperations::add (&(outputData[blockSize]), &(overlapData[blockSize]), static_cast<int> (fftSize - 2 * blockSize));

                    // Save the overlap
                    FloatVectorOperations::copy (overlapData, &(outputData[blockSize]), static_cast<int> (fftSize - blockSize));

                    currentSegment = (currentSegment > 0) ? (currentSegment - 1) : (numInputSegments - 1);

                    inputDataPos = 0;
                }
            }
        */
    }

    /**
      | After each FFT, this function is called
      | to allow convolution to be performed
      | with only 4 SIMD functions calls.
      |
      */
    pub fn prepare_for_convolution(&mut self, samples: *mut f32)  {
        
        todo!();
        /*
            auto FFTSizeDiv2 = fftSize / 2;

            for (size_t i = 0; i < FFTSizeDiv2; i++)
                samples[i] = samples[i << 1];

            samples[FFTSizeDiv2] = 0;

            for (size_t i = 1; i < FFTSizeDiv2; i++)
                samples[i + FFTSizeDiv2] = -samples[((fftSize - i) << 1) + 1];
        */
    }

    /**
      | Does the convolution operation itself
      | only on half of the frequency domain
      | samples.
      |
      */
    pub fn convolution_processing_and_accumulate(&mut self, 
        input:   *const f32,
        impulse: *const f32,
        output:  *mut f32)  {
        
        todo!();
        /*
            auto FFTSizeDiv2 = fftSize / 2;

            FloatVectorOperations::addWithMultiply      (output, input, impulse, static_cast<int> (FFTSizeDiv2));
            FloatVectorOperations::subtractWithMultiply (output, &(input[FFTSizeDiv2]), &(impulse[FFTSizeDiv2]), static_cast<int> (FFTSizeDiv2));

            FloatVectorOperations::addWithMultiply      (&(output[FFTSizeDiv2]), input, &(impulse[FFTSizeDiv2]), static_cast<int> (FFTSizeDiv2));
            FloatVectorOperations::addWithMultiply      (&(output[FFTSizeDiv2]), &(input[FFTSizeDiv2]), impulse, static_cast<int> (FFTSizeDiv2));

            output[fftSize] += input[fftSize] * impulse[fftSize];
        */
    }

    /**
      | Undoes the re-organization of samples from
      | the function prepareForConvolution.
      |
      | Then takes the conjugate of the frequency
      | domain first half of samples to fill the
      | second half, so that the inverse transform
      | will return real samples in the time
      | domain.
      */
    pub fn update_symmetric_frequency_domain_data(&mut self, samples: *mut f32)  {
        
        todo!();
        /*
            auto FFTSizeDiv2 = fftSize / 2;

            for (size_t i = 1; i < FFTSizeDiv2; i++)
            {
                samples[(fftSize - i) << 1] = samples[i];
                samples[((fftSize - i) << 1) + 1] = -samples[FFTSizeDiv2 + i];
            }

            samples[1] = 0.f;

            for (size_t i = 1; i < FFTSizeDiv2; i++)
            {
                samples[i << 1] = samples[(fftSize - i) << 1];
                samples[(i << 1) + 1] = -samples[((fftSize - i) << 1) + 1];
            }
        */
    }
}
