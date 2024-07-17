crate::ix!();

/**
  | When adding an extra component to the
  | tab, this indicates which side of the
  | text it should be placed on.
  |
  */
pub enum TabBarButtonExtraComponentPlacement
{
    beforeText,
    afterText
}

/**
  | In a TabbedButtonBar, this component
  | is used for each of the buttons.
  | 
  | If you want to create a TabbedButtonBar
  | with custom tab components, derive
  | your component from this class, and
  | override the TabbedButtonBar::createTabButton()
  | method to create it instead of the default
  | one.
  | 
  | @see TabbedButtonBar
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TabBarButton<'a> {
    base:                 Button<'a>,
    owner:                &'a mut TabbedButtonBar<'a>,
    overlap_pixels:       i32, // default = 0
    extra_component:      Box<Component<'a>>,
    extra_comp_placement: TabBarButtonExtraComponentPlacement, // default = afterText
}

impl<'a> TabBarButton<'a> {

    /**
      | Returns the bar that contains this button.
      |
      */
    pub fn get_tabbed_button_bar(&self) -> &mut TabbedButtonBar {
        
        todo!();
        /*
            return owner;
        */
    }

    /**
      | Returns the custom component, if there
      | is one.
      |
      */
    pub fn get_extra_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return extraComponent.get();
        */
    }

    /**
      | Returns the placement of the custom
      | component, if there is one.
      |
      */
    pub fn get_extra_component_placement(&self) -> TabBarButtonExtraComponentPlacement {
        
        todo!();
        /*
            return extraCompPlacement;
        */
    }

    /**
      | Creates the tab button.
      |
      */
    pub fn new(
        name: &String,
        bar:  &mut TabbedButtonBar) -> Self {
    
        todo!();
        /*
        : button(name),
        : owner(bar),

            setWantsKeyboardFocus (false);
        */
    }
    
    /**
      | Returns this tab's index in its tab bar.
      |
      */
    pub fn get_index(&self) -> i32 {
        
        todo!();
        /*
            return owner.indexOfTabButton (this);
        */
    }
    
    /**
      | Returns the colour of the tab.
      |
      */
    pub fn get_tab_background_colour(&self) -> Colour {
        
        todo!();
        /*
            return owner.getTabBackgroundColour (getIndex());
        */
    }
    
    /**
      | Returns true if this is the frontmost
      | (selected) tab.
      |
      */
    pub fn is_front_tab(&self) -> bool {
        
        todo!();
        /*
            return getToggleState();
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            getLookAndFeel().drawTabButton (*this, g, shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);
        */
    }
    
    pub fn clicked(&mut self, mods: &ModifierKeys)  {
        
        todo!();
        /*
            if (mods.isPopupMenu())
            owner.popupMenuClickOnTab (getIndex(), getButtonText());
        else
            owner.setCurrentTabIndex (getIndex());
        */
    }
    
    pub fn hit_test(&mut self, mx: i32, my: i32) -> bool {
        
        todo!();
        /*
            auto area = getActiveArea();

        if (owner.isVertical())
        {
            if (isPositiveAndBelow (mx, getWidth())
                 && my >= area.getY() + overlapPixels && my < area.getBottom() - overlapPixels)
                return true;
        }
        else
        {
            if (isPositiveAndBelow (my, getHeight())
                 && mx >= area.getX() + overlapPixels && mx < area.getRight() - overlapPixels)
                return true;
        }

        Path p;
        getLookAndFeel().createTabButtonShape (*this, p, false, false);

        return p.contains ((float) (mx - area.getX()),
                           (float) (my - area.getY()));
        */
    }
    
    pub fn get_best_tab_length(&mut self, depth: i32) -> i32 {
        
        todo!();
        /*
            return getLookAndFeel().getTabButtonBestWidth (*this, depth);
        */
    }
    
    pub fn calc_areas(&self, 
        extra_comp: &mut Rectangle<i32>,
        text_area:  &mut Rectangle<i32>)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();
        textArea = getActiveArea();

        auto depth = owner.isVertical() ? textArea.getWidth() : textArea.getHeight();
        auto overlap = lf.getTabButtonOverlap (depth);

        if (overlap > 0)
        {
            if (owner.isVertical())
                textArea.reduce (0, overlap);
            else
                textArea.reduce (overlap, 0);
        }

        if (extraComponent != nullptr)
        {
            extraComp = lf.getTabButtonExtraComponentBounds (*this, textArea, *extraComponent);

            auto orientation = owner.getOrientation();

            if (orientation == TabbedButtonBar::TabsAtLeft || orientation == TabbedButtonBar::TabsAtRight)
            {
                if (extraComp.getCentreY() > textArea.getCentreY())
                    textArea.setBottom (jmin (textArea.getBottom(), extraComp.getY()));
                else
                    textArea.setTop (jmax (textArea.getY(), extraComp.getBottom()));
            }
            else
            {
                if (extraComp.getCentreX() > textArea.getCentreX())
                    textArea.setRight (jmin (textArea.getRight(), extraComp.getX()));
                else
                    textArea.setLeft (jmax (textArea.getX(), extraComp.getRight()));
            }
        }
        */
    }
    
    /**
      | Returns the area of the component that
      | should contain its text.
      |
      */
    pub fn get_text_area(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            Rectangle<int> extraComp, textArea;
        calcAreas (extraComp, textArea);
        return textArea;
        */
    }
    
    /**
      | Returns an area of the component that's
      | safe to draw in.
      | 
      | This deals with the orientation of the
      | tabs, which affects which side is touching
      | the tabbed box's content component.
      |
      */
    pub fn get_active_area(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            auto r = getLocalBounds();
        auto spaceAroundImage = getLookAndFeel().getTabButtonSpaceAroundImage();
        auto orientation = owner.getOrientation();

        if (orientation != TabbedButtonBar::TabsAtLeft)      r.removeFromRight  (spaceAroundImage);
        if (orientation != TabbedButtonBar::TabsAtRight)     r.removeFromLeft   (spaceAroundImage);
        if (orientation != TabbedButtonBar::TabsAtBottom)    r.removeFromTop    (spaceAroundImage);
        if (orientation != TabbedButtonBar::TabsAtTop)       r.removeFromBottom (spaceAroundImage);

        return r;
        */
    }
    
    /**
      | Sets an extra component that will be
      | shown in the tab.
      | 
      | This optional component will be positioned
      | inside the tab, either to the left or
      | right of the text. You could use this
      | to implement things like a close button
      | or a graphical status indicator. If
      | a non-null component is passed-in,
      | the TabbedButtonBar will take ownership
      | of it and delete it when required.
      |
      */
    pub fn set_extra_component(&mut self, 
        comp:      *mut Component,
        placement: TabBarButtonExtraComponentPlacement)  {
        
        todo!();
        /*
            jassert (extraCompPlacement == beforeText || extraCompPlacement == afterText);
        extraCompPlacement = placement;
        extraComponent.reset (comp);
        addAndMakeVisible (extraComponent.get());
        resized();
        */
    }
    
    pub fn child_bounds_changed(&mut self, c: *mut Component)  {
        
        todo!();
        /*
            if (c == extraComponent.get())
        {
            owner.resized();
            resized();
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (extraComponent != nullptr)
        {
            Rectangle<int> extraComp, textArea;
            calcAreas (extraComp, textArea);

            if (! extraComp.isEmpty())
                extraComponent->setBounds (extraComp);
        }
        */
    }
}
