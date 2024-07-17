crate::ix!();

pub trait GetBorderThickness {

    /**
      | Returns the width of the frame to use
      | around the window. @see getContentComponentBorder
      |
      */
    fn get_border_thickness(&mut self) -> BorderSize<i32>;
}

pub trait GetContentComponentBorder {

    /**
      | Returns the insets to use when positioning
      | the content component. @see getBorderThickness
      |
      */
    fn get_content_component_border(&mut self) -> BorderSize<i32>;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_ResizableWindow.h]

/**
  | A base class for top-level windows that
  | can be dragged around and resized.
  | 
  | To add content to the window, use its
  | setContentOwned() or setContentNonOwned()
  | methods to give it a component that will
  | remain positioned inside it (leaving
  | a gap around the edges for a border).
  | 
  | It's not advisable to add child components
  | directly to a ResizableWindow: put
  | them inside your content component
  | instead. And overriding methods like
  | resized(), moved(), etc is also not
  | recommended - instead override these
  | methods for your content component.
  | (If for some obscure reason you do need
  | to override these methods, always remember
  | to call the super-class's resized()
  | method too, otherwise it'll fail to
  | lay out the window decorations correctly).
  | 
  | By default resizing isn't enabled -
  | use the setResizable() method to enable
  | it and to choose the style of resizing
  | to use.
  | 
  | @see TopLevelWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ResizableWindow<'a> {
    base:                     TopLevelWindow<'a>,
    resizable_corner:         Box<ResizableCornerComponent<'a>>,
    resizable_border:         Box<ResizableBorderComponent<'a>>,
    content_component:        ComponentSafePointer<'a,Component<'a>>,
    splash_screen:            ComponentSafePointer<'a,Component<'a>>,
    owns_content_component:   bool, // default = false
    resize_to_fit_content:    bool, // default = false
    fullscreen:               bool, // default = false
    can_drag:                 bool, // default = true
    drag_started:             bool, // default = false
    dragger:                  ComponentDragger,
    last_non_full_screen_pos: Rectangle<i32>,
    default_constrainer:      ComponentBoundsConstrainer,
    constrainer:              Option<&'a mut ComponentBoundsConstrainer>,

    #[cfg(ALOE_DEBUG)]
    has_been_resized:         bool, // default = false
}

pub trait ResizableWindowInterface: GetBorderThickness + GetContentComponentBorder {}

impl<'a> Drop for ResizableWindow<'a> {

    /**
      | Destructor.
      | 
      | If a content component has been set with
      | setContentOwned(), it will be deleted.
      |
      */
    fn drop(&mut self) {

        todo!();

        /*
            splashScreen.deleteAndZero();

        // Don't delete or remove the resizer components yourself! They're managed by the
        // ResizableWindow, and you should leave them alone! You may have deleted them
        // accidentally by careless use of deleteAllChildren()..?
        jassert (resizableCorner == nullptr || getIndexOfChildComponent (resizableCorner.get()) >= 0);
        jassert (resizableBorder == nullptr || getIndexOfChildComponent (resizableBorder.get()) >= 0);

        resizableCorner.reset();
        resizableBorder.reset();
        clearContentComponent();

        // have you been adding your own components directly to this window..? tut tut tut.
        // Read the instructions for using a ResizableWindow!
        jassert (getNumChildComponents() == 0);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_ResizableWindow.cpp]
impl<'a> ResizableWindow<'a> {

    /**
      | Returns true if the window can be dragged
      | around by the user.
      |
      */
    pub fn is_draggable(&self) -> bool {
        
        todo!();
        /*
            return canDrag;
        */
    }

    /**
      | Returns the bounds constrainer object
      | that this window is using.
      | 
      | You can access this to change its properties.
      |
      */
    pub fn get_constrainer(&mut self) -> *mut ComponentBoundsConstrainer {
        
        todo!();
        /*
            return constrainer;
        */
    }

    /**
      | Returns the current content component.
      | 
      | This will be the component set by setContentOwned()
      | or setContentNonOwned, or nullptr
      | if none has yet been specified.
      | 
      | @see setContentOwned, setContentNonOwned
      |
      */
    pub fn get_content_component(&self) -> *mut Component {
        
        todo!();
        /*
            return contentComponent;
        */
    }

    /**
      | Creates a ResizableWindow.
      | 
      | This constructor doesn't specify a
      | background colour, so the LookAndFeel's
      | default background colour will be used.
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
        : top_level_window(name, shouldAddToDesktop),

            initialise (shouldAddToDesktop);
        */
    }
    
    /**
      | Creates a ResizableWindow.
      | 
      | -----------
      | @param name
      | 
      | the name to give the component
      | ----------
      | @param backgroundColour
      | 
      | the colour to use for filling the window's
      | background.
      | ----------
      | @param addToDesktop
      | 
      | if true, the window will be automatically
      | added to the desktop; if false, you can
      | use it as a child component
      |
      */
    pub fn new_with_background_colour(
        name:                  &String,
        bkgnd:                 Colour,
        should_add_to_desktop: bool) -> Self {
    
        todo!();
        /*
        : top_level_window(name, shouldAddToDesktop),

            setBackgroundColour (bkgnd);
        initialise (shouldAddToDesktop);
        */
    }
    
    pub fn initialise(&mut self, should_add_to_desktop: bool)  {
        
        todo!();
        /*
            /*
          ==========================================================================

           In accordance with the terms of the Aloe 6 End-Use License Agreement, the
           Aloe Code in SECTION A cannot be removed, changed or otherwise rendered
           ineffective unless you have a Aloe Indie or Pro license, or are using
           Aloe under the GPL v3 license.

           End User License Agreement: www.aloe.com/aloe-6-licence

          ==========================================================================
        */

        // BEGIN SECTION A

       #if ! AloePlugin_Build_Standalone
        splashScreen = new ALOESplashScreen (*this);
       #endif

        // END SECTION A

        defaultConstrainer.setMinimumOnscreenAmounts (0x10000, 16, 24, 16);

        lastNonFullScreenPos.setBounds (50, 50, 256, 256);

        if (shouldAddToDesktop)
            addToDesktop();
        */
    }
    
    pub fn get_desktop_window_style_flags(&self) -> i32 {
        
        todo!();
        /*
            int styleFlags = TopLevelWindow::getDesktopWindowStyleFlags();

        if (isResizable() && (styleFlags & ComponentPeer::windowHasTitleBar) != 0)
            styleFlags |= ComponentPeer::windowIsResizable;

        return styleFlags;
        */
    }
    
    /**
      | Removes the current content component.
      | 
      | If the previous content component was
      | added with setContentOwned(), it will
      | also be deleted. If it was added with
      | setContentNonOwned(), it will simply
      | be removed from this component.
      |
      */
    pub fn clear_content_component(&mut self)  {
        
        todo!();
        /*
            if (ownsContentComponent)
        {
            contentComponent.deleteAndZero();
        }
        else
        {
            removeChildComponent (contentComponent);
            contentComponent = nullptr;
        }
        */
    }
    
    pub fn set_content(&mut self, 
        new_content_component:                   *mut Component<'a>,
        take_ownership:                          bool,
        resize_to_fit_when_content_changes_size: bool)  {
        
        todo!();
        /*
            if (newContentComponent != contentComponent)
        {
            clearContentComponent();

            contentComponent = newContentComponent;
            Component::addAndMakeVisible (contentComponent);
        }

        ownsContentComponent = takeOwnership;
        resizeToFitContent = resizeToFitWhenContentChangesSize;

        if (resizeToFitWhenContentChangesSize)
            childBoundsChanged (contentComponent);

        resized(); // must always be called to position the new content comp
        */
    }
    
    /**
      | Changes the current content component.
      | 
      | This sets a component that will be placed
      | in the centre of the ResizableWindow,
      | (leaving a space around the edge for
      | the border).
      | 
      | You should never add components directly
      | to a ResizableWindow (or any of its subclasses)
      | with addChildComponent(). Instead,
      | add them to the content component.
      | 
      | -----------
      | @param newContentComponent
      | 
      | the new component to use - this component
      | will be deleted when it's no longer needed
      | (i.e. when the window is deleted or a
      | new content component is set for it).
      | To set a component that this window will
      | not delete, call setContentNonOwned()
      | instead.
      | ----------
      | @param resizeToFitWhenContentChangesSize
      | 
      | if true, then the ResizableWindow will
      | maintain its size such that it always
      | fits around the size of the content component.
      | If false, the new content will be resized
      | to fit the current space available.
      |
      */
    pub fn set_content_owned(&mut self, 
        new_content_component:                   *mut Component<'a>,
        resize_to_fit_when_content_changes_size: bool)  {
        
        todo!();
        /*
            setContent (newContentComponent, true, resizeToFitWhenContentChangesSize);
        */
    }
    
    /**
      | Changes the current content component.
      | 
      | This sets a component that will be placed
      | in the centre of the ResizableWindow,
      | (leaving a space around the edge for
      | the border).
      | 
      | You should never add components directly
      | to a ResizableWindow (or any of its subclasses)
      | with addChildComponent(). Instead,
      | add them to the content component.
      | 
      | -----------
      | @param newContentComponent
      | 
      | the new component to use - this component
      | will NOT be deleted by this component,
      | so it's the caller's responsibility
      | to manage its lifetime (it's ok to delete
      | it while this window is still using it).
      | To set a content component that the window
      | will delete, call setContentOwned()
      | instead.
      | ----------
      | @param resizeToFitWhenContentChangesSize
      | 
      | if true, then the ResizableWindow will
      | maintain its size such that it always
      | fits around the size of the content component.
      | If false, the new content will be resized
      | to fit the current space available.
      |
      */
    pub fn set_content_non_owned(&mut self, 
        new_content_component:                   *mut Component<'a>,
        resize_to_fit_when_content_changes_size: bool)  {
        
        todo!();
        /*
            setContent (newContentComponent, false, resizeToFitWhenContentChangesSize);
        */
    }
    
    pub fn set_content_component(&mut self, 
        new_content_component:                   *mut Component<'a>,
        delete_old_one:                          bool,
        resize_to_fit_when_content_changes_size: bool)  {
        
        todo!();
        /*
            if (newContentComponent != contentComponent)
        {
            if (deleteOldOne)
            {
                contentComponent.deleteAndZero();
            }
            else
            {
                removeChildComponent (contentComponent);
                contentComponent = nullptr;
            }
        }

        setContent (newContentComponent, true, resizeToFitWhenContentChangesSize);
        */
    }
    
    /**
      | Changes the window so that the content
      | component ends up with the specified
      | size.
      | 
      | This is basically a setSize call on the
      | window, but which adds on the borders,
      | so you can specify the content component's
      | target size.
      |
      */
    pub fn set_content_component_size(&mut self, 
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            jassert (width > 0 && height > 0); // not a great idea to give it a zero size..

        auto border = getContentComponentBorder();

        setSize (width + border.getLeftAndRight(),
                 height + border.getTopAndBottom());
        */
    }
    
    pub fn get_border_thickness(&mut self) -> BorderSize<i32> {
        
        todo!();
        /*
            if (isUsingNativeTitleBar() || isKioskMode())
            return {};

        return BorderSize<int> ((resizableBorder != nullptr && ! isFullScreen()) ? 4 : 1);
        */
    }
    
    pub fn get_content_component_border(&mut self) -> BorderSize<i32> {
        
        todo!();
        /*
            return getBorderThickness();
        */
    }
    
    /**
      | (if overriding this, make sure you call
      | ResizableWindow::moved() in your
      | subclass)
      |
      */
    pub fn moved(&mut self)  {
        
        todo!();
        /*
            updateLastPosIfShowing();
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            TopLevelWindow::visibilityChanged();
        updateLastPosIfShowing();
        */
    }
    
    /**
      | (if overriding this, make sure you call
      | ResizableWindow::resized() in your
      | subclass)
      |
      */
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            const bool resizerHidden = isFullScreen() || isKioskMode() || isUsingNativeTitleBar();

        if (resizableBorder != nullptr)
        {
            resizableBorder->setVisible (! resizerHidden);
            resizableBorder->setBorderThickness (getBorderThickness());
            resizableBorder->setSize (getWidth(), getHeight());
            resizableBorder->toBack();
        }

        if (resizableCorner != nullptr)
        {
            resizableCorner->setVisible (! resizerHidden);

            const int resizerSize = 18;
            resizableCorner->setBounds (getWidth() - resizerSize,
                                        getHeight() - resizerSize,
                                        resizerSize, resizerSize);
        }

        if (contentComponent != nullptr)
        {
            // The window expects to be able to be able to manage the size and position
            // of its content component, so you can't arbitrarily add a transform to it!
            jassert (! contentComponent->isTransformed());

            contentComponent->setBoundsInset (getContentComponentBorder());
        }

        updateLastPosIfShowing();

       #if ALOE_DEBUG
        hasBeenResized = true;
       #endif
        */
    }
    
    pub fn child_bounds_changed(&mut self, child: *mut Component<'a>)  {
        
        todo!();
        /*
            if ((child == contentComponent) && (child != nullptr) && resizeToFitContent)
        {
            // not going to look very good if this component has a zero size..
            jassert (child->getWidth() > 0);
            jassert (child->getHeight() > 0);

            auto borders = getContentComponentBorder();

            setSize (child->getWidth() + borders.getLeftAndRight(),
                     child->getHeight() + borders.getTopAndBottom());
        }
        */
    }
    
    pub fn active_window_status_changed(&mut self)  {
        
        todo!();
        /*
            auto border = getContentComponentBorder();
        auto area = getLocalBounds();

        repaint (area.removeFromTop (border.getTop()));
        repaint (area.removeFromLeft (border.getLeft()));
        repaint (area.removeFromRight (border.getRight()));
        repaint (area.removeFromBottom (border.getBottom()));
        */
    }
    
    /**
      | Make the window resizable or fixed.
      | 
      | -----------
      | @param shouldBeResizable
      | 
      | whether it's resizable at all
      | ----------
      | @param useBottomRightCornerResizer
      | 
      | if true, it'll add a ResizableCornerComponent
      | at the bottom-right; if false, it'll
      | use a ResizableBorderComponent around
      | the edge @see setResizeLimits, isResizable
      |
      */
    pub fn set_resizable(&mut self, 
        should_be_resizable:             bool,
        use_bottom_right_corner_resizer: bool)  {
        
        todo!();
        /*
            if (shouldBeResizable)
        {
            if (useBottomRightCornerResizer)
            {
                resizableBorder.reset();

                if (resizableCorner == nullptr)
                {
                    resizableCorner.reset (new ResizableCornerComponent (this, constrainer));
                    Component::addChildComponent (resizableCorner.get());
                    resizableCorner->setAlwaysOnTop (true);
                }
            }
            else
            {
                resizableCorner.reset();

                if (resizableBorder == nullptr)
                {
                    resizableBorder.reset (new ResizableBorderComponent (this, constrainer));
                    Component::addChildComponent (resizableBorder.get());
                }
            }
        }
        else
        {
            resizableCorner.reset();
            resizableBorder.reset();
        }

        if (isUsingNativeTitleBar())
            recreateDesktopWindow();

        childBoundsChanged (contentComponent);
        resized();
        */
    }
    
    /**
      | Returns true if resizing is enabled.
      | @see setResizable
      |
      */
    pub fn is_resizable(&self) -> bool {
        
        todo!();
        /*
            return resizableCorner != nullptr
            || resizableBorder != nullptr;
        */
    }
    
    /**
      | This sets the maximum and minimum sizes
      | for the window.
      | 
      | If the window's current size is outside
      | these limits, it will be resized to make
      | sure it's within them.
      | 
      | A direct call to setBounds() will bypass
      | any constraint checks, but when the
      | window is dragged by the user or resized
      | by other indirect means, the constrainer
      | will limit the numbers involved.
      | 
      | @see setResizable, setFixedAspectRatio
      |
      */
    pub fn set_resize_limits(&mut self, 
        new_minimum_width:  i32,
        new_minimum_height: i32,
        new_maximum_width:  i32,
        new_maximum_height: i32)  {
        
        todo!();
        /*
            // if you've set up a custom constrainer then these settings won't have any effect..
        jassert (constrainer == &defaultConstrainer || constrainer == nullptr);

        if (constrainer == nullptr)
            setConstrainer (&defaultConstrainer);

        defaultConstrainer.setSizeLimits (newMinimumWidth, newMinimumHeight,
                                          newMaximumWidth, newMaximumHeight);

        setBoundsConstrained (getBounds());
        */
    }
    
    /**
      | Can be used to enable or disable user-dragging
      | of the window.
      |
      */
    pub fn set_draggable(&mut self, should_be_draggable: bool)  {
        
        todo!();
        /*
            canDrag = shouldBeDraggable;
        */
    }
    
    /**
      | Sets the bounds-constrainer object
      | to use for resizing and dragging this
      | window.
      | 
      | A pointer to the object you pass in will
      | be kept, but it won't be deleted by this
      | object, so it's the caller's responsibility
      | to manage it.
      | 
      | If you pass a nullptr, then no constraints
      | will be placed on the positioning of
      | the window.
      |
      */
    pub fn set_constrainer(&mut self, new_constrainer: *mut ComponentBoundsConstrainer)  {
        
        todo!();
        /*
            if (constrainer != newConstrainer)
        {
            constrainer = newConstrainer;

            bool useBottomRightCornerResizer = resizableCorner != nullptr;
            bool shouldBeResizable = useBottomRightCornerResizer || resizableBorder != nullptr;

            resizableCorner.reset();
            resizableBorder.reset();

            setResizable (shouldBeResizable, useBottomRightCornerResizer);
            updatePeerConstrainer();
        }
        */
    }
    
    /**
      | Calls the window's setBounds method,
      | after first checking these bounds with
      | the current constrainer. @see setConstrainer
      |
      */
    pub fn set_bounds_constrained(&mut self, new_bounds: &Rectangle<i32>)  {
        
        todo!();
        /*
            if (constrainer != nullptr)
            constrainer->setBoundsForComponent (this, newBounds, false, false, false, false);
        else
            setBounds (newBounds);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        lf.fillResizableWindowBackground (g, getWidth(), getHeight(),
                                          getBorderThickness(), *this);

        if (! isFullScreen())
            lf.drawResizableWindowBorder (g, getWidth(), getHeight(),
                                          getBorderThickness(), *this);

       #if ALOE_DEBUG
        /* If this fails, then you've probably written a subclass with a resized()
           callback but forgotten to make it call its parent class's resized() method.

           It's important when you override methods like resized(), moved(),
           etc., that you make sure the base class methods also get called.

           Of course you shouldn't really be overriding ResizableWindow::resized() anyway,
           because your content should all be inside the content component - and it's the
           content component's resized() method that you should be using to do your
           layout.
        */
        jassert (hasBeenResized || (getWidth() == 0 && getHeight() == 0));
       #endif
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            resized();

        if (isOnDesktop())
        {
            Component::addToDesktop (getDesktopWindowStyleFlags());
            updatePeerConstrainer();
        }
        */
    }
    
    /**
      | Returns the colour currently being
      | used for the window's background.
      | 
      | As a convenience the window will fill
      | itself with this colour, but you can
      | override the paint() method if you need
      | more customised behaviour.
      | 
      | This method is the same as retrieving
      | the colour for ResizableWindow::backgroundColourId.
      | 
      | @see setBackgroundColour
      |
      */
    pub fn get_background_colour(&self) -> Colour {
        
        todo!();
        /*
            return findColour (backgroundColourId, false);
        */
    }
    
    /**
      | Changes the colour currently being
      | used for the window's background.
      | 
      | As a convenience the window will fill
      | itself with this colour, but you can
      | override the paint() method if you need
      | more customised behaviour.
      | 
      | -----------
      | @note
      | 
      | the opaque state of this window is altered
      | by this call to reflect the opacity of
      | the colour passed-in. On window systems
      | which can't support semi-transparent
      | windows this might cause problems,
      | (though it's unlikely you'll be using
      | this class as a base for a semi-transparent
      | component anyway).
      | 
      | You can also use the ResizableWindow::backgroundColourId
      | colour id to set this colour.
      | 
      | @see getBackgroundColour
      |
      */
    pub fn set_background_colour(&mut self, new_colour: Colour)  {
        
        todo!();
        /*
            auto backgroundColour = newColour;

        if (! Desktop::canUseSemiTransparentWindows())
            backgroundColour = newColour.withAlpha (1.0f);

        setColour (backgroundColourId, backgroundColour);
        setOpaque (backgroundColour.isOpaque());
        repaint();
        */
    }
    
    /**
      | Returns true if the window is currently
      | in full-screen mode. @see setFullScreen
      |
      */
    pub fn is_full_screen(&self) -> bool {
        
        todo!();
        /*
            if (isOnDesktop())
        {
            auto* peer = getPeer();
            return peer != nullptr && peer->isFullScreen();
        }

        return fullscreen;
        */
    }
    
    /**
      | Puts the window into full-screen mode,
      | or restores it to its normal size.
      | 
      | If true, the window will become full-screen;
      | if false, it will return to the last size
      | it was before being made full-screen.
      | 
      | @see isFullScreen
      |
      */
    pub fn set_full_screen(&mut self, should_be_full_screen: bool)  {
        
        todo!();
        /*
            if (shouldBeFullScreen != isFullScreen())
        {
            updateLastPosIfShowing();
            fullscreen = shouldBeFullScreen;

            if (isOnDesktop())
            {
                if (auto* peer = getPeer())
                {
                    // keep a copy of this intact in case the real one gets messed-up while we're un-maximising
                    auto lastPos = lastNonFullScreenPos;

                    peer->setFullScreen (shouldBeFullScreen);

                    if ((! shouldBeFullScreen) && ! lastPos.isEmpty())
                        setBounds (lastPos);
                }
                else
                {
                    jassertfalse;
                }
            }
            else
            {
                if (shouldBeFullScreen)
                    setBounds (0, 0, getParentWidth(), getParentHeight());
                else
                    setBounds (lastNonFullScreenPos);
            }

            resized();
        }
        */
    }
    
    /**
      | Returns true if the window is currently
      | minimised. @see setMinimised
      |
      */
    pub fn is_minimised(&self) -> bool {
        
        todo!();
        /*
            if (auto* peer = getPeer())
            return peer->isMinimised();

        return false;
        */
    }
    
    /**
      | Minimises the window, or restores it
      | to its previous position and size.
      | 
      | When being un-minimised, it'll return
      | to the last position and size it was in
      | before being minimised.
      | 
      | @see isMinimised
      |
      */
    pub fn set_minimised(&mut self, should_minimise: bool)  {
        
        todo!();
        /*
            if (shouldMinimise != isMinimised())
        {
            if (auto* peer = getPeer())
            {
                updateLastPosIfShowing();
                peer->setMinimised (shouldMinimise);
            }
            else
            {
                jassertfalse;
            }
        }
        */
    }
    
    /**
      | Returns true if the window has been placed
      | in kiosk-mode. @see Desktop::setKioskComponent
      |
      */
    pub fn is_kiosk_mode(&self) -> bool {
        
        todo!();
        /*
            if (isOnDesktop())
            if (auto* peer = getPeer())
                return peer->isKioskMode();

        return Desktop::getInstance().getKioskModeComponent() == this;
        */
    }
    
    pub fn update_last_pos_if_showing(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
        {
            updateLastPosIfNotFullScreen();
            updatePeerConstrainer();
        }
        */
    }
    
    pub fn update_last_pos_if_not_full_screen(&mut self)  {
        
        todo!();
        /*
            if (! (isFullScreen() || isMinimised() || isKioskMode()))
            lastNonFullScreenPos = getBounds();
        */
    }
    
    pub fn update_peer_constrainer(&mut self)  {
        
        todo!();
        /*
            if (isOnDesktop())
            if (auto* peer = getPeer())
                peer->setConstrainer (constrainer);
        */
    }
    
    pub fn parent_size_changed(&mut self)  {
        
        todo!();
        /*
            if (isFullScreen() && getParentComponent() != nullptr)
            setBounds (getParentComponent()->getLocalBounds());
        */
    }
    
    /**
      | Returns a string which encodes the window's
      | current size and position.
      | 
      | This string will encapsulate the window's
      | size, position, and whether it's in
      | full-screen mode. It's intended for
      | letting your application save and restore
      | a window's position.
      | 
      | Use the restoreWindowStateFromString()
      | to restore from a saved state.
      | 
      | @see restoreWindowStateFromString
      |
      */
    pub fn get_window_state_as_string(&mut self) -> String {
        
        todo!();
        /*
            updateLastPosIfShowing();
        auto stateString = (isFullScreen() && ! isKioskMode() ? "fs " : "") + lastNonFullScreenPos.toString();

       #if ALOE_LINUX
        if (auto* peer = isOnDesktop() ? getPeer() : nullptr)
        {
            const auto frameSize = peer->getFrameSize();
            stateString << " frame " << frameSize.getTop() << ' ' << frameSize.getLeft()
                        << ' ' << frameSize.getBottom() << ' ' << frameSize.getRight();
        }
       #endif

        return stateString;
        */
    }
    
    /**
      | Restores the window to a previously-saved
      | size and position.
      | 
      | This restores the window's size, position
      | and full-screen status from an string
      | that was previously created with the
      | getWindowStateAsString() method.
      | 
      | 
      | -----------
      | @return
      | 
      | false if the string wasn't a valid window
      | state @see getWindowStateAsString
      |
      */
    pub fn restore_window_state_from_string(&mut self, s: &String) -> bool {
        
        todo!();
        /*
            StringArray tokens;
        tokens.addTokens (s, false);
        tokens.removeEmptyStrings();
        tokens.trim();

        const bool fs = tokens[0].startsWithIgnoreCase ("fs");
        const int firstCoord = fs ? 1 : 0;

        if (tokens.size() < firstCoord + 4)
            return false;

        Rectangle<int> newPos (tokens[firstCoord].getIntValue(),
                               tokens[firstCoord + 1].getIntValue(),
                               tokens[firstCoord + 2].getIntValue(),
                               tokens[firstCoord + 3].getIntValue());

        if (newPos.isEmpty())
            return false;

        auto* peer = isOnDesktop() ? getPeer() : nullptr;

        if (peer != nullptr)
        {
            peer->getFrameSize().addTo (newPos);
        }
       #if ALOE_LINUX
        else
        {
            // We need to adjust for the frame size before we create a peer, as X11
            // doesn't provide this information at construction time.
            if (tokens[firstCoord + 4] == "frame" && tokens.size() == firstCoord + 9)
            {
                BorderSize<int> frame { tokens[firstCoord + 5].getIntValue(),
                                        tokens[firstCoord + 6].getIntValue(),
                                        tokens[firstCoord + 7].getIntValue(),
                                        tokens[firstCoord + 8].getIntValue() };

                frame.addTo (newPos);
                setBounds (newPos);
            }
        }
       #endif

        {
            auto& desktop = Desktop::getInstance();
            auto allMonitors = desktop.getDisplays().getRectangleList (true);
            allMonitors.clipTo (newPos);
            auto onScreenArea = allMonitors.getBounds();

            if (onScreenArea.getWidth() * onScreenArea.getHeight() < 32 * 32)
            {
                auto screen = desktop.getDisplays().getDisplayForRect (newPos)->userArea;

                newPos.setSize (jmin (newPos.getWidth(),  screen.getWidth()),
                                jmin (newPos.getHeight(), screen.getHeight()));

                newPos.setPosition (jlimit (screen.getX(), screen.getRight()  - newPos.getWidth(),  newPos.getX()),
                                    jlimit (screen.getY(), screen.getBottom() - newPos.getHeight(), newPos.getY()));
            }
        }

        if (peer != nullptr)
        {
            peer->getFrameSize().subtractFrom (newPos);
            peer->setNonFullScreenBounds (newPos);
        }

        updateLastPosIfNotFullScreen();

        if (fs)
            setBoundsConstrained (newPos);

        setFullScreen (fs);

        if (! fs)
            setBoundsConstrained (newPos);

        return true;
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (canDrag && ! isFullScreen())
        {
            dragStarted = true;
            dragger.startDraggingComponent (this, e);
        }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (dragStarted)
            dragger.dragComponent (this, e, constrainer);
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            dragStarted = false;
        */
    }

    /**
      | Overridden to warn people about adding
      | components directly to this component
      | instead of using setContentOwned().
      | 
      | If you know what you're doing and are
      | sure you really want to add a component,
      | specify a base-class method call to
      | Component::addAndMakeVisible(),
      | to side-step this warning.
      |
      */
    #[cfg(ALOE_DEBUG)]
    pub fn add_child_component(&mut self, 
        child:   *mut Component<'a>,
        z_order: i32)  {

        let z_order: i32 = z_order.unwrap_or(-1);
        
        todo!();
        /*
            /* Agh! You shouldn't add components directly to a ResizableWindow - this class
           manages its child components automatically, and if you add your own it'll cause
           trouble. Instead, use setContentComponent() to give it a component which
           will be automatically resized and kept in the right place - then you can add
           subcomponents to the content comp. See the notes for the ResizableWindow class
           for more info.

           If you really know what you're doing and want to avoid this assertion, just call
           Component::addChildComponent directly.
        */
        jassertfalse;

        Component::addChildComponent (child, zOrder);
        */
    }
    
    /**
      | Overridden to warn people about adding
      | components directly to this component
      | instead of using setContentOwned().
      | 
      | If you know what you're doing and are
      | sure you really want to add a component,
      | specify a base-class method call to
      | Component::addAndMakeVisible(),
      | to side-step this warning.
      |
      */
    #[cfg(ALOE_DEBUG)]
    pub fn add_and_make_visible(&mut self, 
        child:   *mut Component<'a>,
        z_order: i32)  {

        let z_order: i32 = z_order.unwrap_or(-1);
        
        todo!();
        /*
            /* Agh! You shouldn't add components directly to a ResizableWindow - this class
           manages its child components automatically, and if you add your own it'll cause
           trouble. Instead, use setContentComponent() to give it a component which
           will be automatically resized and kept in the right place - then you can add
           subcomponents to the content comp. See the notes for the ResizableWindow class
           for more info.

           If you really know what you're doing and want to avoid this assertion, just call
           Component::addAndMakeVisible directly.
        */
        jassertfalse;

        Component::addAndMakeVisible (child, zOrder);
        */
    }
}
