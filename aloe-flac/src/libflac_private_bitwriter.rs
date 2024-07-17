crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/bitwriter.h]

/**
  | construction, deletion, initialization,
  | etc functions
  |
  */
pub fn flac_bitwriter_new() -> *mut BitWriter {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_delete(bw: *mut BitWriter)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_init(bw: *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | does not 'free(buffer)'
  |
  */
pub fn flac_bitwriter_free(bw: *mut BitWriter)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_clear(bw: *mut BitWriter)  {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_dump(
        bw:  *const BitWriter,
        out: *mut libc::FILE)  {
    
    todo!();
        /*
        
        */
}

/**
  | CRC functions non-const *bw because
  | they have to cal bitwriter_get_buffer()
  |
  */
pub fn flac_bitwriter_get_write_crc16(
        bw:  *mut BitWriter,
        crc: *mut u16) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_get_write_crc8(
        bw:  *mut BitWriter,
        crc: *mut u8) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | info functions
  |
  */
pub fn flac_bitwriter_is_byte_aligned(bw: *const BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | can be called anytime, returns total
  | # of bits unconsumed
  |
  */
pub fn flac_bitwriter_get_input_bits_unconsumed(bw: *const BitWriter) -> u32 {
    
    todo!();
        /*
        
        */
}

/**
  | direct buffer access there may be no
  | calls on the bitwriter between get and
  | release. the bitwriter continues to
  | own the returned buffer. before get,
  | bitwriter MUST be byte aligned: check
  | with bitwriter_is_byte_aligned()
  |
  */
pub fn flac_bitwriter_get_buffer(
        bw:     *mut BitWriter,
        buffer: *const *const u8,
        bytes:  *mut usize) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_release_buffer(bw: *mut BitWriter)  {
    
    todo!();
        /*
        
        */
}

/**
  | write functions
  |
  */
pub fn flac_bitwriter_write_zeroes(
        bw:   *mut BitWriter,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_raw_uint32(
        bw:   *mut BitWriter,
        val:  u32,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_raw_int32(
        bw:   *mut BitWriter,
        val:  i32,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_raw_uint64(
        bw:   *mut BitWriter,
        val:  u64,
        bits: u32) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | only for bits=32
  |
  */
pub fn flac_bitwriter_write_raw_uint32_little_endian(
        bw:  *mut BitWriter,
        val: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_byte_block(
        bw:    *mut BitWriter,
        vals:  &[u8],
        nvals: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_unary_unsigned(
        bw:  *mut BitWriter,
        val: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_rice_bits(
        val:       i32,
        parameter: u32) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(UNUSED)]
pub fn flac_bitwriter_golomb_bits_signed(
        val:       i32,
        parameter: u32) -> u32 {
    
    todo!();
        /*
        
        */
}

#[cfg(UNUSED)]
pub fn flac_bitwriter_golomb_bits_unsigned(
        val:       u32,
        parameter: u32) -> u32 {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_rice_signed(
        bw:        *mut BitWriter,
        val:       i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_rice_signed_block(
        bw:        *mut BitWriter,
        vals:      *const i32,
        nvals:     u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(UNUSED)]
pub fn flac_bitwriter_write_golomb_signed(
        bw:        *mut BitWriter,
        val:       i32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

#[cfg(UNUSED)]
pub fn flac_bitwriter_write_golomb_unsigned(
        bw:        *mut BitWriter,
        val:       u32,
        parameter: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_utf8_uint32(
        bw:  *mut BitWriter,
        val: u32) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_write_utf8_uint64(
        bw:  *mut BitWriter,
        val: u64) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_bitwriter_zero_pad_to_byte_boundary(bw: *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}
