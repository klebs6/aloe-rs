crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/crc.h]

/**
  | 8 bit CRC generator, MSB shifted first
  | polynomial = x^8 + x^2 + x^1 + x^0 * init
  | = 0
  |
  */
lazy_static!{
    /*
    extern byte const crc8_table[256];
    */
}

macro_rules! crc8_update {
    ($data:ident, $crc:ident) => {
        /*
                (crc) = crc8_table[(crc) ^ (data)];
        */
    }
}

pub fn flac_crc8_update(
        data: u8,
        crc:  *mut u8)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_crc8_update_block(
        data: *const u8,
        len:  u32,
        crc:  *mut u8)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_crc8(
        data: *const u8,
        len:  u32) -> u8 {
    
    todo!();
        /*
        
        */
}

/**
  | 16 bit CRC generator, MSB shifted first
  | polynomial = x^16 + x^15 + x^2 + x^0 init
  | = 0
  |
  */
lazy_static!{
    /*
    extern unsigned const crc16_table[256];
    */
}

macro_rules! crc16_update {
    ($data:ident, $crc:ident) => {
        /*
                ((((crc)<<8) & 0xffff) ^ crc16_table[((crc)>>8) ^ (data)])
        */
    }
}

/**
  | this alternate may be faster on some
  | systems/compilers
  |
  */
#[cfg(UNUSED)]
macro_rules! crc16_update {
    ($data:ident, $crc:ident) => {
        /*
                ((((crc)<<8) ^ crc16_table[((crc)>>8) ^ (data)]) & 0xffff)
        */
    }
}

pub fn flac_crc16(
        data: *const u8,
        len:  u32) -> u32 {
    
    todo!();
        /*
        
        */
}
