crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CustomMenuBarItemHolder<'a> {
    base:   Component<'a>,
    custom: ReferenceCountedObjectPtr<PopupMenuCustomComponent<'a>>,
}

impl<'a> CustomMenuBarItemHolder<'a> {

    pub fn new(custom_component: &ReferenceCountedObjectPtr<PopupMenuCustomComponent>) -> Self {
    
        todo!();
        /*


            setInterceptsMouseClicks (false, true);
            update (customComponent);
        */
    }
    
    pub fn update(&mut self, new_component: &ReferenceCountedObjectPtr<PopupMenuCustomComponent>)  {
        
        todo!();
        /*
            jassert (newComponent != nullptr);

            if (newComponent != custom)
            {
                if (custom != nullptr)
                    removeChildComponent (custom.get());

                custom = newComponent;
                addAndMakeVisible (*custom);
                resized();
            }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            custom->setBounds (getLocalBounds());
        */
    }
}


