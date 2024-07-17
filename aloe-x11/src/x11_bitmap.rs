crate::ix!();

///================================= X11 - Bitmap ===============================
#[no_copy]
#[leak_detector]
pub struct XBitmapImage {
    base:                 ImagePixelData,
    x_image:              *mut XImage, // default = nullptr
    image_depth:          u32,
    image_data_allocated: HeapBlock<u8>,
    image_data_16bit:     HeapBlock<u8>,
    pixel_stride:         i32,
    line_stride:          i32,
    image_data:           *mut u8, // default = nullptr
    gc:                   GC, // default = None
    display:              *mut Display, //= XWindowSystem::getInstance()->getDisplay();

    #[cfg(ALOE_USE_XSHM)]
    segment_info:         XShmSegmentInfo,

    using_xshm:           bool,
}

impl Drop for XBitmapImage {

    fn drop(&mut self) {
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

            if (gc != None)
                X11Symbols::getInstance()->xFreeGC (display, gc);

           #if ALOE_USE_XSHM
            if (isUsingXShm())
            {
                X11Symbols::getInstance()->xShmDetach (display, &segmentInfo);

                X11Symbols::getInstance()->xFlush (display);
                X11Symbols::getInstance()->xDestroyImage (xImage);

                shmdt (segmentInfo.shmaddr);
                shmctl (segmentInfo.shmid, IPC_RMID, nullptr);
            }
            else
           #endif
            {
                xImage->data = nullptr;
                X11Symbols::getInstance()->xDestroyImage (xImage);
            }
        */
    }
}

impl XBitmapImage {

    pub fn new(
        format:      ImagePixelFormat,
        w:           i32,
        h:           i32,
        clear_image: bool,
        image_depth: u32,
        visual:      *mut Visual) -> Self {
    
        todo!();
        /*
        : image_pixel_data(format, w, h),
        : image_depth(imageDepth_),

            jassert (format == Image::RGB || format == Image::ARGB);

            pixelStride = (format == Image::RGB) ? 3 : 4;
            lineStride = ((w * pixelStride + 3) & ~3);

            XWindowSystemUtilities::ScopedXLock xLock;

           #if ALOE_USE_XSHM
            usingXShm = false;

            if ((imageDepth > 16) && XSHMHelpers::isShmAvailable (display))
            {
                zerostruct (segmentInfo);

                segmentInfo.shmid = -1;
                segmentInfo.shmaddr = (char *) -1;
                segmentInfo.readOnly = False;

                xImage = X11Symbols::getInstance()->xShmCreateImage (display, visual, imageDepth, ZPixmap, nullptr,
                                                                     &segmentInfo, (unsigned int) w, (unsigned int) h);

                if (xImage != nullptr)
                {
                    if ((segmentInfo.shmid = shmget (IPC_PRIVATE,
                                                     (size_t) (xImage->bytes_per_line * xImage->height),
                                                     IPC_CREAT | 0777)) >= 0)
                    {
                        if (segmentInfo.shmid != -1)
                        {
                            segmentInfo.shmaddr = (char*) shmat (segmentInfo.shmid, nullptr, 0);

                            if (segmentInfo.shmaddr != (void*) -1)
                            {
                                segmentInfo.readOnly = False;

                                xImage->data = segmentInfo.shmaddr;
                                imageData = (uint8*) segmentInfo.shmaddr;

                                if (X11Symbols::getInstance()->xShmAttach (display, &segmentInfo) != 0)
                                    usingXShm = true;
                                else
                                    jassertfalse;
                            }
                            else
                            {
                                shmctl (segmentInfo.shmid, IPC_RMID, nullptr);
                            }
                        }
                    }
                }
            }

            if (! isUsingXShm())
           #endif
            {
                imageDataAllocated.allocate ((size_t) (lineStride * h), format == Image::ARGB && clearImage);
                imageData = imageDataAllocated;

                xImage = (XImage*) ::calloc (1, sizeof (XImage));

                xImage->width = w;
                xImage->height = h;
                xImage->xoffset = 0;
                xImage->format = ZPixmap;
                xImage->data = (char*) imageData;
                xImage->byte_order = X11Symbols::getInstance()->xImageByteOrder (display);
                xImage->bitmap_unit = X11Symbols::getInstance()->xBitmapUnit (display);
                xImage->bitmap_bit_order = X11Symbols::getInstance()->xBitmapBitOrder (display);
                xImage->bitmap_pad = 32;
                xImage->depth = pixelStride * 8;
                xImage->bytes_per_line = lineStride;
                xImage->bits_per_pixel = pixelStride * 8;
                xImage->red_mask   = 0x00FF0000;
                xImage->green_mask = 0x0000FF00;
                xImage->blue_mask  = 0x000000FF;

                if (imageDepth == 16)
                {
                    int pixStride = 2;
                    auto stride = ((w * pixStride + 3) & ~3);

                    imageData16Bit.malloc (stride * h);
                    xImage->data = imageData16Bit;
                    xImage->bitmap_pad = 16;
                    xImage->depth = pixStride * 8;
                    xImage->bytes_per_line = stride;
                    xImage->bits_per_pixel = pixStride * 8;
                    xImage->red_mask   = visual->red_mask;
                    xImage->green_mask = visual->green_mask;
                    xImage->blue_mask  = visual->blue_mask;
                }

                if (! X11Symbols::getInstance()->xInitImage (xImage))
                    jassertfalse;
            }
        */
    }
    
