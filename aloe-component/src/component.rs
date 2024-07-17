crate::ix!();

pub fn aloe_explicit_focus_order_id() -> Identifier {
    Identifier::new("_jexfo")
}

///-------------------------

pub const colour_property_prefix: &'static str = "jcclr_";

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_Component.h]

pub trait FindColour {

    fn find_colour(&self, colourid: i32) -> Colour;
}

pub trait IsColourSpecified {

    fn is_colour_specified(&self, colourid: i32) -> bool;
}

pub trait SetColour {

    fn set_colour(
        &mut self, 
        colourid:   i32,
        new_colour: Colour);
}

/// This encapsulates what Component needs to know
/// how to call from our LookAndFeel structure --
///
/// we introduced this trait to break the cyclic crate
/// dependency between aloe-component,
/// aloe-lookandfeel, and the
/// aloe-{button,slider,textbox,etc} component
/// crates
///
pub trait LookAndFeelComponentInterface: FindColour + IsColourSpecified {}

/**
  | The base class for all Aloe user-interface
  | objects.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Component<'a> {
    component_name:            String,
    componentid:               String,
    component_title:           String,
    component_description:     String,
    component_help_text:       String,
    parent_component:          *mut Component<'a>, // default = nullptr
    bounds_relative_to_parent: Rectangle<i32>,
    positioner:                Box<ComponentPositioner<'a>>,
    affine_transform:          Box<AffineTransform>,
    child_component_list:      Vec<*mut Component<'a>>,
    look_and_feel:             WeakReference<Box<dyn LookAndFeelComponentInterface>>,
    cursor:                    MouseCursor,
    effect:                    *mut dyn ImageEffectFilter, // default = nullptr
    cached_image:              Box<dyn CachedComponentImage>,
    mouse_listeners:           Box<MouseListenerList>,
    key_listeners:             Box<Vec<*mut dyn KeyListener>>,
    component_listeners:       ListenerList<Box<dyn ComponentListener>>,
    properties:                NamedValueSet,
    master_reference:          WeakReferenceMaster<Component<'a>>,
    accessibility_handler:     Box<AccessibilityHandler<'a>>,
    component_transparency:    u8, // default = 0

    /*
      | Components aren't allowed to have copy
      | constructors, as this would mess up
      | parent hierarchies.
      | 
      | You might need to give your subclasses
      | a private dummy constructor to avoid
      | compiler warnings.
      |
      */
}

impl<'a> MouseListener for Component<'a> {

}

impl<'a> MouseMagnify for Component<'a> {

    /**
      | Called when a pinch-to-zoom mouse-gesture
      | is used.
      | 
      | If not overridden, a component will
      | forward this message to its parent,
      | so that parent components can collect
      | gesture messages that are unused by
      | child components.
      | 
      | -----------
      | @param event
      | 
      | details about the mouse event
      | ----------
      | @param scaleFactor
      | 
      | a multiplier to indicate by how much
      | the size of the target should be changed.
      | A value of 1.0 would indicate no change,
      | values greater than 1.0 mean it should
      | be enlarged.
      |
      */
    fn mouse_magnify(&mut self, 
        e:              &MouseEvent,
        magnify_amount: f32)  {
        
        todo!();
        /*
            // the base class just passes this event up to its parent..
        if (parentComponent != nullptr)
            parentComponent->mouseMagnify (e.getEventRelativeTo (parentComponent), magnifyAmount);
        */
    }
}

impl<'a> MouseWheelMove for Component<'a> {

    /**
      | Called when the mouse-wheel is moved.
      | 
      | This callback is sent to the component
      | that the mouse is over when the wheel
      | is moved.
      | 
      | If not overridden, a component will
      | forward this message to its parent,
      | so that parent components can collect
      | mouse-wheel messages that happen to
      | child components which aren't interested
      | in them. (Bear in mind that if you attach
      | a component as a mouse-listener to other
      | components, then those wheel moves
      | will also end up calling this method
      | and being passed up to the parents, which
      | may not be what you intended to happen).
      | 
      | -----------
      | @param event
      | 
      | details about the mouse event
      | ----------
      | @param wheel
      | 
      | details about the mouse wheel movement
      |
      */
    fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            // the base class just passes this event up to its parent..
        if (parentComponent != nullptr)
            parentComponent->mouseWheelMove (e.getEventRelativeTo (parentComponent), wheel);
        */
    }
}

impl<'a> MouseDoubleClick for Component<'a> {

    /**
      | Called when a mouse button has been double-clicked
      | on a component.
      | 
      | The MouseEvent object passed in contains
      | lots of methods for finding out which
      | button was pressed, as well as which
      | modifier keys (e.g. shift, ctrl) were
      | held down at the time.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseDown, mouseUp
      |
      */
    fn mouse_double_click(&mut self, event: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> MouseUp for Component<'a> {

    /**
      | Called when a mouse button is released.
      | 
      | A mouseUp callback is sent to the component
      | in which a button was pressed even if
      | the mouse is actually over a different
      | component when the button is released.
      | 
      | The MouseEvent object passed in contains
      | lots of methods for finding out which
      | buttons were down just before they were
      | released.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseDown, mouseDrag, mouseDoubleClick,
      | contains
      |
      */
    fn mouse_up(&mut self, event: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> MouseDown for Component<'a> {

    /**
      | Called when a mouse button is pressed.
      | 
      | The MouseEvent object passed in contains
      | lots of methods for finding out which
      | button was pressed, as well as which
      | modifier keys (e.g. shift, ctrl) were
      | held down at the time.
      | 
      | Once a button is held down, the mouseDrag
      | method will be called when the mouse
      | moves, until the button is released.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseUp, mouseDrag, mouseDoubleClick,
      | contains
      |
      */
    fn mouse_down(&mut self, event: &MouseEvent)  { }
}

impl<'a> MouseDrag for Component<'a> {

    /**
      | Called when the mouse is moved while
      | a button is held down.
      | 
      | When a mouse button is pressed inside
      | a component, that component receives
      | mouseDrag callbacks each time the mouse
      | moves, even if the mouse strays outside
      | the component's bounds.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseDown, mouseUp, mouseMove, contains,
      | setDragRepeatInterval
      |
      */
    fn mouse_drag(&mut self, event: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> MouseEnter for Component<'a> {

    /**
      | Called when the mouse first enters a
      | component.
      | 
      | If the mouse button isn't pressed and
      | the mouse moves into a component, this
      | will be called to let the component react
      | to this.
      | 
      | When the mouse button is pressed and
      | held down while being moved in or out
      | of a component, no mouseEnter or mouseExit
      | callbacks are made - only mouseDrag
      | messages are sent to the component that
      | the mouse was originally clicked on,
      | until the button is released.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseExit, mouseDrag, mouseMove,
      | contains
      |
      */
    fn mouse_enter(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl<'a> MouseExit for Component<'a> {

    /**
      | Called when the mouse moves out of a component.
      | 
      | This will be called when the mouse moves
      | off the edge of this component.
      | 
      | If the mouse button was pressed, and
      | it was then dragged off the edge of the
      | component and released, then this callback
      | will happen when the button is released,
      | after the mouseUp callback.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseEnter, mouseDrag, mouseMove,
      | contains
      |
      */
    fn mouse_exit(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl<'a> MouseMove for Component<'a> {

    /**
      | Called when the mouse moves inside a
      | component.
      | 
      | If the mouse button isn't pressed and
      | the mouse moves over a component, this
      | will be called to let the component react
      | to this.
      | 
      | A component will always get a mouseEnter
      | callback before a mouseMove.
      | 
      | -----------
      | @param event
      | 
      | details about the position and status
      | of the mouse event, including the source
      | component in which it occurred @see
      | mouseEnter, mouseExit, mouseDrag,
      | contains
      |
      */
    fn mouse_move(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
}

lazy_static!{
    /*
    static Component* currentlyFocusedComponent = nullptr;
    */
}

impl<'a> Drop for Component<'a> {

    /**
      | Destructor.
      | 
      | -----------
      | @note
      | 
      | when a component is deleted, any child
      | components it contains are NOT automatically
      | deleted. It's your responsibility
      | to manage their lifespan - you may want
      | to use helper methods like deleteAllChildren(),
      | or less haphazard approaches like using
      | std::unique_ptrs or normal object
      | aggregation to manage them.
      | 
      | If the component being deleted is currently
      | the child of another one, then during
      | deletion, it will be removed from its
      | parent, and the parent will receive
      | a childrenChanged() callback. Any
      | ComponentListener objects that have
      | registered with it will also have their
      | ComponentListener::componentBeingDeleted()
      | methods called.
      |
      */

    fn drop(&mut self) {
        todo!();
        /* 
        static_assert (sizeof (flags) <= sizeof (componentFlags), "componentFlags has too many bits!");

        componentListeners.call ([this] (ComponentListener& l) { l.componentBeingDeleted (*this); });

        while (childComponentList.size() > 0)
            removeChildComponent (childComponentList.size() - 1, false, true);

        masterReference.clear();

        if (parentComponent != nullptr)
            parentComponent->removeChildComponent (parentComponent->childComponentList.indexOf (this), true, false);
        else
            giveAwayKeyboardFocusInternal (isParentOf (currentlyFocusedComponent));

        if (flags.hasHeavyweightPeerFlag)
            removeFromDesktop();

        // Something has added some children to this component during its destructor! Not a smart idea!
        jassert (childComponentList.size() == 0);
 */
    }
}
   
impl<'a> Default for Component<'a> {
    
    /**
      | Creates a component.
      | 
      | To get it to actually appear, you'll
      | also need to:
      | 
      | - Either add it to a parent component
      | or use the addToDesktop() method to
      | make it a desktop window
      | 
      | - Set its size and position to something
      | sensible
      | 
      | - Use setVisible() to make it visible
      | 
      | And for it to serve any useful purpose,
      | you'll need to write a subclass of Component
      | or use one of the other types of component
      | from the library.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/components/aloe_Component.cpp]
impl<'a> Component<'a> {

    /**
      | Returns the name of this component.
      | @see setName
      |
      */
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return componentName;
        */
    }

    /**
      | Returns the ID string that was set by
      | setComponentID(). @see setComponentID,
      | findChildWithID
      |
      */
    pub fn get_componentid(&self) -> String {
        
        todo!();
        /*
            return componentID;
        */
    }

    /**
      | Tests whether the component is visible
      | or not.
      | 
      | this doesn't necessarily tell you whether
      | this comp is actually on the screen because
      | this depends on whether all the parent
      | components are also visible - use isShowing()
      | to find this out.
      | 
      | @see isShowing, setVisible
      |
      */
    pub fn is_visible(&self) -> bool {
        
        todo!();
        /*
            return flags.visibleFlag;
        */
    }

    /**
      | Returns the x coordinate of the component's
      | left edge. This is a distance in pixels
      | from the left edge of the component's
      | parent.
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to its bounding box.
      |
      */
    pub fn getx(&self) -> i32 {
        
        todo!();
        /*
            return boundsRelativeToParent.getX();
        */
    }

    /**
      | Returns the y coordinate of the top of
      | this component. This is a distance in
      | pixels from the top edge of the component's
      | parent.
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to its bounding box.
      |
      */
    pub fn gety(&self) -> i32 {
        
        todo!();
        /*
            return boundsRelativeToParent.getY();
        */
    }

    /**
      | Returns the component's width in pixels.
      |
      */
    pub fn get_width(&self) -> i32 {
        
        todo!();
        /*
            return boundsRelativeToParent.getWidth();
        */
    }

    /**
      | Returns the component's height in pixels.
      |
      */
    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            return boundsRelativeToParent.getHeight();
        */
    }

    /**
      | Returns the x coordinate of the component's
      | right-hand edge. This is a distance
      | in pixels from the left edge of the component's
      | parent.
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to its bounding box.
      |
      */
    pub fn get_right(&self) -> i32 {
        
        todo!();
        /*
            return boundsRelativeToParent.getRight();
        */
    }

    /**
      | Returns the component's top-left position
      | as a Point.
      |
      */
    pub fn get_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return boundsRelativeToParent.getPosition();
        */
    }

    /**
      | Returns the y coordinate of the bottom
      | edge of this component. This is a distance
      | in pixels from the top edge of the component's
      | parent.
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to its bounding box.
      |
      */
    pub fn get_bottom(&self) -> i32 {
        
        todo!();
        /*
            return boundsRelativeToParent.getBottom();
        */
    }

    /**
      | Returns this component's bounding
      | box. The rectangle returned is relative
      | to the top-left of the component's parent.
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to its bounding box.
      |
      */
    pub fn get_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return boundsRelativeToParent;
        */
    }

    /**
      | Provides access to the underlying array
      | of child components. The most likely
      | reason you may want to use this is for
      | iteration in a range-based for loop.
      |
      */
    pub fn get_children(&self) -> &[*mut Component] {
        
        todo!();
        /*
            return childComponentList;
        */
    }

    /**
      | Returns the component which this component
      | is inside.
      | 
      | If this is the highest-level component
      | or hasn't yet been added to a parent,
      | this will return null.
      |
      */
    pub fn get_parent_component(&self) -> *mut Component {
        
        todo!();
        /*
            return parentComponent;
        */
    }

    /**
      | Searches the parent components for
      | a component of a specified class.
      | 
      | For example findParentComponentOfClass
      | \<MyComp\>() would return the first
      | parent component that can be dynamically
      | cast to a MyComp, or will return nullptr
      | if none of the parents are suitable.
      |
      */
    pub fn find_parent_component_of_class<TargetClass>(&self) -> *mut TargetClass {
    
        todo!();
        /*
            for (auto* p = parentComponent; p != nullptr; p = p->parentComponent)
                if (auto* target = dynamic_cast<TargetClass*> (p))
                    return target;

            return nullptr;
        */
    }

    /**
      | Returns the current component effect.
      | @see setComponentEffect
      |
      */
    pub fn get_component_effect(&self) -> *mut dyn ImageEffectFilter {
        
        todo!();
        /*
            return effect;
        */
    }

    /* ----------------- Focus methods  ----------------- */

    /**
      | Returns the set of properties that belong
      | to this component.
      | 
      | Each component has a NamedValueSet
      | object which you can use to attach arbitrary
      | items of data to it.
      |
      */
    pub fn get_properties_mut(&mut self) -> &mut NamedValueSet {
        
        todo!();
        /*
            return properties;
        */
    }

    /**
      | Returns the set of properties that belong
      | to this component. Each component has
      | a NamedValueSet object which you can
      | use to attach arbitrary items of data
      | to it.
      |
      */
    pub fn get_properties(&self) -> &NamedValueSet {
        
        todo!();
        /*
            return properties;
        */
    }

    /**
      | Returns the object that was set by setCachedComponentImage().
      | @see setCachedComponentImage
      |
      */
    pub fn get_cached_component_image(&self) -> *mut dyn CachedComponentImage {
        
        todo!();
        /*
            return cachedImage.get();
        */
    }

    /**
      | Sets a flag to indicate whether mouse
      | drag events on this Component should
      | be ignored when it is inside a
      | 
      | Viewport with drag-to-scroll functionality
      | enabled. This is useful for Components
      | such as sliders that should not move
      | when their parent Viewport when dragged.
      |
      */
    pub fn set_viewport_ignore_drag_flag(&mut self, ignore_drag: bool)  {
        
        todo!();
        /*
            flags.viewportIgnoreDragFlag = ignoreDrag;
        */
    }

    /**
      | Retrieves the current state of the Viewport
      | drag-to-scroll functionality flag.
      | @see setViewportIgnoreDragFlag
      |
      */
    pub fn get_viewport_ignore_drag_flag(&self) -> bool {
        
        todo!();
        /*
            return flags.viewportIgnoreDragFlag;
        */
    }
    
    /**
      | Returns the title text for this component.
      | 
      | @see setTitle
      |
      */
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return componentTitle;
        */
    }

    /**
      | Returns the description for this component.
      | 
      | @see setDescription
      |
      */
    pub fn get_description(&self) -> String {
        
        todo!();
        /*
            return componentDescription;
        */
    }

    /**
      | Returns the help text for this component.
      | 
      | @see setHelpText
      |
      */
    pub fn get_help_text(&self) -> String {
        
        todo!();
        /*
            return componentHelpText;
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : component_flags(0),
        
        */
    }
    
    /**
      | Creates a component, setting its name
      | at the same time. @see getName, setName
      |
      */
    pub fn new_from_string(name: &String) -> Self {
    
        todo!();
        /*
        : component_name(name),
        : component_flags(0),

        
        */
    }
    
    pub fn set_name(&mut self, name: &String)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

        if (componentName != name)
        {
            componentName = name;

            if (flags.hasHeavyweightPeerFlag)
                if (auto* peer = getPeer())
                    peer->setTitle (name);

            ComponentBailOutChecker checker (this);
            componentListeners.callChecked (checker, [this] (ComponentListener& l) { l.componentNameChanged (*this); });
        }
        */
    }
    
    /**
      | Sets the component's ID string.
      | 
      | You can retrieve the ID using getComponentID().
      | @see getComponentID, findChildWithID
      |
      */
    pub fn set_componentid(&mut self, newid: &String)  {
        
        todo!();
        /*
            componentID = newID;
        */
    }
    
    pub fn set_visible(&mut self, should_be_visible: bool)  {
        
        todo!();
        /*
            if (flags.visibleFlag != shouldBeVisible)
        {
            // if component methods are being called from threads other than the message
            // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

            const WeakReference<Component> safePointer (this);
            flags.visibleFlag = shouldBeVisible;

            if (shouldBeVisible)
                repaint();
            else
                repaintParent();

            sendFakeMouseMove();

            if (! shouldBeVisible)
            {
                ComponentHelpers::releaseAllCachedImageResources (*this);

                if (hasKeyboardFocus (true))
                {
                    if (parentComponent != nullptr)
                        parentComponent->grabKeyboardFocus();

                    // ensure that keyboard focus is given away if it wasn't taken by parent
                    giveAwayKeyboardFocus();
                }
            }

            if (safePointer != nullptr)
            {
                sendVisibilityChangeMessage();

                if (safePointer != nullptr && flags.hasHeavyweightPeerFlag)
                {
                    if (auto* peer = getPeer())
                    {
                        peer->setVisible (shouldBeVisible);
                        internalHierarchyChanged();
                    }
                }
            }
        }
        */
    }
    
    pub fn send_visibility_change_message(&mut self)  {
        
        todo!();
        /*
            ComponentBailOutChecker checker (this);
        visibilityChanged();

        if (! checker.shouldBailOut())
            componentListeners.callChecked (checker, [this] (ComponentListener& l) { l.componentVisibilityChanged (*this); });
        */
    }
    
    /**
      | Tests whether this component and all
      | its parents are visible.
      | 
      | -----------
      | @return
      | 
      | true only if this component and all its
      | parents are visible. @see isVisible
      |
      */
    pub fn is_showing(&self) -> bool {
        
        todo!();
        /*
            if (! flags.visibleFlag)
            return false;

        if (parentComponent != nullptr)
            return parentComponent->isShowing();

        if (auto* peer = getPeer())
            return ! peer->isMinimised();

        return false;
        */
    }
    
    /**
      | Returns the underlying native window
      | handle for this component.
      | 
      | This is platform-dependent and strictly
      | for power-users only!
      |
      */
    pub fn get_window_handle(&self)  {
        
        todo!();
        /*
            if (auto* peer = getPeer())
            return peer->getNativeHandle();

        return nullptr;
        */
    }
    
    pub fn add_to_desktop(&mut self, 
        style_wanted:               i32,
        native_window_to_attach_to: *mut c_void)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (isOpaque())
            styleWanted &= ~ComponentPeer::windowIsSemiTransparent;
        else
            styleWanted |= ComponentPeer::windowIsSemiTransparent;

        // don't use getPeer(), so that we only get the peer that's specifically
        // for this comp, and not for one of its parents.
        auto* peer = ComponentPeer::getPeerFor (this);

        if (peer == nullptr || styleWanted != peer->getStyleFlags())
        {
            const WeakReference<Component> safePointer (this);

           #if ALOE_LINUX || ALOE_BSD
            // it's wise to give the component a non-zero size before
            // putting it on the desktop, as X windows get confused by this, and
            // a (1, 1) minimum size is enforced here.
            setSize (jmax (1, getWidth()),
                     jmax (1, getHeight()));
           #endif

            const auto unscaledPosition = ScalingHelpers::scaledScreenPosToUnscaled (getScreenPosition());
            const auto topLeft = ScalingHelpers::unscaledScreenPosToScaled (*this, unscaledPosition);

            bool wasFullscreen = false;
            bool wasMinimised = false;
            ComponentBoundsConstrainer* currentConstrainer = nullptr;
            Rectangle<int> oldNonFullScreenBounds;
            int oldRenderingEngine = -1;

            if (peer != nullptr)
            {
                std::unique_ptr<ComponentPeer> oldPeerToDelete (peer);

                wasFullscreen = peer->isFullScreen();
                wasMinimised = peer->isMinimised();
                currentConstrainer = peer->getConstrainer();
                oldNonFullScreenBounds = peer->getNonFullScreenBounds();
                oldRenderingEngine = peer->getCurrentRenderingEngine();

                flags.hasHeavyweightPeerFlag = false;
                Desktop::getInstance().removeDesktopComponent (this);
                internalHierarchyChanged(); // give comps a chance to react to the peer change before the old peer is deleted.

                if (safePointer == nullptr)
                    return;

                setTopLeftPosition (topLeft);
            }

            if (parentComponent != nullptr)
                parentComponent->removeChildComponent (this);

            if (safePointer != nullptr)
            {
                flags.hasHeavyweightPeerFlag = true;

                peer = createNewPeer (styleWanted, nativeWindowToAttachTo);

                Desktop::getInstance().addDesktopComponent (this);

                boundsRelativeToParent.setPosition (topLeft);
                peer->updateBounds();

                if (oldRenderingEngine >= 0)
                    peer->setCurrentRenderingEngine (oldRenderingEngine);

                peer->setVisible (isVisible());

                peer = ComponentPeer::getPeerFor (this);

                if (peer == nullptr)
                    return;

                if (wasFullscreen)
                {
                    peer->setFullScreen (true);
                    peer->setNonFullScreenBounds (oldNonFullScreenBounds);
                }

                if (wasMinimised)
                    peer->setMinimised (true);

               #if ALOE_WINDOWS
                if (isAlwaysOnTop())
                    peer->setAlwaysOnTop (true);
               #endif

                peer->setConstrainer (currentConstrainer);

                repaint();
                internalHierarchyChanged();

                if (auto* handler = getAccessibilityHandler())
                    notifyAccessibilityEventInternal (*handler, InternalAccessibilityEvent::windowOpened);
            }
        }
        */
    }
    
    /**
      | If the component is currently showing
      | on the desktop, this will hide it.
      | 
      | You can also use setVisible() to hide
      | a desktop window temporarily, but removeFromDesktop()
      | will free any system resources that
      | are being used up.
      | 
      | @see addToDesktop, isOnDesktop
      |
      */
    pub fn remove_from_desktop(&mut self)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

        if (flags.hasHeavyweightPeerFlag)
        {
            if (auto* handler = getAccessibilityHandler())
                notifyAccessibilityEventInternal (*handler, InternalAccessibilityEvent::windowClosed);

            ComponentHelpers::releaseAllCachedImageResources (*this);

            auto* peer = ComponentPeer::getPeerFor (this);
            jassert (peer != nullptr);

            flags.hasHeavyweightPeerFlag = false;
            delete peer;

            Desktop::getInstance().removeDesktopComponent (this);
        }
        */
    }
    
