crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/bitreader.h]

pub type BitReaderReadCallback = fn(
    buffer:      &[u8],
    bytes:       *mut usize,
    client_data: *mut c_void
) -> bool;

/**
  | construction, deletion, initialization,
  | etc functions
  |
  */
pub fn flac_bitreader_new() -> *mut BitReader {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_delete(br: *mut BitReader)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_init(
        br:  *mut BitReader,
        rcb: BitReaderReadCallback,
        cd:  *mut c_void) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | does not 'free(br)'
  |
  */
pub fn flac_bitreader_free(br: *mut BitReader)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_clear(br: *mut BitReader) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_dump(
        br:  *const BitReader,
        out: *mut libc::FILE)  {
    
    todo!();
        /*
        
        */
}

/* ------------------ CRC functions  ------------------ */
pub fn flac_bitreader_reset_read_crc16(
        br:   *mut BitReader,
        seed: u16)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_get_read_crc16(br: *mut BitReader) -> u16 {
    
    todo!();
        /*
        
        */
}

/* ----------------- info functions  ----------------- */
pub fn flac_bitreader_is_consumed_byte_aligned(br: *const BitReader) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_bits_left_for_byte_alignment(br: *const BitReader) -> u32 {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_get_input_bits_unconsumed(br: *const BitReader) -> u32 {
    
    todo!();
        /*
        
        */
}

/* ----------------- read functions  ----------------- */

pub fn flac_bitreader_read_raw_uint32(
        br:   *mut BitReader,
        val:  *mut u32,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_raw_int32(
        br:   *mut BitReader,
        val:  *mut i32,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_raw_uint64(
        br:   *mut BitReader,
        val:  *mut u64,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | only for bits=32
  |
  */
pub fn flac_bitreader_read_uint32_little_endian(
        br:  *mut BitReader,
        val: *mut u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | WATCHOUT: does not CRC the skipped data!
  | @@@@ add to unit tests
  |
  */
pub fn flac_bitreader_skip_bits_no_crc(
        br:   *mut BitReader,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | WATCHOUT: does not CRC the read data!
  |
  */
pub fn flac_bitreader_skip_byte_block_aligned_no_crc(
        br:    *mut BitReader,
        nvals: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | WATCHOUT: does not CRC the read data!
  |
  */
pub fn flac_bitreader_read_byte_block_aligned_no_crc(
        br:    *mut BitReader,
        val:   *mut u8,
        nvals: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_unary_unsigned(
        br:  *mut BitReader,
        val: *mut u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_rice_signed(
        br:        *mut BitReader,
        val:       *mut i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_rice_signed_block(
        br:        *mut BitReader,
        vals:      &[i32],
        nvals:     u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(UNUSED)]
pub fn flac_bitreader_read_golomb_signed(
        br:        *mut BitReader,
        val:       *mut i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(UNUSED)]
pub fn flac_bitreader_read_golomb_unsigned(
        br:        *mut BitReader,
        val:       *mut u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_utf8_uint32(
        br:     *mut BitReader,
        val:    *mut u32,
        raw:    *mut u8,
        rawlen: *mut u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitreader_read_utf8_uint64(
        br:     *mut BitReader,
        val:    *mut u64,
        raw:    *mut u8,
        rawlen: *mut u32) -> bool {
    
    todo!();
        /*
        
        */
}
