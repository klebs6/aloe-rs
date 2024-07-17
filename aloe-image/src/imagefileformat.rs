crate::ix!();

pub trait GetFormatName {

    /**
      | Returns a description of this file format.
      | 
      | E.g. "JPEG", "PNG"
      |
      */
    fn get_format_name(&mut self) -> String;
}

pub trait CanUnderstand {

    /**
      | Returns true if the given stream seems
      | to contain data that this format understands.
      | 
      | The format class should only read the
      | first few bytes of the stream and sniff
      | for header bytes that it understands.
      | 
      | -----------
      | @note
      | 
      | this will advance the stream and leave
      | it in a new position, so if you're planning
      | on re-using it, you may want to rewind
      | it after calling this method.
      | 
      | @see decodeImage
      |
      */
    fn can_understand<R: Read>(&mut self, input: &mut R) -> bool;
}

pub trait UsesFileExtension {

    /**
      | Returns true if this format uses the
      | file extension of the given file.
      |
      */
    fn uses_file_extension(&mut self, possible_file: &File) -> bool;
}

pub trait DecodeImage {

    /**
      | Tries to decode and return an image from
      | the given stream.
      | 
      | This will be called for an image format
      | after calling its canUnderStand()
      | method to see if it can handle the stream.
      | 
      | -----------
      | @param input
      | 
      | the stream to read the data from. The
      | stream will be positioned at the start
      | of the image data (but this may not necessarily
      | be position 0)
      | 
      | -----------
      | @return
      | 
      | the image that was decoded, or an invalid
      | image if it fails. @see loadFrom
      |
      */
    fn decode_image<R: Read>(&mut self, input: &mut R) -> Image;
}

pub trait WriteImageToStream {

    /**
      | Attempts to write an image to a stream.
      | 
      | To specify extra information like encoding
      | quality, there will be appropriate
      | parameters in the subclasses of the
      | specific file types.
      | 
      | 
      | -----------
      | @return
      | 
      | true if it nothing went wrong.
      |
      */
    fn write_image_to_stream<W: Write>(
        &mut self, 
        source_image: &Image,
        dest_stream:  &mut W) -> bool;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_ImageFileFormat.h]

/**
  | Base-class for codecs that can read
  | and write image file formats such as
  | PNG, JPEG, etc.
  | 
  | This class also contains static methods
  | to make it easy to load images from files,
  | streams or from memory.
  | 
  | @see Image, ImageCache
  | 
  | @tags{Graphics}
  |
  */
#[derive(Default)]
pub struct ImageFileFormat {


}

pub trait ImageFileFormatInterface:
    GetFormatName
    + CanUnderstand
    + UsesFileExtension
    + DecodeImage
    + WriteImageToStream {}

/**
  | A subclass of ImageFileFormat for reading
  | and writing PNG files.
  | 
  | @see ImageFileFormat, JPEGImageFormat
  | 
  | @tags{Graphics}
  |
  */
pub struct PNGImageFormat {
    base: ImageFileFormat,
}

/**
  | A subclass of ImageFileFormat for reading
  | and writing JPEG files.
  | 
  | @see ImageFileFormat, PNGImageFormat
  | 
  | @tags{Graphics}
  |
  */
pub struct JPEGImageFormat {
    base:    ImageFileFormat,
    quality: f32,
}

impl Default for JPEGImageFormat {

    fn default() -> Self {
    
        todo!();
        /*
        : quality(-1.0f),
        */
    }
}

impl JPEGImageFormat {

    /**
      | Specifies the quality to be used when
      | writing a JPEG file.
      | 
      | -----------
      | @param newQuality
      | 
      | a value 0 to 1.0, where 0 is low quality,
      | 1.0 is best, or any negative value is
      | "default" quality
      |
      */
    pub fn set_quality(&mut self, new_quality: f32)  {
        
        todo!();
        /*
            quality = newQuality;
        */
    }
    
    pub fn get_format_name(&mut self) -> String {
        
        todo!();
        /*
            return "JPEG";
        */
    }
    
    pub fn uses_file_extension(&mut self, f: &File) -> bool {
        
        todo!();
        /*
            return f.hasFileExtension ("jpeg;jpg");
        */
    }
    
