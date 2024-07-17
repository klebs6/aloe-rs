crate::ix!();

pub struct ValueListEditorWindow<'a> {
    base:          DocumentWindow<'a>,
    base2:         DeletedAtShutdown,
    viewport:      Viewport<'a>,
    look_and_feel: LookAndFeel_V3<'a>,
}

impl<'a> Drop for ValueListEditorWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            setLookAndFeel (nullptr);
        */
    }
}

impl<'a> ValueListEditorWindow<'a> {

    pub fn new(list: &mut ValueList) -> Self {
    
        todo!();
        /*


            : DocumentWindow ("Live Values", Colours::lightgrey, DocumentWindow::closeButton)

                setLookAndFeel (&lookAndFeel);
                setUsingNativeTitleBar (true);

                viewport.setViewedComponent (new ValueListHolderComponent (list), true);
                viewport.setSize (700, 600);
                viewport.setScrollBarsShown (true, false);

                setContentNonOwned (&viewport, true);
                setResizable (true, false);
                setResizeLimits (500, 400, 10000, 10000);
                centreWithSize (getWidth(), getHeight());
                setVisible (true);
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            setVisible (false);
        */
    }
    
    pub fn update_items(&mut self, list: &mut ValueList)  {
        
        todo!();
        /*
            if (auto* l = dynamic_cast<ValueListHolderComponent*> (viewport.getViewedComponent()))
                {
                    while (l->getNumChildComponents() < list.values.size())
                    {
                        if (auto* v = list.values [l->getNumChildComponents()])
                            l->addItem (viewport.getMaximumVisibleWidth(), *v, list.getDocument (v->sourceFile));
                        else
                            break;
                    }

                    setVisible (true);
                }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            DocumentWindow::resized();

                if (auto* l = dynamic_cast<ValueListHolderComponent*> (viewport.getViewedComponent()))
                    l->layout (viewport.getMaximumVisibleWidth());
        */
    }
}
