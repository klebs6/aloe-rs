crate::ix!();

#[no_copy]
pub struct PropertyPanelSectionComponent<'a> {
    base:           Component<'a>,
    property_comps: Vec<Box<PropertyComponent<'a>>>,
    title_height:   i32,
    is_open:        bool,
    padding:        i32,
}

impl<'a> Drop for PropertyPanelSectionComponent<'a> {
    fn drop(&mut self) {
        todo!();
        /*
            propertyComps.clear();
        */

    }
}

impl<'a> PropertyPanelSectionComponent<'a> {

    pub fn new(
        section_title:   &String,
        new_properties:  &[*mut PropertyComponent],
        section_is_open: bool,
        extra_padding:   i32) -> Self {
    
        todo!();
        /*
        : component(sectionTitle),
        : is_open(sectionIsOpen),
        : padding(extraPadding),

            lookAndFeelChanged();

            propertyComps.addArray (newProperties);

            for (auto* propertyComponent : propertyComps)
            {
                addAndMakeVisible (propertyComponent);
                propertyComponent->refresh();
            }
        */

    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (titleHeight > 0)
                getLookAndFeel().drawPropertyPanelSectionHeader (g, getName(), isOpen, getWidth(), titleHeight);
        */

    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto y = titleHeight;

            for (auto* propertyComponent : propertyComps)
            {
                propertyComponent->setBounds (1, y, getWidth() - 2, propertyComponent->getPreferredHeight());
                y = propertyComponent->getBottom() + padding;
            }
        */

    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            titleHeight = getLookAndFeel().getPropertyPanelSectionHeaderHeight (getName());
            resized();
            repaint();
        */

    }
    
    pub fn get_preferred_height(&self) -> i32 {
        
        todo!();
        /*
            auto y = titleHeight;

            auto numComponents = propertyComps.size();

            if (numComponents > 0 && isOpen)
            {
                for (auto* propertyComponent : propertyComps)
                    y += propertyComponent->getPreferredHeight();

                y += (numComponents - 1) * padding;
            }

            return y;
        */

    }
    
    pub fn set_open(&mut self, open: bool)  {
        
        todo!();
        /*
            if (isOpen != open)
            {
                isOpen = open;

                for (auto* propertyComponent : propertyComps)
                    propertyComponent->setVisible (open);

                if (auto* propertyPanel = findParentComponentOfClass<PropertyPanel>())
                    propertyPanel->resized();
            }
        */

    }
    
    pub fn refresh_all(&self)  {
        
        todo!();
        /*
            for (auto* propertyComponent : propertyComps)
                propertyComponent->refresh();
        */

    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.getMouseDownX() < titleHeight
                  && e.x < titleHeight
                  && e.getNumberOfClicks() != 2)
                mouseDoubleClick (e);
        */

    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.y < titleHeight)
                setOpen (! isOpen);
        */

    }
}
