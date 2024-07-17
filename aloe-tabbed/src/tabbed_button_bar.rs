crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_TabbedButtonBar.h]

/**
  | A vertical or horizontal bar containing
  | tabs that you can select.
  | 
  | You can use one of these to generate things
  | like a dialog box that has tabbed pages
  | you can flip between. Attach a ChangeListener
  | to the button bar to be told when the user
  | changes the page.
  | 
  | An easier method than doing this is to
  | use a TabbedComponent, which contains
  | its own TabbedButtonBar and which takes
  | care of the layout and other housekeeping.
  | 
  | @see TabbedComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TabbedButtonBar<'a> {
    base:              Component<'a>,
    base2:             ChangeBroadcaster<'a>,
    tabs:              Vec<Box<TabbedButtonBarTabInfo<'a>>>,
    orientation:       TabbedButtonBarOrientation,
    minimum_scale:     f64, // default = 0.7
    current_tab_index: i32, // default = -1
    behind_front_tab:  Box<TabbedButtonBarBehindFrontTabComp<'a>>,
    extra_tabs_button: Box<Button<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_TabbedButtonBar.cpp]
impl<'a> Drop for TabbedButtonBar<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        tabs.clear();
        extraTabsButton.reset();
 */
    }
}

impl<'a> TabbedButtonBar<'a> {

    /**
      | Returns the bar's current orientation.
      | @see setOrientation
      |
      */
    pub fn get_orientation(&self) -> TabbedButtonBarOrientation {
        
        todo!();
        /*
            return orientation;
        */
    }

