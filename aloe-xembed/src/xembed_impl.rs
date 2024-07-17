crate::ix!();

#[cfg(any(target_os="linux", target_os="freebsd", target_os="openbsd", target_os="netbsd"))]
pub struct XembedComponentPimpl<'a> {
    owner:             &'a mut XEmbedComponent,
    client:            Window, // default = 0
    host:              Window, // default = 0
    info_atom:         Atom,
    message_type_atom: Atom,
    client_initiated:  bool,
    wants_focus:       bool, // default = false
    allow_resize:      bool, // default = false
    supports_xembed:   bool, // default = false
    has_been_mapped:   bool, // default = false
    xembed_version:    i32, // default = maxXEmbedVersionToSupport
    last_peer:         *mut ComponentPeer<'a>, // default = nullptr
    key_window:        XembedComponentSharedKeyWindow<'a>::XembedComponentSharedKeyWindowPtr,
}

#[cfg(any(target_os="linux", target_os="freebsd", target_os="openbsd", target_os="netbsd"))]
impl<'a> ComponentListener for XembedComponentPimpl<'a> {

}

#[cfg(any(target_os="linux", target_os="freebsd", target_os="openbsd", target_os="netbsd"))]
impl<'a> Drop for XembedComponentPimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            owner.removeComponentListener (this);
            setClient (0, true);

            if (host != 0)
            {
                auto dpy = getDisplay();

                X11Symbols::getInstance()->xDestroyWindow (dpy, host);
                X11Symbols::getInstance()->xSync (dpy, false);

                auto mask = NoEventMask | KeyPressMask | KeyReleaseMask
                          | EnterWindowMask | LeaveWindowMask | PointerMotionMask
                          | KeymapStateMask | ExposureMask | StructureNotifyMask
                          | FocusChangeMask;

                XEvent event;
                while (X11Symbols::getInstance()->xCheckWindowEvent (dpy, host, mask, &event) == True)
                {}

                host = 0;
            }

            getWidgets().removeAllInstancesOf (this);
        */
    }
}

#[cfg(any(target_os="linux", target_os="freebsd", target_os="openbsd", target_os="netbsd"))]
impl<'a> XembedComponentPimpl<'a> {

    pub fn new(
        parent:               &mut XEmbedComponent,
        x_11window:           Window,
        wants_keyboard_focus: bool,
        is_client_initiated:  bool,
        should_allow_resize:  bool) -> Self {
    
        todo!();
        /*


            : owner (parent),
              infoAtom (XWindowSystem::getInstance()->getAtoms().XembedInfo),
              messageTypeAtom (XWindowSystem::getInstance()->getAtoms().XembedMsgType),
              clientInitiated (isClientInitiated),
              wantsFocus (wantsKeyboardFocus),
              allowResize (shouldAllowResize)

            getWidgets().add (this);

            createHostWindow();

            if (clientInitiated)
                setClient (x11Window, true);

            owner.setWantsKeyboardFocus (wantsFocus);
            owner.addComponentListener (this);
        */
    }
    
    pub fn set_client(&mut self, 
        xembed_client:   Window,
        should_reparent: bool)  {
        
        todo!();
        /*
            removeClient();

            if (xembedClient != 0)
            {
                auto dpy = getDisplay();

                client = xembedClient;

                // if the client has initiated the component then keep the clients size
                // otherwise the client should use the host's window' size
                if (clientInitiated)
                {
                    configureNotify();
                }
                else
                {
                    auto newBounds = getX11BoundsFromAloe();
                    X11Symbols::getInstance()->xResizeWindow (dpy, client, static_cast<unsigned int> (newBounds.getWidth()),
                                                              static_cast<unsigned int> (newBounds.getHeight()));
                }

                auto eventMask = StructureNotifyMask | PropertyChangeMask | FocusChangeMask;

                XWindowAttributes clientAttr;
                X11Symbols::getInstance()->xGetWindowAttributes (dpy, client, &clientAttr);

                if ((eventMask & clientAttr.your_event_mask) != eventMask)
                    X11Symbols::getInstance()->xSelectInput (dpy, client, clientAttr.your_event_mask | eventMask);

                getXEmbedMappedFlag();

                if (shouldReparent)
                    X11Symbols::getInstance()->xReparentWindow (dpy, client, host, 0, 0);

                if (supportsXembed)
                    sendXEmbedEvent (CurrentTime, XEMBED_EMBEDDED_NOTIFY, 0, (long) host, xembedVersion);

                updateMapping();
            }
        */
    }
    
