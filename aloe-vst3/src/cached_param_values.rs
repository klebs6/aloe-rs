crate::ix!();

/**
  | Provides very quick polling of all parameter
  | states.
  | 
  | We must iterate all parameters on each
  | processBlock call to check whether
  | any parameter value has changed. This
  | class attempts to make this polling
  | process as quick as possible.
  | 
  | The indices here are of type typename
  | i32, as they are expected
  | to correspond to parameter information
  | obtained from the IEditController.
  | These indices may not match the indices
  | of parameters returned from AudioProcessor::getParameters(),
  | so be careful!
  |
  */
#[derive(Default)]
pub struct CachedParamValues {
    param_ids:   Vec<ParamID>,
    float_cache: FloatCache,
}

impl CachedParamValues {

    pub fn new(param_ids_in: Vec<ParamID>) -> Self {
    
        todo!();
        /*


            : paramIds (std::move (paramIdsIn)), floatCache (paramIds.size())
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return floatCache.size();
        */
    }
    
    pub fn get_paramid(&self, index: i32) -> ParamID {
        
        todo!();
        /*
            return paramIds[(size_t) index];
        */
    }
    
    pub fn set(&mut self, 
        index: i32,
        value: f32)  {
        
        todo!();
        /*
            floatCache.set                 ((size_t) index, value);
        */
    }
    
    pub fn set_without_notifying(&mut self, 
        index: i32,
        value: f32)  {
        
        todo!();
        /*
            floatCache.setWithoutNotifying ((size_t) index, value);
        */
    }
    
    pub fn get(&self, index: i32) -> f32 {
        
        todo!();
        /*
            return floatCache.get ((size_t) index);
        */
    }
    
    
    pub fn if_set<Callback>(&mut self, callback: Callback)  {
    
        todo!();
        /*
            floatCache.ifSet ([&] (size_t index, float value)
            {
                callback ((typename i32) index, value);
            });
        */
    }
}
