crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/x11/aloe_linux_X11_DragAndDrop.cpp]

lazy_static!{
    /*
    extern void* createDraggingHandCursor();
    extern ComponentPeer* getPeerFor (::Window);
    */
}

#[derive(Default)]
#[leak_detector]
pub struct X11DragState {
    windowh:                         Window, // default = 0
    target_window:                   Window, // default = 0
    drag_and_drop_source_window:     Window, // default = 0
    xdnd_version:                    i32, // default = -1
    is_text:                         bool, // default = false
    dragging:                        bool, // default = false
    expecting_status:                bool, // default = false
    can_drop:                        bool, // default = false
    finish_after_drop_data_received: bool, // default = false
    drag_and_drop_current_mime_type: Atom,
    allowed_types:                   Vec<Atom>,
    src_mime_type_atom_list:         Vec<Atom>,
    drag_info:                       ComponentPeerDragInfo,
    silent_rect:                     Rectangle<i32>,
    text_or_files:                   String,
    completion_callback:             Option<fn() -> ()>, // default = None
}

impl X11DragState {

    pub fn is_dragging(&self) -> bool {
        
        todo!();
        /*
            return dragging;
        */

    }
    
    pub fn handle_external_selection_clear(&mut self)  {
        
        todo!();
        /*
            if (dragging)
                externalResetDragAndDrop();
        */

    }
    
    pub fn handle_external_selection_request(&mut self, evt: &XEvent)  {
        
        todo!();
        /*
            auto targetType = evt.xselectionrequest.target;

            XEvent s;
            s.xselection.type      = SelectionNotify;
            s.xselection.requestor = evt.xselectionrequest.requestor;
            s.xselection.selection = evt.xselectionrequest.selection;
            s.xselection.target    = targetType;
            s.xselection.property  = None;
            s.xselection.time      = evt.xselectionrequest.time;

            auto* display = getDisplay();

            if (allowedTypes.contains (targetType))
            {
                s.xselection.property = evt.xselectionrequest.property;

                X11Symbols::getInstance()->xChangeProperty (display, evt.xselectionrequest.requestor, evt.xselectionrequest.property,
                                                            targetType, 8, PropModeReplace,
                                                            reinterpret_cast<const unsigned char*> (textOrFiles.toRawUTF8()),
                                                            (int) textOrFiles.getNumBytesAsUTF8());
            }

            X11Symbols::getInstance()->xSendEvent (display, evt.xselectionrequest.requestor, True, 0, &s);
        */

    }
    
    pub fn handle_external_drag_and_drop_status(&mut self, client_msg: &XClientMessageEvent)  {
        
        todo!();
        /*
            if (expectingStatus)
            {
                expectingStatus = false;
                canDrop         = false;
                silentRect      = {};

                const auto& atoms = getAtoms();

                if ((clientMsg.data.l[1] & 1) != 0
                    && ((Atom) clientMsg.data.l[4] == atoms.XdndActionCopy
                        || (Atom) clientMsg.data.l[4] == atoms.XdndActionPrivate))
                {
                    if ((clientMsg.data.l[1] & 2) == 0) // target requests silent rectangle
                        silentRect.setBounds ((int) clientMsg.data.l[2] >> 16, (int) clientMsg.data.l[2] & 0xffff,
                                              (int) clientMsg.data.l[3] >> 16, (int) clientMsg.data.l[3] & 0xffff);

                    canDrop = true;
                }
            }
        */

    }
    
    pub fn handle_external_drag_button_release_event(&mut self)  {
        
        todo!();
        /*
            if (dragging)
                X11Symbols::getInstance()->xUngrabPointer (getDisplay(), CurrentTime);

            if (canDrop)
            {
                sendExternalDragAndDropDrop();
            }
            else
            {
                sendExternalDragAndDropLeave();
                externalResetDragAndDrop();
            }
        */

    }
    
