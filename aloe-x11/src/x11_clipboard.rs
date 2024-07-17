crate::ix!();

///=============================== X11 - Clipboard ==============================

/**
  | Read the content of a window property as
  | either a locale-dependent string or an utf8
  | string works only for strings shorter than
  | 1000000 bytes
  */
pub fn clipboard_read_window_property(
    display: *mut Display,
    window:  Window,
    atom:    Atom

) -> String {
    
    todo!();
        /*
            if (display != nullptr)
            {
                XWindowSystemUtilities::GetXProperty prop (window, atom, 0L, 100000, false, AnyPropertyType);

                if (prop.success)
                {
                    if (prop.actualType == XWindowSystem::getInstance()->getXWindowSystemAtoms().utf8String && prop.actualFormat == 8)
                        return String::fromUTF8 ((const char*) prop.data, (int) prop.numItems);

                    if (prop.actualType == XA_STRING && prop.actualFormat == 8)
                        return String ((const char*) prop.data, prop.numItems);
                }
            }

            return {};
        */
}

/**
  | Send a SelectionRequest to the window
  | owning the selection and waits for its
  | answer (with a timeout) 
  */
pub fn clipboard_request_selection_content(
    display:           *mut Display,
    selection_content: &mut String,
    selection:         Atom,
    requested_format:  Atom

) -> bool {

    todo!();
        /*
            auto property_name = X11Symbols::getInstance()->xInternAtom (display, "ALOE_SEL", false);

            // The selection owner will be asked to set the ALOE_SEL property on the
            // aloe_messageWindowHandle with the selection content
            X11Symbols::getInstance()->xConvertSelection (display, selection, requestedFormat, property_name,
                                                          aloe_messageWindowHandle, CurrentTime);

            int count = 50; // will wait at most for 200 ms

            while (--count >= 0)
            {
                XEvent event;

                if (X11Symbols::getInstance()->xCheckTypedWindowEvent (display, aloe_messageWindowHandle, SelectionNotify, &event))
                {
                    if (event.xselection.property == property_name)
                    {
                        jassert (event.xselection.requestor == aloe_messageWindowHandle);

                        selectionContent = readWindowProperty (display, event.xselection.requestor, event.xselection.property);
                        return true;
                    }

                    return false;  // the format we asked for was denied.. (event.xselection.property == None)
                }

                // not very elegant.. we could do a select() or something like that...
                // however clipboard content requesting is inherently slow on x11, it
                // often takes 50ms or more so...
                Thread::sleep (4);
            }

            return false;
        */
}

/**
  | Called from the event loop in aloe_linux_Messaging
  | in response to SelectionRequest events
  |
  */
pub fn clipboard_handle_selection(evt: &mut XSelectionRequestEvent)  {
    
    todo!();
        /*
            // the selection content is sent to the target window as a window property
            XSelectionEvent reply;
            reply.type = SelectionNotify;
            reply.display = evt.display;
            reply.requestor = evt.requestor;
            reply.selection = evt.selection;
            reply.target = evt.target;
            reply.property = None; // == "fail"
            reply.time = evt.time;

            HeapBlock<char> data;
            int propertyFormat = 0;
            size_t numDataItems = 0;

            const auto& atoms = XWindowSystem::getInstance()->getXWindowSystemAtoms();

            if (evt.selection == XA_PRIMARY || evt.selection == atoms.clipboard)
            {
                if (evt.target == XA_STRING || evt.target == atoms.utf8String)
                {
                    auto localContent = XWindowSystem::getInstance()->getLocalClipboardContent();

                    // translate to utf8
                    numDataItems = localContent.getNumBytesAsUTF8() + 1;
                    data.calloc (numDataItems);
                    localContent.copyToUTF8 (data, numDataItems);
                    propertyFormat = 8; // bits/item
                }
                else if (evt.target == atoms.targets)
                {
                    // another application wants to know what we are able to send
                    numDataItems = 2;
                    propertyFormat = 32; // atoms are 32-bit
                    data.calloc (numDataItems * 4);

                    auto* dataXWindowSystemAtoms = unalignedPointerCast<Atom*> (data.getData());

                    dataXWindowSystemAtoms[0] = atoms.utf8String;
                    dataXWindowSystemAtoms[1] = XA_STRING;

                    evt.target = XA_ATOM;
                }
            }
            else
            {
                DBG ("requested unsupported clipboard");
            }

            if (data != nullptr)
            {
                const size_t maxReasonableSelectionSize = 1000000;

                // for very big chunks of data, we should use the "INCR" protocol , which is a pain in the *ss
                if (evt.property != None && numDataItems < maxReasonableSelectionSize)
                {
                    X11Symbols::getInstance()->xChangeProperty (evt.display, evt.requestor,
                                                                evt.property, evt.target,
                                                                propertyFormat /* 8 or 32 */, PropModeReplace,
                                                                reinterpret_cast<const unsigned char*> (data.getData()), (int) numDataItems);
                    reply.property = evt.property; // " == success"
                }
            }

            X11Symbols::getInstance()->xSendEvent (evt.display, evt.requestor, 0, NoEventMask, (XEvent*) &reply);
        */
}
