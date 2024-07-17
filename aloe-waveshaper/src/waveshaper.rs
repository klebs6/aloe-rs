crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_WaveShaper.h]

pub type DefaultWaveshapeFunction<FloatType: num::Float> = fn(x: FloatType) -> FloatType;

/**
  | Applies waveshaping to audio samples
  | as single samples or AudioBlocks.
  | 
  | @tags{DSP}
  |
  */
pub struct WaveShaper<FloatType: num::Float,Function: Fn(FloatType) -> FloatType = DefaultWaveshapeFunction<FloatType>> {
    function_to_use: Function,
    _0:              PhantomData<FloatType>,
}

impl<FloatType: num::Float,Function: Fn(FloatType) -> FloatType> WaveShaper<FloatType,Function> {

    /**
      | Called before processing starts.
      |
      */
    pub fn prepare(&mut self, _0: &ProcessSpec)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the result of processing a single
      | sample.
      |
      */
    pub fn process_sample<SampleType>(&self, input_sample: SampleType) -> SampleType {
    
        todo!();
        /*
            return functionToUse (inputSample);
        */
    }

    /**
      | Processes the input and output buffers
      | supplied in the processing context.
      |
      */
    pub fn process<ProcessContext>(&self, context: &ProcessContext)  {
    
        todo!();
        /*
            if (context.isBypassed)
            {
                if (context.usesSeparateInputAndOutputBlocks())
                    context.getOutputBlock().copyFrom (context.getInputBlock());
            }
            else
            {
                AudioBlock<FloatType>::process (context.getInputBlock(),
                                                context.getOutputBlock(),
                                                functionToUse);
            }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

pub fn create_wave_shaper<FloatType: num::Float, Functor: Fn(FloatType) -> FloatType>(function_to_use: Functor) 
-> WaveShaper<FloatType,Functor> 
{
    todo!();
    /*
        return {functionToUse};
    */
}
