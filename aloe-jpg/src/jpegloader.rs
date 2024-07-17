crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/aloe_JPEGLoader.cpp]

pub mod jpeg_helpers {

    use super::*;

    pub fn fatal_error_handler(p: JCommonPtr)  {
        
        todo!();
        /*
            *((bool*) (p->client_data)) = true;
        */
    }

    pub fn silent_error_callback1(_0: JCommonPtr)  { }

    pub fn silent_error_callback2( _0: JCommonPtr, _1: i32)  { }

    pub fn silent_error_callback3( _0: JCommonPtr, _1: *mut u8)  { }

    pub fn setup_silent_error_handler(err: &mut JpegErrorMgr)  {
        
        todo!();
        /*
            zerostruct (err);

            err.error_exit      = fatalErrorHandler;
            err.emit_message    = silentErrorCallback2;
            err.output_message  = silentErrorCallback1;
            err.format_message  = silentErrorCallback3;
            err.reset_error_mgr = silentErrorCallback1;
        */
    }

    #[cfg(not(ALOE_USING_COREIMAGE_LOADER))]
    pub fn dummy_callback1(_0: JDecompressPtr)  { }

    #[cfg(not(ALOE_USING_COREIMAGE_LOADER))]
    pub fn jpeg_skip(
            decomp_struct: JDecompressPtr,
            num:           i64)  {
        
        todo!();
            /*
                decompStruct->src->next_input_byte += num;

                num = jmin (num, (long) decompStruct->src->bytes_in_buffer);
                decompStruct->src->bytes_in_buffer -= (size_t) num;
            */
    }

    #[cfg(not(ALOE_USING_COREIMAGE_LOADER))]
    pub fn jpeg_fill(_0: JDecompressPtr) -> bool {
        
        todo!();
            /*
                return 0;
            */
    }

    pub const jpeg_buffer_size: i32 = 512;

    pub struct AloeJpegDest {
        base:   JpegDestinationMgr,
        output: *mut dyn Write,
        buffer: *mut u8,
    }

    pub fn jpeg_write_init(_0: JCompressPtr)  { }

    pub fn jpeg_write_terminate(cinfo: JCompressPtr)  {
        
        todo!();
            /*
                AloeJpegDest* const dest = static_cast<AloeJpegDest*> (cinfo->dest);

                const size_t numToWrite = jpegBufferSize - dest->free_in_buffer;
                dest->output->write (dest->buffer, numToWrite);
            */
    }

    pub fn jpeg_write_flush(cinfo: JCompressPtr) -> bool {
        
        todo!();
            /*
                AloeJpegDest* const dest = static_cast<AloeJpegDest*> (cinfo->dest);

                const int numToWrite = jpegBufferSize;

                dest->next_output_byte = reinterpret_cast<JOctet*> (dest->buffer);
                dest->free_in_buffer = jpegBufferSize;

                return (boolean) dest->output->write (dest->buffer, (size_t) numToWrite);
            */
    }
}


#[cfg(ALOE_USING_COREIMAGE_LOADER)]
pub fn aloe_load_with_core_image(input: &mut InputStream) -> Image {
    
    todo!();
        /*
        
        */
}
