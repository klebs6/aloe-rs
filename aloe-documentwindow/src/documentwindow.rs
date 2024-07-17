crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_DocumentWindow.h]

/**
  | A resizable window with a title bar and
  | maximise, minimise and close buttons.
  | 
  | This subclass of ResizableWindow creates
  | a fairly standard type of window with
  | a title bar and various buttons. The
  | name of the component is shown in the
  | title bar, and an icon can optionally
  | be specified with setIcon().
  | 
  | All the methods available to a ResizableWindow
  | are also available to this, so it can
  | easily be made resizable, minimised,
  | maximised, etc.
  | 
  | It's not advisable to add child components
  | directly to a DocumentWindow: put them
  | inside your content component instead.
  | And overriding methods like resized(),
  | moved(), etc is also not recommended
  | - instead override these methods for
  | your content component. (If for some
  | obscure reason you do need to override
  | these methods, always remember to call
  | the super-class's resized() method
  | too, otherwise it'll fail to lay out
  | the window decorations correctly).
  | 
  | You can also automatically add a menu
  | bar to the window, using the setMenuBar()
  | method.
  | 
  | @see ResizableWindow, DialogWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DocumentWindow<'a> {
    base:                               ResizableWindow<'a>,
    title_bar_height:                   i32, // default = 26
    menu_bar_height:                    i32, // default = 24
    required_buttons:                   i32,
    position_title_bar_buttons_on_left: bool,
    draw_title_text_centred:            bool, // default = true
    title_bar_buttons:                  [Option<Box<Button<'a>>>; 3],
    title_bar_icon:                     Image,
    menu_bar:                           Box<Component<'a>>,
    menu_bar_model:                     Option<&'a mut MenuBarModel<'a>>, // default = nullptr
    button_listener:                    Box<DocumentWindowButtonListenerProxy<'a>>,
}

impl<'a> Drop for DocumentWindow<'a> {

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
            // Don't delete or remove the resizer components yourself! They're managed by the
        // DocumentWindow, and you should leave them alone! You may have deleted them
        // accidentally by careless use of deleteAllChildren()..?
        jassert (menuBar == nullptr || getIndexOfChildComponent (menuBar.get()) >= 0);
        jassert (titleBarButtons[0] == nullptr || getIndexOfChildComponent (titleBarButtons[0].get()) >= 0);
        jassert (titleBarButtons[1] == nullptr || getIndexOfChildComponent (titleBarButtons[1].get()) >= 0);
        jassert (titleBarButtons[2] == nullptr || getIndexOfChildComponent (titleBarButtons[2].get()) >= 0);

        for (auto& b : titleBarButtons)
            b.reset();

        menuBar.reset();
        */
    }
}

impl<'a> DocumentWindowInterface for DocumentWindow<'a> { }

impl<'a> CloseButtonPressed for DocumentWindow<'a> { 

    fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            /*  If you've got a close button, you have to override this method to get
            rid of your window!

            If the window is just a pop-up, you should override this method and make
            it delete the window in whatever way is appropriate for your app. E.g. you
            might just want to call "delete this".

            If your app is centred around this window such that the whole app should quit when
            the window is closed, then you will probably want to use this method as an opportunity
            to call ALOEApplicationBase::quit(), and leave the window to be deleted later by your
            ALOEApplicationBase::shutdown() method. (Doing it this way means that your window will
            still get cleaned-up if the app is quit by some other means (e.g. a cmd-Q on the mac
            or closing it via the taskbar icon on Windows).
        */
        jassertfalse;
        */
    }
}

impl<'a> MinimiseButtonPressed for DocumentWindow<'a> { 

    fn minimise_button_pressed(&mut self)  {
        
        todo!();
        /*
            setMinimised (true);
        */
    }
}

impl<'a> MaximiseButtonPressed for DocumentWindow<'a> { 

