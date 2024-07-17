crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_ProcessorWrapper.h]

/**
  | Acts as a polymorphic base class for
  | processors.
  | 
  | This exposes the same set of methods
  | that a processor must implement as virtual
  | methods, so that you can use the ProcessorWrapper
  | class to wrap an instance of a subclass,
  | and then pass that around using ProcessorBase
  | as a base class. @see ProcessorWrapper
  | 
  | @tags{DSP}
  |
  */
pub trait ProcessorBase: Default {

    fn prepare(&mut self, _0: &ProcessSpec);

    fn process(&mut self, _0: &ProcessContextReplacing<f32>);

    fn reset(&mut self);
}

/**
  | Wraps an instance of a given processor
  | class, and exposes it through the ProcessorBase
  | interface. @see ProcessorBase
  | 
  | @tags{DSP}
  |
  */
pub struct ProcessorWrapper<ProcessorType> {
    processor: ProcessorType,
}

impl<ProcessorType> Default for ProcessorWrapper<ProcessorType> {

    fn default() -> Self {
        todo!();
    }
}

impl<ProcessorType> ProcessorBase for ProcessorWrapper<ProcessorType> {

    fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            processor.prepare (spec);
        */
    }
    
    fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            processor.process (context);
        */
    }
    
    fn reset(&mut self)  {
        
        todo!();
        /*
            processor.reset();
        */
    }
}
