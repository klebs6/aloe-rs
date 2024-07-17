crate::ix!();

#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct X11Symbols {
    x_lib:         DynamicLibrary, // default = { "libX11.so.6"  }
    xext_lib:      DynamicLibrary, // default = { "libXext.so.6"  }

    #[cfg(ALOE_USE_XCURSOR)]
    xcursor_lib:   DynamicLibrary, // default = { "libXcursor.so.1"  }

    #[cfg(ALOE_USE_XINERAMA)]
    xinerama_lib:  DynamicLibrary, // default = { "libXinerama.so.1"  }

    #[cfg(ALOE_USE_XRENDER)]
    xrender_lib:   DynamicLibrary, // default = { "libXrender.so.1"  }

    #[cfg(ALOE_USE_XRANDR)]
    xrandr_lib:    DynamicLibrary, // default = { "libXrandr.so.2"  }
}

impl Drop for X11Symbols {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */

    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/x11/aloe_linux_X11_Symbols.cpp]
impl X11Symbols {

    aloe_generate_function_with_default!{
        XAllocClassHint, xAllocClassHint,
        (),
        *mut XClassHint
    }

    aloe_generate_function_with_default!{
        XAllocSizeHints, 
        xAllocSizeHints,
        (),
        *mut XSizeHints
    }

    aloe_generate_function_with_default!{
        XAllocWMHints, 
        xAllocWMHints,
        (),
        *mut XWMHints
    }

    aloe_generate_function_with_default!{
        XBitmapBitOrder, 
        xBitmapBitOrder,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XBitmapUnit, 
        xBitmapUnit,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XChangeActivePointerGrab, 
        xChangeActivePointerGrab,
        (*mut Display, u32, Cursor, Time),
        ()
    }

    aloe_generate_function_with_default!{
        XChangeProperty, xChangeProperty,
        (*mut Display, Window, Atom, Atom, i32, i32, *const u8, i32),
        ()
    }

    aloe_generate_function_with_default!{
        XCheckTypedWindowEvent, xCheckTypedWindowEvent,
        (*mut Display, Window, i32, *mut XEvent),
        bool
    }

    aloe_generate_function_with_default!{
        XCheckWindowEvent, xCheckWindowEvent,
        (*mut Display, Window, u32, *mut XEvent),
        bool
    }

    aloe_generate_function_with_default!{
        XClearArea, 
        xClearArea,
        (*mut Display, Window, i32, i32, u32, u32, bool),
        ()
    }

    aloe_generate_function_with_default!{
        XCloseDisplay, 
        xCloseDisplay,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XConnectionNumber, 
        xConnectionNumber,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XConvertSelection, 
        xConvertSelection,
        (*mut Display, Atom, Atom, Atom, Window, Time),
        ()
    }

    aloe_generate_function_with_default!{
        XCreateColormap, 
        xCreateColormap,
        (*mut Display, Window, *mut Visual, i32),
        Colormap
    }

    aloe_generate_function_with_default!{
        XCreateFontCursor, 
        xCreateFontCursor,
        (*mut Display, u32),
        Cursor
    }

    aloe_generate_function_with_default!{
        XCreateGC, 
        xCreateGC,
        (*mut Display, Drawable, u32, *mut XGCValues),
        GC
    }

    aloe_generate_function_with_default!{
        XCreateImage, xCreateImage,
        (*mut Display, 
         *mut Visual, 
         u32, 
         i32, 
         i32, 
         *const char, 
         u32, u32, i32, i32),
        *mut XImage
    }

    aloe_generate_function_with_default!{
        XCreatePixmap, xCreatePixmap,
        (*mut Display, Drawable, u32, u32, u32),
        Pixmap
    }

    aloe_generate_function_with_default!{
        XCreatePixmapCursor, xCreatePixmapCursor,
        (*mut Display, Pixmap, Pixmap, *mut XColor, *mut XColor, u32, u32),
        Cursor
    }

    aloe_generate_function_with_default!{
        XCreatePixmapFromBitmapData, 
        xCreatePixmapFromBitmapData,
        (*mut Display, 
         Drawable, 
         *const u8, 
         u32, 
         u32, 
         u64, 
         u64, 
         u32),
        Pixmap
    }

