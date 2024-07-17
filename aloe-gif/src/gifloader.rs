crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/aloe_GIFLoader.cpp]

#[cfg(all(any(target_os="macos",target_os="ios"),all(USE_COREGRAPHICS_RENDERING,ALOE_USE_COREIMAGE_LOADER)))]
pub fn aloe_load_with_core_image(input: &mut InputStream) -> Image {
    
    todo!();
        /*
        
        */
}

#[cfg(not(all(any(target_os="macos",target_os="ios"),all(USE_COREGRAPHICS_RENDERING,ALOE_USE_COREIMAGE_LOADER))))]
#[no_copy]
pub struct GIFLoader<'a> {
    image:              Image,
    input:              &'a mut dyn Read,
    buffer:             [u8; 260],
    palette:            [PixelARGB; 256],
    data_block_is_zero: bool,
    fresh:              bool,
    finished:           bool,
    current_bit:        i32,
    last_bit:           i32,
    last_byte_index:    i32,
    code_size:          i32,
    set_code_size:      i32,
    max_code:           i32,
    max_code_size:      i32,
    firstcode:          i32,
    oldcode:            i32,
    clear_code:         i32,
    end_code:           i32,
    table:              [[i32; 2]; GIF_LOADER_MAX_GIF_CODE],
    stack:              [i32; 2 * GIF_LOADER_MAX_GIF_CODE],
    sp:                 *mut i32,
}

#[cfg(not(all(any(target_os="macos",target_os="ios"),all(USE_COREGRAPHICS_RENDERING,ALOE_USE_COREIMAGE_LOADER))))]
pub const GIF_LOADER_MAX_GIF_CODE: usize = 1 << 12;

#[cfg(not(all(any(target_os="macos",target_os="ios"),all(USE_COREGRAPHICS_RENDERING,ALOE_USE_COREIMAGE_LOADER))))]
impl<'a> GIFLoader<'a> {

    pub fn new(in_: &mut dyn Read) -> Self {
    
        todo!();
        /*
        : input(in),
        : data_block_is_zero(false),
        : fresh(false),
        : finished(false),
        : current_bit(0),
        : last_bit(0),
        : last_byte_index(0),
        : code_size(0),
        : set_code_size(0),
        : max_code(0),
        : max_code_size(0),
        : firstcode(0),
        : oldcode(0),
        : clear_code(0),
        : end_code(0),

            int imageWidth, imageHeight;
            if (! getSizeFromHeader (imageWidth, imageHeight))
                return;

            uint8 buf [16];
            if (in.read (buf, 3) != 3)
                return;

            int numColours = 2 << (buf[0] & 7);
            int transparent = -1;

            if ((buf[0] & 0x80) != 0)
                readPalette (numColours);

            for (;;)
            {
                if (input.read (buf, 1) != 1 || buf[0] == ';')
                    break;

                if (buf[0] == '!')
                {
                    if (readExtension (transparent))
                        continue;

                    break;
                }

                if (buf[0] != ',')
                    continue;

                if (input.read (buf, 9) == 9)
                {
                    imageWidth  = (int) ByteOrder::littleEndianShort (buf + 4);
                    imageHeight = (int) ByteOrder::littleEndianShort (buf + 6);

                    numColours = 2 << (buf[8] & 7);

                    if ((buf[8] & 0x80) != 0)
                        if (! readPalette (numColours))
                            break;

                    image = Image (transparent >= 0 ? Image::ARGB : Image::RGB,
                                   imageWidth, imageHeight, transparent >= 0);

                    image.getProperties()->set ("originalImageHadAlpha", transparent >= 0);

                    readImage ((buf[8] & 0x40) != 0, transparent);
                }

                break;
            }
        */
    }
    
    pub fn get_size_from_header(&mut self, 
        w: &mut i32,
        h: &mut i32) -> bool {
        
        todo!();
        /*
            // Add an extra byte for the zero terminator
            char b[7]{};

            if (input.read (b, 6) == 6
                 && (strncmp ("GIF87a", b, 6) == 0
                      || strncmp ("GIF89a", b, 6) == 0))
            {
                if (input.read (b, 4) == 4)
                {
                    w = (int) ByteOrder::littleEndianShort (b);
                    h = (int) ByteOrder::littleEndianShort (b + 2);
                    return w > 0 && h > 0;
                }
            }

            return false;
        */
    }
    
    pub fn read_palette(&mut self, num_cols: i32) -> bool {
        
        todo!();
        /*
            for (int i = 0; i < numCols; ++i)
            {
                uint8 rgb[4];
                input.read (rgb, 3);

                palette[i].setARGB (0xff, rgb[0], rgb[1], rgb[2]);
                palette[i].premultiply();
            }

            return true;
        */
    }
    
    pub fn read_data_block(&mut self, dest: *mut u8) -> i32 {
        
        todo!();
        /*
            uint8 n;
            if (input.read (&n, 1) == 1)
            {
                dataBlockIsZero = (n == 0);

                if (dataBlockIsZero || (input.read (dest, n) == n))
                    return n;
            }

            return -1;
        */
    }
    
    pub fn read_extension(&mut self, transparent: &mut i32) -> i32 {
        
        todo!();
        /*
            uint8 type;
            if (input.read (&type, 1) != 1)
                return false;

            uint8 b [260];
            int n = 0;

            if (type == 0xf9)
            {
                n = readDataBlock (b);
                if (n < 0)
                    return 1;

                if ((b[0] & 1) != 0)
                    transparent = b[3];
            }

            do
            {
                n = readDataBlock (b);
            }
            while (n > 0);

            return n >= 0;
        */
    }
    
