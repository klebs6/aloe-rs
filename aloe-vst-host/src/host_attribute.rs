crate::ix!();

pub enum HostAttributeType
{
    Integer,
    Float,
    String,
    Binary
}

pub union HostAttributeV
{
    int_value:    i64,
    float_value:  f64,
    string_value: *mut TChar,
    binary_value: *mut u8,
}

pub struct HostAttribute {
    v:    HostAttributeV,
    size: u32,
    ty:   HostAttributeType,
}

impl Drop for HostAttribute {

    fn drop(&mut self) {
        todo!();
        /*
            if (size)
                delete[] v.binaryValue;
        */
    }
}

impl From<i64> for HostAttribute {

    fn from(value: i64) -> Self {
    
        todo!();
        /*
        : size(0),
        : ty(kInteger),

            v.intValue = value;
        */
    }
}

impl From<f64> for HostAttribute {
    
    fn from(value: f64) -> Self {
    
        todo!();
        /*
        : size(0),
        : ty(kFloat),

            v.floatValue = value;
        */
    }
}

impl HostAttribute {

    /**
      | size is in code unit (count of TChar)
      |
      */
    pub fn new_from_tchar_raw(
        value:             *const TChar,
        size_in_code_unit: u32

    ) -> Self {
    
        todo!();
        /*
        : size(sizeInCodeUnit),
        : ty(kString),

            v.stringValue = new TChar[sizeInCodeUnit];
            memcpy (v.stringValue, value, sizeInCodeUnit * sizeof (TChar));
        */
    }
    
    pub fn new(
        value:         *const c_void,
        size_in_bytes: u32

    ) -> Self {
    
        todo!();
        /*
        : size(sizeInBytes),
        : ty(kBinary),

            v.binaryValue = new char[sizeInBytes];
            memcpy (v.binaryValue, value, sizeInBytes);
        */
    }
    
    pub fn int_value(&self) -> i64 {
        
        todo!();
        /*
            return v.intValue;
        */
    }
    
    pub fn float_value(&self) -> f64 {
        
        todo!();
        /*
            return v.floatValue;
        */
    }

    /**
      | sizeInCodeUnit is in code unit (count
      | of TChar)
      |
      */
    pub fn string_value(&mut self, size_in_code_unit: &mut u32) -> *const TChar {
        
        todo!();
        /*
            sizeInCodeUnit = size;
            return v.stringValue;
        */
    }
    
    pub fn binary_value(&mut self, size_in_bytes: &mut u32)  {
        
        todo!();
        /*
            sizeInBytes = size;
            return v.binaryValue;
        */
    }
    
    pub fn get_type(&self) -> HostAttributeType {
        
        todo!();
        /*
            return type;
        */
    }
}
