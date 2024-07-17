crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DemoTabbedComponent<'a> {
    base: TabbedComponent<'a>,
}

impl<'a> DemoTabbedComponent<'a> {

    pub fn new(is_running_componen_transforms_demo: bool) -> Self {
    
        todo!();
        /*
        : tabbed_component(TabbedButtonBar::TabsAtTop),

            auto colour = findColour (ResizableWindow::backgroundColourId);

            addTab ("Buttons",     colour, new ButtonsPage (isRunningComponenTransformsDemo), true);
            addTab ("Sliders",     colour, new SlidersPage(),                                 true);
            addTab ("Toolbars",    colour, new ToolbarDemoComp(),                             true);
            addTab ("Misc",        colour, new MiscPage(),                                    true);
            addTab ("Menus",       colour, new MenuPage(),                                    true);
            addTab ("Tables",      colour, new TableDemoComponent(),                          true);
            addTab ("Drag & Drop", colour, new DragAndDropDemo(),                             true);

            getTabbedButtonBar().getTabButton (5)->setExtraComponent (new CustomTabButton (isRunningComponenTransformsDemo),
                                                                      TabBarButton::afterText);
        */
    }
}
