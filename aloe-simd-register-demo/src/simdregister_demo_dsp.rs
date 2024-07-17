crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/SIMDRegisterDemo.h]

pub struct SIMDRegisterDemoDSP<'a, SampleType> 

where SampleType: SampleTypeInterface + HasIIRCoefficients,
      SIMDRegister<SampleType>: HasElementType + Float
    
{
    iir_coefficients:       IIRCoefficientsPtr<SampleType>,
    iir:                    Box<IIRFilter/*<SIMDRegister<SampleType>>*/>,
    interleaved:            AudioBlock<SIMDRegister<SampleType>>,
    zero:                   AudioBlock<SampleType>,
    interleaved_block_data: HeapBlock<u8>,
    zero_data:              HeapBlock<u8>,
    channel_pointers:       HeapBlock<*const SampleType>,           //{ SIMDRegister<f32>::size() };
    type_param:             ChoiceParameter<'a>,                 //{ { "Low-pass", "High-pass", "Band-pass" }, 1, "Type" };
    cutoff_param:           SliderParameter<'a>,                 //{ { 20.0, 20000.0 }, 0.5, 440.0f, "Cutoff", "Hz" };
    q_param:                SliderParameter<'a>,                 //{ { 0.3, 20.0 }, 0.5, 0.7, "Q" };
    parameters:             Vec<*mut DSPDemoParameterBase<'a>>,  //{ &typeParam, &cutoffParam, &qParam };
    sample_rate:            f64, // default = 0.0
}

impl<'a, SampleType> SIMDRegisterDemoDSP<'a, SampleType> 

where SampleType: SampleTypeInterface + HasIIRCoefficients,
      SIMDRegister<SampleType>: HasElementType + Float
{
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;

            iirCoefficients = IIR::Coefficients<f32>::makeLowPass (sampleRate, 440.0f);
            iir.reset (new IIR::Filter<SIMDRegister<f32>> (iirCoefficients));

            interleaved = AudioBlock<SIMDRegister<f32>> (interleavedBlockData, 1, spec.maximumBlockSize);
            zero        = AudioBlock<f32> (zeroData, SIMDRegister<f32>::size(), spec.maximumBlockSize);

            zero.clear();

            auto monoSpec = spec;
            monoSpec.numChannels = 1;
            iir->prepare (monoSpec);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            jassert (context.getInputBlock().getNumSamples()  == context.getOutputBlock().getNumSamples());
            jassert (context.getInputBlock().getNumChannels() == context.getOutputBlock().getNumChannels());

            auto& input  = context.getInputBlock();
            auto& output = context.getOutputBlock();
            auto n = input.getNumSamples();
            auto* inout = channelPointers.getData();

            for (size_t ch = 0; ch < SIMDRegister<f32>::size(); ++ch)
                inout[ch] = (ch < input.getNumChannels() ? const_cast<float*> (input.getChannelPointer (ch)) : zero.getChannelPointer (ch));

            AudioDataConverters::interleaveSamples (inout, reinterpret_cast<float*> (interleaved.getChannelPointer (0)),
                                                    static_cast<int> (n), static_cast<int> (SIMDRegister<f32>::size()));

            iir->process (ProcessContextReplacing<SIMDRegister<f32>> (interleaved));

            for (size_t ch = 0; ch < input.getNumChannels(); ++ch)
                inout[ch] = output.getChannelPointer (ch);

            AudioDataConverters::deinterleaveSamples (reinterpret_cast<float*> (interleaved.getChannelPointer (0)),
                                                      const_cast<float**> (inout),
                                                      static_cast<int> (n), static_cast<int> (SIMDRegister<f32>::size()));
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            iir.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            if (sampleRate != 0.0)
            {
                auto cutoff = static_cast<f32> (cutoffParam.getCurrentValue());
                auto qVal   = static_cast<f32> (qParam.getCurrentValue());

                switch (typeParam.getCurrentSelectedID())
                {
                    case 1:   *iirCoefficients = IIR::ArrayCoefficients<f32>::makeLowPass  (sampleRate, cutoff, qVal); break;
                    case 2:   *iirCoefficients = IIR::ArrayCoefficients<f32>::makeHighPass (sampleRate, cutoff, qVal); break;
                    case 3:   *iirCoefficients = IIR::ArrayCoefficients<f32>::makeBandPass (sampleRate, cutoff, qVal); break;
                    default:  break;
                }
            }
        */
    }
}