    pub fn can_understand(&mut self, in_: &mut dyn Read) -> bool {
        
        todo!();
        /*
            const int bytesNeeded = 24;
        uint8 header [bytesNeeded];

        if (in.read (header, bytesNeeded) == bytesNeeded
                && header[0] == 0xff
                && header[1] == 0xd8
                && header[2] == 0xff)
            return true;

       #if ALOE_USING_COREIMAGE_LOADER
        return header[20] == 'j'
            && header[21] == 'p'
            && header[22] == '2'
            && header[23] == ' ';
       #endif

        return false;
        */
    }
    
    pub fn decode_image(&mut self, in_: &mut dyn Read) -> Image {
        
        todo!();
        /*
            #if ALOE_USING_COREIMAGE_LOADER
        return aloe_loadWithCoreImage (in);
       #else
        using namespace jpeglibNamespace;
        using namespace JPEGHelpers;

        MemoryOutputStream mb;
        mb << in;

        Image image;

        if (mb.getDataSize() > 16)
        {
            struct jpeg_decompress_struct jpegDecompStruct;

            struct jpeg_error_mgr jerr;
            setupSilentErrorHandler (jerr);
            jpegDecompStruct.err = &jerr;

            jpeg_create_decompress (&jpegDecompStruct);

            jpegDecompStruct.src = (jpeg_source_mgr*)(jpegDecompStruct.mem->alloc_small)
                ((JCommonPtr)(&jpegDecompStruct), JPOOL_PERMANENT, sizeof (jpeg_source_mgr));

            bool hasFailed = false;
            jpegDecompStruct.client_data = &hasFailed;

            jpegDecompStruct.src->init_source       = dummyCallback1;
            jpegDecompStruct.src->fill_input_buffer = jpegFill;
            jpegDecompStruct.src->skip_input_data   = jpegSkip;
            jpegDecompStruct.src->resync_to_restart = jpeg_resync_to_restart;
            jpegDecompStruct.src->term_source       = dummyCallback1;

            jpegDecompStruct.src->next_input_byte   = static_cast<const unsigned char*> (mb.getData());
            jpegDecompStruct.src->bytes_in_buffer   = mb.getDataSize();

            jpeg_read_header (&jpegDecompStruct, TRUE);

            if (! hasFailed)
            {
                jpeg_calc_output_dimensions (&jpegDecompStruct);

                if (! hasFailed)
                {
                    const int width  = (int) jpegDecompStruct.output_width;
                    const int height = (int) jpegDecompStruct.output_height;

                    jpegDecompStruct.out_color_space = JCS_RGB;

                    JSAMPARRAY buffer
                        = (*jpegDecompStruct.mem->alloc_sarray) ((JCommonPtr) &jpegDecompStruct,
                                                                 JPOOL_IMAGE,
                                                                 (JDimension) width * 3, 1);

                    if (jpeg_start_decompress (&jpegDecompStruct) && ! hasFailed)
                    {
                        image = Image (Image::RGB, width, height, false);
                        image.getProperties()->set ("originalImageHadAlpha", false);
                        const bool hasAlphaChan = image.hasAlphaChannel(); // (the native image creator may not give back what we expect)

                        const ImageBitmapData destData (image, ImageBitmapData::writeOnly);

                        for (int y = 0; y < height; ++y)
                        {
                            jpeg_read_scanlines (&jpegDecompStruct, buffer, 1);

                            if (hasFailed)
                                break;

                            const uint8* src = *buffer;
                            uint8* dest = destData.getLinePointer (y);

                            if (hasAlphaChan)
                            {
                                for (int i = width; --i >= 0;)
                                {
                                    ((PixelARGB*) dest)->setARGB (0xff, src[0], src[1], src[2]);
                                    ((PixelARGB*) dest)->premultiply();
                                    dest += destData.pixelStride;
                                    src += 3;
                                }
                            }
                            else
                            {
                                for (int i = width; --i >= 0;)
                                {
                                    ((PixelRGB*) dest)->setARGB (0xff, src[0], src[1], src[2]);
                                    dest += destData.pixelStride;
                                    src += 3;
                                }
                            }
                        }

                        if (! hasFailed)
                            jpeg_finish_decompress (&jpegDecompStruct);

                        in.setPosition (((char*) jpegDecompStruct.src->next_input_byte) - (char*) mb.getData());
                    }
                }
            }

            jpeg_destroy_decompress (&jpegDecompStruct);
        }

        return image;
       #endif
        */
    }
    
