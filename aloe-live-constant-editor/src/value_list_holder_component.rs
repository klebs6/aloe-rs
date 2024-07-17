crate::ix!();

pub const VALUE_LIST_HOLDER_COMPONENT_ITEM_HEIGHT: usize = 120;

pub struct ValueListHolderComponent<'a> {
    base:       Component<'a>,
    value_list: &'a mut ValueList<'a>,
    editors:    Vec<Box<LivePropertyEditorBase<'a>>>,
}

impl<'a> ValueListHolderComponent<'a> {
    
    pub fn new(l: &mut ValueList) -> Self {
    
        todo!();
        /*
        : value_list(l),

            setVisible (true);
        */
    }
    
    pub fn add_item(&mut self, 
        width: i32,
        v:     &mut LiveValueBase,
        doc:   &mut CodeDocument)  {
        
        todo!();
        /*
            addAndMakeVisible (editors.add (v.createPropertyComponent (doc)));
            layout (width);
        */
    }
    
    pub fn layout(&mut self, width: i32)  {
        
        todo!();
        /*
            setSize (width, editors.size() * itemHeight);
            resized();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (2, 0);

            for (int i = 0; i < editors.size(); ++i)
                editors.getUnchecked(i)->setBounds (r.removeFromTop (itemHeight));
        */
    }
}
