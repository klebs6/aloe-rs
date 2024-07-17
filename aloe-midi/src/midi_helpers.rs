crate::ix!();

#[inline] pub fn midi_helpers_initial_byte(
        ty:      i32,
        channel: i32) -> u8 {
    
    todo!();
    /*
        return (uint8) (type | jlimit (0, 15, channel - 1));
    */
}

#[inline] pub fn midi_helpers_valid_velocity(v: i32) -> u8 {
    
    todo!();
    /*
        return (uint8) jlimit (0, 127, v);
    */
}
