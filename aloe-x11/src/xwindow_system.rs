crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/x11/aloe_linux_XWindowSystem.h]

#[no_copy]
#[leak_detector]
pub struct XWindowSystem {
    base:                    DeletedAtShutdown,
    x_is_available:          bool, // default = false
    atoms:                   XWindowSystemAtoms,
    display:                 *mut Display, // default = nullptr
    display_visuals:         Box<XWindowSystemDisplayVisuals>,

    #[cfg(ALOE_USE_XSHM)]
    shm_paints_pending_map:  HashMap<Window,i32>,

    shm_completion_event:    i32, // default = 0
    pointer_map:             [i32; 5], // default = {}
    local_clipboard_content: String,
    parent_screen_position:  Point<i32>,
}

aloe_declare_singleton!{
    XWindowSystem, false
}

aloe_implement_singleton!{
    XWindowSystem
}

impl Drop for XWindowSystem {
    fn drop(&mut self) {
        todo!();
        /*
            if (xIsAvailable)
        {
            destroyXDisplay();

            if (ALOEApplicationBase::isStandaloneApp())
                X11ErrorHandling::removeXErrorHandlers();
        }

        X11Symbols::deleteInstance();
        clearSingletonInstance();
        */
    }
}

impl XWindowSystem {
    
    pub fn new() -> Self {
    
        todo!();
        /*


            xIsAvailable = X11Symbols::getInstance()->loadAllSymbols();

        if (! xIsAvailable)
            return;

        if (ALOEApplicationBase::isStandaloneApp())
        {
            // Initialise xlib for multiple thread support
            static bool initThreadCalled = false;

            if (! initThreadCalled)
            {
                if (! X11Symbols::getInstance()->xInitThreads())
                {
                    // This is fatal!  Print error and closedown
                    Logger::outputDebugString ("Failed to initialise xlib thread support.");
                    Process::terminate();

                    return;
                }

                initThreadCalled = true;
            }

            X11ErrorHandling::installXErrorHandlers();
        }

        if (! initialiseXDisplay())
        {
            if (ALOEApplicationBase::isStandaloneApp())
                X11ErrorHandling::removeXErrorHandlers();

            X11Symbols::deleteInstance();
            xIsAvailable = false;
        }
        */
    }
    
    pub fn create_window(&self, 
        parent_to_add_to: Window,
        peer:             *mut LinuxComponentPeer) -> Window {
        
        todo!();
        /*
            if (! xIsAvailable)
        {
            // can't open a window on a system that doesn't have X11 installed!
            jassertfalse;
            return 0;
        }

        auto styleFlags = peer->getStyleFlags();

        XWindowSystemUtilities::ScopedXLock xLock;

        auto root = X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display));

        auto visualAndDepth = displayVisuals->getBestVisualForWindow ((styleFlags & ComponentPeer::windowIsSemiTransparent) != 0);

        auto colormap = X11Symbols::getInstance()->xCreateColormap (display, root, visualAndDepth.visual, AllocNone);
        X11Symbols::getInstance()->xInstallColormap (display, colormap);

        // Set up the window attributes
        XSetWindowAttributes swa;
        swa.border_pixel = 0;
        swa.background_pixmap = None;
        swa.colormap = colormap;
        swa.override_redirect = ((styleFlags & ComponentPeer::windowIsTemporary) != 0) ? True : False;
        swa.event_mask = getAllEventsMask (styleFlags & ComponentPeer::windowIgnoresMouseClicks);

        auto windowH = X11Symbols::getInstance()->xCreateWindow (display, parentToAddTo != 0 ? parentToAddTo : root,
                                                                 0, 0, 1, 1,
                                                                 0, visualAndDepth.depth, InputOutput, visualAndDepth.visual,
                                                                 CWBorderPixel | CWColormap | CWBackPixmap | CWEventMask | CWOverrideRedirect,
                                                                 &swa);

        // Set the window context to identify the window handle object
        if (X11Symbols::getInstance()->xSaveContext (display, (XID) windowH, windowHandleXContext, (XPointer) peer))
        {
            // Failed
            jassertfalse;

            Logger::outputDebugString ("Failed to create context information for window.\n");
            X11Symbols::getInstance()->xDestroyWindow (display, windowH);

            return 0;
        }

        // Set window manager hints
        if (auto wmHints = makeXFreePtr (X11Symbols::getInstance()->xAllocWMHints()))
        {
            wmHints->flags = InputHint | StateHint;
            wmHints->input = True;
            wmHints->initial_state = NormalState;
            X11Symbols::getInstance()->xSetWMHints (display, windowH, wmHints.get());
        }

        // Set class hint
        if (auto* app = ALOEApplicationBase::getInstance())
        {
            if (auto classHint = makeXFreePtr (X11Symbols::getInstance()->xAllocClassHint()))
            {
                auto appName = app->getApplicationName();
                classHint->res_name  = (char*) appName.getCharPointer().getAddress();
                classHint->res_class = (char*) appName.getCharPointer().getAddress();

                X11Symbols::getInstance()->xSetClassHint (display, windowH, classHint.get());
            }
        }

        // Set the window type
        setWindowType (windowH, styleFlags);

        // Define decoration
        if ((styleFlags & ComponentPeer::windowHasTitleBar) == 0)
            removeWindowDecorations (windowH);
        else
            addWindowButtons (windowH, styleFlags);

        // Associate the PID, allowing to be shut down when something goes wrong
        auto pid = (unsigned long) getpid();
        xchangeProperty (windowH, atoms.pid, XA_CARDINAL, 32, &pid, 1);

        // Set window manager protocols
        xchangeProperty (windowH, atoms.protocols, XA_ATOM, 32, atoms.protocolList, 2);

        // Set drag and drop flags
        xchangeProperty (windowH, atoms.XdndTypeList, XA_ATOM, 32, atoms.allowedMimeTypes, numElementsInArray (atoms.allowedMimeTypes));
        xchangeProperty (windowH, atoms.XdndActionList, XA_ATOM, 32, atoms.allowedActions, numElementsInArray (atoms.allowedActions));
        xchangeProperty (windowH, atoms.XdndActionDescription, XA_STRING, 8, "", 0);

        auto dndVersion = XWindowSystemUtilities::XWindowSystemAtoms::DndVersion;
        xchangeProperty (windowH, atoms.XdndAware, XA_ATOM, 32, &dndVersion, 1);

        unsigned long info[2] = { 0, 1 };
        xchangeProperty (windowH, atoms.XembedInfo, atoms.XembedInfo, 32, (unsigned char*) info, 2);

        return windowH;
        */
    }
    
    pub fn destroy_window(&mut self, windowh: Window)  {
        
        todo!();
        /*
            auto* peer = dynamic_cast<LinuxComponentPeer*> (getPeerFor (windowH));

        if (peer == nullptr)
        {
            jassertfalse;
            return;
        }

       #if ALOE_X11_SUPPORTS_XEMBED
        aloe_handleXEmbedEvent (peer, nullptr);
       #endif

        deleteIconPixmaps (windowH);
        dragAndDropStateMap.erase (peer);

        XWindowSystemUtilities::ScopedXLock xLock;

        XPointer handlePointer;

        if (! X11Symbols::getInstance()->xFindContext (display, (XID) windowH, windowHandleXContext, &handlePointer))
            X11Symbols::getInstance()->xDeleteContext (display, (XID) windowH, windowHandleXContext);

        X11Symbols::getInstance()->xDestroyWindow (display, windowH);

        // Wait for it to complete and then remove any events for this
        // window from the event queue.
        X11Symbols::getInstance()->xSync (display, false);

        XEvent event;
        while (X11Symbols::getInstance()->xCheckWindowEvent (display, windowH,
                                                             getAllEventsMask (peer->getStyleFlags() & ComponentPeer::windowIgnoresMouseClicks),
                                                             &event) == True)
        {}

       #if ALOE_USE_XSHM
        if (XSHMHelpers::isShmAvailable (display))
            shmPaintsPendingMap.erase (windowH);
       #endif
        */
    }
    
    pub fn set_title(&self, 
        windowh: Window,
        title:   &String)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        XTextProperty nameProperty;
        char* strings[] = { const_cast<char*> (title.toRawUTF8()) };

        XWindowSystemUtilities::ScopedXLock xLock;