    fn maximise_button_pressed(&mut self)  {
        
        todo!();
        /*
            setFullScreen (! isFullScreen());
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_DocumentWindow.cpp]
impl<'a> DocumentWindow<'a> {

    /**
      | Creates a DocumentWindow.
      | 
      | -----------
      | @param name
      | 
      | the name to give the component - this
      | is also the title shown at the top of the
      | window. To change this later, use setName()
      | ----------
      | @param backgroundColour
      | 
      | the colour to use for filling the window's
      | background.
      | ----------
      | @param requiredButtons
      | 
      | specifies which of the buttons (close,
      | minimise, maximise) should be shown
      | on the title bar. This value is a bitwise
      | combination of values from the DocumentWindowTitleBarButtons
      | enum. Note that it can be "allButtons"
      | to get them all. You can change this later
      | with the setTitleBarButtonsRequired()
      | method, which can also specify where
      | they are positioned.
      | ----------
      | @param addToDesktop
      | 
      | if true, the window will be automatically
      | added to the desktop; if false, you can
      | use it as a child component @see DocumentWindowTitleBarButtons
      |
      */
    pub fn new(
        title:             &String,
        background_colour: Colour,
        required_buttons:  i32,
        add_to_desktop:    Option<bool>

    ) -> Self {

        let add_to_desktop: bool = add_to_desktop.unwrap_or(true);
    
        todo!();
        /*


            : ResizableWindow (title, backgroundColour, addToDesktop_),
          requiredButtons (requiredButtons_),
         #if ALOE_MAC
          positionTitleBarButtonsOnLeft (true)
         #else
          positionTitleBarButtonsOnLeft (false)
         #endif

        setResizeLimits (128, 128, 32768, 32768);

        DocumentWindow::lookAndFeelChanged();
        */
    }
    
    pub fn repaint_title_bar(&mut self)  {
        
        todo!();
        /*
            repaint (getTitleBarArea());
        */
    }
    
    /**
      | Changes the component's name.
      | 
      | (This is overridden from Component::setName()
      | to cause a repaint, as the name is what
      | gets drawn across the window's title
      | bar).
      |
      */
    pub fn set_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            if (newName != getName())
        {
            Component::setName (newName);
            repaintTitleBar();
        }
        */
    }
    
    /**
      | Sets an icon to show in the title bar,
      | next to the title.
      | 
      | A copy is made internally of the image,
      | so the caller can delete the image after
      | calling this. If an empty Image is passed-in,
      | any existing icon will be removed.
      |
      */
    pub fn set_icon(&mut self, image_to_use: &Image)  {
        
        todo!();
        /*
            titleBarIcon = imageToUse;
        repaintTitleBar();
        */
    }
    