    pub fn handle_external_drag_motion_notify(&mut self)  {
        
        todo!();
        /*
            auto* display = getDisplay();

            auto newTargetWindow = externalFindDragTargetWindow (X11Symbols::getInstance()
                                                                   ->xRootWindow (display,
                                                                                  X11Symbols::getInstance()->xDefaultScreen (display)));

            if (targetWindow != newTargetWindow)
            {
                if (targetWindow != None)
                    sendExternalDragAndDropLeave();

                canDrop = false;
                silentRect = {};

                if (newTargetWindow == None)
                    return;

                xdndVersion = getDnDVersionForWindow (newTargetWindow);

                if (xdndVersion == -1)
                    return;

                targetWindow = newTargetWindow;
                sendExternalDragAndDropEnter();
            }

            if (! expectingStatus)
                sendExternalDragAndDropPosition();
        */

    }
    
    pub fn handle_drag_and_drop_position(&mut self, 
        client_msg: &XClientMessageEvent,
        peer:       *mut ComponentPeer)  {
        
        todo!();
        /*
            if (dragAndDropSourceWindow == 0)
                return;

            dragAndDropSourceWindow = (::Window) clientMsg.data.l[0];

            if (windowH == 0)
                windowH = (::Window) peer->getNativeHandle();

            auto dropPos = Desktop::getInstance().getDisplays().physicalToLogical (Point<int> ((int) clientMsg.data.l[2] >> 16,
                                                                                               (int) clientMsg.data.l[2] & 0xffff));
            dropPos -= peer->getBounds().getPosition();

            const auto& atoms = getAtoms();

            auto targetAction = atoms.XdndActionCopy;

            for (int i = numElementsInArray (atoms.allowedActions); --i >= 0;)
            {
                if ((Atom) clientMsg.data.l[4] == atoms.allowedActions[i])
                {
                    targetAction = atoms.allowedActions[i];
                    break;
                }
            }

            sendDragAndDropStatus (true, targetAction);

            if (dragInfo.position != dropPos)
            {
                dragInfo.position = dropPos;

                if (dragInfo.isEmpty())
                    updateDraggedFileList (clientMsg, (::Window) peer->getNativeHandle());

                if (! dragInfo.isEmpty())
                    peer->handleDragMove (dragInfo);
            }
        */

    }
    
    pub fn handle_drag_and_drop_drop(&mut self, 
        client_msg: &XClientMessageEvent,
        peer:       *mut ComponentPeer)  {
        
        todo!();
        /*
            if (dragInfo.isEmpty())
            {
                // no data, transaction finished in handleDragAndDropSelection()
                finishAfterDropDataReceived = true;
                updateDraggedFileList (clientMsg, (::Window) peer->getNativeHandle());
            }
            else
            {
                handleDragAndDropDataReceived();  // data was already received
            }
        */

    }
    
