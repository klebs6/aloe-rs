crate::ix!();

lazy_static!{
    /*
    static uint32 lastUniquePeerID = 1;
    */
}

/**
  | A combination of these flags is passed
  | to the ComponentPeer constructor.
  |
  */
pub enum ComponentPeerStyleFlags
{
    /**
      | Indicates that the window should have
      | a corresponding entry on the taskbar
      | (ignored on MacOSX)
      |
      */
    windowAppearsOnTaskbar      = (1 << 0),    

    /**
      | Indicates that the window is a temporary
      | popup, like a menu, tooltip, etc.
      |
      */
    windowIsTemporary           = (1 << 1),    

    /**
      | Indicates that the window should let
      | mouse clicks pass through it (may not
      | be possible on some platforms).
      |
      */
    windowIgnoresMouseClicks    = (1 << 2),    

    /**
      | Indicates that the window should have
      | a normal OS-specific title bar and frame.
      | if not specified, the window will be
      | borderless.
      |
      */
    windowHasTitleBar           = (1 << 3),    

    /**
      | Indicates that the window should have
      | a resizable border.
      |
      */
    windowIsResizable           = (1 << 4),    

    /**
      | Indicates that if the window has a title
      | bar, it should have a minimise button
      | on it.
      |
      */
    windowHasMinimiseButton     = (1 << 5),    

    /**
      | Indicates that if the window has a title
      | bar, it should have a maximise button
      | on it.
      |
      */
    windowHasMaximiseButton     = (1 << 6),    

    /**
      | Indicates that if the window has a title
      | bar, it should have a close button on
      | it.
      |
      */
    windowHasCloseButton        = (1 << 7),    

    /**
      | Indicates that the window should have
      | a drop-shadow (this may not be possible
      | on all platforms).
      |
      */
    windowHasDropShadow         = (1 << 8),    

    /**
      | Not intended for public use - this tells
      | a window not to do its own repainting,
      | but only to repaint when the performAnyPendingRepaintsNow()
      | method is called.
      |
      */
    windowRepaintedExplictly    = (1 << 9),    

    /**
      | Tells the window not to catch any keypresses.
      | This can be used for things like plugin
      | windows, to stop them interfering with
      | the host's shortcut keys. This will
      | prevent the window from gaining keyboard
      | focus.
      |
      */
    windowIgnoresKeyPresses     = (1 << 10),   

    /**
      | Not intended for public use - makes a
      | window transparent.
      |
      */
    windowIsSemiTransparent     = (1 << 30),    
}

/**
  | Structure to describe drag and drop
  | information
  |
  */
#[derive(Default)]
pub struct ComponentPeerDragInfo {
    files:    Vec<String>,
    text:     String,
    position: Point<i32>,
}

impl ComponentPeerDragInfo {

    pub fn is_empty(&self) -> bool {

        todo!();
        /*
           return files.size() == 0 && text.isEmpty();
           */
    }

    pub fn clear(&mut self)  {

        todo!();
        /*
           files.clear(); text.clear();
           */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_ComponentPeer.h]

/**
  | The Component class uses a ComponentPeer
  | internally to create and manage a real
  | operating-system window.
  | 
  | This is an abstract base class - the platform
  | specific code contains implementations
  | of it for the various platforms.
  | 
  | User-code should very rarely need to
  | have any involvement with this class.
  | 
  | @see Component::createNewPeer
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ComponentPeer<'a> {
    component:                           &'a mut Component<'a>,
    style_flags:                         i32,
    last_non_fullscreen_bounds:          Rectangle<i32>,
    constrainer:                         *mut ComponentBoundsConstrainer, // default = nullptr
    scale_factor_listeners:              ListenerList<Box<dyn ComponentPeerScaleFactorListener>>,
    last_focused_component:              WeakReference<Component<'a>>,
    drag_and_drop_target_component:      WeakReference<Component<'a>>,
    last_drag_and_drop_comp_under_mouse: *mut Component<'a>, // default = nullptr
    uniqueid:                            u32,
    is_window_minimised:                 bool, // default = false
}

pub trait ComponentPeerInterface:
GetWindowNativeHandle 
+ SetVisible
+ SetTitle
+ SetDocumentEditedStatus
+ SetRepresentedFile
+ SetBounds
+ GetBounds
+ LocalToGlobalPoint
+ GlobalToLocalPoint
+ LocalToGlobalRectangle
+ GlobalToLocalRectangle
+ SetMinimised
+ IsMinimised
+ SetFullScreen
+ IsFullScreen
+ IsKioskMode
+ SetIcon
+ ContainsPoint
+ GetFrameSize
+ HandleScreenSizeChange
+ SetAlwaysOnTop
+ ToFront
+ ToBehind
+ IsFocused
+ GrabFocus
+ TextInputRequired
+ DismissPendingTextInput
+ Repaint
+ PerformAnyPendingRepaintsNow
+ SetAlpha
+ GetAvailableRenderingEngines
+ GetCurrentRenderingEngine
+ SetCurrentRenderingEngine
+ GetPlatformScaleFactor
{}

pub mod component_peer {

    use super::*;

    lazy_static!{
        /*
        static std::function<ModifierKeys()> getNativeRealtimeModifiers;
        std::function<ModifierKeys()> ComponentPeer::getNativeRealtimeModifiers = nullptr;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_ComponentPeer.cpp]
impl<'a> Drop for ComponentPeer<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            auto& desktop = Desktop::getInstance();
        desktop.peers.removeFirstMatchingValue (this);
        desktop.triggerFocusCallback();
        */
    }
}