    aloe_generate_function_with_default!{
        XCreateWindow, xCreateWindow,
        (*mut Display, Window, i32, i32, u32, u32, u32, i32, 
         u32, 
         *mut Visual, 
         u64, 
         *mut XSetWindowAttributes),
        Window
    }

    aloe_generate_function_with_default!{
        XDefaultRootWindow, 
        xDefaultRootWindow,
        (*mut Display),
        Window
    }

    aloe_generate_function_with_default!{
        XDefaultScreen, 
        xDefaultScreen,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XDefaultScreenOfDisplay, 
        xDefaultScreenOfDisplay,
        (*mut Display),
        *mut Screen
    }

    aloe_generate_function_with_default!{
        XDefaultVisual, 
        xDefaultVisual,
        (*mut Display, i32),
        *mut Visual
    }

    aloe_generate_function_with_default!{
        XDefineCursor, 
        xDefineCursor,
        (*mut Display, Window, Cursor),
        i32
    }

    aloe_generate_function_with_default!{
        XDeleteContext, xDeleteContext,
        (*mut Display, XID, XContext),
        i32
    }

    aloe_generate_function_with_default!{
        XDeleteProperty, xDeleteProperty,
        (*mut Display, Window, Atom),
        ()
    }

    aloe_generate_function_with_default!{
        XDestroyImage, 
        xDestroyImage,
        (*mut XImage),
        ()
    }

    aloe_generate_function_with_default!{
        XDestroyWindow, 
        xDestroyWindow,
        (*mut Display, Window),
        ()
    }

    aloe_generate_function_with_default!{
        XDisplayHeight, 
        xDisplayHeight,
        (*mut Display, i32),
        i32
    }

    aloe_generate_function_with_default!{
        XDisplayHeightMM, xDisplayHeightMM,
        (*mut Display, i32),
        i32
    }

    aloe_generate_function_with_default!{
        XDisplayWidth, 
        xDisplayWidth,
        (*mut Display, i32),
        i32
    }

    aloe_generate_function_with_default!{
        XDisplayWidthMM, 
        xDisplayWidthMM,
        (*mut Display, i32),
        i32
    }

    aloe_generate_function_with_default!{
        XEventsQueued, xEventsQueued,
        (*mut Display, i32),
        i32
    }

    aloe_generate_function_with_default!{
        XFindContext, xFindContext,
        (*mut Display, XID, XContext, *mut XPointer),
        i32
    }

    aloe_generate_function_with_default!{
        XFlush, xFlush,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XFree, xFree,
        (*mut c_void),
        ()
    }

    aloe_generate_function_with_default!{
        XFreeCursor, xFreeCursor,
        (*mut Display, Cursor),
        ()
    }

    aloe_generate_function_with_default!{
        XFreeColormap ,xFreeColormap,
        (*mut Display, Colormap),
        ()
    }

    aloe_generate_function_with_default!{
        XFreeGC, xFreeGC,
        (*mut Display, GC),
        ()
    }

    aloe_generate_function_with_default!{
        XFreeModifiermap, xFreeModifiermap,
        (*mut XModifierKeymap),
        ()
    }

    aloe_generate_function_with_default!{
        XFreePixmap, 
        xFreePixmap,
        (*mut Display, Pixmap),
        ()
    }

    aloe_generate_function_with_default!{
        XGetAtomName, xGetAtomName,
        (*mut Display, Atom),
        *mut char
    }

    aloe_generate_function_with_default!{
        XGetErrorDatabaseText, xGetErrorDatabaseText,
        (*mut Display, *const char, *const char, *const char, *const char, i32),
        ()
    }

    aloe_generate_function_with_default!{
        XGetErrorText, xGetErrorText,
        (*mut Display, i32, *const char, i32),
        ()
    }

    aloe_generate_function_with_default!{
        XGetGeometry, xGetGeometry,
        (*mut Display, 
         Drawable, 
         *mut Window, 
         *mut i32, 
         *mut i32, 
         *mut u32, 
         *mut u32, 
         *mut u32, 
         *mut u32),
        Status
    }

    aloe_generate_function_with_default!{
        XGetInputFocus, xGetInputFocus,
        (*mut Display, *mut Window, *mut i32),
        ()
    }

    aloe_generate_function_with_default!{
        XGetModifierMapping, 
        xGetModifierMapping,
        (*mut Display),
        *mut XModifierKeymap
    }

