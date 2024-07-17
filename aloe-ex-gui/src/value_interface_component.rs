crate::ix!();

pub struct ValueInterfaceComponent<'a> {
    base: Component<'a>,
}

impl<'a> ValueInterfaceComponent<'a> {
    
    pub fn new(owner: &mut CustomWidgetComponent) -> Self {
    
        todo!();
        /*


            : customWidgetComponent (owner)

                valueTypeBox.addItemList ({ "Numeric", "Ranged", "Text" }, 1);
                valueTypeBox.setSelectedId (1);
                addAndMakeVisible (valueTypeBox);

                valueTypeBox.onChange = [this]
                {
                    updateValueUI();
                    customWidgetComponent.accessibleComponent.invalidateAccessibilityHandler();
                };

                addAndMakeVisible (readOnlyToggle);

                numericValueEditor.setInputRestrictions (10, "0123456789.");
                addChildComponent (numericValueEditor);

                addChildComponent (rangedValueComponent);
                addChildComponent (textValueEditor);

                updateValueUI();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

                grid.templateRows = { Grid::TrackInfo (Grid::Fr (1)), Grid::TrackInfo (Grid::Fr (2)) };
                grid.templateColumns = { Grid::TrackInfo (Grid::Fr (2)), Grid::TrackInfo (Grid::Fr (1)) };

                auto& valueEditComponent = [this]() -> Component&
                {
                    if (numericValueEditor.isVisible())    return numericValueEditor;
                    if (rangedValueComponent.isVisible())  return rangedValueComponent;
                    if (textValueEditor.isVisible())       return textValueEditor;

                    jassertfalse;
                    return numericValueEditor;
                }();

                grid.items = { GridItem (valueTypeBox).withMargin (2), GridItem (readOnlyToggle).withMargin (2),
                               GridItem (valueEditComponent).withMargin (2).withColumn ({ GridItem::Span (2) }), };

                grid.performLayout (getLocalBounds());
        */
    }
    
    pub fn get_value_interface(&mut self) -> Box<dyn AccessibilityValueInterface> {
        
        todo!();
        /*
            struct Numeric  : public AccessibilityNumericValueInterface
                {
                    explicit Numeric (ValueInterfaceComponent& o)
                        : owner (o)
                    {
                    }

                    bool isReadOnly() const override          { return owner.readOnlyToggle.getToggleState(); }
                    double getCurrentValue() const override   { return owner.numericValueEditor.getText().getDoubleValue(); }
                    void setValue (double newValue) override  { owner.numericValueEditor.setText (String (newValue)); }

                    ValueInterfaceComponent& owner;
                };

                struct Ranged  : public AccessibilityRangedNumericValueInterface
                {
                    explicit Ranged (ValueInterfaceComponent& o)
                        : owner (o)
                    {
                    }

                    bool isReadOnly() const override                { return owner.readOnlyToggle.getToggleState(); }
                    double getCurrentValue() const override         { return owner.rangedValueComponent.valueSlider.getValue(); }
                    void setValue (double newValue) override        { owner.rangedValueComponent.valueSlider.setValue (newValue); }

                    AccessibleValueRange getRange() const override
                    {
                        const auto& slider = owner.rangedValueComponent.valueSlider;

                        return { { slider.getMinimum(), slider.getMaximum() },
                                 slider.getInterval() };
                    }

                    ValueInterfaceComponent& owner;
                };

                struct Text  : public AccessibilityTextValueInterface
                {
                    explicit Text (ValueInterfaceComponent& o)
                        : owner (o)
                    {
                    }

                    bool isReadOnly() const override                         { return owner.readOnlyToggle.getToggleState(); }
                    String getCurrentValueAsString() const override          { return owner.textValueEditor.getText(); }
                    void setValueAsString (const String& newValue) override  { owner.textValueEditor.setText (newValue); }

                    ValueInterfaceComponent& owner;
                };

                const auto valueType = indexToValueType (valueTypeBox.getSelectedId());

                if (valueType == ValueType::numeric)  return std::make_unique<Numeric> (*this);
                if (valueType == ValueType::ranged)   return std::make_unique<Ranged>  (*this);
                if (valueType == ValueType::text)     return std::make_unique<Text>    (*this);

                jassertfalse;
                return nullptr;
        */
    }
}
