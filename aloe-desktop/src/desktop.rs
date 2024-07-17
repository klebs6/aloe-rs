crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/desktop/aloe_Desktop.h]

//TODO
pub struct MouseInputSourceList<'a> {
    phantom: PhantomData<&'a i32>,
}

/**
  | Describes and controls aspects of the
  | computer's desktop.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Desktop<'a> {
    base:                            DeletedAtShutdown,
    base2:                           Timer,
    base3:                           AsyncUpdater<'a>,
    mouse_sources:                   Box<MouseInputSourceList<'a>>,
    mouse_listeners:                 ListenerList<Rc<RefCell<dyn MouseListener>>>,
    focus_listeners:                 ListenerList<Rc<RefCell<dyn FocusChangeListener>>>,
    desktop_components:              Vec<*mut Component<'a>>,
    peers:                           Vec<*mut ComponentPeer<'a>>,
    displays:                        Box<Displays>,
    last_fake_mouse_move:            Point<f32>,
    mouse_click_counter:             i32, // default = 0
    mouse_wheel_counter:             i32, // default = 0
    default_look_and_feel:           Box<dyn LookAndFeelDesktopInterface>,
    current_look_and_feel:           WeakReference<Box<dyn LookAndFeelDesktopInterface>>,
    kiosk_mode_component:            *mut Component<'a>, // default = nullptr
    kiosk_component_original_bounds: Rectangle<i32>,
    kiosk_mode_reentrant:            bool, // default = false
    allowed_orientations:            i32, // default = allOrientations
    master_scale_factor:             f32,
    animator:                        ComponentAnimator<'a>,
}

//TODO
pub trait LookAndFeelDesktopInterface {}

lazy_static!{
    /*
    static Desktop* instance;
    Desktop* Desktop::instance = nullptr;
    */
}

impl<'a> Drop for Desktop<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        setScreenSaverEnabled (true);
        animator.cancelAllAnimations (false);

        jassert (instance == this);
        instance = nullptr;

        // doh! If you don't delete all your windows before exiting, you're going to
        // be leaking memory!
        jassert (desktopComponents.size() == 0);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/desktop/aloe_Desktop.cpp]
impl<'a> Desktop<'a> {
    