    aloe_generate_function_with_default!{
        XGetPointerMapping, xGetPointerMapping,
        (*mut Display, &[u8], i32),
        i32
    }

    aloe_generate_function_with_default!{
        XGetSelectionOwner, xGetSelectionOwner,
        (*mut Display, Atom),
        Window
    }

    aloe_generate_function_with_default!{
        XGetVisualInfo, xGetVisualInfo,
        (*mut Display, i32, *mut XVisualInfo, *mut i32),
        *mut XVisualInfo
    }

    aloe_generate_function_with_default!{
        XGetWMHints, 
        xGetWMHints,
        (*mut Display, Window),
        *mut XWMHints
    }

    aloe_generate_function_with_default!{
        XGetWindowAttributes, xGetWindowAttributes,
        (*mut Display, Window, *mut XWindowAttributes),
        Status
    }

    aloe_generate_function_with_default!{
        XGetWindowProperty, xGetWindowProperty,
        (*mut Display, Window, Atom, i32, i32, bool, Atom, 
         *mut Atom, *mut i32, *mut u32, *mut u32, *mut *mut u8),
        i32
    }

    aloe_generate_function_with_default!{
        XGrabPointer, xGrabPointer,
        (*mut Display, Window, bool, u32, i32, i32, Window, Cursor, Time),
        i32
    }

    aloe_generate_function_with_default!{
        XGrabServer, xGrabServer,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XImageByteOrder, xImageByteOrder,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XInitImage, xInitImage,
        (*mut XImage),
        Status
    }

    aloe_generate_function_with_default!{
        XInitThreads, xInitThreads,
        (),
        Status
    }

    aloe_generate_function_with_default!{
        XInstallColormap, xInstallColormap,
        (*mut Display, Colormap),
        ()
    }

    aloe_generate_function_with_default!{
        XInternAtom, xInternAtom,
        (*mut Display, *const char, bool),
        Atom
    }

    aloe_generate_function_with_default!{
        XkbKeycodeToKeysym, xkbKeycodeToKeysym,
        (*mut Display, KeyCode, u32, u32),
        KeySym
    }

    aloe_generate_function_with_default!{
        XKeysymToKeycode, xKeysymToKeycode,
        (*mut Display, KeySym),
        KeyCode
    }

    aloe_generate_function_with_default!{
        XListProperties, xListProperties,
        (*mut Display, Window, *mut i32),
        *mut Atom
    }

    aloe_generate_function_with_default!{
        XLockDisplay, xLockDisplay,
        (*mut Display),
        ()
    }

    aloe_generate_function_with_default!{
        XLookupString, xLookupString,
        (*mut XKeyEvent, *const char, i32, *mut KeySym, *mut XComposeStatus),
        i32
    }

    aloe_generate_function_with_default!{
        XMapRaised, xMapRaised,
        (*mut Display, Window),
        ()
    }

    aloe_generate_function_with_default!{
        XMapWindow, xMapWindow,
        (*mut Display, Window),
        ()
    }

    aloe_generate_function_with_default!{
        XMoveResizeWindow, xMoveResizeWindow,
        (*mut Display, Window, i32, i32, u32, u32),
        ()
    }

    aloe_generate_function_with_default!{
        XNextEvent, xNextEvent,
        (*mut Display, *mut XEvent),
        i32
    }

    aloe_generate_function_with_default!{
        XOpenDisplay, xOpenDisplay,
        (*const u8),
        *mut Display
    }

    aloe_generate_function_with_default!{
        XPeekEvent, xPeekEvent,
        (*mut Display, *mut XEvent),
        ()
    }

    aloe_generate_function_with_default!{
        XPending, xPending,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XPutImage, xPutImage,
        (*mut Display, Drawable, GC, *mut XImage, i32, i32, i32, i32, u32, u32),
        ()
    }

    aloe_generate_function_with_default!{
        XPutPixel, xPutPixel,
        (*mut XImage, i32, i32, u64),
        ()
    }

    aloe_generate_function_with_default!{
        XQueryBestCursor, xQueryBestCursor,
        (*mut Display, Drawable, u32, u32, *mut u32, *mut u32),
        Status
    }

    aloe_generate_function_with_default!{
        XQueryExtension, xQueryExtension,
        (*mut Display, *const char, *mut i32, *mut i32, *mut i32),
        bool
    }