    pub fn focus_gained(&mut self, change_type: FocusChangeType)  {
        
        todo!();
        /*
            if (client != 0 && supportsXembed && wantsFocus)
            {
                updateKeyFocus();
                sendXEmbedEvent (CurrentTime, XEMBED_FOCUS_IN,
                                 (changeType == focusChangedByTabKey ? XEMBED_FOCUS_FIRST : XEMBED_FOCUS_CURRENT));
            }
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            if (client != 0 && supportsXembed && wantsFocus)
            {
                sendXEmbedEvent (CurrentTime, XEMBED_FOCUS_OUT);
                updateKeyFocus();
            }
        */
    }
    
    pub fn brought_to_front(&mut self)  {
        
        todo!();
        /*
            if (client != 0 && supportsXembed)
                sendXEmbedEvent (CurrentTime, XEMBED_WINDOW_ACTIVATE);
        */
    }
    
    pub fn get_host_windowid(&mut self) -> u64 {
        
        todo!();
        /*
            // You are using the client initiated version of the protocol. You cannot
            // retrieve the window id of the host. Please read the documentation for
            // the XEmebedComponent class.
            jassert (! clientInitiated);

            return host;
        */
    }
    
    pub fn component_parent_hierarchy_changed(&mut self, _0: &mut Component<'a>)  {
        
        todo!();
        /*
            peerChanged (owner.getPeer());
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0: &mut Component<'a>,
        _1: bool,
        _2: bool)  {
        
        todo!();
        /*
            if (host != 0 && lastPeer != nullptr)
            {
                auto dpy = getDisplay();
                auto newBounds = getX11BoundsFromAloe();
                XWindowAttributes attr;

                if (X11Symbols::getInstance()->xGetWindowAttributes (dpy, host, &attr))
                {
                    Rectangle<int> currentBounds (attr.x, attr.y, attr.width, attr.height);
                    if (currentBounds != newBounds)
                    {
                        X11Symbols::getInstance()->xMoveResizeWindow (dpy, host, newBounds.getX(), newBounds.getY(),
                                                                      static_cast<unsigned int> (newBounds.getWidth()),
                                                                      static_cast<unsigned int> (newBounds.getHeight()));
                    }
                }

                if (client != 0 && X11Symbols::getInstance()->xGetWindowAttributes (dpy, client, &attr))
                {
                    Rectangle<int> currentBounds (attr.x, attr.y, attr.width, attr.height);

                    if ((currentBounds.getWidth() != newBounds.getWidth()
                         || currentBounds.getHeight() != newBounds.getHeight()))
                    {
                        X11Symbols::getInstance()->xMoveResizeWindow (dpy, client, 0, 0,
                                                                      static_cast<unsigned int> (newBounds.getWidth()),
                                                                      static_cast<unsigned int> (newBounds.getHeight()));
                    }
                }
            }
        */
    }
    
    pub fn create_host_window(&mut self)  {
        
        todo!();
        /*
            auto dpy = getDisplay();
            int defaultScreen = X11Symbols::getInstance()->xDefaultScreen (dpy);
            Window root = X11Symbols::getInstance()->xRootWindow (dpy, defaultScreen);

            XSetWindowAttributes swa;
            swa.border_pixel = 0;
            swa.background_pixmap = None;
            swa.override_redirect = True;
            swa.event_mask = SubstructureNotifyMask | StructureNotifyMask | FocusChangeMask;

            host = X11Symbols::getInstance()->xCreateWindow (dpy, root, 0, 0, 1, 1, 0, CopyFromParent,
                                                             InputOutput, CopyFromParent,
                                                             CWEventMask | CWBorderPixel | CWBackPixmap | CWOverrideRedirect,
                                                             &swa);
        */
    }
    
    pub fn remove_client(&mut self)  {
        
        todo!();
        /*
            if (client != 0)
            {
                auto dpy = getDisplay();
                X11Symbols::getInstance()->xSelectInput (dpy, client, 0);

                keyWindow = nullptr;

                int defaultScreen = X11Symbols::getInstance()->xDefaultScreen (dpy);
                Window root = X11Symbols::getInstance()->xRootWindow (dpy, defaultScreen);

                if (hasBeenMapped)
                {
                    X11Symbols::getInstance()->xUnmapWindow (dpy, client);
                    hasBeenMapped = false;
                }

                X11Symbols::getInstance()->xReparentWindow (dpy, client, root, 0, 0);
                client = 0;

                X11Symbols::getInstance()->xSync (dpy, False);
            }
        */
    }
    
    pub fn update_mapping(&mut self)  {
        
        todo!();
        /*
            if (client != 0)
            {
                const bool shouldBeMapped = getXEmbedMappedFlag();

                if (shouldBeMapped != hasBeenMapped)
                {
                    hasBeenMapped = shouldBeMapped;

                    if (shouldBeMapped)
                        X11Symbols::getInstance()->xMapWindow (getDisplay(), client);
                    else
                        X11Symbols::getInstance()->xUnmapWindow (getDisplay(), client);
                }
            }
        */
    }
    
