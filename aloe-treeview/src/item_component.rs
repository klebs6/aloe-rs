crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TreeViewItemComponent<'a> {
    base:                 Component<'a>,
    item:                 &'a mut TreeViewItem<'a>,
    custom_component:     Box<Component<'a>>,
    mouse_is_over_button: bool, // default = false
}

impl<'a> TreeViewItemComponent<'a> {

    pub fn new(item_to_represent: &mut TreeViewItem) -> Self {
    
        todo!();
        /*
        : item(itemToRepresent),
        : custom_component(item.createItemComponent()),

            if (hasCustomComponent())
                addAndMakeVisible (*customComponent);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            item.draw (g, getWidth(), mouseIsOverButton);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (hasCustomComponent())
            {
                auto itemPosition = item.getItemPosition (false);

                customComponent->setBounds (getLocalBounds().withX (itemPosition.getX())
                                                            .withWidth (itemPosition.getWidth()));
            }
        */
    }
    
    pub fn set_mouse_is_over_button(&mut self, is_over: bool)  {
        
        todo!();
        /*
            mouseIsOverButton = isOver;
        */
    }
    
    pub fn get_represented_item(&self) -> &mut TreeViewItem {
        
        todo!();
        /*
            return item;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            if (hasCustomComponent() && customComponent->getAccessibilityHandler() != nullptr)
                return nullptr;

            return std::make_unique<TreeViewItemComponentItemAccessibilityHandler> (*this);
        */
    }
    
    pub fn has_custom_component(&self) -> bool {
        
        todo!();
        /*
            return customComponent.get() != nullptr;
        */
    }
}
