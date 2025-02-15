crate::ix!();

#[no_copy]
#[leak_detector]
#[cfg(target_os="linux")]
pub struct SystemTrayIconComponentImpl {
    image: Image,
}

#[cfg(target_os="linux")]
impl SystemTrayIconComponentImpl {

    pub fn new(
        im:      &Image,
        windowh: Window) -> Self {
    
        todo!();
        /*
        : image(im),

            XWindowSystemUtilities::ScopedXLock xLock;

            auto* display = XWindowSystem::getInstance()->getDisplay();

            auto* screen = X11Symbols::getInstance()->xDefaultScreenOfDisplay (display);
            auto screenNumber = X11Symbols::getInstance()->xScreenNumberOfScreen (screen);

            String screenAtom ("_NET_SYSTEM_TRAY_S");
            screenAtom << screenNumber;
            Atom selectionAtom = XWindowSystemUtilities::Atoms::getCreating (display, screenAtom.toUTF8());

            X11Symbols::getInstance()->xGrabServer (display);
            auto managerWin = X11Symbols::getInstance()->xGetSelectionOwner (display, selectionAtom);

            if (managerWin != None)
                X11Symbols::getInstance()->xSelectInput (display, managerWin, StructureNotifyMask);

            X11Symbols::getInstance()->xUngrabServer (display);
            X11Symbols::getInstance()->xFlush (display);

            if (managerWin != None)
            {
                XEvent ev = { 0 };
                ev.xclient.type = ClientMessage;
                ev.xclient.window = managerWin;
                ev.xclient.message_type = XWindowSystemUtilities::Atoms::getCreating (display, "_NET_SYSTEM_TRAY_OPCODE");
                ev.xclient.format = 32;
                ev.xclient.data.l[0] = CurrentTime;
                ev.xclient.data.l[1] = 0 /*SYSTEM_TRAY_REQUEST_DOCK*/;
                ev.xclient.data.l[2] = (long) windowH;
                ev.xclient.data.l[3] = 0;
                ev.xclient.data.l[4] = 0;

                X11Symbols::getInstance()->xSendEvent (display, managerWin, False, NoEventMask, &ev);
                X11Symbols::getInstance()->xSync (display, False);
            }

            // For older KDE's ...
            long atomData = 1;
            Atom trayAtom = XWindowSystemUtilities::Atoms::getCreating (display, "KWM_DOCKWINDOW");
            X11Symbols::getInstance()->xChangeProperty (display, windowH, trayAtom, trayAtom,
                                                        32, PropModeReplace, (unsigned char*) &atomData, 1);

            // For more recent KDE's...
            trayAtom = XWindowSystemUtilities::Atoms::getCreating (display, "_KDE_NET_WM_SYSTEM_TRAY_WINDOW_FOR");
            X11Symbols::getInstance()->xChangeProperty (display, windowH, trayAtom, XA_WINDOW,
                                                        32, PropModeReplace, (unsigned char*) &windowH, 1);

            // A minimum size must be specified for GNOME and Xfce, otherwise the icon is displayed with a width of 1
            if (auto* hints = X11Symbols::getInstance()->xAllocSizeHints())
            {
                hints->flags = PMinSize;
                hints->min_width = 22;
                hints->min_height = 22;
                X11Symbols::getInstance()->xSetWMNormalHints (display, windowH, hints);
                X11Symbols::getInstance()->xFree (hints);
            }
        */
    }
}
