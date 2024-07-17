crate::ix!();

/**
  | A panel which holds a vertical stack
  | of components which can be expanded
  | and contracted.
  | 
  | Each section has its own header bar which
  | can be dragged up and down to resize it,
  | or double-clicked to fully expand that
  | section.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ConcertinaPanel<'a> {
    base:          Component<'a>,
    current_sizes: Box<ConcertinaPanelSizes<'a>>,
    holders:       Vec<Box<ConcertinaPanelHolder<'a>>>,
    animator:      ComponentAnimator<'a>,
    header_height: i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ConcertinaPanel.cpp]
impl<'a> Default for ConcertinaPanel<'a> {
       
    /**
      | Creates an empty concertina panel.
      | 
      | You can call addPanel() to add some components
      | to it.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            : currentSizes (new ConcertinaPanelSizes()),
          headerHeight (20)
        */
    }
}

impl<'a> ConcertinaPanel<'a> {

    /**
      | Returns the number of panels. @see getPanel
      |
      */
    pub fn get_num_panels(&self) -> i32 {
        
        todo!();
        /*
            return holders.size();
        */
    }
    
    /**
      | Returns one of the panels. @see getNumPanels()
      |
      */
    pub fn get_panel(&self, index: i32) -> *mut Component {
        
        todo!();
        /*
            if (PanelHolder* h = holders[index])
            return h->component;

        return nullptr;
        */
    }
    
    /**
      | Adds a component to the panel.
      | 
      | -----------
      | @param insertIndex
      | 
      | the index at which this component will
      | be inserted, or
      | 
      | -1 to append it to the end of the list.
      | ----------
      | @param component
      | 
      | the component that will be shown
      | ----------
      | @param takeOwnership
      | 
      | if true, then the ConcertinaPanel will
      | take ownership of the content component,
      | and will delete it later when it's no
      | longer needed. If false, it won't delete
      | it, and you must make sure it doesn't
      | get deleted while in use.
      |
      */
    pub fn add_panel(&mut self, 
        insert_index:   i32,
        component:      *mut Component,
        take_ownership: bool)  {
        
        todo!();
        /*
            jassert (component != nullptr); // can't use a null pointer here!
        jassert (indexOfComp (component) < 0); // You can't add the same component more than once!

        auto holder = new PanelHolder (component, takeOwnership);
        holders.insert (insertIndex, holder);
        currentSizes->sizes.insert (insertIndex, ConcertinaPanelSizes::Panel (headerHeight, headerHeight, std::numeric_limits<int>::max()));
        addAndMakeVisible (holder);
        resized();
        */
    }
    
    /**
      | Removes one of the panels.
      | 
      | If the takeOwnership flag was set when
      | the panel was added, then this will also
      | delete the component.
      |
      */
    pub fn remove_panel(&mut self, component: *mut Component)  {
        
        todo!();
        /*
            auto index = indexOfComp (component);

        if (index >= 0)
        {
            currentSizes->sizes.remove (index);
            holders.remove (index);
            resized();
        }
        */
    }
    
    /**
      | Resizes one of the panels.
      | 
      | The panelComponent must point to a valid
      | panel component.
      | 
      | If animate is true, the panels will be
      | animated into their new positions;
      | if false, they will just be immediately
      | resized.
      |
      */
    pub fn set_panel_size(&mut self, 
        panel_component: *mut Component,
        height:          i32,
        animate:         bool) -> bool {
        
        todo!();
        /*
            auto index = indexOfComp (panelComponent);
        jassert (index >= 0); // The specified component doesn't seem to have been added!

        height += currentSizes->get(index).minSize;
        auto oldSize = currentSizes->get(index).size;
        setLayout (currentSizes->withResizedPanel (index, height, getHeight()), animate);
        return oldSize != currentSizes->get(index).size;
        */
    }
    
    /**
      | Attempts to make one of the panels full-height.
      | 
      | The panelComponent must point to a valid
      | panel component.
      | 
      | If this component has had a maximum size
      | set, then it will be expanded to that
      | size. Otherwise, it'll fill as much
      | of the total space as possible.
      |
      */
    pub fn expand_panel_fully(&mut self, 
        component: *mut Component,
        animate:   bool) -> bool {
        
        todo!();
        /*
            return setPanelSize (component, getHeight(), animate);
        */
    }
    
