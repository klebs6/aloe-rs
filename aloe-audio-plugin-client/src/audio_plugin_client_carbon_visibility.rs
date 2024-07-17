crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_CarbonVisibility.h]

/** 
  | When you wrap a WindowRef as an NSWindow, it
  | seems to bugger up the HideWindow
  | function, so when the host tries (and fails) to
  | hide the window, this stuff catches the event
  | and forces it to update.
  */
#[cfg(all(ALOE_SUPPORT_CARBON,ALOE_MAC_WINDOW_VISIBITY_BODGE))]
pub fn window_visibility_bodge(
        _0:   EventHandlerCallRef,
        e:    EventRef,
        user: *mut c_void) -> OSStatus {
    
    todo!();
        /*
            NSWindow* hostWindow = (NSWindow*) user;

        switch (GetEventKind (e))
        {
            case kEventWindowInit:    [hostWindow display]; break;
            case kEventWindowShown:   [hostWindow orderFront: nil]; break;
            case kEventWindowHidden:  [hostWindow orderOut: nil]; break;
        }

        return eventNotHandledErr;
        */
}

#[cfg(all(ALOE_SUPPORT_CARBON,ALOE_MAC_WINDOW_VISIBITY_BODGE))]
#[inline] pub fn attach_window_hiding_hooks(
        comp:            *mut Component,
        host_window_ref: *mut c_void,
        ns_window:       *mut NSWindow)  {
    
    todo!();
        /*
            const EventTypeSpec eventsToCatch[] =
        {
            { kEventClassWindow, kEventWindowInit },
            { kEventClassWindow, kEventWindowShown },
            { kEventClassWindow, kEventWindowHidden }
        };

        EventHandlerRef ref;
        InstallWindowEventHandler ((WindowRef) hostWindowRef,
                                   NewEventHandlerUPP (windowVisibilityBodge),
                                   GetEventTypeCount (eventsToCatch), eventsToCatch,
                                   (void*) nsWindow, &ref);

        comp->getProperties().set ("carbonEventRef", String::toHexString ((pointer_sized_int) (void*) ref));
        */
}

#[cfg(all(ALOE_SUPPORT_CARBON,ALOE_MAC_WINDOW_VISIBITY_BODGE))]
#[inline] pub fn remove_window_hiding_hooks(comp: *mut Component)  {
    
    todo!();
        /*
            if (comp != nullptr)
            RemoveEventHandler ((EventHandlerRef) (void*) (pointer_sized_int)
                                  comp->getProperties() ["carbonEventRef"].toString().getHexValue64());
        */
}

///-------------------------------
#[cfg(target_os="macos")]
#[inline] pub fn attach_window_hiding_hooks(
        _0: *mut c_void,
        _1: *mut c_void,
        _2: *mut c_void)  {
    
    todo!();
        /*
        
        */
}


#[cfg(target_os="macos")]
#[inline] pub fn remove_window_hiding_hooks(_0: *mut c_void)  {
    
    todo!();
        /*
        
        */
}
