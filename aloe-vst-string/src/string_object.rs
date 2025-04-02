crate::ix!();

/* ---------- StringObject Implementation  ---------- */

/**
  | StringObject implements IStringResult
  | and IString methods.
  | 
  | It can therefore be exchanged with other
  | Steinberg objects using one or both
  | of these interfaces.
  | 
  | \see String, ConstString
  |
  */
#[derive(Default)]
pub struct StringObject {
    base:  FObject,
    base2: String,
}

impl StringObject {

    pub fn new_from_u16_ptr(
        str_:          *const u16,
        n:             Option<i32>,
        is_terminated: Option<bool>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);
        let is_terminated: bool = is_terminated.unwrap_or(true);

        todo!();
        /*
        : string(str, n, isTerminated),

        
        */
    }
    
    pub fn new_from_u8_ptr(
        str_:          *const u8,
        n:             Option<i32>,
        is_terminated: Option<bool>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);
        let is_terminated: bool = is_terminated.unwrap_or(true);

        todo!();
        /*
        : string(str, n, isTerminated),

        
        */
    }
    
    pub fn new_from_stringobject(
        str_: &StringObject,
        n:    Option<i32>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);
        todo!();
        /*
        : string(str, n),

        
        */
    }
    
    pub fn new_from_string(
        str_: &String,
        n:    Option<i32>

    ) -> Self {

        let n: i32 = n.unwrap_or(-1);
        todo!();
        /*
        : string(str, n),

        
        */
    }
    
    pub fn new_from_fvariant(var: &FVariant) -> Self {
    
        todo!();
        /*
        : string(var),

        
        */
    }
    
    #[PLUGIN_API]
    pub fn set_text(&mut self, text: *const u8)  {
        
        todo!();
        /*
            assign (text);
        */
    }
    
    #[PLUGIN_API]
    pub fn set_text8(&mut self, text: *const u8)  {
        
        todo!();
        /*
            assign (text);
        */
    }
    
    #[PLUGIN_API]
    pub fn set_text16(&mut self, text: *const u16)  {
        
        todo!();
        /*
            assign (text);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_text8(&mut self) -> *const u8 {
        
        todo!();
        /*
            return text8 ();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_text16(&mut self) -> *const u16 {
        
        todo!();
        /*
            return text16 ();
        */
    }
    
    #[PLUGIN_API]
    pub fn take(&mut self, 
        s:       *mut c_void,
        is_wide: bool)  {
        
        todo!();
        /*
            String::take (s, _isWide);
        */
    }
    
    #[PLUGIN_API]
    pub fn is_wide_string(&self) -> bool {
        
        todo!();
        /*
            return String::isWideString ();
        */
    }
}

obj_methods!{
    StringObject, FObject
}

funknown_methods2!{
    IStringResult, IString, FObject
}

impl FUnknown for StringObject {

    fn query_interface(
        &mut self, 
        _: [i8; 16], 
        _: *mut *mut aloe_3p::c_void

    ) -> i32 
    { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { todo!() }
    fn release(&mut self) -> u32 { todo!() }
}

impl IStringResult for StringObject {

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn set_text(&mut self, text: *const u8)  {
        
        todo!();
        /*
        
        */
    }
}

impl IString for StringObject {

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn set_text8(&mut self, text: *const u8)  {
        
        todo!();
        /*
        
        */
    }

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn set_text16(&mut self, text: *const u16)  {
        
        todo!();
        /*
        
        */
    }

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn get_text8(&mut self) -> *const u8 {
        
        todo!();
        /*
        
        */
    }

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn get_text16(&mut self) -> *const u16 {
        
        todo!();
        /*
        
        */
    }

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn take(&mut self, 
        s:       *mut c_void,
        is_wide: bool)  {
        
        todo!();
        /*
        
        */
    }

    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn is_wide_string(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}