    pub fn get_parent_x11window(&mut self) -> Window {
        
        todo!();
        /*
            if (auto* peer = owner.getPeer())
                return reinterpret_cast<Window> (peer->getNativeHandle());

            return {};
        */
    }
    
    pub fn get_display(&mut self) -> *mut Display {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->getDisplay();
        */
    }
    
    pub fn get_xembed_mapped_flag(&mut self) -> bool {
        
        todo!();
        /*
            XWindowSystemUtilities::GetXProperty embedInfo (client, infoAtom, 0, 2, false, infoAtom);

            if (embedInfo.success && embedInfo.actualFormat == 32
                 && embedInfo.numItems >= 2 && embedInfo.data != nullptr)
            {
                long version;
                memcpy (&version, embedInfo.data, sizeof (long));

                supportsXembed = true;
                xembedVersion = jmin ((int) maxXEmbedVersionToSupport, (int) version);

                long flags;
                memcpy (&flags, embedInfo.data + sizeof (long), sizeof (long));

                return ((flags & XEMBED_MAPPED) != 0);
            }
            else
            {
                supportsXembed = false;
                xembedVersion = maxXEmbedVersionToSupport;
            }

            return true;
        */
    }
    
    pub fn property_changed(&mut self, a: &Atom)  {
        
        todo!();
        /*
            if (a == infoAtom)
                updateMapping();
        */
    }
    
    pub fn configure_notify(&mut self)  {
        
        todo!();
        /*
            XWindowAttributes attr;
            auto dpy = getDisplay();

            if (X11Symbols::getInstance()->xGetWindowAttributes (dpy, client, &attr))
            {
                XWindowAttributes hostAttr;

                if (X11Symbols::getInstance()->xGetWindowAttributes (dpy, host, &hostAttr))
                    if (attr.width != hostAttr.width || attr.height != hostAttr.height)
                        X11Symbols::getInstance()->xResizeWindow (dpy, host, (unsigned int) attr.width, (unsigned int) attr.height);

                // as the client window is not on any screen yet, we need to guess
                // on which screen it might appear to get a scaling factor :-(
                auto& displays = Desktop::getInstance().getDisplays();
                auto* peer = owner.getPeer();
                const double scale = (peer != nullptr ? peer->getPlatformScaleFactor()
                                                      : displays.getPrimaryDisplay()->scale);

                Point<int> topLeftInPeer
                    = (peer != nullptr ? peer->getComponent().getLocalPoint (&owner, Point<int> (0, 0))
                       : owner.getBounds().getTopLeft());

                Rectangle<int> newBounds (topLeftInPeer.getX(), topLeftInPeer.getY(),
                                          static_cast<int> (static_cast<double> (attr.width)  / scale),
                                          static_cast<int> (static_cast<double> (attr.height) / scale));

                if (peer != nullptr)
                    newBounds = owner.getLocalArea (&peer->getComponent(), newBounds);

                jassert (newBounds.getX() == 0 && newBounds.getY() == 0);

                if (newBounds != owner.getLocalBounds())
                    owner.setSize (newBounds.getWidth(), newBounds.getHeight());
            }
        */
    }
    
    pub fn peer_changed(&mut self, new_peer: *mut ComponentPeer)  {
        
        todo!();
        /*
            if (newPeer != lastPeer)
            {
                if (lastPeer != nullptr)
                    keyWindow = nullptr;

                auto dpy = getDisplay();
                Window rootWindow = X11Symbols::getInstance()->xRootWindow (dpy, DefaultScreen (dpy));
                Rectangle<int> newBounds = getX11BoundsFromAloe();

                if (newPeer == nullptr)
                    X11Symbols::getInstance()->xUnmapWindow (dpy, host);

                Window newParent = (newPeer != nullptr ? getParentX11Window() : rootWindow);
                X11Symbols::getInstance()->xReparentWindow (dpy, host, newParent, newBounds.getX(), newBounds.getY());

                lastPeer = newPeer;

                if (newPeer != nullptr)
                {
                    if (wantsFocus)
                    {
                        keyWindow = XembedComponentSharedKeyWindow::getKeyWindowForPeer (newPeer);
                        updateKeyFocus();
                    }

                    componentMovedOrResized (owner, true, true);
                    X11Symbols::getInstance()->xMapWindow (dpy, host);

                    broughtToFront();
                }
            }
        */
    }
    
    pub fn update_key_focus(&mut self)  {
        
        todo!();
        /*
            if (lastPeer != nullptr && lastPeer->isFocused())
                X11Symbols::getInstance()->xSetInputFocus (getDisplay(), getCurrentFocusWindow (lastPeer), RevertToParent, CurrentTime);
        */
    }
    
