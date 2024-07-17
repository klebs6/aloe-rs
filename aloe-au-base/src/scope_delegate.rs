crate::ix!();

pub trait AUScopeDelegateInterface {

    fn set_number_of_elements(&mut self, num_elements: u32);

    fn get_number_of_elements(&mut self) -> u32;

    fn get_element(&mut self, element_index: u32) -> *mut AUElement;
}

/**
  | AUScopeDelegates are a way to get virtual
  | scopes.
  |
  */
pub struct AUScopeDelegate {
    creator: *mut AUBase,
    scope:   AudioUnitScope,
}

impl Default for AUScopeDelegate {
    
    fn default() -> Self {
        todo!();
        /*
        : creator(NULL),
        : scope(0),

        
        */
    }
}

impl AUScopeDelegate {

    pub fn initialize(&mut self, 
        creator:      *mut AUBase,
        scope:        AudioUnitScope,
        num_elements: u32)  {
        
        todo!();
        /*
            mCreator = creator;
            mScope = scope;
            SetNumberOfElements(numElements);
        */
    }
    
    pub fn get_creator(&self) -> *mut AUBase {
        
        todo!();
        /*
            return mCreator;
        */
    }
    
    pub fn get_scope(&self) -> AudioUnitScope {
        
        todo!();
        /*
            return mScope;
        */
    }
}