    aloe_generate_function_with_default!{
        XQueryPointer, xQueryPointer,
        (*mut Display, Window, *mut Window, *mut Window, 
         *mut i32, *mut i32, *mut i32, *mut i32, *mut u32),
        bool
    }

    aloe_generate_function_with_default!{
        XQueryTree, xQueryTree,
        (*mut Display, Window, *mut Window, *mut Window, *mut *mut Window, *mut u32),
        Status
    }

    aloe_generate_function_with_default!{
        XRefreshKeyboardMapping, xRefreshKeyboardMapping,
        (*mut XMappingEvent),
        ()
    }

    aloe_generate_function_with_default!{
        XReparentWindow, xReparentWindow,
        (*mut Display, Window, Window, i32, i32),
        ()
    }

    aloe_generate_function_with_default!{
        XResizeWindow, xResizeWindow,
        (*mut Display, Window, u32, u32),
        ()
    }

    aloe_generate_function_with_default!{
        XRestackWindows, xRestackWindows,
        (*mut Display, &[Window], i32),
        ()
    }

    aloe_generate_function_with_default!{
        XRootWindow, xRootWindow,
        (*mut Display, i32),
        Window
    }

    aloe_generate_function_with_default!{
        XSaveContext, xSaveContext,
        (*mut Display, XID, XContext, XPointer),
        i32
    }

    aloe_generate_function_with_default!{
        XScreenCount, xScreenCount,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XScreenNumberOfScreen, xScreenNumberOfScreen,
        (*mut Screen),
        i32
    }

    aloe_generate_function_with_default!{
        XSelectInput, xSelectInput,
        (*mut Display, Window, long),
        ()
    }

    aloe_generate_function_with_default!{
        XSendEvent, xSendEvent,
        (*mut Display, Window, bool, long, *mut XEvent),
        Status
    }

    aloe_generate_function_with_default!{
        XSetClassHint, xSetClassHint,
        (*mut Display, Window, *mut XClassHint),
        ()
    }

    aloe_generate_function_with_default!{
        XSetErrorHandler, xSetErrorHandler,
        (XErrorHandler),
        XErrorHandler
    }

    aloe_generate_function_with_default!{
        XSetIOErrorHandler, xSetIOErrorHandler,
        (XIOErrorHandler),
        XIOErrorHandler
    }

    aloe_generate_function_with_default!{
        XSetInputFocus, xSetInputFocus,
        (*mut Display, Window, i32, ::Time),
        ()
    }

    aloe_generate_function_with_default!{
        XSetSelectionOwner, xSetSelectionOwner,
        (*mut Display, Atom, Window, ::Time),
        ()
    }

    aloe_generate_function_with_default!{
        XSetWMHints, xSetWMHints,
        (*mut Display, Window, *mut XWMHints),
        ()
    }

    aloe_generate_function_with_default!{
        XSetWMIconName, xSetWMIconName,
        (*mut Display, Window, *mut XTextProperty),
        ()
    }

    aloe_generate_function_with_default!{
        XSetWMName, xSetWMName,
        (*mut Display, Window, *mut XTextProperty),
        ()
    }

    aloe_generate_function_with_default!{
        XSetWMNormalHints, xSetWMNormalHints,
        (*mut Display, Window, *mut XSizeHints),
        ()
    }

    aloe_generate_function_with_default!{
        XStringListToTextProperty, xStringListToTextProperty,
        (*mut *mut char, i32, *mut XTextProperty),
        Status
    }

    aloe_generate_function_with_default!{
        XSync, xSync,
        (*mut Display, bool),
        ()
    }

    aloe_generate_function_with_default!{
        XSynchronize, xSynchronize,
        (*mut Display, bool),
        i32
    }

    aloe_generate_function_with_default!{
        XTranslateCoordinates, xTranslateCoordinates,
        (*mut Display, Window, Window, i32, i32, *mut i32, *mut i32, *mut Window),
        bool
    }

    aloe_generate_function_with_default!{
        XrmUniqueQuark, xrmUniqueQuark,
        (),
        i32
    }

    aloe_generate_function_with_default!{
        XUngrabPointer, xUngrabPointer,
        (*mut Display, ::Time),
        ()
    }

    aloe_generate_function_with_default!{
        XUngrabServer, xUngrabServer,
        (*mut Display),
        i32
    }