    /**
      | This lets you prevent the screensaver
      | from becoming active.
      | 
      | Handy if you're running some sort of
      | presentation app where having a screensaver
      | appear would be annoying.
      | 
      | Pass false to disable the screensaver,
      | and true to re-enable it. (Note that
      | this won't enable a screensaver unless
      | the user has actually set one up).
      | 
      | The disablement will only happen while
      | the Aloe application is the foreground
      | process - if another task is running
      | in front of it, then the screensaver
      | will be unaffected.
      | 
      | @see isScreenSaverEnabled
      |
      */
    pub fn set_screen_saver_enabled(is_enabled: bool)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if the screensaver has
      | not been turned off.
      | 
      | This will return the last value passed
      | into setScreenSaverEnabled(). Note
      | that it won't tell you whether the user
      | is actually using a screen saver, just
      | whether this app is deliberately preventing
      | one from running.
      | 
      | @see setScreenSaverEnabled
      |
      */
    pub fn is_screen_saver_enabled() -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the component that is currently
      | being used in kiosk-mode.
      | 
      | This is the component that was last set
      | by setKioskModeComponent(). If none
      | has been set, this returns nullptr.
      |
      */
    pub fn get_kiosk_mode_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return kioskModeComponent;
        */
    }

    /**
      | The Desktop object has a ComponentAnimator
      | instance which can be used for performing
      | your animations.
      | 
      | Having a single shared ComponentAnimator
      | object makes it more efficient when
      | multiple components are being moved
      | around simultaneously. It's also more
      | convenient than having to manage your
      | own instance of one.
      | 
      | @see ComponentAnimator
      |
      */
    pub fn get_animator(&mut self) -> &mut ComponentAnimator {
        
        todo!();
        /*
            return animator;
        */
    }

    /**
      | Returns the Displays object representing
      | the connected displays.
      | 
      | @see Displays
      |
      */
    pub fn get_displays(&self) -> &Displays {
        
        todo!();
        /*
            return *displays;
        */
    }

    /**
      | Returns the current global scale factor,
      | as set by setGlobalScaleFactor().
      | @see setGlobalScaleFactor
      |
      */
    pub fn get_global_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return masterScaleFactor;
        */
    }

    /**
      | OSX-specific function to check for
      | the "dark" title-bar and menu mode.
      |
      */
    #[cfg(target_os="macos")]
    pub fn is_osx_dark_mode_active() -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn allowed_orientations_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_kiosk_component(&mut self, 
        _0:                   *mut Component,
        should_be_enabled:    bool,
        allow_menus_and_bars: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : mouseSources (new MouseInputSource::SourceList()),
          masterScaleFactor ((float) getDefaultMasterScale())

        displays.reset (new Displays (*this));
        */
    }
    
    /**
      | There's only one desktop object, and
      | this method will return it.
      |
      */
    pub fn get_instance(&mut self) -> &mut Desktop {
        
        todo!();
        /*
            if (instance == nullptr)
            instance = new Desktop();

        return *instance;
        */
    }
    
    /**
      | Returns the number of components that
      | are currently active as top-level desktop
      | windows.
      | 
      | @see getComponent, Component::addToDesktop
      |
      */
    pub fn get_num_components(&self) -> i32 {
        
        todo!();
        /*
            return desktopComponents.size();
        */
    }
    
    /**
      | Returns one of the top-level desktop
      | window components.
      | 
      | The index is from 0 to getNumComponents()
      | - 1. This could return 0 if the index is
      | out-of-range.
      | 
      | @see getNumComponents, Component::addToDesktop
      |
      */
    pub fn get_component(&self, index: i32) -> *mut Component {
        
        todo!();
        /*
            return desktopComponents [index];
        */
    }
    
    /**
      | Finds the component at a given screen
      | location.
      | 
      | This will drill down into top-level
      | windows to find the child component
      | at the given position.
      | 
      | Returns nullptr if the coordinates
      | are inside a non-Aloe window.
      |
      */
    pub fn find_component_at(&self, screen_position: Point<i32>) -> *mut Component {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        for (int i = desktopComponents.size(); --i >= 0;)
        {
            auto* c = desktopComponents.getUnchecked(i);

            if (c->isVisible())
            {
                auto relative = c->getLocalPoint (nullptr, screenPosition);

                if (c->contains (relative))
                    return c->getComponentAt (relative);
            }
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the current default look-and-feel
      | for components which don't have one
      | explicitly set. @see setDefaultLookAndFeel
      |
      */
    pub fn get_default_look_and_feel(&mut self) -> &mut dyn LookAndFeelDesktopInterface {
        
        todo!();
        /*
            if (auto lf = currentLookAndFeel.get())
            return *lf;

        if (defaultLookAndFeel == nullptr)
            defaultLookAndFeel.reset (new LookAndFeel_V4());

        auto lf = defaultLookAndFeel.get();
        jassert (lf != nullptr);
        currentLookAndFeel = lf;
        return *lf;
        */
    }
    
    /**
      | Changes the default look-and-feel.
      | 
      | -----------
      | @param newDefaultLookAndFeel
      | 
      | the new look-and-feel object to use
      | - if this is set to nullptr, it will revert
      | to using the system's default one. The
      | object passed-in must be deleted by
      | the caller when it's no longer needed.
      | @see getDefaultLookAndFeel
      |
      */
    pub fn set_default_look_and_feel(&mut self, new_default_look_and_feel: *mut dyn LookAndFeelDesktopInterface)  {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED
        currentLookAndFeel = newDefaultLookAndFeel;

        for (int i = getNumComponents(); --i >= 0;)
            if (auto* c = getComponent (i))
                c->sendLookAndFeelChange();
        */
    }
    
    pub fn add_desktop_component(&mut self, c: *mut Component)  {
        
        todo!();
        /*
            jassert (c != nullptr);
        jassert (! desktopComponents.contains (c));
        desktopComponents.addIfNotAlreadyThere (c);
        */
    }
    
    pub fn remove_desktop_component(&mut self, c: *mut Component)  {
        
        todo!();
        /*
            desktopComponents.removeFirstMatchingValue (c);
        */
    }
    
    pub fn component_brought_to_front(&mut self, c: *mut Component)  {
        
        todo!();
        /*
            auto index = desktopComponents.indexOf (c);
        jassert (index >= 0);

        if (index >= 0)
        {
            int newIndex = -1;

            if (! c->isAlwaysOnTop())
            {
                newIndex = desktopComponents.size();

                while (newIndex > 0 && desktopComponents.getUnchecked (newIndex - 1)->isAlwaysOnTop())
                    --newIndex;

                --newIndex;
            }

            desktopComponents.move (index, newIndex);
        }
        */
    }
    
    /**
      | Returns the mouse position.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor.
      | 
      | -----------
      | @note
      | 
      | this is just a shortcut for calling getMainMouseSource().getScreenPosition(),
      | and you should only resort to grabbing
      | the global mouse position if there's
      | really no way to get the coordinates
      | via a mouse event callback instead.
      |
      */
    pub fn get_mouse_position(&mut self) -> Point<i32> {
        
        todo!();
        /*
            return getMousePositionFloat().roundToInt();
        */
    }
    
    pub fn get_mouse_position_float(&mut self) -> Point<f32> {
        
        todo!();
        /*
            return getInstance().getMainMouseSource().getScreenPosition();
        */
    }
    
    /**
      | Makes the mouse pointer jump to a given
      | location.
      | 
      | The coordinates are relative to the
      | top-left of the main monitor.
      | 
      | -----------
      | @note
      | 
      | this is a pretty old method, kept around
      | mainly for backwards-compatibility,
      | and you should use the MouseInputSource
      | class directly in new code.
      |
      */
    pub fn set_mouse_position(&mut self, new_position: Point<i32>)  {
        
        todo!();
        /*
            getInstance().getMainMouseSource().setScreenPosition (newPosition.toFloat());
        */
    }
    
    /**
      | Returns the last position at which a
      | mouse button was pressed.
      | 
      | -----------
      | @note
      | 
      | this is just a shortcut for calling getMainMouseSource().getLastMouseDownPosition(),
      | and in a multi-touch environment, it
      | doesn't make much sense. ALWAYS prefer
      | to get this information via other means,
      | such as MouseEvent::getMouseDownScreenPosition()
      | if possible, and only ever call this
      | as a last resort.
      |
      */
    pub fn get_last_mouse_down_position(&mut self) -> Point<i32> {
        
        todo!();
        /*
            return getInstance().getMainMouseSource().getLastMouseDownPosition().roundToInt();
        */
    }
    
    /**
      | Returns the number of times the mouse
      | button has been clicked since the app
      | started.
      | 
      | Each mouse-down event increments this
      | number by 1. @see getMouseWheelMoveCounter
      |
      */
    pub fn get_mouse_button_click_counter(&self) -> i32 {
        
        todo!();
        /*
            return mouseClickCounter;
        */
    }
    
    /**
      | Returns the number of times the mouse
      | wheel has been moved since the app started.
      | 
      | Each mouse-wheel event increments
      | this number by 1. @see getMouseButtonClickCounter
      |
      */
    pub fn get_mouse_wheel_move_counter(&self) -> i32 {
        
        todo!();
        /*
            return mouseWheelCounter;
        */
    }
    
    pub fn increment_mouse_click_counter(&mut self)  {
        
        todo!();
        /*
            ++mouseClickCounter;
        */
    }
    
    pub fn increment_mouse_wheel_counter(&mut self)  {
        
        todo!();
        /*
            ++mouseWheelCounter;
        */
    }
    
    /**
      | Provides access to the array of mouse
      | sources, for iteration.
      | 
      | In a traditional single-mouse system,
      | there might be only one MouseInputSource.
      | On a multi-touch system, there could
      | be one input source per potential finger.
      | The number of mouse sources returned
      | here may increase dynamically as the
      | program runs.
      | 
      | To find out how many mouse events are
      | currently happening, use getNumDraggingMouseSources().
      |
      */
    pub fn get_mouse_sources(&self) -> &[MouseInputSource] {
        
        todo!();
        /*
            return mouseSources->sourceArray;
        */
    }
    
    /**
      | Returns the number of MouseInputSource
      | objects the system has at its disposal.
      | 
      | In a traditional single-mouse system,
      | there might be only one MouseInputSource.
      | On a multi-touch system, there could
      | be one input source per potential finger.
      | The number of mouse sources returned
      | here may increase dynamically as the
      | program runs.
      | 
      | To find out how many mouse events are
      | currently happening, use getNumDraggingMouseSources().
      | @see getMouseSource
      |
      */
    pub fn get_num_mouse_sources(&self) -> i32 {
        
        todo!();
        /*
            return mouseSources->sources.size();
        */
    }
    
    /**
      | Returns the number of mouse-sources
      | that are currently being dragged.
      | 
      | In a traditional single-mouse system,
      | this will be 0 or 1, depending on whether
      | a
      | 
      | Aloe component has the button down on
      | it. In a multi-touch system, this could
      | be any number from 0 to the number of simultaneous
      | touches that can be detected.
      |
      */
    pub fn get_num_dragging_mouse_sources(&self) -> i32 {
        
        todo!();
        /*
            return mouseSources->getNumDraggingMouseSources();
        */
    }
    
    /**
      | Returns one of the system's MouseInputSource
      | objects.
      | 
      | The index should be from 0 to getNumMouseSources()
      | - 1. Out-of-range indexes will return
      | a null pointer.
      | 
      | In a traditional single-mouse system,
      | there might be only one object. On a multi-touch
      | system, there could be one input source
      | per potential finger.
      |
      */
    pub fn get_mouse_source(&self, index: i32) -> *mut MouseInputSource {
        
        todo!();
        /*
            return mouseSources->getMouseSource (index);
        */
    }
    
    /**
      | Returns one of the mouse sources that's
      | currently being dragged.
      | 
      | The index should be between 0 and getNumDraggingMouseSources()
      | - 1. If the index is out of range, or if
      | no mice or fingers are down, this will
      | return a null pointer.
      |
      */
    pub fn get_dragging_mouse_source(&self, index: i32) -> *mut MouseInputSource {
        
        todo!();
        /*
            return mouseSources->getDraggingMouseSource (index);
        */
    }
    
    /**
      | Returns the main mouse input device
      | that the system is using. @see getNumMouseSources()
      |
      */
    pub fn get_main_mouse_source(&self) -> MouseInputSource {
        
        todo!();
        /*
            return MouseInputSource (mouseSources->sources.getUnchecked(0));
        */
    }
    
    /**
      | Ensures that a non-stop stream of mouse-drag
      | events will be sent during the current
      | mouse-drag operation.
      | 
      | This allows you to make sure that mouseDrag()
      | events are sent continuously, even
      | when the mouse isn't moving. This can
      | be useful for things like auto-scrolling
      | components when the mouse is near an
      | edge.
      | 
      | Call this method during a mouseDown()
      | or mouseDrag() callback, specifying
      | the minimum interval between consecutive
      | mouse drag callbacks. The callbacks
      | will continue until the mouse is released,
      | and then the interval will be reset,
      | so you need to make sure it's called every
      | time you begin a drag event.
      | 
      | Passing an interval of 0 or less will
      | cancel the auto-repeat.
      | 
      | @see mouseDrag
      |
      */
    pub fn begin_drag_auto_repeat(&mut self, interval: i32)  {
        
        todo!();
        /*
            mouseSources->beginDragAutoRepeat (interval);
        */
    }
    
    /**
      | Registers a FocusChangeListener that
      | will receive a callback whenever the
      | focused component changes.
      | 
      | @see removeFocusChangeListener
      |
      */
    pub fn add_focus_change_listener(&mut self, l: *mut dyn FocusChangeListener)  {
        
        todo!();
        /*
            focusListeners.add (l);
        */
    }
    
    /**
      | Unregisters a FocusChangeListener
      | that was added with the addFocusChangeListener()
      | method.
      | 
      | @see addFocusChangeListener
      |
      */
    pub fn remove_focus_change_listener(&mut self, l: *mut dyn FocusChangeListener)  {
        
        todo!();
        /*
            focusListeners.remove (l);
        */
    }
    
    pub fn trigger_focus_callback(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            // The component may be deleted during this operation, but we'll use a SafePointer rather than a
        // BailOutChecker so that any remaining listeners will still get a callback (with a null pointer).
        focusListeners.call ([currentFocus = WeakReference<Component> { Component::getCurrentlyFocusedComponent() }] (FocusChangeListener& l)
        {
            l.globalFocusChanged (currentFocus.get());
        });
        */
    }
    
    pub fn reset_timer(&mut self)  {
        
        todo!();
        /*
            if (mouseListeners.size() == 0)
            stopTimer();
        else
            startTimer (100);

        lastFakeMouseMove = getMousePositionFloat();
        */
    }
    
    pub fn get_mouse_listeners(&mut self) -> &mut ListenerList<Rc<RefCell<dyn MouseListener>>> {
        
        todo!();
        /*
            resetTimer();
        return mouseListeners;
        */
    }
    
    /**
      | Registers a MouseListener that will
      | receive all mouse events that occur
      | on any component.
      | 
      | @see removeGlobalMouseListener
      |
      */
    pub fn add_global_mouse_listener(&mut self, listener: *mut dyn MouseListener)  {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED
        mouseListeners.add (listener);
        resetTimer();
        */
    }
    
    /**
      | Unregisters a MouseListener that was
      | added with the addGlobalMouseListener()
      | method.
      | 
      | @see addGlobalMouseListener
      |
      */
    pub fn remove_global_mouse_listener(&mut self, listener: *mut dyn MouseListener)  {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED
        mouseListeners.remove (listener);
        resetTimer();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (lastFakeMouseMove != getMousePositionFloat())
            sendMouseMove();
        */
    }
    
    pub fn send_mouse_move(&mut self)  {
        
        todo!();
        /*
            if (! mouseListeners.isEmpty())
        {
            startTimer (20);

            lastFakeMouseMove = getMousePositionFloat();

            if (auto* target = findComponentAt (lastFakeMouseMove.roundToInt()))
            {
                Component::BailOutChecker checker (target);
                auto pos = target->getLocalPoint (nullptr, lastFakeMouseMove);
                auto now = Time::getCurrentTime();

                const MouseEvent me (getMainMouseSource(), pos, ModifierKeys::currentModifiers, MouseInputSource::invalidPressure,
                                     MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                     MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                     target, target, now, pos, now, 0, false);

                if (me.mods.isAnyMouseButtonDown())
                    mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseDrag (me); });
                else
                    mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseMove (me); });
            }
        }
        */
    }
    
    /**
      | Takes a component and makes it full-screen,
      | removing the taskbar, dock, etc.
      | 
      | The component must already be on the
      | desktop for this method to work. It will
      | be resized to completely fill the screen
      | and any extraneous taskbars, menu bars,
      | etc will be hidden.
      | 
      | To exit kiosk mode, just call setKioskModeComponent
      | (nullptr). When this is called, the
      | component that's currently being used
      | will be resized back to the size and position
      | it was in before being put into this mode.
      | 
      | If allowMenusAndBars is true, things
      | like the menu and dock (on mac) are still
      | allowed to pop up when the mouse moves
      | onto them. If this is false, it'll try
      | to hide as much on-screen paraphernalia
      | as possible.
      |
      */
    pub fn set_kiosk_mode_component(
        &mut self, 
        component_to_use:     *mut Component,
        allow_menus_and_bars: Option<bool>

    ) {

        let allow_menus_and_bars: bool = allow_menus_and_bars.unwrap_or(true);
        
        todo!();
        /*
            if (kioskModeReentrant)
            return;

        const ScopedValueSetter<bool> setter (kioskModeReentrant, true, false);

        if (kioskModeComponent != componentToUse)
        {
            // agh! Don't delete or remove a component from the desktop while it's still the kiosk component!
            jassert (kioskModeComponent == nullptr || ComponentPeer::getPeerFor (kioskModeComponent) != nullptr);

            if (auto* oldKioskComp = kioskModeComponent)
            {
                kioskModeComponent = nullptr; // (to make sure that isKioskMode() returns false when resizing the old one)
                setKioskComponent (oldKioskComp, false, allowMenusAndBars);
                oldKioskComp->setBounds (kioskComponentOriginalBounds);
            }

            kioskModeComponent = componentToUse;

            if (kioskModeComponent != nullptr)
            {
                // Only components that are already on the desktop can be put into kiosk mode!
                jassert (ComponentPeer::getPeerFor (kioskModeComponent) != nullptr);

                kioskComponentOriginalBounds = kioskModeComponent->getBounds();
                setKioskComponent (kioskModeComponent, true, allowMenusAndBars);
            }
        }
        */
    }
    
    /**
      | Sets which orientations the display
      | is allowed to auto-rotate to.
      | 
      | For devices that support rotating desktops,
      | this lets you specify which of the orientations
      | your app can use.
      | 
      | The parameter is a bitwise or-ed combination
      | of the values in DesktopDisplayOrientation,
      | and must contain at least one set bit.
      |
      */
    pub fn set_orientations_enabled(&mut self, new_orientations: i32)  {
        
        todo!();
        /*
            if (allowedOrientations != newOrientations)
        {
            // Dodgy set of flags being passed here! Make sure you specify at least one permitted orientation.
            jassert (newOrientations != 0 && (newOrientations & ~allOrientations) == 0);

            allowedOrientations = newOrientations;
            allowedOrientationsChanged();
        }
        */
    }
    
    /**
      | Returns the set of orientations the
      | display is allowed to rotate to. @see
      | setOrientationsEnabled
      |
      */
    pub fn get_orientations_enabled(&self) -> i32 {
        
        todo!();
        /*
            return allowedOrientations;
        */
    }
    
    /**
      | Returns whether the display is allowed
      | to auto-rotate to the given orientation.
      | 
      | Each orientation can be enabled using
      | setOrientationEnabled(). By default,
      | all orientations are allowed.
      |
      */
    pub fn is_orientation_enabled(&self, orientation: DesktopDisplayOrientation) -> bool {
        
        todo!();
        /*
            // Make sure you only pass one valid flag in here...
        jassert (orientation == upright || orientation == upsideDown
                  || orientation == rotatedClockwise || orientation == rotatedAntiClockwise);

        return (allowedOrientations & orientation) != 0;
        */
    }
    
    /**
      | Sets a global scale factor to be used
      | for all desktop windows.
      | 
      | Setting this will also scale the monitor
      | sizes that are returned by getDisplays().
      |
      */
    pub fn set_global_scale_factor(&mut self, new_scale_factor: f32)  {
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (masterScaleFactor != newScaleFactor)
        {
            masterScaleFactor = newScaleFactor;
            displays->refresh();
        }
        */
    }
    
    /**
      | Returns true on a headless system where
      | there are no connected displays.
      |
      */
    pub fn is_headless(&self) -> bool {
        
        todo!();
        /*
            return displays->displays.isEmpty();
        */
    }
    
    /**
      | True if the OS supports semitransparent
      | windows
      |
      */
    #[cfg(target_os="android")]
    pub fn can_use_semi_transparent_windows(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_default_master_scale(&mut self) -> f64 {
        
        todo!();
        /*
            return 1.0;
        */
    }
    
    /**
      | In a tablet device which can be turned
      | around, this returns the current orientation.
      |
      */
    #[cfg(target_os="android")]
    pub fn get_current_orientation(&self) -> DesktopDisplayOrientation {
        
        todo!();
        /*
            enum
        {
            ROTATION_0   = 0,
            ROTATION_90  = 1,
            ROTATION_180 = 2,
            ROTATION_270 = 3
        };

        JNIEnv* env = getEnv();
        LocalRef<jstring> windowServiceString (javaString ("window"));

        LocalRef<jobject> windowManager = LocalRef<jobject> (env->CallObjectMethod (getAppContext().get(), AndroidContext.getSystemService, windowServiceString.get()));

        if (windowManager.get() != nullptr)
        {
            LocalRef<jobject> display = LocalRef<jobject> (env->CallObjectMethod (windowManager, AndroidWindowManager.getDefaultDisplay));

            if (display.get() != nullptr)
            {
                int rotation = env->CallIntMethod (display, AndroidDisplay.getRotation);

                switch (rotation)
                {
                    case ROTATION_0:   return upright;
                    case ROTATION_90:  return rotatedAntiClockwise;
                    case ROTATION_180: return upsideDown;
                    case ROTATION_270: return rotatedClockwise;
                }
            }
        }

        jassertfalse;
        return upright;
        */
    }
}
