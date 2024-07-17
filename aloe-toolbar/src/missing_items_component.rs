crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ToolbarMissingItemsComponent<'a> {
    base:        PopupMenuCustomComponent<'a>,
    owner:       ComponentSafePointer<'a,Toolbar<'a>>,
    height:      i32,
    old_indexes: Vec<i32>,
}

impl<'a> Drop for ToolbarMissingItemsComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (owner != nullptr)
            {
                for (int i = 0; i < getNumChildComponents(); ++i)
                {
                    if (auto* tc = dynamic_cast<ToolbarItemComponent*> (getChildComponent (i)))
                    {
                        tc->setVisible (false);
                        auto index = oldIndexes.removeAndReturn (i);
                        owner->addChildComponent (tc, index);
                        --i;
                    }
                }

                owner->resized();
            }
        */
    }
}

impl<'a> ToolbarMissingItemsComponent<'a> {

    pub fn new(
        bar: &mut Toolbar,
        h:   i32) -> Self {
    
        todo!();
        /*


            : typename PopupMenu::CustomComponent (true),
              owner (&bar),
              height (h)

            for (int i = bar.items.size(); --i >= 0;)
            {
                auto* tc = bar.items.getUnchecked(i);

                if (tc != nullptr && dynamic_cast<ToolbarSpacer*> (tc) == nullptr && ! tc->isVisible())
                {
                    oldIndexes.insert (0, i);
                    addAndMakeVisible (tc, 0);
                }
            }

            layout (400);
        */
    }
    
    pub fn layout(&mut self, preferred_width: i32)  {
        
        todo!();
        /*
            const int indent = 8;
            auto x = indent;
            auto y = indent;
            int maxX = 0;

            for (auto* c : getChildren())
            {
                if (auto* tc = dynamic_cast<ToolbarItemComponent*> (c))
                {
                    int preferredSize = 1, minSize = 1, maxSize = 1;

                    if (tc->getToolbarItemSizes (height, false, preferredSize, minSize, maxSize))
                    {
                        if (x + preferredSize > preferredWidth && x > indent)
                        {
                            x = indent;
                            y += height;
                        }

                        tc->setBounds (x, y, preferredSize, height);

                        x += preferredSize;
                        maxX = jmax (maxX, x);
                    }
                }
            }

            setSize (maxX + 8, y + height + 8);
        */
    }
    
    pub fn get_ideal_size(&mut self, 
        ideal_width:  &mut i32,
        ideal_height: &mut i32)  {
        
        todo!();
        /*
            idealWidth = getWidth();
            idealHeight = getHeight();
        */
    }
}