    /**
      | Returns true if this component is currently
      | showing on the desktop. @see addToDesktop,
      | removeFromDesktop
      |
      */
    pub fn is_on_desktop(&self) -> bool {
        
        todo!();
        /*
            return flags.hasHeavyweightPeerFlag;
        */
    }
    
    /**
      | Returns the heavyweight window that
      | contains this component.
      | 
      | If this component is itself on the desktop,
      | this will return the window object that
      | it is using. Otherwise, it will return
      | the window of its top-level parent component.
      | 
      | This may return nullptr if there isn't
      | a desktop component.
      | 
      | @see addToDesktop, isOnDesktop
      |
      */
    pub fn get_peer(&self) -> *mut ComponentPeer {
        
        todo!();
        /*
            if (flags.hasHeavyweightPeerFlag)
            return ComponentPeer::getPeerFor (this);

        if (parentComponent == nullptr)
            return nullptr;

        return parentComponent->getPeer();
        */
    }
    
    pub fn user_tried_to_close_window(&mut self)  {
        
        todo!();
        /*
            /* This means that the user's trying to get rid of your window with the 'close window' system
           menu option (on windows) or possibly the task manager - you should really handle this
           and delete or hide your component in an appropriate way.

           If you want to ignore the event and don't want to trigger this assertion, just override
           this method and do nothing.
        */
        jassertfalse;
        */
    }
    
