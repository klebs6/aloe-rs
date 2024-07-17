crate::ix!();

#[derive(Default)]
#[no_copy]
pub struct PropertyPanelPropertyHolderComponent<'a> {
    base:     Component<'a>,
    sections: Vec<Box<PropertyPanelSectionComponent<'a>>>,
}

impl<'a> PropertyPanelPropertyHolderComponent<'a> {

    pub fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn update_layout(&mut self, width: i32)  {
        
        todo!();
        /*
            auto y = 0;

            for (auto* section : sections)
            {
                section->setBounds (0, y, width, section->getPreferredHeight());
                y = section->getBottom();
            }

            setSize (width, y);
            repaint();
        */

    }
    
    pub fn refresh_all(&self)  {
        
        todo!();
        /*
            for (auto* section : sections)
                section->refreshAll();
        */

    }
    
    pub fn insert_section(&mut self, 
        index_to_insert_at: i32,
        new_section:        *mut PropertyPanelSectionComponent)  {
        
        todo!();
        /*
            sections.insert (indexToInsertAt, newSection);
            addAndMakeVisible (newSection, 0);
        */

    }
    
    pub fn get_section_with_non_empty_name(&self, target_index: i32) -> *mut PropertyPanelSectionComponent {
        
        todo!();
        /*
            auto index = 0;
            for (auto* section : sections)
            {
                if (section->getName().isNotEmpty())
                    if (index++ == targetIndex)
                        return section;
            }

            return nullptr;
        */

    }
}