    /**
      | Changes the height of the title-bar.
      |
      */
    pub fn set_title_bar_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            titleBarHeight = newHeight;
        resized();
        repaintTitleBar();
        */
    }
    
    /**
      | Changes the set of title-bar buttons
      | being shown.
      | 
      | -----------
      | @param requiredButtons
      | 
      | specifies which of the buttons (close,
      | minimise, maximise) should be shown
      | on the title bar. This value is a bitwise
      | combination of values from the DocumentWindowTitleBarButtons
      | enum. Note that it can be "allButtons"
      | to get them all.
      | ----------
      | @param positionTitleBarButtonsOnLeft
      | 
      | if true, the buttons should go at the
      | left side of the bar; if false, they'll
      | be placed at the right
      |
      */
    pub fn set_title_bar_buttons_required(&mut self, 
        buttons: i32,
        on_left: bool)  {
        
        todo!();
        /*
            requiredButtons = buttons;
        positionTitleBarButtonsOnLeft = onLeft;
        lookAndFeelChanged();
        */
    }
    
    /**
      | Sets whether the title should be centred
      | within the window.
      | 
      | If true, the title text is shown in the
      | middle of the title-bar; if false, it'll
      | be shown at the left of the bar.
      |
      */
    pub fn set_title_bar_text_centred(&mut self, text_should_be_centred: bool)  {
        
        todo!();
        /*
            drawTitleTextCentred = textShouldBeCentred;
        repaintTitleBar();
        */
    }
    
    /**
      | Creates a menu inside this window.
      | 
      | -----------
      | @param menuBarModel
      | 
      | this specifies a MenuBarModel that
      | should be used to generate the contents
      | of a menu bar that will be placed just
      | below the title bar, and just above any
      | content component. If this value is
      | a nullptr, any existing menu bar will
      | be removed from the component; if it
      | is not a nullptr, one will be added if
      | it's required.
      | ----------
      | @param menuBarHeight
      | 
      | the height of the menu bar component,
      | if one is needed. Pass a value of zero
      | or less to use the look-and-feel's default
      | size.
      |
      */
    pub fn set_menu_bar(
        &mut self, 
        menu_bar_model:  *mut MenuBarModel,
        menu_bar_height: Option<i32>

    ) {

        let menu_bar_height: i32 = menu_bar_height.unwrap_or(0);
        
        todo!();
        /*
            if (menuBarModel != newMenuBarModel)
        {
            menuBar.reset();

            menuBarModel = newMenuBarModel;
            menuBarHeight = newMenuBarHeight > 0 ? newMenuBarHeight
                                                 : getLookAndFeel().getDefaultMenuBarHeight();

            if (menuBarModel != nullptr)
                setMenuBarComponent (new MenuBarComponent (menuBarModel));

            resized();
        }
        */
    }
    
    /**
      | Returns the current menu bar component,
      | or null if there isn't one.
      | 
      | This is probably a MenuBarComponent,
      | unless a custom one has been set using
      | setMenuBarComponent().
      |
      */
    pub fn get_menu_bar_component(&self) -> *mut Component {
        
        todo!();
        /*
            return menuBar.get();
        */
    }
    
    /**
      | Replaces the current menu bar with a
      | custom component.
      | 
      | The component will be owned and deleted
      | by the document window.
      |
      */
    pub fn set_menu_bar_component(&mut self, new_menu_bar_component: *mut Component<'a>)  {
        
        todo!();
        /*
            menuBar.reset (newMenuBarComponent);
        Component::addAndMakeVisible (menuBar.get()); // (call the superclass method directly to avoid the assertion in ResizableWindow)

        if (menuBar != nullptr)
            menuBar->setEnabled (isActiveWindow());

        resized();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            ResizableWindow::paint (g);

        auto titleBarArea = getTitleBarArea();
        g.reduceClipRegion (titleBarArea);
        g.setOrigin (titleBarArea.getPosition());

        int titleSpaceX1 = 6;
        int titleSpaceX2 = titleBarArea.getWidth() - 6;

        for (auto& b : titleBarButtons)
        {
            if (b != nullptr)
            {
                if (positionTitleBarButtonsOnLeft)
                    titleSpaceX1 = jmax (titleSpaceX1, b->getRight() + (getWidth() - b->getRight()) / 8);
                else
                    titleSpaceX2 = jmin (titleSpaceX2, b->getX() - (b->getX() / 8));
            }
        }

        getLookAndFeel().drawDocumentWindowTitleBar (*this, g,
                                                     titleBarArea.getWidth(),
                                                     titleBarArea.getHeight(),
                                                     titleSpaceX1,
                                                     jmax (1, titleSpaceX2 - titleSpaceX1),
                                                     titleBarIcon.isValid() ? &titleBarIcon : nullptr,
                                                     ! drawTitleTextCentred);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            ResizableWindow::resized();

        if (auto* b = getMaximiseButton())
            b->setToggleState (isFullScreen(), dontSendNotification);

        auto titleBarArea = getTitleBarArea();

        getLookAndFeel()
            .positionDocumentWindowButtons (*this,
                                            titleBarArea.getX(), titleBarArea.getY(),
                                            titleBarArea.getWidth(), titleBarArea.getHeight(),
                                            titleBarButtons[0].get(),
                                            titleBarButtons[1].get(),
                                            titleBarButtons[2].get(),
                                            positionTitleBarButtonsOnLeft);

        if (menuBar != nullptr)
            menuBar->setBounds (titleBarArea.getX(), titleBarArea.getBottom(),
                                titleBarArea.getWidth(), menuBarHeight);
        */
    }
    
    pub fn get_border_thickness(&mut self) -> BorderSize<i32> {
        
        todo!();
        /*
            return ResizableWindow::getBorderThickness();
        */
    }
    
    pub fn get_content_component_border(&mut self) -> BorderSize<i32> {
        
        todo!();
        /*
            auto border = getBorderThickness();

        if (! isKioskMode())
            border.setTop (border.getTop()
                            + (isUsingNativeTitleBar() ? 0 : titleBarHeight)
                            + (menuBar != nullptr ? menuBarHeight : 0));

        return border;
        */
    }
    
    /**
      | Returns the current title bar height.
      |
      */
    pub fn get_title_bar_height(&self) -> i32 {
        
        todo!();
        /*
            return isUsingNativeTitleBar() ? 0 : jmin (titleBarHeight, getHeight() - 4);
        */
    }
    
    pub fn get_title_bar_area(&mut self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (isKioskMode())
            return {};

        auto border = getBorderThickness();
        return { border.getLeft(), border.getTop(), getWidth() - border.getLeftAndRight(), getTitleBarHeight() };
        */
    }
    
    /**
      | Returns the close button, (or nullptr
      | if there isn't one).
      |
      */
    pub fn get_close_button(&self) -> *mut Button {
        
        todo!();
        /*
            return titleBarButtons[2].get();
        */
    }
    
    /**
      | Returns the minimise button, (or nullptr
      | if there isn't one).
      |
      */
    pub fn get_minimise_button(&self) -> *mut Button {
        
        todo!();
        /*
            return titleBarButtons[0].get();
        */
    }
    
    /**
      | Returns the maximise button, (or nullptr
      | if there isn't one).
      |
      */
    pub fn get_maximise_button(&self) -> *mut Button {
        
        todo!();
        /*
            return titleBarButtons[1].get();
        */
    }
    
    pub fn get_desktop_window_style_flags(&self) -> i32 {
        
        todo!();
        /*
            auto styleFlags = ResizableWindow::getDesktopWindowStyleFlags();

        if ((requiredButtons & minimiseButton) != 0)  styleFlags |= ComponentPeer::windowHasMinimiseButton;
        if ((requiredButtons & maximiseButton) != 0)  styleFlags |= ComponentPeer::windowHasMaximiseButton;
        if ((requiredButtons & closeButton)    != 0)  styleFlags |= ComponentPeer::windowHasCloseButton;

        return styleFlags;
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            for (auto& b : titleBarButtons)
            b.reset();

        if (! isUsingNativeTitleBar())
        {
            auto& lf = getLookAndFeel();

            if ((requiredButtons & minimiseButton) != 0)  titleBarButtons[0].reset (lf.createDocumentWindowButton (minimiseButton));
            if ((requiredButtons & maximiseButton) != 0)  titleBarButtons[1].reset (lf.createDocumentWindowButton (maximiseButton));
            if ((requiredButtons & closeButton)    != 0)  titleBarButtons[2].reset (lf.createDocumentWindowButton (closeButton));

            for (auto& b : titleBarButtons)
            {
                if (b != nullptr)
                {
                    if (buttonListener == nullptr)
                        buttonListener.reset (new DocumentWindowButtonListenerProxy (*this));

                    b->addListener (buttonListener.get());
                    b->setWantsKeyboardFocus (false);

                    // (call the Component method directly to avoid the assertion in ResizableWindow)
                    Component::addAndMakeVisible (b.get());
                }
            }

            if (auto* b = getCloseButton())
            {
               #if ALOE_MAC
                b->addShortcut (KeyPress ('w', ModifierKeys::commandModifier, 0));
               #else
                b->addShortcut (KeyPress (KeyPress::F4Key, ModifierKeys::altModifier, 0));
               #endif
            }
        }

        activeWindowStatusChanged();

        ResizableWindow::lookAndFeelChanged();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn active_window_status_changed(&mut self)  {
        
        todo!();
        /*
            ResizableWindow::activeWindowStatusChanged();
        bool isActive = isActiveWindow();

        for (auto& b : titleBarButtons)
            if (b != nullptr)
                b->setEnabled (isActive);

        if (menuBar != nullptr)
            menuBar->setEnabled (isActive);
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (getTitleBarArea().contains (e.x, e.y))
            if (auto* maximise = getMaximiseButton())
                maximise->triggerClick();
        */
    }
    
    pub fn user_tried_to_close_window(&mut self)  {
        
        todo!();
        /*
            closeButtonPressed();
        */
    }
}
