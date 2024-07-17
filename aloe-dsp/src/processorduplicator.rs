crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_ProcessorDuplicator.h]

/**
  | Converts a mono processor class into
  | a multi-channel version by duplicating
  | it and applying multichannel buffers
  | across an array of instances.
  | 
  | When the prepare method is called, it
  | uses the specified number of channels
  | to instantiate the appropriate number
  | of instances, which it then uses in its
  | process() method.
  | 
  | @tags{DSP}
  |
  */
pub struct ProcessorDuplicator<MonoProcessorType,StateType> {
    state:      Rc<RefCell<StateType>>,
    processors: Vec<Box<MonoProcessorType>>,
}

impl<MonoProcessorType,StateType> 
Default for ProcessorDuplicator<MonoProcessorType,StateType> {
    
    fn default() -> Self {
        todo!();
        /*


            : state (new StateType()
        */
    }
}

impl<MonoProcessorType,StateType> 
ProcessorDuplicator<MonoProcessorType,StateType> {

    pub fn new(state_to_use: *mut StateType) -> Self {
    
        todo!();
        /*
        : state(stateToUse),

        
        */
    }
    
    pub fn new_from_state_type_ptr(state_to_use: Rc<RefCell<StateType>>) -> Self {
    
        todo!();
        /*
        : state(std::move (stateToUse)),

        
        */
    }
    
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            processors.removeRange ((int) spec.numChannels, processors.size());

            while (static_cast<size_t> (processors.size()) < spec.numChannels)
                processors.add (new MonoProcessorType (state));

            auto monoSpec = spec;
            monoSpec.numChannels = 1;

            for (auto* p : processors)
                p->prepare (monoSpec);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto* p : processors) p->reset();
        */
    }
    
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            jassert ((int) context.getInputBlock().getNumChannels()  <= processors.size());
            jassert ((int) context.getOutputBlock().getNumChannels() <= processors.size());

            auto numChannels = static_cast<size_t> (jmin (context.getInputBlock().getNumChannels(),
                                                          context.getOutputBlock().getNumChannels()));

            for (size_t chan = 0; chan < numChannels; ++chan)
                processors[(int) chan]->process (MonoProcessContext<ProcessContext> (context, chan));
        */
    }
}