    aloe_generate_function_with_default!{
        XUnlockDisplay, xUnlockDisplay,
        (*mut Display),
        ()
    }

    aloe_generate_function_with_default!{
        XUnmapWindow, xUnmapWindow,
        (*mut Display, Window),
        ()
    }

    aloe_generate_function_with_default!{
        XWarpPointer, xWarpPointer,
        (*mut Display, Window, Window, i32, i32, u32, u32, i32, i32),
        ()
    }

    #[cfg(ALOE_USE_XCURSOR)]
    aloe_generate_function_with_default!{
        XcursorImageCreate, xcursorImageCreate,
        (i32, i32),
        *mut XcursorImage
       }

    #[cfg(ALOE_USE_XCURSOR)]
    aloe_generate_function_with_default!{
        XcursorImageLoadCursor, xcursorImageLoadCursor,
        (*mut Display, *mut XcursorImage),
        Cursor
       }

    #[cfg(ALOE_USE_XCURSOR)]
    aloe_generate_function_with_default!{
        XcursorImageDestroy, xcursorImageDestroy,
        (*mut XcursorImage),
        ()
       }

    #[cfg(ALOE_USE_XINERAMA)]
    aloe_generate_function_with_default!{
        XineramaIsActive, xineramaIsActive,
        (*mut Display),
        bool
       }

    #[cfg(ALOE_USE_XINERAMA)]
    aloe_generate_function_with_default!{
        XineramaQueryScreens, xineramaQueryScreens,
        (*mut Display, *mut i32),
        *mut XineramaScreenInfo
       }

    #[cfg(ALOE_USE_XRENDER)]
    aloe_generate_function_with_default!{
        XRenderQueryVersion, xRenderQueryVersion,
        (*mut Display, *mut i32, *mut i32),
        Status
       }

    #[cfg(ALOE_USE_XRENDER)]
    aloe_generate_function_with_default!{
        XRenderFindStandardFormat, xRenderFindStandardFormat,
        (*mut Display, i32),
        *mut XRenderPictFormat
       }

    #[cfg(ALOE_USE_XRENDER)]
    aloe_generate_function_with_default!{
        XRenderFindFormat, xRenderFindFormat,
        (*mut Display, u64, *mut XRenderPictFormat, i32),
        *mut XRenderPictFormat
       }

    #[cfg(ALOE_USE_XRENDER)]
    aloe_generate_function_with_default!{
        XRenderFindVisualFormat, xRenderFindVisualFormat,
        (*mut Display, *mut Visual),
        *mut XRenderPictFormat
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRGetScreenResources, xRRGetScreenResources,
        (*mut Display, Window),
        *mut XRRScreenResources
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRFreeScreenResources, xRRFreeScreenResources,
        (*mut XRRScreenResources),
        ()
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRGetOutputInfo, xRRGetOutputInfo,
        (*mut Display, *mut XRRScreenResources, RROutput),
        *mut XRROutputInfo
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRFreeOutputInfo, xRRFreeOutputInfo,
        (*mut XRROutputInfo),
        ()
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRGetCrtcInfo, xRRGetCrtcInfo,
        (*mut Display, *mut XRRScreenResources, RRCrtc),
        *mut XRRCrtcInfo
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRFreeCrtcInfo, xRRFreeCrtcInfo,
        (*mut XRRCrtcInfo),
        ()
       }

    #[cfg(ALOE_USE_XRANDR)]
    aloe_generate_function_with_default!{
        XRRGetOutputPrimary, xRRGetOutputPrimary,
        (*mut Display, Window),
        RROutput
       }

    #[cfg(ALOE_USE_XSHM)]
    aloe_generate_function_with_default!{
        XShmAttach, xShmAttach,
        (*mut Display, *mut XShmSegmentInfo),
        bool
       }

    #[cfg(ALOE_USE_XSHM)]
    aloe_generate_function_with_default!{
        XShmCreateImage, xShmCreateImage,
        (*mut Display, *mut Visual, u32, i32, *const u8, *mut XShmSegmentInfo, u32, u32),
        *mut XImage
       }

    #[cfg(ALOE_USE_XSHM)]
    aloe_generate_function_with_default!{
        XShmDetach, xShmDetach,
        (*mut Display, *mut XShmSegmentInfo),
        bool
       }

