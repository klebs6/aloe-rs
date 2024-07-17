crate::ix!();

/**
  | @name Streams are byteOrder aware.
  |
  */
#[inline] pub fn fstreamer_set_byte_order(e: i32)  {
    
    todo!();
    /*
        byteOrder = (int16)e;
    */
}

#[inline] pub fn fstreamer_get_byte_order() -> i32 {
    
    todo!();
    /*
        return byteOrder;
    */
}

pub fn fstreamer_write_int8(c: i8) -> bool {
    
    todo!();
    /*
        return writeChar8 (c);
    */
}

pub fn fstreamer_read_int8(c: &mut i8) -> bool {
    
    todo!();
    /*
        return readChar8 (c);
    */
}

pub fn fstreamer_write_int8u(c: u8) -> bool {
    
    todo!();
    /*
        return writeUChar8 (c);
    */
}

pub fn fstreamer_read_int8u(c: &mut u8) -> bool {
    
    todo!();
    /*
        return readUChar8 (c);
    */
}
