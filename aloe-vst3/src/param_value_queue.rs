crate::ix!();

/**
  | A queue which can store up to one element.
  | 
  | This is more memory-efficient than
  | storing large vectors of parameter
  | changes that we'll just throw away.
  |
  */
#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[ALOE_DECLARE_Vst3_COM_QUERY_METHODS]
pub struct ParamValueQueue {
    param_id:        ParamID,
    parameter_index: i32,
    cached_value:    f32,
    size:            i32, // default = 0
    ref_count:       Atomic<i32>,
}

impl IParamValueQueue for ParamValueQueue {

    #[PLUGIN_API]
    fn get_parameter_id(&mut self) -> ParamID {
        
        todo!();
        /*
            return paramId;
        */
    }
    
    #[PLUGIN_API]
    fn get_point_count(&mut self) -> i32 {
        
        todo!();
        /*
            return size;
        */
    }
    
    #[PLUGIN_API]
    fn get_point(&mut self, 
        index:         i32,
        sample_offset: &mut i32,
        value:         &mut ParamValue) -> tresult {
        
        todo!();
        /*
            if (! isPositiveAndBelow (index, size))
                return kResultFalse;

            sampleOffset = 0;
            value = cachedValue;

            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn add_point(
        &mut self, 
        _0:    i32,
        value: ParamValue,
        index: &mut i32

    ) -> tresult {
        
        todo!();
        /*
            index = size++;
            set ((float) value);

            return kResultTrue;
        */
    }
}

impl FUnknown for ParamValueQueue {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl ParamValueQueue {

    pub fn new(
        id_in:              ParamID,
        parameter_index_in: i32) -> Self {
    
        todo!();
        /*
        : param_id(idIn),
        : parameter_index(parameterIndexIn),

        
        */
    }
    
    pub fn get_parameter_index(&self) -> i32 {
        
        todo!();
        /*
            return parameterIndex;
        */
    }

    
    pub fn set(&mut self, value_in: f32)  {
        
        todo!();
        /*
            cachedValue = valueIn;
            size = 1;
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            size = 0;
        */
    }
    
    pub fn get(&self) -> f32 {
        
        todo!();
        /*
            jassert (size > 0);
            return cachedValue;
        */
    }
}
