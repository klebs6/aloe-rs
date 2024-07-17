crate::ix!();

#[cfg(target_os="windows")]
#[cfg(ALOE_MINGW)]
pub mod windows_accessibility {
    use super::*;

    pub fn get_uia_root_object_id() -> i64 {
        
        todo!();
            /*
                return -1;
            */
    }


    pub fn handle_wm_get_object(
            _0: *mut AccessibilityHandler,
            _1: WPARAM,
            _2: LPARAM,
            _3: *mut LRESULT) -> bool {
        
        todo!();
            /*
                return false;
            */
    }

    pub fn revoke_uia_map_entries_for_window(_0: HWND)  {
        
        todo!();
            /*
            
            */
    }
}

#[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
pub struct AccessibilityNativeImpl { }

#[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
impl AccessibilityNativeImpl {
    
    pub fn new(_0: &mut AccessibilityHandler) -> Self {
    
        todo!();
        /*


        
        */
    }
}

impl<'a> AccessibilityHandler<'a> {
    
    /**
      | Used to send a notification to any observing
      | accessibility clients that something
      | has changed in the UI element.
      | 
      | @see AccessibilityEvent
      |
      */
    #[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
    pub fn notify_accessibility_event(&self, _0: AccessibilityEvent)  {
        
        todo!();
        /*
        
        */
    }

    #[cfg(ALOE_NATIVE_ACCESSIBILITY_INCLUDED)]
    pub fn notify_accessibility_event(&self, event_type: AccessibilityEvent)  {
        
        todo!();
        /*
            auto notification = [eventType]
        {
            switch (eventType)
            {
                case AccessibilityEvent::textSelectionChanged:  return TYPE_VIEW_TEXT_SELECTION_CHANGED;
                case AccessibilityEvent::textChanged:           return TYPE_VIEW_TEXT_CHANGED;

                case AccessibilityEvent::titleChanged:
                case AccessibilityEvent::structureChanged:      return TYPE_WINDOW_CONTENT_CHANGED;

                case AccessibilityEvent::rowSelectionChanged:
                case AccessibilityEvent::valueChanged:          break;
            }

            return 0;
        }();

        if (notification == 0)
            return;

        const auto contentChangeTypes = [eventType]
        {
            if (eventType == AccessibilityEvent::titleChanged)      return CONTENT_CHANGE_TYPE_CONTENT_DESCRIPTION;
            if (eventType == AccessibilityEvent::structureChanged)  return CONTENT_CHANGE_TYPE_SUBTREE;

            return 0;
        }();

        sendAccessibilityEventImpl (*this, notification, contentChangeTypes);
        */
    }
    
    /**
      | Posts an announcement to be made to the
      | user.
      | 
      | -----------
      | @param announcementString
      | 
      | a localised string containing the announcement
      | to be read out
      | ----------
      | @param priority
      | 
      | the appropriate priority level for
      | the announcement
      |
      */
    #[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
    pub fn post_announcement(&mut self, 
        _0: &String,
        _1: AccessibilityHandlerAnnouncementPriority)  {
        
        todo!();
        /*
        
        */
    }

    #[cfg(ALOE_NATIVE_ACCESSIBILITY_INCLUDED)]
    pub fn post_announcement(
        &mut self, 
        announcement_string: &String,
        _1:                  AccessibilityHandlerAnnouncementPriority)  {
        
        todo!();
        /*
            if (! areAnyAccessibilityClientsActive())
            return;

        const auto rootView = []
        {
            LocalRef<jobject> activity (getMainActivity());

            if (activity != nullptr)
            {
                auto* env = getEnv();

                LocalRef<jobject> mainWindow (env->CallObjectMethod (activity.get(), AndroidActivity.getWindow));
                LocalRef<jobject> decorView (env->CallObjectMethod (mainWindow.get(), AndroidWindow.getDecorView));

                return LocalRef<jobject> (env->CallObjectMethod (decorView.get(), AndroidView.getRootView));
            }

            return LocalRef<jobject>();
        }();

        if (rootView != nullptr)
            getEnv()->CallVoidMethod (rootView.get(),
                                      AndroidView.announceForAccessibility,
                                      javaString (announcementString).get());
        */
    }
    
    #[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
    pub fn get_native_implementation(&self) -> *mut AccessibilityNativeHandle {
        
        todo!();
        /*
            return nullptr;
        */
    }

    #[cfg(ALOE_NATIVE_ACCESSIBILITY_INCLUDED)]
    pub fn get_native_implementation(&self) -> *mut AccessibilityNativeHandle {
        
        todo!();
        /*
            return nativeImpl.get();
        */
    }
    
    
    #[cfg(not(ALOE_NATIVE_ACCESSIBILITY_INCLUDED))]
    pub fn create_native_impl(&mut self, _0: &mut AccessibilityHandler) -> Box<AccessibilityNativeImpl> {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    ///----------------------------------------
    #[cfg(ALOE_NATIVE_ACCESSIBILITY_INCLUDED)]
    pub fn create_native_impl(&mut self, handler: &mut AccessibilityHandler) -> Box<AccessibilityNativeImpl> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityNativeImpl> (handler);
        */
    }
}

