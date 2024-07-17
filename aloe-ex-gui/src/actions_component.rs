crate::ix!();

pub struct ActionsComponent<'a> {
    base: Component<'a>,
}

impl<'a> ActionsComponent<'a> {

    pub fn new(owner: &mut CustomWidgetComponent) -> Self {
    
        todo!();
        /*
        : custom_widget_component(owner),

            addAndMakeVisible (press);
                addAndMakeVisible (toggle);
                addAndMakeVisible (focus);
                addAndMakeVisible (showMenu);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };
                grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (1)) };

                grid.items = { GridItem (press).withMargin (2), GridItem (toggle).withMargin (2),
                               GridItem (focus).withMargin (2), GridItem (showMenu).withMargin (2), };

                grid.performLayout (getLocalBounds());
        */
    }
    
    pub fn get_actions(&mut self) -> AccessibilityActions {
        
        todo!();
        /*
            AccessibilityActions result;

                if (press.isActionEnabled())     result.addAction (AccessibilityActionType::press,    [this] { press.onAction(); });
                if (toggle.isActionEnabled())    result.addAction (AccessibilityActionType::toggle,   [this] { toggle.onAction(); });
                if (focus.isActionEnabled())     result.addAction (AccessibilityActionType::focus,    [this] { focus.onAction(); });
                if (showMenu.isActionEnabled())  result.addAction (AccessibilityActionType::showMenu, [this] { showMenu.onAction(); });

                return result;
        */
    }
}
