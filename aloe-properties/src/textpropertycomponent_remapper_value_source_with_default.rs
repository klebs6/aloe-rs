crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TextPropertyComponentRemapperValueSourceWithDefault<'a> {
    base:               ValueSource<'a>,
    value_with_default: WeakReference<ValueWithDefault<'a>>,
}

impl<'a> TextPropertyComponentRemapperValueSourceWithDefault<'a> {

    pub fn new(vwd: *mut ValueWithDefault) -> Self {
    
        todo!();
        /*
        : value_with_default(vwd),

        
        */

    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            if (valueWithDefault == nullptr || valueWithDefault->isUsingDefault())
                return {};

            return valueWithDefault->get();
        */

    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            if (valueWithDefault == nullptr)
                return;

            if (newValue.toString().isEmpty())
                valueWithDefault->resetToDefault();
            else
                *valueWithDefault = newValue;
        */

    }
}

