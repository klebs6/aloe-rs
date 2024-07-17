crate::ix!();

pub trait GetDesktopWindowStyleFlags {

    fn get_desktop_window_style_flags(&self) -> i32;
}

pub trait ActiveWindowStatusChanged {

    /**
      | This callback happens when this window
      | becomes active or inactive. @see isActiveWindow
      |
      */
    fn active_window_status_changed(&mut self);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_TopLevelWindow.h]

/**
  | A base class for top-level windows.
  | 
  | This class is used for components that
  | are considered a major part of your application
  | - e.g. ResizableWindow, DocumentWindow,
  | DialogWindow, AlertWindow, etc. Things
  | like menus that pop up briefly aren't
  | derived from it.
  | 
  | A TopLevelWindow is probably on the
  | desktop, but this isn't mandatory -
  | it could itself be the child of another
  | component.
  | 
  | The class manages a list of all instances
  | of top-level windows that are in use,
  | and each one is also given the concept
  | of being "active". The active window
  | is one that is actively being used by
  | the user. This isn't quite the same as
  | the component with the keyboard focus,
  | because there may be a popup menu or other
  | temporary window which gets keyboard
  | focus while the active top level window
  | is unchanged.
  | 
  | A top-level window also has an optional
  | drop-shadow.
  | 
  | @see ResizableWindow, DocumentWindow,
  | DialogWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TopLevelWindow<'a> {
    base:                 Component<'a>,
    use_drop_shadow:      bool, // default = true
    use_native_title_bar: bool, // default = false
    is_currently_active:  bool, // default = false
    shadower:             Box<DropShadower<'a>>,
}

impl<'a> Drop for TopLevelWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            shadower.reset();
        TopLevelWindowManager::getInstance()->removeWindow (this);
        */
    }
}

impl<'a> TopLevelWindow<'a> {

    /**
      | True if this is currently the TopLevelWindow
      | that is actively being used.
      | 
      | This isn't quite the same as having keyboard
      | focus, because the focus may be on a child
      | component or a temporary pop-up menu,
      | etc, while this window is still considered
      | to be active.
      | 
      | @see activeWindowStatusChanged
      |
      */
    pub fn is_active_window(&self) -> bool {
        
        todo!();
        /*
            return isCurrentlyActive;
        */
    }

    /**
      | True if drop-shadowing is enabled.
      |
      */
    pub fn is_drop_shadow_enabled(&self) -> bool {
        
        todo!();
        /*
            return useDropShadow;
        */
    }
    
    /**
      | Creates a TopLevelWindow.
      | 
      | -----------
      | @param name
      | 
      | the name to give the component
      | ----------
      | @param addToDesktop
      | 
      | if true, the window will be automatically
      | added to the desktop; if false, you can
      | use it as a child component
      |
      */
    pub fn new(
        name:                  &String,
        should_add_to_desktop: bool) -> Self {
    
        todo!();
        /*
        : component(name),

            setTitle (name);

        setOpaque (true);

        if (shouldAddToDesktop)
            Component::addToDesktop (TopLevelWindow::getDesktopWindowStyleFlags());
        else
            setDropShadowEnabled (true);

        setWantsKeyboardFocus (true);
        setBroughtToFrontOnMouseClick (true);
        isCurrentlyActive = TopLevelWindowManager::getInstance()->addWindow (this);
        */
    }
    
    pub fn focus_of_child_component_changed(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            auto* wm = TopLevelWindowManager::getInstance();

        if (hasKeyboardFocus (true))
            wm->checkFocus();
        else
            wm->checkFocusAsync();
        */
    }
    
    pub fn set_window_active(&mut self, is_now_active: bool)  {
        
        todo!();
        /*
            if (isCurrentlyActive != isNowActive)
        {
            isCurrentlyActive = isNowActive;
            activeWindowStatusChanged();
        }
        */
    }
    
    pub fn active_window_status_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns true if the window is currently
      | using an OS-native title bar. @see setUsingNativeTitleBar
      |
      */
    pub fn is_using_native_title_bar(&self) -> bool {
        
        todo!();
        /*
            return useNativeTitleBar && (isOnDesktop() || ! isShowing());
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
            if (auto* p = getPeer())
                if ((p->getStyleFlags() & (ComponentPeer::windowIsTemporary
                                            | ComponentPeer::windowIgnoresKeyPresses)) == 0)
                    toFront (true);
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            setDropShadowEnabled (useDropShadow);
        */
    }
    
    pub fn get_desktop_window_style_flags(&self) -> i32 {
        
        todo!();
        /*
            int styleFlags = ComponentPeer::windowAppearsOnTaskbar;

        if (useDropShadow)       styleFlags |= ComponentPeer::windowHasDropShadow;
        if (useNativeTitleBar)   styleFlags |= ComponentPeer::windowHasTitleBar;

        return styleFlags;
        */
    }
    
    /**
      | Turns the drop-shadow on and off.
      |
      */
    pub fn set_drop_shadow_enabled(&mut self, use_shadow: bool)  {
        
        todo!();
        /*
            useDropShadow = useShadow;

        if (isOnDesktop())
        {
            shadower.reset();
            Component::addToDesktop (getDesktopWindowStyleFlags());
        }
        else
        {
            if (useShadow && isOpaque())
            {
                if (shadower == nullptr)
                {
                    shadower.reset (getLookAndFeel().createDropShadowerForComponent (this));

                    if (shadower != nullptr)
                        shadower->setOwner (this);
                }
            }
            else
            {
                shadower.reset();
            }
        }
        */
    }
    
