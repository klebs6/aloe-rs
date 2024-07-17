crate::ix!();

///=============================== X11 - Render =================================
#[cfg(ALOE_USE_XRENDER)]
pub fn xrender_is_available(display: *mut Display) -> bool {
    
    todo!();
        /*
            int major, minor;
             return X11Symbols::getInstance()->xRenderQueryVersion (display, &major, &minor);
        */
}

#[cfg(ALOE_USE_XRENDER)]
pub fn xrender_has_compositing_window_manager(display: *mut Display) -> bool {
    
    todo!();
        /*
            return display != nullptr
                     && X11Symbols::getInstance()->xGetSelectionOwner (display,
                                                                       XWindowSystemUtilities::XWindowSystemAtoms::getCreating (display, "_NET_WM_CM_S0")) != 0;
        */
}

#[cfg(ALOE_USE_XRENDER)]
pub fn xrender_find_picture_format(display: *mut Display) -> *mut XRenderPictFormat {
    
    todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

             if (isAvailable (display))
             {
                 if (auto* pictFormat = X11Symbols::getInstance()->xRenderFindStandardFormat (display, PictStandardARGB32))
                 {
                     XRenderPictFormat desiredFormat;
                     desiredFormat.type = PictTypeDirect;
                     desiredFormat.depth = 32;

                     desiredFormat.direct.alphaMask = 0xff;
                     desiredFormat.direct.redMask   = 0xff;
                     desiredFormat.direct.greenMask = 0xff;
                     desiredFormat.direct.blueMask  = 0xff;

                     desiredFormat.direct.alpha = 24;
                     desiredFormat.direct.red   = 16;
                     desiredFormat.direct.green = 8;
                     desiredFormat.direct.blue  = 0;

                     pictFormat = X11Symbols::getInstance()->xRenderFindFormat (display,
                                                                                PictFormatType | PictFormatDepth
                                                                                 | PictFormatRedMask | PictFormatRed
                                                                                 | PictFormatGreenMask | PictFormatGreen
                                                                                 | PictFormatBlueMask | PictFormatBlue
                                                                                 | PictFormatAlphaMask | PictFormatAlpha,
                                                                                &desiredFormat,
                                                                                0);

                     return pictFormat;
                 }
             }

             return nullptr;
        */
}
