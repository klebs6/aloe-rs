crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_PropertyPanel.h]

/**
  | A panel that holds a list of PropertyComponent
  | objects.
  | 
  | This panel displays a list of PropertyComponents,
  | and allows them to be organised into
  | collapsible sections.
  | 
  | To use, simply create one of these and
  | add your properties to it with addProperties()
  | or addSection().
  | 
  | @see PropertyComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct PropertyPanel<'a> {
    base:                      Component<'a>,
    viewport:                  Viewport<'a>,
    property_holder_component: *mut PropertyPanelPropertyHolderComponent<'a>,
    message_when_empty:        String,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_PropertyPanel.cpp]
impl<'a> Drop for PropertyPanel<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            clear();
        */

    }
}
    
impl<'a> Default for PropertyPanel<'a> {

    /**
      | Creates an empty property panel.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            init();
        */
    }
}

impl<'a> PropertyPanel<'a> {

    /**
      | Returns the PropertyPanel's internal
      | Viewport.
      |
      */
    pub fn get_viewport(&mut self) -> &mut Viewport {
        
        todo!();
        /*
            return viewport;
        */

    }
    
    /**
      | Creates an empty property panel.
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : component(name),

            init();
        */

    }
    
    pub fn init(&mut self)  {
        
        todo!();
        /*
            messageWhenEmpty = TRANS("(nothing selected)");

        addAndMakeVisible (viewport);
        viewport.setViewedComponent (propertyHolderComponent = new PropertyPanelPropertyHolderComponent());
        viewport.setFocusContainerType (FocusContainerType::keyboardFocusContainer);
        */

    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isEmpty())
        {
            g.setColour (Colours::black.withAlpha (0.5f));
            g.setFont (14.0f);
            g.drawText (messageWhenEmpty, getLocalBounds().withHeight (30),
                        Justification::centred, true);
        }
        */

    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            viewport.setBounds (getLocalBounds());
        updatePropHolderLayout();
        */

    }
    
    /**
      | Deletes all property components from
      | the panel.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            if (! isEmpty())
        {
            propertyHolderComponent->sections.clear();
            updatePropHolderLayout();
        }
        */

    }
    
    /**
      | Returns true if the panel contains no
      | properties.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return propertyHolderComponent->sections.size() == 0;
        */

    }
    
    /**
      | Returns the height that the panel needs
      | in order to display all of its content
      | without scrolling.
      |
      */
    pub fn get_total_content_height(&self) -> i32 {
        
        todo!();
        /*
            return propertyHolderComponent->getHeight();
        */

    }
    
    /**
      | Adds a set of properties to the panel.
      | 
      | The components in the list will be owned
      | by this object and will be automatically
      | deleted later on when no longer needed.
      | 
      | These properties are added without
      | them being inside a named section. If
      | you want them to be kept together in a
      | collapsible section, use addSection()
      | instead.
      |
      */
    pub fn add_properties(
        &mut self, 
        new_properties:                   &[*mut PropertyComponent],
        extra_padding_between_components: Option<i32>

    ) {

        let extra_padding_between_components: i32 = extra_padding_between_components.unwrap_or(0);
        
        todo!();
        /*
            if (isEmpty())
            repaint();

        propertyHolderComponent->insertSection (-1, new PropertyPanelSectionComponent ({}, newProperties, true, extraPaddingBetweenComponents));
        updatePropHolderLayout();
        */

    }
    
    /**
      | Adds a set of properties to the panel.
      | 
      | These properties are added under a section
      | heading with a plus/minus button that
      | allows it to be opened and closed. If
      | indexToInsertAt is < 0 then it will be
      | added at the end of the list, or before
      | the given index if the index is non-zero.
      | 
      | The components in the list will be owned
      | by this object and will be automatically
      | deleted later on when no longer needed.
      | 
      | To add properties without them being
      | in a section, use addProperties().
      |
      */
    pub fn add_section(
        &mut self, 
        section_title:                    &String,
        new_properties:                   &[*mut PropertyComponent],
        should_be_open:                   Option<bool>,
        index_to_insert_at:               Option<i32>,
        extra_padding_between_components: Option<i32>

    ) {

        let should_be_open: bool = should_be_open.unwrap_or(true);
        let index_to_insert_at: i32 = index_to_insert_at.unwrap_or(-1);
        let extra_padding_between_components: i32 = extra_padding_between_components.unwrap_or(0);
        
        todo!();
        /*
            jassert (sectionTitle.isNotEmpty());

        if (isEmpty())
            repaint();

        propertyHolderComponent->insertSection (indexToInsertAt, new PropertyPanelSectionComponent (sectionTitle,
                                                                                       newProperties,
                                                                                       shouldBeOpen,
                                                                                       extraPaddingBetweenComponents));

        updatePropHolderLayout();
        */

    }
    
    pub fn update_prop_holder_layout(&self)  {
        
        todo!();
        /*
            auto maxWidth = viewport.getMaximumVisibleWidth();
        propertyHolderComponent->updateLayout (maxWidth);

        auto newMaxWidth = viewport.getMaximumVisibleWidth();
        if (maxWidth != newMaxWidth)
        {
            // need to do this twice because of scrollbars changing the size, etc.
            propertyHolderComponent->updateLayout (newMaxWidth);
        }
        */

    }
    
    /**
      | Calls the refresh() method of all PropertyComponents
      | in the panel
      |
      */
    pub fn refresh_all(&self)  {
        
        todo!();
        /*
            propertyHolderComponent->refreshAll();
        */

    }
    
    /**
      | Returns a list of all the names of sections
      | in the panel.
      | 
      | These are the sections that have been
      | added with addSection().
      |
      */
    pub fn get_section_names(&self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> s;

        for (auto* section : propertyHolderComponent->sections)
        {
            if (section->getName().isNotEmpty())
                s.add (section->getName());
        }

        return s;
        */

    }
    
    /**
      | Returns true if the section at this index
      | is currently open.
      | 
      | The index is from 0 up to the number of
      | items returned by getSectionNames().
      |
      */
    pub fn is_section_open(&self, section_index: i32) -> bool {
        
        todo!();
        /*
            if (auto* s = propertyHolderComponent->getSectionWithNonEmptyName (sectionIndex))
            return s->isOpen;

        return false;
        */

    }
    
    /**
      | Opens or closes one of the sections.
      | 
      | The index is from 0 up to the number of
      | items returned by getSectionNames().
      |
      */
    pub fn set_section_open(&mut self, 
        section_index:  i32,
        should_be_open: bool)  {
        
        todo!();
        /*
            if (auto* s = propertyHolderComponent->getSectionWithNonEmptyName (sectionIndex))
            s->setOpen (shouldBeOpen);
        */

    }
    
    /**
      | Enables or disables one of the sections.
      | 
      | The index is from 0 up to the number of
      | items returned by getSectionNames().
      |
      */
    pub fn set_section_enabled(&mut self, 
        section_index:     i32,
        should_be_enabled: bool)  {
        
        todo!();
        /*
            if (auto* s = propertyHolderComponent->getSectionWithNonEmptyName (sectionIndex))
            s->setEnabled (shouldBeEnabled);
        */

    }
    
    /**
      | Remove one of the sections using the
      | section index.
      | 
      | The index is from 0 up to the number of
      | items returned by getSectionNames().
      |
      */
    pub fn remove_section(&mut self, section_index: i32)  {
        
        todo!();
        /*
            if (auto* s = propertyHolderComponent->getSectionWithNonEmptyName (sectionIndex))
        {
            propertyHolderComponent->sections.removeObject (s);
            updatePropHolderLayout();
        }
        */

    }
    
    /**
      | Saves the current state of open/closed
      | sections so it can be restored later.
      | 
      | To restore this state, use restoreOpennessState().
      | @see restoreOpennessState
      |
      */
    pub fn get_openness_state(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            auto xml = std::make_unique<XmlElement> ("PROPERTYPANELSTATE");

        xml->setAttribute ("scrollPos", viewport.getViewPositionY());

        auto sections = getSectionNames();
        for (auto s : sections)
        {
            if (s.isNotEmpty())
            {
                auto* e = xml->createNewChildElement ("SECTION");
                e->setAttribute ("name", s);
                e->setAttribute ("open", isSectionOpen (sections.indexOf (s)) ? 1 : 0);
            }
        }

        return xml;
        */

    }
    
    /**
      | Restores a previously saved arrangement
      | of open/closed sections.
      | 
      | This will try to restore a snapshot of
      | the panel's state that was created by
      | the getOpennessState() method. If
      | any of the sections named in the original
      | 
      | XML aren't present, they will be ignored.
      | 
      | @see getOpennessState
      |
      */
    pub fn restore_openness_state(&mut self, xml: &XmlElement)  {
        
        todo!();
        /*
            if (xml.hasTagName ("PROPERTYPANELSTATE"))
        {
            auto sections = getSectionNames();

            for (auto* e : xml.getChildWithTagNameIterator ("SECTION"))
            {
                setSectionOpen (sections.indexOf (e->getStringAttribute ("name")),
                                e->getBoolAttribute ("open"));
            }

            viewport.setViewPosition (viewport.getViewPositionX(),
                                      xml.getIntAttribute ("scrollPos", viewport.getViewPositionY()));
        }
        */

    }
    
    /**
      | Sets a message to be displayed when there
      | are no properties in the panel.
      | 
      | The default message is "nothing selected".
      |
      */
    pub fn set_message_when_empty(&mut self, new_message: &String)  {
        
        todo!();
        /*
            if (messageWhenEmpty != newMessage)
        {
            messageWhenEmpty = newMessage;
            repaint();
        }
        */

    }
    
    /**
      | Returns the message that is displayed
      | when there are no properties. @see setMessageWhenEmpty
      |
      */
    pub fn get_message_when_empty(&self) -> &String {
        
        todo!();
        /*
            return messageWhenEmpty;
        */

    }
}
