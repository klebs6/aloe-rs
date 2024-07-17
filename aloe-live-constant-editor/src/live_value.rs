crate::ix!();

#[no_copy]
pub struct LiveValue<Type> {
    base: LiveValueBase,
    value:          Type,
    original_value: Type,
}

impl<Type> LiveValue<Type> {
    
    #[inline] pub fn into_type(self) -> Type {
        todo!();
        /*
            return value;
        */
    }
}

impl<Type> Into<*mut u8> for LiveValue<Type> {
    
    #[inline] fn into(self) -> *mut u8 {
        todo!();
        /*
            return castToCharPointer (value);
        */
    }
}

impl<Type> LiveValue<Type> {

    pub fn new(
        file:          *const u8,
        line:          i32,
        initial_value: &Type) -> Self {
    
        todo!();
        /*
        : live_value_base(file, line),
        : value(initialValue),
        : original_value(initialValue),

        
        */
    }
    
    pub fn get(&self) -> Type {
        
        todo!();
        /*
            return value;
        */
    }
    
    pub fn create_property_component(&mut self, doc: &mut CodeDocument) -> *mut LivePropertyEditorBase {
        
        todo!();
        /*
            return new LivePropertyEditor<Type> (*this, doc);
        */
    }
    
    pub fn get_string_value(&self, prefer_hex: bool) -> String {
        
        todo!();
        /*
            return getAsString (value, preferHex);
        */
    }
    
    pub fn get_code_value(&self, prefer_hex: bool) -> String {
        
        todo!();
        /*
            return getAsCode (value, preferHex);
        */
    }
    
    pub fn get_original_string_value(&self, prefer_hex: bool) -> String {
        
        todo!();
        /*
            return getAsString (originalValue, preferHex);
        */
    }
    
    pub fn set_string_value(&mut self, s: &String)  {
        
        todo!();
        /*
            setFromString (value, s);
        */
    }
    
    pub fn is_string(&self) -> bool {
        
        todo!();
        /*
            return isStringType<Type>::value;
        */
    }
}