    pub fn handle_drag_and_drop_enter(&mut self, 
        client_msg: &XClientMessageEvent,
        peer:       *mut ComponentPeer)  {
        
        todo!();
        /*
            dragInfo.clear();
            srcMimeTypeAtomList.clear();

            dragAndDropCurrentMimeType = 0;
            auto dndCurrentVersion = (static_cast<unsigned long> (clientMsg.data.l[1]) & 0xff000000) >> 24;

            if (dndCurrentVersion < 3 || dndCurrentVersion > XWindowSystemUtilities::Atoms::DndVersion)
            {
                dragAndDropSourceWindow = 0;
                return;
            }

            const auto& atoms = getAtoms();

            dragAndDropSourceWindow = (::Window) clientMsg.data.l[0];

            if ((clientMsg.data.l[1] & 1) != 0)
            {
                XWindowSystemUtilities::ScopedXLock xLock;
                XWindowSystemUtilities::GetXProperty prop (dragAndDropSourceWindow, atoms.XdndTypeList, 0, 0x8000000L, false, XA_ATOM);

                if (prop.success && prop.actualType == XA_ATOM && prop.actualFormat == 32 && prop.numItems != 0)
                {
                    auto* types = prop.data;

                    for (unsigned long i = 0; i < prop.numItems; ++i)
                    {
                        unsigned long type;
                        memcpy (&type, types, sizeof (unsigned long));

                        if (type != None)
                            srcMimeTypeAtomList.add (type);

                        types += sizeof (unsigned long);
                    }
                }
            }

            if (srcMimeTypeAtomList.isEmpty())
            {
                for (int i = 2; i < 5; ++i)
                    if (clientMsg.data.l[i] != None)
                        srcMimeTypeAtomList.add ((unsigned long) clientMsg.data.l[i]);

                if (srcMimeTypeAtomList.isEmpty())
                {
                    dragAndDropSourceWindow = 0;
                    return;
                }
            }

            for (int i = 0; i < srcMimeTypeAtomList.size() && dragAndDropCurrentMimeType == 0; ++i)
                for (int j = 0; j < numElementsInArray (atoms.allowedMimeTypes); ++j)
                    if (srcMimeTypeAtomList[i] == atoms.allowedMimeTypes[j])
                        dragAndDropCurrentMimeType = atoms.allowedMimeTypes[j];

            handleDragAndDropPosition (clientMsg, peer);
        */

    }
    
    pub fn handle_drag_and_drop_exit(&mut self)  {
        
        todo!();
        /*
            if (auto* peer = getPeerFor (windowH))
                peer->handleDragExit (dragInfo);
        */

    }
    
    pub fn handle_drag_and_drop_selection(&mut self, evt: &XEvent)  {
        
        todo!();
        /*
            dragInfo.clear();

            if (evt.xselection.property != None)
            {
                StringArray lines;

                {
                    MemoryBlock dropData;

                    for (;;)
                    {
                        XWindowSystemUtilities::GetXProperty prop (evt.xany.window, evt.xselection.property,
                                           (long) (dropData.getSize() / 4), 65536, false, AnyPropertyType);

                        if (! prop.success)
                            break;

                        dropData.append (prop.data, (size_t) (prop.actualFormat / 8) * prop.numItems);

                        if (prop.bytesLeft <= 0)
                            break;
                    }

                    lines.addLines (dropData.toString());
                }

                if (XWindowSystemUtilities::Atoms::isMimeTypeFile (getDisplay(), dragAndDropCurrentMimeType))
                {
                    for (const auto& line : lines)
                    {
                        const auto escaped = line.replace ("+", "%2B").replace ("file://", String(), true);
                        dragInfo.files.add (Url::removeEscapeChars (escaped));
                    }

                    dragInfo.files.trim();
                    dragInfo.files.removeEmptyStrings();
                }
                else
                {
                    dragInfo.text = lines.joinIntoString ("\n");
                }

                if (finishAfterDropDataReceived)
                    handleDragAndDropDataReceived();
            }
        */

    }
    
    pub fn external_reset_drag_and_drop(&mut self)  {
        
        todo!();
        /*
            if (dragging)
            {
                XWindowSystemUtilities::ScopedXLock xLock;
                X11Symbols::getInstance()->xUngrabPointer (getDisplay(), CurrentTime);
            }

            if (completionCallback != nullptr)
                completionCallback();
        */

    }
    