    pub fn write_image_to_stream(
        &mut self, 
        image: &Image,
        out:   &mut dyn Write

    ) -> bool {
        
        todo!();
        /*
            using namespace jpeglibNamespace;
        using namespace JPEGHelpers;

        jpeg_compress_struct jpegCompStruct;
        zerostruct (jpegCompStruct);
        jpeg_create_compress (&jpegCompStruct);

        struct jpeg_error_mgr jerr;
        setupSilentErrorHandler (jerr);
        jpegCompStruct.err = &jerr;

        AloeJpegDest dest;
        jpegCompStruct.dest = &dest;

        dest.output = &out;
        HeapBlock<char> tempBuffer (jpegBufferSize);
        dest.buffer = tempBuffer;
        dest.next_output_byte = (JOctet*) dest.buffer;
        dest.free_in_buffer = jpegBufferSize;
        dest.init_destination = jpegWriteInit;
        dest.empty_output_buffer = jpegWriteFlush;
        dest.term_destination = jpegWriteTerminate;

        jpegCompStruct.image_width  = (JDimension) image.getWidth();
        jpegCompStruct.image_height = (JDimension) image.getHeight();
        jpegCompStruct.input_components = 3;
        jpegCompStruct.in_color_space = JCS_RGB;
        jpegCompStruct.write_JFIF_header = 1;

        jpegCompStruct.X_density = 72;
        jpegCompStruct.Y_density = 72;

        jpeg_set_defaults (&jpegCompStruct);

        jpegCompStruct.dct_method = JDCT_FLOAT;
        jpegCompStruct.optimize_coding = 1;

        if (quality < 0.0f)
            quality = 0.85f;

        jpeg_set_quality (&jpegCompStruct, jlimit (0, 100, roundToInt (quality * 100.0f)), TRUE);

        jpeg_start_compress (&jpegCompStruct, TRUE);

        const int strideBytes = (int) (jpegCompStruct.image_width * (unsigned int) jpegCompStruct.input_components);

        JSAMPARRAY buffer = (*jpegCompStruct.mem->alloc_sarray) ((JCommonPtr) &jpegCompStruct,
                                                                 JPOOL_IMAGE, (JDimension) strideBytes, 1);

        const ImageBitmapData srcData (image, ImageBitmapData::readOnly);

        while (jpegCompStruct.next_scanline < jpegCompStruct.image_height)
        {
            uint8* dst = *buffer;

            if (srcData.pixelFormat == Image::RGB)
            {
                const uint8* src = srcData.getLinePointer ((int) jpegCompStruct.next_scanline);

                for (int i = srcData.width; --i >= 0;)
                {
                    *dst++ = ((const PixelRGB*) src)->getRed();
                    *dst++ = ((const PixelRGB*) src)->getGreen();
                    *dst++ = ((const PixelRGB*) src)->getBlue();
                    src += srcData.pixelStride;
                }
            }
            else
            {
                for (int x = 0; x < srcData.width; ++x)
                {
                    const Colour pixel (srcData.getPixelColour (x, (int) jpegCompStruct.next_scanline));
                    *dst++ = pixel.getRed();
                    *dst++ = pixel.getGreen();
                    *dst++ = pixel.getBlue();
                }
            }

            jpeg_write_scanlines (&jpegCompStruct, buffer, 1);
        }

        jpeg_finish_compress (&jpegCompStruct);
        jpeg_destroy_compress (&jpegCompStruct);

        return true;
        */
    }
}


/**
  | A subclass of ImageFileFormat for reading
  | GIF files.
  | 
  | @see ImageFileFormat, PNGImageFormat,
  | JPEGImageFormat
  | 
  | @tags{Graphics}
  |
  */
pub struct GIFImageFormat {
    base: ImageFileFormat,
}

impl GIFImageFormat {

    pub fn get_format_name(&mut self) -> String {
        
        todo!();
        /*
            return "GIF";
        */
    }
    
    pub fn uses_file_extension(&mut self, f: &File) -> bool {
        
        todo!();
        /*
            return f.hasFileExtension ("gif");
        */
    }
    
    pub fn can_understand(&mut self, in_: &mut dyn Read) -> bool {
        
        todo!();
        /*
            char header [4];

        return (in.read (header, sizeof (header)) == (int) sizeof (header))
                 && header[0] == 'G'
                 && header[1] == 'I'
                 && header[2] == 'F';
        */
    }
    
