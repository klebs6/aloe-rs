crate::ix!();

pub struct FftEngineImpl<InstanceToUse> {
    base:             FftEngine,
    _instance_to_use: PhantomData<InstanceToUse>,
}

impl<InstanceToUse> Default for FftEngineImpl<InstanceToUse> {
    
    fn default() -> Self {
        todo!();
        /*
        : engine(InstanceToUse::priority),

        
        */
    }
}

impl<InstanceToUse> FftEngineImpl<InstanceToUse> {

    pub fn create(&self, order: i32) -> *mut dyn FftInstance {
        
        todo!();
        /*
            return InstanceToUse::create (order);
        */
    }
}
