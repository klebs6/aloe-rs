crate::ix!();

pub type EventHandlerCallRef = *mut c_void;
pub type EventHandlerRef     = *mut c_void;
pub type EventRef            = *mut c_void;
pub type HIViewRef           = *mut c_void;
pub type NSWindow            = *mut c_void;
pub type OSStatus            = i32;  // This is usually defined as a signed integer type
pub type WindowRef           = *mut c_void;

/**
  | Non-public utility function that hosts can use
  | if they need to get hold of the internals of
  | a carbon wrapper window..
  */
pub fn get_carbon_window<'a>(possible_carbon_component: *mut Component<'a>)  {
    
    todo!();
        /*
            if (CarbonViewWrapperComponent* cv = dynamic_cast<CarbonViewWrapperComponent*> (possibleCarbonComponent))
            return cv->carbonWindow;

        return nullptr;
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_mac_CarbonViewWrapperComponent.h]

pub trait CarbonViewWrapperComponentInterface {

    fn attach_view(&mut self, 
            window_ref: WindowRef,
            root_view:  HIViewRef) -> HIViewRef;

    fn remove_view(&mut self, embedded_view: HIViewRef);

    fn handle_mouse_down(&mut self, _0: i32, _1: i32);

    fn handle_paint(&mut self);

    fn get_embedded_view_size(&mut self, 
        w: &mut i32,
        h: &mut i32) -> bool {
        
        todo!();
        /*
            if (embeddedView == 0)
                return false;

            HIRect bounds;
            HIViewGetBounds (embeddedView, &bounds);
            w = jmax (1, roundToInt (bounds.size.width));
            h = jmax (1, roundToInt (bounds.size.height));
            return true;
        */
    }
}

/**
  | Creates a floating carbon window that
  | can be used to hold a carbon UI.
  | 
  | This is a handy class that's designed
  | to be inlined where needed, e.g. in the
  | audio plugin hosting code.
  | 
  | @tags{GUI}
  |
  */
pub struct CarbonViewWrapperComponent<'a> {
    base:                           Component<'a>,
    base2:                          ComponentMovementWatcher<'a>,
    base3:                          Timer,
    carbon_window:                  *mut NSWindow,
    keep_plugin_window_when_hidden: bool,
    wrapper_window:                 WindowRef,
    embedded_view:                  HIViewRef,
    recursive_resize:               bool,
    repaint_child_on_creation:      bool,
    creation_time:                  Time,
    event_handler_ref:              EventHandlerRef,
}

impl<'a> Drop for CarbonViewWrapperComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            jassert (embeddedView == 0); // must call deleteWindow() in the subclass's destructor!
        */
    }
}

impl<'a> Default for CarbonViewWrapperComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : component_movement_watcher(this),
        : carbon_window(nil),
        : keep_plugin_window_when_hidden(false),
        : wrapper_window(nil),
        : embedded_view(0),
        : recursive_resize(false),
        : repaint_child_on_creation(true),

        
        */
    }
}

impl<'a> CarbonViewWrapperComponent<'a> {

    pub fn create_window(&mut self)  {
        
        todo!();
        /*
            if (wrapperWindow == nil)
            {
                Rect r;
                r.left   = (short) getScreenX();
                r.top    = (short) getScreenY();
                r.right  = (short) (r.left + getWidth());
                r.bottom = (short) (r.top + getHeight());

                CreateNewWindow (kDocumentWindowClass,
                                 (WindowAttributes) (kWindowStandardHandlerAttribute | kWindowCompositingAttribute
                                                      | kWindowNoShadowAttribute | kWindowNoTitleBarAttribute),
                                 &r, &wrapperWindow);

                jassert (wrapperWindow != 0);
                if (wrapperWindow == 0)
                    return;

                carbonWindow = [[NSWindow alloc] initWithWindowRef: wrapperWindow];

                [getOwnerWindow() addChildWindow: carbonWindow
                                         ordered: NSWindowAbove];

                embeddedView = attachView (wrapperWindow, HIViewGetRoot (wrapperWindow));

                // Check for the plugin creating its own floating window, and if there is one,
                // we need to reparent it to make it visible..
                if (carbonWindow.childWindows.count > 0)
                    if (NSWindow* floatingChildWindow = [[carbonWindow childWindows] objectAtIndex: 0])
                        [getOwnerWindow() addChildWindow: floatingChildWindow
                                                 ordered: NSWindowAbove];

                EventTypeSpec windowEventTypes[] =
                {
                    { kEventClassWindow, kEventWindowGetClickActivation },
                    { kEventClassWindow, kEventWindowHandleDeactivate },
                    { kEventClassWindow, kEventWindowBoundsChanging },
                    { kEventClassMouse,  kEventMouseDown },
                    { kEventClassMouse,  kEventMouseMoved },
                    { kEventClassMouse,  kEventMouseDragged },
                    { kEventClassMouse,  kEventMouseUp },
                    { kEventClassWindow, kEventWindowDrawContent },
                    { kEventClassWindow, kEventWindowShown },
                    { kEventClassWindow, kEventWindowHidden }
                };

                EventHandlerUPP upp = NewEventHandlerUPP (carbonEventCallback);
                InstallWindowEventHandler (wrapperWindow, upp,
                                           sizeof (windowEventTypes) / sizeof (EventTypeSpec),
                                           windowEventTypes, this, &eventHandlerRef);

                setOurSizeToEmbeddedViewSize();
                setEmbeddedWindowToOurSize();

                creationTime = Time::getCurrentTime();
            }
        */
    }
    