        if (X11Symbols::getInstance()->xStringListToTextProperty (strings, 1, &nameProperty))
        {
            X11Symbols::getInstance()->xSetWMName (display, windowH, &nameProperty);
            X11Symbols::getInstance()->xSetWMIconName (display, windowH, &nameProperty);

            X11Symbols::getInstance()->xFree (nameProperty.value);
        }
        */
    }
    
    pub fn set_icon(&self, 
        windowh:  Window,
        new_icon: &Image)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        auto dataSize = newIcon.getWidth() * newIcon.getHeight() + 2;
        HeapBlock<unsigned long> data (dataSize);

        int index = 0;
        data[index++] = (unsigned long) newIcon.getWidth();
        data[index++] = (unsigned long) newIcon.getHeight();

        for (int y = 0; y < newIcon.getHeight(); ++y)
            for (int x = 0; x < newIcon.getWidth(); ++x)
                data[index++] = (unsigned long) newIcon.getPixelAt (x, y).getARGB();

        XWindowSystemUtilities::ScopedXLock xLock;
        xchangeProperty (windowH, XWindowSystemUtilities::XWindowSystemAtoms::getCreating (display, "_NET_WM_ICON"),
                         XA_CARDINAL, 32, data.getData(), dataSize);

        deleteIconPixmaps (windowH);

        auto wmHints = makeXFreePtr (X11Symbols::getInstance()->xGetWMHints (display, windowH));

        if (wmHints == nullptr)
            wmHints = makeXFreePtr (X11Symbols::getInstance()->xAllocWMHints());

        if (wmHints != nullptr)
        {
            wmHints->flags |= IconPixmapHint | IconMaskHint;
            wmHints->icon_pixmap = PixmapHelpers::createColourPixmapFromImage (display, newIcon);
            wmHints->icon_mask = PixmapHelpers::createMaskPixmapFromImage (display, newIcon);

            X11Symbols::getInstance()->xSetWMHints (display, windowH, wmHints.get());
        }

        X11Symbols::getInstance()->xSync (display, False);
        */
    }
    
    pub fn set_visible(&self, 
        windowh:           Window,
        should_be_visible: bool)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;

        if (shouldBeVisible)
            X11Symbols::getInstance()->xMapWindow (display, windowH);
        else
            X11Symbols::getInstance()->xUnmapWindow (display, windowH);
        */
    }
    
    pub fn set_bounds(&self, 
        windowh:        Window,
        new_bounds:     Rectangle<i32>,
        is_full_screen: bool)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        if (auto* peer = getPeerFor (windowH))
        {
            if (peer->isFullScreen() && ! isFullScreen)
            {
                // When transitioning back from fullscreen, we might need to remove
                // the FULLSCREEN window property
                Atom fs = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_NET_WM_STATE_FULLSCREEN");

                if (fs != None)
                {
                    auto root = X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display));

                    XClientMessageEvent clientMsg;
                    clientMsg.display = display;
                    clientMsg.window = windowH;
                    clientMsg.type = ClientMessage;
                    clientMsg.format = 32;
                    clientMsg.message_type = atoms.windowState;
                    clientMsg.data.l[0] = 0;  // Remove
                    clientMsg.data.l[1] = (long) fs;
                    clientMsg.data.l[2] = 0;
                    clientMsg.data.l[3] = 1;  // Normal Source

                    XWindowSystemUtilities::ScopedXLock xLock;
                    X11Symbols::getInstance()->xSendEvent (display, root, false,
                                                           SubstructureRedirectMask | SubstructureNotifyMask,
                                                           (XEvent*) &clientMsg);
                }
            }

            XWindowSystemUtilities::ScopedXLock xLock;

            if (auto hints = makeXFreePtr (X11Symbols::getInstance()->xAllocSizeHints()))
            {
                hints->flags  = USSize | USPosition;
                hints->x      = newBounds.getX();
                hints->y      = newBounds.getY();
                hints->width  = newBounds.getWidth();
                hints->height = newBounds.getHeight();

                if ((peer->getStyleFlags() & ComponentPeer::windowIsResizable) == 0)
                {
                    hints->min_width  = hints->max_width  = hints->width;
                    hints->min_height = hints->max_height = hints->height;
                    hints->flags |= PMinSize | PMaxSize;
                }

                X11Symbols::getInstance()->xSetWMNormalHints (display, windowH, hints.get());
            }

            auto windowBorder = peer->getFrameSize();

            X11Symbols::getInstance()->xMoveResizeWindow (display, windowH,
                                                          newBounds.getX() - windowBorder.getLeft(),
                                                          newBounds.getY() - windowBorder.getTop(),
                                                          (unsigned int) newBounds.getWidth(),
                                                          (unsigned int) newBounds.getHeight());
        }
        */
    }
    
    pub fn contains(&self, 
        windowh:   Window,
        local_pos: Point<i32>) -> bool {
        
        todo!();
        /*
            Window root, child;
        int wx, wy;
        unsigned int ww, wh, bw, bitDepth;

        XWindowSystemUtilities::ScopedXLock xLock;

        return X11Symbols::getInstance()->xGetGeometry (display, (::Drawable) windowH, &root, &wx, &wy, &ww, &wh, &bw, &bitDepth)
              && X11Symbols::getInstance()->xTranslateCoordinates (display, windowH, windowH, localPos.getX(), localPos.getY(), &wx, &wy, &child)
              && child == None;
        */
    }
    
    pub fn get_border_size(&self, windowh: Window) -> BorderSize<i32> {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;
        auto hints = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_NET_FRAME_EXTENTS");

        if (hints != None)
        {
            XWindowSystemUtilities::GetXProperty prop (windowH, hints, 0, 4, false, XA_CARDINAL);

            if (prop.success && prop.actualFormat == 32)
            {
                auto data = prop.data;
                std::array<unsigned long, 4> sizes;

                for (auto& size : sizes)
                {
                    memcpy (&size, data, sizeof (unsigned long));
                    data += sizeof (unsigned long);
                }

                return { (int) sizes[2], (int) sizes[0], (int) sizes[3], (int) sizes[1] };
            }
        }

        return {};
        */
    }
    
    pub fn get_window_bounds(&mut self, 
        windowh:       Window,
        parent_window: Window) -> Rectangle<i32> {
        
        todo!();
        /*
            jassert (windowH != 0);

        Window root, child;
        int wx = 0, wy = 0;
        unsigned int ww = 0, wh = 0, bw, bitDepth;

        XWindowSystemUtilities::ScopedXLock xLock;

        if (X11Symbols::getInstance()->xGetGeometry (display, (::Drawable) windowH, &root, &wx, &wy, &ww, &wh, &bw, &bitDepth))
        {
            int rootX = 0, rootY = 0;

            if (! X11Symbols::getInstance()->xTranslateCoordinates (display, windowH, root, 0, 0, &rootX, &rootY, &child))
                rootX = rootY = 0;

            if (parentWindow == 0)
            {
                wx = rootX;
                wy = rootY;
            }
            else
            {
                parentScreenPosition = Point<int> (rootX, rootY);
            }
        }

        return { wx, wy, (int) ww, (int) wh };
        */
    }
    
    pub fn get_physical_parent_screen_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return parentScreenPosition;
        */
    }
    
    pub fn set_minimised(&self, 
        windowh:             Window,
        should_be_minimised: bool)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        if (shouldBeMinimised)
        {
            auto root = X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display));

            XClientMessageEvent clientMsg;
            clientMsg.display = display;
            clientMsg.window = windowH;
            clientMsg.type = ClientMessage;
            clientMsg.format = 32;
            clientMsg.message_type = atoms.changeState;
            clientMsg.data.l[0] = IconicState;

            XWindowSystemUtilities::ScopedXLock xLock;
            X11Symbols::getInstance()->xSendEvent (display, root, false, SubstructureRedirectMask | SubstructureNotifyMask, (XEvent*) &clientMsg);
        }
        */
    }
    
    pub fn is_minimised(&self, windowh: Window) -> bool {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;
        XWindowSystemUtilities::GetXProperty prop (windowH, atoms.state, 0, 64, false, atoms.state);

        if (prop.success && prop.actualType == atoms.state
            && prop.actualFormat == 32 && prop.numItems > 0)
        {
            unsigned long state;
            memcpy (&state, prop.data, sizeof (unsigned long));

            return state == IconicState;
        }

        return false;
        */
    }
    
    pub fn set_maximised(&self, 
        windowh:             Window,
        should_be_maximised: bool)  {
        
        todo!();
        /*
            const auto root = X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display));

        XEvent ev;
        ev.xclient.window = windowH;
        ev.xclient.type   = ClientMessage;
        ev.xclient.format = 32;
        ev.xclient.message_type = XWindowSystemUtilities::XWindowSystemAtoms::getCreating (display, "_NET_WM_STATE");
        ev.xclient.data.l[0] = shouldBeMaximised;
        ev.xclient.data.l[1] = (long) XWindowSystemUtilities::XWindowSystemAtoms::getCreating (display, "_NET_WM_STATE_MAXIMIZED_HORZ");
        ev.xclient.data.l[2] = (long) XWindowSystemUtilities::XWindowSystemAtoms::getCreating (display, "_NET_WM_STATE_MAXIMIZED_VERT");
        ev.xclient.data.l[3] = 1;
        ev.xclient.data.l[4] = 0;

        XWindowSystemUtilities::ScopedXLock xLock;
        X11Symbols::getInstance()->xSendEvent (display, root, false, SubstructureRedirectMask | SubstructureNotifyMask, &ev);
        */
    }
    
    pub fn to_front(&self, 
        windowh: Window,
        _1:      bool)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;
        XEvent ev;
        ev.xclient.type = ClientMessage;
        ev.xclient.serial = 0;
        ev.xclient.send_event = True;
        ev.xclient.message_type = atoms.activeWin;
        ev.xclient.window = windowH;
        ev.xclient.format = 32;
        ev.xclient.data.l[0] = 2;
        ev.xclient.data.l[1] = getUserTime (windowH);
        ev.xclient.data.l[2] = 0;
        ev.xclient.data.l[3] = 0;
        ev.xclient.data.l[4] = 0;

        X11Symbols::getInstance()->xSendEvent (display, X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display)),
                                               False, SubstructureRedirectMask | SubstructureNotifyMask, &ev);

        X11Symbols::getInstance()->xSync (display, False);
        */
    }
    
    pub fn to_behind(&self, 
        windowh:      Window,
        other_window: Window)  {
        
        todo!();
        /*
            jassert (windowH != 0 && otherWindow != 0);

        Window newStack[] = { otherWindow, windowH };

        XWindowSystemUtilities::ScopedXLock xLock;
        X11Symbols::getInstance()->xRestackWindows (display, newStack, 2);
        */
    }
    
    pub fn is_focused(&self, windowh: Window) -> bool {
        
        todo!();
        /*
            jassert (windowH != 0);

        int revert = 0;
        Window focusedWindow = 0;
        XWindowSystemUtilities::ScopedXLock xLock;
        X11Symbols::getInstance()->xGetInputFocus (display, &focusedWindow, &revert);

        if (focusedWindow == PointerRoot)
            return false;

        return isParentWindowOf (windowH, focusedWindow);
        */
    }
    
    pub fn get_focus_window(&self, windowh: Window) -> Window {
        
        todo!();
        /*
            jassert (windowH != 0);

       #if ALOE_X11_SUPPORTS_XEMBED
        if (auto w = (Window) aloe_getCurrentFocusWindow (dynamic_cast<LinuxComponentPeer*> (getPeerFor (windowH))))
            return w;
       #endif

        return windowH;
        */
    }
    
    pub fn grab_focus(&self, windowh: Window) -> bool {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowAttributes atts;
        XWindowSystemUtilities::ScopedXLock xLock;

        if (windowH != 0
            && X11Symbols::getInstance()->xGetWindowAttributes (display, windowH, &atts)
            && atts.map_state == IsViewable
            && ! isFocused (windowH))
        {
            X11Symbols::getInstance()->xSetInputFocus (display, getFocusWindow (windowH), RevertToParent, (Time) getUserTime (windowH));
            return true;
        }

        return false;
        */
    }
    
    pub fn can_use_semi_transparent_windows(&self) -> bool {
        
        todo!();
        /*
            #if ALOE_USE_XRENDER
        if (XRender::hasCompositingWindowManager (display))
        {
            int matchedDepth = 0, desiredDepth = 32;

            return Visuals::findVisualFormat (display, desiredDepth, matchedDepth) != nullptr
                    && matchedDepth == desiredDepth;
        }
       #endif

        return false;
        */
    }
    
    pub fn can_use_argb_images(&self) -> bool {
        
        todo!();
        /*
            static bool canUseARGB = false;

       #if ALOE_USE_XSHM
        static bool checked = false;

        if (! checked)
        {
            if (XSHMHelpers::isShmAvailable (display))
            {
                XWindowSystemUtilities::ScopedXLock xLock;
                XShmSegmentInfo segmentinfo;

                auto testImage = X11Symbols::getInstance()->xShmCreateImage (display,
                                                                             X11Symbols::getInstance()->xDefaultVisual (display, X11Symbols::getInstance()->xDefaultScreen (display)),
                                                                             24, ZPixmap, nullptr, &segmentinfo, 64, 64);

                canUseARGB = testImage != nullptr && testImage->bits_per_pixel == 32;
                X11Symbols::getInstance()->xDestroyImage (testImage);
            }
            else
            {
                canUseARGB = false;
            }

            checked = true;
        }
       #endif

        return canUseARGB;
        */
    }
    
    pub fn create_image(&self, 
        is_semi_transparent: bool,
        width:               i32,
        height:              i32,
        argb:                bool) -> Image {
        
        todo!();
        /*
            auto visualAndDepth = displayVisuals->getBestVisualForWindow (isSemiTransparent);

       #if ALOE_USE_XSHM
        return Image (new XBitmapImage (argb ? Image::ARGB : Image::RGB,
       #else
        return Image (new XBitmapImage (Image::RGB,
       #endif
                                        (width + 31) & ~31,
                                        (height + 31) & ~31,
                                        false, (unsigned int) visualAndDepth.depth, visualAndDepth.visual));
        */
    }
    
    pub fn blit_to_window(&self, 
        windowh:          Window,
        image:            Image,
        destination_rect: Rectangle<i32>,
        total_rect:       Rectangle<i32>)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        auto* xbitmap = static_cast<XBitmapImage*> (image.getPixelData());

        xbitmap->blitToWindow (windowH,
                               destinationRect.getX(), destinationRect.getY(),
                               (unsigned int) destinationRect.getWidth(),
                               (unsigned int) destinationRect.getHeight(),
                               destinationRect.getX() - totalRect.getX(), destinationRect.getY() - totalRect.getY());
        */
    }
    
    pub fn process_pending_paints_for_window(&mut self, windowh: Window)  {
        
        todo!();
        /*
            #if ALOE_USE_XSHM
        if (! XSHMHelpers::isShmAvailable (display))
            return;

        if (getNumPaintsPendingForWindow (windowH) > 0)
        {
            XWindowSystemUtilities::ScopedXLock xLock;

            XEvent evt;
            while (X11Symbols::getInstance()->xCheckTypedWindowEvent (display, windowH, shmCompletionEvent, &evt))
                removePendingPaintForWindow (windowH);
        }
       #endif
        */
    }
    
    pub fn get_num_paints_pending_for_window(&mut self, windowh: Window) -> i32 {
        
        todo!();
        /*
            #if ALOE_USE_XSHM
        if (XSHMHelpers::isShmAvailable (display))
            return shmPaintsPendingMap[windowH];
       #endif

        return 0;
        */
    }
    
    pub fn add_pending_paint_for_window(&mut self, windowh: Window)  {
        
        todo!();
        /*
            #if ALOE_USE_XSHM
        if (XSHMHelpers::isShmAvailable (display))
            ++shmPaintsPendingMap[windowH];
       #endif
        */
    }
    
    pub fn remove_pending_paint_for_window(&mut self, windowh: Window)  {
        
        todo!();
        /*
            #if ALOE_USE_XSHM
        if (XSHMHelpers::isShmAvailable (display))
            --shmPaintsPendingMap[windowH];
       #endif
        */
    }
    
    pub fn set_screen_saver_enabled(&self, enabled: bool)  {
        
        todo!();
        /*
            using tXScreenSaverSuspend = void (*) (Display*, Bool);
        static tXScreenSaverSuspend xScreenSaverSuspend = nullptr;

        if (xScreenSaverSuspend == nullptr)
            if (void* h = dlopen ("libXss.so.1", RTLD_GLOBAL | RTLD_NOW))
                xScreenSaverSuspend = (tXScreenSaverSuspend) dlsym (h, "XScreenSaverSuspend");

        XWindowSystemUtilities::ScopedXLock xLock;

        if (xScreenSaverSuspend != nullptr)
            xScreenSaverSuspend (display, ! enabled);
        */
    }
    
    pub fn get_current_mouse_position(&self) -> Point<f32> {
        
        todo!();
        /*
            Window root, child;
        int x, y, winx, winy;
        unsigned int mask;

        XWindowSystemUtilities::ScopedXLock xLock;

        if (X11Symbols::getInstance()->xQueryPointer (display,
                                                      X11Symbols::getInstance()->xRootWindow (display,
                                                                                              X11Symbols::getInstance()->xDefaultScreen (display)),
                                                      &root, &child,
                                                      &x, &y, &winx, &winy, &mask) == False)
        {
            x = y = -1;
        }

        return { (float) x, (float) y };
        */
    }
    
    pub fn set_mouse_position(&self, pos: Point<f32>)  {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;

        auto root = X11Symbols::getInstance()->xRootWindow (display,
                                                            X11Symbols::getInstance()->xDefaultScreen (display));

        X11Symbols::getInstance()->xWarpPointer (display, None, root, 0, 0, 0, 0,
                                                 roundToInt (pos.getX()), roundToInt (pos.getY()));
        */
    }
    
    pub fn create_custom_mouse_cursor_info(&self, 
        image:   &Image,
        hotspot: Point<i32>)  {
        
        todo!();
        /*
            if (display == nullptr)
            return nullptr;

        XWindowSystemUtilities::ScopedXLock xLock;

        auto imageW = (unsigned int) image.getWidth();
        auto imageH = (unsigned int) image.getHeight();
        auto hotspotX = hotspot.x;
        auto hotspotY = hotspot.y;

       #if ALOE_USE_XCURSOR
        if (auto xcImage = makeDeletedPtr (X11Symbols::getInstance()->xcursorImageCreate ((int) imageW, (int) imageH),
                                           [] (XcursorImage* i) { X11Symbols::getInstance()->xcursorImageDestroy (i); }))
        {
            xcImage->xhot = (XcursorDim) hotspotX;
            xcImage->yhot = (XcursorDim) hotspotY;
            auto* dest = xcImage->pixels;

            for (int y = 0; y < (int) imageH; ++y)
                for (int x = 0; x < (int) imageW; ++x)
                    *dest++ = image.getPixelAt (x, y).getARGB();

            auto* result = (void*) X11Symbols::getInstance()->xcursorImageLoadCursor (display, xcImage.get());

            if (result != nullptr)
                return result;
        }
       #endif

        auto root = X11Symbols::getInstance()->xRootWindow (display,
                                                            X11Symbols::getInstance()->xDefaultScreen (display));

        unsigned int cursorW, cursorH;
        if (! X11Symbols::getInstance()->xQueryBestCursor (display, root, imageW, imageH, &cursorW, &cursorH))
            return nullptr;

        Image im (Image::ARGB, (int) cursorW, (int) cursorH, true);

        {
            Graphics g (im);

            if (imageW > cursorW || imageH > cursorH)
            {
                hotspotX = (hotspotX * (int) cursorW) / (int) imageW;
                hotspotY = (hotspotY * (int) cursorH) / (int) imageH;

                g.drawImage (image, Rectangle<float> ((float) imageW, (float) imageH),
                             RectanglePlacement::xLeft | RectanglePlacement::yTop | RectanglePlacement::onlyReduceInSize);
            }
            else
            {
                g.drawImageAt (image, 0, 0);
            }
        }

        auto stride = (cursorW + 7) >> 3;
        HeapBlock<char> maskPlane, sourcePlane;
        maskPlane.calloc (stride * cursorH);
        sourcePlane.calloc (stride * cursorH);

        auto msbfirst = (X11Symbols::getInstance()->xBitmapBitOrder (display) == MSBFirst);

        for (auto y = (int) cursorH; --y >= 0;)
        {
            for (auto x = (int) cursorW; --x >= 0;)
            {
                auto mask   = (char) (1 << (msbfirst ? (7 - (x & 7)) : (x & 7)));
                auto offset = (unsigned int) y * stride + ((unsigned int) x >> 3);

                auto c = im.getPixelAt (x, y);

                if (c.getAlpha() >= 128)        maskPlane[offset]   |= mask;
                if (c.getBrightness() >= 0.5f)  sourcePlane[offset] |= mask;
            }
        }

        auto xFreePixmap = [this] (Pixmap& p) { X11Symbols::getInstance()->xFreePixmap (display, p); };
        XValueHolder<Pixmap> sourcePixmap (X11Symbols::getInstance()->xCreatePixmapFromBitmapData (display, root, sourcePlane.getData(), cursorW, cursorH, 0xffff, 0, 1), xFreePixmap);
        XValueHolder<Pixmap> maskPixmap   (X11Symbols::getInstance()->xCreatePixmapFromBitmapData (display, root, maskPlane.getData(),   cursorW, cursorH, 0xffff, 0, 1), xFreePixmap);

        XColor white, black;
        black.red = black.green = black.blue = 0;
        white.red = white.green = white.blue = 0xffff;

        return (void*) X11Symbols::getInstance()->xCreatePixmapCursor (display, sourcePixmap.value, maskPixmap.value, &white, &black,
                                                                       (unsigned int) hotspotX, (unsigned int) hotspotY);
        */
    }
    
    pub fn delete_mouse_cursor(&self, cursor_handle: *mut c_void)  {
        
        todo!();
        /*
            if (cursorHandle != nullptr && display != nullptr)
        {
            XWindowSystemUtilities::ScopedXLock xLock;
            X11Symbols::getInstance()->xFreeCursor (display, (Cursor) cursorHandle);
        }
        */
    }
    
    pub fn create_standard_mouse_cursor(&self, ty: MouseCursorStandardCursorType)  {
        
        todo!();
        /*
            if (display == nullptr)
            return None;

        unsigned int shape;

        switch (type)
        {
            case MouseCursor::NormalCursor:
            case MouseCursor::ParentCursor:                  return None; // Use parent cursor
            case MouseCursor::NoCursor:                      return CustomMouseCursorInfo (Image (Image::ARGB, 16, 16, true), {}).create();

            case MouseCursor::WaitCursor:                    shape = XC_watch; break;
            case MouseCursor::IBeamCursor:                   shape = XC_xterm; break;
            case MouseCursor::PointingHandCursor:            shape = XC_hand2; break;
            case MouseCursor::LeftRightResizeCursor:         shape = XC_sb_h_double_arrow; break;
            case MouseCursor::UpDownResizeCursor:            shape = XC_sb_v_double_arrow; break;
            case MouseCursor::UpDownLeftRightResizeCursor:   shape = XC_fleur; break;
            case MouseCursor::TopEdgeResizeCursor:           shape = XC_top_side; break;
            case MouseCursor::BottomEdgeResizeCursor:        shape = XC_bottom_side; break;
            case MouseCursor::LeftEdgeResizeCursor:          shape = XC_left_side; break;
            case MouseCursor::RightEdgeResizeCursor:         shape = XC_right_side; break;
            case MouseCursor::TopLeftCornerResizeCursor:     shape = XC_top_left_corner; break;
            case MouseCursor::TopRightCornerResizeCursor:    shape = XC_top_right_corner; break;
            case MouseCursor::BottomLeftCornerResizeCursor:  shape = XC_bottom_left_corner; break;
            case MouseCursor::BottomRightCornerResizeCursor: shape = XC_bottom_right_corner; break;
            case MouseCursor::CrosshairCursor:               shape = XC_crosshair; break;
            case MouseCursor::DraggingHandCursor:            return createDraggingHandCursor();

            case MouseCursor::CopyingCursor:
            {
                constexpr unsigned char copyCursorData[] = {
                    71,73,70,56,57,97,21,0,21,0,145,0,0,0,0,0,255,255,255,0,128,128,255,255,255,33,249,4,1,0,0,3,0,44,0,0,0,0,
                    21,0,21,0,0,2,72,4,134,169,171,16,199,98,11,79,90,71,161,93,56,111,78,133,218,215,137,31,82,154,100,200,
                    86,91,202,142,12,108,212,87,235,174,15,54,214,126,237,226,37,96,59,141,16,37,18,201,142,157,230,204,51,112,
                    252,114,147,74,83,5,50,68,147,208,217,16,71,149,252,124,5,0,59,0,0
                };

                return CustomMouseCursorInfo (ImageFileFormat::loadFrom (copyCursorData, (size_t) numElementsInArray (copyCursorData)), { 1, 3 }).create();
            }

            case MouseCursor::NumStandardCursorTypes:
            default:
            {
                jassertfalse;
                return None;
            }
        }

        XWindowSystemUtilities::ScopedXLock xLock;

        return (void*) X11Symbols::getInstance()->xCreateFontCursor (display, shape);
        */
    }
    
    pub fn show_cursor(&self, 
        windowh:       Window,
        cursor_handle: *mut c_void)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;
        X11Symbols::getInstance()->xDefineCursor (display, windowH, (Cursor) cursorHandle);
        */
    }
    
    pub fn is_key_currently_down(&self, key_code: i32) -> bool {
        
        todo!();
        /*
            int keysym;

        if (keyCode & Keys::extendedKeyModifier)
        {
            keysym = 0xff00 | (keyCode & 0xff);
        }
        else
        {
            keysym = keyCode;

            if (keysym == (XK_Tab & 0xff)
                || keysym == (XK_Return & 0xff)
                || keysym == (XK_Escape & 0xff)
                || keysym == (XK_BackSpace & 0xff))
            {
                keysym |= 0xff00;
            }
        }

        XWindowSystemUtilities::ScopedXLock xLock;

        auto keycode = X11Symbols::getInstance()->xKeysymToKeycode (display, (KeySym) keysym);
        auto keybyte = keycode >> 3;
        auto keybit = (1 << (keycode & 7));

        return (Keys::keyStates [keybyte] & keybit) != 0;
        */
    }
    
    pub fn get_native_realtime_modifiers(&self) -> ModifierKeys {
        
        todo!();
        /*
            Window root, child;
        int x, y, winx, winy;
        unsigned int mask;
        int mouseMods = 0;

        XWindowSystemUtilities::ScopedXLock xLock;

        if (X11Symbols::getInstance()->xQueryPointer (display,
                                                      X11Symbols::getInstance()->xRootWindow (display,
                                                                                              X11Symbols::getInstance()->xDefaultScreen (display)),
                                                      &root, &child, &x, &y, &winx, &winy, &mask) != False)
        {
            if ((mask & Button1Mask) != 0)  mouseMods |= ModifierKeys::leftButtonModifier;
            if ((mask & Button2Mask) != 0)  mouseMods |= ModifierKeys::middleButtonModifier;
            if ((mask & Button3Mask) != 0)  mouseMods |= ModifierKeys::rightButtonModifier;
        }

        ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withoutMouseButtons().withFlags (mouseMods);

        return ModifierKeys::currentModifiers;
        */
    }
    
    pub fn find_displays(&self, master_scale: f32) -> Vec<Display> {
        
        todo!();
        /*
            Vec<Displays::Display> displays;
        auto workAreaHints = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_NET_WORKAREA");

       #if ALOE_USE_XRANDR
        if (workAreaHints != None)
        {
            int major_opcode, first_event, first_error;

            if (X11Symbols::getInstance()->xQueryExtension (display, "RANDR", &major_opcode, &first_event, &first_error))
            {
                auto numMonitors = X11Symbols::getInstance()->xScreenCount (display);
                auto mainDisplay = X11Symbols::getInstance()->xRRGetOutputPrimary (display, X11Symbols::getInstance()->xRootWindow (display, 0));

                for (int i = 0; i < numMonitors; ++i)
                {
                    auto rootWindow = X11Symbols::getInstance()->xRootWindow (display, i);
                    XWindowSystemUtilities::GetXProperty prop (rootWindow, workAreaHints, 0, 4, false, XA_CARDINAL);

                    if (! hasWorkAreaData (prop))
                        continue;

                    if (auto screens = makeDeletedPtr (X11Symbols::getInstance()->xRRGetScreenResources (display, rootWindow),
                                                       [] (XRRScreenResources* srs) { X11Symbols::getInstance()->xRRFreeScreenResources (srs); }))
                    {
                        for (int j = 0; j < screens->noutput; ++j)
                        {
                            if (screens->outputs[j])
                            {
                                // Xrandr on the raspberry pi fails to determine the main display (mainDisplay == 0)!
                                // Detect this edge case and make the first found display the main display
                                if (! mainDisplay)
                                    mainDisplay = screens->outputs[j];

                                if (auto output = makeDeletedPtr (X11Symbols::getInstance()->xRRGetOutputInfo (display, screens.get(), screens->outputs[j]),
                                                                  [] (XRROutputInfo* oi) { X11Symbols::getInstance()->xRRFreeOutputInfo (oi); }))
                                {
                                    if (output->crtc)
                                    {
                                        if (auto crtc = makeDeletedPtr (X11Symbols::getInstance()->xRRGetCrtcInfo (display, screens.get(), output->crtc),
                                                                        [] (XRRCrtcInfo* ci) { X11Symbols::getInstance()->xRRFreeCrtcInfo (ci); }))
                                        {
                                            Displays::Display d;
                                            d.totalArea = { crtc->x, crtc->y, (int) crtc->width, (int) crtc->height };
                                            d.isMain = (mainDisplay == screens->outputs[j]) && (i == 0);
                                            d.dpi = DisplayHelpers::getDisplayDPI (display, 0);

                                            // The raspberry pi returns a zero sized display, so we need to guard for divide-by-zero
                                            if (output->mm_width > 0 && output->mm_height > 0)
                                                d.dpi = ((static_cast<double> (crtc->width)  * 25.4 * 0.5) / static_cast<double> (output->mm_width))
                                                      + ((static_cast<double> (crtc->height) * 25.4 * 0.5) / static_cast<double> (output->mm_height));

                                            auto scale = DisplayHelpers::getDisplayScale (output->name, d.dpi);
                                            scale = (scale <= 0.1 || ! ALOEApplicationBase::isStandaloneApp()) ? 1.0 : scale;

                                            d.scale = masterScale * scale;

                                            if (d.isMain)
                                                displays.insert (0, d);
                                            else
                                                displays.add (d);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if (! displays.isEmpty() && ! displays.getReference (0).isMain)
                    displays.getReference (0).isMain = true;
            }
        }

        if (displays.isEmpty())
       #endif
       #if ALOE_USE_XINERAMA
        {
            auto screens = DisplayHelpers::xineramaQueryDisplays (display);
            auto numMonitors = screens.size();

            for (int index = 0; index < numMonitors; ++index)
            {
                for (auto j = numMonitors; --j >= 0;)
                {
                    if (screens[j].screen_number == index)
                    {
                        Displays::Display d;
                        d.totalArea = { screens[j].x_org, screens[j].y_org,
                                        screens[j].width, screens[j].height };
                        d.isMain = (index == 0);
                        d.scale = masterScale;
                        d.dpi = DisplayHelpers::getDisplayDPI (display, 0); // (all screens share the same DPI)

                        displays.add (d);
                    }
                }
            }
        }

        if (displays.isEmpty())
       #endif
        {
            if (workAreaHints != None)
            {
                auto numMonitors = X11Symbols::getInstance()->xScreenCount (display);

                for (int i = 0; i < numMonitors; ++i)
                {
                    XWindowSystemUtilities::GetXProperty prop (X11Symbols::getInstance()->xRootWindow (display, i),
                                                               workAreaHints, 0, 4, false, XA_CARDINAL);

                    auto workArea = getWorkArea (prop);

                    if (! workArea.isEmpty())
                    {
                        Displays::Display d;

                        d.totalArea = workArea;
                        d.isMain = displays.isEmpty();
                        d.scale = masterScale;
                        d.dpi = DisplayHelpers::getDisplayDPI (display, i);

                        displays.add (d);
                    }
                }
            }

            if (displays.isEmpty())
            {
                Displays::Display d;
                d.totalArea = { X11Symbols::getInstance()->xDisplayWidth  (display, X11Symbols::getInstance()->xDefaultScreen (display)),
                                X11Symbols::getInstance()->xDisplayHeight (display, X11Symbols::getInstance()->xDefaultScreen (display)) };
                d.isMain = true;
                d.scale = masterScale;
                d.dpi = DisplayHelpers::getDisplayDPI (display, 0);

                displays.add (d);
            }
        }

        for (auto& d : displays)
            d.userArea = d.totalArea; // Aloe currently does not support requesting the user area on Linux

        return displays;
        */
    }
    
    pub fn create_key_proxy(&self, windowh: Window) -> Window {
        
        todo!();
        /*
            jassert (windowH != 0);

        XSetWindowAttributes swa;
        swa.event_mask = KeyPressMask | KeyReleaseMask | FocusChangeMask;

        auto keyProxy = X11Symbols::getInstance()->xCreateWindow (display, windowH,
                                                                  -1, -1, 1, 1, 0, 0,
                                                                  InputOnly, CopyFromParent,
                                                                  CWEventMask,
                                                                  &swa);

        X11Symbols::getInstance()->xMapWindow (display, keyProxy);
        X11Symbols::getInstance()->xSaveContext (display, (XID) keyProxy, windowHandleXContext, (XPointer) this);

        return keyProxy;
        */
    }
    
    pub fn delete_key_proxy(&self, key_proxy: Window)  {
        
        todo!();
        /*
            jassert (keyProxy != 0);

        XPointer handlePointer;

        if (! X11Symbols::getInstance()->xFindContext (display, (XID) keyProxy, windowHandleXContext, &handlePointer))
              X11Symbols::getInstance()->xDeleteContext (display, (XID) keyProxy, windowHandleXContext);

        X11Symbols::getInstance()->xDestroyWindow (display, keyProxy);
        X11Symbols::getInstance()->xSync (display, false);

        XEvent event;
        while (X11Symbols::getInstance()->xCheckWindowEvent (display, keyProxy, getAllEventsMask (false), &event) == True)
        {}
        */
    }
    
    pub fn external_drag_file_init(&self, 
        peer:     *mut LinuxComponentPeer,
        files:    &StringArray,
        _2:       bool,
        callback: fn() -> ()) -> bool {
        
        todo!();
        /*
            auto& dragState = dragAndDropStateMap[peer];

        if (dragState.isDragging())
            return false;

        StringArray uriList;

        for (auto& f : files)
        {
            if (f.matchesWildcard ("?\*:\/\/\*", false))
                uriList.add (f);
            else
                uriList.add ("file://" + f);
        }

        return dragState.externalDragInit ((Window) peer->getNativeHandle(), false, uriList.joinIntoString ("\r\n"), std::move (callback));
        */
    }
    
    pub fn external_drag_text_init(&self, 
        peer:     *mut LinuxComponentPeer,
        text:     &String,
        callback: fn() -> ()) -> bool {
        
        todo!();
        /*
            auto& dragState = dragAndDropStateMap[peer];

        if (dragState.isDragging())
            return false;

        return dragState.externalDragInit ((Window) peer->getNativeHandle(), true, text, std::move (callback));
        */
    }
    
    pub fn copy_text_to_clipboard(&mut self, clip_text: &String)  {
        
        todo!();
        /*
            localClipboardContent = clipText;

        X11Symbols::getInstance()->xSetSelectionOwner (display, XA_PRIMARY,      aloe_messageWindowHandle, CurrentTime);
        X11Symbols::getInstance()->xSetSelectionOwner (display, atoms.clipboard, aloe_messageWindowHandle, CurrentTime);
        */
    }
    
    pub fn get_text_from_clipboard(&self) -> String {
        
        todo!();
        /*
            /* 1) try to read from the "CLIPBOARD" selection first (the "high
           level" clipboard that is supposed to be filled by ctrl-C
           etc). When a clipboard manager is running, the content of this
           selection is preserved even when the original selection owner
           exits.

           2) and then try to read from "PRIMARY" selection (the "legacy" selection
           filled by good old x11 apps such as xterm)
        */

        auto getContentForSelection = [this] (Atom selectionAtom) -> String
        {
            auto selectionOwner = X11Symbols::getInstance()->xGetSelectionOwner (display, selectionAtom);

            if (selectionOwner == None)
                return {};

            if (selectionOwner == aloe_messageWindowHandle)
                return localClipboardContent;

            String content;

            if (! ClipboardHelpers::requestSelectionContent (display, content, selectionAtom, atoms.utf8String))
                ClipboardHelpers::requestSelectionContent (display, content, selectionAtom, XA_STRING);

            return content;
        };

        auto content = getContentForSelection (atoms.clipboard);

        if (content.isEmpty())
            content = getContentForSelection (XA_PRIMARY);

        return content;
        */
    }
    
    pub fn is_parent_window_of(&self, 
        windowh:        Window,
        possible_child: Window) -> bool {
        
        todo!();
        /*
            if (windowH == 0 || possibleChild == 0)
            return false;

        if (possibleChild == windowH)
            return true;

        Window* windowList = nullptr;
        uint32 windowListSize = 0;
        Window parent, root;

        XWindowSystemUtilities::ScopedXLock xLock;
        const auto result = X11Symbols::getInstance()->xQueryTree (display, possibleChild, &root, &parent, &windowList, &windowListSize);
        const auto deleter = makeXFreePtr (windowList);

        if (result == 0 || parent == root)
            return false;

        return isParentWindowOf (windowH, parent);
        */
    }
    
    pub fn is_front_window(&self, windowh: Window) -> bool {
        
        todo!();
        /*
            jassert (windowH != 0);

        Window* windowList = nullptr;
        uint32 windowListSize = 0;

        XWindowSystemUtilities::ScopedXLock xLock;
        Window parent;
        auto root = X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display));

        const auto queryResult = X11Symbols::getInstance()->xQueryTree (display, root, &root, &parent, &windowList, &windowListSize);
        const auto deleter = makeXFreePtr (windowList);

        if (queryResult == 0)
            return false;

        for (int i = (int) windowListSize; --i >= 0;)
        {
            if (auto* peer = dynamic_cast<LinuxComponentPeer*> (getPeerFor (windowList[i])))
                return peer == dynamic_cast<LinuxComponentPeer*> (getPeerFor (windowH));
        }

        return false;
        */
    }
    
    pub fn xchange_property(&self, 
        windowh:      Window,
        property:     Atom,
        ty:           Atom,
        format:       i32,
        data:         *const c_void,
        num_elements: i32)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        X11Symbols::getInstance()->xChangeProperty (display, windowH, property, type, format, PropModeReplace, (const unsigned char*) data, numElements);
        */
    }
    
    pub fn remove_window_decorations(&self, windowh: Window)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        auto hints = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_MOTIF_WM_HINTS");

        if (hints != None)
        {
            MotifWmHints motifHints;
            zerostruct (motifHints);

            motifHints.flags = 2; /* MWM_HINTS_DECORATIONS */
            motifHints.decorations = 0;

            XWindowSystemUtilities::ScopedXLock xLock;
            xchangeProperty (windowH, hints, hints, 32, &motifHints, 4);
        }

        hints = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_WIN_HINTS");

        if (hints != None)
        {
            long gnomeHints = 0;

            XWindowSystemUtilities::ScopedXLock xLock;
            xchangeProperty (windowH, hints, hints, 32, &gnomeHints, 1);
        }

        hints = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "KWM_WIN_DECORATION");

        if (hints != None)
        {
            long kwmHints = 2; /*KDE_tinyDecoration*/

            XWindowSystemUtilities::ScopedXLock xLock;
            xchangeProperty (windowH, hints, hints, 32, &kwmHints, 1);
        }

        hints = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_KDE_NET_WM_WINDOW_TYPE_OVERRIDE");

        if (hints != None)
        {
            XWindowSystemUtilities::ScopedXLock xLock;
            xchangeProperty (windowH, atoms.windowType, XA_ATOM, 32, &hints, 1);
        }
        */
    }
    
    pub fn add_window_buttons(&self, 
        windowh:     Window,
        style_flags: i32)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;
        auto motifAtom = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_MOTIF_WM_HINTS");

        if (motifAtom != None)
        {
            MotifWmHints motifHints;
            zerostruct (motifHints);

            motifHints.flags = 1 | 2; /* MWM_HINTS_FUNCTIONS | MWM_HINTS_DECORATIONS */
            motifHints.decorations = 2 /* MWM_DECOR_BORDER */ | 8 /* MWM_DECOR_TITLE */ | 16; /* MWM_DECOR_MENU */

            motifHints.functions = 4 /* MWM_FUNC_MOVE */;

            if ((styleFlags & ComponentPeer::windowHasCloseButton) != 0)
                motifHints.functions |= 32; /* MWM_FUNC_CLOSE */

            if ((styleFlags & ComponentPeer::windowHasMinimiseButton) != 0)
            {
                motifHints.functions |= 8; /* MWM_FUNC_MINIMIZE */
                motifHints.decorations |= 0x20; /* MWM_DECOR_MINIMIZE */
            }

            if ((styleFlags & ComponentPeer::windowHasMaximiseButton) != 0)
            {
                motifHints.functions |= 0x10; /* MWM_FUNC_MAXIMIZE */
                motifHints.decorations |= 0x40; /* MWM_DECOR_MAXIMIZE */
            }

            if ((styleFlags & ComponentPeer::windowIsResizable) != 0)
            {
                motifHints.functions |= 2; /* MWM_FUNC_RESIZE */
                motifHints.decorations |= 0x4; /* MWM_DECOR_RESIZEH */
            }

            xchangeProperty (windowH, motifAtom, motifAtom, 32, &motifHints, 5);
        }

        auto actionsAtom = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_NET_WM_ALLOWED_ACTIONS");

        if (actionsAtom != None)
        {
            std::vector<Atom> netHints;

            addAtomIfExists ((styleFlags & ComponentPeer::windowIsResizable)       != 0, "_NET_WM_ACTION_RESIZE",     display, netHints);
            addAtomIfExists ((styleFlags & ComponentPeer::windowHasMaximiseButton) != 0, "_NET_WM_ACTION_FULLSCREEN", display, netHints);
            addAtomIfExists ((styleFlags & ComponentPeer::windowHasMinimiseButton) != 0, "_NET_WM_ACTION_MINIMIZE",   display, netHints);
            addAtomIfExists ((styleFlags & ComponentPeer::windowHasCloseButton)    != 0, "_NET_WM_ACTION_CLOSE",      display, netHints);

            auto numHints = (int) netHints.size();

            if (numHints > 0)
                xchangeProperty (windowH, actionsAtom, XA_ATOM, 32, netHints.data(), numHints);
        }
        */
    }
    
    pub fn set_window_type(&self, 
        windowh:     Window,
        style_flags: i32)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        if (atoms.windowType != None)
        {
            auto hint = (styleFlags & ComponentPeer::windowIsTemporary) != 0
                        || ((styleFlags & ComponentPeer::windowHasDropShadow) == 0 && Desktop::canUseSemiTransparentWindows())
                            ? XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_NET_WM_WINDOW_TYPE_COMBO")
                            : XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, "_NET_WM_WINDOW_TYPE_NORMAL");

            if (hint != None)
                xchangeProperty (windowH, atoms.windowType, XA_ATOM, 32, &hint, 1);
        }

        if (atoms.windowState != None)
        {
            std::vector<Atom> netStateHints;

            addAtomIfExists ((styleFlags & ComponentPeer::windowAppearsOnTaskbar) == 0, "_NET_WM_STATE_SKIP_TASKBAR", display, netStateHints);
            addAtomIfExists (getPeerFor (windowH)->getComponent().isAlwaysOnTop(),      "_NET_WM_STATE_ABOVE",        display, netStateHints);

            auto numHints = (int) netStateHints.size();

            if (numHints > 0)
                xchangeProperty (windowH, atoms.windowState, XA_ATOM, 32, netStateHints.data(), numHints);
        }
        */
    }
    
    pub fn initialise_pointer_map(&mut self)  {
        
        todo!();
        /*
            auto numButtons = X11Symbols::getInstance()->xGetPointerMapping (display, nullptr, 0);
        pointerMap[2] = pointerMap[3] = pointerMap[4] = Keys::NoButton;

        if (numButtons == 2)
        {
            pointerMap[0] = Keys::LeftButton;
            pointerMap[1] = Keys::RightButton;
        }
        else if (numButtons >= 3)
        {
            pointerMap[0] = Keys::LeftButton;
            pointerMap[1] = Keys::MiddleButton;
            pointerMap[2] = Keys::RightButton;

            if (numButtons >= 5)
            {
                pointerMap[3] = Keys::WheelUp;
                pointerMap[4] = Keys::WheelDown;
            }
        }
        */
    }
    
    pub fn delete_icon_pixmaps(&self, windowh: Window)  {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::ScopedXLock xLock;

        if (auto wmHints = makeXFreePtr (X11Symbols::getInstance()->xGetWMHints (display, windowH)))
        {
            if ((wmHints->flags & IconPixmapHint) != 0)
            {
                wmHints->flags &= ~IconPixmapHint;
                X11Symbols::getInstance()->xFreePixmap (display, wmHints->icon_pixmap);
            }

            if ((wmHints->flags & IconMaskHint) != 0)
            {
                wmHints->flags &= ~IconMaskHint;
                X11Symbols::getInstance()->xFreePixmap (display, wmHints->icon_mask);
            }

            X11Symbols::getInstance()->xSetWMHints (display, windowH, wmHints.get());
        }
        */
    }

    /**
      | Alt and Num lock are not defined by standard
      | X modifier constants: check what they're mapped
      | to
      */
    pub fn update_modifier_mappings(&self)  {
        
        todo!();
        /*
            XWindowSystemUtilities::ScopedXLock xLock;
        auto altLeftCode = X11Symbols::getInstance()->xKeysymToKeycode (display, XK_Alt_L);
        auto numLockCode = X11Symbols::getInstance()->xKeysymToKeycode (display, XK_Num_Lock);

        Keys::AltMask = 0;
        Keys::NumLockMask = 0;

        if (auto mapping = makeDeletedPtr (X11Symbols::getInstance()->xGetModifierMapping (display),
                                           [] (XModifierKeymap* mk) { X11Symbols::getInstance()->xFreeModifiermap (mk); }))
        {
            for (int modifierIdx = 0; modifierIdx < 8; ++modifierIdx)
            {
                for (int keyIndex = 0; keyIndex < mapping->max_keypermod; ++keyIndex)
                {
                    auto key = mapping->modifiermap[(modifierIdx * mapping->max_keypermod) + keyIndex];

                    if (key == altLeftCode)
                        Keys::AltMask = 1 << modifierIdx;
                    else if (key == numLockCode)
                        Keys::NumLockMask = 1 << modifierIdx;
                }
            }
        }
        */
    }
    
    pub fn get_user_time(&self, windowh: Window) -> i64 {
        
        todo!();
        /*
            jassert (windowH != 0);

        XWindowSystemUtilities::GetXProperty prop (windowH, atoms.userTime, 0, 65536, false, XA_CARDINAL);

        if (! prop.success)
            return 0;

        long result = 0;
        std::memcpy (&result, prop.data, sizeof (long));

        return result;
        */
    }
    
    pub fn initialise_xdisplay(&mut self) -> bool {
        
        todo!();
        /*
            jassert (display == nullptr);

        String displayName (getenv ("DISPLAY"));

        if (displayName.isEmpty())
            displayName = ":0.0";

        // it seems that on some systems XOpenDisplay will occasionally
        // fail the first time, but succeed on a second attempt..
        for (int retries = 2; --retries >= 0;)
        {
            display = X11Symbols::getInstance()->xOpenDisplay (displayName.toUTF8());

            if (display != nullptr)
                break;
        }

        // No X Server running
        if (display == nullptr)
            return false;

       #if ALOE_DEBUG_XERRORS_SYNCHRONOUSLY
        X11Symbols::getInstance()->xSynchronize (display, True);
       #endif

        // Create a context to store user data associated with Windows we create
        windowHandleXContext = (XContext) X11Symbols::getInstance()->xrmUniqueQuark();

        // Create our message window (this will never be mapped)
        auto screen = X11Symbols::getInstance()->xDefaultScreen (display);
        auto root = X11Symbols::getInstance()->xRootWindow (display, screen);
        X11Symbols::getInstance()->xSelectInput (display, root, SubstructureNotifyMask);

        // We're only interested in client messages for this window, which are always sent
        XSetWindowAttributes swa;
        swa.event_mask = NoEventMask;
        aloe_messageWindowHandle = X11Symbols::getInstance()->xCreateWindow (display, root,
                                                                             0, 0, 1, 1, 0, 0, InputOnly,
                                                                             X11Symbols::getInstance()->xDefaultVisual (display, screen),
                                                                             CWEventMask, &swa);

        X11Symbols::getInstance()->xSync (display, False);

        atoms = XWindowSystemUtilities::XWindowSystemAtoms (display);

        initialisePointerMap();
        updateModifierMappings();

       #if ALOE_USE_XSHM
        if (XSHMHelpers::isShmAvailable (display))
            shmCompletionEvent = X11Symbols::getInstance()->xShmGetEventBase (display) + ShmCompletion;
       #endif

        displayVisuals = std::make_unique<XWindowSystemDisplayVisuals> (display);

        if (! displayVisuals->isValid())
        {
            Logger::outputDebugString ("ERROR: System doesn't support 32, 24 or 16 bit RGB display.\n");
            return false;
        }

        // Setup input event handler
        LinuxEventLoop::registerFdCallback (X11Symbols::getInstance()->xConnectionNumber (display),
                                            [this] (int)
                                            {
                                                do
                                                {
                                                    XEvent evt;

                                                    {
                                                        XWindowSystemUtilities::ScopedXLock xLock;

                                                        if (! X11Symbols::getInstance()->xPending (display))
                                                            return;

                                                        X11Symbols::getInstance()->xNextEvent (display, &evt);
                                                    }

                                                    if (evt.type == SelectionRequest && evt.xany.window == aloe_messageWindowHandle)
                                                    {
                                                        ClipboardHelpers::handleSelection (evt.xselectionrequest);
                                                    }
                                                    else if (evt.xany.window != aloe_messageWindowHandle)
                                                    {
                                                        windowMessageReceive (evt);
                                                    }

                                                } while (display != nullptr);
                                            });

        return true;
        */
    }
    
    pub fn destroy_xdisplay(&mut self)  {
        
        todo!();
        /*
            if (xIsAvailable)
        {
            jassert (display != nullptr);

            XWindowSystemUtilities::ScopedXLock xLock;

            X11Symbols::getInstance()->xDestroyWindow (display, aloe_messageWindowHandle);
            aloe_messageWindowHandle = 0;
            X11Symbols::getInstance()->xSync (display, True);

            LinuxEventLoop::unregisterFdCallback (X11Symbols::getInstance()->xConnectionNumber (display));

            X11Symbols::getInstance()->xCloseDisplay (display);
            display = nullptr;
            displayVisuals = nullptr;
        }
        */
    }
    
    pub fn handle_window_message(&self, 
        peer:  *mut LinuxComponentPeer,
        event: &mut XEvent)  {
        
        todo!();
        /*
            switch (event.xany.type)
        {
            case KeyPressEventType:     handleKeyPressEvent        (peer, event.xkey);                     break;
            case KeyRelease:            handleKeyReleaseEvent      (peer, event.xkey);                     break;
            case ButtonPress:           handleButtonPressEvent     (peer, event.xbutton);                  break;
            case ButtonRelease:         handleButtonReleaseEvent   (peer, event.xbutton);                  break;
            case MotionNotify:          handleMotionNotifyEvent    (peer, event.xmotion);                  break;
            case EnterNotify:           handleEnterNotifyEvent     (peer, event.xcrossing);                break;
            case LeaveNotify:           handleLeaveNotifyEvent     (peer, event.xcrossing);                break;
            case FocusIn:               handleFocusInEvent         (peer);                                 break;
            case FocusOut:              handleFocusOutEvent        (peer);                                 break;
            case Expose:                handleExposeEvent          (peer, event.xexpose);                  break;
            case MappingNotify:         handleMappingNotify        (event.xmapping);                       break;
            case ClientMessage:         handleClientMessageEvent   (peer, event.xclient, event);           break;
            case SelectionNotify:       dragAndDropStateMap[peer].handleDragAndDropSelection (event);      break;
            case ConfigureNotify:       handleConfigureNotifyEvent (peer, event.xconfigure);               break;
            case ReparentNotify:
            case GravityNotify:         handleGravityNotify (peer);                                        break;
            case SelectionClear:        dragAndDropStateMap[peer].handleExternalSelectionClear();          break;
            case SelectionRequest:      dragAndDropStateMap[peer].handleExternalSelectionRequest (event);  break;
            case PropertyNotify:        propertyNotifyEvent        (peer, event.xproperty);                break;

            case CirculateNotify:
            case CreateNotify:
            case DestroyNotify:
            case UnmapNotify:
                break;

            case MapNotify:
                peer->handleBroughtToFront();
                break;

            default:
               #if ALOE_USE_XSHM
                if (XSHMHelpers::isShmAvailable (display))
                {
                    XWindowSystemUtilities::ScopedXLock xLock;

                    if (event.xany.type == shmCompletionEvent)
                        XWindowSystem::getInstance()->removePendingPaintForWindow ((Window) peer->getNativeHandle());
                }
               #endif
                break;
        }
        */

    }
    
    pub fn handle_key_press_event(&self, 
        peer:      *mut LinuxComponentPeer,
        key_event: &mut XKeyEvent)  {
        
        todo!();
        /*
            auto oldMods = ModifierKeys::currentModifiers;

        char utf8 [64] = { 0 };
        aloe_wchar unicodeChar = 0;
        int keyCode = 0;
        bool keyDownChange = false;
        KeySym sym;

        {
            XWindowSystemUtilities::ScopedXLock xLock;
            updateKeyStates ((int) keyEvent.keycode, true);

            String oldLocale (::setlocale (LC_ALL, nullptr));
            ::setlocale (LC_ALL, "");
            X11Symbols::getInstance()->xLookupString (&keyEvent, utf8, sizeof (utf8), &sym, nullptr);

            if (oldLocale.isNotEmpty())
                ::setlocale (LC_ALL, oldLocale.toRawUTF8());

            unicodeChar = *CharPointer_UTF8 (utf8);
            keyCode = (int) unicodeChar;

            if (keyCode < 0x20)
                keyCode = (int) X11Symbols::getInstance()->xkbKeycodeToKeysym (display, (::KeyCode) keyEvent.keycode, 0,
                                                                               ModifierKeys::currentModifiers.isShiftDown() ? 1 : 0);

            keyDownChange = (sym != NoSymbol) && ! updateKeyModifiersFromSym (sym, true);
        }

        bool keyPressed = false;

        if ((sym & 0xff00) == 0xff00 || keyCode == XK_ISO_Left_Tab)
        {
            switch (sym)  // Translate keypad
            {
                case XK_KP_Add:         keyCode = XK_plus;      break;
                case XK_KP_Subtract:    keyCode = XK_hyphen;    break;
                case XK_KP_Divide:      keyCode = XK_slash;     break;
                case XK_KP_Multiply:    keyCode = XK_asterisk;  break;
                case XK_KP_Enter:       keyCode = XK_Return;    break;
                case XK_KP_Insert:      keyCode = XK_Insert;    break;
                case XK_Delete:
                case XK_KP_Delete:      keyCode = XK_Delete;    break;
                case XK_KP_Left:        keyCode = XK_Left;      break;
                case XK_KP_Right:       keyCode = XK_Right;     break;
                case XK_KP_Up:          keyCode = XK_Up;        break;
                case XK_KP_Down:        keyCode = XK_Down;      break;
                case XK_KP_Home:        keyCode = XK_Home;      break;
                case XK_KP_End:         keyCode = XK_End;       break;
                case XK_KP_Page_Down:   keyCode = XK_Page_Down; break;
                case XK_KP_Page_Up:     keyCode = XK_Page_Up;   break;

                case XK_KP_0:           keyCode = XK_0;         break;
                case XK_KP_1:           keyCode = XK_1;         break;
                case XK_KP_2:           keyCode = XK_2;         break;
                case XK_KP_3:           keyCode = XK_3;         break;
                case XK_KP_4:           keyCode = XK_4;         break;
                case XK_KP_5:           keyCode = XK_5;         break;
                case XK_KP_6:           keyCode = XK_6;         break;
                case XK_KP_7:           keyCode = XK_7;         break;
                case XK_KP_8:           keyCode = XK_8;         break;
                case XK_KP_9:           keyCode = XK_9;         break;

                default:                break;
            }

            switch (keyCode)
            {
                case XK_Left:
                case XK_Right:
                case XK_Up:
                case XK_Down:
                case XK_Page_Up:
                case XK_Page_Down:
                case XK_End:
                case XK_Home:
                case XK_Delete:
                case XK_Insert:
                    keyPressed = true;
                    keyCode = (keyCode & 0xff) | Keys::extendedKeyModifier;
                    break;

                case XK_Tab:
                case XK_Return:
                case XK_Escape:
                case XK_BackSpace:
                    keyPressed = true;
                    keyCode &= 0xff;
                    break;

                case XK_ISO_Left_Tab:
                    keyPressed = true;
                    keyCode = XK_Tab & 0xff;
                    break;

                default:
                    if (sym >= XK_F1 && sym <= XK_F35)
                    {
                        keyPressed = true;
                        keyCode = static_cast<int> ((sym & 0xff) | Keys::extendedKeyModifier);
                    }
                    break;
            }
        }

        if (utf8[0] != 0 || ((sym & 0xff00) == 0 && sym >= 8))
            keyPressed = true;

        if (oldMods != ModifierKeys::currentModifiers)
            peer->handleModifierKeysChange();

        if (keyDownChange)
            peer->handleKeyUpOrDown (true);

        if (keyPressed)
            peer->handleKeyPress (keyCode, unicodeChar);
        */

    }
    
    pub fn handle_key_release_event(&self, 
        peer:      *mut LinuxComponentPeer,
        key_event: &XKeyEvent)  {
        
        todo!();
        /*
            auto isKeyReleasePartOfAutoRepeat = [&]() -> bool
        {
            if (X11Symbols::getInstance()->xPending (display))
            {
                XEvent e;
                X11Symbols::getInstance()->xPeekEvent (display, &e);

                // Look for a subsequent key-down event with the same timestamp and keycode
                return e.type           == KeyPressEventType
                      && e.xkey.keycode == keyEvent.keycode
                      && e.xkey.time    == keyEvent.time;
            }

            return false;
        }();

        if (! isKeyReleasePartOfAutoRepeat)
        {
            updateKeyStates ((int) keyEvent.keycode, false);
            KeySym sym;

            {
                XWindowSystemUtilities::ScopedXLock xLock;
                sym = X11Symbols::getInstance()->xkbKeycodeToKeysym (display, (::KeyCode) keyEvent.keycode, 0, 0);
            }

            auto oldMods = ModifierKeys::currentModifiers;
            auto keyDownChange = (sym != NoSymbol) && ! updateKeyModifiersFromSym (sym, false);

            if (oldMods != ModifierKeys::currentModifiers)
                peer->handleModifierKeysChange();

            if (keyDownChange)
                peer->handleKeyUpOrDown (false);
        }
        */

    }
    
    pub fn handle_wheel_event(&self, 
        peer:               *mut LinuxComponentPeer,
        button_press_event: &XButtonPressedEvent,
        amount:             f32)  {
        
        todo!();
        /*
            MouseWheelDetails wheel;
        wheel.deltaX = 0.0f;
        wheel.deltaY = amount;
        wheel.isReversed = false;
        wheel.isSmooth = false;
        wheel.isInertial = false;

        peer->handleMouseWheel (MouseInputSource::InputSourceType::mouse, getLogicalMousePos (buttonPressEvent, peer->getPlatformScaleFactor()),
                                getEventTime (buttonPressEvent), wheel);
        */

    }
    
    pub fn handle_button_press_event_with_flag(
        &self, 
        peer:                 *mut LinuxComponentPeer,
        button_press_event:   &XButtonPressedEvent,
        button_modifier_flag: i32
    ) {
        
        todo!();
        /*
            ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withFlags (buttonModifierFlag);
        peer->toFront (true);
        peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse, getLogicalMousePos (buttonPressEvent, peer->getPlatformScaleFactor()),
                                ModifierKeys::currentModifiers, MouseInputSource::invalidPressure,
                                MouseInputSource::invalidOrientation, getEventTime (buttonPressEvent), {});
        */

    }
    
    pub fn handle_button_press_event(&self, 
        peer:               *mut LinuxComponentPeer,
        button_press_event: &XButtonPressedEvent)  {
        
        todo!();
        /*
            updateKeyModifiers ((int) buttonPressEvent.state);

        auto mapIndex = (uint32) (buttonPressEvent.button - Button1);

        if (mapIndex < (uint32) numElementsInArray (pointerMap))
        {
            switch (pointerMap[mapIndex])
            {
                case Keys::WheelUp:         handleWheelEvent (peer, buttonPressEvent,  50.0f / 256.0f); break;
                case Keys::WheelDown:       handleWheelEvent (peer, buttonPressEvent, -50.0f / 256.0f); break;
                case Keys::LeftButton:      handleButtonPressEvent (peer, buttonPressEvent, ModifierKeys::leftButtonModifier); break;
                case Keys::RightButton:     handleButtonPressEvent (peer, buttonPressEvent, ModifierKeys::rightButtonModifier); break;
                case Keys::MiddleButton:    handleButtonPressEvent (peer, buttonPressEvent, ModifierKeys::middleButtonModifier); break;
                default: break;
            }
        }
        */

    }
    
    pub fn handle_button_release_event(&self, 
        peer:             *mut LinuxComponentPeer,
        button_rel_event: &XButtonReleasedEvent)  {
        
        todo!();
        /*
            updateKeyModifiers ((int) buttonRelEvent.state);

        if (peer->getParentWindow() != 0)
            peer->updateWindowBounds();

        auto mapIndex = (uint32) (buttonRelEvent.button - Button1);

        if (mapIndex < (uint32) numElementsInArray (pointerMap))
        {
            switch (pointerMap[mapIndex])
            {
                case Keys::LeftButton:      ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withoutFlags (ModifierKeys::leftButtonModifier);   break;
                case Keys::RightButton:     ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withoutFlags (ModifierKeys::rightButtonModifier);  break;
                case Keys::MiddleButton:    ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withoutFlags (ModifierKeys::middleButtonModifier); break;
                default: break;
            }
        }

        auto& dragState = dragAndDropStateMap[peer];

        if (dragState.isDragging())
            dragState.handleExternalDragButtonReleaseEvent();

        peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse, getLogicalMousePos (buttonRelEvent, peer->getPlatformScaleFactor()),
                                ModifierKeys::currentModifiers, MouseInputSource::invalidPressure, MouseInputSource::invalidOrientation, getEventTime (buttonRelEvent));
        */

    }
    
    pub fn handle_motion_notify_event(&self, 
        peer:        *mut LinuxComponentPeer,
        moved_event: &XPointerMovedEvent)  {
        
        todo!();
        /*
            updateKeyModifiers ((int) movedEvent.state);

        auto& dragState = dragAndDropStateMap[peer];

        if (dragState.isDragging())
            dragState.handleExternalDragMotionNotify();

        peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse, getLogicalMousePos (movedEvent, peer->getPlatformScaleFactor()),
                                ModifierKeys::currentModifiers, MouseInputSource::invalidPressure,
                                MouseInputSource::invalidOrientation, getEventTime (movedEvent));
        */

    }
    
    pub fn handle_enter_notify_event(&self, 
        peer:        *mut LinuxComponentPeer,
        enter_event: &XEnterWindowEvent)  {
        
        todo!();
        /*
            if (peer->getParentWindow() != 0)
            peer->updateWindowBounds();

        if (! ModifierKeys::currentModifiers.isAnyMouseButtonDown())
        {
            updateKeyModifiers ((int) enterEvent.state);
            peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse, getLogicalMousePos (enterEvent, peer->getPlatformScaleFactor()),
                                    ModifierKeys::currentModifiers, MouseInputSource::invalidPressure,
                                    MouseInputSource::invalidOrientation, getEventTime (enterEvent));
        }
        */

    }
    
    pub fn handle_leave_notify_event(&self, 
        peer:        *mut LinuxComponentPeer,
        leave_event: &XLeaveWindowEvent)  {
        
        todo!();
        /*
            // Suppress the normal leave if we've got a pointer grab, or if
        // it's a bogus one caused by clicking a mouse button when running
        // in a Window manager
        if (((! ModifierKeys::currentModifiers.isAnyMouseButtonDown()) && leaveEvent.mode == NotifyNormal)
             || leaveEvent.mode == NotifyUngrab)
        {
            updateKeyModifiers ((int) leaveEvent.state);
            peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse, getLogicalMousePos (leaveEvent, peer->getPlatformScaleFactor()),
                                    ModifierKeys::currentModifiers, MouseInputSource::invalidPressure,
                                    MouseInputSource::invalidOrientation, getEventTime (leaveEvent));
        }
        */

    }
    
    pub fn handle_focus_in_event(&self, peer: *mut LinuxComponentPeer)  {
        
        todo!();
        /*
            peer->isActiveApplication = true;

        if (isFocused ((Window) peer->getNativeHandle()) && ! peer->focused)
        {
            peer->focused = true;
            peer->handleFocusGain();
        }
        */

    }
    
    pub fn handle_focus_out_event(&self, peer: *mut LinuxComponentPeer)  {
        
        todo!();
        /*
            if (! isFocused ((Window) peer->getNativeHandle()) && peer->focused)
        {
            peer->focused = false;
            peer->isActiveApplication = false;

            peer->handleFocusLoss();
        }
        */

    }
    
    pub fn handle_expose_event(&self, 
        peer:         *mut LinuxComponentPeer,
        expose_event: &mut XExposeEvent)  {
        
        todo!();
        /*
            // Batch together all pending expose events
        XEvent nextEvent;
        XWindowSystemUtilities::ScopedXLock xLock;

        // if we have opengl contexts then just repaint them all
        // regardless if this is really necessary
        peer->repaintOpenGLContexts();

        auto windowH = (Window) peer->getNativeHandle();

        if (exposeEvent.window != windowH)
        {
            Window child;
            X11Symbols::getInstance()->xTranslateCoordinates (display, exposeEvent.window, windowH,
                                                              exposeEvent.x, exposeEvent.y, &exposeEvent.x, &exposeEvent.y,
                                                              &child);
        }

        // exposeEvent is in local window local coordinates so do not convert with
        // physicalToScaled, but rather use currentScaleFactor
        auto currentScaleFactor = peer->getPlatformScaleFactor();

        peer->repaint (Rectangle<int> (exposeEvent.x, exposeEvent.y,
                                       exposeEvent.width, exposeEvent.height) / currentScaleFactor);

        while (X11Symbols::getInstance()->xEventsQueued (display, QueuedAfterFlush) > 0)
        {
            X11Symbols::getInstance()->xPeekEvent (display, &nextEvent);

            if (nextEvent.type != Expose || nextEvent.xany.window != exposeEvent.window)
                break;

            X11Symbols::getInstance()->xNextEvent (display, &nextEvent);
            auto& nextExposeEvent = (XExposeEvent&) nextEvent.xexpose;

            peer->repaint (Rectangle<int> (nextExposeEvent.x, nextExposeEvent.y,
                                           nextExposeEvent.width, nextExposeEvent.height) / currentScaleFactor);
        }
        */

    }
    
    pub fn dismiss_blocking_modals(&self, peer: *mut LinuxComponentPeer)  {
        
        todo!();
        /*
            if (peer->getComponent().isCurrentlyBlockedByAnotherModalComponent())
            if (auto* currentModalComp = Component::getCurrentlyModalComponent())
                currentModalComp->inputAttemptWhenModal();
        */

    }
    
    pub fn handle_configure_notify_event(&self, 
        peer:       *mut LinuxComponentPeer,
        conf_event: &mut XConfigureEvent)  {
        
        todo!();
        /*
            peer->updateWindowBounds();
        peer->updateBorderSize();
        peer->handleMovedOrResized();

        // if the native title bar is dragged, need to tell any active menus, etc.
        if ((peer->getStyleFlags() & ComponentPeer::windowHasTitleBar) != 0)
            dismissBlockingModals (peer);

        auto windowH = (Window) peer->getNativeHandle();

        if (confEvent.window == windowH && confEvent.above != 0 && isFrontWindow (windowH))
            peer->handleBroughtToFront();
        */

    }
    
    pub fn handle_gravity_notify(&self, peer: *mut LinuxComponentPeer)  {
        
        todo!();
        /*
            peer->updateWindowBounds();
        peer->updateBorderSize();
        peer->handleMovedOrResized();
        */

    }
    
    pub fn property_notify_event(&self, 
        peer:  *mut LinuxComponentPeer,
        event: &XPropertyEvent)  {
        
        todo!();
        /*
            const auto isStateChangeEvent = [&]
        {
            if (event.atom != atoms.state)
                return false;

            return isMinimised (event.window);
        };

        const auto isHidden = [&]
        {
            if (event.atom != atoms.windowState)
                return false;

            XWindowSystemUtilities::ScopedXLock xLock;
            XWindowSystemUtilities::GetXProperty prop (event.window, atoms.windowState, 0, 128, false, XA_ATOM);

            if (! (prop.success && prop.actualFormat == 32 && prop.actualType == XA_ATOM))
                return false;

            const auto data = reinterpret_cast<const long*> (prop.data);
            const auto end = data + prop.numItems;

            return std::find (data, end, atoms.windowStateHidden) != end;
        };

        if (isStateChangeEvent() || isHidden())
            dismissBlockingModals (peer);
        */

    }
    
    pub fn handle_mapping_notify(&self, mapping_event: &mut XMappingEvent)  {
        
        todo!();
        /*
        if (mappingEvent.request != MappingPointer)
        {
            // Deal with modifier/keyboard mapping
            XWindowSystemUtilities::ScopedXLock xLock;
            X11Symbols::getInstance()->xRefreshKeyboardMapping (&mappingEvent);
            updateModifierMappings();
        }
        */

    }
    
    pub fn handle_client_message_event(&self, 
        peer:       *mut LinuxComponentPeer,
        client_msg: &mut XClientMessageEvent,
        event:      &mut XEvent)  {
        
        todo!();
        /*
            if (clientMsg.message_type == atoms.protocols && clientMsg.format == 32)
        {
            auto atom = (Atom) clientMsg.data.l[0];

            if (atom == atoms.protocolList [XWindowSystemUtilities::XWindowSystemAtoms::PING])
            {
                auto root = X11Symbols::getInstance()->xRootWindow (display, X11Symbols::getInstance()->xDefaultScreen (display));

                clientMsg.window = root;

                X11Symbols::getInstance()->xSendEvent (display, root, False, NoEventMask, &event);
                X11Symbols::getInstance()->xFlush (display);
            }
            else if (atom == atoms.protocolList [XWindowSystemUtilities::XWindowSystemAtoms::TAKE_FOCUS])
            {
                if ((peer->getStyleFlags() & ComponentPeer::windowIgnoresKeyPresses) == 0)
                {
                    XWindowAttributes atts;

                    XWindowSystemUtilities::ScopedXLock xLock;
                    if (clientMsg.window != 0
                         && X11Symbols::getInstance()->xGetWindowAttributes (display, clientMsg.window, &atts))
                    {
                        if (atts.map_state == IsViewable)
                        {
                            auto windowH = (Window) peer->getNativeHandle();

                            X11Symbols::getInstance()->xSetInputFocus (display, (clientMsg.window == windowH ? getFocusWindow (windowH)
                                                                                                             : clientMsg.window),
                                                                       RevertToParent, (Time) clientMsg.data.l[1]);
                        }
                    }
                }
            }
            else if (atom == atoms.protocolList [XWindowSystemUtilities::XWindowSystemAtoms::DELETE_WINDOW])
            {
                peer->handleUserClosingWindow();
            }
        }
        else if (clientMsg.message_type == atoms.XdndEnter)
        {
            dragAndDropStateMap[peer].handleDragAndDropEnter (clientMsg, peer);
        }
        else if (clientMsg.message_type == atoms.XdndLeave)
        {
            dragAndDropStateMap[peer].handleDragAndDropExit();
        }
        else if (clientMsg.message_type == atoms.XdndPosition)
        {
            dragAndDropStateMap[peer].handleDragAndDropPosition (clientMsg, peer);
        }
        else if (clientMsg.message_type == atoms.XdndDrop)
        {
            dragAndDropStateMap[peer].handleDragAndDropDrop (clientMsg, peer);
        }
        else if (clientMsg.message_type == atoms.XdndStatus)
        {
            dragAndDropStateMap[peer].handleExternalDragAndDropStatus (clientMsg);
        }
        else if (clientMsg.message_type == atoms.XdndFinished)
        {
            dragAndDropStateMap[peer].externalResetDragAndDrop();
        }
        else if (clientMsg.message_type == atoms.XembedMsgType && clientMsg.format == 32)
        {
            handleXEmbedMessage (peer, clientMsg);
        }
        */

    }
    
    pub fn handle_xembed_message(&self, 
        peer:       *mut LinuxComponentPeer,
        client_msg: &mut XClientMessageEvent)  {
        
        todo!();
        /*
            switch (clientMsg.data.l[1])
        {
            case 0:   // XEMBED_EMBEDDED_NOTIFY
                peer->setParentWindow ((Window) clientMsg.data.l[3]);
                peer->updateWindowBounds();
                peer->getComponent().setBounds (peer->getBounds());
                break;
            case 4:   // XEMBED_FOCUS_IN
                handleFocusInEvent (peer);
                break;
            case 5:   // XEMBED_FOCUS_OUT
                handleFocusOutEvent (peer);
                break;

            default:
                break;
        }
        */

    }
    
    pub fn dismiss_blocking_modals_with_configuration(
        &self, 
        peer:      *mut LinuxComponentPeer,
        configure: &XConfigureEvent
    ) {
        
        todo!();
        /*
            if (peer == nullptr)
            return;

        const auto peerHandle = peer->getWindowHandle();

        if (configure.window != peerHandle && isParentWindowOf (configure.window, peerHandle))
            dismissBlockingModals (peer);
        */

    }
    
    pub fn window_message_receive(&mut self, event: &mut XEvent)  {
        
        todo!();
        /*
            if (event.xany.window != None)
        {
           #if ALOE_X11_SUPPORTS_XEMBED
            if (! aloe_handleXEmbedEvent (nullptr, &event))
           #endif
            {
                if (auto* peer = dynamic_cast<LinuxComponentPeer*> (getPeerFor (event.xany.window)))
                {
                    XWindowSystem::getInstance()->handleWindowMessage (peer, event);
                    return;
                }

                if (event.type != ConfigureNotify)
                    return;

                const auto* instance = XWindowSystem::getInstance();

                for (auto i = ComponentPeer::getNumPeers(); --i >= 0;)
                    instance->dismissBlockingModals (dynamic_cast<LinuxComponentPeer*> (ComponentPeer::getPeer (i)),
                                                     event.xconfigure);
            }
        }
        else if (event.xany.type == KeymapNotify)
        {
            auto& keymapEvent = (const XKeymapEvent&) event.xkeymap;
            memcpy (Keys::keyStates, keymapEvent.key_vector, 32);
        }
        */

    }
    
    pub fn get_local_clipboard_content(&self) -> String {
        
        todo!();
        /*
            return localClipboardContent;
        */
    }
    
    pub fn get_display(&mut self) -> *mut Display {
        
        todo!();
        /*
            return display;
        */
    }
    
    pub fn get_atoms(&self) -> &XWindowSystemAtoms {
        
        todo!();
        /*
            return atoms;
        */
    }
    
    pub fn is_x11available(&self) -> bool {
        
        todo!();
        /*
            return xIsAvailable;
        */
    }
}
