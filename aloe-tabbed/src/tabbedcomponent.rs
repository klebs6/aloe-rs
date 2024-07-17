crate::ix!();

pub fn tabbed_component_delete_component_id() -> Identifier {
    Identifier::new("deleteByTabComp_")
}

pub fn tabbed_component_delete_if_necessary<'a>(comp: *mut Component<'a>) {
    
    todo!();
    /*
        if (comp != nullptr && (bool) comp->getProperties() [deleteComponentId])
                delete comp;
    */
}

pub fn tabbed_component_get_tab_area(
    content:     &mut Rectangle<i32>,
    outline:     &mut BorderSize<i32>,
    orientation: TabbedButtonBarOrientation,
    tab_depth:   i32) -> Rectangle<i32> {

    todo!();
    /*
        switch (orientation)
            {
                case TabbedButtonBar::TabsAtTop:    outline.setTop (0);     return content.removeFromTop (tabDepth);
                case TabbedButtonBar::TabsAtBottom: outline.setBottom (0);  return content.removeFromBottom (tabDepth);
                case TabbedButtonBar::TabsAtLeft:   outline.setLeft (0);    return content.removeFromLeft (tabDepth);
                case TabbedButtonBar::TabsAtRight:  outline.setRight (0);   return content.removeFromRight (tabDepth);
                default: jassertfalse; break;
            }

            return Rectangle<int>();
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_TabbedComponent.h]
    
/**
  | A component with a TabbedButtonBar
  | along one of its sides.
  | 
  | This makes it easy to create a set of tabbed
  | pages, just add a bunch of tabs with addTab(),
  | and this will take care of showing the
  | pages for you when the user clicks on
  | a different tab.
  | 
  | @see TabbedButtonBar
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TabbedComponent<'a> {
    base:               Component<'a>,
    tabs:               Box<TabbedButtonBar<'a>>,
    content_components: Vec<WeakReference<Component<'a>>>,
    panel_component:    WeakReference<Component<'a>>,
    tab_depth:          i32, // default = 30
    outline_thickness:  i32, // default = 1
    edge_indent:        i32, // default = 0
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_TabbedComponent.cpp]
impl<'a> Drop for TabbedComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        clearTabs();
        tabs.reset();
 */
    }
}

impl<'a> TabbedComponent<'a> {
    
    /**
      | Returns the current thickness of the
      | tab bar. @see setTabBarDepth
      |
      */
    pub fn get_tab_bar_depth(&self) -> i32 {
        
        todo!();
        /*
            return tabDepth;
        */
    }

