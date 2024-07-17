crate::ix!();

pub struct StringRegion {
    text:   CharPointerType,
    start:  i32,
    length: i32,
}

impl StringRegion {

    pub fn new(s: &String) -> Self {
    
        todo!();
        /*
            : text (s.getCharPointer()), start (0), length (s.length())
        */
    }
    
    pub fn new_from_ptr_len(
        t:   CharPointerType,
        s:   i32,
        len: i32) -> Self {
    
        todo!();
        /*


            : text (t), start (s), length (len)
        */
    }
    
    pub fn increment_start(&mut self)  {
        
        todo!();
        /*
            ++text; ++start; --length;
        */
    }
}