    #[cfg(ALOE_USE_XSHM)]
    aloe_generate_function_with_default!{
        XShmGetEventBase, xShmGetEventBase,
        (*mut Display),
        Status
       }

    #[cfg(ALOE_USE_XSHM)]
    aloe_generate_function_with_default!{
        XShmPutImage, xShmPutImage,
        (*mut Display, Drawable, GC, *mut XImage, i32, i32, i32, i32, u32, u32, bool),
        bool
       }

    #[cfg(ALOE_USE_XSHM)]
    aloe_generate_function_with_default!{
        XShmQueryVersion, xShmQueryVersion,
        (*mut Display, *mut i32, *mut i32, *mut bool),
        bool
       }

        
    aloe_declare_singleton!{
        X11Symbols, false
    }
}

impl X11Symbols {
    
    pub fn load_all_symbols(&mut self) -> bool {
        
        todo!();
        /*
            using namespace X11SymbolHelpers;

        if (! loadSymbols (xLib, xextLib,
                           makeSymbolBinding (xAllocClassHint,             "XAllocClassHint"),
                           makeSymbolBinding (xAllocSizeHints,             "XAllocSizeHints"),
                           makeSymbolBinding (xAllocWMHints,               "XAllocWMHints"),
                           makeSymbolBinding (xBitmapBitOrder,             "XBitmapBitOrder"),
                           makeSymbolBinding (xBitmapUnit,                 "XBitmapUnit"),
                           makeSymbolBinding (xChangeActivePointerGrab,    "XChangeActivePointerGrab"),
                           makeSymbolBinding (xChangeProperty,             "XChangeProperty"),
                           makeSymbolBinding (xCheckTypedWindowEvent,      "XCheckTypedWindowEvent"),
                           makeSymbolBinding (xCheckWindowEvent,           "XCheckWindowEvent"),
                           makeSymbolBinding (xClearArea,                  "XClearArea"),
                           makeSymbolBinding (xCloseDisplay,               "XCloseDisplay"),
                           makeSymbolBinding (xConnectionNumber,           "XConnectionNumber"),
                           makeSymbolBinding (xConvertSelection,           "XConvertSelection"),
                           makeSymbolBinding (xCreateColormap,             "XCreateColormap"),
                           makeSymbolBinding (xCreateFontCursor,           "XCreateFontCursor"),
                           makeSymbolBinding (xCreateGC,                   "XCreateGC"),
                           makeSymbolBinding (xCreateImage,                "XCreateImage"),
                           makeSymbolBinding (xCreatePixmap,               "XCreatePixmap"),
                           makeSymbolBinding (xCreatePixmapCursor,         "XCreatePixmapCursor"),
                           makeSymbolBinding (xCreatePixmapFromBitmapData, "XCreatePixmapFromBitmapData"),
                           makeSymbolBinding (xCreateWindow,               "XCreateWindow"),
                           makeSymbolBinding (xDefaultRootWindow,          "XDefaultRootWindow"),
                           makeSymbolBinding (xDefaultScreen,              "XDefaultScreen"),
                           makeSymbolBinding (xDefaultScreenOfDisplay,     "XDefaultScreenOfDisplay"),
                           makeSymbolBinding (xDefaultVisual,              "XDefaultVisual"),
                           makeSymbolBinding (xDefineCursor,               "XDefineCursor"),
                           makeSymbolBinding (xDeleteContext,              "XDeleteContext"),
                           makeSymbolBinding (xDeleteProperty,             "XDeleteProperty"),
                           makeSymbolBinding (xDestroyImage,               "XDestroyImage"),
                           makeSymbolBinding (xDestroyWindow,              "XDestroyWindow"),
                           makeSymbolBinding (xDisplayHeight,              "XDisplayHeight"),
                           makeSymbolBinding (xDisplayHeightMM,            "XDisplayHeightMM"),
                           makeSymbolBinding (xDisplayWidth,               "XDisplayWidth"),
                           makeSymbolBinding (xDisplayWidthMM,             "XDisplayWidthMM"),
                           makeSymbolBinding (xEventsQueued,               "XEventsQueued"),
                           makeSymbolBinding (xFindContext,                "XFindContext"),
                           makeSymbolBinding (xFlush,                      "XFlush"),
                           makeSymbolBinding (xFree,                       "XFree"),
                           makeSymbolBinding (xFreeCursor,                 "XFreeCursor"),
                           makeSymbolBinding (xFreeColormap,               "XFreeColormap"),
                           makeSymbolBinding (xFreeGC,                     "XFreeGC"),
                           makeSymbolBinding (xFreeModifiermap,            "XFreeModifiermap"),
                           makeSymbolBinding (xFreePixmap,                 "XFreePixmap"),
                           makeSymbolBinding (xGetAtomName,                "XGetAtomName"),
                           makeSymbolBinding (xGetErrorDatabaseText,       "XGetErrorDatabaseText"),
                           makeSymbolBinding (xGetErrorText,               "XGetErrorText"),
                           makeSymbolBinding (xGetGeometry,                "XGetGeometry"),
                           makeSymbolBinding (xGetInputFocus,              "XGetInputFocus"),
                           makeSymbolBinding (xGetModifierMapping,         "XGetModifierMapping"),
                           makeSymbolBinding (xGetPointerMapping,          "XGetPointerMapping"),
                           makeSymbolBinding (xGetSelectionOwner,          "XGetSelectionOwner"),
                           makeSymbolBinding (xGetVisualInfo,              "XGetVisualInfo"),
                           makeSymbolBinding (xGetWMHints,                 "XGetWMHints"),
                           makeSymbolBinding (xGetWindowAttributes,        "XGetWindowAttributes"),
                           makeSymbolBinding (xGetWindowProperty,          "XGetWindowProperty"),
                           makeSymbolBinding (xGrabPointer,                "XGrabPointer"),
                           makeSymbolBinding (xGrabServer,                 "XGrabServer"),
                           makeSymbolBinding (xImageByteOrder,             "XImageByteOrder"),
                           makeSymbolBinding (xInitImage,                  "XInitImage"),
                           makeSymbolBinding (xInitThreads,                "XInitThreads"),
                           makeSymbolBinding (xInstallColormap,            "XInstallColormap"),
                           makeSymbolBinding (xInternAtom,                 "XInternAtom"),
                           makeSymbolBinding (xkbKeycodeToKeysym,          "XkbKeycodeToKeysym"),
                           makeSymbolBinding (xKeysymToKeycode,            "XKeysymToKeycode"),
                           makeSymbolBinding (xListProperties,             "XListProperties"),
                           makeSymbolBinding (xLockDisplay,                "XLockDisplay"),
                           makeSymbolBinding (xLookupString,               "XLookupString"),
                           makeSymbolBinding (xMapRaised,                  "XMapRaised"),
                           makeSymbolBinding (xMapWindow,                  "XMapWindow"),
                           makeSymbolBinding (xMoveResizeWindow,           "XMoveResizeWindow"),
                           makeSymbolBinding (xNextEvent,                  "XNextEvent"),
                           makeSymbolBinding (xOpenDisplay,                "XOpenDisplay"),
                           makeSymbolBinding (xPeekEvent,                  "XPeekEvent"),
                           makeSymbolBinding (xPending,                    "XPending"),
                           makeSymbolBinding (xPutImage,                   "XPutImage"),
                           makeSymbolBinding (xPutPixel,                   "XPutPixel"),
                           makeSymbolBinding (xQueryBestCursor,            "XQueryBestCursor"),
                           makeSymbolBinding (xQueryExtension,             "XQueryExtension"),
                           makeSymbolBinding (xQueryPointer,               "XQueryPointer"),
                           makeSymbolBinding (xQueryTree,                  "XQueryTree"),
                           makeSymbolBinding (xRefreshKeyboardMapping,     "XRefreshKeyboardMapping"),
                           makeSymbolBinding (xReparentWindow,             "XReparentWindow"),
                           makeSymbolBinding (xResizeWindow,               "XResizeWindow"),
                           makeSymbolBinding (xRestackWindows,             "XRestackWindows"),
                           makeSymbolBinding (xRootWindow,                 "XRootWindow"),
                           makeSymbolBinding (xSaveContext,                "XSaveContext"),
                           makeSymbolBinding (xScreenCount,                "XScreenCount"),
                           makeSymbolBinding (xScreenNumberOfScreen,       "XScreenNumberOfScreen"),
                           makeSymbolBinding (xSelectInput,                "XSelectInput"),
                           makeSymbolBinding (xSendEvent,                  "XSendEvent"),
                           makeSymbolBinding (xSetClassHint,               "XSetClassHint"),
                           makeSymbolBinding (xSetErrorHandler,            "XSetErrorHandler"),
                           makeSymbolBinding (xSetIOErrorHandler,          "XSetIOErrorHandler"),
                           makeSymbolBinding (xSetInputFocus,              "XSetInputFocus"),
                           makeSymbolBinding (xSetSelectionOwner,          "XSetSelectionOwner"),
                           makeSymbolBinding (xSetWMHints,                 "XSetWMHints"),
                           makeSymbolBinding (xSetWMIconName,              "XSetWMIconName"),
                           makeSymbolBinding (xSetWMName,                  "XSetWMName"),
                           makeSymbolBinding (xSetWMNormalHints,           "XSetWMNormalHints"),
                           makeSymbolBinding (xStringListToTextProperty,   "XStringListToTextProperty"),
                           makeSymbolBinding (xSync,                       "XSync"),
                           makeSymbolBinding (xSynchronize,                "XSynchronize"),
                           makeSymbolBinding (xTranslateCoordinates,       "XTranslateCoordinates"),
                           makeSymbolBinding (xrmUniqueQuark,              "XrmUniqueQuark"),
                           makeSymbolBinding (xUngrabPointer,              "XUngrabPointer"),
                           makeSymbolBinding (xUngrabServer,               "XUngrabServer"),
                           makeSymbolBinding (xUnlockDisplay,              "XUnlockDisplay"),
                           makeSymbolBinding (xUnmapWindow,                "XUnmapWindow"),
                           makeSymbolBinding (xWarpPointer,                "XWarpPointer")))
            return false;

       #if ALOE_USE_XCURSOR
        loadSymbols (xcursorLib,
                     makeSymbolBinding (xcursorImageCreate,          "XcursorImageCreate"),
                     makeSymbolBinding (xcursorImageLoadCursor,      "XcursorImageLoadCursor"),
                     makeSymbolBinding (xcursorImageDestroy,         "XcursorImageDestroy"));
       #endif
       #if ALOE_USE_XINERAMA
        loadSymbols (xineramaLib,
                     makeSymbolBinding (xineramaIsActive,            "XineramaIsActive"),
                     makeSymbolBinding (xineramaQueryScreens,        "XineramaQueryScreens"));
       #endif
       #if ALOE_USE_XRENDER
        loadSymbols (xrenderLib,
                     makeSymbolBinding (xRenderQueryVersion,         "XRenderQueryVersion"),
                     makeSymbolBinding (xRenderFindStandardFormat,   "XRenderFindStandardFormat"),
                     makeSymbolBinding (xRenderFindFormat,           "XRenderFindFormat"),
                     makeSymbolBinding (xRenderFindVisualFormat,     "XRenderFindVisualFormat"));
       #endif
       #if ALOE_USE_XRANDR
        loadSymbols (xrandrLib,
                     makeSymbolBinding (xRRGetScreenResources,       "XRRGetScreenResources"),
                     makeSymbolBinding (xRRFreeScreenResources,      "XRRFreeScreenResources"),
                     makeSymbolBinding (xRRGetOutputInfo,            "XRRGetOutputInfo"),
                     makeSymbolBinding (xRRFreeOutputInfo,           "XRRFreeOutputInfo"),
                     makeSymbolBinding (xRRGetCrtcInfo,              "XRRGetCrtcInfo"),
                     makeSymbolBinding (xRRFreeCrtcInfo,             "XRRFreeCrtcInfo"),
                     makeSymbolBinding (xRRGetOutputPrimary,         "XRRGetOutputPrimary"));
       #endif
       #if ALOE_USE_XSHM
        loadSymbols (xLib, xextLib,
                     makeSymbolBinding (xShmAttach,                  "XShmAttach"),
                     makeSymbolBinding (xShmCreateImage,             "XShmCreateImage"),
                     makeSymbolBinding (xShmDetach,                  "XShmDetach"),
                     makeSymbolBinding (xShmGetEventBase,            "XShmGetEventBase"),
                     makeSymbolBinding (xShmPutImage,                "XShmPutImage"),
                     makeSymbolBinding (xShmQueryVersion,            "XShmQueryVersion"));
       #endif

        return true;
        */

    }
}

aloe_implement_singleton!{
    X11Symbols
}