    pub fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            sendDataChangeMessage();
            return std::make_unique<LowLevelGraphicsSoftwareRenderer> (Image (this));
        */
    }
    
    pub fn initialise_bitmap_data(&mut self, 
        bitmap: &mut ImageBitmapData,
        x:      i32,
        y:      i32,
        mode:   ImageBitmapDataReadWriteMode)  {
        
        todo!();
        /*
            bitmap.data = imageData + x * pixelStride + y * lineStride;
            bitmap.pixelFormat = pixelFormat;
            bitmap.lineStride = lineStride;
            bitmap.pixelStride = pixelStride;

            if (mode != ImageBitmapData::readOnly)
                sendDataChangeMessage();
        */
    }
    
    pub fn clone(&mut self) -> ImagePixelDataPtr {
        
        todo!();
        /*
            jassertfalse;
            return nullptr;
        */
    }
    
    pub fn create_type(&self) -> Box<dyn ImageType> {
        
        todo!();
        /*
            return std::make_unique<NativeImageType>();
        */
    }
    
    pub fn blit_to_window(&mut self, 
        window: Window,
        dx:     i32,
        dy:     i32,
        dw:     u32,
        dh:     u32,
        sx:     i32,
        sy:     i32)  {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

           #if ALOE_USE_XSHM
            if (isUsingXShm())
                XWindowSystem::getInstance()->addPendingPaintForWindow (window);
           #endif

            if (gc == None)
            {
                XGCValues gcvalues;
                gcvalues.foreground = None;
                gcvalues.background = None;
                gcvalues.function = GXcopy;
                gcvalues.plane_mask = AllPlanes;
                gcvalues.clip_mask = None;
                gcvalues.graphics_exposures = False;

                gc = X11Symbols::getInstance()->xCreateGC (display, window,
                                                           GCBackground | GCForeground | GCFunction | GCPlaneMask | GCClipMask | GCGraphicsExposures,
                                                           &gcvalues);
            }

            if (imageDepth == 16)
            {
                auto rMask   = (uint32) xImage->red_mask;
                auto gMask   = (uint32) xImage->green_mask;
                auto bMask   = (uint32) xImage->blue_mask;
                auto rShiftL = (uint32) jmax (0,  getShiftNeeded (rMask));
                auto rShiftR = (uint32) jmax (0, -getShiftNeeded (rMask));
                auto gShiftL = (uint32) jmax (0,  getShiftNeeded (gMask));
                auto gShiftR = (uint32) jmax (0, -getShiftNeeded (gMask));
                auto bShiftL = (uint32) jmax (0,  getShiftNeeded (bMask));
                auto bShiftR = (uint32) jmax (0, -getShiftNeeded (bMask));

                ImageBitmapData srcData (Image (this), ImageBitmapData::readOnly);

                for (int y = sy; y < sy + (int) dh; ++y)
                {
                    auto* p = srcData.getPixelPointer (sx, y);

                    for (int x = sx; x < sx + (int) dw; ++x)
                    {
                        auto* pixel = (PixelRGB*) p;
                        p += srcData.pixelStride;

                        X11Symbols::getInstance()->xPutPixel (xImage, x, y,
                                                                  (((((uint32) pixel->getRed())   << rShiftL) >> rShiftR) & rMask)
                                                                | (((((uint32) pixel->getGreen()) << gShiftL) >> gShiftR) & gMask)
                                                                | (((((uint32) pixel->getBlue())  << bShiftL) >> bShiftR) & bMask));
                    }
                }
            }

            // blit results to screen.
           #if ALOE_USE_XSHM
            if (isUsingXShm())
                X11Symbols::getInstance()->xShmPutImage (display, (::Drawable) window, gc, xImage, sx, sy, dx, dy, dw, dh, True);
            else
           #endif
                X11Symbols::getInstance()->xPutImage (display, (::Drawable) window, gc, xImage, sx, sy, dx, dy, dw, dh);
        */
    }

    #[cfg(ALOE_USE_XSHM)]
    pub fn is_using_xshm(&self) -> bool {
        
        todo!();
        /*
            return usingXShm;
        */
    }
    
    pub fn get_shift_needed(mask: u32) -> i32 {
        
        todo!();
        /*
            for (int i = 32; --i >= 0;)
                if (((mask >> i) & 1) != 0)
                    return i - 7;

            jassertfalse;
            return 0;
        */
    }
}
