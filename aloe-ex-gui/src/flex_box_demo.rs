crate::ix!();

#[no_copy]
#[leak_detector]
pub struct FlexBoxDemo<'a> {
    base:     Component<'a>,
    flex_box: FlexBox<'a>,
    panels:   Vec<Box<DemoFlexPanel<'a>>>,
    controls: Vec<Box<Component<'a>>>,
    buttons:  Vec<Box<ToggleButton<'a>>>,
}

impl<'a> Default for FlexBoxDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setupPropertiesPanel();
            setupFlexBoxItems();

            setSize (1000, 500)
        */
    }
}

impl<'a> FlexBoxDemo<'a> {
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            flexBox.performLayout (getFlexBoxBounds());
        */
    }
    
    pub fn get_flex_box_bounds(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return getLocalBounds().withTrimmedLeft (300)
                                   .reduced (10)
                                   .toFloat();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                               Colours::lightgrey));
            g.setColour (Colours::white);
            g.fillRect (getFlexBoxBounds());
        */
    }
    
    pub fn setup_properties_panel(&mut self)  {
        
        todo!();
        /*
            auto directionGroup = addControl (new GroupComponent ("direction", "flex-direction"));
            directionGroup->setBounds (10, 30, 140, 110);

            int i = 0;
            int groupID    = 1234;
            int leftMargin = 15;
            int topMargin  = 45;

            createToggleButton ("row",            groupID, leftMargin, topMargin + i++ * 22, true,  [this] { flexBox.flexDirection = FlexBox::Direction::row; }).setToggleState (true, dontSendNotification);
            createToggleButton ("row-reverse",    groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.flexDirection = FlexBox::Direction::rowReverse; });
            createToggleButton ("column",         groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.flexDirection = FlexBox::Direction::column; });
            createToggleButton ("column-reverse", groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.flexDirection = FlexBox::Direction::columnReverse; });

            auto wrapGroup = addControl (new GroupComponent ("wrap", "flex-wrap"));
            wrapGroup->setBounds (160, 30, 140, 110);

            i = 0;
            ++groupID;
            leftMargin = 165;

            createToggleButton ("nowrap",       groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.flexWrap = FlexBox::Wrap::noWrap; });
            createToggleButton ("wrap",         groupID, leftMargin, topMargin + i++ * 22, true,  [this] { flexBox.flexWrap = FlexBox::Wrap::wrap; });
            createToggleButton ("wrap-reverse", groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.flexWrap = FlexBox::Wrap::wrapReverse; });

            auto justifyGroup = addControl (new GroupComponent ("justify", "justify-content"));
            justifyGroup->setBounds (10, 150, 140, 140);

            i = 0;
            ++groupID;
            leftMargin = 15;
            topMargin  = 165;

            createToggleButton ("flex-start",    groupID, leftMargin, topMargin + i++ * 22, true,  [this] { flexBox.justifyContent = FlexBox::JustifyContent::flexStart; });
            createToggleButton ("flex-end",      groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.justifyContent = FlexBox::JustifyContent::flexEnd; });
            createToggleButton ("center",        groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.justifyContent = FlexBox::JustifyContent::center; });
            createToggleButton ("space-between", groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.justifyContent = FlexBox::JustifyContent::spaceBetween; });
            createToggleButton ("space-around",  groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.justifyContent = FlexBox::JustifyContent::spaceAround; });

            auto alignGroup = addControl (new GroupComponent ("align", "align-items"));
            alignGroup->setBounds (160, 150, 140, 140);

            i = 0;
            ++groupID;
            leftMargin = 165;
            topMargin  = 165;

            createToggleButton ("stretch",    groupID, leftMargin, topMargin + i++ * 22, true,  [this] { flexBox.alignItems = FlexBox::AlignItems::stretch; });
            createToggleButton ("flex-start", groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignItems = FlexBox::AlignItems::flexStart; });
            createToggleButton ("flex-end",   groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignItems = FlexBox::AlignItems::flexEnd; });
            createToggleButton ("center",     groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignItems = FlexBox::AlignItems::center; });

            auto alignContentGroup = addControl (new GroupComponent ("content", "align-content"));
            alignContentGroup->setBounds (10, 300, 140, 160);

            i = 0;
            ++groupID;
            leftMargin = 15;
            topMargin  = 315;

            createToggleButton ("stretch",       groupID, leftMargin, topMargin + i++ * 22, true,  [this] { flexBox.alignContent = FlexBox::AlignContent::stretch; });
            createToggleButton ("flex-start",    groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignContent = FlexBox::AlignContent::flexStart; });
            createToggleButton ("flex-end",      groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignContent = FlexBox::AlignContent::flexEnd; });
            createToggleButton ("center",        groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignContent = FlexBox::AlignContent::center; });
            createToggleButton ("space-between", groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignContent = FlexBox::AlignContent::spaceBetween; });
            createToggleButton ("space-around",  groupID, leftMargin, topMargin + i++ * 22, false, [this] { flexBox.alignContent = FlexBox::AlignContent::spaceAround; });
        */
    }
    
    pub fn setup_flex_box_items(&mut self)  {
        
        todo!();
        /*
            addItem (Colours::orange);
            addItem (Colours::aqua);
            addItem (Colours::lightcoral);
            addItem (Colours::aquamarine);
            addItem (Colours::forestgreen);
        */
    }
    
    pub fn add_item(&mut self, colour: Colour)  {
        
        todo!();
        /*
            flexBox.items.add (FlexItem (100, 150)
                                 .withMargin (10)
                                 .withWidth (200));

            auto& flexItem = flexBox.items.getReference (flexBox.items.size() - 1);

            auto panel = panels.add (new DemoFlexPanel (colour, flexItem));
            flexItem.associatedComponent = panel;
            addAndMakeVisible (panel);
        */
    }
    
    pub fn create_toggle_button(&mut self, 
        text:      &str,
        groupid:   i32,
        x:         i32,
        y:         i32,
        toggle_on: bool,
        fn_:       fn() -> ()) -> &mut ToggleButton {
        
        todo!();
        /*
            auto* tb = buttons.add (new ToggleButton());
            tb->setButtonText (text);
            tb->setRadioGroupId (groupID);
            tb->setToggleState (toggleOn, dontSendNotification);

            tb->onClick = [this, fn]
            {
                fn();
                resized();
            };

            tb->setBounds (x, y, 130, 22);
            addAndMakeVisible (tb);
            return *tb;
        */
    }
    
    pub fn add_control<ComponentType>(&mut self, new_control_comp: *mut ComponentType) -> *mut ComponentType {
    
        todo!();
        /*
            controls.add (newControlComp);
            addAndMakeVisible (newControlComp);
            return newControlComp;
        */
    }
}