    /**
      | Returns the current component that's
      | filling the panel.
      | 
      | This will return nullptr if there isn't
      | one.
      |
      */
    pub fn get_current_content_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return panelComponent.get();
        */
    }


    /**
      | Returns the tab button bar component
      | that is being used.
      |
      */
    pub fn get_tabbed_button_bar(&self) -> &mut TabbedButtonBar {
        
        todo!();
        /*
            return *tabs;
        */
    }
    
    /**
      | Creates a TabbedComponent, specifying
      | where the tabs should be placed.
      | 
      | Once created, add some tabs with the
      | addTab() method.
      |
      */
    pub fn new(orientation: TabbedButtonBarOrientation) -> Self {
    
        todo!();
        /*


            tabs.reset (new TabbedComponentButtonBar (*this, orientation));
        addAndMakeVisible (tabs.get());
        */
    }
    
    /**
      | Changes the placement of the tabs.
      | 
      | This will rearrange the layout to place
      | the tabs along the appropriate side
      | of this component, and will shift the
      | content component accordingly.
      | 
      | @see TabbedButtonBar::setOrientation
      |
      */
    pub fn set_orientation(&mut self, orientation: TabbedButtonBarOrientation)  {
        
        todo!();
        /*
            tabs->setOrientation (orientation);
        resized();
        */
    }
    
    /**
      | Returns the current tab placement.
      | @see setOrientation, TabbedButtonBar::getOrientation
      |
      */
    pub fn get_orientation(&self) -> TabbedButtonBarOrientation {
        
        todo!();
        /*
            return tabs->getOrientation();
        */
    }
    
    /**
      | Specifies how many pixels wide or high
      | the tab-bar should be.
      | 
      | If the tabs are placed along the top or
      | bottom, this specified the height of
      | the bar; if they're along the left or
      | right edges, it'll be the width of the
      | bar.
      |
      */
    pub fn set_tab_bar_depth(&mut self, new_depth: i32)  {
        
        todo!();
        /*
            if (tabDepth != newDepth)
        {
            tabDepth = newDepth;
            resized();
        }
        */
    }
    
    pub fn create_tab_button(&mut self, 
        tab_name:  &String,
        tab_index: i32) -> *mut TabBarButton {
        
        todo!();
        /*
            return new TabBarButton (tabName, *tabs);
        */
    }
    
    /**
      | Removes all the tabs from the bar. @see
      | TabbedButtonBar::clearTabs
      |
      */
    pub fn clear_tabs(&mut self)  {
        
        todo!();
        /*
            if (panelComponent != nullptr)
        {
            panelComponent->setVisible (false);
            removeChildComponent (panelComponent.get());
            panelComponent = nullptr;
        }

        tabs->clearTabs();

        for (int i = contentComponents.size(); --i >= 0;)
            TabbedComponentHelpers::deleteIfNecessary (contentComponents.getReference (i));

        contentComponents.clear();
        */
    }
    
    /**
      | Adds a tab to the tab-bar.
      | 
      | The component passed in will be shown
      | for the tab. If deleteComponentWhenNotNeeded
      | is true, then the TabbedComponent will
      | take ownership of the component and
      | will delete it when the tab is removed
      | or when this object is deleted.
      | 
      | @see TabbedButtonBar::addTab
      |
      */
    pub fn add_tab(&mut self, 
        tab_name:                         &String,
        tab_background_colour:            Colour,
        content_component:                *mut Component<'a>,
        delete_component_when_not_needed: bool,
        insert_index:                     Option<i32>)  {

        let insert_index: i32 = insert_index.unwrap_or(-1);
        
        todo!();
        /*
            contentComponents.insert (insertIndex, WeakReference<Component> (contentComponent));

        if (deleteComponentWhenNotNeeded && contentComponent != nullptr)
            contentComponent->getProperties().set (TabbedComponentHelpers::deleteComponentId, true);

        tabs->addTab (tabName, tabBackgroundColour, insertIndex);
        resized();
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
            tabs->setTabName (tabIndex, newName);
        */
    }
    
    /**
      | Gets rid of one of the tabs.
      |
      */
    pub fn remove_tab(&mut self, tab_index: i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (tabIndex, contentComponents.size()))
        {
            TabbedComponentHelpers::deleteIfNecessary (contentComponents.getReference (tabIndex).get());
            contentComponents.remove (tabIndex);
            tabs->removeTab (tabIndex);
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
            contentComponents.move (currentIndex, newIndex);
        tabs->moveTab (currentIndex, newIndex, animate);
        */
    }
    
    /**
      | Returns the number of tabs in the bar.
      |
      */
    pub fn get_num_tabs(&self) -> i32 {
        
        todo!();
        /*
            return tabs->getNumTabs();
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
            return tabs->getTabNames();
        */
    }
    
    /**
      | Returns the content component that
      | was added for the given index.
      | 
      | Be careful not to reposition or delete
      | the components that are returned, as
      | this will interfere with the TabbedComponent's
      | behaviour.
      |
      */
    pub fn get_tab_content_component(&self, tab_index: i32) -> *mut Component<'a> {
        
        todo!();
        /*
            return contentComponents[tabIndex].get();
        */
    }
    
    /**
      | Returns the colour of one of the tabs.
      |
      */
    pub fn get_tab_background_colour(&self, tab_index: i32) -> Colour {
        
        todo!();
        /*
            return tabs->getTabBackgroundColour (tabIndex);
        */
    }
    
    /**
      | Changes the background colour of one
      | of the tabs.
      |
      */
    pub fn set_tab_background_colour(&mut self, 
        tab_index:  i32,
        new_colour: Colour)  {
        
        todo!();
        /*
            tabs->setTabBackgroundColour (tabIndex, newColour);

        if (getCurrentTabIndex() == tabIndex)
            repaint();
        */
    }
    
    /**
      | Changes the currently-selected tab.
      | 
      | To deselect all the tabs, pass -1 as the
      | index. @see TabbedButtonBar::setCurrentTabIndex
      |
      */
    pub fn set_current_tab_index(&mut self, 
        new_tab_index:       i32,
        send_change_message: Option<bool>)  {

        let send_change_message: bool = send_change_message.unwrap_or(true);
        
        todo!();
        /*
            tabs->setCurrentTabIndex (newTabIndex, sendChangeMessage);
        */
    }
    
    /**
      | Returns the index of the currently selected
      | tab. @see addTab, TabbedButtonBar::getCurrentTabIndex()
      |
      */
    pub fn get_current_tab_index(&self) -> i32 {
        
        todo!();
        /*
            return tabs->getCurrentTabIndex();
        */
    }
    
    /**
      | Returns the name of the currently selected
      | tab. @see addTab, TabbedButtonBar::getCurrentTabName()
      |
      */
    pub fn get_current_tab_name(&self) -> String {
        
        todo!();
        /*
            return tabs->getCurrentTabName();
        */
    }
    
    /**
      | Specifies the thickness of an outline
      | that should be drawn around the content
      | component.
      | 
      | If this thickness is > 0, a line will be
      | drawn around the three sides of the content
      | component which don't touch the tab-bar,
      | and the content component will be inset
      | by this amount.
      | 
      | To set the colour of the line, use setColour
      | (outlineColourId, ...).
      |
      */
    pub fn set_outline(&mut self, thickness: i32)  {
        
        todo!();
        /*
            outlineThickness = thickness;
        resized();
        repaint();
        */
    }
    
    /**
      | Specifies a gap to leave around the edge
      | of the content component.
      | 
      | Each edge of the content component will
      | be indented by the given number of pixels.
      |
      */
    pub fn set_indent(&mut self, indent_thickness: i32)  {
        
        todo!();
        /*
            edgeIndent = indentThickness;
        resized();
        repaint();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (backgroundColourId));

        auto content = getLocalBounds();
        BorderSize<int> outline (outlineThickness);
        TabbedComponentHelpers::getTabArea (content, outline, getOrientation(), tabDepth);

        g.reduceClipRegion (content);
        g.fillAll (tabs->getTabBackgroundColour (getCurrentTabIndex()));

        if (outlineThickness > 0)
        {
            RectangleList<int> rl (content);
            rl.subtract (outline.subtractedFrom (content));

            g.reduceClipRegion (rl);
            g.fillAll (findColour (outlineColourId));
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto content = getLocalBounds();
        BorderSize<int> outline (outlineThickness);

        tabs->setBounds (TabbedComponentHelpers::getTabArea (content, outline, getOrientation(), tabDepth));
        content = BorderSize<int> (edgeIndent).subtractedFrom (outline.subtractedFrom (content));

        for (auto& c : contentComponents)
            if (auto comp = c.get())
                comp->setBounds (content);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            for (auto& c : contentComponents)
            if (auto comp = c.get())
              comp->lookAndFeelChanged();
        */
    }
    
    pub fn change_callback(&mut self, 
        new_current_tab_index: i32,
        new_tab_name:          &String)  {
        
        todo!();
        /*
            auto* newPanelComp = getTabContentComponent (getCurrentTabIndex());

        if (newPanelComp != panelComponent)
        {
            if (panelComponent != nullptr)
            {
                panelComponent->setVisible (false);
                removeChildComponent (panelComponent);
            }

            panelComponent = newPanelComp;

            if (panelComponent != nullptr)
            {
                // do these ops as two stages instead of addAndMakeVisible() so that the
                // component has always got a parent when it gets the visibilityChanged() callback
                addChildComponent (panelComponent);
                panelComponent->sendLookAndFeelChange();
                panelComponent->setVisible (true);
                panelComponent->toFront (true);
            }

            repaint();
        }

        resized();
        currentTabChanged (newCurrentTabIndex, newTabName);
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