    pub fn delete_window(&mut self)  {
        
        todo!();
        /*
            removeView (embeddedView);
            embeddedView = 0;

            if (wrapperWindow != nil)
            {
                NSWindow* ownerWindow = getOwnerWindow();

                if ([[ownerWindow childWindows] count] > 0)
                {
                    [ownerWindow removeChildWindow: carbonWindow];
                    [carbonWindow close];
                }

                RemoveEventHandler (eventHandlerRef);
                DisposeWindow (wrapperWindow);
                wrapperWindow = nil;
            }
        */
    }
    
    pub fn set_our_size_to_embedded_view_size(&mut self)  {
        
        todo!();
        /*
            int w, h;
            if (getEmbeddedViewSize (w, h))
            {
                if (w != getWidth() || h != getHeight())
                {
                    startTimer (50);
                    setSize (w, h);

                    if (Component* p = getParentComponent())
                        p->setSize (w, h);
                }
                else
                {
                    startTimer (jlimit (50, 500, getTimerInterval() + 20));
                }
            }
            else
            {
                stopTimer();
            }
        */
    }
    
    pub fn set_embedded_window_to_our_size(&mut self)  {
        
        todo!();
        /*
            if (! recursiveResize)
            {
                recursiveResize = true;

                if (embeddedView != 0)
                {
                    HIRect r;
                    r.origin.x = 0;
                    r.origin.y = 0;
                    r.size.width  = (float) getWidth();
                    r.size.height = (float) getHeight();
                    HIViewSetFrame (embeddedView, &r);
                }

                if (wrapperWindow != nil)
                {
                    jassert (getTopLevelComponent()->getDesktopScaleFactor() == 1.0f);
                    Rectangle<int> screenBounds (getScreenBounds() * Desktop::getInstance().getGlobalScaleFactor());

                    Rect wr;
                    wr.left   = (short) screenBounds.getX();
                    wr.top    = (short) screenBounds.getY();
                    wr.right  = (short) screenBounds.getRight();
                    wr.bottom = (short) screenBounds.getBottom();

                    SetWindowBounds (wrapperWindow, kWindowContentRgn, &wr);

                    // This group stuff is mainly a workaround for Mackie plugins like FinalMix..
                    WindowGroupRef group = GetWindowGroup (wrapperWindow);
                    WindowRef attachedWindow;

                    if (GetIndexedWindow (group, 2, kWindowGroupContentsReturnWindows, &attachedWindow) == noErr)
                    {
                        SelectWindow (attachedWindow);
                        ActivateWindow (attachedWindow, TRUE);
                        HideWindow (wrapperWindow);
                    }

                    ShowWindow (wrapperWindow);
                }

                recursiveResize = false;
            }
        */
    }
    
    pub fn component_moved_or_resized(
        &mut self, 
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            setEmbeddedWindowToOurSize();
        */
    }

    /**
       (overridden to intercept movements of the
       top-level window)
      */
    pub fn component_moved_or_resized_with_component(
        &mut self, 
        component:   &mut Component<'a>,
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            ComponentMovementWatcher::componentMovedOrResized (component, wasMoved, wasResized);

            if (&component == getTopLevelComponent())
                setEmbeddedWindowToOurSize();
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            deleteWindow();
            createWindow();
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
                createWindow();
            else if (! keepPluginWindowWhenHidden)
                deleteWindow();

            setEmbeddedWindowToOurSize();
        */
    }
    
    pub fn recursive_hi_view_repaint(view: HIViewRef)  {
        
        todo!();
        /*
            HIViewSetNeedsDisplay (view, true);
            HIViewRef child = HIViewGetFirstSubview (view);

            while (child != 0)
            {
                recursiveHIViewRepaint (child);
                child = HIViewGetNextView (child);
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
            {
                setOurSizeToEmbeddedViewSize();

                // To avoid strange overpainting problems when the UI is first opened, we'll
                // repaint it a few times during the first second that it's on-screen..
                if (repaintChildOnCreation && (Time::getCurrentTime() - creationTime).inMilliseconds() < 1000)
                    recursiveHIViewRepaint (HIViewGetRoot (wrapperWindow));
            }
        */
    }
    
    pub fn set_repaints_child_hi_view_when_created(&mut self, b: bool)  {
        
        todo!();
        /*
            repaintChildOnCreation = b;
        */
    }
    
    pub fn carbon_event_handler(&mut self, 
        next_handler_ref: EventHandlerCallRef,
        event:            EventRef) -> OSStatus {
        
        todo!();
        /*
            switch (GetEventKind (event))
            {
                case kEventWindowHandleDeactivate:
                    ActivateWindow (wrapperWindow, TRUE);
                    return noErr;

                case kEventWindowGetClickActivation:
                {
                    getTopLevelComponent()->toFront (false);
                    [carbonWindow makeKeyAndOrderFront: nil];

                    ClickActivationResult howToHandleClick = kActivateAndHandleClick;

                    SetEventParameter (event, kEventParamClickActivation, typeClickActivationResult,
                                       sizeof (ClickActivationResult), &howToHandleClick);

                    if (embeddedView != 0)
                        HIViewSetNeedsDisplay (embeddedView, true);

                    return noErr;
                }
            }

            return eventNotHandledErr;
        */
    }
    
    pub fn carbon_event_callback(
        next_handler_ref: EventHandlerCallRef,
        event:            EventRef,
        user_data:        *mut c_void) -> OSStatus {
        
        todo!();
        /*
            return ((CarbonViewWrapperComponent*) userData)->carbonEventHandler (nextHandlerRef, event);
        */
    }
    
    pub fn get_owner_window(&self) -> *mut NSWindow {
        
        todo!();
        /*
            return [((NSView*) getWindowHandle()) window];
        */
    }
}
