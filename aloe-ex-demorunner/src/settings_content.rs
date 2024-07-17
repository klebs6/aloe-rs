crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/UI/SettingsContent.h]

pub struct SettingsContent<'a> {
    base:              Component<'a>,
    settings_viewport: Viewport<'a>,
    inner_content:     InnerContent<'a>,
}

impl<'a> SettingsContent<'a> {

    pub fn new(top_level_component: &mut MainComponent) -> Self {
    
        todo!();
        /*


            : innerContent (topLevelComponent)

            settingsViewport.setViewedComponent (&innerContent, false);
            addAndMakeVisible (settingsViewport);

            setFocusContainerType (FocusContainerType::focusContainer);
            setTitle ("DemoRunner Settings");

            setOpaque (true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            constexpr int minimumWidth  = 350;
            constexpr int minimumHeight = 550;

            auto r = getLocalBounds();
            const auto scrollBarWidth = getLookAndFeel().getDefaultScrollbarWidth();

            innerContent.setSize (jmax (r.getWidth() - scrollBarWidth, minimumWidth),
                                  jmax (r.getHeight(), minimumHeight));

            settingsViewport.setBounds (r);
        */
    }
}