    /**
      | Sets whether an OS-native title bar
      | will be used, or a Aloe one. @see isUsingNativeTitleBar
      |
      */
    pub fn set_using_native_title_bar(&mut self, should_use_native_title_bar: bool)  {
        
        todo!();
        /*
            if (useNativeTitleBar != shouldUseNativeTitleBar)
        {
            FocusRestorer focusRestorer;
            useNativeTitleBar = shouldUseNativeTitleBar;
            recreateDesktopWindow();
            sendLookAndFeelChange();
        }
        */
    }
    
    pub fn recreate_desktop_window(&mut self)  {
        
        todo!();
        /*
            if (isOnDesktop())
        {
            Component::addToDesktop (getDesktopWindowStyleFlags());
            toFront (true);
        }
        */
    }
    
    /**
      | Adds the window to the desktop using
      | the default flags.
      |
      */
    pub fn add_to_desktop(&mut self)  {
        
        todo!();
        /*
            shadower.reset();
        Component::addToDesktop (getDesktopWindowStyleFlags());
        setDropShadowEnabled (isDropShadowEnabled()); // force an update to clear away any fake shadows if necessary.
        */
    }
    
    pub fn add_to_desktop_with_window_style_flags(
        &mut self, 
        window_style_flags:         i32,
        native_window_to_attach_to: *mut c_void
    ) {

        todo!();

        /*
            /* It's not recommended to change the desktop window flags directly for a TopLevelWindow,
           because this class needs to make sure its layout corresponds with settings like whether
           it's got a native title bar or not.

           If you need custom flags for your window, you can override the getDesktopWindowStyleFlags()
           method. If you do this, it's best to call the base class's getDesktopWindowStyleFlags()
           method, then add or remove whatever flags are necessary from this value before returning it.
        */
        jassert ((windowStyleFlags & ~ComponentPeer::windowIsSemiTransparent)
                   == (getDesktopWindowStyleFlags() & ~ComponentPeer::windowIsSemiTransparent));

        Component::addToDesktop (windowStyleFlags, nativeWindowToAttachTo);

        if (windowStyleFlags != getDesktopWindowStyleFlags())
            sendLookAndFeelChange();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::window);
        */
    }
    
    /**
      | This will set the bounds of the window
      | so that it's centred in front of another
      | window.
      | 
      | If your app has a few windows open and
      | want to pop up a dialog box for one of them,
      | you can use this to show it in front of
      | the relevant parent window, which is
      | a bit neater than just having it appear
      | in the middle of the screen.
      | 
      | If componentToCentreAround is nullptr,
      | then the currently active TopLevelWindow
      | will be used instead. If no window is
      | focused, it'll just default to the middle
      | of the screen.
      |
      */
    pub fn centre_around_component(&mut self, 
        c:      *mut Component<'a>,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            if (c == nullptr)
            c = TopLevelWindow::getActiveTopLevelWindow();

        if (c == nullptr || c->getBounds().isEmpty())
        {
            centreWithSize (width, height);
        }
        else
        {
            auto targetCentre = c->localPointToGlobal (c->getLocalBounds().getCentre()) / getDesktopScaleFactor();
            auto parentArea = c->getParentMonitorArea();

            if (auto* parent = getParentComponent())
            {
                targetCentre = parent->getLocalPoint (nullptr, targetCentre);
                parentArea   = parent->getLocalBounds();
            }

            setBounds (Rectangle<int> (targetCentre.x - width / 2,
                                       targetCentre.y - height / 2,
                                       width, height)
                         .constrainedWithin (parentArea.reduced (12, 12)));
        }
        */
    }
    
    /**
      | Returns the number of TopLevelWindow
      | objects currently in use. @see getTopLevelWindow
      |
      */
    pub fn get_num_top_level_windows(&mut self) -> i32 {
        
        todo!();
        /*
            return TopLevelWindowManager::getInstance()->windows.size();
        */
    }
    
    /**
      | Returns one of the TopLevelWindow objects
      | currently in use.
      | 
      | The index is 0 to (getNumTopLevelWindows()
      | - 1).
      |
      */
    pub fn get_top_level_window(&mut self, index: i32) -> *mut TopLevelWindow {
        
        todo!();
        /*
            return TopLevelWindowManager::getInstance()->windows [index];
        */
    }
    
    /**
      | Returns the currently-active top level
      | window.
      | 
      | There might not be one, of course, so
      | this can return nullptr.
      |
      */
    pub fn get_active_top_level_window(&mut self) -> *mut TopLevelWindow {
        
        todo!();
        /*
            TopLevelWindow* best = nullptr;
        int bestNumTWLParents = -1;

        for (int i = TopLevelWindow::getNumTopLevelWindows(); --i >= 0;)
        {
            auto* tlw = TopLevelWindow::getTopLevelWindow (i);

            if (tlw->isActiveWindow())
            {
                int numTWLParents = 0;

                for (auto* c = tlw->getParentComponent(); c != nullptr; c = c->getParentComponent())
                    if (dynamic_cast<const TopLevelWindow*> (c) != nullptr)
                        ++numTWLParents;

                if (bestNumTWLParents < numTWLParents)
                {
                    best = tlw;
                    bestNumTWLParents = numTWLParents;
                }
            }
        }

        return best;
        */
    }
}