    pub fn handle_xembed_cmd(&mut self, 
        x_time: &Time,
        opcode: i64,
        detail: i64,
        data1:  i64,
        data2:  i64)  {
        
        todo!();
        /*
            switch (opcode)
            {
                case XEMBED_REQUEST_FOCUS:
                    if (wantsFocus)
                        owner.grabKeyboardFocus();
                    break;

                case XEMBED_FOCUS_NEXT:
                    if (wantsFocus)
                        owner.moveKeyboardFocusToSibling (true);
                    break;

                case XEMBED_FOCUS_PREV:
                    if (wantsFocus)
                        owner.moveKeyboardFocusToSibling (false);
                    break;

                default:
                    break;
            }
        */
    }
    
    pub fn handle_x11event(&mut self, e: &XEvent) -> bool {
        
        todo!();
        /*
            if (e.xany.window == client && client != 0)
            {
                switch (e.type)
                {
                    case PropertyNotify:
                        propertyChanged (e.xproperty.atom);
                        return true;

                    case ConfigureNotify:
                        if (allowResize)
                            configureNotify();
                        else
                            MessageManager::callAsync ([this] {componentMovedOrResized (owner, true, true);});

                        return true;

                    default:
                        break;
                }
            }
            else if (e.xany.window == host && host != 0)
            {
                switch (e.type)
                {
                    case ReparentNotify:
                        if (e.xreparent.parent == host && e.xreparent.window != client)
                        {
                            setClient (e.xreparent.window, false);
                            return true;
                        }
                        break;

                    case CreateNotify:
                        if (e.xcreatewindow.parent != e.xcreatewindow.window && e.xcreatewindow.parent == host && e.xcreatewindow.window != client)
                        {
                            setClient (e.xcreatewindow.window, false);
                            return true;
                        }
                        break;

                    case GravityNotify:
                        componentMovedOrResized (owner, true, true);
                        return true;

                    case ClientMessage:
                        if (e.xclient.message_type == messageTypeAtom && e.xclient.format == 32)
                        {
                            handleXembedCmd ((::Time) e.xclient.data.l[0], e.xclient.data.l[1],
                                             e.xclient.data.l[2], e.xclient.data.l[3],
                                             e.xclient.data.l[4]);

                            return true;
                        }
                        break;

                    default:
                        break;
                }
            }

            return false;
        */
    }
    
    pub fn send_xembed_event(&mut self, 
        x_time:       &Time,
        opcode:       i64,
        opcode_minor: i64,
        data1:        i64,
        data2:        i64)  {

        let opcode_minor: i64 = opcode_minor.unwrap_or(0);
        let data1: i64 = data1.unwrap_or(0);
        let data2: i64 = data2.unwrap_or(0);

        todo!();
        /*
            XClientMessageEvent msg;
            auto dpy = getDisplay();

            ::memset (&msg, 0, sizeof (XClientMessageEvent));
            msg.window = client;
            msg.type = ClientMessage;
            msg.message_type = messageTypeAtom;
            msg.format = 32;
            msg.data.l[0] = (long) xTime;
            msg.data.l[1] = opcode;
            msg.data.l[2] = opcodeMinor;
            msg.data.l[3] = data1;
            msg.data.l[4] = data2;

            X11Symbols::getInstance()->xSendEvent (dpy, client, False, NoEventMask, (XEvent*) &msg);
            X11Symbols::getInstance()->xSync (dpy, False);
        */
    }
    
    pub fn get_x11bounds_from_aloe(&mut self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (auto* peer = owner.getPeer())
            {
                auto r = peer->getComponent().getLocalArea (&owner, owner.getLocalBounds());
                return r * peer->getPlatformScaleFactor();
            }

            return owner.getLocalBounds();
        */
    }
    
    pub fn get_widgets() -> &'a mut Vec<*mut XembedComponentPimpl<'a>> {
        
        todo!();
        /*
            static Vec<XembedComponentPimpl*> i;
            return i;
        */
    }
    
    pub fn dispatch_x11event(
        p:         *mut ComponentPeer,
        event_arg: *const XEvent) -> bool {
        
        todo!();
        /*
            if (eventArg != nullptr)
            {
                auto& e = *eventArg;

                if (auto w = e.xany.window)
                    for (auto* widget : getWidgets())
                        if (w == widget->host || w == widget->client)
                            return widget->handleX11Event (e);
            }
            else
            {
                for (auto* widget : getWidgets())
                    if (widget->owner.getPeer() == p)
                        widget->peerChanged (nullptr);
            }

            return false;
        */
    }
    
    pub fn get_current_focus_window(p: *mut ComponentPeer) -> Window {
        
        todo!();
        /*
            if (p != nullptr)
            {
                for (auto* widget : getWidgets())
                    if (widget->owner.getPeer() == p && widget->owner.hasKeyboardFocus (false))
                        return widget->client;
            }

            return XembedComponentSharedKeyWindow::getCurrentFocusWindow (p);
        */
    }
}
