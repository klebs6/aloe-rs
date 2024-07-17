crate::ix!();

pub const SETTINGS_CONTENT_TITLE_LABEL_FONT_HEIGHT: f32 = 18.0;
pub const SETTINGS_CONTENT_ITEM_HEIGHT:             i32 = 30;
pub const SETTINGS_CONTENT_ITEM_SPACING:            i32 = 7;

pub struct GraphicsSettingsGroup<'a> {
    base:                   Component<'a>,
    base2:                  ComponentMovementWatcher<'a>,
    main_component:         &'a mut MainComponent<'a>,
    peer:                   *mut ComponentPeer<'a>, // default = nullptr
    title_label:            Label<'a>,              // { {}, "Graphics" },
    look_and_feel_label:    Label<'a>,              // { {}, "LookAndFeel:" },
    renderer_label:         Label<'a>,              // { {}, "Renderer:" };
    look_and_feel_selector: ComboBox<'a>,
    renderer_selector:      ComboBox<'a>,
    look_and_feel_names:    StringArray,
    look_and_feels:         Vec<Box<dyn LookAndFeel>>,
}

impl<'a> GraphicsSettingsGroup<'a> {

    pub fn new(comp: &mut MainComponent) -> Self {
    
        todo!();
        /*


            : ComponentMovementWatcher (&comp),
                  mainComponent (comp)

                addAndMakeVisible (titleLabel);
                titleLabel.setFont (titleLabelFontHeight);

                addLookAndFeels();

                addAndMakeVisible (lookAndFeelSelector);

                for (int i = 0; i < lookAndFeelNames.size(); ++i)
                    lookAndFeelSelector.addItem (lookAndFeelNames.getReference (i), i + 1);

                lookAndFeelSelector.setSelectedItemIndex (lookAndFeelNames.indexOf ("LookAndFeel_V4 (Dark)"));

                lookAndFeelSelector.onChange = [this]
                {
                    auto* lf = lookAndFeels.getUnchecked (lookAndFeelSelector.getSelectedItemIndex());
                    Desktop::getInstance().setDefaultLookAndFeel (lf);
                };

                addAndMakeVisible (lookAndFeelLabel);
                lookAndFeelLabel.setJustificationType (Justification::centredRight);
                lookAndFeelLabel.attachToComponent (&lookAndFeelSelector, true);

                addAndMakeVisible (rendererSelector);
                rendererSelector.onChange = [this] { mainComponent.setRenderingEngine (rendererSelector.getSelectedItemIndex()); };

                addAndMakeVisible (rendererLabel);
                rendererLabel.setJustificationType (Justification::centredRight);
                rendererLabel.attachToComponent (&rendererSelector, true);

                setFocusContainerType (FocusContainerType::focusContainer);
                setTitle ("Graphics Settings");
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                titleLabel.setBounds (bounds.removeFromTop (itemHeight));
                bounds.removeFromTop (itemSpacing);

                const auto xPos  = roundToInt ((float) bounds.getX() + ((float) bounds.getWidth() * 0.35f));
                const auto width = roundToInt ((float) bounds.getWidth() * 0.6f);

                lookAndFeelSelector.setBounds (bounds.removeFromTop (itemHeight).withWidth (width).withX (xPos));
                bounds.removeFromTop (itemSpacing);

                rendererSelector.setBounds (bounds.removeFromTop (itemHeight).withWidth (width).withX (xPos));
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0: bool,
        _1: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            auto* newPeer = mainComponent.getPeer();

                if (peer != newPeer)
                {
                    peer = newPeer;

                    if (peer != nullptr)
                        refreshRenderingEngineSelector();
                }
        */
    }
    
    pub fn refresh_rendering_engine_selector(&mut self)  {
        
        todo!();
        /*
            rendererSelector.clear (NotificationType::dontSendNotification);

                rendererSelector.addItemList (mainComponent.getRenderingEngines(), 1);
                rendererSelector.setSelectedItemIndex (mainComponent.getCurrentRenderingEngine());
        */
    }
    
    pub fn add_look_and_feels(&mut self)  {
        
        todo!();
        /*
            lookAndFeelNames.addArray ({ "LookAndFeel_V1", "LookAndFeel_V2", "LookAndFeel_V3",
                                             "LookAndFeel_V4 (Dark)", "LookAndFeel_V4 (Midnight)",
                                             "LookAndFeel_V4 (Grey)", "LookAndFeel_V4 (Light)" });

                lookAndFeels.add (new LookAndFeel_V1());
                lookAndFeels.add (new LookAndFeel_V2());
                lookAndFeels.add (new LookAndFeel_V3());
                lookAndFeels.add (new LookAndFeel_V4 (LookAndFeel_V4::getDarkColourScheme()));
                lookAndFeels.add (new LookAndFeel_V4 (LookAndFeel_V4::getMidnightColourScheme()));
                lookAndFeels.add (new LookAndFeel_V4 (LookAndFeel_V4::getGreyColourScheme()));
                lookAndFeels.add (new LookAndFeel_V4 (LookAndFeel_V4::getLightColourScheme()));
        */
    }
}