    pub fn decode_image(&mut self, in_: &mut dyn Read) -> Image {
        
        todo!();
        /*
            #if (ALOE_MAC || ALOE_IOS) && USE_COREGRAPHICS_RENDERING && ALOE_USE_COREIMAGE_LOADER
        return aloe_loadWithCoreImage (in);
       #else
        const std::unique_ptr<GIFLoader> loader (new GIFLoader (in));
        return loader->image;
       #endif
        */
    }
    
    pub fn write_image_to_stream(
        &mut self, 
        source_image: &Image,
        dest_stream:  &mut dyn Write

    ) -> bool {
        
        todo!();
        /*
            jassertfalse; // writing isn't implemented for GIFs!
        return false;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_ImageFileFormat.cpp]
pub struct DefaultImageFormats {
    png:     PNGImageFormat,
    jpg:     JPEGImageFormat,
    gif:     GIFImageFormat,
    formats: [*mut ImageFileFormat; 4],
}

impl Default for DefaultImageFormats {
    
    fn default() -> Self {
        todo!();
        /*


            formats[0] = &png;
            formats[1] = &jpg;
            formats[2] = &gif;
            formats[3] = nullptr;
        */
    }
}

impl DefaultImageFormats {

    pub fn get() -> *mut *mut ImageFileFormat {
        
        todo!();
        /*
            static DefaultImageFormats formats;
            return formats.formats;
        */
    }
}

impl ImageFileFormat {

    /**
      | Tries the built-in formats to see if
      | it can find one to read this stream.
      | 
      | There are currently built-in decoders
      | for PNG, JPEG and GIF formats.
      | 
      | The object that is returned should not
      | be deleted by the caller. @see canUnderstand,
      | decodeImage, loadFrom
      |
      */
    pub fn find_image_format_for_stream<R: Read>(&mut self, input: &mut R) -> *mut ImageFileFormat {
        
        todo!();
        /*
            const int64 streamPos = input.getPosition();

        for (ImageFileFormat** i = DefaultImageFormats::get(); *i != nullptr; ++i)
        {
            const bool found = (*i)->canUnderstand (input);
            input.setPosition (streamPos);

            if (found)
                return *i;
        }

        return nullptr;
        */
    }
    
    /**
      | Looks for a format that can handle the
      | given file extension.
      | 
      | There are currently built-in formats
      | for PNG, JPEG and GIF formats.
      | 
      | The object that is returned should not
      | be deleted by the caller.
      |
      */
    pub fn find_image_format_for_file_extension(&mut self, file: &File) -> *mut ImageFileFormat {
        
        todo!();
        /*
            for (ImageFileFormat** i = DefaultImageFormats::get(); *i != nullptr; ++i)
            if ((*i)->usesFileExtension (file))
                return *i;

        return nullptr;
        */
    }
    
    /**
      | Tries to load an image from a stream.
      | 
      | This will use the findImageFormatForStream()
      | method to locate a suitable codec, and
      | use that to load the image.
      | 
      | 
      | -----------
      | @return
      | 
      | the image that was decoded, or an invalid
      | image if it fails.
      |
      */
    pub fn load_from_reader<R: Read>(&mut self, input: &mut R) -> Image {
        
        todo!();
        /*
            if (ImageFileFormat* format = findImageFormatForStream (input))
            return format->decodeImage (input);

        return Image();
        */
    }
    
    /**
      | Tries to load an image from a file.
      | 
      | This will use the findImageFormatForStream()
      | method to locate a suitable codec, and
      | use that to load the image.
      | 
      | 
      | -----------
      | @return
      | 
      | the image that was decoded, or an invalid
      | image if it fails.
      |
      */
    pub fn load_from_file(&mut self, file: &File) -> Image {
        
        todo!();
        /*
            FileInputStream stream (file);

        if (stream.openedOk())
        {
            BufferedInputStream b (stream, 8192);
            return loadFrom (b);
        }

        return Image();
        */
    }
    
    /**
      | Tries to load an image from a block of
      | raw image data.
      | 
      | This will use the findImageFormatForStream()
      | method to locate a suitable codec, and
      | use that to load the image.
      | 
      | 
      | -----------
      | @return
      | 
      | the image that was decoded, or an invalid
      | image if it fails.
      |
      */
    pub fn load_from_raw(
        &mut self, 
        raw_data:  *const c_void,
        num_bytes: usize) -> Image {
        
        todo!();
        /*
            if (rawData != nullptr && numBytes > 4)
        {
            MemoryInputStream stream (rawData, numBytes, false);
            return loadFrom (stream);
        }

        return Image();
        */
    }
}