    pub fn clear_table(&mut self)  {
        
        todo!();
        /*
            int i;
            for (i = 0; i < clearCode; ++i)
            {
                table[0][i] = 0;
                table[1][i] = i;
            }

            for (; i < maxGifCode; ++i)
            {
                table[0][i] = 0;
                table[1][i] = 0;
            }
        */
    }
    
    pub fn initialise(&mut self, input_code_size: i32)  {
        
        todo!();
        /*
            setCodeSize = inputCodeSize;
            codeSize = setCodeSize + 1;
            clearCode = 1 << setCodeSize;
            endCode = clearCode + 1;
            maxCodeSize = 2 * clearCode;
            maxCode = clearCode + 2;

            getCode (0, true);

            fresh = true;
            clearTable();
            sp = stack;
        */
    }
    
    pub fn read_lzw_byte(&mut self) -> i32 {
        
        todo!();
        /*
            if (fresh)
            {
                fresh = false;

                for (;;)
                {
                    firstcode = oldcode = getCode (codeSize, false);

                    if (firstcode != clearCode)
                        return firstcode;
                }
            }

            if (sp > stack)
                return *--sp;

            int code;

            while ((code = getCode (codeSize, false)) >= 0)
            {
                if (code == clearCode)
                {
                    clearTable();
                    codeSize = setCodeSize + 1;
                    maxCodeSize = 2 * clearCode;
                    maxCode = clearCode + 2;
                    sp = stack;
                    firstcode = oldcode = getCode (codeSize, false);
                    return firstcode;
                }
                else if (code == endCode)
                {
                    if (dataBlockIsZero)
                        return -2;

                    uint8 buf [260];
                    int n;

                    while ((n = readDataBlock (buf)) > 0)
                    {}

                    if (n != 0)
                        return -2;
                }

                const int incode = code;

                if (code >= maxCode)
                {
                    *sp++ = firstcode;
                    code = oldcode;
                }

                while (code >= clearCode)
                {
                    *sp++ = table[1][code];
                    if (code == table[0][code])
                        return -2;

                    code = table[0][code];
                }

                *sp++ = firstcode = table[1][code];

                if ((code = maxCode) < maxGifCode)
                {
                    table[0][code] = oldcode;
                    table[1][code] = firstcode;
                    ++maxCode;

                    if (maxCode >= maxCodeSize && maxCodeSize < maxGifCode)
                    {
                        maxCodeSize <<= 1;
                        ++codeSize;
                    }
                }

                oldcode = incode;

                if (sp > stack)
                    return *--sp;
            }

            return code;
        */
    }
    
    pub fn get_code(&mut self, 
        code_size:         i32,
        should_initialise: bool) -> i32 {
        
        todo!();
        /*
            if (shouldInitialise)
            {
                currentBit = 0;
                lastBit = 0;
                finished = false;
                return 0;
            }

            if ((currentBit + codeSize_) >= lastBit)
            {
                if (finished)
                    return -1;

                buffer[0] = buffer [lastByteIndex - 2];
                buffer[1] = buffer [lastByteIndex - 1];

                const int n = readDataBlock (buffer + 2);

                if (n == 0)
                    finished = true;

                lastByteIndex = 2 + n;
                currentBit = (currentBit - lastBit) + 16;
                lastBit = (2 + n) * 8 ;
            }

            int result = 0;
            int i = currentBit;

            for (int j = 0; j < codeSize_; ++j)
            {
                result |= ((buffer[i >> 3] & (1 << (i & 7))) != 0) << j;
                ++i;
            }

            currentBit += codeSize_;
            return result;
        */
    }
    
    pub fn read_image(&mut self, 
        interlace:   i32,
        transparent: i32) -> bool {
        
        todo!();
        /*
            uint8 c;
            if (input.read (&c, 1) != 1)
                return false;

            initialise (c);

            if (transparent >= 0)
                palette [transparent].setARGB (0, 0, 0, 0);

            int xpos = 0, ypos = 0, yStep = 8, pass = 0;

            const ImageBitmapData destData (image, ImageBitmapData::writeOnly);
            uint8* p = destData.getPixelPointer (0, 0);
            const bool hasAlpha = image.hasAlphaChannel();

            for (;;)
            {
                const int index = readLZWByte();
                if (index < 0)
                    break;

                if (hasAlpha)
                    ((PixelARGB*) p)->set (palette [index]);
                else
                    ((PixelRGB*)  p)->set (palette [index]);

                p += destData.pixelStride;

                if (++xpos == destData.width)
                {
                    xpos = 0;

                    if (interlace)
                    {
                        ypos += yStep;

                        while (ypos >= destData.height)
                        {
                            switch (++pass)
                            {
                                case 1:     ypos = 4; yStep = 8; break;
                                case 2:     ypos = 2; yStep = 4; break;
                                case 3:     ypos = 1; yStep = 2; break;
                                default:    return true;
                            }
                        }
                    }
                    else
                    {
                        if (++ypos >= destData.height)
                            break;
                    }

                    p = destData.getPixelPointer (xpos, ypos);
                }
            }

            return true;
        */
    }
}