    pub fn external_drag_init(&mut self, 
        window: Window,
        text:   bool,
        str_:   &String,
        cb:     fn() -> ()) -> bool {
        
        todo!();
        /*
            windowH            = window;
            isText             = text;
            textOrFiles        = str;
            targetWindow       = windowH;
            completionCallback = std::move (cb);

            auto* display = getDisplay();

            allowedTypes.add (XWindowSystemUtilities::Atoms::getCreating (display, isText ? "text/plain" : "text/uri-list"));

            auto pointerGrabMask = (unsigned int) (Button1MotionMask | ButtonReleaseMask);

            XWindowSystemUtilities::ScopedXLock xLock;

            if (X11Symbols::getInstance()->xGrabPointer (display, windowH, True, pointerGrabMask,
                                                         GrabModeAsync, GrabModeAsync, None, None, CurrentTime) == GrabSuccess)
            {
                const auto& atoms = getAtoms();

                // No other method of changing the pointer seems to work, this call is needed from this very context
                X11Symbols::getInstance()->xChangeActivePointerGrab (display, pointerGrabMask, (Cursor) createDraggingHandCursor(), CurrentTime);

                X11Symbols::getInstance()->xSetSelectionOwner (display, atoms.XdndSelection, windowH, CurrentTime);

                // save the available types to XdndTypeList
                X11Symbols::getInstance()->xChangeProperty (display, windowH, atoms.XdndTypeList, XA_ATOM, 32, PropModeReplace,
                                                            reinterpret_cast<const unsigned char*> (allowedTypes.getRawDataPointer()), allowedTypes.size());

                dragging = true;
                xdndVersion = getDnDVersionForWindow (targetWindow);

                sendExternalDragAndDropEnter();
                handleExternalDragMotionNotify();

                return true;
            }

            return false;
        */

    }
    
    pub fn get_atoms(&self) -> &XWindowSystemAtoms {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->getAtoms();
        */

    }
    
    pub fn get_display(&self) -> *mut Display {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->getDisplay();
        */

    }
    
    pub fn send_drag_and_drop_message(&mut self, msg: &mut XClientMessageEvent)  {
        
        todo!();
        /*
            auto* display = getDisplay();

            msg.type      = ClientMessage;
            msg.display   = display;
            msg.window    = dragAndDropSourceWindow;
            msg.format    = 32;
            msg.data.l[0] = (long) windowH;

            XWindowSystemUtilities::ScopedXLock xLock;
            X11Symbols::getInstance()->xSendEvent (display, dragAndDropSourceWindow, False, 0, (XEvent*) &msg);
        */

    }
    
    pub fn send_external_drag_and_drop_message(&mut self, msg: &mut XClientMessageEvent) -> bool {
        
        todo!();
        /*
            auto* display = getDisplay();

            msg.type      = ClientMessage;
            msg.display   = display;
            msg.window    = targetWindow;
            msg.format    = 32;
            msg.data.l[0] = (long) windowH;

            XWindowSystemUtilities::ScopedXLock xLock;
            return X11Symbols::getInstance()->xSendEvent (display, targetWindow, False, 0, (XEvent*) &msg) != 0;
        */

    }
    
    pub fn send_external_drag_and_drop_drop(&mut self)  {
        
        todo!();
        /*
            XClientMessageEvent msg;
            zerostruct (msg);

            msg.message_type = getAtoms().XdndDrop;
            msg.data.l[2] = CurrentTime;

            sendExternalDragAndDropMessage (msg);
        */

    }
    
    pub fn send_external_drag_and_drop_enter(&mut self)  {
        
        todo!();
        /*
            XClientMessageEvent msg;
            zerostruct (msg);

            msg.message_type = getAtoms().XdndEnter;
            msg.data.l[1] = (xdndVersion << 24);

            for (int i = 0; i < 3; ++i)
                msg.data.l[i + 2] = (long) allowedTypes[i];

            sendExternalDragAndDropMessage (msg);
        */

    }
    
    pub fn send_external_drag_and_drop_position(&mut self)  {
        
        todo!();
        /*
            XClientMessageEvent msg;
            zerostruct (msg);

            const auto& atoms = getAtoms();

            msg.message_type = atoms.XdndPosition;

            auto mousePos = Desktop::getInstance().getMousePosition();

            if (silentRect.contains (mousePos)) // we've been asked to keep silent
                return;

            mousePos = Desktop::getInstance().getDisplays().logicalToPhysical (mousePos);

            msg.data.l[1] = 0;
            msg.data.l[2] = (mousePos.x << 16) | mousePos.y;
            msg.data.l[3] = CurrentTime;
            msg.data.l[4] = (long) atoms.XdndActionCopy; // this is all Aloe currently supports

            expectingStatus = sendExternalDragAndDropMessage (msg);
        */

    }
    
