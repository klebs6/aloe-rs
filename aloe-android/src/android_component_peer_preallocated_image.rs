crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AndroidComponentPeerPreallocatedImage {
    base:           ImagePixelData,
    data:           *mut i32,
    allocated_data: HeapBlock<i32>,
    has_alpha:      bool,
}

impl Drop for AndroidComponentPeerPreallocatedImage {

    fn drop(&mut self) {
        todo!();
        /* 
                if (hasAlpha)
                {
                    auto pix = (PixelARGB*) data;

                    for (int i = width * height; --i >= 0;)
                    {
                        pix->unpremultiply();
                        ++pix;
                    }
                }
             */
    }
}

impl AndroidComponentPeerPreallocatedImage {

    pub fn new(
        width:     i32,
        height:    i32,
        data:      *mut i32,
        has_alpha: bool) -> Self {
    
        todo!();
        /*
        : image_pixel_data(Image::ARGB, width_, height_),
        : data(data_),
        : has_alpha(hasAlpha_),

            if (hasAlpha_)
                    zeromem (data_, static_cast<size_t> (width * height) * sizeof (jint));
        */
    }
    
    pub fn create_type(&self) -> Box<dyn ImageType> {
        
        todo!();
        /*
            return std::make_unique<SoftwareImageType>();
        */
    }
    
    pub fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            return std::make_unique<LowLevelGraphicsSoftwareRenderer> (Image (this));
        */
    }
    
    pub fn initialise_bitmap_data(
        &mut self, 
        bm:   &mut ImageBitmapData,
        x:    i32,
        y:    i32,
        mode: ImageBitmapDataReadWriteMode

    ) {
        
        todo!();
        /*
            bm.lineStride = width * static_cast<int> (sizeof (jint));
                bm.pixelStride = static_cast<int> (sizeof (jint));
                bm.pixelFormat = Image::ARGB;
                bm.data = (uint8*) (data + x + y * width);
        */
    }
    
    pub fn clone(&mut self) -> ImagePixelDataPtr {
        
        todo!();
        /*
            auto s = new AndroidComponentPeerPreallocatedImage (width, height, nullptr, hasAlpha);
                s->allocatedData.malloc (sizeof (jint) * static_cast<size_t> (width * height));
                s->data = s->allocatedData;
                memcpy (s->data, data, sizeof (jint) * static_cast<size_t> (width * height));
                return s;
        */
    }
}
