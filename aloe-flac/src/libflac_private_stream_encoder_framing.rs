crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/stream_encoder_framing.h]

pub fn flac_add_metadata_block(
        metadata: *const StreamMetadata,
        bw:       *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_frame_add_header(
        header: *const FrameHeader,
        bw:     *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_subframe_add_constant(
        subframe:     *const Subframe_Constant,
        subframe_bps: u32,
        wasted_bits:  u32,
        bw:           *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_subframe_add_fixed(
        subframe:         *const Subframe_Fixed,
        residual_samples: u32,
        subframe_bps:     u32,
        wasted_bits:      u32,
        bw:               *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_subframe_add_lpc(
        subframe:         *const Subframe_LPC,
        residual_samples: u32,
        subframe_bps:     u32,
        wasted_bits:      u32,
        bw:               *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn flac_subframe_add_verbatim(
        subframe:     *const Subframe_Verbatim,
        samples:      u32,
        subframe_bps: u32,
        wasted_bits:  u32,
        bw:           *mut BitWriter) -> bool {
    
    todo!();
        /*
        
        */
}
