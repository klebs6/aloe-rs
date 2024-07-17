crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/OverdriveDemo.h]

pub struct OverdriveDemoDSP<'a> {
    overdrive:      ProcessorChain /* <OverdriveGainProcessor,OverdriveBiasProcessor,OverdriveDriveProcessor,OverdriveDCFilter,OverdriveGainProcessor> */,
    in_gain_param:  SliderParameter<'a>,                //{ { -100.0, 60.0 }, 3, 24.0,  "Input Gain",  "dB" };
    out_gain_param: SliderParameter<'a>,                //{ { -100.0, 20.0 }, 3, -18.0, "Output Gain", "dB" };
    parameters:     Vec<*mut DSPDemoParameterBase<'a>>, //{ &inGainParam, &outGainParam };
    sample_rate:    f64, // default = 0.0
}

pub type OverdriveGainProcessor  = Gain<f32>;
pub type OverdriveBiasProcessor  = Bias<f32>;
pub type OverdriveDriveProcessor = WaveShaper<f32>;
pub type OverdriveDCFilter       = ProcessorDuplicator<IIRFilterTransposedDirectFormII<f32>,IIRCoefficients<f32>>;

impl<'a> OverdriveDemoDSP<'a> {

    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = spec.sampleRate;

            auto& gainUp = overdrive.get<0>();
            gainUp.setGainDecibels (24);

            auto& bias = overdrive.get<1>();
            bias.setBias (0.4f);

            auto& wavShaper = overdrive.get<2>();
            wavShaper.functionToUse = std::tanh;

            auto& dcFilter = overdrive.get<3>();
            dcFilter.state = IIR::Coefficients<float>::makeHighPass (sampleRate, 5.0);

            auto& gainDown = overdrive.get<4>();
            gainDown.setGainDecibels (-18.0f);

            overdrive.prepare (spec);
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            overdrive.process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            overdrive.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            if (sampleRate != 0.0)
            {
                overdrive.get<0>().setGainDecibels (static_cast<float> (inGainParam.getCurrentValue()));
                overdrive.get<4>().setGainDecibels (static_cast<float> (outGainParam.getCurrentValue()));
            }
        */
    }
}