    /**
      | Returns true if the orientation is TabsAtLeft
      | or TabsAtRight.
      |
      */
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return orientation == TabsAtLeft || orientation == TabsAtRight;
        */
    }

    /**
      | Returns the thickness of the bar, which
      | may be its width or height, depending
      | on the orientation.
      |
      */
    pub fn get_thickness(&self) -> i32 {
        
        todo!();
        /*
            return isVertical() ? getWidth() : getHeight();
        */
    }

    /**
      | Returns the index of the currently selected
      | tab.
      | 
      | This could return -1 if none are selected.
      |
      */
    pub fn get_current_tab_index(&self) -> i32 {
        
        todo!();
        /*
            return currentTabIndex;
        */
    }

    /**
      | Creates a TabbedButtonBar with a given
      | orientation.
      | 
      | You can change the orientation later
      | if you need to.
      |
      */
    pub fn new(orientation_to_use: TabbedButtonBarOrientation) -> Self {
    
        todo!();
        /*
        : orientation(orientationToUse),

            setInterceptsMouseClicks (false, true);
        behindFrontTab.reset (new TabbedButtonBarBehindFrontTabComp (*this));
        addAndMakeVisible (behindFrontTab.get());
        setFocusContainerType (FocusContainerType::keyboardFocusContainer);
        */
    }
    
    /**
      | Changes the bar's orientation.
      | 
      | This won't change the bar's actual size
      | - you'll need to do that yourself, but
      | this determines which direction the
      | tabs go in, and which side they're stuck
      | to.
      |
      */
    pub fn set_orientation(&mut self, new_orientation: TabbedButtonBarOrientation)  {
        
        todo!();
        /*
            orientation = newOrientation;

        for (auto* child : getChildren())
            child->resized();

        resized();
        */
    }
    
    pub fn create_tab_button(&mut self, 
        name:  &String,
        index: i32) -> *mut TabBarButton {
        
        todo!();
        /*
            return new TabBarButton (name, *this);
        */
    }
    
    /**
      | Changes the minimum scale factor to
      | which the tabs can be compressed when
      | trying to fit a lot of tabs on-screen.
      |
      */
    pub fn set_minimum_tab_scale_factor(&mut self, new_minimum_scale: f64)  {
        
        todo!();
        /*
            minimumScale = newMinimumScale;
        resized();
        */
    }
    
    /**
      | Deletes all the tabs from the bar. @see
      | addTab
      |
      */
    pub fn clear_tabs(&mut self)  {
        
        todo!();
        /*
            tabs.clear();
        extraTabsButton.reset();
        setCurrentTabIndex (-1);
        */
    }
    
    /**
      | Adds a tab to the bar.
      | 
      | Tabs are added in left-to-right reading
      | order.
      | 
      | If this is the first tab added, it'll
      | also be automatically selected.
      |
      */
    pub fn add_tab(&mut self, 
        tab_name:              &String,
        tab_background_colour: Colour,
        insert_index:          i32)  {
        
        todo!();
        /*
            jassert (tabName.isNotEmpty()); // you have to give them all a name..

        if (tabName.isNotEmpty())
        {
            if (! isPositiveAndBelow (insertIndex, tabs.size()))
                insertIndex = tabs.size();

            auto* currentTab = tabs[currentTabIndex];

            auto* newTab = new TabbedButtonBarTabInfo();
            newTab->name = tabName;
            newTab->colour = tabBackgroundColour;
            newTab->button.reset (createTabButton (tabName, insertIndex));
            jassert (newTab->button != nullptr);

            tabs.insert (insertIndex, newTab);
            currentTabIndex = tabs.indexOf (currentTab);
            addAndMakeVisible (newTab->button.get(), insertIndex);

            resized();

            if (currentTabIndex < 0)
                setCurrentTabIndex (0);
        }
        */
    }
    
    /**
      | Changes the name of one of the tabs.
      |
      */
    pub fn set_tab_name(&mut self, 
        tab_index: i32,
        new_name:  &String)  {
        
        todo!();
        /*
            if (auto* tab = tabs[tabIndex])
        {
            if (tab->name != newName)
            {
                tab->name = newName;
                tab->button->setButtonText (newName);
                resized();
            }
        }
        */
    }
    
    /**
      | Gets rid of one of the tabs.
      |
      */
    pub fn remove_tab(&mut self, 
        index_to_remove: i32,
        animate:         Option<bool>)  {

        let animate: bool = animate.unwrap_or(false);
        
        todo!();
        /*
            if (isPositiveAndBelow (indexToRemove, tabs.size()))
        {
            auto oldSelectedIndex = currentTabIndex;

            if (indexToRemove == currentTabIndex)
                oldSelectedIndex = -1;
            else if (indexToRemove < oldSelectedIndex)
                --oldSelectedIndex;

            tabs.remove (indexToRemove);

            setCurrentTabIndex (oldSelectedIndex);
            updateTabPositions (animate);
        }
        */
    }
    
    /**
      | Moves a tab to a new index in the list.
      | 
      | Pass -1 as the index to move it to the end
      | of the list.
      |
      */
    pub fn move_tab(&mut self, 
        current_index: i32,
        new_index:     i32,
        animate:       Option<bool>)  {

        let animate: bool = animate.unwrap_or(false);
        
        todo!();
        /*
            auto* currentTab = tabs[currentTabIndex];
        tabs.move (currentIndex, newIndex);
        currentTabIndex = tabs.indexOf (currentTab);
        updateTabPositions (animate);
        */
    }
    
    /**
      | Returns the number of tabs in the bar.
      |
      */
    pub fn get_num_tabs(&self) -> i32 {
        
        todo!();
        /*
            return tabs.size();
        */
    }
    
    /**
      | Returns the name of the currently selected
      | tab.
      | 
      | This could be an empty string if none
      | are selected.
      |
      */
    pub fn get_current_tab_name(&self) -> String {
        
        todo!();
        /*
            if (auto* tab = tabs [currentTabIndex])
            return tab->name;

        return {};
        */
    }
    
    /**
      | Returns a list of all the tab names in
      | the bar.
      |
      */
    pub fn get_tab_names(&self) -> Vec<String> {
        
        todo!();
        /*
            StringArray names;

        for (auto* t : tabs)
            names.add (t->name);

        return names;
        */
    }
    
    /**
      | Changes the currently selected tab.
      | 
      | This will send a change message and cause
      | a synchronous callback to the currentTabChanged()
      | method. (But if the given tab is already
      | selected, nothing will be done).
      | 
      | To deselect all the tabs, use an index
      | of -1.
      |
      */
    pub fn set_current_tab_index(
        &mut self, 
        new_index:                  i32,
        should_send_change_message: Option<bool>

    ) {

        let should_send_change_message: bool = should_send_change_message.unwrap_or(true);
        
        todo!();
        /*
            if (currentTabIndex != newIndex)
        {
            if (! isPositiveAndBelow (newIndex, tabs.size()))
                newIndex = -1;

            currentTabIndex = newIndex;

            for (int i = 0; i < tabs.size(); ++i)
                tabs.getUnchecked(i)->button->setToggleState (i == newIndex, dontSendNotification);

            resized();

            if (shouldSendChangeMessage)
                sendChangeMessage();

            currentTabChanged (newIndex, getCurrentTabName());
        }
        */
    }
    
    /**
      | Returns the button for a specific tab.
      | 
      | The button that is returned may be deleted
      | later by this component, so don't hang
      | on to the pointer that is returned. A
      | null pointer may be returned if the index
      | is out of range.
      |
      */
    pub fn get_tab_button(&self, index: i32) -> *mut TabBarButton {
        
        todo!();
        /*
            if (auto* tab = tabs[index])
            return static_cast<TabBarButton*> (tab->button.get());

        return nullptr;
        */
    }
    
    /**
      | Returns the index of a TabBarButton
      | if it belongs to this bar.
      |
      */
    pub fn index_of_tab_button(&self, button: *const TabBarButton) -> i32 {
        
        todo!();
        /*
            for (int i = tabs.size(); --i >= 0;)
            if (tabs.getUnchecked(i)->button.get() == button)
                return i;

        return -1;
        */
    }
    
    /**
      | Returns the final bounds of this button
      | if it is currently being animated.
      |
      */
    pub fn get_target_bounds(&self, button: *mut TabBarButton) -> Rectangle<i32> {
        
        todo!();
        /*
            if (button == nullptr || indexOfTabButton (button) == -1)
            return {};

        auto& animator = Desktop::getInstance().getAnimator();

        return animator.isAnimating (button) ? animator.getComponentDestination (button)
                                             : button->getBounds();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            extraTabsButton.reset();
        resized();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawTabbedButtonBarBackground (*this, g);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            updateTabPositions (false);
        */
    }
    
    pub fn update_tab_positions(&mut self, animate: bool)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        auto depth = getWidth();
        auto length = getHeight();

        if (! isVertical())
            std::swap (depth, length);

        auto overlap = lf.getTabButtonOverlap (depth) + lf.getTabButtonSpaceAroundImage() * 2;

        auto totalLength = jmax (0, overlap);
        auto numVisibleButtons = tabs.size();

        for (int i = 0; i < tabs.size(); ++i)
        {
            auto* tb = tabs.getUnchecked(i)->button.get();

            totalLength += tb->getBestTabLength (depth) - overlap;
            tb->overlapPixels = jmax (0, overlap / 2);
        }

        double scale = 1.0;

        if (totalLength > length)
            scale = jmax (minimumScale, length / (double) totalLength);

        const bool isTooBig = (int) (totalLength * scale) > length;
        int tabsButtonPos = 0;

        if (isTooBig)
        {
            if (extraTabsButton == nullptr)
            {
                extraTabsButton.reset (lf.createTabBarExtrasButton());
                addAndMakeVisible (extraTabsButton.get());
                extraTabsButton->setAlwaysOnTop (true);
                extraTabsButton->setTriggeredOnMouseDown (true);
                extraTabsButton->onClick = [this] { showExtraItemsMenu(); };
            }

            auto buttonSize = jmin (proportionOfWidth (0.7f), proportionOfHeight (0.7f));
            extraTabsButton->setSize (buttonSize, buttonSize);

            if (isVertical())
            {
                tabsButtonPos = getHeight() - buttonSize / 2 - 1;
                extraTabsButton->setCentrePosition (getWidth() / 2, tabsButtonPos);
            }
            else
            {
                tabsButtonPos = getWidth() - buttonSize / 2 - 1;
                extraTabsButton->setCentrePosition (tabsButtonPos, getHeight() / 2);
            }

            totalLength = 0;

            for (int i = 0; i < tabs.size(); ++i)
            {
                auto* tb = tabs.getUnchecked(i)->button.get();
                auto newLength = totalLength + tb->getBestTabLength (depth);

                if (i > 0 && newLength * minimumScale > tabsButtonPos)
                {
                    totalLength += overlap;
                    break;
                }

                numVisibleButtons = i + 1;
                totalLength = newLength - overlap;
            }

            scale = jmax (minimumScale, tabsButtonPos / (double) totalLength);
        }
        else
        {
            extraTabsButton.reset();
        }

        int pos = 0;

        TabBarButton* frontTab = nullptr;
        auto& animator = Desktop::getInstance().getAnimator();

        for (int i = 0; i < tabs.size(); ++i)
        {
            if (auto* tb = getTabButton (i))
            {
                auto bestLength = roundToInt (scale * tb->getBestTabLength (depth));

                if (i < numVisibleButtons)
                {
                    auto newBounds = isVertical() ? Rectangle<int> (0, pos, getWidth(), bestLength)
                                                  : Rectangle<int> (pos, 0, bestLength, getHeight());

                    if (animate)
                    {
                        animator.animateComponent (tb, newBounds, 1.0f, 200, false, 3.0, 0.0);
                    }
                    else
                    {
                        animator.cancelAnimation (tb, false);
                        tb->setBounds (newBounds);
                    }

                    tb->toBack();

                    if (i == currentTabIndex)
                        frontTab = tb;

                    tb->setVisible (true);
                }
                else
                {
                    tb->setVisible (false);
                }

                pos += bestLength - overlap;
            }
        }

        behindFrontTab->setBounds (getLocalBounds());

        if (frontTab != nullptr)
        {
            frontTab->toFront (false);
            behindFrontTab->toBehind (frontTab);
        }
        */
    }
    
    /**
      | Returns the colour of a tab.
      | 
      | This is the colour that was specified
      | in addTab().
      |
      */
    pub fn get_tab_background_colour(&mut self, tab_index: i32) -> Colour {
        
        todo!();
        /*
            if (auto* tab = tabs[tabIndex])
            return tab->colour;

        return Colours::transparentBlack;
        */
    }
    
    /**
      | Changes the background colour of a tab.
      | @see addTab, getTabBackgroundColour
      |
      */
    pub fn set_tab_background_colour(&mut self, 
        tab_index:  i32,
        new_colour: Colour)  {
        
        todo!();
        /*
            if (auto* tab = tabs [tabIndex])
        {
            if (tab->colour != newColour)
            {
                tab->colour = newColour;
                repaint();
            }
        }
        */
    }
    
    pub fn show_extra_items_menu(&mut self)  {
        
        todo!();
        /*
            PopupMenu m;

        for (int i = 0; i < tabs.size(); ++i)
        {
            auto* tab = tabs.getUnchecked(i);

            if (! tab->button->isVisible())
                m.addItem (PopupMenu::Item (tab->name)
                             .setTicked (i == currentTabIndex)
                             .setAction ([this, i] { setCurrentTabIndex (i); }));
        }

        m.showMenuAsync (PopupMenu::Options()
                            .withDeletionCheck (*this)
                            .withTargetComponent (extraTabsButton.get()));
        */
    }
    
    pub fn current_tab_changed(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn popup_menu_click_on_tab(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}