    /**
      | Sets a maximum size for one of the panels.
      |
      */
    pub fn set_maximum_panel_size(&mut self, 
        component:    *mut Component,
        maximum_size: i32)  {
        
        todo!();
        /*
            auto index = indexOfComp (component);
        jassert (index >= 0); // The specified component doesn't seem to have been added!

        if (index >= 0)
        {
            currentSizes->get(index).maxSize = currentSizes->get(index).minSize + maximumSize;
            resized();
        }
        */
    }
    
    /**
      | Sets the height of the header section
      | for one of the panels.
      |
      */
    pub fn set_panel_header_size(&mut self, 
        component:   *mut Component,
        header_size: i32)  {
        
        todo!();
        /*
            auto index = indexOfComp (component);
        jassert (index >= 0); // The specified component doesn't seem to have been added!

        if (index >= 0)
        {
            auto oldMin = currentSizes->get (index).minSize;

            currentSizes->get (index).minSize = headerSize;
            currentSizes->get (index).size += headerSize - oldMin;
            resized();
        }
        */
    }
    
    /**
      | Sets a custom header Component for one
      | of the panels.
      | 
      | -----------
      | @param panelComponent
      | 
      | the panel component to add the custom
      | header to.
      | ----------
      | @param customHeaderComponent
      | 
      | the custom component to use for the panel
      | header.
      | 
      | This can be nullptr to clear the custom
      | header component and just use the standard
      | LookAndFeel panel.
      | ----------
      | @param takeOwnership
      | 
      | if true, then the PanelHolder will take
      | ownership of the custom header component,
      | and will delete it later when it's no
      | longer needed. If false, it won't delete
      | it, and you must make sure it doesn't
      | get deleted while in use.
      |
      */
    pub fn set_custom_panel_header(&mut self, 
        component:        *mut Component,
        custom_component: *mut Component,
        take_ownership:   bool)  {
        
        todo!();
        /*
            OptionalScopedPointer<Component> optional (customComponent, takeOwnership);

        auto index = indexOfComp (component);
        jassert (index >= 0); // The specified component doesn't seem to have been added!

        if (index >= 0)
            holders.getUnchecked (index)->setCustomHeaderComponent (optional.release(), takeOwnership);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            applyLayout (getFittedSizes(), false);
        */
    }
    
    pub fn index_of_comp(&self, comp: *mut Component) -> i32 {
        
        todo!();
        /*
            for (int i = 0; i < holders.size(); ++i)
            if (holders.getUnchecked(i)->component == comp)
                return i;

        return -1;
        */
    }
    
    pub fn get_fitted_sizes(&self) -> ConcertinaPanelSizes {
        
        todo!();
        /*
            return currentSizes->fittedInto (getHeight());
        */
    }
    
    pub fn apply_layout(&mut self, 
        sizes:   &ConcertinaPanelSizes,
        animate: bool)  {
        
        todo!();
        /*
            if (! animate)
            animator.cancelAllAnimations (false);

        const int animationDuration = 150;
        auto w = getWidth();
        int y = 0;

        for (int i = 0; i < holders.size(); ++i)
        {
            PanelHolder& p = *holders.getUnchecked (i);

            auto h = sizes.get (i).size;
            const Rectangle<int> pos (0, y, w, h);

            if (animate)
                animator.animateComponent (&p, pos, 1.0f, animationDuration, false, 1.0, 1.0);
            else
                p.setBounds (pos);

            y += h;
        }
        */
    }
    
    pub fn set_layout(&mut self, 
        sizes:   &ConcertinaPanelSizes,
        animate: bool)  {
        
        todo!();
        /*
            *currentSizes = sizes;
        applyLayout (getFittedSizes(), animate);
        */
    }
    
    pub fn panel_header_double_clicked(&mut self, component: *mut Component)  {
        
        todo!();
        /*
            if (! expandPanelFully (component, true))
            setPanelSize (component, 0, true);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}