    pub fn minimisation_state_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_desktop_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return Desktop::getInstance().getGlobalScaleFactor();
        */
    }
    
    /**
      | Indicates whether any parts of the component
      | might be transparent.
      | 
      | Components that always paint all of
      | their contents with solid colour and
      | thus completely cover any components
      | behind them should use this method to
      | tell the repaint system that they are
      | opaque.
      | 
      | This information is used to optimise
      | drawing, because it means that objects
      | underneath opaque windows don't need
      | to be painted.
      | 
      | By default, components are considered
      | transparent, unless this is used to
      | make it otherwise.
      | 
      | @see isOpaque
      |
      */
    pub fn set_opaque(&mut self, should_be_opaque: bool)  {
        
        todo!();
        /*
            if (shouldBeOpaque != flags.opaqueFlag)
        {
            flags.opaqueFlag = shouldBeOpaque;

            if (flags.hasHeavyweightPeerFlag)
                if (auto* peer = ComponentPeer::getPeerFor (this))
                    addToDesktop (peer->getStyleFlags());  // recreates the heavyweight window

            repaint();
        }
        */
    }
    
    /**
      | Returns true if no parts of this component
      | are transparent.
      | 
      | 
      | -----------
      | @return
      | 
      | the value that was set by setOpaque,
      | (the default being false) @see setOpaque
      |
      */
    pub fn is_opaque(&self) -> bool {
        
        todo!();
        /*
            return flags.opaqueFlag;
        */
    }
    
    /**
      | Gives the component a CachedComponentImage
      | that should be used to buffer its painting.
      | 
      | The object that is passed-in will be
      | owned by this component, and will be
      | deleted automatically later on. @see
      | setBufferedToImage
      |
      */
    pub fn set_cached_component_image(&mut self, new_cached_image: *mut dyn CachedComponentImage)  {
        
        todo!();
        /*
            if (cachedImage.get() != newCachedImage)
        {
            cachedImage.reset (newCachedImage);
            repaint();
        }
        */
    }
    
    /**
      | Makes the component use an internal
      | buffer to optimise its redrawing.
      | 
      | Setting this flag to true will cause
      | the component to allocate an internal
      | buffer into which it paints itself and
      | all its child components, so that when
      | asked to redraw itself, it can use this
      | buffer rather than actually calling
      | the paint() method.
      | 
      | Parts of the buffer are invalidated
      | when repaint() is called on this component
      | or its children. The buffer is then repainted
      | at the next paint() callback.
      | 
      | @see repaint, paint, createComponentSnapshot
      |
      */
    pub fn set_buffered_to_image(&mut self, should_be_buffered: bool)  {
        
        todo!();
        /*
            // This assertion means that this component is already using a custom CachedComponentImage,
        // so by calling setBufferedToImage, you'll be deleting the custom one - this is almost certainly
        // not what you wanted to happen... If you really do know what you're doing here, and want to
        // avoid this assertion, just call setCachedComponentImage (nullptr) before setBufferedToImage().
        jassert (cachedImage == nullptr || dynamic_cast<StandardCachedComponentImage*> (cachedImage.get()) != nullptr);

        if (shouldBeBuffered)
        {
            if (cachedImage == nullptr)
                cachedImage.reset (new StandardCachedComponentImage (*this));
        }
        else
        {
            cachedImage.reset();
        }
        */
    }
    
    pub fn reorder_child_internal(&mut self, 
        source_index: i32,
        dest_index:   i32)  {
        
        todo!();
        /*
            if (sourceIndex != destIndex)
        {
            auto* c = childComponentList.getUnchecked (sourceIndex);
            jassert (c != nullptr);
            c->repaintParent();

            childComponentList.move (sourceIndex, destIndex);

            sendFakeMouseMove();
            internalChildrenChanged();
        }
        */
    }
    
    /**
      | Brings the component to the front of
      | its siblings.
      | 
      | If some of the component's siblings
      | have had their 'always-on-top' flag
      | set, then they will still be kept in front
      | of this one (unless of course this one
      | is also 'always-on-top').
      | 
      | -----------
      | @param shouldAlsoGainKeyboardFocus
      | 
      | if true, this will also try to assign
      | keyboard focus to the component (see
      | grabKeyboardFocus() for more details)
      | @see toBack, toBehind, setAlwaysOnTop
      |
      */
    pub fn to_front(&mut self, should_grab_keyboard_focus: bool)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

        if (flags.hasHeavyweightPeerFlag)
        {
            if (auto* peer = getPeer())
            {
                peer->toFront (shouldGrabKeyboardFocus);

                if (shouldGrabKeyboardFocus && ! hasKeyboardFocus (true))
                    grabKeyboardFocus();
            }
        }
        else if (parentComponent != nullptr)
        {
            auto& childList = parentComponent->childComponentList;

            if (childList.getLast() != this)
            {
                auto index = childList.indexOf (this);

                if (index >= 0)
                {
                    int insertIndex = -1;

                    if (! flags.alwaysOnTopFlag)
                    {
                        insertIndex = childList.size() - 1;

                        while (insertIndex > 0 && childList.getUnchecked (insertIndex)->isAlwaysOnTop())
                            --insertIndex;
                    }

                    parentComponent->reorderChildInternal (index, insertIndex);
                }
            }

            if (shouldGrabKeyboardFocus)
            {
                internalBroughtToFront();

                if (isShowing())
                    grabKeyboardFocus();
            }
        }
        */
    }
    
    /**
      | Changes this component's z-order so
      | that it's just behind another component.
      | @see toFront, toBack
      |
      */
    pub fn to_behind(&mut self, other: *mut Component<'a>)  {
        
        todo!();
        /*
            if (other != nullptr && other != this)
        {
            // the two components must belong to the same parent..
            jassert (parentComponent == other->parentComponent);

            if (parentComponent != nullptr)
            {
                auto& childList = parentComponent->childComponentList;
                auto index = childList.indexOf (this);

                if (index >= 0 && childList [index + 1] != other)
                {
                    auto otherIndex = childList.indexOf (other);

                    if (otherIndex >= 0)
                    {
                        if (index < otherIndex)
                            --otherIndex;

                        parentComponent->reorderChildInternal (index, otherIndex);
                    }
                }
            }
            else if (isOnDesktop())
            {
                jassert (other->isOnDesktop());

                if (other->isOnDesktop())
                {
                    auto* us = getPeer();
                    auto* them = other->getPeer();
                    jassert (us != nullptr && them != nullptr);

                    if (us != nullptr && them != nullptr)
                        us->toBehind (them);
                }
            }
        }
        */
    }
    
    /**
      | Changes this component's z-order to
      | be at the back of all its siblings.
      | 
      | If the component is set to be 'always-on-top',
      | it will only be moved to the back of the
      | other other 'always-on-top' components.
      | 
      | @see toFront, toBehind, setAlwaysOnTop
      |
      */
    pub fn to_back(&mut self)  {
        
        todo!();
        /*
            if (isOnDesktop())
        {
            jassertfalse; //xxx need to add this to native window
        }
        else if (parentComponent != nullptr)
        {
            auto& childList = parentComponent->childComponentList;

            if (childList.getFirst() != this)
            {
                auto index = childList.indexOf (this);

                if (index > 0)
                {
                    int insertIndex = 0;

                    if (flags.alwaysOnTopFlag)
                        while (insertIndex < childList.size() && ! childList.getUnchecked (insertIndex)->isAlwaysOnTop())
                            ++insertIndex;

                    parentComponent->reorderChildInternal (index, insertIndex);
                }
            }
        }
        */
    }
    
    /**
      | Sets whether the component should always
      | be kept at the front of its siblings.
      | @see isAlwaysOnTop
      |
      */
    pub fn set_always_on_top(&mut self, should_stay_on_top: bool)  {
        
        todo!();
        /*
            if (shouldStayOnTop != flags.alwaysOnTopFlag)
        {
            ComponentBailOutChecker checker (this);

            flags.alwaysOnTopFlag = shouldStayOnTop;

            if (isOnDesktop())
            {
                if (auto* peer = getPeer())
                {
                    if (! peer->setAlwaysOnTop (shouldStayOnTop))
                    {
                        // some kinds of peer can't change their always-on-top status, so
                        // for these, we'll need to create a new window
                        auto oldFlags = peer->getStyleFlags();
                        removeFromDesktop();
                        addToDesktop (oldFlags);
                    }
                }
            }

            if (shouldStayOnTop && ! checker.shouldBailOut())
                toFront (false);

            if (! checker.shouldBailOut())
                internalHierarchyChanged();
        }
        */
    }
    
    /**
      | Returns true if this component is set
      | to always stay in front of its siblings.
      | @see setAlwaysOnTop
      |
      */
    pub fn is_always_on_top(&self) -> bool {
        
        todo!();
        /*
            return flags.alwaysOnTopFlag;
        */
    }
    
    /**
      | Returns a proportion of the component's
      | width. This is a handy equivalent of
      | (getWidth() * proportion).
      |
      */
    pub fn proportion_of_width(&self, proportion: f32) -> i32 {
        
        todo!();
        /*
            return roundToInt (proportion * (float) boundsRelativeToParent.getWidth());
        */
    }
    
    /**
      | Returns a proportion of the component's
      | height. This is a handy equivalent of
      | (getHeight() * proportion).
      |
      */
    pub fn proportion_of_height(&self, proportion: f32) -> i32 {
        
        todo!();
        /*
            return roundToInt (proportion * (float) boundsRelativeToParent.getHeight());
        */
    }
    
    /**
      | Returns the width of the component's
      | parent.
      | 
      | If the component has no parent (i.e.
      | if it's on the desktop), this will return
      | the width of the screen.
      |
      */
    pub fn get_parent_width(&self) -> i32 {
        
        todo!();
        /*
            return parentComponent != nullptr ? parentComponent->getWidth()
                                          : getParentMonitorArea().getWidth();
        */
    }
    
    /**
      | Returns the height of the component's
      | parent.
      | 
      | If the component has no parent (i.e.
      | if it's on the desktop), this will return
      | the height of the screen.
      |
      */
    pub fn get_parent_height(&self) -> i32 {
        
        todo!();
        /*
            return parentComponent != nullptr ? parentComponent->getHeight()
                                          : getParentMonitorArea().getHeight();
        */
    }
    
    /**
      | Returns the screen coordinates of the
      | monitor that contains this component.
      | 
      | If there's only one monitor, this will
      | return its size - if there are multiple
      | monitors, it will return the area of
      | the monitor that contains the component's
      | centre.
      |
      */
    pub fn get_parent_monitor_area(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return Desktop::getInstance().getDisplays().getDisplayForRect (getScreenBounds())->userArea;
        */
    }
    
    /**
      | Returns this component's x coordinate
      | relative the screen's top-left origin.
      | @see getX, localPointToGlobal
      |
      */
    pub fn get_screenx(&self) -> i32 {
        
        todo!();
        /*
            return getScreenPosition().x;
        */
    }
    
    /**
      | Returns this component's y coordinate
      | relative the screen's top-left origin.
      | @see getY, localPointToGlobal
      |
      */
    pub fn get_screeny(&self) -> i32 {
        
        todo!();
        /*
            return getScreenPosition().y;
        */
    }
    
    /**
      | Returns the position of this component's
      | top-left corner relative to the screen's
      | top-left. @see getScreenBounds
      |
      */
    pub fn get_screen_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return localPointToGlobal (Point<int>());
        */
    }
    
    /**
      | Returns the bounds of this component,
      | relative to the screen's top-left.
      | @see getScreenPosition
      |
      */
    pub fn get_screen_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return localAreaToGlobal (getLocalBounds());
        */
    }
    
    /**
      | Converts a point to be relative to this
      | component's coordinate space.
      | 
      | This takes a point relative to a different
      | component, and returns its position
      | relative to this component. If the sourceComponent
      | parameter is null, the source point
      | is assumed to be a global screen coordinate.
      |
      */
    pub fn get_local_point_from_point_i32(
        &self, 
        source: *const Component<'a>,
        point:  Point<i32>

    ) -> Point<i32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (this, source, point);
        */
    }
    
    /**
      | Converts a point to be relative to this
      | component's coordinate space.
      | 
      | This takes a point relative to a different
      | component, and returns its position
      | relative to this component. If the sourceComponent
      | parameter is null, the source point
      | is assumed to be a global screen coordinate.
      |
      */
    pub fn get_local_point_from_point_f32(
        &self, 
        source: *const Component<'a>,
        point:  Point<f32>

    ) -> Point<f32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (this, source, point);
        */
    }
    
    /**
      | Converts a rectangle to be relative
      | to this component's coordinate space.
      | 
      | This takes a rectangle that is relative
      | to a different component, and returns
      | its position relative to this component.
      | If the sourceComponent parameter is
      | null, the source rectangle is assumed
      | to be a screen coordinate.
      | 
      | If you've used setTransform() to apply
      | one or more transforms to components,
      | then the source rectangle may not actually
      | be rectangular when converted to the
      | target space, so in that situation this
      | will return the smallest rectangle
      | that fully contains the transformed
      | area.
      |
      */
    pub fn get_local_area(
        &self, 
        source: *const Component<'a>,
        area:   Rectangle<i32>

    ) -> Rectangle<i32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (this, source, area);
        */
    }
    
    /**
      | Converts a rectangle to be relative
      | to this component's coordinate space.
      | 
      | This takes a rectangle that is relative
      | to a different component, and returns
      | its position relative to this component.
      | If the sourceComponent parameter is
      | null, the source rectangle is assumed
      | to be a screen coordinate.
      | 
      | If you've used setTransform() to apply
      | one or more transforms to components,
      | then the source rectangle may not actually
      | be rectangular when converted to the
      | target space, so in that situation this
      | will return the smallest rectangle
      | that fully contains the transformed
      | area.
      |
      */
    pub fn get_local_area_with_rect_f32(
        &self, 
        source: *const Component<'a>,
        area:   Rectangle<f32>

    ) -> Rectangle<f32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (this, source, area);
        */
    }
    
    /**
      | Converts a point relative to this component's
      | top-left into a screen coordinate.
      | @see getLocalPoint, localAreaToGlobal
      |
      */
    pub fn local_point_to_global_with_point_i32(&self, point: Point<i32>) -> Point<i32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (nullptr, this, point);
        */
    }
    
    /**
      | Converts a point relative to this component's
      | top-left into a screen coordinate.
      | @see getLocalPoint, localAreaToGlobal
      |
      */
    pub fn local_point_to_global_with_point_f32(&self, point: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (nullptr, this, point);
        */
    }
    
    /**
      | Converts a rectangle from this component's
      | coordinate space to a screen coordinate.
      | 
      | If you've used setTransform() to apply
      | one or more transforms to components,
      | then the source rectangle may not actually
      | be rectangular when converted to the
      | target space, so in that situation this
      | will return the smallest rectangle
      | that fully contains the transformed
      | area. @see getLocalPoint, localPointToGlobal
      |
      */
    pub fn local_area_to_global_with_rect_i32(&self, area: Rectangle<i32>) -> Rectangle<i32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (nullptr, this, area);
        */
    }
    
    /**
      | Converts a rectangle from this component's
      | coordinate space to a screen coordinate.
      | 
      | If you've used setTransform() to apply
      | one or more transforms to components,
      | then the source rectangle may not actually
      | be rectangular when converted to the
      | target space, so in that situation this
      | will return the smallest rectangle
      | that fully contains the transformed
      | area. @see getLocalPoint, localPointToGlobal
      |
      */
    pub fn local_area_to_global_with_rect_f32(&self, area: Rectangle<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            return ComponentHelpers::convertCoordinate (nullptr, this, area);
        */
    }
    
    /**
      | Changes the component's position and
      | size.
      | 
      | The coordinates are relative to the
      | top-left of the component's parent,
      | or relative to the origin of the screen
      | if the component is on the desktop.
      | 
      | If this method changes the component's
      | top-left position, it will make a synchronous
      | call to moved(). If it changes the size,
      | it will also make a call to resized().
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to whatever bounds you set for
      | it.
      | 
      | @see setTopLeftPosition, setSize,
      | ComponentListener::componentMovedOrResized
      |
      */
    pub fn set_bounds_with_xywh(
        &mut self, 
        x: i32,
        y: i32,
        w: i32,
        h: i32

    )  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

        if (w < 0) w = 0;
        if (h < 0) h = 0;

        const bool wasResized  = (getWidth() != w || getHeight() != h);
        const bool wasMoved    = (getX() != x || getY() != y);

       #if ALOE_DEBUG
        // It's a very bad idea to try to resize a window during its paint() method!
        jassert (! (flags.isInsidePaintCall && wasResized && isOnDesktop()));
       #endif

        if (wasMoved || wasResized)
        {
            const bool showing = isShowing();

            if (showing)
            {
                // send a fake mouse move to trigger enter/exit messages if needed..
                sendFakeMouseMove();

                if (! flags.hasHeavyweightPeerFlag)
                    repaintParent();
            }

            boundsRelativeToParent.setBounds (x, y, w, h);

            if (showing)
            {
                if (wasResized)
                    repaint();
                else if (! flags.hasHeavyweightPeerFlag)
                    repaintParent();
            }
            else if (cachedImage != nullptr)
            {
                cachedImage->invalidateAll();
            }

            flags.isMoveCallbackPending = wasMoved;
            flags.isResizeCallbackPending = wasResized;

            if (flags.hasHeavyweightPeerFlag)
                if (auto* peer = getPeer())
                    peer->updateBounds();

            sendMovedResizedMessagesIfPending();
        }
        */
    }
    
    pub fn send_moved_resized_messages_if_pending(&mut self)  {
        
        todo!();
        /*
            const bool wasMoved   = flags.isMoveCallbackPending;
        const bool wasResized = flags.isResizeCallbackPending;

        if (wasMoved || wasResized)
        {
            flags.isMoveCallbackPending = false;
            flags.isResizeCallbackPending = false;

            sendMovedResizedMessages (wasMoved, wasResized);
        }
        */
    }
    
    pub fn send_moved_resized_messages(&mut self, 
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            ComponentBailOutChecker checker (this);

        if (wasMoved)
        {
            moved();

            if (checker.shouldBailOut())
                return;
        }

        if (wasResized)
        {
            resized();

            if (checker.shouldBailOut())
                return;

            for (int i = childComponentList.size(); --i >= 0;)
            {
                childComponentList.getUnchecked(i)->parentSizeChanged();

                if (checker.shouldBailOut())
                    return;

                i = jmin (i, childComponentList.size());
            }
        }

        if (parentComponent != nullptr)
            parentComponent->childBoundsChanged (this);

        if (! checker.shouldBailOut())
        {
            componentListeners.callChecked (checker, [this, wasMoved, wasResized] (ComponentListener& l)
            {
                l.componentMovedOrResized (*this, wasMoved, wasResized);
            });
        }

        if ((wasMoved || wasResized) && ! checker.shouldBailOut())
            if (auto* handler = getAccessibilityHandler())
                notifyAccessibilityEventInternal (*handler, InternalAccessibilityEvent::elementMovedOrResized);
        */
    }
    
    /**
      | Changes the size of the component.
      | 
      | A synchronous call to resized() will
      | occur if the size actually changes.
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to whatever bounds you set for
      | it.
      |
      */
    pub fn set_size(&mut self, w: i32, h: i32)  {
        
        todo!();
        /*
            setBounds (getX(), getY(), w, h);
        */
    }
    
    /**
      | Moves the component to a new position.
      | 
      | Changes the component's top-left position
      | (without changing its size). The position
      | is relative to the top-left of the component's
      | parent.
      | 
      | If the component actually moves, this
      | method will make a synchronous call
      | to moved().
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to whatever bounds you set for
      | it.
      | 
      | @see setBounds, ComponentListener::componentMovedOrResized
      |
      */
    pub fn set_top_left_position_with_xy(&mut self, x: i32, y: i32)  {
        
        todo!();
        /*
            setTopLeftPosition ({ x, y });
        */
    }
    
    /**
      | Moves the component to a new position.
      | 
      | Changes the component's top-left position
      | (without changing its size). The position
      | is relative to the top-left of the component's
      | parent.
      | 
      | If the component actually moves, this
      | method will make a synchronous call
      | to moved().
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to whatever bounds you set for
      | it.
      | 
      | @see setBounds, ComponentListener::componentMovedOrResized
      |
      */
    pub fn set_top_left_position_with_point_i32(&mut self, pos: Point<i32>)  {
        
        todo!();
        /*
            setBounds (pos.x, pos.y, getWidth(), getHeight());
        */
    }
    
    /**
      | Moves the component to a new position.
      | 
      | Changes the position of the component's
      | top-right corner (keeping it the same
      | size). The position is relative to the
      | top-left of the component's parent.
      | 
      | If the component actually moves, this
      | method will make a synchronous call
      | to moved().
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to whatever bounds you set for
      | it.
      |
      */
    pub fn set_top_right_position(&mut self, x: i32, y: i32)  {
        
        todo!();
        /*
            setTopLeftPosition (x - getWidth(), y);
        */
    }
    
    /**
      | Changes the component's position and
      | size.
      | 
      | The coordinates are relative to the
      | top-left of the component's parent,
      | or relative to the origin of the screen
      | if the component is on the desktop.
      | 
      | If this method changes the component's
      | top-left position, it will make a synchronous
      | call to moved(). If it changes the size,
      | it will also make a call to resized().
      | 
      | -----------
      | @note
      | 
      | if you've used setTransform() to apply
      | a transform, then the component's bounds
      | will no longer be a direct reflection
      | of the position at which it appears within
      | its parent, as the transform will be
      | applied to whatever bounds you set for
      | it.
      | 
      | @see setBounds
      |
      */
    pub fn set_bounds(&mut self, r: Rectangle<i32>)  {
        
        todo!();
        /*
            setBounds (r.getX(), r.getY(), r.getWidth(), r.getHeight());
        */
    }
    
    /**
      | Changes the position of the component's
      | centre.
      | 
      | Leaves the component's size unchanged,
      | but sets the position of its centre relative
      | to its parent's top-left.
      | 
      | @see setBounds
      |
      */
    pub fn set_centre_position(&mut self, p: Point<i32>)  {
        
        todo!();
        /*
            setBounds (getBounds().withCentre (p.transformedBy (getTransform().inverted())));
        */
    }
    
    /**
      | Changes the position of the component's
      | centre.
      | 
      | Leaves the component's size unchanged,
      | but sets the position of its centre relative
      | to its parent's top-left.
      | 
      | @see setBounds
      |
      */
    pub fn set_centre_position_with_xy(&mut self, x: i32, y: i32)  {
        
        todo!();
        /*
            setCentrePosition ({ x, y });
        */
    }
    
    /**
      | Changes the position of the component's
      | centre.
      | 
      | Leaves the size unchanged, but positions
      | its centre relative to its parent's
      | size. E.g. setCentreRelative (0.5f,
      | 0.5f) would place it centrally in its
      | parent.
      |
      */
    pub fn set_centre_relative(&mut self, x: f32, y: f32)  {
        
        todo!();
        /*
            setCentrePosition (roundToInt ((float) getParentWidth()  * x),
                           roundToInt ((float) getParentHeight() * y));
        */
    }
    
    /**
      | Changes the component's position and
      | size in terms of fractions of its parent's
      | size.
      | 
      | The values are factors of the parent's
      | size, so for example setBoundsRelative
      | ({ 0.2f, 0.2f, 0.5f, 0.5f }) would give
      | it half the width and height of the parent,
      | with its top-left position 20% of the
      | way across and down the parent.
      | 
      | @see setBounds
      |
      */
    pub fn set_bounds_relative(&mut self, target: Rectangle<f32>)  {
        
        todo!();
        /*
            setBounds ((target * Point<float> ((float) getParentWidth(),
                                           (float) getParentHeight())).toNearestInt());
        */
    }
    
    /**
      | Changes the component's position and
      | size in terms of fractions of its parent's
      | size.
      | 
      | The values are factors of the parent's
      | size, so for example setBoundsRelative
      | (0.2f, 0.2f, 0.5f, 0.5f) would give
      | it half the width and height of the parent,
      | with its top-left position 20% of the
      | way across and down the parent.
      | 
      | @see setBounds
      |
      */
    pub fn set_bounds_relative_with_xywh(
        &mut self, 
        x: f32,
        y: f32,
        w: f32,
        h: f32

    )  {
        
        todo!();
        /*
            setBoundsRelative ({ x, y, w, h });
        */
    }
    
    /**
      | Changes the component's size and centres
      | it within its parent.
      | 
      | After changing the size, the component
      | will be moved so that it's centred within
      | its parent. If the component is on the
      | desktop (or has no parent component),
      | then it'll be centred within the main
      | monitor area.
      |
      */
    pub fn centre_with_size(&mut self, 
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            auto parentArea = ComponentHelpers::getParentOrMainMonitorBounds (*this)
                              .transformedBy (getTransform().inverted());

        setBounds (parentArea.getCentreX() - width / 2,
                   parentArea.getCentreY() - height / 2,
                   width, height);
        */
    }
    
    /**
      | Changes the component's position and
      | size based on the amount of space to leave
      | around it.
      | 
      | This will position the component within
      | its parent, leaving the specified number
      | of pixels around each edge.
      | 
      | @see setBounds
      |
      */
    pub fn set_bounds_inset(&mut self, borders: BorderSize<i32>)  {
        
        todo!();
        /*
            setBounds (borders.subtractedFrom (ComponentHelpers::getParentOrMainMonitorBounds (*this)));
        */
    }
    
    /**
      | Positions the component within a given
      | rectangle, keeping its proportions
      | unchanged.
      | 
      | If onlyReduceInSize is false, the component
      | will be resized to fill as much of the
      | rectangle as possible without changing
      | its aspect ratio (the component's current
      | size is used to determine its aspect
      | ratio, so a zero-size component won't
      | work here). If onlyReduceInSize is
      | true, it will only be resized if it's
      | too big to fit inside the rectangle.
      | 
      | It will then be positioned within the
      | rectangle according to the justification
      | flags specified.
      | 
      | @see setBounds
      |
      */
    pub fn set_bounds_to_fit(&mut self, 
        target_area:         Rectangle<i32>,
        justification:       Justification,
        only_reduce_in_size: bool)  {
        
        todo!();
        /*
            if (getLocalBounds().isEmpty() || targetArea.isEmpty())
        {
            // it's no good calling this method unless both the component and
            // target rectangle have a finite size.
            jassertfalse;
            return;
        }

        auto sourceArea = targetArea.withZeroOrigin();

        if (onlyReduceInSize
             && getWidth() <= targetArea.getWidth()
             && getHeight() <= targetArea.getHeight())
        {
            sourceArea = getLocalBounds();
        }
        else
        {
            auto sourceRatio = getHeight() / (double) getWidth();
            auto targetRatio = targetArea.getHeight() / (double) targetArea.getWidth();

            if (sourceRatio <= targetRatio)
                sourceArea.setHeight (jmin (targetArea.getHeight(),
                                            roundToInt (targetArea.getWidth() * sourceRatio)));
            else
                sourceArea.setWidth (jmin (targetArea.getWidth(),
                                           roundToInt (targetArea.getHeight() / sourceRatio)));
        }

        if (! sourceArea.isEmpty())
            setBounds (justification.appliedToRectangle (sourceArea, targetArea));
        */
    }
    
    /**
      | Sets a transform matrix to be applied
      | to this component.
      | 
      | If you set a transform for a component,
      | the component's position will be warped
      | by it, relative to the component's parent's
      | top-left origin. This means that the
      | values you pass into setBounds() will
      | no longer reflect the actual area within
      | the parent that the component covers,
      | as the bounds will be transformed and
      | the component will probably end up actually
      | appearing somewhere else within its
      | parent.
      | 
      | When using transforms you need to be
      | extremely careful when converting
      | coordinates between the coordinate
      | spaces of different components or the
      | screen - you should always use getLocalPoint(),
      | getLocalArea(), etc to do this, and
      | never just manually add a component's
      | position to a point in order to convert
      | it between different components (but
      | I'm sure you would never have done that
      | anyway...).
      | 
      | Currently, transforms are not supported
      | for desktop windows, so the transform
      | will be ignored if you put a component
      | on the desktop.
      | 
      | To remove a component's transform,
      | simply pass AffineTransform() as the
      | parameter to this method.
      |
      */
    pub fn set_transform(&mut self, new_transform: &AffineTransform)  {
        
        todo!();
        /*
            // If you pass in a transform with no inverse, the component will have no dimensions,
        // and there will be all sorts of maths errors when converting coordinates.
        jassert (! newTransform.isSingularity());

        if (newTransform.isIdentity())
        {
            if (affineTransform != nullptr)
            {
                repaint();
                affineTransform.reset();
                repaint();
                sendMovedResizedMessages (false, false);
            }
        }
        else if (affineTransform == nullptr)
        {
            repaint();
            affineTransform.reset (new AffineTransform (newTransform));
            repaint();
            sendMovedResizedMessages (false, false);
        }
        else if (*affineTransform != newTransform)
        {
            repaint();
            *affineTransform = newTransform;
            repaint();
            sendMovedResizedMessages (false, false);
        }
        */
    }
    
    /**
      | Returns true if a non-identity transform
      | is being applied to this component.
      | For more details about transforms,
      | see setTransform(). @see setTransform
      |
      */
    pub fn is_transformed(&self) -> bool {
        
        todo!();
        /*
            return affineTransform != nullptr;
        */
    }
    
    /**
      | Returns the transform that is currently
      | being applied to this component. For
      | more details about transforms, see
      | setTransform(). @see setTransform
      |
      */
    pub fn get_transform(&self) -> AffineTransform {
        
        todo!();
        /*
            return affineTransform != nullptr ? *affineTransform : AffineTransform();
        */
    }
    
    /**
      | Returns the approximate scale factor
      | for a given component by traversing
      | its parent hierarchy and applying each
      | transform and finally scaling this
      | by the global scale factor.
      |
      */
    pub fn get_approximate_scale_factor_for_component(&mut self, target_component: *mut Component<'a>) -> f32 {
        
        todo!();
        /*
            AffineTransform transform;

        for (auto* target = targetComponent; target != nullptr; target = target->getParentComponent())
        {
            transform = transform.followedBy (target->getTransform());

            if (target->isOnDesktop())
                transform = transform.scaled (target->getDesktopScaleFactor());
        }

        auto transformScale = std::sqrt (std::abs (transform.getDeterminant()));
        return transformScale / Desktop::getInstance().getGlobalScaleFactor();
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            if (! flags.ignoresMouseClicksFlag)
            return true;

        if (flags.allowChildMouseClicksFlag)
        {
            for (int i = childComponentList.size(); --i >= 0;)
            {
                auto& child = *childComponentList.getUnchecked (i);

                if (child.isVisible()
                     && ComponentHelpers::hitTest (child, ComponentHelpers::convertFromParentSpace (child, Point<int> (x, y))))
                    return true;
            }
        }

        return false;
        */
    }
    
    /**
      | Changes the default return value for
      | the hitTest() method.
      | 
      | Setting this to false is an easy way to
      | make a component pass all its mouse events
      | (not just clicks) through to the components
      | behind it.
      | 
      | When a component is created, the default
      | setting for this is true.
      | 
      | -----------
      | @param allowClicksOnThisComponent
      | 
      | if true, hitTest() will always return
      | true; if false, it will return false
      | (or true for child components if allowClicksOnChildComponents
      | is true)
      | ----------
      | @param allowClicksOnChildComponents
      | 
      | if this is true and allowClicksOnThisComponent
      | is false, then child components can
      | be clicked on as normal but clicks on
      | this component pass straight through;
      | if this is false and allowClicksOnThisComponent
      | is false, then neither this component
      | nor any child components can be clicked
      | on @see hitTest, getInterceptsMouseClicks
      |
      */
    pub fn set_intercepts_mouse_clicks(&mut self, 
        allow_clicks:                     bool,
        allow_clicks_on_child_components: bool)  {
        
        todo!();
        /*
            flags.ignoresMouseClicksFlag = ! allowClicks;
        flags.allowChildMouseClicksFlag = allowClicksOnChildComponents;
        */
    }
    
    /**
      | Retrieves the current state of the mouse-click
      | interception flags.
      | 
      | On return, the two parameters are set
      | to the state used in the last call to setInterceptsMouseClicks().
      | 
      | @see setInterceptsMouseClicks
      |
      */
    pub fn get_intercepts_mouse_clicks(&self, 
        allows_clicks_on_this_component:   &mut bool,
        allows_clicks_on_child_components: &mut bool)  {
        
        todo!();
        /*
            allowsClicksOnThisComponent = ! flags.ignoresMouseClicksFlag;
        allowsClicksOnChildComponents = flags.allowChildMouseClicksFlag;
        */
    }
    
    /**
      | Returns true if a given point lies within
      | this component or one of its children.
      | 
      | Never override this method! Use hitTest
      | to create custom hit regions.
      | 
      | -----------
      | @param localPoint
      | 
      | the coordinate to test, relative to
      | this component's top-left.
      | 
      | -----------
      | @return
      | 
      | true if the point is within the component's
      | hit-test area, but only if that part
      | of the component isn't clipped by its
      | parent component. Note that this won't
      | take into account any overlapping sibling
      | components which might be in the way
      | - for that, see reallyContains() @see
      | hitTest, reallyContains, getComponentAt
      |
      */
    pub fn contains(&mut self, point: Point<i32>) -> bool {
        
        todo!();
        /*
            return containsInternal (point.toFloat());
        */
    }
    
    pub fn contains_internal(&mut self, point: Point<f32>) -> bool {
        
        todo!();
        /*
            if (ComponentHelpers::hitTest (*this, point.roundToInt()))
        {
            if (parentComponent != nullptr)
                return parentComponent->containsInternal (ComponentHelpers::convertToParentSpace (*this, point));

            if (flags.hasHeavyweightPeerFlag)
                if (auto* peer = getPeer())
                    return peer->contains (ComponentHelpers::localPositionToRawPeerPos (*this, point).roundToInt(), true);
        }

        return false;
        */
    }
    
    /**
      | Returns true if a given point lies in
      | this component, taking any overlapping
      | siblings into account.
      | 
      | -----------
      | @param localPoint
      | 
      | the coordinate to test, relative to
      | this component's top-left.
      | ----------
      | @param returnTrueIfWithinAChild
      | 
      | if the point actually lies within a child
      | of this component, this determines
      | whether that is counted as a hit. @see
      | contains, getComponentAt
      |
      */
    pub fn really_contains(&mut self, 
        point:                        Point<i32>,
        return_true_if_within_achild: bool) -> bool {
        
        todo!();
        /*
            return reallyContainsInternal (point.toFloat(), returnTrueIfWithinAChild);
        */
    }
    
    pub fn really_contains_internal(&mut self, 
        point:                        Point<f32>,
        return_true_if_within_achild: bool) -> bool {
        
        todo!();
        /*
            if (! containsInternal (point))
            return false;

        auto* top = getTopLevelComponent();
        auto* compAtPosition = top->getComponentAtInternal (top->getLocalPoint (this, point));

        return (compAtPosition == this) || (returnTrueIfWithinAChild && isParentOf (compAtPosition));
        */
    }
    
    /**
      | Returns the component at a certain point
      | within this one.
      | 
      | -----------
      | @param position
      | 
      | the coordinate to test, relative to
      | this component's top-left.
      | 
      | -----------
      | @return
      | 
      | the component that is at this position
      | - which may be 0, this component, or one
      | of its children. Note that overlapping
      | siblings that might actually be in the
      | way are not taken into account by this
      | method - to account for these, instead
      | call getComponentAt on the top-level
      | parent of this component. @see hitTest,
      | contains, reallyContains
      |
      */
    pub fn get_component_at(&mut self, position: Point<i32>) -> *mut Component {
        
        todo!();
        /*
            return getComponentAtInternal (position.toFloat());
        */
    }
    
    pub fn get_component_at_internal(&mut self, position: Point<f32>) -> *mut Component {
        
        todo!();
        /*
            if (flags.visibleFlag && ComponentHelpers::hitTest (*this, position.roundToInt()))
        {
            for (int i = childComponentList.size(); --i >= 0;)
            {
                auto* child = childComponentList.getUnchecked (i);

                child = child->getComponentAtInternal (ComponentHelpers::convertFromParentSpace (*child, position));

                if (child != nullptr)
                    return child;
            }

            return this;
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the component at a certain point
      | within this one.
      | 
      | -----------
      | @param x
      | 
      | the x coordinate to test, relative to
      | this component's left edge.
      | ----------
      | @param y
      | 
      | the y coordinate to test, relative to
      | this component's top edge.
      | 
      | -----------
      | @return
      | 
      | the component that is at this position
      | - which may be 0, this component, or one
      | of its children. Note that overlapping
      | siblings that might actually be in the
      | way are not taken into account by this
      | method - to account for these, instead
      | call getComponentAt on the top-level
      | parent of this component. @see hitTest,
      | contains, reallyContains
      |
      */
    pub fn get_component_at_xy(&mut self, x: i32, y: i32) -> *mut Component {
        
        todo!();
        /*
            return getComponentAt ({ x, y });
        */
    }
    
    /**
      | Adds a child component to this one.
      | 
      | Adding a child component does not mean
      | that the component will own or delete
      | the child - it's your responsibility
      | to delete the component. Note that it's
      | safe to delete a component without first
      | removing it from its parent - doing so
      | will automatically remove it and send
      | out the appropriate notifications
      | before the deletion completes.
      | 
      | If the child is already a child of this
      | component, then no action will be taken,
      | and its z-order will be left unchanged.
      | 
      | -----------
      | @param child
      | 
      | the new component to add. If the component
      | passed-in is already the child of another
      | component, it'll first be removed from
      | its current parent.
      | ----------
      | @param zOrder
      | 
      | The index in the child-list at which
      | this component should be inserted.
      | A value of -1 will insert it in front of
      | the others, 0 is the back. @see removeChildComponent,
      | addAndMakeVisible, addChildAndSetID,
      | getChild, ComponentListener::componentChildrenChanged
      |
      */
    pub fn add_child_component_with_maybe_z_order(
        &mut self, 
        child:   &mut Component<'a>,
        z_order: Option<i32>

    ) {

        let z_order: i32 = z_order.unwrap_or(-1);
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

        jassert (this != &child); // adding a component to itself!?

        if (child.parentComponent != this)
        {
            if (child.parentComponent != nullptr)
                child.parentComponent->removeChildComponent (&child);
            else
                child.removeFromDesktop();

            child.parentComponent = this;

            if (child.isVisible())
                child.repaintParent();

            if (! child.isAlwaysOnTop())
            {
                if (zOrder < 0 || zOrder > childComponentList.size())
                    zOrder = childComponentList.size();

                while (zOrder > 0)
                {
                    if (! childComponentList.getUnchecked (zOrder - 1)->isAlwaysOnTop())
                        break;

                    --zOrder;
                }
            }

            childComponentList.insert (zOrder, &child);

            child.internalHierarchyChanged();
            internalChildrenChanged();
        }
        */
    }
    
    /**
      | Adds a child component to this one, and
      | also makes the child visible if it isn't
      | already.
      | 
      | This is the same as calling setVisible
      | (true) on the child and then addChildComponent().
      | See addChildComponent() for more details.
      | 
      | -----------
      | @param child
      | 
      | the new component to add. If the component
      | passed-in is already the child of another
      | component, it'll first be removed from
      | its current parent.
      | ----------
      | @param zOrder
      | 
      | The index in the child-list at which
      | this component should be inserted.
      | A value of -1 will insert it in front of
      | the others, 0 is the back.
      |
      */
    pub fn add_and_make_visible(
        &mut self, 
        child:   &mut Component<'a>,
        z_order: Option<i32>

    ) {

        let z_order: i32 = z_order.unwrap_or(-1);
        
        todo!();
        /*
            child.setVisible (true);
        addChildComponent (child, zOrder);
        */
    }
    
    /**
      | Adds a child component to this one.
      | 
      | Adding a child component does not mean
      | that the component will own or delete
      | the child - it's your responsibility
      | to delete the component. Note that it's
      | safe to delete a component without first
      | removing it from its parent - doing so
      | will automatically remove it and send
      | out the appropriate notifications
      | before the deletion completes.
      | 
      | If the child is already a child of this
      | component, then no action will be taken,
      | and its z-order will be left unchanged.
      | 
      | -----------
      | @param child
      | 
      | the new component to add. If the component
      | passed-in is already the child of another
      | component, it'll first be removed from
      | its current parent.
      | ----------
      | @param zOrder
      | 
      | The index in the child-list at which
      | this component should be inserted.
      | A value of -1 will insert it in front of
      | the others, 0 is the back. @see removeChildComponent,
      | addAndMakeVisible, addChildAndSetID,
      | getChild, ComponentListener::componentChildrenChanged
      |
      */
    pub fn add_child_component(
        &mut self, 
        child:   *mut Component<'a>,
        z_order: Option<i32>

    ) {

        let z_order: i32 = z_order.unwrap_or(-1);
        
        todo!();
        /*
            if (child != nullptr)
            addChildComponent (*child, zOrder);
        */
    }
    
    /**
      | Adds a child component to this one, and
      | also makes the child visible if it isn't
      | already.
      | 
      | This is the same as calling setVisible
      | (true) on the child and then addChildComponent().
      | See addChildComponent() for more details.
      | 
      | -----------
      | @param child
      | 
      | the new component to add. If the component
      | passed-in is already the child of another
      | component, it'll first be removed from
      | its current parent.
      | ----------
      | @param zOrder
      | 
      | The index in the child-list at which
      | this component should be inserted.
      | A value of -1 will insert it in front of
      | the others, 0 is the back.
      |
      */
    pub fn add_and_make_visible_via_ptr(
        &mut self, 
        child:   *mut Component<'a>,
        z_order: Option<i32>

    ) {

        let z_order: i32 = z_order.unwrap_or(-1);
        
        todo!();
        /*
            if (child != nullptr)
            addAndMakeVisible (*child, zOrder);
        */
    }
    
    /**
      | Adds a child component to this one, makes
      | it visible, and sets its component ID.
      | @see addAndMakeVisible, addChildComponent
      |
      */
    pub fn add_child_and_setid(&mut self, 
        child:   *mut Component<'a>,
        childid: &String)  {
        
        todo!();
        /*
            if (child != nullptr)
        {
            child->setComponentID (childID);
            addAndMakeVisible (child);
        }
        */
    }
    
    /**
      | Removes one of this component's child-components.
      | 
      | If the child passed-in isn't actually
      | a child of this component (either because
      | it's invalid or is the child of a different
      | parent), then no action is taken.
      | 
      | -----------
      | @note
      | 
      | removing a child will not delete it!
      | But it's ok to delete a component without
      | first removing it - doing so will automatically
      | remove it and send out the appropriate
      | notifications before the deletion
      | completes.
      | 
      | @see addChildComponent, ComponentListener::componentChildrenChanged
      |
      */
    pub fn remove_child_component(&mut self, child: *mut Component<'a>)  {
        
        todo!();
        /*
            removeChildComponent (childComponentList.indexOf (child), true, true);
        */
    }
    
    /**
      | Removes one of this component's child-components
      | by index.
      | 
      | This will return a pointer to the component
      | that was removed, or null if the index
      | was out-of-range.
      | 
      | -----------
      | @note
      | 
      | removing a child will not delete it!
      | But it's ok to delete a component without
      | first removing it - doing so will automatically
      | remove it and send out the appropriate
      | notifications before the deletion
      | completes.
      | 
      | @see addChildComponent, ComponentListener::componentChildrenChanged
      |
      */
    pub fn remove_child_component_with_index(&mut self, index: i32) -> *mut Component {
        
        todo!();
        /*
            return removeChildComponent (index, true, true);
        */
    }
    
    pub fn remove_child_component_with_index_and_maybe_send_events(
        &mut self, 
        index:              i32,
        send_parent_events: bool,
        send_child_events:  bool

    ) -> *mut Component {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED_OR_OFFSCREEN

        if (auto* child = childComponentList [index])
        {
            sendParentEvents = sendParentEvents && child->isShowing();

            if (sendParentEvents)
            {
                sendFakeMouseMove();

                if (child->isVisible())
                    child->repaintParent();
            }

            childComponentList.remove (index);
            child->parentComponent = nullptr;

            ComponentHelpers::releaseAllCachedImageResources (*child);

            // (NB: there are obscure situations where child->isShowing() = false, but it still has the focus)
            if (child->hasKeyboardFocus (true))
            {
                const WeakReference<Component> safeThis (this);

                child->giveAwayKeyboardFocusInternal (sendChildEvents || currentlyFocusedComponent != child);

                if (sendParentEvents)
                {
                    if (safeThis == nullptr)
                        return child;

                    grabKeyboardFocus();
                }
            }

            if (sendChildEvents)
                child->internalHierarchyChanged();

            if (sendParentEvents)
                internalChildrenChanged();

            return child;
        }

        return nullptr;
        */
    }
    
    /**
      | Removes all this component's children.
      | 
      | -----------
      | @note
      | 
      | this won't delete them! To do that, use
      | deleteAllChildren() instead.
      |
      */
    pub fn remove_all_children(&mut self)  {
        
        todo!();
        /*
            while (! childComponentList.isEmpty())
            removeChildComponent (childComponentList.size() - 1);
        */
    }
    
    /**
      | Removes and deletes all of this component's
      | children. My advice is to avoid this
      | method! It's an old function that is
      | only kept here for backwards-compatibility
      | with legacy code, and should be viewed
      | with extreme suspicion by anyone attempting
      | to write modern C++. In almost all cases,
      | it's much smarter to manage the lifetimes
      | of your child components via modern
      | RAII techniques such as simply making
      | them member variables, or using std::unique_ptr,
      | OwnedArray, etc to manage their lifetimes
      | appropriately. @see removeAllChildren
      |
      */
    pub fn delete_all_children(&mut self)  {
        
        todo!();
        /*
            while (! childComponentList.isEmpty())
            delete (removeChildComponent (childComponentList.size() - 1));
        */
    }
    
    /**
      | Returns the number of child components
      | that this component contains.
      | 
      | @see getChildren, getChildComponent,
      | getIndexOfChildComponent
      |
      */
    pub fn get_num_child_components(&self) -> i32 {
        
        todo!();
        /*
            return childComponentList.size();
        */
    }
    
    /**
      | Returns one of this component's child
      | components, by it index.
      | 
      | The component with index 0 is at the back
      | of the z-order, the one at the front will
      | have index (getNumChildComponents()
      | - 1).
      | 
      | If the index is out-of-range, this will
      | return a null pointer.
      | 
      | @see getChildren, getNumChildComponents,
      | getIndexOfChildComponent
      |
      */
    pub fn get_child_component(&self, index: i32) -> *mut Component {
        
        todo!();
        /*
            return childComponentList[index];
        */
    }
    
    /**
      | Returns the index of this component
      | in the list of child components.
      | 
      | A value of 0 means it is first in the list
      | (i.e. behind all other components).
      | Higher values are further towards the
      | front.
      | 
      | Returns -1 if the component passed-in
      | is not a child of this component.
      | 
      | @see getChildren, getNumChildComponents,
      | getChildComponent, addChildComponent,
      | toFront, toBack, toBehind
      |
      */
    pub fn get_index_of_child_component(&self, child: *const Component<'a>) -> i32 {
        
        todo!();
        /*
            return childComponentList.indexOf (const_cast<Component*> (child));
        */
    }
    
    /**
      | Looks for a child component with the
      | specified ID. @see setComponentID,
      | getComponentID
      |
      */
    pub fn find_child_withid(&self, targetid: &str) -> *mut Component {
        
        todo!();
        /*
            for (auto* c : childComponentList)
            if (c->componentID == targetID)
                return c;

        return nullptr;
        */
    }
    
    /**
      | Returns the highest-level component
      | which contains this one or its parents.
      | 
      | This will search upwards in the parent-hierarchy
      | from this component, until it finds
      | the highest one that doesn't have a parent
      | (i.e. is on the desktop or not yet added
      | to a parent), and will return that.
      |
      */
    pub fn get_top_level_component(&self) -> *mut Component {
        
        todo!();
        /*
            auto* comp = this;

        while (comp->parentComponent != nullptr)
            comp = comp->parentComponent;

        return const_cast<Component*> (comp);
        */
    }
    
    /**
      | Checks whether a component is anywhere
      | inside this component or its children.
      | 
      | This will recursively check through
      | this component's children to see if
      | the given component is anywhere inside.
      |
      */
    pub fn is_parent_of(&self, possible_child: *const Component<'a>) -> bool {
        
        todo!();
        /*
            while (possibleChild != nullptr)
        {
            possibleChild = possibleChild->parentComponent;

            if (possibleChild == this)
                return true;
        }

        return false;
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn children_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn internal_children_changed(&mut self)  {
        
        todo!();
        /*
            if (componentListeners.isEmpty())
        {
            childrenChanged();
        }
        else
        {
            ComponentBailOutChecker checker (this);

            childrenChanged();

            if (! checker.shouldBailOut())
                componentListeners.callChecked (checker, [this] (ComponentListener& l) { l.componentChildrenChanged (*this); });
        }
        */
    }
    
    pub fn internal_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            ComponentBailOutChecker checker (this);

        parentHierarchyChanged();

        if (checker.shouldBailOut())
            return;

        componentListeners.callChecked (checker, [this] (ComponentListener& l) { l.componentParentHierarchyChanged (*this); });

        if (checker.shouldBailOut())
            return;

        for (int i = childComponentList.size(); --i >= 0;)
        {
            childComponentList.getUnchecked (i)->internalHierarchyChanged();

            if (checker.shouldBailOut())
            {
                // you really shouldn't delete the parent component during a callback telling you
                // that it's changed..
                jassertfalse;
                return;
            }

            i = jmin (i, childComponentList.size());
        }

        if (flags.hasHeavyweightPeerFlag)
            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::structureChanged);
        */
    }
    
    /**
      | Runs a component modally, waiting until
      | the loop terminates.
      | 
      | This method first makes the component
      | visible, brings it to the front and gives
      | it the keyboard focus.
      | 
      | It then runs a loop, dispatching messages
      | from the system message queue, but blocking
      | all mouse or keyboard messages from
      | reaching any components other than
      | this one and its children.
      | 
      | This loop continues until the component's
      | exitModalState() method is called
      | (or the component is deleted), and then
      | this method returns, returning the
      | value passed into exitModalState().
      | 
      | -----------
      | @note
      | 
      | you SHOULD NEVER USE THIS METHOD! Modal
      | loops are a dangerous construct because
      | things that happen during the events
      | that they dispatch could affect the
      | state of objects which are currently
      | in use somewhere on the stack, so when
      | the loop finishes and the stack unwinds,
      | horrible problems can occur. This is
      | especially bad in plugins, where the
      | host may choose to delete the plugin
      | during runModalLoop(), so that when
      | it returns, the entire DLL could have
      | been unloaded from memory! Also, some
      | OSes deliberately make it impossible
      | to run modal loops (e.g. Android), so
      | this method won't even exist on some
      | platforms.
      | 
      | @see enterModalState, exitModalState,
      | isCurrentlyModal, getCurrentlyModalComponent,
      | isCurrentlyBlockedByAnotherModalComponent,
      | ModalComponentManager
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn run_modal_loop(&mut self) -> i32 {
        
        todo!();
        /*
            if (! MessageManager::getInstance()->isThisTheMessageThread())
        {
            // use a callback so this can be called from non-gui threads
            return (int) (pointer_sized_int) MessageManager::getInstance()
                                               ->callFunctionOnMessageThread (&ComponentHelpers::runModalLoopCallback, this);
        }

        if (! isCurrentlyModal (false))
            enterModalState (true);

        return ModalComponentManager::getInstance()->runEventLoopForCurrentComponent();
        */
    }
    
    /**
      | Puts the component into a modal state.
      | 
      | This makes the component modal, so that
      | messages are blocked from reaching
      | any components other than this one and
      | its children, but unlike runModalLoop(),
      | this method returns immediately.
      | 
      | If takeKeyboardFocus is true, the component
      | will use grabKeyboardFocus() to get
      | the focus, which is usually what you'll
      | want it to do. If not, it will leave the
      | focus unchanged.
      | 
      | The callback is an optional object which
      | will receive a callback when the modal
      | component loses its modal status, either
      | by being hidden or when exitModalState()
      | is called. If you pass an object in here,
      | the system will take care of deleting
      | it later, after making the callback
      | 
      | If deleteWhenDismissed is true, then
      | when it is dismissed, the component
      | will be deleted and then the callback
      | will be called. (This will safely handle
      | the situation where the component is
      | deleted before its exitModalState()
      | method is called).
      | 
      | @see exitModalState, runModalLoop,
      | ModalComponentManager::attachCallback
      |
      */
    pub fn enter_modal_state(
        &mut self, 
        take_keyboard_focus:   Option<bool>,
        callback:              *mut dyn ModalComponentManagerCallback,
        delete_when_dismissed: Option<bool>

    ) {

        let take_keyboard_focus:   bool = take_keyboard_focus.unwrap_or(true);
        let delete_when_dismissed: bool = delete_when_dismissed.unwrap_or(false);
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (! isCurrentlyModal (false))
        {
            auto& mcm = *ModalComponentManager::getInstance();
            mcm.startModal (this, deleteWhenDismissed);
            mcm.attachCallback (this, callback);

            setVisible (true);

            if (shouldTakeKeyboardFocus)
                grabKeyboardFocus();
        }
        else
        {
            // Probably a bad idea to try to make a component modal twice!
            jassertfalse;
        }
        */
    }
    
    /**
      | Ends a component's modal state.
      | 
      | If this component is currently modal,
      | this will turn off its modalness, and
      | return a value to the runModalLoop()
      | method that might have be running its
      | modal loop.
      | 
      | @see runModalLoop, enterModalState,
      | isCurrentlyModal
      |
      */
    pub fn exit_modal_state(&mut self, return_value: i32)  {
        
        todo!();
        /*
            if (isCurrentlyModal (false))
        {
            if (MessageManager::getInstance()->isThisTheMessageThread())
            {
                auto& mcm = *ModalComponentManager::getInstance();
                mcm.endModal (this, returnValue);
                mcm.bringModalComponentsToFront();

                // If any of the mouse sources are over another Component when we exit the modal state then send a mouse enter event
                for (auto& ms : Desktop::getInstance().getMouseSources())
                    if (auto* c = ms.getComponentUnderMouse())
                        c->internalMouseEnter (ms, ms.getScreenPosition(), Time::getCurrentTime());
            }
            else
            {
                MessageManager::callAsync ([target = WeakReference<Component> { this }, returnValue]
                {
                    if (target != nullptr)
                        target->exitModalState (returnValue);
                });
            }
        }
        */
    }
    
    /**
      | Returns true if this component is the
      | modal one.
      | 
      | It's possible to have nested modal components,
      | e.g. a pop-up dialog box that launches
      | another pop-up. If onlyConsiderForemostModalComponent
      | is true then isCurrentlyModal will
      | only return true for the one at the top
      | of the stack. If onlyConsiderForemostModalComponent
      | is false then isCurrentlyModal will
      | return true for any modal component
      | in the stack.
      | 
      | @see getCurrentlyModalComponent
      |
      */
    pub fn is_currently_modal(
        &self, 
        only_consider_foremost_modal_component: Option<bool>

    ) -> bool {

        let only_consider_foremost_modal_component: bool = only_consider_foremost_modal_component.unwrap_or(true);
        
        todo!();
        /*
            auto& mcm = *ModalComponentManager::getInstance();

        return onlyConsiderForemostModalComponent ? mcm.isFrontModalComponent (this)
                                                  : mcm.isModal (this);
        */
    }
    
    /**
      | Checks whether there's a modal component
      | somewhere that's stopping this one
      | from receiving messages.
      | 
      | If there is a modal component, its canModalEventBeSentToComponent()
      | method will be called to see if it will
      | still allow this component to receive
      | events.
      | 
      | @see runModalLoop, getCurrentlyModalComponent
      |
      */
    pub fn is_currently_blocked_by_another_modal_component(&self) -> bool {
        
        todo!();
        /*
            auto* mc = getCurrentlyModalComponent();

        return ! (mc == nullptr || mc == this || mc->isParentOf (this)
                   || mc->canModalEventBeSentToComponent (this));
        */
    }
    
    /**
      | Returns the number of components that
      | are currently in a modal state. @see
      | getCurrentlyModalComponent
      |
      */
    pub fn get_num_currently_modal_components(&mut self) -> i32 {
        
        todo!();
        /*
            return ModalComponentManager::getInstance()->getNumModalComponents();
        */
    }
    
    /**
      | Returns one of the components that are
      | currently modal.
      | 
      | The index specifies which of the possible
      | modal components to return. The order
      | of the components in this list is the
      | reverse of the order in which they became
      | modal - so the component at index 0 is
      | always the active component, and the
      | others are progressively earlier ones
      | that are themselves now blocked by later
      | ones.
      | 
      | 
      | -----------
      | @return
      | 
      | the modal component, or null if no components
      | are modal (or if the index is out of range)
      | @see getNumCurrentlyModalComponents,
      | runModalLoop, isCurrentlyModal
      |
      */
    pub fn get_currently_modal_component(&mut self, index: Option<i32>) -> *mut Component {

        let index: i32 = index.unwrap_or(0);
        
        todo!();
        /*
            return ModalComponentManager::getInstance()->getModalComponent (index);
        */
    }
    
    /**
      | Indicates whether the component should
      | be brought to the front when clicked.
      | 
      | Setting this flag to true will cause
      | the component to be brought to the front
      | when the mouse is clicked somewhere
      | inside it or its child components.
      | 
      | -----------
      | @note
      | 
      | a top-level desktop window might still
      | be brought to the front by the operating
      | system when it's clicked, depending
      | on how the OS works.
      | 
      | By default this is set to false.
      | 
      | @see setMouseClickGrabsKeyboardFocus
      |
      */
    pub fn set_brought_to_front_on_mouse_click(&mut self, should_be_brought_to_front: bool)  {
        
        todo!();
        /*
            flags.bringToFrontOnClickFlag = shouldBeBroughtToFront;
        */
    }
    
    /**
      | Indicates whether the component should
      | be brought to the front when clicked-on.
      | @see setBroughtToFrontOnMouseClick
      |
      */
    pub fn is_brought_to_front_on_mouse_click(&self) -> bool {
        
        todo!();
        /*
            return flags.bringToFrontOnClickFlag;
        */
    }
    
    /**
      | Changes the mouse cursor shape to use
      | when the mouse is over this component.
      | 
      | -----------
      | @note
      | 
      | the cursor set by this method can be overridden
      | by the getMouseCursor method.
      | 
      | @see MouseCursor
      |
      */
    pub fn set_mouse_cursor(&mut self, new_cursor: &MouseCursor)  {
        
        todo!();
        /*
            if (cursor != newCursor)
        {
            cursor = newCursor;

            if (flags.visibleFlag)
                updateMouseCursor();
        }
        */
    }
    
    pub fn get_mouse_cursor(&mut self) -> MouseCursor {
        
        todo!();
        /*
            return cursor;
        */
    }
    
    /**
      | Forces the current mouse cursor to be
      | updated.
      | 
      | If you're overriding the getMouseCursor()
      | method to control which cursor is displayed,
      | then this will only be checked each time
      | the user moves the mouse. So if you want
      | to force the system to check that the
      | cursor being displayed is up-to-date
      | (even if the mouse is just sitting there),
      | call this method.
      | 
      | (If you're changing the cursor using
      | setMouseCursor(), you don't need to
      | bother calling this).
      |
      */
    pub fn update_mouse_cursor(&self)  {
        
        todo!();
        /*
            Desktop::getInstance().getMainMouseSource().forceMouseCursorUpdate();
        */
    }
    
    /**
      | Causes automatic repaints when the
      | mouse enters or exits this component.
      | 
      | If turned on, then when the mouse enters/exits,
      | or when the button is pressed/released
      | on the component, it will trigger a repaint.
      | 
      | This is handy for things like buttons
      | that need to draw themselves differently
      | when the mouse moves over them, and it
      | avoids having to override all the different
      | mouse callbacks and call repaint().
      | 
      | @see mouseEnter, mouseExit, mouseDown,
      | mouseUp
      |
      */
    pub fn set_repaints_on_mouse_activity(&mut self, should_repaint: bool)  {
        
        todo!();
        /*
            flags.repaintOnMouseActivityFlag = shouldRepaint;
        */
    }
    
    /**
      | Returns the component's current transparency
      | level. See setAlpha() for more details.
      |
      */
    pub fn get_alpha(&self) -> f32 {
        
        todo!();
        /*
            return (255 - componentTransparency) / 255.0f;
        */
    }
    
    /**
      | Changes the transparency of this component.
      | When painted, the entire component
      | and all its children will be rendered
      | with this as the overall opacity level,
      | where 0 is completely invisible, and
      | 1.0 is fully opaque (i.e. normal).
      | 
      | @see getAlpha, alphaChanged
      |
      */
    pub fn set_alpha(&mut self, new_alpha: f32)  {
        
        todo!();
        /*
            auto newIntAlpha = (uint8) (255 - jlimit (0, 255, roundToInt (newAlpha * 255.0)));

        if (componentTransparency != newIntAlpha)
        {
            componentTransparency = newIntAlpha;
            alphaChanged();
        }
        */
    }
    
    pub fn alpha_changed(&mut self)  {
        
        todo!();
        /*
            if (flags.hasHeavyweightPeerFlag)
        {
            if (auto* peer = getPeer())
                peer->setAlpha (getAlpha());
        }
        else
        {
            repaint();
        }
        */
    }
    
    /**
      | Marks the whole component as needing
      | to be redrawn.
      | 
      | Calling this will not do any repainting
      | immediately, but will mark the component
      | as 'dirty'. At some point in the near
      | future the operating system will send
      | a paint message, which will redraw all
      | the dirty regions of all components.
      | There's no guarantee about how soon
      | after calling repaint() the redraw
      | will actually happen, and other queued
      | events may be delivered before a redraw
      | is done.
      | 
      | If the setBufferedToImage() method
      | has been used to cause this component
      | to use a buffer, the repaint() call will
      | invalidate the cached buffer. If setCachedComponentImage()
      | has been used to provide a custom image
      | cache, that cache will be invalidated
      | appropriately.
      | 
      | To redraw just a subsection of the component
      | rather than the whole thing, use the
      | repaint (int, int, int, int) method.
      | 
      | @see paint
      |
      */
    pub fn repaint(&mut self)  {
        
        todo!();
        /*
            internalRepaintUnchecked (getLocalBounds(), true);
        */
    }
    
    /**
      | Marks a subsection of this component
      | as needing to be redrawn.
      | 
      | Calling this will not do any repainting
      | immediately, but will mark the given
      | region of the component as 'dirty'.
      | At some point in the near future the operating
      | system will send a paint message, which
      | will redraw all the dirty regions of
      | all components. There's no guarantee
      | about how soon after calling repaint()
      | the redraw will actually happen, and
      | other queued events may be delivered
      | before a redraw is done.
      | 
      | The region that is passed in will be clipped
      | to keep it within the bounds of this component.
      | 
      | @see repaint()
      |
      */
    pub fn repaint_xywh(
        &mut self, 
        x: i32,
        y: i32,
        w: i32,
        h: i32)  {
        
        todo!();
        /*
            internalRepaint ({ x, y, w, h });
        */
    }
    
    /**
      | Marks a subsection of this component
      | as needing to be redrawn.
      | 
      | Calling this will not do any repainting
      | immediately, but will mark the given
      | region of the component as 'dirty'.
      | At some point in the near future the operating
      | system will send a paint message, which
      | will redraw all the dirty regions of
      | all components. There's no guarantee
      | about how soon after calling repaint()
      | the redraw will actually happen, and
      | other queued events may be delivered
      | before a redraw is done.
      | 
      | The region that is passed in will be clipped
      | to keep it within the bounds of this component.
      | 
      | @see repaint()
      |
      */
    pub fn repaint_rect_i32(&mut self, area: Rectangle<i32>)  {
        
        todo!();
        /*
            internalRepaint (area);
        */
    }
    
    pub fn repaint_parent(&mut self)  {
        
        todo!();
        /*
            if (parentComponent != nullptr)
            parentComponent->internalRepaint (ComponentHelpers::convertToParentSpace (*this, getLocalBounds()));
        */
    }
    
    pub fn internal_repaint(&mut self, area: Rectangle<i32>)  {
        
        todo!();
        /*
            area = area.getIntersection (getLocalBounds());

        if (! area.isEmpty())
            internalRepaintUnchecked (area, false);
        */
    }
    
    pub fn internal_repaint_unchecked(&mut self, 
        area:                Rectangle<i32>,
        is_entire_component: bool)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (flags.visibleFlag)
        {
            if (cachedImage != nullptr)
                if (! (isEntireComponent ? cachedImage->invalidateAll()
                                         : cachedImage->invalidate (area)))
                    return;

            if (area.isEmpty())
                return;

            if (flags.hasHeavyweightPeerFlag)
            {
                if (auto* peer = getPeer())
                {
                    // Tweak the scaling so that the component's integer size exactly aligns with the peer's scaled size
                    auto peerBounds = peer->getBounds();
                    auto scaled = area * Point<float> ((float) peerBounds.getWidth()  / (float) getWidth(),
                                                       (float) peerBounds.getHeight() / (float) getHeight());

                    peer->repaint (affineTransform != nullptr ? scaled.transformedBy (*affineTransform) : scaled);
                }
            }
            else
            {
                if (parentComponent != nullptr)
                    parentComponent->internalRepaint (ComponentHelpers::convertToParentSpace (*this, area));
            }
        }
        */
    }
    
    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
            // if your component is marked as opaque, you must implement a paint
        // method and ensure that its entire area is completely painted.
        jassert (getBounds().isEmpty() || ! isOpaque());
        */
    }
    
    pub fn paint_over_children(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
            // all painting is done in the subclasses
        */
    }
    
    pub fn paint_within_parent_context(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setOrigin (getPosition());

        if (cachedImage != nullptr)
            cachedImage->paint (g);
        else
            paintEntireComponent (g, false);
        */
    }
    
    pub fn paint_component_and_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto clipBounds = g.getClipBounds();

        if (flags.dontClipGraphicsFlag)
        {
            paint (g);
        }
        else
        {
            Graphics::ScopedSaveState ss (g);

            if (! (ComponentHelpers::clipObscuredRegions (*this, g, clipBounds, {}) && g.isClipEmpty()))
                paint (g);
        }

        for (int i = 0; i < childComponentList.size(); ++i)
        {
            auto& child = *childComponentList.getUnchecked (i);

            if (child.isVisible())
            {
                if (child.affineTransform != nullptr)
                {
                    Graphics::ScopedSaveState ss (g);

                    g.addTransform (*child.affineTransform);

                    if ((child.flags.dontClipGraphicsFlag && ! g.isClipEmpty()) || g.reduceClipRegion (child.getBounds()))
                        child.paintWithinParentContext (g);
                }
                else if (clipBounds.intersects (child.getBounds()))
                {
                    Graphics::ScopedSaveState ss (g);

                    if (child.flags.dontClipGraphicsFlag)
                    {
                        child.paintWithinParentContext (g);
                    }
                    else if (g.reduceClipRegion (child.getBounds()))
                    {
                        bool nothingClipped = true;

                        for (int j = i + 1; j < childComponentList.size(); ++j)
                        {
                            auto& sibling = *childComponentList.getUnchecked (j);

                            if (sibling.flags.opaqueFlag && sibling.isVisible() && sibling.affineTransform == nullptr)
                            {
                                nothingClipped = false;
                                g.excludeClipRegion (sibling.getBounds());
                            }
                        }

                        if (nothingClipped || ! g.isClipEmpty())
                            child.paintWithinParentContext (g);
                    }
                }
            }
        }

        Graphics::ScopedSaveState ss (g);
        paintOverChildren (g);
        */
    }
    
    /**
      | Draws this component and all its subcomponents
      | onto the specified graphics context.
      | 
      | You should very rarely have to use this
      | method, it's simply there in case you
      | need to draw a component with a custom
      | graphics context for some reason, e.g.
      | for creating a snapshot of the component.
      | 
      | It calls paint(), paintOverChildren()
      | and recursively calls paintEntireComponent()
      | on its children in order to render the
      | entire tree.
      | 
      | The graphics context may be left in an
      | undefined state after this method returns,
      | so you may need to reset it if you're going
      | to use it again.
      | 
      | If ignoreAlphaLevel is false, then
      | the component will be drawn with the
      | opacity level specified by getAlpha();
      | if ignoreAlphaLevel is true, then this
      | will be ignored and an alpha of 1.0 will
      | be used.
      |
      */
    pub fn paint_entire_component(&mut self, 
        g:                  &mut Graphics,
        ignore_alpha_level: bool)  {
        
        todo!();
        /*
            // If sizing a top-level-window and the OS paint message is delivered synchronously
        // before resized() is called, then we'll invoke the callback here, to make sure
        // the components inside have had a chance to sort their sizes out..
       #if ALOE_DEBUG
        if (! flags.isInsidePaintCall) // (avoids an assertion in plugins hosted in WaveLab)
       #endif
            sendMovedResizedMessagesIfPending();

       #if ALOE_DEBUG
        flags.isInsidePaintCall = true;
       #endif

        if (effect != nullptr)
        {
            auto scale = g.getInternalContext().getPhysicalPixelScaleFactor();

            auto scaledBounds = getLocalBounds() * scale;

            Image effectImage (flags.opaqueFlag ? Image::RGB : Image::ARGB,
                               scaledBounds.getWidth(), scaledBounds.getHeight(), ! flags.opaqueFlag);
            {
                Graphics g2 (effectImage);
                g2.addTransform (AffineTransform::scale ((float) scaledBounds.getWidth()  / (float) getWidth(),
                                                         (float) scaledBounds.getHeight() / (float) getHeight()));
                paintComponentAndChildren (g2);
            }

            Graphics::ScopedSaveState ss (g);

            g.addTransform (AffineTransform::scale (1.0f / scale));
            effect->applyEffect (effectImage, g, scale, ignoreAlphaLevel ? 1.0f : getAlpha());
        }
        else if (componentTransparency > 0 && ! ignoreAlphaLevel)
        {
            if (componentTransparency < 255)
            {
                g.beginTransparencyLayer (getAlpha());
                paintComponentAndChildren (g);
                g.endTransparencyLayer();
            }
        }
        else
        {
            paintComponentAndChildren (g);
        }

       #if ALOE_DEBUG
        flags.isInsidePaintCall = false;
       #endif
        */
    }
    
    /**
      | This allows you to indicate that this
      | component doesn't require its graphics
      | context to be clipped when it is being
      | painted.
      | 
      | Most people will never need to use this
      | setting, but in situations where you
      | have a very large number of simple components
      | being rendered, and where they are guaranteed
      | never to do any drawing beyond their
      | own boundaries, setting this to true
      | will reduce the overhead involved in
      | clipping the graphics context that
      | gets passed to the component's paint()
      | callback. If you enable this mode, you'll
      | need to make sure your paint method doesn't
      | call anything like Graphics::fillAll(),
      | and doesn't draw beyond the component's
      | bounds, because that'll produce artifacts.
      | Your component also can't have any child
      | components that may be placed beyond
      | its bounds.
      |
      */
    pub fn set_painting_is_unclipped(&mut self, should_paint_without_clipping: bool)  {
        
        todo!();
        /*
            flags.dontClipGraphicsFlag = shouldPaintWithoutClipping;
        */
    }
    
    /**
      | Returns true if this component doesn't
      | require its graphics context to be clipped
      | when it is being painted.
      |
      */
    pub fn is_painting_unclipped(&self) -> bool {
        
        todo!();
        /*
            return flags.dontClipGraphicsFlag;
        */
    }
    
    /**
      | Generates a snapshot of part of this
      | component.
      | 
      | This will return a new Image, the size
      | of the rectangle specified, containing
      | a snapshot of the specified area of the
      | component and all its children.
      | 
      | The image may or may not have an alpha-channel,
      | depending on whether the image is opaque
      | or not.
      | 
      | If the clipImageToComponentBounds
      | parameter is true and the area is greater
      | than the size of the component, it'll
      | be clipped. If clipImageToComponentBounds
      | is false then parts of the component
      | beyond its bounds can be drawn.
      | 
      | @see paintEntireComponent
      |
      */
    pub fn create_component_snapshot(
        &mut self, 
        area_to_grab:                   Rectangle<i32>,
        clip_image_to_component_bounds: Option<bool>,
        scale_factor:                   Option<f32>

    ) -> Image {

        let clip_image_to_component_bounds: bool = clip_image_to_component_bounds.unwrap_or(true);
        let scale_factor: f32 = scale_factor.unwrap_or(1.0);
        
        todo!();
        /*
            auto r = areaToGrab;

        if (clipImageToComponentBounds)
            r = r.getIntersection (getLocalBounds());

        if (r.isEmpty())
            return {};

        auto w = roundToInt (scaleFactor * (float) r.getWidth());
        auto h = roundToInt (scaleFactor * (float) r.getHeight());

        Image image (flags.opaqueFlag ? Image::RGB : Image::ARGB, w, h, true);

        Graphics g (image);

        if (w != getWidth() || h != getHeight())
            g.addTransform (AffineTransform::scale ((float) w / (float) r.getWidth(),
                                                    (float) h / (float) r.getHeight()));
        g.setOrigin (-r.getPosition());

        paintEntireComponent (g, true);

        return image;
        */
    }
    
    /**
      | Adds an effect filter to alter the component's
      | appearance.
      | 
      | When a component has an effect filter
      | set, then this is applied to the results
      | of its paint() method. There are a few
      | preset effects, such as a drop-shadow
      | or glow, but they can be user-defined
      | as well.
      | 
      | The effect that is passed in will not
      | be deleted by the component - the caller
      | must take care of deleting it.
      | 
      | To remove an effect from a component,
      | pass a null pointer in as the parameter.
      | 
      | @see ImageEffectFilter, DropShadowEffect,
      | GlowEffect
      |
      */
    pub fn set_component_effect(&mut self, new_effect: *mut dyn ImageEffectFilter)  {
        
        todo!();
        /*
            if (effect != newEffect)
        {
            effect = newEffect;
            repaint();
        }
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Calls the lookAndFeelChanged() method
      | in this component and all its children.
      | 
      | This will recurse through the children
      | and their children, calling lookAndFeelChanged()
      | on them all.
      | 
      | @see lookAndFeelChanged
      |
      */
    pub fn send_look_and_feel_change(&mut self)  {
        
        todo!();
        /*
            const WeakReference<Component> safePointer (this);
        repaint();
        lookAndFeelChanged();

        if (safePointer != nullptr)
        {
            colourChanged();

            if (safePointer != nullptr)
            {
                for (int i = childComponentList.size(); --i >= 0;)
                {
                    childComponentList.getUnchecked (i)->sendLookAndFeelChange();

                    if (safePointer == nullptr)
                        return;

                    i = jmin (i, childComponentList.size());
                }
            }
        }
        */
    }
    
    /**
      | Looks for a colour that has been registered
      | with the given colour ID number.
      | 
      | If a colour has been set for this ID number
      | using setColour(), then it is returned.
      | If none has been set, the method will
      | try calling the component's LookAndFeel
      | class's findColour() method. If none
      | has been registered with the look-and-feel
      | either, it will just return black.
      | 
      | The colour IDs for various purposes
      | are stored as enums in the components
      | that they are relevant to - for an example,
      | see Slider::ColourIds, Label::ColourIds,
      | TextEditor::ColourIds, TreeView::ColourIds,
      | etc.
      | 
      | @see setColour, isColourSpecified,
      | colourChanged, LookAndFeel::findColour,
      | LookAndFeel::setColour
      |
      */
    pub fn find_colour(
        &self, 
        colourid:            i32,
        inherit_from_parent: Option<bool>

    ) -> Colour {

        let inherit_from_parent: bool = inherit_from_parent.unwrap_or(false);
        
        todo!();
        /*
            if (auto* v = properties.getVarPointer (ComponentHelpers::getColourPropertyID (colourID)))
            return Colour ((uint32) static_cast<int> (*v));

        if (inheritFromParent && parentComponent != nullptr
             && (lookAndFeel == nullptr || ! lookAndFeel->isColourSpecified (colourID)))
            return parentComponent->findColour (colourID, true);

        return getLookAndFeel().findColour (colourID);
        */
    }
    
    /**
      | Returns true if the specified colour
      | ID has been explicitly set for this component
      | using the setColour() method.
      |
      */
    pub fn is_colour_specified(&self, colourid: i32) -> bool {
        
        todo!();
        /*
            return properties.contains (ComponentHelpers::getColourPropertyID (colourID));
        */
    }
    
    /**
      | If a colour has been set with setColour(),
      | this will remove it.
      | 
      | This allows you to make a colour revert
      | to its default state.
      |
      */
    pub fn remove_colour(&mut self, colourid: i32)  {
        
        todo!();
        /*
            if (properties.remove (ComponentHelpers::getColourPropertyID (colourID)))
            colourChanged();
        */
    }
    
    /**
      | Registers a colour to be used for a particular
      | purpose.
      | 
      | Changing a colour will cause a synchronous
      | callback to the colourChanged() method,
      | which your component can override if
      | it needs to do something when colours
      | are altered.
      | 
      | For more details about colour IDs, see
      | the comments for findColour().
      | 
      | @see findColour, isColourSpecified,
      | colourChanged, LookAndFeel::findColour,
      | LookAndFeel::setColour
      |
      */
    pub fn set_colour(&mut self, 
        colourid: i32,
        colour:   Colour)  {
        
        todo!();
        /*
            if (properties.set (ComponentHelpers::getColourPropertyID (colourID), (int) colour.getARGB()))
            colourChanged();
        */
    }
    
    /**
      | This looks for any colours that have
      | been specified for this component,
      | and copies them to the specified target
      | component.
      |
      */
    pub fn copy_all_explicit_colours_to(&self, target: &mut Component<'a>)  {
        
        todo!();
        /*
            bool changed = false;

        for (int i = properties.size(); --i >= 0;)
        {
            auto name = properties.getName(i);

            if (name.toString().startsWith (colourPropertyPrefix))
                if (target.properties.set (name, properties [name]))
                    changed = true;
        }

        if (changed)
            target.colourChanged();
        */
    }
    
    /**
      | Returns the ComponentPositioner object that
      | has been set for this component. @see
      | setPositioner()
      |
      */
    pub fn get_positioner(&self) -> *mut ComponentPositioner {
        
        todo!();
        /*
            return positioner.get();
        */
    }
    
    /**
      | Sets a new ComponentPositioner object for this
      | component.
      | 
      | If there's currently another positioner
      | set, it will be deleted. The object that
      | is passed in will be deleted automatically
      | by this component when it's no longer
      | required. Pass a null pointer to clear
      | the current positioner. @see getPositioner()
      |
      */
    pub fn set_positioner(&mut self, new_positioner: *mut ComponentPositioner)  {
        
        todo!();
        /*
            // You can only assign a positioner to the component that it was created for!
        jassert (newPositioner == nullptr || this == &(newPositioner->getComponent()));
        positioner.reset (newPositioner);
        */
    }
    
    /**
      | Returns the component's bounds, relative
      | to its own origin.
      | 
      | This is like getBounds(), but returns
      | the rectangle in local coordinates,
      | In practice, it'll return a rectangle
      | with position (0, 0), and the same size
      | as this component.
      |
      */
    pub fn get_local_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return boundsRelativeToParent.withZeroOrigin();
        */
    }
    
    /**
      | Returns the area of this component's
      | parent which this component covers.
      | 
      | The returned area is relative to the
      | parent's coordinate space. If the component
      | has an affine transform specified,
      | then the resulting area will be the smallest
      | rectangle that fully covers the component's
      | transformed bounding box. If this component
      | has no parent, the return value will
      | simply be the same as getBounds().
      |
      */
    pub fn get_bounds_in_parent(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return affineTransform == nullptr ? boundsRelativeToParent
                                          : boundsRelativeToParent.transformedBy (*affineTransform);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn moved(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn child_bounds_changed(&mut self, _0: *mut Component<'a>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn parent_size_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Adds a listener to be told about changes
      | to the component hierarchy or position.
      | 
      | Component listeners get called when
      | this component's size, position or
      | children change - see the ComponentListener
      | class for more details.
      | 
      | -----------
      | @param newListener
      | 
      | the listener to register - if this is
      | already registered, it will be ignored.
      | @see ComponentListener, removeComponentListener
      |
      */
    pub fn add_component_listener(&mut self, new_listener: *mut dyn ComponentListener)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
       #if ALOE_DEBUG || ALOE_LOG_ASSERTIONS
        if (getParentComponent() != nullptr)
        {
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED
        }
       #endif

        componentListeners.add (newListener);
        */
    }
    
    /**
      | Removes a component listener. @see
      | addComponentListener
      |
      */
    pub fn remove_component_listener(&mut self, listener_to_remove: *mut dyn ComponentListener)  {
        
        todo!();
        /*
            componentListeners.remove (listenerToRemove);
        */
    }
    
    pub fn input_attempt_when_modal(&mut self)  {
        
        todo!();
        /*
            ModalComponentManager::getInstance()->bringModalComponentsToFront();
        getLookAndFeel().playAlertSound();
        */
    }
    
    pub fn can_modal_event_be_sent_to_component(&mut self, _0: *const Component<'a>) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn internal_modal_input_attempt(&mut self)  {
        
        todo!();
        /*
            if (auto* current = getCurrentlyModalComponent())
            current->inputAttemptWhenModal();
        */
    }
    
    /**
      | Dispatches a numbered message to this
      | component.
      | 
      | This is a quick and cheap way of allowing
      | simple asynchronous messages to be
      | sent to components. It's also safe,
      | because if the component that you send
      | the message to is a null or dangling pointer,
      | this won't cause an error.
      | 
      | The command ID is later delivered to
      | the component's handleCommandMessage()
      | method by the application's message
      | queue.
      | 
      | @see handleCommandMessage
      |
      */
    pub fn post_command_message(&mut self, commandid: i32)  {
        
        todo!();
        /*
            MessageManager::callAsync ([target = WeakReference<Component> { this }, commandID]
        {
            if (target != nullptr)
                target->handleCommandMessage (commandID);
        });
        */
    }
    
    pub fn handle_command_message(&mut self, _0: i32)  {
        
        todo!();
        /*
            // used by subclasses
        */
    }
    
    /**
      | Registers a listener to be told when
      | mouse events occur in this component.
      | 
      | If you need to get informed about mouse
      | events in a component but can't or don't
      | want to override its methods, you can
      | attach any number of listeners to the
      | component, and these will get told about
      | the events in addition to the component's
      | own callbacks being called.
      | 
      | -----------
      | @note
      | 
      | a MouseListener can also be attached
      | to more than one component.
      | 
      | -----------
      | @param newListener
      | 
      | the listener to register
      | ----------
      | @param wantsEventsForAllNestedChildComponents
      | 
      | if true, the listener will receive callbacks
      | for events that happen to any child component
      | within this component, including deeply-nested
      | child components. If false, it will
      | only be told about events that this component
      | handles. @see MouseListener, removeMouseListener
      |
      */
    pub fn add_mouse_listener(
        &mut self, 
        new_listener:                                 *mut dyn MouseListener,
        wants_events_for_all_nested_child_components: bool

    ) {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        // If you register a component as a mouselistener for itself, it'll receive all the events
        // twice - once via the direct callback that all components get anyway, and then again as a listener!
        jassert ((newListener != this) || wantsEventsForAllNestedChildComponents);

        if (mouseListeners == nullptr)
            mouseListeners.reset (new MouseListenerList());

        mouseListeners->addListener (newListener, wantsEventsForAllNestedChildComponents);
        */
    }
    
    /**
      | Deregisters a mouse listener. @see
      | addMouseListener, MouseListener
      |
      */
    pub fn remove_mouse_listener(&mut self, listener_to_remove: *mut dyn MouseListener)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (mouseListeners != nullptr)
            mouseListeners->removeListener (listenerToRemove);
        */
    }
    
    pub fn internal_mouse_enter(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time)  {
        
        todo!();
        /*
            if (isCurrentlyBlockedByAnotherModalComponent())
        {
            // if something else is modal, always just show a normal mouse cursor
            source.showMouseCursor (MouseCursor::NormalCursor);
            return;
        }

        if (flags.repaintOnMouseActivityFlag)
            repaint();

        ComponentBailOutChecker checker (this);

        const MouseEvent me (source, relativePos, source.getCurrentModifiers(), MouseInputSource::invalidPressure,
                             MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                             MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                             this, this, time, relativePos, time, 0, false);
        mouseEnter (me);

        flags.cachedMouseInsideComponent = true;

        if (checker.shouldBailOut())
            return;

        Desktop::getInstance().getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseEnter (me); });

        MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseEnter, me);
        */
    }
    
    pub fn internal_mouse_exit(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time)  {
        
        todo!();
        /*
            if (isCurrentlyBlockedByAnotherModalComponent())
        {
            // if something else is modal, always just show a normal mouse cursor
            source.showMouseCursor (MouseCursor::NormalCursor);
            return;
        }

        if (flags.repaintOnMouseActivityFlag)
            repaint();

        flags.cachedMouseInsideComponent = false;

        ComponentBailOutChecker checker (this);

        const MouseEvent me (source, relativePos, source.getCurrentModifiers(), MouseInputSource::invalidPressure,
                             MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                             MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                             this, this, time, relativePos, time, 0, false);

        mouseExit (me);

        if (checker.shouldBailOut())
            return;

        Desktop::getInstance().getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseExit (me); });

        MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseExit, me);
        */
    }
    
    pub fn internal_mouse_down(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time,
        pressure:     f32,
        orientation:  f32,
        rotation:     f32,
        tiltx:        f32,
        tilty:        f32)  {
        
        todo!();
        /*
            auto& desktop = Desktop::getInstance();
        ComponentBailOutChecker checker (this);

        if (isCurrentlyBlockedByAnotherModalComponent())
        {
            flags.mouseDownWasBlocked = true;
            internalModalInputAttempt();

            if (checker.shouldBailOut())
                return;

            // If processing the input attempt has exited the modal loop, we'll allow the event
            // to be delivered..
            if (isCurrentlyBlockedByAnotherModalComponent())
            {
                // allow blocked mouse-events to go to global listeners..
                const MouseEvent me (source, relativePos, source.getCurrentModifiers(), pressure,
                                     orientation, rotation, tiltX, tiltY, this, this, time, relativePos,
                                     time, source.getNumberOfMultipleClicks(), false);

                desktop.getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseDown (me); });
                return;
            }
        }

        flags.mouseDownWasBlocked = false;

        for (auto* c = this; c != nullptr; c = c->parentComponent)
        {
            if (c->isBroughtToFrontOnMouseClick())
            {
                c->toFront (true);

                if (checker.shouldBailOut())
                    return;
            }
        }

        if (! flags.dontFocusOnMouseClickFlag)
        {
            grabKeyboardFocusInternal (focusChangedByMouseClick, true);

            if (checker.shouldBailOut())
                return;
        }

        if (flags.repaintOnMouseActivityFlag)
            repaint();

        const MouseEvent me (source, relativePos, source.getCurrentModifiers(), pressure,
                             orientation, rotation, tiltX, tiltY, this, this, time, relativePos,
                             time, source.getNumberOfMultipleClicks(), false);
        mouseDown (me);

        if (checker.shouldBailOut())
            return;

        desktop.getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseDown (me); });

        MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseDown, me);
        */
    }
    
    pub fn internal_mouse_up(&mut self, 
        source:        MouseInputSource,
        relative_pos:  Point<f32>,
        time:          Time,
        old_modifiers: ModifierKeys,
        pressure:      f32,
        orientation:   f32,
        rotation:      f32,
        tiltx:         f32,
        tilty:         f32)  {
        
        todo!();
        /*
            if (flags.mouseDownWasBlocked && isCurrentlyBlockedByAnotherModalComponent())
            return;

        ComponentBailOutChecker checker (this);

        if (flags.repaintOnMouseActivityFlag)
            repaint();

        const MouseEvent me (source, relativePos, oldModifiers, pressure, orientation,
                             rotation, tiltX, tiltY, this, this, time,
                             getLocalPoint (nullptr, source.getLastMouseDownPosition()),
                             source.getLastMouseDownTime(),
                             source.getNumberOfMultipleClicks(),
                             source.isLongPressOrDrag());
        mouseUp (me);

        if (checker.shouldBailOut())
            return;

        auto& desktop = Desktop::getInstance();
        desktop.getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseUp (me); });

        MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseUp, me);

        if (checker.shouldBailOut())
            return;

        // check for double-click
        if (me.getNumberOfClicks() >= 2)
        {
            mouseDoubleClick (me);

            if (checker.shouldBailOut())
                return;

            desktop.mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseDoubleClick (me); });
            MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseDoubleClick, me);
        }
        */
    }
    
    pub fn internal_mouse_drag(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time,
        pressure:     f32,
        orientation:  f32,
        rotation:     f32,
        tiltx:        f32,
        tilty:        f32)  {
        
        todo!();
        /*
            if (! isCurrentlyBlockedByAnotherModalComponent())
        {
            ComponentBailOutChecker checker (this);

            const MouseEvent me (source, relativePos, source.getCurrentModifiers(),
                                 pressure, orientation, rotation, tiltX, tiltY, this, this, time,
                                 getLocalPoint (nullptr, source.getLastMouseDownPosition()),
                                 source.getLastMouseDownTime(),
                                 source.getNumberOfMultipleClicks(),
                                 source.isLongPressOrDrag());
            mouseDrag (me);

            if (checker.shouldBailOut())
                return;

            Desktop::getInstance().getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseDrag (me); });

            MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseDrag, me);
        }
        */
    }
    
    pub fn internal_mouse_move(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time)  {
        
        todo!();
        /*
            auto& desktop = Desktop::getInstance();

        if (isCurrentlyBlockedByAnotherModalComponent())
        {
            // allow blocked mouse-events to go to global listeners..
            desktop.sendMouseMove();
        }
        else
        {
            ComponentBailOutChecker checker (this);

            const MouseEvent me (source, relativePos, source.getCurrentModifiers(), MouseInputSource::invalidPressure,
                                 MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                                 MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                                 this, this, time, relativePos, time, 0, false);
            mouseMove (me);

            if (checker.shouldBailOut())
                return;

            desktop.getMouseListeners().callChecked (checker, [&] (MouseListener& l) { l.mouseMove (me); });

            MouseListenerList::template sendMouseEvent<const MouseEvent&> (*this, checker, &MouseListener::mouseMove, me);
        }
        */
    }
    
    pub fn internal_mouse_wheel(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time,
        wheel:        &MouseWheelDetails)  {
        
        todo!();
        /*
            auto& desktop = Desktop::getInstance();
        ComponentBailOutChecker checker (this);

        const MouseEvent me (source, relativePos, source.getCurrentModifiers(), MouseInputSource::invalidPressure,
                             MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                             MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                             this, this, time, relativePos, time, 0, false);

        if (isCurrentlyBlockedByAnotherModalComponent())
        {
            // allow blocked mouse-events to go to global listeners..
            desktop.mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseWheelMove (me, wheel); });
        }
        else
        {
            mouseWheelMove (me, wheel);

            if (checker.shouldBailOut())
                return;

            desktop.mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseWheelMove (me, wheel); });

            if (! checker.shouldBailOut())
                MouseListenerList::template sendMouseEvent<const MouseEvent&, const MouseWheelDetails&> (*this, checker, &MouseListener::mouseWheelMove, me, wheel);
        }
        */
    }
    
    pub fn internal_magnify_gesture(&mut self, 
        source:       MouseInputSource,
        relative_pos: Point<f32>,
        time:         Time,
        amount:       f32)  {
        
        todo!();
        /*
            auto& desktop = Desktop::getInstance();
        ComponentBailOutChecker checker (this);

        const MouseEvent me (source, relativePos, source.getCurrentModifiers(), MouseInputSource::invalidPressure,
                             MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                             MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                             this, this, time, relativePos, time, 0, false);

        if (isCurrentlyBlockedByAnotherModalComponent())
        {
            // allow blocked mouse-events to go to global listeners..
            desktop.mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseMagnify (me, amount); });
        }
        else
        {
            mouseMagnify (me, amount);

            if (checker.shouldBailOut())
                return;

            desktop.mouseListeners.callChecked (checker, [&] (MouseListener& l) { l.mouseMagnify (me, amount); });

            if (! checker.shouldBailOut())
                MouseListenerList::template sendMouseEvent<const MouseEvent&, float> (*this, checker, &MouseListener::mouseMagnify, me, amount);
        }
        */
    }
    
    pub fn send_fake_mouse_move(&self)  {
        
        todo!();
        /*
            if (flags.ignoresMouseClicksFlag && ! flags.allowChildMouseClicksFlag)
            return;

        auto mainMouse = Desktop::getInstance().getMainMouseSource();

        if (! mainMouse.isDragging())
            mainMouse.triggerFakeMove();
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
      | time you begin a drag event. Passing
      | an interval of 0 or less will cancel the
      | auto-repeat.
      | 
      | @see mouseDrag, Desktop::beginDragAutoRepeat
      |
      */
    pub fn begin_drag_auto_repeat(&mut self, interval: i32)  {
        
        todo!();
        /*
            Desktop::getInstance().beginDragAutoRepeat (interval);
        */
    }
    
    pub fn brought_to_front(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn internal_brought_to_front(&mut self)  {
        
        todo!();
        /*
            if (flags.hasHeavyweightPeerFlag)
            Desktop::getInstance().componentBroughtToFront (this);

        ComponentBailOutChecker checker (this);
        broughtToFront();

        if (checker.shouldBailOut())
            return;

        componentListeners.callChecked (checker, [this] (ComponentListener& l) { l.componentBroughtToFront (*this); });

        if (checker.shouldBailOut())
            return;

        // When brought to the front and there's a modal component blocking this one,
        // we need to bring the modal one to the front instead..
        if (auto* cm = getCurrentlyModalComponent())
            if (cm->getTopLevelComponent() != getTopLevelComponent())
                ModalComponentManager::getInstance()->bringModalComponentsToFront (false); // very important that this is false, otherwise in Windows,
                                                                                           // non-front components can't get focus when another modal comp is
                                                                                           // active, and therefore can't receive mouse-clicks
        */
    }
    
    pub fn focus_gained(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn focus_of_child_component_changed(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn internal_keyboard_focus_gain(&mut self, cause: FocusChangeType)  {
        
        todo!();
        /*
            internalKeyboardFocusGain (cause, WeakReference<Component> (this));
        */
    }
    
    pub fn internal_keyboard_focus_gain_with_maybe_component(&mut self, 
        cause:        FocusChangeType,
        safe_pointer: &WeakReference<Component<'a>>)  {
        
        todo!();
        /*
            focusGained (cause);

        if (safePointer != nullptr)
        {
            if (auto* handler = getAccessibilityHandler())
                handler->grabFocus();

            internalChildKeyboardFocusChange (cause, safePointer);
        }
        */
    }
    
    pub fn internal_keyboard_focus_loss(&mut self, cause: FocusChangeType)  {
        
        todo!();
        /*
            const WeakReference<Component> safePointer (this);

        focusLost (cause);

        if (safePointer != nullptr)
        {
            if (auto* handler = getAccessibilityHandler())
                handler->giveAwayFocus();

            internalChildKeyboardFocusChange (cause, safePointer);
        }
        */
    }
    
    pub fn internal_child_keyboard_focus_change(&mut self, 
        cause:        FocusChangeType,
        safe_pointer: &WeakReference<Component<'a>>)  {
        
        todo!();
        /*
            const bool childIsNowKeyboardFocused = hasKeyboardFocus (true);

        if (flags.childKeyboardFocusedFlag != childIsNowKeyboardFocused)
        {
            flags.childKeyboardFocusedFlag = childIsNowKeyboardFocused;

            focusOfChildComponentChanged (cause);

            if (safePointer == nullptr)
                return;
        }

        if (parentComponent != nullptr)
            parentComponent->internalChildKeyboardFocusChange (cause, parentComponent);
        */
    }
    
    /**
      | Sets a flag to indicate whether this
      | component wants keyboard focus or not.
      | 
      | By default components aren't actually
      | interested in gaining the keyboard
      | focus, but this method can be used to
      | turn this on.
      | 
      | See the grabKeyboardFocus() method
      | for details about the way a component
      | is chosen to receive the focus.
      | 
      | @see grabKeyboardFocus, giveAwayKeyboardFocus,
      | getWantsKeyboardFocus
      |
      */
    pub fn set_wants_keyboard_focus(&mut self, wants_focus: bool)  {
        
        todo!();
        /*
            flags.wantsKeyboardFocusFlag = wantsFocus;
        */
    }
    
    /**
      | Chooses whether a click on this component
      | automatically grabs the focus.
      | 
      | By default this is set to true, but you
      | might want a component which can be focused,
      | but where you don't want the user to be
      | able to affect it directly by clicking.
      |
      */
    pub fn set_mouse_click_grabs_keyboard_focus(&mut self, should_grab_focus: bool)  {
        
        todo!();
        /*
            flags.dontFocusOnMouseClickFlag = ! shouldGrabFocus;
        */
    }
    
    /**
      | Returns the last value set with setMouseClickGrabsKeyboardFocus().
      | 
      | @see setMouseClickGrabsKeyboardFocus
      |
      */
    pub fn get_mouse_click_grabs_keyboard_focus(&self) -> bool {
        
        todo!();
        /*
            return ! flags.dontFocusOnMouseClickFlag;
        */
    }
    
    /**
      | Returns true if the component is interested
      | in getting keyboard focus.
      | 
      | This returns the flag set by setWantsKeyboardFocus().
      | The default setting is false.
      | 
      | @see setWantsKeyboardFocus
      |
      */
    pub fn get_wants_keyboard_focus(&self) -> bool {
        
        todo!();
        /*
            return flags.wantsKeyboardFocusFlag && ! flags.isDisabledFlag;
        */
    }
    
    /**
      | Sets whether this component is a container
      | for components that can have their focus
      | traversed, and the type of focus traversal
      | that it supports.
      | 
      | @see FocusContainerType, isFocusContainer,
      | isKeyboardFocusContainer, FocusTraverser,
      | createFocusTraverser, KeyboardFocusTraverser,
      | createKeyboardFocusTraverser
      |
      */
    pub fn set_focus_container_type(&mut self, container_type: FocusContainerType)  {
        
        todo!();
        /*
            flags.isFocusContainerFlag = (containerType == FocusContainerType::focusContainer
                                      || containerType == FocusContainerType::keyboardFocusContainer);

        flags.isKeyboardFocusContainerFlag = (containerType == FocusContainerType::keyboardFocusContainer);
        */
    }
    
    /**
      | Returns true if this component has been
      | marked as a focus container.
      | 
      | @see setFocusContainer
      |
      */
    pub fn is_focus_container(&self) -> bool {
        
        todo!();
        /*
            return flags.isFocusContainerFlag;
        */
    }
    
    /**
      | Returns true if this component has been
      | marked as a keyboard focus container.
      | 
      | @see setFocusContainer
      |
      */
    pub fn is_keyboard_focus_container(&self) -> bool {
        
        todo!();
        /*
            return flags.isKeyboardFocusContainerFlag;
        */
    }
    
    /**
      | Returns the focus container for this
      | component.
      | 
      | @see isFocusContainer, setFocusContainer
      |
      */
    pub fn find_focus_container(&self) -> *mut Component {
        
        todo!();
        /*
            return findContainer (this, &Component::isFocusContainer);
        */
    }
    
    /**
      | Returns the keyboard focus container
      | for this component.
      | 
      | @see isFocusContainer, setFocusContainer
      |
      */
    pub fn find_keyboard_focus_container(&self) -> *mut Component {
        
        todo!();
        /*
            return findContainer (this, &Component::isKeyboardFocusContainer);
        */
    }
    
    /**
      | Returns the focus order of this component,
      | if one has been specified.
      | 
      | By default components don't have a focus
      | order - in that case, this will return
      | 0.
      | 
      | @see setExplicitFocusOrder
      |
      */
    pub fn get_explicit_focus_order(&self) -> i32 {
        
        todo!();
        /*
            return properties [aloe_explicitFocusOrderId];
        */
    }
    
    /**
      | Sets the focus order of this component.
      | 
      | The focus order is used by the default
      | traverser implementation returned
      | by createFocusTraverser() as part
      | of its algorithm for deciding the order
      | in which components should be traversed.
      | A value of 0 or less is taken to mean that
      | no explicit order is wanted, and that
      | traversal should use other factors,
      | like the component's position.
      | 
      | @see getExplicitFocusOrder, FocusTraverser,
      | createFocusTraverser
      |
      */
    pub fn set_explicit_focus_order(&mut self, new_focus_order_index: i32)  {
        
        todo!();
        /*
            properties.set (aloe_explicitFocusOrderId, newFocusOrderIndex);
        */
    }
    
    pub fn create_focus_traverser(&mut self) -> Box<dyn ComponentTraverser> {
        
        todo!();
        /*
            if (flags.isFocusContainerFlag || parentComponent == nullptr)
            return std::make_unique<FocusTraverser>();

        return parentComponent->createFocusTraverser();
        */
    }
    
    pub fn create_keyboard_focus_traverser(&mut self) -> Box<dyn ComponentTraverser> {
        
        todo!();
        /*
            if (flags.isKeyboardFocusContainerFlag || parentComponent == nullptr)
            return std::make_unique<KeyboardFocusTraverser>();

        return parentComponent->createKeyboardFocusTraverser();
        */
    }
    
    pub fn take_keyboard_focus(&mut self, cause: FocusChangeType)  {
        
        todo!();
        /*
            if (currentlyFocusedComponent == this)
            return;

        if (auto* peer = getPeer())
        {
            const WeakReference<Component> safePointer (this);
            peer->grabFocus();

            if (! peer->isFocused() || currentlyFocusedComponent == this)
                return;

            WeakReference<Component> componentLosingFocus (currentlyFocusedComponent);
            currentlyFocusedComponent = this;

            Desktop::getInstance().triggerFocusCallback();

            // call this after setting currentlyFocusedComponent so that the one that's
            // losing it has a chance to see where focus is going
            if (componentLosingFocus != nullptr)
                componentLosingFocus->internalKeyboardFocusLoss (cause);

            if (currentlyFocusedComponent == this)
                internalKeyboardFocusGain (cause, safePointer);
        }
        */
    }
    
    pub fn grab_keyboard_focus_internal(&mut self, 
        cause:          FocusChangeType,
        can_try_parent: bool)  {
        
        todo!();
        /*
            if (! isShowing())
            return;

        if (flags.wantsKeyboardFocusFlag
            && (isEnabled() || parentComponent == nullptr))
        {
            takeKeyboardFocus (cause);
            return;
        }

        if (isParentOf (currentlyFocusedComponent) && currentlyFocusedComponent->isShowing())
            return;

        if (auto traverser = createKeyboardFocusTraverser())
        {
            if (auto* defaultComp = traverser->getDefaultComponent (this))
            {
                defaultComp->grabKeyboardFocusInternal (cause, false);
                return;
            }
        }

        // if no children want it and we're allowed to try our parent comp,
        // then pass up to parent, which will try our siblings.
        if (canTryParent && parentComponent != nullptr)
            parentComponent->grabKeyboardFocusInternal (cause, true);
        */
    }
    
    /**
      | Tries to give keyboard focus to this
      | component.
      | 
      | When the user clicks on a component or
      | its grabKeyboardFocus() method is
      | called, the following procedure is
      | used to work out which component should
      | get it:
      | 
      | - if the component that was clicked on
      | actually wants focus (as indicated
      | by calling getWantsKeyboardFocus),
      | it gets it.
      | 
      | - if the component itself doesn't want
      | focus, it will try to pass it on to whichever
      | of its children is the default component,
      | as determined by the getDefaultComponent()
      | implemetation of the ComponentTraverser
      | returned by createKeyboardFocusTraverser().
      | 
      | - if none of its children want focus at
      | all, it will pass it up to its parent instead,
      | unless it's a top-level component without
      | a parent, in which case it just takes
      | the focus itself.
      | 
      | Important note! It's obviously not
      | possible for a component to be focused
      | unless it's actually visible, on-screen,
      | and inside a window that is also visible.
      | So there's no point trying to call this
      | in the component's own constructor
      | or before all of its parent hierarchy
      | has been fully instantiated.
      | 
      | @see giveAwayKeyboardFocus, setWantsKeyboardFocus,
      | getWantsKeyboardFocus, hasKeyboardFocus,
      | getCurrentlyFocusedComponent, focusGained,
      | focusLost, keyPressed, keyStateChanged
      |
      */
    pub fn grab_keyboard_focus(&mut self)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        grabKeyboardFocusInternal (focusChangedDirectly, true);

        // A component can only be focused when it's actually on the screen!
        // If this fails then you're probably trying to grab the focus before you've
        // added the component to a parent or made it visible. Or maybe one of its parent
        // components isn't yet visible.
        jassert (isShowing() || isOnDesktop());
        */
    }
    
    pub fn give_away_keyboard_focus_internal(&mut self, send_focus_loss_event: bool)  {
        
        todo!();
        /*
            if (hasKeyboardFocus (true))
        {
            if (auto* componentLosingFocus = currentlyFocusedComponent)
            {
                currentlyFocusedComponent = nullptr;

                if (sendFocusLossEvent && componentLosingFocus != nullptr)
                    componentLosingFocus->internalKeyboardFocusLoss (focusChangedDirectly);

                Desktop::getInstance().triggerFocusCallback();
            }
        }
        */
    }
    
    /**
      | If this component or any of its children
      | currently have the keyboard focus,
      | this will defocus it, send a focus change
      | notification, and try to pass the focus
      | to the next component.
      | 
      | @see grabKeyboardFocus, setWantsKeyboardFocus,
      | getCurrentlyFocusedComponent, focusGained,
      | focusLost
      |
      */
    pub fn give_away_keyboard_focus(&mut self)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        giveAwayKeyboardFocusInternal (true);
        */
    }
    
    /**
      | Tries to move the keyboard focus to one
      | of this component's siblings.
      | 
      | This will try to move focus to either
      | the next or previous component, as determined
      | by the getNextComponent() and getPreviousComponent()
      | implemetations of the ComponentTraverser
      | returned by createKeyboardFocusTraverser().
      | 
      | This is the method that is used when shifting
      | focus by pressing the tab key.
      | 
      | -----------
      | @param moveToNext
      | 
      | if true, the focus will move forwards;
      | if false, it will move backwards @see
      | grabKeyboardFocus, giveAwayKeyboardFocus,
      | setFocusContainer, setWantsKeyboardFocus
      |
      */
    pub fn move_keyboard_focus_to_sibling(&mut self, move_to_next: bool)  {
        
        todo!();
        /*
            // if component methods are being called from threads other than the message
        // thread, you'll need to use a MessageManagerLock object to make sure it's thread-safe.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (parentComponent != nullptr)
        {
            if (auto traverser = createKeyboardFocusTraverser())
            {
                auto findComponentToFocus = [&]() -> Component*
                {
                    if (auto* comp = (moveToNext ? traverser->getNextComponent (this)
                                                 : traverser->getPreviousComponent (this)))
                        return comp;

                    if (auto* focusContainer = findKeyboardFocusContainer())
                    {
                        auto allFocusableComponents = traverser->getAllComponents (focusContainer);

                        if (! allFocusableComponents.empty())
                            return moveToNext ? allFocusableComponents.front()
                                              : allFocusableComponents.back();
                    }

                    return nullptr;
                };

                if (auto* nextComp = findComponentToFocus())
                {
                    if (nextComp->isCurrentlyBlockedByAnotherModalComponent())
                    {
                        const WeakReference<Component> nextCompPointer (nextComp);
                        internalModalInputAttempt();

                        if (nextCompPointer == nullptr || nextComp->isCurrentlyBlockedByAnotherModalComponent())
                            return;
                    }

                    nextComp->grabKeyboardFocusInternal (focusChangedByTabKey, true);
                    return;
                }
            }

            parentComponent->moveKeyboardFocusToSibling (moveToNext);
        }
        */
    }
    
    /**
      | Returns true if this component currently
      | has the keyboard focus.
      | 
      | -----------
      | @param trueIfChildIsFocused
      | 
      | if this is true, then the method returns
      | true if either this component or any
      | of its children (recursively) have
      | the focus. If false, the method only
      | returns true if this component has the
      | focus.
      | 
      | @see grabKeyboardFocus, giveAwayKeyboardFocus,
      | setWantsKeyboardFocus, getCurrentlyFocusedComponent,
      | focusGained, focusLost
      |
      */
    pub fn has_keyboard_focus(&self, true_if_child_is_focused: bool) -> bool {
        
        todo!();
        /*
            return (currentlyFocusedComponent == this)
                || (trueIfChildIsFocused && isParentOf (currentlyFocusedComponent));
        */
    }
    
    /**
      | Returns the component that currently
      | has the keyboard focus.
      | 
      | 
      | -----------
      | @return
      | 
      | the focused component, or nullptr if
      | nothing is focused.
      |
      */
    pub fn get_currently_focused_component(&mut self) -> *mut Component {
        
        todo!();
        /*
            return currentlyFocusedComponent;
        */
    }
    
    /**
      | If any component has keyboard focus,
      | this will defocus it.
      |
      */
    pub fn unfocus_all_components(&mut self)  {
        
        todo!();
        /*
            if (currentlyFocusedComponent != nullptr)
            currentlyFocusedComponent->giveAwayKeyboardFocus();
        */
    }
    
    /**
      | Returns true if the component (and all
      | its parents) are enabled.
      | 
      | Components are enabled by default,
      | and can be disabled with setEnabled().
      | Exactly what difference this makes
      | to the component depends on the type.
      | E.g. buttons and sliders will choose
      | to draw themselves differently, etc.
      | 
      | -----------
      | @note
      | 
      | if one of this component's parents is
      | disabled, this will always return false,
      | even if this component itself is enabled.
      | 
      | @see setEnabled, enablementChanged
      |
      */
    pub fn is_enabled(&self) -> bool {
        
        todo!();
        /*
            return (! flags.isDisabledFlag)
                && (parentComponent == nullptr || parentComponent->isEnabled());
        */
    }
    
    /**
      | Enables or disables this component.
      | 
      | Disabling a component will also cause
      | all of its child components to become
      | disabled.
      | 
      | Similarly, enabling a component which
      | is inside a disabled parent component
      | won't make any difference until the
      | parent is re-enabled.
      | 
      | @see isEnabled, enablementChanged
      |
      */
    pub fn set_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            if (flags.isDisabledFlag == shouldBeEnabled)
        {
            flags.isDisabledFlag = ! shouldBeEnabled;

            // if any parent components are disabled, setting our flag won't make a difference,
            // so no need to send a change message
            if (parentComponent == nullptr || parentComponent->isEnabled())
                sendEnablementChangeMessage();

            ComponentBailOutChecker checker (this);
            componentListeners.callChecked (checker, [this] (ComponentListener& l) { l.componentEnablementChanged (*this); });
        }
        */
    }
    
    pub fn enablement_changed(&mut self)  { }
    
    pub fn send_enablement_change_message(&mut self)  {
        
        todo!();
        /*
            const WeakReference<Component> safePointer (this);

        enablementChanged();

        if (safePointer == nullptr)
            return;

        for (int i = getNumChildComponents(); --i >= 0;)
        {
            if (auto* c = getChildComponent (i))
            {
                c->sendEnablementChangeMessage();

                if (safePointer == nullptr)
                    return;
            }
        }
        */
    }
    
    /**
      | Returns true if the mouse is currently
      | over this component.
      | 
      | If the mouse isn't over the component,
      | this will return false, even if the mouse
      | is currently being dragged - so you can
      | use this in your mouseDrag method to
      | find out whether it's really over the
      | component or not.
      | 
      | -----------
      | @note
      | 
      | when the mouse button is being held down,
      | then the only component for which this
      | method will return true is the one that
      | was originally clicked on.
      | 
      | Also note that on a touch-screen device,
      | this will only return true when a finger
      | is actually down - as soon as all touch
      | is released, isMouseOver will always
      | return false.
      | 
      | If includeChildren is true, then this
      | will also return true if the mouse is
      | over any of the component's children
      | (recursively) as well as the component
      | itself.
      | 
      | @see isMouseButtonDown. isMouseOverOrDragging,
      | mouseDrag
      |
      */
    pub fn is_mouse_over(&self, include_children: Option<bool>) -> bool {

        let include_children: bool = include_children.unwrap_or(false);
        
        todo!();
        /*
            if (! MessageManager::getInstance()->isThisTheMessageThread())
            return flags.cachedMouseInsideComponent;

        for (auto& ms : Desktop::getInstance().getMouseSources())
        {
            auto* c = ms.getComponentUnderMouse();

            if (c != nullptr && (c == this || (includeChildren && isParentOf (c))))
                if (ms.isDragging() || ! (ms.isTouch() || ms.isPen()))
                    if (c->reallyContainsInternal (c->getLocalPoint (nullptr, ms.getScreenPosition()), false))
                        return true;
        }

        return false;
        */
    }
    
    /**
      | Returns true if the mouse button is currently
      | held down in this component.
      | 
      | -----------
      | @note
      | 
      | this is a test to see whether the mouse
      | is being pressed in this component,
      | so it'll return false if called on component
      | A when the mouse is actually being dragged
      | in component B.
      | 
      | @see isMouseButtonDownAnywhere,
      | isMouseOver, isMouseOverOrDragging
      |
      */
    pub fn is_mouse_button_down(&self, include_children: Option<bool>) -> bool {

        let include_children: bool = include_children.unwrap_or(false);
        
        todo!();
        /*
            for (auto& ms : Desktop::getInstance().getMouseSources())
        {
            auto* c = ms.getComponentUnderMouse();

            if (c == this || (includeChildren && isParentOf (c)))
                if (ms.isDragging())
                    return true;
        }

        return false;
        */
    }
    
    /**
      | True if the mouse is over this component,
      | or if it's being dragged in this component.
      | This is a handy equivalent to (isMouseOver()
      | || isMouseButtonDown()). @see isMouseOver,
      | isMouseButtonDown, isMouseButtonDownAnywhere
      |
      */
    pub fn is_mouse_over_or_dragging(&self, include_children: Option<bool>) -> bool {

        let include_children: bool = include_children.unwrap_or(false);
        
        todo!();
        /*
            for (auto& ms : Desktop::getInstance().getMouseSources())
        {
            auto* c = ms.getComponentUnderMouse();

            if (c == this || (includeChildren && isParentOf (c)))
                if (ms.isDragging() || ! ms.isTouch())
                    return true;
        }

        return false;
        */
    }
    
    /**
      | Returns true if a mouse button is currently
      | down.
      | 
      | Unlike isMouseButtonDown, this will
      | test the current state of the buttons
      | without regard to which component (if
      | any) it has been pressed in.
      | 
      | @see isMouseButtonDown, ModifierKeys
      |
      */
    pub fn is_mouse_button_down_anywhere(&mut self) -> bool {
        
        todo!();
        /*
            return ModifierKeys::currentModifiers.isAnyMouseButtonDown();
        */
    }
    
    /**
      | Returns the mouse's current position,
      | relative to this component. The return
      | value is relative to the component's
      | top-left corner.
      |
      */
    pub fn get_mouse_xy_relative(&self) -> Point<i32> {
        
        todo!();
        /*
            return getLocalPoint (nullptr, Desktop::getMousePosition());
        */
    }
    
    /**
      | Adds a listener that wants to hear about
      | keypresses that this component receives.
      | 
      | The listeners that are registered with
      | a component are called by its keyPressed()
      | or keyStateChanged() methods (assuming
      | these haven't been overridden to do
      | something else).
      | 
      | If you add an object as a key listener,
      | be careful to remove it when the object
      | is deleted, or the component will be
      | left with a dangling pointer.
      | 
      | @see keyPressed, keyStateChanged,
      | removeKeyListener
      |
      */
    pub fn add_key_listener(&mut self, new_listener: *mut dyn KeyListener)  {
        
        todo!();
        /*
            if (keyListeners == nullptr)
            keyListeners.reset (new Vec<KeyListener*>());

        keyListeners->addIfNotAlreadyThere (newListener);
        */
    }
    
    /**
      | Removes a previously-registered key
      | listener. @see addKeyListener
      |
      */
    pub fn remove_key_listener(&mut self, listener_to_remove: *mut dyn KeyListener)  {
        
        todo!();
        /*
            if (keyListeners != nullptr)
            keyListeners->removeFirstMatchingValue (listenerToRemove);
        */
    }
    
    pub fn key_pressed(&mut self, _0: &KeyPress) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn key_state_changed(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn modifier_keys_changed(&mut self, modifiers: &ModifierKeys)  {
        
        todo!();
        /*
            if (parentComponent != nullptr)
            parentComponent->modifierKeysChanged (modifiers);
        */
    }
    
    pub fn internal_modifier_keys_changed(&mut self)  {
        
        todo!();
        /*
            sendFakeMouseMove();
        modifierKeysChanged (ModifierKeys::currentModifiers);
        */
    }
    
    /**
      | Sets the title for this component.
      | 
      | If this component supports accessibility
      | using the default AccessibilityHandler
      | implementation, this string will be
      | passed to accessibility clients requesting
      | a title and may be read out by a screen
      | reader.
      | 
      | @see getTitle, getAccessibilityHandler
      |
      */
    pub fn set_title(&mut self, new_title: &String)  {
        
        todo!();
        /*
            componentTitle = newTitle;
        */
    }
    
    /**
      | Sets the description for this component.
      | 
      | If this component supports accessibility
      | using the default AccessibilityHandler
      | implementation, this string will be
      | passed to accessibility clients requesting
      | a description and may be read out by a
      | screen reader.
      | 
      | @see getDescription, getAccessibilityHandler
      |
      */
    pub fn set_description(&mut self, new_description: &String)  {
        
        todo!();
        /*
            componentDescription = newDescription;
        */
    }
    
    /**
      | Sets the help text for this component.
      | 
      | If this component supports accessibility
      | using the default AccessibilityHandler
      | implementation, this string will be
      | passed to accessibility clients requesting
      | help text and may be read out by a screen
      | reader.
      | 
      | @see getHelpText, getAccessibilityHandler
      |
      */
    pub fn set_help_text(&mut self, new_help_text: &String)  {
        
        todo!();
        /*
            componentHelpText = newHelpText;
        */
    }
    
    /**
      | Sets whether this component and its
      | children are visible to accessibility
      | clients.
      | 
      | If this flag is set to false then the getAccessibilityHandler()
      | method will return nullptr and this
      | component and its children will not
      | be visible to any accessibility clients.
      | 
      | By default this is set to true.
      | 
      | @see isAccessible, getAccessibilityHandler
      |
      */
    pub fn set_accessible(&mut self, should_be_accessible: bool)  {
        
        todo!();
        /*
            flags.accessibilityIgnoredFlag = ! shouldBeAccessible;

        if (flags.accessibilityIgnoredFlag)
            invalidateAccessibilityHandler();
        */
    }
    
    /**
      | Returns true if this component and its
      | children are visible to accessibility
      | clients.
      | 
      | @see setAccessible
      |
      */
    pub fn is_accessible(&self) -> bool {
        
        todo!();
        /*
            return (! flags.accessibilityIgnoredFlag
                && (parentComponent == nullptr || parentComponent->isAccessible()));
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::unspecified);
        */
    }
    
    /**
      | @internal
      |
      */
    pub fn create_ignored_accessibility_handler(&mut self, comp: &mut Component<'a>) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (comp, AccessibilityRole::ignored);
        */
    }
    
    /**
      | Invalidates the AccessibilityHandler
      | that is currently being used for this
      | component.
      | 
      | Use this to indicate that something
      | in the accessible component has changed
      | and its handler needs to be updated.
      | This will trigger a call to createAccessibilityHandler().
      |
      */
    pub fn invalidate_accessibility_handler(&mut self)  {
        
        todo!();
        /*
            accessibilityHandler = nullptr;
        */
    }
    
    /**
      | Returns the accessibility handler
      | for this component, or nullptr if this
      | component is not accessible.
      | 
      | @see setAccessible
      |
      */
    pub fn get_accessibility_handler(&mut self) -> *mut AccessibilityHandler {
        
        todo!();
        /*
            if (! isAccessible() || getWindowHandle() == nullptr)
            return nullptr;

        if (accessibilityHandler == nullptr
            || accessibilityHandler->getTypeIndex() != std::type_index (typeid (*this)))
        {
            accessibilityHandler = createAccessibilityHandler();
        }

        return accessibilityHandler.get();
        */
    }
}