#[cfg(target_os="linux")]
impl<'a> CreateNewPeer for Component<'a> {
    
    fn create_new_peer(
        &mut self, 
        style_flags:                i32,
        native_window_to_attach_to: *mut c_void

    ) -> *mut ComponentPeer {
        
        todo!();
        /*
            return new LinuxComponentPeer (*this, styleFlags, (::Window) nativeWindowToAttachTo);
        */
    }
}

impl<'a> ComponentPeer<'a> {

    /**
      | Returns the component being represented
      | by this peer.
      |
      */
    pub fn get_component(&mut self) -> &mut Component<'a> {

        todo!();
        /*
           return component;
           */
    }

    /**
      | Returns the set of style flags that were
      | set when the window was created. @see
      | Component::addToDesktop
      |
      */
    pub fn get_style_flags(&self) -> i32 {

        todo!();
        /*
           return styleFlags;
           */
    }

    /**
      | Returns a unique ID for this peer.
      | 
      | Each peer that is created is given a different
      | ID.
      |
      */
    pub fn get_uniqueid(&self) -> u32 {

        todo!();
        /*
           return uniqueID;
           */
    }

    /**
      | Returns the current constrainer, if
      | one has been set.
      |
      */
    pub fn get_constrainer(&self) -> *mut ComponentBoundsConstrainer {

        todo!();
        /*
           return constrainer;
           */
    }

    /**
      | Adds a scale factor listener.
      |
      */
    pub fn add_scale_factor_listener(&mut self, listener_to_add: *mut dyn ComponentPeerScaleFactorListener)  {
        
        todo!();
        /*
            scaleFactorListeners.add (listenerToAdd);
        */
    }

    /**
      | Removes a scale factor listener.
      |
      */
    pub fn remove_scale_factor_listener(&mut self, listener_to_remove: *mut dyn ComponentPeerScaleFactorListener)  {
        
        todo!();
        /*
            scaleFactorListeners.remove (listenerToRemove);
        */
    }
    
    /**
      | Creates a peer.
      | 
      | The component is the one that we intend
      | to represent, and the style flags are
      | a combination of the values in the ComponentPeerStyleFlags
      | enum
      |
      */
    pub fn new(
        comp:  &mut Component<'a>,
        flags: i32) -> Self {
    
        todo!();
        /*


            : component (comp),
          styleFlags (flags),
          uniqueID (lastUniquePeerID += 2) // increment by 2 so that this can never hit 0

        Desktop::getInstance().peers.add (this);
        */
    }
    
    /**
      | Returns the number of currently-active
      | peers. @see getPeer
      |
      */
    pub fn get_num_peers(&mut self) -> i32 {
        
        todo!();
        /*
            return Desktop::getInstance().peers.size();
        */
    }
    
    /**
      | Returns one of the currently-active
      | peers. @see getNumPeers
      |
      */
    pub fn get_peer(&mut self, index: i32) -> *mut ComponentPeer<'a> {
        
        todo!();
        /*
            return Desktop::getInstance().peers [index];
        */
    }
    
    /**
      | Returns the peer that's attached to
      | the given component, or nullptr if there
      | isn't one.
      |
      */
    pub fn get_peer_for(&mut self, component: *const Component<'a>) -> *mut ComponentPeer<'a> {
        
        todo!();
        /*
            for (auto* peer : Desktop::getInstance().peers)
            if (&(peer->getComponent()) == component)
                return peer;

        return nullptr;
        */
    }
    
    /**
      | Checks if this peer object is valid.
      | @see getNumPeers
      |
      */
    pub fn is_valid_peer(&mut self, peer: *const ComponentPeer<'a>) -> bool {
        
        todo!();
        /*
            return Desktop::getInstance().peers.contains (const_cast<ComponentPeer*> (peer));
        */
    }
    
    /**
      | Updates the peer's bounds to match its
      | component.
      |
      */
    pub fn update_bounds(&mut self)  {
        
        todo!();
        /*
            setBounds (ScalingHelpers::scaledScreenPosToUnscaled (component, component.getBoundsInParent()), false);
        */
    }
    
    pub fn is_kiosk_mode(&self) -> bool {
        
        todo!();
        /*
            return Desktop::getInstance().getKioskModeComponent() == &component;
        */
    }
    
    pub fn handle_mouse_event(
        &mut self, 
        ty:              MouseInputSourceType,
        pos:             Point<f32>,
        new_mods:        ModifierKeys,
        new_pressure:    f32,
        new_orientation: f32,
        time:            i64,
        pen:             PenDetails,
        touch_index:     Option<i32>
    )  {

        let touch_index: i32 = touch_index.unwrap_or(0);
        
        todo!();
        /*
            if (auto* mouse = Desktop::getInstance().mouseSources->getOrCreateMouseInputSource (type, touchIndex))
            MouseInputSource (*mouse).handleEvent (*this, pos, time, newMods, newPressure, newOrientation, pen);
        */
    }
    
    pub fn handle_mouse_wheel(
        &mut self, 
        ty:          MouseInputSourceType,
        pos:         Point<f32>,
        time:        i64,
        wheel:       &MouseWheelDetails,
        touch_index: Option<i32>

    ) {

        let touch_index: i32 = touch_index.unwrap_or(0);
        
        todo!();
        /*
            if (auto* mouse = Desktop::getInstance().mouseSources->getOrCreateMouseInputSource (type, touchIndex))
            MouseInputSource (*mouse).handleWheel (*this, pos, time, wheel);
        */
    }
    
    pub fn handle_magnify_gesture(
        &mut self, 
        ty:           MouseInputSourceType,
        pos:          Point<f32>,
        time:         i64,
        scale_factor: f32,
        touch_index:  Option<i32>

    ) {

        let touch_index: i32 = touch_index.unwrap_or(0);
        
        todo!();
        /*
            if (auto* mouse = Desktop::getInstance().mouseSources->getOrCreateMouseInputSource (type, touchIndex))
            MouseInputSource (*mouse).handleMagnifyGesture (*this, pos, time, scaleFactor);
        */
    }
    
    /**
      | This is called to repaint the component
      | into the given context.
      |
      */
    pub fn handle_paint(&mut self, context_to_paint_to: &mut dyn LowLevelGraphicsContext)  {
        
        todo!();
        /*
            Graphics g (contextToPaintTo);

        if (component.isTransformed())
            g.addTransform (component.getTransform());

        auto peerBounds = getBounds();
        auto componentBounds = component.getLocalBounds();

        if (component.isTransformed())
            componentBounds = componentBounds.transformedBy (component.getTransform());

        if (peerBounds.getWidth() != componentBounds.getWidth() || peerBounds.getHeight() != componentBounds.getHeight())
            // Tweak the scaling so that the component's integer size exactly aligns with the peer's scaled size
            g.addTransform (AffineTransform::scale ((float) peerBounds.getWidth()  / (float) componentBounds.getWidth(),
                                                    (float) peerBounds.getHeight() / (float) componentBounds.getHeight()));

      #if ALOE_ENABLE_REPAINT_DEBUGGING
       #ifdef ALOE_IS_REPAINT_DEBUGGING_ACTIVE
        if (ALOE_IS_REPAINT_DEBUGGING_ACTIVE)
       #endif
        {
            g.saveState();
        }
      #endif

        ALOE_TRY
        {
            component.paintEntireComponent (g, true);
        }
        ALOE_CATCH_EXCEPTION

      #if ALOE_ENABLE_REPAINT_DEBUGGING
       #ifdef ALOE_IS_REPAINT_DEBUGGING_ACTIVE
        if (ALOE_IS_REPAINT_DEBUGGING_ACTIVE)
       #endif
        {
            // enabling this code will fill all areas that get repainted with a colour overlay, to show
            // clearly when things are being repainted.
            g.restoreState();

            static Random rng;

            g.fillAll (Colour ((uint8) rng.nextInt (255),
                               (uint8) rng.nextInt (255),
                               (uint8) rng.nextInt (255),
                               (uint8) 0x50));
        }
      #endif

        /** If this fails, it's probably be because your CPU floating-point precision mode has
            been set to low.. This setting is sometimes changed by things like Direct3D, and can
            mess up a lot of the calculations that the library needs to do.
        */
        jassert (roundToInt (10.1f) == 10);
        */
    }
    
    pub fn get_target_for_key_press(&mut self) -> *mut Component<'a> {
        
        todo!();
        /*
            auto* c = Component::getCurrentlyFocusedComponent();

        if (c == nullptr)
            c = &component;

        if (c->isCurrentlyBlockedByAnotherModalComponent())
            if (auto* currentModalComp = Component::getCurrentlyModalComponent())
                c = currentModalComp;

        return c;
        */
    }
    
    /**
      | Called when a key is pressed.
      | 
      | For keycode info, see the KeyPress class.
      | 
      | Returns true if the keystroke was used.
      |
      */
    pub fn handle_key_press_with_key_code(
        &mut self, 
        key_code:       i32,
        text_character: wchar_t

    ) -> bool {
        
        todo!();
        /*
            return handleKeyPress (KeyPress (keyCode,
                                         ModifierKeys::currentModifiers.withoutMouseButtons(),
                                         textCharacter));
        */
    }
    
    /**
      | Called when a key is pressed.
      | 
      | Returns true if the keystroke was used.
      |
      */
    pub fn handle_key_press(&mut self, key_info: &KeyPress) -> bool {
        
        todo!();
        /*
            bool keyWasUsed = false;

        for (auto* target = getTargetForKeyPress(); target != nullptr; target = target->getParentComponent())
        {
            const WeakReference<Component> deletionChecker (target);

            if (auto* keyListeners = target->keyListeners.get())
            {
                for (int i = keyListeners->size(); --i >= 0;)
                {
                    keyWasUsed = keyListeners->getUnchecked (i)->keyPressed (keyInfo, target);

                    if (keyWasUsed || deletionChecker == nullptr)
                        return keyWasUsed;

                    i = jmin (i, keyListeners->size());
                }
            }

            keyWasUsed = target->keyPressed (keyInfo);

            if (keyWasUsed || deletionChecker == nullptr)
                break;
        }

        if (! keyWasUsed && keyInfo.isKeyCode (KeyPress::tabKey))
        {
            if (auto* currentlyFocused = Component::getCurrentlyFocusedComponent())
            {
                currentlyFocused->moveKeyboardFocusToSibling (! keyInfo.getModifiers().isShiftDown());
                return true;
            }
        }

        return keyWasUsed;
        */
    }
    
    /**
      | Called whenever a key is pressed or released.
      | 
      | Returns true if the keystroke was used.
      |
      */
    pub fn handle_key_up_or_down(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            bool keyWasUsed = false;

        for (auto* target = getTargetForKeyPress(); target != nullptr; target = target->getParentComponent())
        {
            const WeakReference<Component> deletionChecker (target);

            keyWasUsed = target->keyStateChanged (isKeyDown);

            if (keyWasUsed || deletionChecker == nullptr)
                break;

            if (auto* keyListeners = target->keyListeners.get())
            {
                for (int i = keyListeners->size(); --i >= 0;)
                {
                    keyWasUsed = keyListeners->getUnchecked (i)->keyStateChanged (isKeyDown, target);

                    if (keyWasUsed || deletionChecker == nullptr)
                        return keyWasUsed;

                    i = jmin (i, keyListeners->size());
                }
            }
        }

        return keyWasUsed;
        */
    }
    
    /**
      | Called whenever a modifier key is pressed
      | or released.
      |
      */
    pub fn handle_modifier_keys_change(&mut self)  {
        
        todo!();
        /*
            auto* target = Desktop::getInstance().getMainMouseSource().getComponentUnderMouse();

        if (target == nullptr)
            target = Component::getCurrentlyFocusedComponent();

        if (target == nullptr)
            target = &component;

        target->internalModifierKeysChanged();
        */
    }
    
    /**
      | Returns the currently focused TextInputTarget,
      | or null if none is found.
      |
      */
    pub fn find_current_text_input_target(&mut self) -> *mut dyn TextInputTarget {
        
        todo!();
        /*
            auto* c = Component::getCurrentlyFocusedComponent();

        if (c == &component || component.isParentOf (c))
            if (auto* ti = dynamic_cast<TextInputTarget*> (c))
                if (ti->isTextInputActive())
                    return ti;

        return nullptr;
        */
    }
    
    pub fn dismiss_pending_text_input(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Called when the window is brought to
      | the front, either by the OS or by a call
      | to toFront().
      |
      */
    pub fn handle_brought_to_front(&mut self)  {
        
        todo!();
        /*
            component.internalBroughtToFront();
        */
    }
    
    /**
      | Sets a constrainer to use if the peer
      | can resize itself.
      | 
      | The constrainer won't be deleted by
      | this object, so the caller must manage
      | its lifetime.
      |
      */
    pub fn set_constrainer(&mut self, new_constrainer: *mut ComponentBoundsConstrainer)  {
        
        todo!();
        /*
            constrainer = newConstrainer;
        */
    }
    
    /**
      | This is called when the window's bounds
      | change.
      | 
      | A peer implementation must call this
      | when the window is moved and resized,
      | so that this method can pass the message
      | on to the component.
      |
      */
    pub fn handle_moved_or_resized(&mut self)  {
        
        todo!();
        /*
            const bool nowMinimised = isMinimised();

        if (component.flags.hasHeavyweightPeerFlag && ! nowMinimised)
        {
            const WeakReference<Component> deletionChecker (&component);

            auto newBounds = Component::ComponentHelpers::rawPeerPositionToLocal (component, getBounds());
            auto oldBounds = component.getBounds();

            const bool wasMoved   = (oldBounds.getPosition() != newBounds.getPosition());
            const bool wasResized = (oldBounds.getWidth() != newBounds.getWidth() || oldBounds.getHeight() != newBounds.getHeight());

            if (wasMoved || wasResized)
            {
                component.boundsRelativeToParent = newBounds;

                if (wasResized)
                    component.repaint();

                component.sendMovedResizedMessages (wasMoved, wasResized);

                if (deletionChecker == nullptr)
                    return;
            }
        }

        if (isWindowMinimised != nowMinimised)
        {
            isWindowMinimised = nowMinimised;
            component.minimisationStateChanged (nowMinimised);
            component.sendVisibilityChangeMessage();
        }

        if (! isFullScreen())
            lastNonFullscreenBounds = component.getBounds();
        */
    }
    
    /**
      | Called when the window gains keyboard
      | focus.
      |
      */
    pub fn handle_focus_gain(&mut self)  {
        
        todo!();
        /*
            if (component.isParentOf (lastFocusedComponent)
              && lastFocusedComponent->isShowing()
              && lastFocusedComponent->getWantsKeyboardFocus())
        {
            Component::currentlyFocusedComponent = lastFocusedComponent;
            Desktop::getInstance().triggerFocusCallback();
            lastFocusedComponent->internalKeyboardFocusGain (Component::focusChangedDirectly);
        }
        else
        {
            if (! component.isCurrentlyBlockedByAnotherModalComponent())
                component.grabKeyboardFocus();
            else
                typename ModalComponentManager::getInstance()->bringModalComponentsToFront();
        }
        */
    }
    
    /**
      | Called when the window loses keyboard
      | focus.
      |
      */
    pub fn handle_focus_loss(&mut self)  {
        
        todo!();
        /*
            if (component.hasKeyboardFocus (true))
        {
            lastFocusedComponent = Component::currentlyFocusedComponent;

            if (lastFocusedComponent != nullptr)
            {
                Component::currentlyFocusedComponent = nullptr;
                Desktop::getInstance().triggerFocusCallback();
                lastFocusedComponent->internalKeyboardFocusLoss (Component::focusChangedByMouseClick);
            }
        }
        */
    }
    
    pub fn get_last_focused_subcomponent(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return (component.isParentOf (lastFocusedComponent) && lastFocusedComponent->isShowing())
                    ? static_cast<Component*> (lastFocusedComponent)
                    : &component;
        */
    }
    
    pub fn handle_screen_size_change(&mut self)  {
        
        todo!();
        /*
            component.parentSizeChanged();
        handleMovedOrResized();
        */
    }
    
    /**
      | Sets the size to restore to if fullscreen
      | mode is turned off.
      |
      */
    pub fn set_non_full_screen_bounds(&mut self, new_bounds: &Rectangle<i32>)  {
        
        todo!();
        /*
            lastNonFullscreenBounds = newBounds;
        */
    }
    
    /**
      | Returns the size to restore to if fullscreen
      | mode is turned off.
      |
      */
    pub fn get_non_full_screen_bounds(&self) -> &Rectangle<i32> {
        
        todo!();
        /*
            return lastNonFullscreenBounds;
        */
    }
    
    /**
      | Converts a position relative to the
      | top-left of this component to screen
      | coordinates.
      |
      */
    pub fn local_to_global_point_i32(&mut self, p: Point<i32>) -> Point<i32> {
        
        todo!();
        /*
            return localToGlobal (p.toFloat()).roundToInt();
        */
    }
    
    /**
      | Converts a screen coordinate to a position
      | relative to the top-left of this component.
      |
      */
    pub fn global_to_local_point_i32(&mut self, p: Point<i32>) -> Point<i32> {
        
        todo!();
        /*
            return globalToLocal (p.toFloat()).roundToInt();
        */
    }
    
    pub fn local_to_global_rect_i32(&mut self, relative_position: &Rectangle<i32>) -> Rectangle<i32> {
        
        todo!();
        /*
            return relativePosition.withPosition (localToGlobal (relativePosition.getPosition()));
        */
    }
    
    /**
      | Converts a screen area to a position
      | relative to the top-left of this component.
      |
      */
    pub fn global_to_local_rect_i32(&mut self, screen_position: &Rectangle<i32>) -> Rectangle<i32> {
        
        todo!();
        /*
            return screenPosition.withPosition (globalToLocal (screenPosition.getPosition()));
        */
    }
    
    /**
      | Converts a rectangle relative to the
      | top-left of this component to screen
      | coordinates.
      |
      */
    pub fn local_to_global_rect_f32(&mut self, relative_position: &Rectangle<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            return relativePosition.withPosition (localToGlobal (relativePosition.getPosition()));
        */
    }
    
    pub fn global_to_local_rect_f32(&mut self, screen_position: &Rectangle<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            return screenPosition.withPosition (globalToLocal (screenPosition.getPosition()));
        */
    }
    
    /**
      | Returns the area in peer coordinates
      | that is covered by the given sub-comp
      | (which may be at any depth)
      |
      */
    pub fn get_area_covered_by(&self, sub_component: &Component<'a>) -> Rectangle<i32> {
        
        todo!();
        /*
            return ScalingHelpers::scaledScreenPosToUnscaled
                (component, component.getLocalArea (&subComponent, subComponent.getLocalBounds()));
        */
    }

    pub fn handle_drag_move(&mut self, info: &ComponentPeerDragInfo) -> bool {
        
        todo!();
        /*
            auto* compUnderMouse = component.getComponentAt (info.position);
        auto* lastTarget = dragAndDropTargetComponent.get();
        Component* newTarget = nullptr;

        if (compUnderMouse != lastDragAndDropCompUnderMouse)
        {
            lastDragAndDropCompUnderMouse = compUnderMouse;
            newTarget = DragHelpers::findDragAndDropTarget (compUnderMouse, info, lastTarget);

            if (newTarget != lastTarget)
            {
                if (lastTarget != nullptr)
                {
                    if (DragHelpers::isFileDrag (info))
                        dynamic_cast<FileDragAndDropTarget*> (lastTarget)->fileDragExit (info.files);
                    else
                        dynamic_cast<TextDragAndDropTarget*> (lastTarget)->textDragExit (info.text);
                }

                dragAndDropTargetComponent = nullptr;

                if (DragHelpers::isSuitableTarget (info, newTarget))
                {
                    dragAndDropTargetComponent = newTarget;
                    auto pos = newTarget->getLocalPoint (&component, info.position);

                    if (DragHelpers::isFileDrag (info))
                        dynamic_cast<FileDragAndDropTarget*> (newTarget)->fileDragEnter (info.files, pos.x, pos.y);
                    else
                        dynamic_cast<TextDragAndDropTarget*> (newTarget)->textDragEnter (info.text, pos.x, pos.y);
                }
            }
        }
        else
        {
            newTarget = lastTarget;
        }

        if (! DragHelpers::isSuitableTarget (info, newTarget))
            return false;

        auto pos = newTarget->getLocalPoint (&component, info.position);

        if (DragHelpers::isFileDrag (info))
            dynamic_cast<FileDragAndDropTarget*> (newTarget)->fileDragMove (info.files, pos.x, pos.y);
        else
            dynamic_cast<TextDragAndDropTarget*> (newTarget)->textDragMove (info.text, pos.x, pos.y);

        return true;
        */
    }
    
    pub fn handle_drag_exit(&mut self, info: &ComponentPeerDragInfo) -> bool {
        
        todo!();
        /*
            ComponentPeerDragInfo info2 (info);
        info2.position.setXY (-1, -1);
        const bool used = handleDragMove (info2);

        jassert (dragAndDropTargetComponent == nullptr);
        lastDragAndDropCompUnderMouse = nullptr;
        return used;
        */
    }
    
    pub fn handle_drag_drop(&mut self, info: &ComponentPeerDragInfo) -> bool {
        
        todo!();
        /*
            handleDragMove (info);

        if (WeakReference<Component> targetComp = dragAndDropTargetComponent)
        {
            dragAndDropTargetComponent = nullptr;
            lastDragAndDropCompUnderMouse = nullptr;

            if (DragHelpers::isSuitableTarget (info, targetComp))
            {
                if (targetComp->isCurrentlyBlockedByAnotherModalComponent())
                {
                    targetComp->internalModalInputAttempt();

                    if (targetComp->isCurrentlyBlockedByAnotherModalComponent())
                        return true;
                }

                ComponentPeer::ComponentPeerDragInfo infoCopy (info);
                infoCopy.position = targetComp->getLocalPoint (&component, info.position);

                // We'll use an async message to deliver the drop, because if the target decides
                // to run a modal loop, it can gum-up the operating system..
                MessageManager::callAsync ([=]
                {
                    if (auto* c = targetComp.get())
                    {
                        if (DragHelpers::isFileDrag (info))
                            dynamic_cast<FileDragAndDropTarget*> (c)->filesDropped (infoCopy.files, infoCopy.position.x, infoCopy.position.y);
                        else
                            dynamic_cast<TextDragAndDropTarget*> (c)->textDropped (infoCopy.text, infoCopy.position.x, infoCopy.position.y);
                    }
                });

                return true;
            }
        }

        return false;
        */
    }
    
    pub fn handle_user_closing_window(&mut self)  {
        
        todo!();
        /*
            component.userTriedToCloseWindow();
        */
    }
    
    pub fn set_document_edited_status(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn set_represented_file(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_current_rendering_engine(&self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn set_current_rendering_engine(&mut self, index: i32)  {
        
        todo!();
        /*
            jassert (index == 0); ignoreUnused (index);
        */
    }
    
    /**
      | On desktop platforms this method will
      | check all the mouse and key states and
      | return a ModifierKeys object representing
      | them.
      | 
      | This isn't recommended and is only needed
      | in special circumstances for up-to-date
      | modifier information at times when
      | the app's event loop isn't running normally.
      | 
      | Another reason to avoid this method
      | is that it's not stateless and calling
      | it may update the ModifierKeys::currentModifiers
      | object, which could cause subtle changes
      | in the behaviour of some components.
      |
      */
    pub fn get_current_modifiers_realtime(&mut self) -> ModifierKeys {
        
        todo!();
        /*
            if (getNativeRealtimeModifiers != nullptr)
            return getNativeRealtimeModifiers();

        return ModifierKeys::currentModifiers;
        */
    }
    
    pub fn force_display_update(&mut self)  {
        
        todo!();
        /*
            Desktop::getInstance().displays->refresh();
        */
    }
}
