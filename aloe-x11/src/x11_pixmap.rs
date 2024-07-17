crate::ix!();

///=============================== X11 - Pixmap =================================

pub fn pixmap_create_colour_pixmap_from_image(
    display: *mut Display,
    image:   &Image

) -> Pixmap {
    
    todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

            auto width  = (unsigned int) image.getWidth();
            auto height = (unsigned int) image.getHeight();
            HeapBlock<uint32> colour (width * height);
            int index = 0;

            for (int y = 0; y < (int) height; ++y)
                for (int x = 0; x < (int) width; ++x)
                    colour[index++] = image.getPixelAt (x, y).getARGB();

            auto ximage = makeXFreePtr (X11Symbols::getInstance()->xCreateImage (display, (Visual*) CopyFromParent, 24, ZPixmap,
                                                                                 0, reinterpret_cast<const char*> (colour.getData()),
                                                                                 width, height, 32, 0));

            auto pixmap = X11Symbols::getInstance()->xCreatePixmap (display,
                                                                    X11Symbols::getInstance()->xDefaultRootWindow (display),
                                                                    width, height, 24);

            XValueHolder<GC> gc (X11Symbols::getInstance()->xCreateGC (display, pixmap, 0, nullptr),
                                 [&display] (GC& g) { X11Symbols::getInstance()->xFreeGC (display, g); });
            X11Symbols::getInstance()->xPutImage (display, pixmap, gc.value, ximage.get(), 0, 0, 0, 0, width, height);

            return pixmap;
        */
}

pub fn pixmap_create_mask_pixmap_from_image(
    display: *mut Display,
    image:   &Image

) -> Pixmap {
    
    todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

            auto width  = (unsigned int) image.getWidth();
            auto height = (unsigned int) image.getHeight();
            auto stride = (width + 7) >> 3;
            HeapBlock<char> mask;
            mask.calloc (stride * height);

            auto msbfirst = (X11Symbols::getInstance()->xBitmapBitOrder (display) == MSBFirst);

            for (unsigned int y = 0; y < height; ++y)
            {
                for (unsigned int x = 0; x < width; ++x)
                {
                    auto bit = (char) (1 << (msbfirst ? (7 - (x & 7)) : (x & 7)));
                    auto offset = y * stride + (x >> 3);

                    if (image.getPixelAt ((int) x, (int) y).getAlpha() >= 128)
                        mask[offset] |= bit;
                }
            }

            return X11Symbols::getInstance()->xCreatePixmapFromBitmapData (display, X11Symbols::getInstance()->xDefaultRootWindow (display),
                                                                           mask.getData(), width, height, 1, 0, 1);
        */
}
