crate::ix!();

pub trait GetBypassParameter {

    /**
      | Returns the parameter that controls
      | the AudioProcessor's bypass state.
      | 
      | If this method returns a nullptr then
      | you can still control the bypass by calling
      | processBlockBypassed instead of processBlock.
      | On the other hand, if this method returns
      | a non-null value, you should never call
      | processBlockBypassed but use the returned
      | parameter to control the bypass state
      | instead.
      | 
      | A plug-in can override this function
      | to return a parameter which control's
      | your plug-in's bypass. You should always
      | check the value of this parameter in
      | your processBlock callback and bypass
      | any effects if it is non-zero.
      |
      */
    fn get_bypass_parameter(&self) -> *mut dyn AudioProcessorParameterInterface {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

pub trait IsBypassed {

    /**
      | Returns if the node is bypassed or not.
      |
      */
    fn is_bypassed(&self) -> bool;
}

pub trait SetBypassed {

    /**
      | Tell this node to bypass processing.
      |
      */
    fn set_bypassed(&mut self, should_be_bypassed: bool);
}