    pub fn send_drag_and_drop_status(&mut self, 
        accept_drop: bool,
        drop_action: Atom)  {
        
        todo!();
        /*
            XClientMessageEvent msg;
            zerostruct (msg);

            msg.message_type = getAtoms().XdndStatus;
            msg.data.l[1]    = (acceptDrop ? 1 : 0) | 2; // 2 indicates that we want to receive position messages
            msg.data.l[4]    = (long) dropAction;

            sendDragAndDropMessage (msg);
        */

    }
    
    pub fn send_external_drag_and_drop_leave(&mut self)  {
        
        todo!();
        /*
            XClientMessageEvent msg;
            zerostruct (msg);

            msg.message_type = getAtoms().XdndLeave;
            sendExternalDragAndDropMessage (msg);
        */

    }
    
    pub fn send_drag_and_drop_finish(&mut self)  {
        
        todo!();
        /*
            XClientMessageEvent msg;
            zerostruct (msg);

            msg.message_type = getAtoms().XdndFinished;
            sendDragAndDropMessage (msg);
        */

    }
    
    pub fn update_dragged_file_list(&mut self, 
        client_msg: &XClientMessageEvent,
        requestor:  Window)  {
        
        todo!();
        /*
            jassert (dragInfo.isEmpty());

            if (dragAndDropSourceWindow != None && dragAndDropCurrentMimeType != None)
            {
                auto* display = getDisplay();

                XWindowSystemUtilities::ScopedXLock xLock;
                X11Symbols::getInstance()->xConvertSelection (display, getAtoms().XdndSelection, dragAndDropCurrentMimeType,
                                                              XWindowSystemUtilities::Atoms::getCreating (display, "JXSelectionWindowProperty"),
                                                              requestor, (::Time) clientMsg.data.l[2]);
            }
        */

    }
    
    pub fn is_window_dn_daware(&self, w: Window) -> bool {
        
        todo!();
        /*
            int numProperties = 0;
            auto* properties = X11Symbols::getInstance()->xListProperties (getDisplay(), w, &numProperties);

            bool dndAwarePropFound = false;

            for (int i = 0; i < numProperties; ++i)
                if (properties[i] == getAtoms().XdndAware)
                    dndAwarePropFound = true;

            if (properties != nullptr)
                X11Symbols::getInstance()->xFree (properties);

            return dndAwarePropFound;
        */

    }
    
    pub fn get_dn_dversion_for_window(&mut self, target: Window) -> i32 {
        
        todo!();
        /*
            XWindowSystemUtilities::GetXProperty prop (target, getAtoms().XdndAware, 0, 2, false, AnyPropertyType);

            if (prop.success && prop.data != None && prop.actualFormat == 32 && prop.numItems == 1)
                return jmin ((int) prop.data[0], (int) XWindowSystemUtilities::Atoms::DndVersion);

            return -1;
        */

    }
    
    pub fn external_find_drag_target_window(&mut self, target: Window) -> Window {
        
        todo!();
        /*
            if (target == None)
                return None;

            if (isWindowDnDAware (target))
                return target;

            ::Window child, phonyWin;
            int phony;
            unsigned int uphony;

            X11Symbols::getInstance()->xQueryPointer (getDisplay(), target, &phonyWin, &child, &phony, &phony, &phony, &phony, &uphony);

            return externalFindDragTargetWindow (child);
        */

    }
    
    pub fn handle_drag_and_drop_data_received(&mut self)  {
        
        todo!();
        /*
            ComponentPeer::DragInfo dragInfoCopy (dragInfo);

            sendDragAndDropFinish();

            if (! dragInfoCopy.isEmpty())
                if (auto* peer = getPeerFor (windowH))
                    peer->handleDragDrop (dragInfoCopy);
        */

    }
}
