crate::ix!();

pub struct MultiDocumentPanelTabbedComponentInternal<'a> {
    base: TabbedComponent<'a>,
}

impl<'a> Default for MultiDocumentPanelTabbedComponentInternal<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : tabbed_component(TabbedButtonBar::TabsAtTop),

        
        */
    }
}

impl<'a> MultiDocumentPanelTabbedComponentInternal<'a> {

    pub fn current_tab_changed(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
            if (auto* owner = findParentComponentOfClass<MultiDocumentPanel>())
                    owner->updateOrder();
        */
    }
}

