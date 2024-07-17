crate::ix!();

#[no_copy]
#[leak_detector]
pub struct NavigableComponent<'a> {
    base:              Component<'a>,
    background_colour: Colour, // default = getRandomBrightColour()
    title_label:       Label<'a>,
    focusable_toggle:  ToggleButton<'a>, // default = "Focusable" 
    default_toggle:    ToggleButton<'a>, // default = "Default" 
    order_label:       Label<'a>, // default = "Order" 
    order_box:         ComboBox<'a>,
    children:          Vec<Box<NavigableComponent<'a>>>,
}

impl<'a> NavigableComponent<'a> {

    pub fn new(
        index: i32,
        total: i32,
        owner: &mut NavigableComponentsHolder) -> Self {
    
        todo!();
        /*


            const auto textColour = Colours::black.withAlpha (0.8f);

                    titleLabel.setColour (Label::textColourId, textColour);
                    orderLabel.setColour (Label::textColourId, textColour);

                    const auto setToggleButtonColours = [textColour] (ToggleButton& b)
                    {
                        b.setColour (ToggleButton::textColourId, textColour);
                        b.setColour (ToggleButton::tickDisabledColourId, textColour);
                        b.setColour (ToggleButton::tickColourId, textColour);
                    };

                    setToggleButtonColours (focusableToggle);
                    setToggleButtonColours (defaultToggle);

                    const auto title = "Component " + String (index);
                    setTitle (title);
                    titleLabel.setText (title, dontSendNotification);
                    focusableToggle.setToggleState (true, dontSendNotification);
                    defaultToggle.setToggleState (index == 1, dontSendNotification);

                    for (int i = 1; i <= total; ++i)
                        orderBox.addItem (String (i), i);

                    orderBox.setSelectedId (index);

                    orderBox.onChange = [this, &owner] { owner.orderChanged (*this); };

                    focusableToggle.onClick = [this] { repaint(); };

                    defaultToggle.onClick = [this, &owner]
                    {
                        if (! defaultToggle.getToggleState())
                            defaultToggle.setToggleState (true, dontSendNotification);
                        else
                            owner.defaultChanged (*this);
                    };

                    addAndMakeVisible (titleLabel);

                    addAndMakeVisible (focusableToggle);
                    addAndMakeVisible (defaultToggle);
                    addAndMakeVisible (orderLabel);
                    addAndMakeVisible (orderBox);

                    setFocusContainerType (FocusContainerType::focusContainer);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (backgroundColour.withAlpha (focusableToggle.getToggleState() ? 1.0f : 0.5f));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                    grid.templateRows = { Grid::TrackInfo (Grid::Fr (2)),
                                          Grid::TrackInfo (Grid::Fr (3)),
                                          Grid::TrackInfo (Grid::Fr (3)) };

                    grid.templateColumns = { Grid::TrackInfo (Grid::Fr (1)),
                                             Grid::TrackInfo (Grid::Fr (1)),
                                             Grid::TrackInfo (Grid::Fr (1)),
                                             Grid::TrackInfo (Grid::Fr (1)) };

                    grid.items = { GridItem (titleLabel).withMargin (2).withColumn ({ GridItem::Span (4) }),
                                   GridItem (focusableToggle).withMargin (2).withColumn ({ GridItem::Span (2) }),
                                   GridItem (defaultToggle).withMargin (2).withColumn ({ GridItem::Span (2) }),
                                   GridItem (orderLabel).withMargin (2),
                                   GridItem (orderBox).withMargin (2).withColumn ({ GridItem::Span (3) }) };

                    grid.performLayout (getLocalBounds());
        */
    }
    
    pub fn order_changed(&mut self, changed_child: &NavigableComponent)  {
        
        todo!();
        /*
            const auto addressMatches = [&] (const std::unique_ptr<NavigableComponent>& child)
                {
                    return child.get() == &changedChild;
                };

                const auto iter = std::find_if (children.begin(), children.end(), addressMatches);

                if (iter != children.end())
                    std::swap (*iter, *std::next (children.begin(), changedChild.orderBox.getSelectedItemIndex()));

                int order = 1;

                for (auto& child : children)
                    child->orderBox.setSelectedId (order++);

                if (auto* handler = getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::structureChanged);
        */
    }
    
    pub fn default_changed(&mut self, new_default: &NavigableComponent)  {
        
        todo!();
        /*
            for (auto& child : children)
                    if (child->defaultToggle.getToggleState() && child.get() != &newDefault)
                        child->defaultToggle.setToggleState (false, dontSendNotification);
        */
    }
}
