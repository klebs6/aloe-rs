crate::ix!();

///================================ X11 - Visuals ===============================

pub fn x11_visuals_find_visual_with_depth(
    display:       *mut Display,
    desired_depth: i32

) -> *mut Visual {
    
    todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

            Visual* visual = nullptr;
            int numVisuals = 0;
            auto desiredMask = VisualNoMask;
            XVisualInfo desiredVisual;

            desiredVisual.screen = X11Symbols::getInstance()->xDefaultScreen (display);
            desiredVisual.depth = desiredDepth;

            desiredMask = VisualScreenMask | VisualDepthMask;

            if (desiredDepth == 32)
            {
                desiredVisual.c_class    = TrueColor;
                desiredVisual.red_mask   = 0x00FF0000;
                desiredVisual.green_mask = 0x0000FF00;
                desiredVisual.blue_mask  = 0x000000FF;
                desiredVisual.bits_per_rgb = 8;

                desiredMask |= VisualClassMask;
                desiredMask |= VisualRedMaskMask;
                desiredMask |= VisualGreenMaskMask;
                desiredMask |= VisualBlueMaskMask;
                desiredMask |= VisualBitsPerRGBMask;
            }

            if (auto xvinfos = makeXFreePtr (X11Symbols::getInstance()->xGetVisualInfo (display, desiredMask, &desiredVisual, &numVisuals)))
            {
                for (int i = 0; i < numVisuals; i++)
                {
                    if (xvinfos.get()[i].depth == desiredDepth)
                    {
                        visual = xvinfos.get()[i].visual;
                        break;
                    }
                }
            }

            return visual;
        */
}

pub fn x11_visuals_find_visual_format(
    display:       *mut Display,
    desired_depth: i32,
    matched_depth: &mut i32

) -> *mut Visual {
    
    todo!();
        /*
            Visual* visual = nullptr;

            if (desiredDepth == 32)
            {
               #if ALOE_USE_XSHM
                if (XSHMHelpers::isShmAvailable (display))
                {
                   #if ALOE_USE_XRENDER
                    if (XRender::isAvailable (display))
                    {
                        if (XRender::findPictureFormat (display) != nullptr)
                        {
                            int numVisuals = 0;
                            XVisualInfo desiredVisual;
                            desiredVisual.screen = X11Symbols::getInstance()->xDefaultScreen (display);
                            desiredVisual.depth = 32;
                            desiredVisual.bits_per_rgb = 8;

                            if (auto xvinfos = makeXFreePtr (X11Symbols::getInstance()->xGetVisualInfo (display,
                                                                                                        VisualScreenMask | VisualDepthMask | VisualBitsPerRGBMask,
                                                                                                        &desiredVisual, &numVisuals)))
                            {
                                for (int i = 0; i < numVisuals; ++i)
                                {
                                    auto pictVisualFormat = X11Symbols::getInstance()->xRenderFindVisualFormat (display, xvinfos.get()[i].visual);

                                    if (pictVisualFormat != nullptr
                                         && pictVisualFormat->type == PictTypeDirect
                                         && pictVisualFormat->direct.alphaMask)
                                    {
                                        visual = xvinfos.get()[i].visual;
                                        matchedDepth = 32;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                   #endif
                    if (visual == nullptr)
                    {
                        visual = findVisualWithDepth (display, 32);

                        if (visual != nullptr)
                            matchedDepth = 32;
                    }
                }
               #endif
            }

            if (visual == nullptr && desiredDepth >= 24)
            {
                visual = findVisualWithDepth (display, 24);

                if (visual != nullptr)
                    matchedDepth = 24;
            }

            if (visual == nullptr && desiredDepth >= 16)
            {
                visual = findVisualWithDepth (display, 16);

                if (visual != nullptr)
                    matchedDepth = 16;
            }

            return visual;
        */
}
