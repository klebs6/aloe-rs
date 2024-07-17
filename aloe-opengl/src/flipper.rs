crate::ix!();

pub struct Flipper<PixelType> {
    phantom: PhantomData<PixelType>,
}

impl<PixelType> Flipper<PixelType> {

    pub fn flip(
        data_copy:   &mut HeapBlock<PixelARGB>,
        src_data:    *const u8,
        line_stride: i32,
        w:           i32,
        h:           i32)  {
        
        todo!();
        /*
            dataCopy.malloc (w * h);

            for (int y = 0; y < h; ++y)
            {
                auto* src = (const PixelType*) srcData;
                auto* dst = (PixelARGB*) (dataCopy + w * (h - 1 - y));

                for (int x = 0; x < w; ++x)
                    dst[x].set (src[x]);

                srcData += lineStride;
            }
        */
    }
}

